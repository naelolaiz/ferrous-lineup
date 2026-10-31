# Exercises: music theory in code

These exercises practise this part on the real code: notes and chords,
numerals, the editing tools, the hearing of chords and keys, sets, rows
and micromodes, voice leading and takes. Each one
says what is **already there**, so you extend it rather than build it
again. Some are things to play in the app and listen to; some are
pieces of Rust with a clear end; the ones called "Think about it" are
questions, with no code to write. Many are music theory first: work out
the answer on paper, then let the code check it.

Try the Rust ones on a branch of your own. Run `cargo test` after each
change, and `cargo fmt --all --check` and
`cargo clippy --workspace --all-targets -- -D warnings` before you call
one finished: CI runs all three.

## Notes, scales and chords

### 1. A chord that is still missing (lineup-core, easy)

`ChordQuality::ALL` holds thirty qualities, from the triads to
`Dominant13` and `Major13` (see [Chords: building, naming and
inversions](chords.md)). Read `ChordQuality::intervals`,
`symbol`, `name` and `numeral_suffix`, then add one that is absent,
such as the minor-major ninth, `MinorMajor9` (0, 3, 7, 11, 14), to
`ChordQuality::ALL` too, whose length, part of its type, goes up to
31. The compiler lists every `match` to update. `SPELLINGS`, in
`harmony/symbol.rs`, needs a line for the new quality, and
`every_quality_is_written_and_read_back` fails until its symbol reads
back as the new quality. `no_two_qualities_have_the_same_notes` tells
you if its pitch set collides with an existing one.

As pitch classes, why do big chords so often look like other chords on
another root? Write out the notes of C13 and find a seventh chord
hiding in them.

*Practises:* exhaustive `match`, arrays whose length is part of the
type, letting tests guard your data.

### 2. Cadd2 and the power chord (lineup-core, medium)

Pop charts sometimes write **Cadd2**: the same notes as Cadd9, with the
D played low, next to the C. Add "add2" to the spellings of `Add9` in
`SPELLINGS`, and a test that reads it. What is lost by reading it as
`Add9`?

**C5** is a **power chord**: just C and G, no third, the guitar's
favourite. It is not a `ChordQuality`. Before adding it, list what else
would have to know about it: `ChordQuality::family`, which every chord
must answer as major, minor or diminished; `is_minor`; the harmony
keys; and `find_chord`, which would then name every bare fifth. Is a
chord without a third major or minor? Decide, and write the decision
into a test.

*Practises:* following a type through the program before changing it.

### 3. The line cliché (lineup-core, small)

The **line cliché** keeps one chord while a single voice moves down by
semitones: `Am | Am/G# | Am/G | Am/F#`. Read it with
`library::chart::parse_chart_in`, play it with `chart::accompany` and
`Bass::Held`, and check the bass notes in a test. Is G♯ in A minor? In
which minor? And what is Am/F♯, as a chord of its own, in the names of
[Chords](chords.md#slash-chords-and-the-six-nine-chord)?

*Practises:* reading chart text, slash chords, a test on music you can
sing.

### 4. A drop-2 voicing (lineup-core, medium)

`tools::inversion(pitches, times)` and `tools::invert(notes, up)` turn
chords over (see [Editing tools as plain
functions](tools.md#inversions)). Jazz pianists and
arrangers use another voicing too, **drop 2**: take a chord in close
position and lower its second-highest note by an octave. C E G B, from
C4 up, becomes G3 C4 E4 B4.

Add `tools::drop_two(notes)` beside `invert`, with a test like
`inverting_chords_up_and_down`, and return `None` when a note would
fall below `Note::MIN`. Then put it in the piano roll's **Tools** menu.
Does the chord lane still name the chord the same way, or does the new
bass change its name? Why?

*Practises:* `Vec` manipulation (`remove`, `insert`, `sort`), tests
that check your own output.

### 5. Snap to scale, with a direction (lineup-core and lineup-app, medium)

`tools::nearest_in_key` and `tools::snap_to_scale` move notes outside
the key to the nearest note of it, for the piano roll's **Snap to
scale**. `nearest_in_key` tries one semitone below, then above, then
two, so the lower note wins a tie, and in C major C♯ goes to C.

Add `nearest_in_key_towards(note, scale, up)`, which breaks a tie in
the direction the melody is moving: a C♯ on its way up to D should
become D. Write tests for notes exactly halfway between two notes of
the key, both ways. `snapping_moves_only_notes_outside_the_key` pins
the old rule; would the new function change its answers?

*Practises:* searching with iterators (`find_map`), keeping the logic
out of the window.

### 6. Minor numerals, two ways (the app and lineup-core, easy)

**Edit > Settings… > Harmony > Roman numerals in minor keys** writes the
numerals of a minor key in two ways (see [Minor keys: bIII or
III](numerals.md#minor-keys-biii-or-iii)).

1. In A harmonic minor, what does each way write for C augmented
   (C E G♯)? And for E minor? Work it out, then check with the harmony
   keys.
2. Dorian is a minor mode too, but `Scale::is_minor_key` is false for
   it, so its numerals always count from the major scale. D Dorian is
   D E F G A B C. Which of its numerals would change if they counted
   from its own scale, and why might that confuse more than help?
3. `MinorNumerals::with` keeps the old setting in a guard,
   `let _restore = Restore(MinorNumerals::current());`, whose `drop`
   puts it back. Rename `_restore` to `restore` and run `cargo clippy`.
   Then change it to a bare `_` and run `cargo test -p lineup-core`.
   What happens each time, and why does `_` behave so differently from
   `_restore`?

*Practises:* reading numerals, guards that clean up in `Drop`.

### 7. Think about it: naming chords

- `find_chord` tries the bass note as the root first. Can you find a
  set of notes where this gives a surprising name? Is there a better
  rule?
- When nothing fits, `find_chord` adds the missing fifth and tries
  again, but only accepts chords of four notes or more. Why would
  accepting a triad that way be wrong? Think of C and E alone.

## Numerals and what chords do

### 8. The Neapolitan comes next (lineup-core, medium)

After a chord, the harmony keys outline the chords that often follow
it, from the tables in `harmony/progression.rs` (see [What comes next:
knowledge as data](numerals.md#what-comes-next-knowledge-as-data)).
The chord sheet already names the **Neapolitan**, ♭II, a major chord
on the lowered second degree (D♭ in C), but `next_chords` has no slot
for it, so after a D♭ chord nothing lights up.

A Neapolitan usually goes to V, or to I⁶₄ and then V. Add a `Slot`
with root 1 and `ChordFamily::Major`, with a move to V and a reason, to
`MAJOR` and to `MINOR`. Write the tests first: after D♭ in C major, G;
after D♭ in C minor, G too. Is D♭ a note of C minor's scale, or is the
Neapolitan outside that key as well? A `Suggestion` is only made for a
chord whose root is a note of the scale: find the field of
`Suggestion` that needs it, and explain why the Neapolitan itself can
never be suggested.

*Practises:* extending a data table, tests as a specification.

### 9. Functions by ear (the app, easy)

Type these on the chord sheet and listen.

1. `C Fm C`, then `C F C`. The only note that changes is A to A♭. What
   does it do to the mood? Find a song you know that does this.
2. In G major, which chord is V7/V? Which of its notes is not in G
   major, and where does that note want to go? Type it and check the
   function the sheet gives it.
3. Write a modulation from C major to F major through a pivot chord.
   Which chords do the two keys share? Which chord shows that F major
   has arrived? (Which note does F major have that C major does not?)

*Practises:* hearing borrowed chords, secondary dominants and pivots.

### 10. Spelling decides (lineup-core, small)

1. `is_neapolitan` checks the chord's spelling with
   `key.tonic_name().letter.steps_up_to(symbol.root.letter) == 1`.
   Explain why C♯ major in C fails that test while D♭ major passes,
   though both are played on the same keys of the piano. Find the test
   that says so.
2. The chord sheet does not name the **augmented sixth** chords
   (Italian, French and German), which lead to the dominant too. Look
   up their notes in C minor. Which of them sounds exactly like a
   dominant seventh on A♭, and what in its spelling tells the two
   apart? What would a `Role::AugmentedSixth` have to check?

*Practises:* why music spells notes, letters as well as semitones.

### 11. A row of borrowed chords (lineup-app, medium)

Songs in major keys often borrow chords from the minor key on the same
tonic: iv (F minor in C), ♭VI (A♭), ♭VII (B♭) and ♭III (E♭). This is
**modal interchange**, behind many of pop's bittersweet moments, and
the chord sheet already names such chords as borrowed. The harmony
keys, though, play only the key's own chords.

Add a second, smaller row of keys under the harmony keys with these
four chords, built from numerals with `library::numeral::parse` and
`RomanChord::in_key`. Give them their own colour, and a tooltip that
says which minor-key chord each one is.

*Practises:* reusing the library's numerals, egui layout.

### 12. Think about it: tables or rules

`next_chords` keeps its knowledge in tables of root, family and moves.
Most of the table could be computed from a few rules instead: roots
falling a fifth, steps up to the dominant. Which entries would the rules
miss? Is a table that says plainly what it means better than rules that
are shorter?

## Editing tools

### 13. Humanize velocities (lineup-core and lineup-app, easy)

Notes all played exactly as hard sound mechanical. The demos already
vary their velocities (`library/interpret.rs`), but the piano roll has
no tool for your own notes. Add
`tools::humanize(notes, amount, seed) -> Vec<NoteEvent>`, which moves
each velocity up or down by at most `amount`, kept within 1 to 127, and
a "Humanize velocity" item in the piano roll's **Tools** menu.

Ferrous Lineup has no random number crate, and does not need one:
`ToneRow::random` and the soloist use small generators of their own,
and `library/interpret.rs` uses the "SplitMix64" mixer, a few lines of
`wrapping_mul` and shifts. Why does the function take a `seed` instead
of making one up? How would you test it, and what should the same seed
give twice in a row? Where would the piano roll get a different seed
each time?

*Practises:* pure functions with an explicit seed, `clamp`.

### 14. Retrograde a phrase (lineup-core and lineup-app, medium)

Playing a phrase backwards is a classic way to make a variation. Rows
have their retrograde (see [Twelve-tone rows and their
rules](twelve-tone.md)), but the piano roll has no tool for
a phrase. Add
`tools::retrograde(notes: &[NoteEvent], span: usize) -> Vec<NoteEvent>`:
within `span` steps, each note keeps its pitch and length, and its
place is mirrored.

First decide what "mirrored" means for a note of length 2 in a span of
16: does its start mirror, its end, or both? Which choice keeps the
phrase inside the span and sounds most like the original played
backwards? What happens to `nudge` and `trim`? Add a "Retrograde" item
to the **Tools** menu, with a test on a four-note phrase you can sing.

*Practises:* deciding a musical rule before coding it, `NoteEvent`
fields.

### 15. Note-length tools (lineup-core and lineup-app, medium)

The piano roll's **Length** sets how long *new* notes are, but no tool
changes notes already written. Add two:

- `tools::set_note_length(notes, steps)` makes every note `steps` long,
  keeping its start and pitch;
- `tools::legato(notes, pattern_length)` makes each note last until the
  next note starts, so a line joins up.

`NoteEvent` keeps its length in whole steps, plus `trim`, how many
ticks before the end of its last step it stops. A note made longer
must clear a `trim` that no longer means anything, and `tidy_notes`
keeps a pattern's notes in order and inside it. Check what happens when
two notes of a channel would then overlap. Why is `set_note_length`
not simply `note.length = steps`?

*Practises:* note lengths and ticks, keeping a pattern's rules.

### 16. A strum tool, or a channel strum (all three crates, harder)

A guitarist strikes a chord one string after another. The band's
guitar does it already: `strum` in `band/guitar.rs` starts each string
a few ticks after the one before. Your own notes cannot do it yet. Work
out two designs before writing any code.

- **In the notes.** A Strum tool gives each note of a chord a growing
  `nudge`, from the lowest note up, a few ticks apart. Nothing else in
  `NoteEvent` changes: the file saves `nudge` already, and Quantize
  puts the notes back on the grid. A nudge is at most 23 ticks, one step less one.
- **In the instrument.** Each channel gets a "strum" setting in
  milliseconds. When several notes of a channel start in the same
  frame, the audio thread starts them that far apart, lowest first.
  Nothing in the pattern changes, but `Channel`, the `InsertChannel`
  command, the app's `project_commands` and the file format all need
  the new field.

Which one lets you strum one chord and not the next? Which one changes
fewer files? Real studios have both.

*Practises:* designing across crates.

### 17. Think about it: tools

- The selection holds notes by value, which works because a channel
  never holds two notes in the same place: `same_place` compares pitch,
  start and `nudge`. Imagine two notes in exactly the same place. What
  would go wrong with the selection and with `replace_notes`? Would a
  permanent id for every note fix it, and what would ids cost in the
  file?
- `tools::transpose` refuses to move a chord if one note would leave the
  MIDI range. Another design would move the notes that can move and
  leave the rest. When would that be better? How could the piano roll
  tell you why nothing happened?

## Hearing harmony and finding the key

### 18. A walking bass, by hand (pencil and paper, easy)

[Hearing harmony](hearing.md#a-walking-bass-worked-through)
works out how C and C7 score over a walking bass, with the passing
B♭ weighing 0.15 a step, and says that at 0.35 C would still win, by
0.941 to 0.925. Work those two numbers out yourself: the B♭ then weighs
1.4 and the profile 29.4. Then change `PASSING_BASS` in
`harmony/hearing.rs` and write the case as a test. At what weight does
C7 win?

*Practises:* following a scoring rule by hand before trusting it.

### 19. When to cut a bar (pencil and paper, easy)

When the chord sheet is filled from the notes, a cut costs 0.08 halfway
through the bar (`CUT_HALF_BAR`) and 0.12 on any other beat
(`CUT_BEAT`) (see [Cutting a bar into
chords](hearing.md#cutting-a-bar-into-chords)). A bar of
4/4 plays C, then G7. Each of them, heard alone, scores 1.02 per beat;
the whole bar, heard as one stretch, 1.0 per beat.

Does the sheet cut the bar when G7 comes on beat 4, for one beat? And
when it comes on beat 3, for two? Look at the `if total > here.0` in
the loop: what happens on a tie? How high must the score of each chord
alone be for the cut to happen in each case?

*Practises:* dynamic programming on a small case.

### 20. The Augurs chord (the app and pencil, easy)

Play E, G♯ and B with the left hand and E♭, G, B♭ and D♭ with the
right: the chord that opens the Augurs of Spring in *The Rite of
Spring*. Now play the A♭ harmonic minor scale, A♭ B♭ C♭ D♭ E♭ F♭ G,
and check that every note of the chord is in it. Why does
`in_one_key`, in `harmony/hearing.rs`, leave harmonic minor out?

Then a question about the cache of `write_heard_chords`: its key holds
the notes of the bars around a bar too, not only the bar's own. Why?

*Practises:* enharmonic spelling, what a cache key must include.

### 21. Measure the library's ear (lineup-core, medium)

The library has a simpler ear of its own, `library::listen::hear_chords`,
for the band's patterns, and `every_style_is_heard_as_its_chart` checks
it on every style as written. Write a report of your own, an ignored
test: loop over the styles, play each chart with every `Comping` and a
few `Bass` lines through `chart::accompany`, hear it, and count how
often the chords come back as written. Print the combinations that
fail most. Then try to raise the number by changing one constant at the
top of `listen.rs`, keeping the existing test passing. Run it with
`cargo test --release`: the same test runs many times faster.

*Practises:* measuring before changing, nested loops over enums.

### 22. Where the Habanera changes key (the app, easy)

1. Open the Habanera demo (Bizet's, from *Carmen*), turn on **🔍 Find
   the key**, and click bars 9, 10 and 11 on the ruler. Where exactly
   does D minor turn to D major? Is the D major refrain just "the same
   tune in another key"? Compare the first notes of both.
2. Hold A, C and E on the typing keyboard for a few bars, then add G♯,
   the raised seventh. Does the listener become surer of A minor? Why
   does one note matter so much?

*Practises:* hearing a modulation, what makes a key a key.

### 23. The price of a key change (lineup-core, easy)

In `harmony/key_finding.rs`, set `CHANGE_COST` to 0.8 and run the
report:

```sh
cargo test --release -p lineup-core --test key_finding report -- --ignored --nocapture
```

How many changes are found now, and how many of them are not in the
scores? Try 3.0. What does that tell you about the penalty?

*Practises:* tuning by measurement.

## Sets, rows and micromodes

### 24. A tour of the Row window (the app, easy)

1. Open **File > Demos > Twelve-tone > Triad chorale** and press Space.
   Watch the section names in the playlist: the same music, then
   backwards, upside down, and both.
2. Open the piano roll (**F7**) and click **Row…**. Pick **Famous and
   example rows > Schoenberg's Suite**. What does the window say about
   its groups of three, and which form is it combinatorial with?
3. Click the label **I11** at the top of the matrix, set **Together**
   to **6: hexachords**,
   and click **Write I11 into current pattern**: two hexachords as
   chords. Press **Ctrl+Z** to take it back.
4. Tick the three **Serialism** boxes, set **Octaves** to **Wide**, and
   write P4 again: Webern-like points of sound.
5. Click **Sets…**, type `octatonic`, and click **Use as the key**. The
   harmony keys now play chords from the octatonic scale.

### 25. Distinct forms (lineup-core, small)

A row has 48 forms, but they are not always 48 different rows. Write a
test that collects them into a `HashSet<ToneRow>` and counts them.
Check that Webern's Symphony row has 24 and Schoenberg's Suite 48.
Can any row have fewer than 24? (Could a row be one of its own I
forms? Write out what that means for each note, and you will find that
at most two notes could fit.)

*Practises:* `HashSet`, a proof by counting.

### 26. Derived rows (lineup-core, medium)

Webern's Concerto, Op. 24, uses a row whose four groups of three all
belong to the same class, 3-3 [014] (see
[Micromodes](micromodes.md#a-row-made-of-one-micromode-weberns-concerto)).
Such a row is called **derived**. Add `ToneRow::is_derived(size)`, true
when every segment of that size has the same set class, and a
`random_derived` that builds one by backtracking, as
`random_all_interval` does.

*Practises:* backtracking, `segments` and set classes.

### 27. Forte's way (lineup-core, small)

For six classes, Forte's prime form differs from Rahn's, which Ferrous
Lineup uses (see [Pitch-class sets](sets.md)). Write
`forte_prime()`, which breaks ties from the start of the set instead of
from the end, and show it in the Sets window's details when it differs.
Why can the trick of "the smallest number" not do this one directly?
(Think about which bits a comparison of numbers looks at first.)

*Practises:* bits as sets, reading a definition carefully.

### 28. Multiplying by 5 (lineup-core, medium)

Multiplying every pitch class by 5 (mod 12) turns the chromatic scale
into the circle of fourths: 0 5 10 3 8 1… This **M5** operation is
another way to transform a row or a set, used by some composers after
Webern. Add `PitchSet::multiply` and `ToneRow::multiply`, and find
which set classes M5 turns into a different class. (The Z-related
pairs are worth a look.)

*Practises:* modular arithmetic, testing over every set class.

### 29. A series of its own (lineup-core and lineup-app, medium)

With **Durations** ticked, the row writes its own numbers as the
durations. Let the user type a second row, used only for the
durations, and show both series in the window. `RowWriting` derives
`Copy`: why does it still compile once it holds a second `ToneRow`, and
why would it not if the second series were a `Vec<u8>`?

*Practises:* `Copy` and what it asks of every field.

### 30. The twelve-tone rules by ear (the app, easy)

1. Choose **twelve-tone row** as the key, pick **Schoenberg's Suite**
   in **Row…** with E as the key's note, and write P4 (E F G C♯ F♯ E♭
   A♭ D B C A B♭) as a melody. Tick **Voice leading**. Then swap two
   notes, then leave one out. How many notes does the check mark each
   time, and where does the dashed outline go?
2. Write the first three notes of P4 as a chord, then the rest as a
   melody. Then write notes 1, 2 and 4 as the chord. What changes?
3. Write P4 in one track and P4 an octave lower in another, in the same
   rhythm. What does the check say, and why does it say it only once?
4. Write a chord of C, E and G in a row key. Then G, B, D and F. Which
   rule do they break, and why did Berg break it on purpose?

### 31. A repeated note (lineup-core, small)

In `Walk::take`, in `harmony/voice_leading/serial.rs`, a note that only
repeats the note just played is never out of place. Find the lines that
make it so. Then write a test where a chord shares a note with the
chord before it, and check that nothing is marked.

*Practises:* reading bit masks, a test for an exception to a rule.

### 32. A tour of the micromodes (the app, easy)

1. Open the piano roll (**F7**) on the starting song's **Chords**
   pattern, press **Ctrl+A**, and click **Micromodes…**. Read the
   colours of C major: eight Mayor 2, six triads, one diminished.
2. Click **Menor 3**, then **▶ Hear it**. Try **Inverted (I)** and
   **Move it up**, and watch the triangle turn and flip on the clock.
3. Click **3-3 in Sets →**. The Sets window opens on 3-3. Pick **7-35**
   and click one of the micromodes under **Micromodes inside** to come
   back.
4. Open **File > Demos > Micromodes > Triads, two notes kept** and
   press Space. Listen for the one note that moves in each chord.
5. Click **Webern's trichords** in the Library's **Twelve-tone rows**
   group. With **3 notes** chosen under the key, the harmony keys play
   its four groups of three, each labelled 3-3. Now switch the key menu
   from **twelve-tone row** to **pitch-class set**: the key becomes the
   row's first group of three, set 3-3 on C, and the harmony keys play
   its forms, 3-3 T11 and so on. Pick **3** in the Menor row of the
   same menu: the same notes are now Menor 3, and nothing in the song
   moves.
6. Back on the starting song, while it plays, click **Fourths groove**
   in the Library: the song goes on in sus chords.

### 33. The micromodes of a melody (lineup-core, small)

Write `melody_micromodes(notes: &[PitchClass]) -> Vec<Option<&'static
Micromode>>`, which names the micromode of every three notes in a row:
notes 1 to 3, 2 to 4, 3 to 5 and so on (`windows(3)` does the
walking). Why is the answer `None` for some windows? Test it on
Webern's Concerto row: its four groups are Menor 3, but what are the
windows that straddle two groups?

*Practises:* `windows`, `Option` in a list.

### 34. Every colour at once (lineup-core, small)

Loop over `SetClass::all()` and find the smallest sets whose micromode
content has no zero: sets that hold all twelve micromodes. How many
notes do they need, and which are they? (For the size: the augmented
triad and Menor 1 must both fit.)

*Practises:* searching a table, `micromodes_in`.

### 35. Schat's tonalities (lineup-core, medium)

Webern's Concerto row splits the twelve notes into four forms of
Menor 3. Write a backtracking search that finds every way to split the
twelve notes into four forms of one micromode, and count them for each
of the twelve. One micromode cannot do it at all: which, and why?
(Think about which notes the forms of each micromode can take.)

*Practises:* backtracking, counting carefully.

### 36. A walk that remembers more (lineup-core, medium)

The walk of a micromode style that keeps two notes picks the freshest
new note first, and only then the form used least recently (see [The
walk: choosing with a
tuple](micromodes.md#the-walk-choosing-with-a-tuple)).
Make a walk that is sure to use every form it can reach before any
comes back, and test that Menor 3 visits all 24 of its forms in 24
steps. Is that always possible? For which micromodes can a "keep two notes" walk
visit every form exactly once? (This is a question about paths in a
graph.)

*Practises:* graphs, backtracking.

### 37. Hear the family (lineup-app, small)

Add a **▶** next to each family's name in the Micromodes window that
plays its micromodes one after another, from middle C.
`MicroAction::Hear` plays notes together; what would you add to play
them in turn? Look at how the piano roll's preview of a chord is
stopped.

*Practises:* actions as data, timing in the window.

## Voice leading

### 38. Parallel octaves (the app, easy)

Write I–IV–V–I in C major in four parts, the bass C3 F3 G3 C3 and the
soprano E5 F5 D5 E5. Tick **Voice leading**. Then move the soprano's D5
to G5 and read what the check says. Why are octaves between the
soprano and the bass avoided, while thirds and sixths between them are
welcome?

### 39. The leading tone and the seventh (the app, easy)

1. Write G7 to C with the seventh, F, in the alto falling to E. Then
   write G7 to Am, the deceptive cadence. Where must the leading tone
   go now? And the seventh?
2. The check leaves the leading tone alone in inner parts. Write V7 to
   I with the tenor going from B down to G. What does the C chord gain,
   and what would it lack if the tenor rose to C instead?
3. Write C, F, G7, C in four parts with the bass C3 F2 G2 C3, every
   inner part moving by step or holding. Read the line of moves at each
   change: are the outer parts ever in parallel or similar motion?

### 40. A unison to an octave (lineup-core, small)

In `kind_of_move`, a unison going to an octave falls through to the
last two arms of the `match`. Work out which kind it gets when both
parts rise, then write a test in `voice_leading.rs` that checks it,
with the `satb` helper the other tests use.

*Practises:* reading a `match` arm by arm.

### 41. A step of three semitones (lineup-core, small)

`Move::size` calls one or two semitones a step. An augmented second,
A♭ to B, is three semitones, and its name says "second". Which size
does the code give it, and which would a harmony teacher give it?
Change `size` to count letters, as `motion` does, and write a test.
Which other moves change size?

*Practises:* letters and semitones, again.

## Takes

### 42. A legato repeat (lineup-listen, medium)

`voice::notes` cuts a note sung again on the same pitch where the
loudness dips or jumps (see [From a pitch line to
notes](listen.md#from-a-pitch-line-to-notes)), so "la, la" is two notes
but a repeat sung legato, with no dip and no new attack, is one. Make a
test voice that sings the same pitch twice with only a 3 dB dip between
them, and see what comes out. What else changes at the start of a sung
syllable? (Think of the consonant: a short burst of noise, with no
pitch, where the pitch finder's best candidate has a low probability.)
Try cutting where the voicing probability drops for a frame or two.

*Practises:* measuring a change on known data.

### 43. Vibrato that wobbles too far (lineup-listen, easy)

`pitch_cuts` takes a move of more than three quarters of a semitone,
held for 70 ms, as a new note. An opera singer's vibrato can swing a
whole semitone, five or six times a second. Make a test voice with a
vibrato of ±1 semitone at 5.5 Hz (the test's `voice` uses ±0.25) and
see whether it breaks into notes. Which of `AWAY` and `STAYS` would you
change, and what would that cost a fast run of sixteenth notes?

*Practises:* the limits of a method.

### 44. Which take wins (lineup-core, medium)

Where two runs cover the same bars, the later one is kept (`kept` in
`take.rs`): sing the chorus again until it is right. Some singers would
rather keep the take that was most in tune. Every `SungNote` has its
pitch with a fraction, so how far each note was from its semitone is
known. Write a version of `kept` that keeps, for each stretch sung more
than once, the run whose notes were closest to their semitones, and a
test with two runs over the same bars.

*Practises:* walking ranges, choosing by a measure.

### 45. A take that follows a performance (lineup-core, harder)

`Take::laid` places a recording added from a file straight on at the
song's tempo, so a performance that slows down drifts from the bars
(see [Rough edges](listen.md#rough-edges)). If the song has tempo
markers that follow the performance, the marks already follow them.
What could find those markers from the recording itself? (A beat
tracker, such as Dan Ellis's dynamic programming, used for MIDI
performances in [From a MIDI performance or a score to a
demo](../songs/from-midi-and-scores.md#finding-the-beat).) Sketch what
it would take, and which crate each part would go in.

*Practises:* planning across crates.

### 46. Think about it: takes

- The tuning of a take is one number for the whole take. A singer who
  goes flat slowly over a long take would need what instead?
- Why does a take keep its recording as a reference, rather than only
  the notes heard in it?
- The marks are written by a thread that wakes every 5 ms. Why can the
  microphone's callback not write them itself, and what would go wrong
  if the recorder thread fell behind by a second?
