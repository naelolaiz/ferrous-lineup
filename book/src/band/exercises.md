# Exercises: the band

These exercises practise this part on the real code: the library, the
chord sheet, the band and its players, the orchestra, the song's form,
the stops, the soloist, your own styles, the practice tools and the
lead sheet. Each one says what is **already there**, so you extend it
rather than build it again. Some are things to try in the app and
listen to; some are pieces of Rust with a clear end; the ones called
"Think about it" are questions, with no code to write.

Try the Rust ones on a branch of your own. Run `cargo test` after each
change, and `cargo fmt --all --check` and
`cargo clippy --workspace --all-targets -- -D warnings` before you call
one finished: CI runs all three. An exercise that adds something to the
project file needs the next free format number (see [The version
number](../program/saving.md#the-version-number)).

## The library

### 1. A style of your own (lineup-core, easy)

`STYLES` in `library/style.rs` is a table of `style(...)` rows (see
[Styles: a table in a
`static`](library.md#styles-a-table-in-a-static)). Add one: a
progression from a song you like, written in numerals. For example the
whole of Pachelbel's Canon, `I | V | vi | iii | IV | I | IV | V` (the
**Piano ballad** style has only its first six chords), with
`Comping::ArpUpDown` and `Bass::Held`. Run `cargo test -p lineup-core`
before you open the app: if a numeral is misspelled, a bar cannot hold
its chords or the name is taken, `every_style_parses_and_fits_in_a_pattern`
or `names_are_unique_and_rhythms_exist` names the style.

Then try it in the app over a song in another key. Which chord names
change, and which numerals stay the same?

*Practises:* `const fn` tables, struct literals, reading test failures.

### 2. A rhythm with a cowbell and a fill (lineup-core, easy)

Add a drum pattern to `RHYTHMS` in `library/rhythm.rs`, in its genre's
block, with a line for `Kick`, one for `Snare` and one for `Cowbell`,
and a fill that exists, such as `"Tom fill"`. In 4/4 every line is
sixteen characters, from `X`, `x`, `o`, `h`, `l` and `.`; spaces are
only for reading. Put the cowbell on an offbeat or a clave, not on
every step, so it is heard as a colour of its own.

Done when `every_line_is_whole_bars_of_known_characters` and
`names_are_unique_and_fills_exist` pass, the Library lists the rhythm
with its sentence, and, with the band playing it, the fill comes in the
last bar of every eight.

*Practises:* data as text, tests that read the whole table.

### 3. A third chacarera (lineup-core, easy)

The library has a `Chacarera` rhythm in 6/8 and a `Chacarera trunca`
in 3/4. Read both first: a bar of each is twelve steps. Which steps
start the beats of a 6/8 bar, and which those of a 3/4 bar? Now write
a third bombo pattern for the same dance, with the head (`LowTom`) on
the 6/8 beats and the rim (`Rim`) on the 3/4 ones, the two against
three of the genre, and a style that uses it. Check in the rack that
the beat colours come in two groups of six.

*Practises:* reading existing data before adding to it, meters with
the same number of steps.

### 4. Near the song's tempo (the app, easy)

In the Library, on a song at 60 BPM, tick **Near 60 BPM**. Which
tempos does it keep? Then on a song at 180. `NEAR_SPEED` is 1.15: work the ranges out
before you look, and say why a fixed ±15 BPM would have been worse
(see [The Library tab](library.md#the-library-tab)).

*Practises:* ratios.

### 5. The same genre (lineup-app, medium)

Add a fourth box, **Same genre as the song's style**, that keeps the
presets of the genre the band plays. What does `Narrowing` need to
know, where do you get it from, and what should the box do in a song
without a band?

*Practises:* adding a filter without touching the others.

### 6. Think about it: closures that run once

`write_pattern_style` takes `write: impl FnOnce(&mut Project, usize)`.
The two closures passed to it today only read what they capture.
Would `impl Fn` work for them? What could a later closure do that `Fn`
would refuse, and why is asking for `FnOnce` the kindest choice for
whoever writes the next one?

## The chord sheet

### 7. How many bars fit (pencil, easy)

`Columns::count` (see [Why 1, 2, 4, 8, 12 or
16](chord-sheet.md#why-1-2-4-8-12-or-16)) gets the width of
the sheet and the width the widest bar needs. Work out by hand how many
bars a line holds in a sheet 900 pixels wide when the widest bar needs
100, 120 and 250 pixels. Then add those three cases to the test next to
`count`, and run it.

*Practises:* reading a short function closely.

### 8. Insert several bars (lineup-app, easy)

**Insert a bar before** and **Insert a bar after** call
`project.insert_bars(bar, after, 1)`. Add **Insert bars…**, which asks
how many. Which line of `open_bars` already makes room for more than
one, and what stops a song from growing past its longest?

*Practises:* using what the core already offers.

### 9. A bar inside a repeat (lineup-core, medium)

Write a test in `bars.rs`: a clip repeated three times, and a bar
inserted in the middle of its second time round. Write down first
which clips, at which steps, you expect afterwards; then run it. The
tests `a_clip_of_a_pattern_played_elsewhere_too_is_cut_in_two` and
`a_repeated_clip_loses_the_repeats_in_the_deleted_bars` show how to set
one up.

*Practises:* writing the expectation before the code tells you.

### 10. Think about it: before or after

`close_bars` reads the time signature, the tempo and the key at the end
of the deleted bars *before* it moves anything (see [What a deleted bar
takes with it](chord-sheet.md#what-a-deleted-bar-takes-with-it)).
What would it read if it asked afterwards, and what would the song
sound like?

### 11. The simile sign (lineup-core, medium)

Today `%` in a bar means "the harmony before goes on", so after
`| C G |`, a `| % |` holds G, while a musician reading a chart plays
C G again. Make `%` repeat the whole bar before. `read_bar` sees only
its own bar: what must it be given? What must `write_bar` and
`text::write` write for a bar that holds the chord before, and for one
that repeats a bar of two chords? Projects store the chords, not the
text: which files could change their meaning, and which can't?

*Practises:* changing what a notation means, and who reads it.

### 12. A title in the song (lineup-core and lineup-app, medium)

`text::read` reads a `Title:` line into `TextSheet::title`, but the
song does not keep it: **Export chord sheet…** writes the song's file
name as the title. Keep the title in the project and write it back.
It is a new field of the file: what does an older file read as?

*Practises:* a new field from the text format to the project file.

### 13. Paste in the song's key (lineup-app, medium)

A chart pasted from a message keeps its chord names, even when its
`Key:` line says another key than the song's. Add **Paste in the
song's key**, which moves the pasted chords from the chart's key to
the song's. `HarmonyName::moved(from, to)` already does it for one
harmony, keeping letters right. What does the paste need to know that
it does not keep today?

*Practises:* reusing a theory function in an editing action.

### 14. Bars per line in text (lineup-core, easy)

`text::write` puts at most four bars on a line, `BARS_PER_LINE`. Make
it a parameter. What in the column layout assumes four, and how does
`widths` have to change?

*Practises:* turning a constant into a parameter, arrays and vectors.

### 15. Think about it: other chart formats

- ChordPro, a common format for songbooks, writes chords inside the
  lyrics: `[C]Twinkle twinkle [F]little [C]star`. It has no bar lines.
  What would a reader need to guess to make a `TextSheet` from it?
- Some players want the numeral large and the chord name small under
  it. `HarmonyName::caption` gives the numeral of a chord. What should
  the large line say for a set, which has no numeral?

## How the band plays

### 16. Bass lines by eye (the app, easy)

Type `C | Am | F | G7 | C` and pick **Jazz waltz**. Look at the bass in
the piano roll: which note does it play on beat 3 of each bar, and how
far, by name, is it from the next bar's root? Then pick **Swing**, in
4/4: what does the walk put on beats 2 and 3 now? Check your answers
against `walk` in [The bass](band.md#the-bass).

*Practises:* reading a bass line.

### 17. Where the chord changes (the app, easy)

Type a bar `Dm7 G7` and a bar `C / / G7` in **Pop beat**. On which
beat does G7 start in each? Look at the piano part: does its rhythm
change in those bars, or only its notes?

### 18. Alberti bass (the app, easy)

Pick **Alberti bass** over `C | Am | F | G7`. The pattern is "low,
high, middle, high". Which note is low in the Am bar, and why is it
not A? (Hint: look at all four bars, and at [The harmonies, as
notes](band.md#the-harmonies-as-notes).)

### 19. A part you changed (the app, easy)

In a new song, change one note of the bass, then type a new chord in
bar 2. Which parts follow the new chord, and which doesn't? What does
the notice say? Click **Play it again**: what happens to your note?

### 20. A band over a demo (the app, easy)

Open **Canon in D** and click **Pop ballad** in the Library. Look at
the channel rack: which channels are muted, and which play on? Then
pick **No band** in the band's menu. Is the demo back exactly as it
was?

### 21. Think about it: what a fingerprint leaves out

- `part_print` leaves out the track a clip is on and its pattern's
  number. What would the user see if it did not?
- `band_styles()` returns `&'static [BandStyle]`. Why can `Band` hold a
  `&'static BandStyle` without a lifetime parameter, when it could not
  hold a `&BandStyle` borrowed from a local `Vec`?

## A bigger band, horns, and following your chords

### 22. Four guitarists (the app, easy)

Make a chord sheet with `C Am F G` and try **Pop beat**, **Folk**,
**Rock beat** and **Swing**. Click **Piano**, **Bass** and **Drums** in
the **You play** row, so that only the guitar is left. How does each
guitarist play the same four chords?

### 23. Chord shapes (pencil and the piano roll, easy)

In **Pop beat**, read the guitar's notes for G and for F in the piano
roll and write their shapes as chord books do (G is `320003`; see [The
music: how a guitarist plays a
chord](band.md#the-music-how-a-guitarist-plays-a-chord)). Why
does the F shape have no open strings? Which shape would you play for
E7, and which notes of the chord does it double? Check your answers
with `open_chords_are_the_ones_in_chord_books` and
`other_keys_take_barre_chords`.

### 24. Full chords through distortion (lineup-core, easy)

In `band/extras.rs`, change Rock beat's guitar from
`GuitarPart::Power(Comping::Pulse)` to
`GuitarPart::Strum(Comping::Pulse)`: full shapes through the overdriven
guitar. Listen with a SoundFont. Why do rock guitarists avoid this?
Put it back.

### 25. A guitar for Lo-fi (lineup-core, easy)

**Lo-fi** has no extra players. Give it a nylon-string guitar picking
the chords, `GuitarPart::Strum(Comping::ArpUp)`, in `BY_STYLE` of
`extras.rs`. Then misspell the style's name: which test fails, and
what does it print?

*Practises:* tables of constants, tests that guard names.

### 26. Think about it: the guitar

- Over a major chord the boogie shuffle rocks between the fifth and
  the sixth; over a minor chord it stays on the fifth (see
  `the_boogie_alternates_fifth_and_sixth`). What chord do A, C, E and F
  make, and why would a minor sixth over A minor be risky? What would a
  major sixth, F♯, turn the chord into?
- `try_frets` takes `found: &mut impl FnMut(&[u8])`. Try
  `found: impl FnMut(&[u8])`. What does the compiler say about the
  recursive call, and why?

### 27. A second keyboard (lineup-core, harder)

Add an electric piano (program 4) playing the style's comping an
octave up in the **R&B** and **Neo soul** styles. Which parts of
`band.rs` and `file.rs` must change (look at `written` and
`written_too` in [In the file](band.md#in-the-file))? Check
that a song whose band has only three players saves with the same
format number as before.

### 28. Horns held and struck (the app, easy)

Tick **Horn section** in **Swing** and in **Pop ballad**, over the same
chords. Compare the horns' notes in the piano roll: what is the same,
and what differs? Why does smooth voice leading matter even more for
long notes than for short ones?

### 29. Common tones (the app, easy)

Type `C Am F G` in **Pop beat** with the horns, and write down each
chord's horn voicing from the piano roll. Which horn keeps its note
from one chord to the next, and for how many chords? A note two chords
share is a **common tone**: why does the horn on it not move?

### 30. Cheap fifths (lineup-core, easy)

`cost` charges `PARALLEL_FIFTHS`, 24 semitones, for each pair of horns
moving in parallel fifths. Make it 2 and run
`cargo test -p lineup-core horns`. Which test fails, between which two
chords, and which horns move in fifths? Put it back.

### 31. Think about it: names in two tables

`HELD` and `BY_STYLE` in `horns.rs` name styles by their names. What
happens if a style is renamed in the library? Find the test that
would catch it.

### 32. A split point (lineup-app, medium)

Arranger keyboards often have a **split point**: only the keys below
it choose the chord, so the right hand can play a tune. Add one, say
below C4, as a second option next to **Accompaniment follows chords**.
Which keys reach `chord_held` now, and what still happens to the keys
above?

### 33. Two keys a major second apart (lineup-core, easy)

`chord_held` reads two keys by the interval between them (see
[Following the chords you
hold](band.md#following-the-chords-you-hold)). Make two keys
a major second apart a sus2 chord on the lower one. Which test needs a
new case? Before your change, holding F below G asked for no chord;
what does it ask for now, and is that what a player holding them would
mean?

### 34. Think about it: chords at once

Notes already sounding keep their old pitch until they end, so a chord
held for a whole bar changes only at the next bar. Arranger keyboards
play the new chord at once. What would the audio thread need to know
to do that, and what would it cost?

## The orchestra

### 35. Clap with the bass drum (the app, easy)

Apply the Orchestra to **The Rite of Spring, The Augurs of Spring** and
mute the **Song** track. Clap along with the bass drum. Can you predict
where the next stroke comes? Unmute the song and listen to the horns'
accents falling on the same eighths.

### 36. Felt beats by hand (pencil, easy)

`TimeSignature::felt_groups` gives 13/16 as 2 + 2 + 2 + 2 + 2 + 3. On
which sixteenths do the horns strike in a bar of 13/16 where they keep
to the felt beats? Open **Petrushka, The Moor's Room**, apply the
Orchestra, find a bar of 13/16 on the ruler and check. If they play
something else there, which rule of [Playing along with the
score](orchestra.md#playing-along-with-the-score) explains it?

### 37. `in_meter` for the tresillo bar (lineup-core, easy)

Work out by hand the line `in_meter` writes for a bar of 3+3+2/8 from
the Orchestra's `X... .... o... ....`. Then add it to
`a_rhythm_that_follows_the_meter_plays_each_bar_in_its_own` in
`rhythm.rs`, and run it.

### 38. A style in 4/4 over changing meters (the app, easy)

With the Orchestra playing the **Ritual of Abduction**, click **Pop
beat** in the Library. What happens to the ruler, and why does a style
in 4/4 have to do that, when the Orchestra did not?

### 39. Only the voices (the app, easy)

Open **Shchedryk (Carol of the Bells)**, apply the Orchestra with only
**Voices / instruments** ticked, and open the **Mixer**. Which
instruments do the Song track's channels play? Which do the new Piano
and Bass tracks play?

### 40. Accents nobody wrote (lineup-core, easy)

In `band/following.rs`, change `TUTTI` from `(9, 5)` to `(3, 2)` and
run `cargo test -p lineup-core accents_of_the_augurs`. Do accents
appear that Stravinsky did not write? Put it back.

## Form, variations, sections and locks

### 41. The intro leads somewhere (the app, easy)

Write `| Dm7 | G7 | Cmaj7 | Cmaj7 |`, a ii–V–I in C, and give it a
2-bar intro. Which chords does the intro write? They do not lead into
C: what do they lead into, and why?

### 42. The last chord (the app, easy)

Set the key to A minor (the **Key** menu beside the harmony keys),
write a song that starts on Am, and turn the ending on. Which chord
does it end on? Now change the first chord to F: which chord does it
end on now, and why is that still right?

### 43. How long is the song (the app, easy)

Export a song with a 4-bar intro, three choruses and an ending as a
WAV file. How many bars long is it? Check your answer against the
file's length and the tempo.

### 44. The form moves (the app, easy)

- Give a song an 8-bar intro, change a chord in it, then change the
  intro to 2 bars. What happens to your chord? Undo it.
- Type a chord in the first bar of the ending. Where is the ending now,
  and where is the end-repeat sign?

### 45. Think about it: the form in Rust

- `Form` is `Copy`; `Band` is not. Look at their fields: why could
  `Band` not be?
- In `played_pieces`, the chorus repeats only
  `if from <= start && end <= to`. What would a loop region from the
  middle of the intro to the middle of the chorus play without that
  test?

### 46. In two, then walking (the app, easy)

Choose the **Jazz ballad** style, write `| Cmaj7 | Am7 | Dm7 | G7 |`
twice, and set bars 5 to 8 to variation b. Open the bass's patterns in
the piano roll. How many notes does a bar have in a, and in b? Which
notes does the bass play in the Dm7 bar of each?

### 47. Fills and crashes (the app, easy)

- In an eight-bar **Pop beat**, set bar 5 to variation b and bar 6
  back to a. Which bars get a drum fill, and where do the crashes
  fall? Why could that sound restless in a real song?
- Choose **Swing** and play a few bars in each variation. What changes
  and what stays? Why does the time stay on the same cymbal?

### 48. A style for some bars (the app, easy)

- Write `| C | Am | F | G |` twice in **Pop beat**, and make bars 5 to
  8 **Bossa nova**. Where does the bossa nova's bass line start, and
  how does it differ from the pop one?
- Play a section in **Chorale**, which has no drums. What do the drums
  play there, and in the bar before it?

### 49. Locks (the app, easy)

- Open **Ode to Joy**, lock **Keys** in the rack, and apply **Doo-wop**.
  Which tracks changed, and what did Keys keep?
- In a new song, lock the kick and apply **Bossa nova**. What does the
  kick play now?

### 50. Think about it: markers and borrows

- `tidy` for variation markers sorts, then calls `dedup_by`, then
  `retain`. What would go wrong if it called `retain` before sorting?
- `played_in_runs` takes `play: impl Fn(...)`. Could it take an
  `FnMut`? What would change where it is called?
- `where_it_plays` returns `impl Iterator`. What would change if it
  returned a `Vec`, and why does `notes.extend(...)` accept both?
- `StylePattern` holds a `&'a mut Pattern`. What does the borrow
  checker say if you call another `Project` method while one is alive,
  and why is that a good thing?

## Stops and the soloist

### 51. Stop-time (the app, easy)

Type a twelve-bar blues in F with shots on the first four bars:
`| F7. | F7. | F7. | F7. | Bb7 | Bb7 | F7 | F7 | C7 | Bb7 | F7 | C7 |`,
and pick **Shuffle**. Play along in the gaps with the F blues scale
(F A♭ B♭ B C E♭). What changes in the feel when the band comes back in
bar 5?

### 52. Pushes (the app, easy)

Type `| C | Am | F | G |` in **Pop beat** and listen. Then push every
chord after the first: `| C | ^Am | ^F | ^G |`. Clap the piano's
rhythm both ways. Which beat is no longer struck? Then compare the
piano and the bass in the piano roll: how long does each hold a pushed
chord?

### 53. A hold over two bars (lineup-core, medium)

Type `C..` in bar 1, nothing in bar 2 and `G7` in bar 3, so the hold
lasts two bars, and look at the bass: one note. [Each player strikes
its own way](stops.md#each-player-strikes-its-own-way) says
why the bass uses `bass_struck` and not `Bass::Held`. Write a test in
`band.rs` that would fail if it did use `Bass::Held`.

### 54. A shot a quarter long (lineup-core, easy)

A shot is an eighth note: `SHOT_STEPS` in `band/stops.rs` is 2. Make
it a quarter note and run `cargo test -p lineup-core`. Tests fail not
only in `stops.rs` (`a_shot_strikes_once_and_the_band_stops`) but in
the library's charts and demos too: why do those care how long a shot
is? Fix the expectations, or decide the change was wrong.

### 55. A push of a quarter (lineup-core, medium)

Add `^^^C`, a push of a quarter note: a variant of `Push`. Let the
compiler show you every `match` that must learn it (`steps`, `marks`,
`describe`). Which other places need it? (Hint: `read_marks` refuses
three carets today, and `breaks_menu` in the app lists the pushes.)

### 56. Think about it: a fill before a stop

The drums play a fill in the bar before a change. Should a shot or a
rest in that bar keep it? Find where `drum_part` adds the fill and
where `apply_to_drums` takes the groove out, decide, and write a test
for what you chose.

### 57. Guide tones and chord scales (the app, easy)

- With the soloist on, open its part in the piano roll and find the
  notes marked `3` and `7`. Where does a 7th move to a 3rd a half step
  away at a chord change? Sing only the 3rds and 7ths, one a bar, over
  the band.
- Type `| Am7 | D7 | Gmaj7 | E7 | Am7 | D7 | G6 | G6 |` in G major.
  Work out the scale of each chord with the rules of [Chord
  scales](soloist.md#chord-scales) before you look. One
  chord plays an F natural, although the key has F♯: which one, and
  which other note of its scale is not in G major?
- In A minor, type `| Bm7b5 | E7 | Am | Am |`. Which note of E7's
  scale is not in A natural minor? Listen for it in the solo.

### 58. Leaps and enclosures (lineup-core, medium)

- The soloist leaps to the second chord tone one time in five
  (`dice.chance(20)` in `target`). Make it 60 and play a few solos with
  **Another solo**. How does the line change? Which tests still pass,
  and why?
- Add an **enclosure**, a favourite bebop figure: before a target, a
  scale note above it and then the note a half step below it, as D B C
  into C. Give it a new `NoteRole`, let the compiler show you every
  `match` that must learn it, and write a test in the style of
  `roles_name_what_each_note_does`.

### 59. A new solo every chorus (lineup-core, harder)

With **Choruses** set to 3, the soloist plays the same solo three
times. Make each chorus a new solo. Where is the chorus repeated (look
for `repeated_chorus`), and how could each time round get its own seed
while the song still plays the same every time it is opened?

## Your own styles and the practice tools

### 60. Read the files (the app and a text editor, easy)

Save a style from the style editor (see [Trying
it](style-editor.md#trying-it)) and open the
`.lineup-style` file in a text editor. Find its `format`, `bars`,
`chart` and the `lanes` with each channel and its notes. Then apply
it to a song, save the song, and search its file for `custom_rhythm`:
the whole style is inside. Why must it be, and what is the song's
format number at least?

### 61. Out-of-key dominants (lineup-core, medium)

Over a chord outside the key, `adapt_pitch` in `user_style.rs` keeps
the distances written over C, so the E7 that leads to Am plays
E F♯ G♯ A B C♯ D (see [Chords from outside the
key](style-editor.md#chords-from-outside-the-key)). The
soloist's chord scales, in `harmony/chord_scale.rs`, would give it the
F and C of A minor. Use them for dominant chords outside the key.
Which written notes of a style over E7 in C major now play
differently? Write the test first, in the style of
`template_notes_stay_in_the_key`, and listen: is a pattern that sounds
the same over every chord worth more than the right notes?

### 62. How many bars (pencil, easy)

A user style holds at most 256 steps and at most sixteen bars. How many
bars can a style in 7/8 have, and one in 6/4?

### 63. The metronome counts (the app, easy)

- Turn on the metronome over **Silent Night**, in 6/8. How many clicks
  are there in a bar, and where does the high one fall? Now
  **Greensleeves**, in 3/4: the same six eighth notes to a bar,
  counted differently.
- With the count-in on, pause a song halfway through a bar, then press
  Play again. What do you hear before the music, and why is that
  better than counting a whole bar and then starting halfway through
  the next?
- Set the speed to 50% and export a WAV file. How long is it, compared
  with the song at full speed? Why?

### 64. Tapping (the app, easy)

- Tap 4 times at an even pace, then 4 more times faster without
  stopping. Which tempo do you get, and why not the average of the
  eight gaps?
- In a song in 6/8, tap 80 times a minute. Which tempo does the song
  get, and why?
- Take over the bass with **You play**, then change a chord, and give
  the bass back: is the new chord in its part?

### 65. Think about it: the click and the count-in

- `ClickVoice::strike(Click::None)` returns at once. What would you
  hear if it set `level` to 0 instead?
- In `begin_play`, what goes wrong if `self.stop()` is not called when
  Play arrives while the song is already playing?

### 66. The speed trainer by hand (the app, easy)

- Set **from 50% to 100% by 10% every 2 passes**. Write down the speed
  of each of the first eight times round, then play it with the trace
  on and check.
- Set **from 100% to 60%**. What does the trainer do now? Think of a
  use for it (hint: a passage you can play fast, but not yet evenly).
- Start a run, Pause in the middle of it, then press Play. Then Stop
  and press Play. What is the speed each time, and why is it useful
  that the two buttons differ?
- With the trainer on and nothing looped, play a whole song. When does
  the speed go up?

### 67. Think about it: the trainer's messages

- In `came_round`, `self.laps_seen.replace(laps)` runs before the check
  for `self.training`. What would go wrong when you turn the trainer on
  in the middle of a song if it ran after it?
- The audio thread throws an old list of lap tempos away with
  `throw_away` when a new request replaces it. What would happen if it
  simply assigned the new list, and why does that matter on the audio
  thread in particular?

### 68. More for the ear (lineup-core, medium)

- **A tune for the major seventh.** `Interval::tune` has none going up.
  Find a song whose melody starts with one, check it on a piano, and
  add it. Does `going_up_names_a_tune` need to change?
- **Tunes going down.** Add a `tune_down` (the start of Beethoven's
  Fifth Symphony is a major third down) and use it when the question
  went down.
- **What you get wrong.** Keep, for each interval, how often it was
  asked and how often it was right, and ask the weak ones more often.
  How would `IntervalQuestion::new` use the counts without losing the
  rule "same seed, same question"?

### 69. Think about it: ear training

- Sixth chords are left out because C6 and Am7 have the same notes.
  Could they be asked fairly if the chord always played over its root
  an octave below? Would inversions still make sense?
- Let the progression exercise play the song's own chord sheet, with
  the names hidden. What would it need from `chord_sheet.rs`?

## The lead sheet and parts

### 70. Notation choices (lineup-core, medium)

- **Courtesy accidentals.** When a bar has an F♯ and the next bar an F
  natural by the key signature, many lead sheets print a natural in
  brackets as a reminder. Where would `BarAccidentals` need to know
  about the bar before?
- **Half-diminished.** Some lead sheets write Em7♭5 as Eø7. Add a
  choice for it. Is it a choice of `layout.rs`, or of the chord's name?

### 71. Think about it: the page

- **Endings.** When the last bar of a repeated section differs the
  second time, lead sheets use first and second endings: a bracket
  "1." over one bar, "2." over the other. What would the playlist need
  to tell `score.rs`?
- **Bars of rest.** A tune silent for eight bars could show one bar
  with a thick line and "8" over it. Where in `systems` would it go,
  and how does it change the bar numbers of the next line?

### 72. More instruments (lineup-core, medium)

- **The exact octave.** Add a choice that writes every part exactly as
  the instrument plays it: a ninth up for the tenor saxophone, an
  octave and a sixth for the baritone. Which function changes? How
  many leger lines does a tune around C5 need for the tenor, and why
  do books of lead sheets not do this?
- **D and G instruments.** A trumpet in D sounds a whole tone higher
  than written, an alto flute in G a fourth lower. Add them. What do
  `note()` and `semitones()` return, and what else in the window and
  the label has to know about them?
- **The E♭ clarinet** sounds a minor third *higher* than it reads. Is
  that a new `Transposition`, or the alto saxophone's part in another
  octave? What would `shift` need to know to tell them apart?

### 73. Parts as text, and the concert key (lineup-core, easy)

- Give **Export chord sheet…** the same choice of instruments. Which
  single function does the work for every chord? (See [The chords:
  moved letters and
  all](lead-sheet.md#the-chords-moved-letters-and-all).)
- Some parts print the song's own key under the instrument, "Concert
  key: B♭". Where in `layout.rs` would it go, and what does `lay_out`
  need to know that it doesn't now?
