# Exercises: songs as data

These exercises practise this part on the real songs: the constants in
`library/song/`, the demo they become, and the music they were read
from. Each one says what is **already there**, so you extend it rather
than build it again. Most are things to open in the app and listen to,
or to work out with a pencil; some are pieces of Rust with a clear end;
the ones called "Think about it" are questions, with no code to write.

Try the Rust ones on a branch of your own. Run `cargo test` after each
change, and `cargo fmt --all --check` and
`cargo clippy --workspace --all-targets -- -D warnings` before you call
one finished: CI runs all three.

## Songs as constants

### 1. A new song in its genre file (lineup-core, medium)

Add a public-domain tune to the demos. `Song`, `Part`, `SONGS` and the
genre files all exist; you are adding an entry, not a feature (see [A
song is a constant](demos.md#a-song-is-a-constant)).

1. Read two songs in the genre file you chose, such as `folk.rs` or
   `blues.rs`, and copy the shape of one, every field from `name` to
   `about`.
2. Write the tune with `|` between bars and a chart under it, and give
   the part a name such as "Verse" or "Tune".
3. Make it `pub(super) const`, and add it to `SONGS` in `song.rs`, at
   the end of its genre's block.
4. Let the tests find the mistakes: a bar that does not add up, a chord
   that cannot be read, a style in another meter, a note that is not in
   the key.

```sh
cargo test -p lineup-core library
cargo run --release -- --demos
```

It is done when the tests pass and the song is under **File > Demos >
Songs**, in its genre, opens on the chord sheet and plays.

*Practises:* struct literals, `const` data, reading test failures.

### 2. Extend a song that is already there (lineup-core, medium)

**Shenandoah** plays its song twice after a pickup, and **Danny Boy**
a verse and a chorus. Pick one and add a part the demo does not play
yet: an introduction for the band alone, a last verse, or a coda. Write
it bar by bar, give it a chart, and add it to the song's `form`. Then
open the demo and turn on the chord sheet: does the sheet show the
chords you wrote, in the bars you meant?

*Practises:* the melody notation, `form` as a list of names.

### 3. A last chorus a semitone higher (lineup-core, easy)

Many pop songs lift their last chorus by a semitone. Add a copy of a
song's chorus with `.in_key(...)` one semitone above the song's key, and
put it last in the `form`. Which numerals of the chart have to change?
(None: they are read in the part's key.) Play the end: the demo's last
bar is on the home chord of the new key, as
`a_song_that_ends_after_a_modulation_ends_in_the_new_key` checks.

*Practises:* builder methods on a `const` value.

### 4. A canon at the fifth (lineup-core, medium)

`with_voices` takes `(voice, bar)` pairs, so every voice of a round
sings the tune at the same pitch. Many canons have the second voice a
fifth higher or lower. Make each pair a triple, `(voice, bar,
semitones)`, and move each voice's notes by its semitones where the
demo builds the voices. Every round already written gets a `0`. Then
try Frère Jacques with its second voice `7` semitones up: which notes
clash, and why does a round need its copies at the same pitch to fit
the same chord?

*Practises:* changing a type and following the compiler's errors.

### 5. Your song's first notes (lineup-core, easy)

`the_first_notes_are_right` checks that Ode to Joy starts F♯ F♯ G A
and Twinkle, Twinkle starts C C G G. Add a line for a song you know
well, with its first four notes as MIDI numbers and their names in the
message. Get one wrong on purpose first, and read what the failure
says.

*Practises:* `assert_eq!` with a message, MIDI note numbers.

### 6. Lean on the highest note (lineup-core, medium)

`perform` in `library/interpret.rs` gives a note of a tune up to six
velocity steps for how high it is in its phrase. Singers often lean on
the single *highest* note of a phrase more than on the notes near it.
Give that one note two steps more, and write the test first: in
Twinkle, Twinkle, which note is the highest of the first phrase? Keep
the test `a_tune_gets_its_accents_but_stays_close_to_how_it_was_written`
passing.

*Practises:* `Iterator::max_by_key`, writing the test before the code.

### 7. Search by meter and key (the app, easy)

Open **File > Demos** and type `7/8` in the search box, then `5/4`,
then `2+2+3/8`. Then try `d minor`. Which matches come first, and which
are fainter? Why does `fold` keep `/` and `+` when it drops every other
mark (see [Search](demos.md#search))?

### 8. Think about it: methods that take `self`

`in_key` and `with_tempo` are `const fn`s that take `self` by value and
return a new `Part`. Why does that let them be chained inside a
constant, `Part::new(...).in_key(...).with_tempo(...)`? Why would a
method taking `&mut self` not work there?

## Reading the music

### 9. Which black key? (pencil, easy)

In C major, name the black key between F and G, with the rules of
[Spelling the
notes](from-midi-and-scores.md#spelling-the-notes): over a D7
chord; in the line G, ?, F; and over an A♭7 going to G. Then name the
black key between A and B over Gm, and over F♯7.

### 10. A new note, or a new mode? (pencil and the app, easy)

Write the scales of E natural minor, E harmonic minor and E Dorian.
Which note differs in each? Then play bars 21 to 23 of **Balderrama**.
The tune sings C♯: over which chord? Is the music in E Dorian there, or
is the C♯ a note on its way somewhere? And why is the song written in
E natural minor, not E harmonic minor (see [The tune and the
key](from-midi-and-scores.md#the-tune-and-the-key))?

### 11. Two ways to count six (the app, easy)

Clap six even claps, then group them 3+3 (ONE two three FOUR five six)
and 2+2+2 (ONE two THREE four FIVE six). Play **Acuarela del río**, a
polca in 6/8, and clap along with the bass, then with the drums. Which
grouping does each play?

### 12. A pianist's tempo (the playback trace, easy)

Trace **Uno**:

```sh
ferrous-lineup --trace "Uno" --out uno.tsv
```

and read its `# tempo:` lines for the first eight bars. Where does the
pianist slow down, and where in the phrase does it happen? How many
times does the tempo change in the whole song?

### 13. Roots a fifth apart (the app, easy)

In **For Unto Us a Child Is Born**, play bars 41 and 42 with only the
Keys channel unmuted. Write down the chords from the second beat of
bar 41 and their roots. How far is each root from the next? The note
the Demos menu shows for the song names the same chain.

### 14. Another 4/2 chord (the app, easy)

Bar 28 of the Handel chorus has `A7/G`, a seventh chord with its
seventh in the bass, figured 4/2 (see [Figured
bass](from-midi-and-scores.md#figured-bass-chords-written-as-numbers)).
Bar 45 has another. Which chord is it, which chord follows it, and is
the next bass note a step below?

### 15. The trout's dominants (the app, easy)

**Die Forelle** is in D♭ major, and its third verse is the one that
changes. Find the four bars of E♭7 on the chord sheet. What chord
comes after them, and what is it the dominant of? And what is E♭7 the
dominant of?

### 16. Why C minor needs B natural (the app, easy)

In **Sheep May Safely Graze**, find the middle section's bars in C
minor on the chord sheet. Which chord brings in B natural, and why does
a minor key need it? (Compare the key's scale, the harmonic minor, with
C natural minor.)

### 17. Who plays Boléro (the app, easy)

Play **Boléro** and write down, for each statement of the tune, the
instrument that plays it: the pattern names on the Song track tell
you. How many statements are there before the music leaves C major,
and which key does it go to? Then read Boléro's parts in
`classical.rs`: in "Horn, celesta and piccolos", `played_by` gives the
piccolos 19 and 28 semitones. Which intervals above the horn's tune
are those, and which notes of the harmonic series do they copy?

### 18. The Petrushka chord (pencil, easy)

Play C, E, G and F♯, A♯, C♯ together: two major triads a tritone apart,
the Petrushka chord. Now play the octatonic scale C, D♭, E♭, E, F♯, G,
A, B♭, which alternates half and whole steps. Which of the scale's
notes are not in the chord? Why can the chord not be in any one major or
minor key?

### 19. A mode over a drone (the app, easy)

Open **Petrushka, The Shrovetide Fair in the Evening**, turn on **🔍
Find the key** and play the Coachmen's Dance, written in E Mixolydian.
Which key does the finder show? E Mixolydian has the notes of A major:
why does a key finder that compares how long each note sounds hear the
key with the same notes? Then print the key-finding report:

```sh
cargo test --release -p lineup-core --test key_finding report -- --ignored --nocapture
```

and find the last tableau of Petrushka in it. Which keys does the score
give, and which does the finder hear in their place?

### 20. A chord the key does not have (the app, easy)

**Tea for Two** is in A♭ major. Find the key marker where its second
part starts. Which chord of A♭ is C major? (Count up from A♭: C is the
third note, so C major is III, a chord the key of A♭ does not
contain.)

### 21. From minor to major (the app, easy)

**Volver**'s verse is in G minor and its refrain in G major. Find the
key marker where the refrain starts. Which two notes of G harmonic
minor change in G major? Find them in the refrain's first bars.

### 22. Two tempos and four chords (pencil and the playback trace, easy)

Trace **The Big Bang Theory theme** and find the first note of bar 5 and
of bar 6. How long is a bar of 4/4 at 159 beats a minute? (4 × 60 / 159
= 1.509 seconds.) And at 130? Then:

- Name the four chords of the verse, IVadd9 V I vi7, in G and then in A.
- In bar 10 the tune alternates G and F♯ over G, then over Em7. Over G,
  F♯ is the chord's major seventh; what is it over E minor?

### 23. A tango against its lead sheet (research, medium)

Pick one of the tangos in `tango_played.rs`, such as **Patotero
sentimental**, and find a lead sheet of it. Transpose its chords to the
pianist's key, then compare one part bar by bar with the demo's chart.
Which chords did the notes alone name by a colour (a sixth, a ninth)
rather than the chord the song is built on?

### 24. Think about it: a script in the app

The scripts that read MIDI files and engravings are not part of the
app. What would it take to make **File > Import MIDI file…** write a demo
constant instead of a project? List what a person decides now that the
importer would have to decide: the key, where the parts begin, which
track is the tune, which chords a lead sheet allows. Which of those does
the app already know how to guess (see [Hearing
harmony](../theory/hearing.md))?
