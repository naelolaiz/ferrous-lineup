# An orchestra for music that changes meter

Files: [`crates/lineup-core/src/library/rhythm.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/rhythm.rs),
[`crates/lineup-core/src/library/style.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/style.rs),
[`crates/lineup-core/src/library/apply.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/apply.rs),
[`crates/lineup-core/src/band.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/band.rs),
[`crates/lineup-core/src/band/following.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/band/following.rs),
[`crates/lineup-app/src/app/library.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/library.rs),
[`crates/lineup-core/src/library/song/classical.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/song/classical.rs),
[`crates/lineup-core/src/project.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/project.rs)
and [`crates/lineup-app/src/library_browser.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/library_browser.rs)

Open a simple demo, say **Old MacDonald Had a Farm**, and the
**Library** lights up the style that matches it, Afropop. Click it, click **Apply**, and a band
comes in: a piano, a bass and drums playing the song's chords. The
demos of [the concert hall](concert-hall.md) had nothing lit. The Rite
of Spring, Petrushka, Boléro and the others had no matching style, and
any style you picked did more harm than good: the
**Ritual of Abduction** changes meter 32 times in 67 bars, and a style
in 4/4 turned all of it into 4/4, at the style's tempo, with the
score's own parts muted.

Now these demos have a style of their own, **Orchestra**. It plays
every bar in that bar's meter, keeps the song's time signatures and
tempo, and plays along with the score instead of muting it. It is the
matching style of the eleven concert hall demos: Promenade, the
*Pathétique* waltz in five, Boléro, La Mer, the Augurs of Spring, the
Ritual of Abduction and the five Petrushka demos.

## Trying it

1. Open **File > Demos**, type `Augurs` in the search box and press
   Enter: **The Rite of Spring, The Augurs of Spring** opens and plays.
   Stop it.
2. Look at the **Library**, on the left. **Orchestra (any meter)**
   is lit, the way a matching style always is. "Any meter" says the
   style is in no time signature of its own.

   ![The Library with "Orchestra (any meter)" lit, between "One drop" and "Paidushko (2+3/8)".](../images/step-9-orchestra-library.png)

3. Click it. Under **Timing** there are no Time signature and Tempo
   boxes any more, only a line that says the preset keeps the song's
   time signatures and tempo. Under **Accompaniment**, **Playing
   patterns** and **Voices / instruments** are both ticked. Either one
   alone brings the orchestra in too (see [Voices alone](#voices-alone)).

   ![The Orchestra popup: Playing patterns and Voices / instruments ticked, Chord progression not ticked, and under Timing: "Keeps the song's time signatures and tempo: this preset plays every bar in that bar's meter."](../images/step-9-orchestra-popup.png)

4. Click **Apply**. Four tracks appear under the song: **Piano**,
   **Bass**, **Drums** and **Strings**. The Song track stays on, the
   Time box still says 2/4 and the tempo still says 112.
5. Press Play. The strings hold each chord under Stravinsky's
   stamping strings. The horns, the timpani and the bass drum stay
   silent until bar 3, then strike *with* the score's accents: off the
   beat twice in bar 3, off the beat in bar 5, on the beat in bars 6 and
   7, off it again in bar 8. In the quiet bars 9 to 12 they rest, and the
   strings play softly.

   ![The Augurs of Spring after Apply: under the Song track, the Piano, Bass and Drums tracks have short strokes in bars 3 and 5 to 8, none in bars 9 to 13, and more from bar 14; the Strings track holds chords all along.](../images/step-9-orchestra-augurs.png)

Now do the same with **The Rite of Spring, Ritual of Abduction**, which
starts in 9/8 at Stravinsky's dotted quarter = 132 (the tempo box
shows 198, the same speed counted in quarter notes) and goes through 4/8, 5/8,
12/8, 6/8, 7/8, 3/4, 2/8 and more. The tracks are cut into short clips,
one for each run of bars in one meter, and each clip plays its bars the
way that meter is felt.

![The Ritual of Abduction after Apply: the Song track on top, and below it the Piano, Bass, Drums and Strings tracks. Above bars 9 to 11 the ruler shows 4/8, 5/8 and 9/8, and the band's clips are cut at each change.](../images/step-9-orchestra-ritual.png)

The orchestra plays the chords the app recognised in the score, the
ones on the chord sheet (**F5**). Where no chord was recognised, only
the bass drum plays. If you
tick **Chord progression** in the popup, it plays its own chords
instead, I and ♭VII of the Mixolydian mode, the way a style's
progression always replaces the song's chords.

### Voices alone

Untick **Playing patterns** before **Apply**, so that only **Voices /
instruments** is ticked, and the same four tracks come in. Untick the
voices and keep the patterns, and they come in as well: an orchestra
that follows the song always brings its own players and its own
patterns, on tracks of its own.

That was not so at first. With no band yet, the Library brought one in
only when both boxes were ticked. With one box, it did what it does for
a song without a band: with the voices, it gave the style's instruments
to the song's own channels named Keys and Bass; with the patterns, it
played the style as it plays for a pattern, in one meter, not alongside
the score. The Rite has no Keys channel, so the voices changed nothing
at all, and in Carol of the Bells the choir on the Song track turned
into horns.

Now copying a style's voices brings its band in on tracks of its own,
for every style, not only the Orchestra, and the song's own channels
keep their instruments and their notes. A style that does not follow
the song, picked with its voices but not its patterns, plays **Plain
accents**: its instruments strike the song's chords, without the
style's grooves. Picked with its patterns alone, it still opens in a
new pattern so you can try it, as before. The Orchestra always comes
in with its horns, timpani, bass drum and strings, whichever box is
ticked.

## The music

### Felt beats

A bar of 9/8 has nine eighth notes, but nobody counts nine. The
eighths come in three groups of three, and you feel three beats, each a
dotted quarter long. A bar of 5/8 has five eighths in two beats, a
short one of two and a long one of three. A bar of 2/4 at a fast tempo
goes by as a single beat. [Odd meters](odd-meters.md) calls these the
*felt* beats, and `TimeSignature::felt_groups` gives them for any time
signature.

A conductor beats the felt beats, not every eighth: in the Ritual, three
beats for a bar of 9/8, two for 5/8, one for 2/8. In music whose meter
changes all the time, that is what keeps an orchestra together, and it
is what the Orchestra style plays:

| Meter | Felt as | The horns and timpani strike on eighth (or quarter) |
|---|---|---|
| 2/8, 3/8, 2/4, 3/4 | one beat | 1 |
| 4/8 | 2 + 2 | 1 and 3 |
| 5/8 | 2 + 3 | 1 and 3 |
| 6/8 | 3 + 3 | 1 and 4 |
| 7/8 | 2 + 2 + 3 | 1, 3 and 5 |
| 9/8 | 3 + 3 + 3 | 1, 4 and 7 |
| 12/8 | 3 + 3 + 3 + 3 | 1, 4, 7 and 10 |
| 4/4 | 2 + 2 | quarters 1 and 3 |

The horns strike the chord on each of those beats and the timpani play
its root; when a chord changes between two beats, they play the new
one where it starts. The style's own drum line hits the first beat of
the bar hard and the others softly, but the orchestra keeps only the
first beat, and only where the music is loud: a bass drum on every
beat of a quiet bar would cover the score.

### Playing along with the score

A band plays the same groove bar after bar. An orchestra that joins a
symphony cannot: the music grows loud and soft, stops, and puts its
accents where the composer wants them. The Augurs of Spring is the
famous case. The strings stamp the same chord on every eighth note, all
equally hard, and eight horns throw accents onto some of those eighths
at places nobody can predict: the second and fourth eighths of one bar,
the second of another, the first of the next two. The shock of the
piece is that the accents never settle into a pattern. A bass drum on
every first beat would turn them into one.

So the Orchestra listens to the score before it plays, and changes
what its players would play:

- **Dynamics.** It plays as loud as the score, bar by bar: softer in a
  *piano* bar, harder in a *forte* one. Where the score is silent, it
  is silent.
- **Accents.** Where the score accents a chord, the horns strike it,
  the timpani strike its bass and the bass drum strikes with them, as
  hard as the score.
- **Passages of accents.** Near such accents, within four bars, the
  horns, the timpani and the bass drum play *only* the accents, so the
  irregular pattern stays the composer's.
- Everywhere else they keep to the felt beats, and the strings hold the
  chords.

What counts as an accent? A chord that passes both of the first two
tests below, or a note that passes the third, the way you would hear
them:

1. The whole orchestra plays it much louder than the rest of its bar,
   *and* than the same place in the bars around it: nearly twice as
   loud. A tutti chord after a thin texture.
2. An instrument in it plays a short note, between rests:
   nothing an eighth before it, nothing an eighth after it, and silence
   after the note ends. The horns in the Augurs do that. A melody of
   quarter notes does not (its notes touch), and neither does a chord
   held for four bars, which only comes in. That is why Carol of the
   Bells, whose choir holds a chord under the tune, has no accents, and
   its orchestra plays one stroke a bar from the start.
3. The score marks it itself, a sforzando or a strong accent:
   very hard (velocity 120 or more) and a fifth harder than the rest of
   its part in that bar.

A bar where most eighths would count as accents has none: the music
just got louder.

### Why horns and timpani

The style needs a part that marks the harmony on the beat and a part
that marks the bass, the jobs a piano and a bass guitar do in a band.
In an orchestra those jobs have long gone to the horns and the
timpani: in a symphony by Haydn or Beethoven, the loud chords that
close a phrase are the horns and trumpets playing the harmony while the
timpani strike the tonic or the dominant underneath. The strings hold
each chord for as long as it lasts, the way a band's string pad does
([A bigger band](bigger-band.md)).

The orchestra plays *along* with the score. In a pop demo the band
mutes the song's own chords and bass, because two pianos playing the
same chords in two different voicings sound like a mistake. In the
Rite, the song's parts *are* the piece: muting them would leave only
the band. So the Orchestra comes in on top, and every channel of the
score keeps its own mute.

## How it works

### A rhythm that follows the meter

A rhythm in the library is written in one time signature: its drum
lines are bars of steps, `X... .... o... ....` for a bar of 4/4, where
`X` is a hard hit, `o` a soft one and `.` silence. The band lays a
line out bar after bar, and in a song with time markers it writes each
run of bars in one meter as a pattern of its own.

The Orchestra rhythm has one line, that 4/4 bar for the kick, and a new
flag on `Rhythm`:

```rust
pub struct Rhythm {
    // ...
    pub follows_the_meter: bool,
}
```

The rhythms are built by a `const fn rhythm(...)`, and a builder method
sets the flag, the same way `swung` sets the swing:

```rust
const fn following_the_meter(self) -> Rhythm {
    Rhythm {
        follows_the_meter: true,
        ..self
    }
}
```

`..self` is *struct update syntax*: every field not named takes its
value from `self`. A `const fn` can do it because it only moves
values, so the whole table of rhythms is still built at compile time.

Drum notes for a bar now come from `Rhythm::notes_in(drum, time,
length)`. For an ordinary rhythm it does what it always did. For one
that follows the meter it first writes the line again for `time`:

```rust
fn in_meter(line: &str, own: TimeSignature, time: TimeSignature) -> String {
    // the hit on the first beat, and the hit on the second felt beat,
    // of the line's own bar...
    let mut bar = vec!['.'; time.steps_per_bar()];
    for (index, start) in felt_starts(time).into_iter().enumerate() {
        if let Some(step) = bar.get_mut(start) {
            *step = if index == 0 { first } else { other };
        }
    }
    bar.into_iter().collect()
}
```

`felt_starts(time)` turns the felt groups into steps: a group of three
eighths is three times `time.unit_steps()`, the steps one eighth lasts.
For 9/8 that gives steps 0, 6 and 12, and the 4/4 bar's `X` and `o`
land there: `X.....o.....o.....`.

A rhythm that follows the meter has no time signature of its own, so
`Rhythm::for_time`, which picks a rhythm for a song's new time
signature (see [Odd meters](odd-meters.md)), never picks it.

### A style that follows the song

A style knows whether it follows the song by asking its rhythm:

```rust
pub fn follows_the_song(&self) -> bool {
    super::Rhythm::named(self.rhythm).is_some_and(|rhythm| rhythm.follows_the_meter)
}
```

`Option::is_some_and` is `true` only when there is a value and the
closure says yes for it, so a style whose rhythm is missing does not
follow anything.

Applying a style used to put the song in the style's time: a function
called `remeter` laid every bar out again in the style's one meter.
Now the parts to apply, `StyleParts`, have a step that drops the time
signature and the tempo when the style follows the song:

```rust
pub fn following(mut self, follows: bool) -> Self {
    if follows {
        self.time_signature = false;
        self.tempo = false;
    }
    self
}
```

The library click and the band menu on the chord sheet both go
through it, so neither re-meters the Rite. The chord sheet's menu of
styles for a section, which only offers styles in the bar's own time
signature, now offers the Orchestra in any meter too. In the
app the same flag swaps the Timing boxes for the line you saw in the
popup, and the Library adds "(any meter)" to the style's name.

### Joining the song

The Library's **Apply** ends up in `band_plays_parts` in
`crates/lineup-app/src/app/library.rs`. It decides first whether a
band comes in at all:

```rust
let follows = match action {
    LibraryAction::Style(style) | LibraryAction::StyleInSong(style) => {
        style.follows_the_song()
    }
    LibraryAction::Rhythm(rhythm) | LibraryAction::RhythmInSong(rhythm) => {
        rhythm.follows_the_meter
    }
    _ => false,
};
let has_music = !self.project.playlist().chord_markers().is_empty()
    || self.project.playlist().clips().iter().any(|clip| {
        self.project
            .pattern(clip.pattern)
            .is_some_and(|pattern| !pattern.is_empty())
    });
let joins = if follows {
    parts.rhythm || parts.for_preset().instruments
} else {
    parts.for_preset().instruments && has_music
};
if self.project.band().is_none() && !joins {
    return false;
}
```

`match` with `|` between two patterns runs the same arm for both, and
`_ => false` covers every other kind of Library item. `has_music` is
true when the song has chords on the chord sheet, or any notes whose
chords the band can hear; `Iterator::any` stops at the first clip whose
pattern has a note. When `band_plays_parts` returns `false`, the caller
goes on as before: it opens the style in a pattern of its own to try,
or, while the song plays, applies it to the song's own patterns.

The same rule is in `Project::apply_band_parts` in `band.rs`, which
builds the band, with one more line: a band that came in for its voices
alone plays plain accents.

```rust
band.accents_only = !parts.rhythm && !follows;
```

### Listening to the score

`crates/lineup-core/src/band/following.rs` is a new module of the
band. `band.rs` declares it with `mod following;`, and Rust finds it in
`band/following.rs`, a folder named after the parent file. Its type is
`pub(super)`: visible to `band.rs` and nothing else.

`Following::hear(project, bars)` collects what the song plays. It walks
the clips that are heard, skips the band's own patterns, and records
every note's start in a `BTreeMap<usize, Vec<(usize, u8, usize)>>`:
step to a list of (channel, velocity, length). A `BTreeMap` keeps its
keys in order, so `struck.range(bar.start..bar.end())` gives the notes
of one bar, in time order, without looking at the others.

It returns an `Option<Following>`: `None` when the song plays no note
at all, a chord sheet with nothing else. There is no score to follow
then, and the orchestra plays as its style wrote. The same goes for the
bars before the score's first note and after its last one ends:

```rust
let first = *struck.keys().next()?;
let last = struck
    .iter()
    .flat_map(|(&at, notes)| notes.iter().map(move |&(_, _, length)| at + length.max(1)))
    .max()?;
```

The `?` after `next()` and `max()` returns `None` from `hear` when
there is nothing, the same way it passes an error up from a function
that returns a `Result`. `flat_map` turns each step's list of notes
into the steps where they end, one long iterator, and `max` takes the
last. The `move` before the inner closure copies `at` into it: the
inner iterator is used after the outer closure has returned, when its
own `at` is gone.

How loud a bar is: the loudest note at each step it plays, and the
middle one of those, divided by 100 (mezzo-forte in the demos).

```rust
fn middle(mut values: Vec<u32>) -> u32 {
    if values.is_empty() {
        return 0;
    }
    values.sort_unstable();
    values[(values.len() - 1) / 2]
}
```

The middle value (the *median*) rather than the average, because one
loud accent should not make the whole bar loud.

The thresholds are written as fractions, so all the arithmetic stays in
whole numbers:

```rust
const TUTTI: (u32, u32) = (9, 5);
// ...
let (times, by) = TUTTI;
let stands_out = strength(at) * by >= in_bar * times
    && strength(at) * by >= elsewhere * times;
```

`strength(at) * 5 >= in_bar * 9` is `strength(at) >= 1.8 * in_bar`
without a floating-point number in sight. `elsewhere` is the same place
in the two bars before and the two bars after, the quieter of the two
sides; the first bar has no bars before it, so it compares with the
bars after only:

```rust
let elsewhere = match (same_place(before.clone()), same_place(after.clone())) {
    (Some(before), Some(after)) => before.min(after),
    (one, other) => one.or(other).unwrap_or(0),
};
```

Matching on a pair of `Option`s handles all four cases in two arms:
the first takes both values, the second whichever exists, or 0.

To know whether a channel strikes "alone", `hear` keeps each channel's
starts in order, and finds the note's place among them with
`partition_point`, a binary search: the number of starts before `at`.

```rust
let place = steps.partition_point(|&step| step < at);
let before = place
    .checked_sub(1)
    .is_none_or(|before| steps[before] + ALONE < at);
let after = steps
    .get(place + 1)
    .is_none_or(|&after| after > at + ALONE.max(length));
```

`checked_sub(1)` is `None` for the first note instead of underflowing
below zero, and `Option::is_none_or` is `true` for `None`: no note
before the first one is as good as a rest.

`Following::play_along(player, notes, ...)` then takes what a player
of the style wrote and changes it: notes in a silent bar are dropped,
the horns' and timpani's notes in a passage of accents are dropped,
the bass drum keeps only loud first beats, and every velocity is scaled
by the bar's loudness. Then it adds the strikes: at each accent, the
horns play the notes of the chord that is held there, the timpani its
bass, and the bass drum a hit, as hard as the score's accent.

`write_band`, which writes every player's part, calls it only for a
style that follows the song:

```rust
let mut part = self.band_notes(&playing, player, &score);
if let Some(following) = &following
    && playing.style.follows_the_song()
{
    part = self.played_along(following, &playing, player, part, &score);
}
```

`if let ... && ...` is a *let chain*: the `if` runs only when the
pattern matches and the condition after `&&` holds.

### 999 patterns

The band writes a pattern for each player and each run of bars in one
meter. Two runs can share a pattern when they play the same notes, as
the drums of two runs of 9/8 do, but the chords change from run to
run. The Ritual of Abduction has 33 runs and the band four players, on
top of the score's own patterns, and a project could hold only 99
patterns. Once the limit was reached the band quietly stopped
writing, and whole stretches of the Ritual had no drums. The limit is
now 999 (`MAX_PATTERNS` in `project.rs`). The twelve-tone window,
which says when a row would need more patterns than a project can
hold, now reads the number from `MAX_PATTERNS` instead of saying 99.

## Checking it

The tests in `band.rs`:

- `an_orchestra_plays_every_bar_of_the_rite_of_spring_in_its_own_meter`
  checks that the Ritual keeps every meter change, its tempo changes
  and every channel's mute, that the players are a French horn, timpani
  and a string ensemble, that the bass drum plays only with the score's
  accents or on the first beat of a bar, and not in every bar, and that
  the horns and timpani play on felt beats, chord changes or accents.
- `the_orchestra_strikes_with_the_accents_of_the_augurs_of_spring`
  copies only the voices and checks that the song's channels and notes
  stay as they were, that the horns, timpani and bass drum strike at
  steps 18, 22, 34, 40, 48 and 58 in the first eight bars and nowhere
  else, and that in bars 9 to 12 they rest and the strings play below
  velocity 60.
- `a_tune_or_a_held_chord_is_not_an_accent` hears no accents in Carol
  of the Bells, and checks the horns play the first beat of each of its
  first eight bars.
- `with_no_score_to_follow_the_orchestra_plays_as_written` applies it
  to a chord sheet with no notes, and checks every player plays.
- `copying_only_the_voices_brings_a_band_that_plays_plain_accents`
  copies only the voices of a style that does not follow the song,
  Emotional pop, onto the tango El Choclo, and checks its band comes in
  playing plain accents while the song keeps its sounds and its notes.
- `picking_the_orchestra_keeps_the_meters_of_the_song` picks the
  Orchestra for the Augurs from the band menu and then picks
  **Pop beat**: the first keeps 2/4, the second puts the song in 4/4.

In the app, `copying_only_the_voices_or_the_patterns_brings_the_band_and_leaves_the_song_alone`
in `ux_tests.rs` clicks through the Library popup for the Augurs, the
Ritual, Petrushka's Room, Carol of the Bells and a tango, with one box
ticked, and checks the band came in and the song's channels and notes
did not change.

To hear it without listening, save the Augurs after **Apply** and read
its [playback trace](playback-trace.md):

```sh
ferrous-lineup --trace augurs.lineup --out augurs.tsv
awk -F'\t' '$11 == "Kick" && $3 <= 16 { print $3, $4, $5, $16 }' augurs.tsv
```

Column 3 is the bar, 4 the beat, 5 how far into the beat in steps (a
quarter note has four, so 2 is the eighth after the beat) and 16 the
velocity:

```text
3 1 2.000 127
3 2 2.000 127
5 1 2.000 127
6 1 0.000 127
7 1 0.000 127
8 1 2.000 127
14 1 2.000 127
14 2 2.000 127
16 1 2.000 127
```

Off the beat twice in bar 3, off it in bar 5, on it in bars 6 and 7,
off it in bar 8: the horns' accents as Stravinsky wrote them, with
nothing in the quiet bars between 8 and 14.

## Exercises

1. Apply the Orchestra to **The Rite of Spring, The Augurs of Spring**
   and mute the **Song** track. Clap along with the bass drum. Can you
   predict where the next stroke comes? Unmute the song and listen to
   the horns' accents falling on the same eighths.
2. Open **Petrushka, The Moor's Room**, apply the Orchestra and find the
   bar of 13/16 in the ruler. On which sixteenths do the horns strike
   there, and does the bass drum play? Check your answer in the
   playback trace, and say why from the rules above.
3. With the Orchestra playing the Ritual of Abduction, pick **Pop beat**
   from the chord sheet's band menu. What happens to the ruler, and
   why does a style in 4/4 have to do that?
4. Work out by hand the line `in_meter` writes for a bar of 7/8 from the
   Orchestra's `X... .... o... ....`, using the felt groups 2 + 2 + 3.
   Then write a test for it next to
   `a_rhythm_that_follows_the_meter_plays_each_bar_in_its_own` in
   `rhythm.rs`.
5. Open **Shchedryk (Carol of the Bells)**, apply the Orchestra with
   only **Voices / instruments** ticked, and open the **Mixer**. Which
   instruments do the Song track's Lead, Keys and Bass channels play?
   Which do the new Piano and Bass tracks play?
6. In `following.rs`, change `TUTTI` from `(9, 5)` to `(3, 2)` and run
   `cargo test -p lineup-core accents_of_the_augurs`. Do accents appear
   that Stravinsky did not write? Put it back.
