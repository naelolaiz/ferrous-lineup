# Ear training

Files: [`crates/lineup-core/src/ear_training.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/ear_training.rs),
[`crates/lineup-audio/src/phrase.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-audio/src/phrase.rs),
[`crates/lineup-app/src/ear_training.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/ear_training.rs)
and [`crates/lineup-app/src/app/ear.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/ear.rs)

Everything so far helps you *see* harmony: chord names on the sheet,
numerals and colours on the harmony keys, set names in the twelve-tone
window. Musicians also need to *hear* it: to tell a minor third from a
major one, a diminished chord from a minor one, and the V of a song
from its IV, without looking. That is a skill of its own, and it is
learned the way ear-training games teach it: something plays, you name it, and you are told at once whether you were right.

This is item 13 of the plan. It is **experimental** and stays out of
the way: it lives in its own window, which is closed until you open
it, and it changes nothing in the song or in the file.

## What you see

**View > Ear training** opens the window. Three tabs sit at the top:
**Intervals**, **Chords** and **Progressions**. Under them, a box says
what to ask; then **Start** asks the first question, and plays it. If
the song was playing, it pauses, so the question is heard alone.

![The Intervals tab: m3, M3, P4, P5 and P8 chosen, played Up. The answer P5 is green, and the text explains: "Bb3 up to F4: a perfect fifth, 7 semitones. Open and stable: the root and the fifth of every major and minor chord. Like Twinkle, Twinkle, Little Star, at its first leap."](../images/step-9-ear-intervals.png)

**Intervals.** Two notes play, and the big buttons are the answers.
The box at the top picks which intervals can be asked (a minor third,
a major third, a fourth, a fifth and the octave to begin with), and how
they are played: **Up** (low note first), **Down** (high note first) or
**Together**. Fewer choices make it easier; start with a few that sound
very different, and add the others one at a time.

After you answer, your button turns green or red, the right one turns
green, and a box explains the answer: the two notes with their octaves,
the interval's full name, how many semitones it spans, what gives it
its sound and, going up, a tune that starts with it. Then every answer
button plays *its own* interval from the same note, so you can hear the
one you said next to the right one. **Play again** plays the question
as often as you like, before or after answering; **Next** asks another
(it is **Skip** before you answer, and a skipped question doesn't
count).

![The Chords tab with all nine kinds and Inversions ticked. The answer "maj" is red, "dim7" green, and the text: "Ddim7: D diminished seventh, root position. D F Ab Cb: a minor third, a minor third and a minor third stacked up from D. Cb is the B key, written Cb because it is the chord's seventh..."](../images/step-9-ear-chords.png)

**Chords.** A chord plays, and you name its kind: major, minor,
diminished, augmented, or one of five seventh chords. The root changes
from question to question; only the kind counts. **Note by note** plays
the chord one note at a time from the bass up, each held, so it ends as
the whole chord. With **Inversions** ticked, the lowest note is not
always the root (more on that below).

![The Progressions tab while the band plays: four slots, the first showing I, the second framed in white because its chord is sounding, and below them seven harmony keys showing only I ii iii IV V vi vii° and home/away/tension, iii and vii° dimmed.](../images/step-9-ear-progression-playing.png)

**Progressions.** This is the exercise the plan asked for: the band
plays a few chords in a key, one bar each, and you name them by their
numerals. The question says the key ("In F major") and the first chord
is always I, home, so you know where you start. The answers are the
harmony keys you know from the bar at the bottom of the window, with
their colours and their job (home, away, tension) but with the chord
names hidden. Keys for chords that can't be asked are dim. While the
band plays, a white frame walks along the slots, as it does on the
harmony keys, so you know which bar you are hearing.

Click the keys in order, one per chord after I. Then each slot turns
green or red, a red one says what the chord was ("was IV"), and the box
spells out the chords, their numerals and why each one follows the one
before, in the words of the harmony keys' hints:

![The answered progression: slots I, vi (was IV), IV (was V), V (was I) and the text "F – Bb – C – F: I IV V I in F major. I to IV: moving away from home, toward the dominant. IV to V: a step up to the dominant, ready to come home. V to I: the strongest ending. The leading tone rises to the root."](../images/step-9-ear-progression.png)

After answering, every slot and every key plays its chord on the piano,
and **Hear the key** plays I, IV, V and I: a cadence that says where
home is, which helps when you lose your way.

The box at the top picks the **Keys** (major, minor or both), the
**Chords** that can come up (I IV V; or I ii IV V vi; or all seven),
whether they are **Sevenths** (V7, ii7, Imaj7...), who plays them
(**Piano**, **Piano and bass**, or the **Band** with drums), how many
chords (4 or 8) and the **Tempo**.

**The score.** The line at the bottom counts right answers, the
percentage and how many in a row. Each tab keeps its own, and in a
progression every chord counts as one answer. **Start again** sets it
back to zero. The score is forgotten when the app closes; the choices
in the boxes are remembered.

## The theory in the answers

The app teaches with its answers, so they have to be exactly right.
This is what they say, and why.

### Intervals

An interval's name has two parts. The **number** counts letters, both
ends included: C up to E is C, D, E, three letters, a *third*. C up to
Eb is a third too: same letters. The **quality** then comes from the
semitones: a third of 4 semitones is *major*, of 3 *minor*.

Seconds, thirds, sixths and sevenths come in major and minor. Unisons,
fourths, fifths and octaves are called **perfect** instead: turned
upside down (the low note moved up an octave), a perfect interval gives
another perfect one, while a major one becomes minor and a minor one
major. The numbers of an interval and its upside-down twin add up to
nine, their semitones to twelve: a major third (4) turned over is a
minor sixth (8), a perfect fourth (5) a perfect fifth (7). One semitone
more than major or perfect is *augmented*, one less than minor or
perfect *diminished*.

| Button | Name | Semitones | From C | A tune that starts with it |
|--------|------|-----------|--------|----------------------------|
| m2 | minor second | 1 | C–Db | Jaws, the shark's two notes |
| M2 | major second | 2 | C–D | Happy Birthday, at its first step up |
| m3 | minor third | 3 | C–Eb | Greensleeves |
| M3 | major third | 4 | C–E | When the Saints Go Marching In |
| P4 | perfect fourth | 5 | C–F | Here Comes the Bride |
| TT | tritone (augmented fourth) | 6 | C–F# | Maria, from West Side Story |
| P5 | perfect fifth | 7 | C–G | Twinkle, Twinkle, Little Star, at its first leap |
| m6 | minor sixth | 8 | C–Ab | The Entertainer, at its first leap up |
| M6 | major sixth | 9 | C–A | My Bonnie Lies over the Ocean |
| m7 | minor seventh | 10 | C–Bb | Somewhere, from West Side Story |
| M7 | major seventh | 11 | C–B | |
| P8 | octave | 12 | C–C | Over the Rainbow |

The tunes start with the interval going up, so the app names one only
when the question went up. The major seventh has none: few well-known
tunes start with it.

The **tritone** is the one interval with two numbers. Six semitones up
from C is the F# key or the Gb key: C to F# is four letters (C D E F),
an *augmented fourth*; C to Gb is five (C D E F G), a *diminished
fifth*. Same keys, same sound, two spellings. Its name says what it
is: three whole tones, exactly half an octave.

**Spelling.** The explanation writes the notes as a musician would.
The bottom note is a white key, or a black key written with a sharp or
a flat, whichever writes the *pair* with fewer of them: C# up to E is a
minor third (writing it Db up to Fb would take two flats), but Db up to
F is a major third (C# up to E# would take two sharps). When both are
as simple, a black key keeps its usual name: Bb up to Db, not A# up to
C#. The tritone is an augmented fourth or a diminished fifth by the
same rule: F up to B (no accidental) rather than F up to Cb, and an
augmented fourth when both are as simple: C up to F# rather than C up
to Gb, Bb up to E rather than Bb up to Fb or A# up to E.

### Chords

Every chord the exercise plays is built of thirds, stacked up from the
root. The nine kinds, on C:

| Button | Name | Symbol | Notes | Thirds, from the bottom |
|--------|------|--------|-------|-------------------------|
| maj | major | C | C E G | major, minor |
| m | minor | Cm | C Eb G | minor, major |
| dim | diminished | Cdim | C Eb Gb | minor, minor |
| aug | augmented | Caug | C E G# | major, major |
| 7 | dominant seventh | C7 | C E G Bb | major, minor, minor |
| maj7 | major seventh | Cmaj7 | C E G B | major, minor, major |
| m7 | minor seventh | Cm7 | C Eb G Bb | minor, major, minor |
| m7b5 | half-diminished seventh | Cm7b5 (Cø7) | C Eb Gb Bb | minor, minor, major |
| dim7 | diminished seventh | Cdim7 (C°7) | C Eb Gb Bbb | minor, minor, minor |

Two chords that are often taught early are **left out on purpose**:
sixth chords and suspended chords. C6 is C E G A, the very same keys as
Am7 (A C E G); Csus2 (C D G) has the keys of Gsus4 (G C D). Played on
their own, nothing tells them apart: what makes one C6 and the other
Am7 is the bass and the music around it. A quiz that asked them would
mark right answers wrong. A test checks that no two of the nine kinds
share their notes on any two roots.

Some spellings look odd at first: **Bbb** in Cdim7, **Cb** in Fdim. They
are not mistakes. The seventh of a C chord is some kind of B (C D E F G
A B, seven letters), and a diminished seventh is a semitone below the
minor seventh Bb, so it is written B double flat, though it is the A
key. In the same way the fifth of F is some kind of C, and a diminished
fifth is a semitone below C: Cb, the B key. The explanation says so
whenever it writes a note with a letter other than its key's usual
names, because the letter says which job the note does in the chord.

**Inversions.** A chord is *inverted* when a note other than its root
is lowest: the third (first inversion), the fifth (second inversion) or
the seventh (third inversion). The symbol names the bass after a slash:
C/E is C major with E lowest ([Chord names](chord-names.md),
[Inversions](inversions.md)). An
inverted chord keeps its kind, so the answer doesn't change, but it
sounds less settled and is harder to name; that is what the
**Inversions** box is for.

Two kinds are special. The augmented chord's notes are evenly spaced (a
major third apart, and a major third from the top note back up to the
root), and so are the diminished seventh's (a minor third apart). Put
any of their notes in the bass and you get the very same keys as the
chord in root position on *that* note: Caug/E is also Eaug (E G# B#,
where B# is the C key). The ear cannot tell which note is the root, so
the explanation of an inverted augmented or diminished seventh chord
says which other chord it also is. The kind is still certain, so the
question stays fair.

### Progressions

The numerals and their colours come from the harmony keys
([The harmony keys](../step-8/harmony-keys.md)): upper case for major
chords, lower case for minor, ° for diminished, and three jobs, home
(blue), away (green) and tension (orange). In C major:

```text
  I    ii    iii   IV    V     vi    vii°
  C    Dm    Em    F     G     Am    Bdim
```

A minor key reads `i ii° bIII iv V bVI bVII`. The numerals count along
the *major* scale from the same tonic, so the chords on the lowered
third, sixth and seventh notes get a flat: in A minor, C is bIII. And V
is major, as in nearly all music in minor keys: E major in A minor,
with G# pulling up to A. With **Sevenths** the keys become `Imaj7 ii7
iii7 IVmaj7 V7 vi7 viiø7` in major and `i7 iiø7 bIIImaj7 iv7 V7 bVImaj7
bVII7` in minor.

**How a progression is made.** It starts on I. Each next chord is
usually one of the moves the harmony keys suggest after the chord
before (the same list behind their faint yellow hints, [Now and
next](../step-8/harmony-keys.md#now-and-next)), the most usual move most
often. One time in four it is any chord chosen instead, so that chords
the usual moves rarely reach, such as iii, are heard too. It never
plays the same chord twice in a row, comes back to I only at the end,
and ends on I or V: a full stop, or a half cadence, a question waiting
for its answer.

**What the band plays.** Each chord lasts a bar of four beats. The
piano plays it on beats 1 and 3, each chord voiced close to the one
before, as a pianist's hands move ([Playing a chord: voice
leading](../step-8/harmony-keys.md#playing-a-chord-voice-leading)). The
bass plays the root on the same beats, in its low octave. The drums
keep time: bass drum on 1 and 3, snare on 2 and 4, hi-hat on every
eighth. The last chord is held for the whole bar over a crash. With
the **Piano** alone, the piano also plays the root under the chord:
without a bass, a close voicing could be heard as an inversion.

**Listening hints.**

- **Follow the bass.** It plays the roots, so its line *is* the
  progression. A root falling a fifth (V to I, ii to V) sounds like
  arriving.
- **I and V are the easiest pair.** V is tension, I is rest; V almost
  asks for I.
- **IV and ii both lead away and to V.** They share two notes (F A in C
  major: F A C and D F A), so they sound alike; ii is the darker one.
- **vi is a sad home.** It shares two notes with I (C E G and A C E),
  and so does iii (E G B). If a chord feels like home but minor, it is
  probably vi.
- **Hear the key** whenever you lose track of where home is.

## How it is built

The work splits along the lines the app already has. `lineup-core`
makes the questions and their explanations and knows nothing of
windows or sound. `lineup-audio` gains a way to play a few notes apart
from the song. `lineup-app` draws the window and joins the two.

### A question from a number

Every question comes from a 64-bit **seed**:

```rust
pub fn new(settings: &IntervalSettings, seed: u64) -> Self {
    let mut random = Random(seed);
    let intervals = settings.intervals.places(Interval::ALL.len());
    let directions = settings.directions.places(Direction::ALL.len());
    let interval = Interval::ALL[intervals[random.below(intervals.len())]];
    let direction = Direction::ALL[directions[random.below(directions.len())]];
    let low = Note::new(INTERVAL_LOWEST + random.below(INTERVAL_SPAN) as u8);
    IntervalQuestion { low, interval, direction }
}
```

`Random` is the small SplitMix64 generator the twelve-tone rows already
shuffle with ([Twelve-tone rows](../step-8/twelve-tone.md)): the same
seed always gives the same numbers, so the same question. That makes
the tests simple, because they can ask thousands of questions and check
each one, and a failing seed can be replayed exactly. The window takes
its first seed from the clock and adds a large odd constant for every
new question. If the question it gets is the same as the last one, it
tries again, up to eight times, so the same question rarely comes
twice in a row.

### A set of answers in sixteen bits

Which intervals or chords are asked is a set of places in a list. The
list is short (twelve intervals at most), so the set is a `u16`, one bit
per place, as the pitch-class sets were twelve bits
([Sets in bits](../step-8/twelve-tone.md#sets-in-bits)):

```rust
pub struct Choices(u16);

impl Choices {
    pub fn has(self, place: usize) -> bool {
        self.0 & (1 << place) != 0
    }

    /// Adds the answer at `place`, or takes it away. The last one left
    /// stays: there is always something to ask.
    pub fn toggle(&mut self, place: usize) {
        let toggled = self.0 ^ (1 << place);
        if toggled != 0 {
            self.0 = toggled;
        }
    }
}
```

`^` (exclusive or) flips one bit. Refusing to flip the last one off
means a question can always be asked, without a special case anywhere
else.

### Cues: what to play, in seconds

A question doesn't play itself: it returns **cues**, plain values that
say who plays what, when and how loud.

```rust
pub struct Cue {
    pub part: Part,      // Piano, Bass or Drum(Drum)
    pub start: f64,      // seconds from the start of the question
    pub length: f64,
    pub note: Note,
    pub velocity: u8,
}
```

Keeping sound out of `lineup-core` means the tests can check *what*
would be played without playing it: that every bar of a progression
plays its chord over its root, that the piano alone adds the root,
that a chord question's lowest note is the bass its symbol names.

The answers to compare with use the same idea. `cues_for(interval)`
plays another interval from the same bottom note (or, going down, to
the same top note), and `cues_as(quality)` another kind of chord on the
same root with the same chord note in the bass. So when you click a
wrong answer after answering, you hear exactly what you said next to
what was played.

### A phrase on the audio thread

The song is made of patterns and clips on a timeline. A question isn't:
it is a few notes that start *now*, on instruments of their own, while
the song's channels stay as they are. So `lineup-audio` gets a new
type, a `Phrase`: its instruments and its notes, with their times in
frames.

As always, the audio thread must not allocate or free memory
([The audio thread, grown up](../step-2/realtime.md)). So the UI
thread builds the whole phrase, instruments included, and sends it in
one command:

```rust
pub enum Command {
    // ...
    PlayPhrase(Box<Phrase>),
    StopPhrase,
}
```

The `Box` matters. A `Command` is as big as its biggest variant, and a
test keeps them at 80 bytes or less so the queue stays small. A
`Phrase` has two `Vec`s and a few counters, around 80 bytes of its own;
boxed, the command carries only a pointer.

On the audio thread the phrase only starts each note when its time
comes and adds its instruments to the mix:

```rust
pub fn next_sample(&mut self) -> f32 {
    while let Some(note) = self.notes.get(self.next)
        && note.start <= self.clock
    {
        if let Some(instrument) = self.instruments.get_mut(note.instrument) {
            instrument.note_on(note.pitch, note.velocity, note.gate, self.sample_rate);
        }
        self.next += 1;
    }
    self.clock += 1;
    let mix: f32 = self.instruments.iter_mut().map(Instrument::next_sample).sum();
    mix * VOLUME
}
```

The notes are sorted by start, so `next` only ever moves forward: one
comparison per sample when nothing starts. `while let ... && ...` is a
let chain in a loop: it goes on while there *is* a next note *and* it
is due. Each note is given its length (`gate`) when it starts, and the
instrument lets go of it by itself, so the phrase doesn't need a list
of notes to stop.

When a new phrase arrives, the old one goes back to the UI thread
through the trash queue to be freed. A phrase that has played its last
note waits two seconds, for the sound to ring out, and then goes the
same way. **Stop** (closing the window) lets go of every note and drops
the ones not started yet.

The instruments come from `build_instrument`, the same function the
channels use ([app/ear.rs](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/ear.rs)):
the piano is a channel with General MIDI program 0, the bass program
33, the drums the built-in kit. So with **Sounds > SoundFont
instruments** on, the questions play the SoundFont's piano, bass and
drums, like the band ([Real instrument sounds](soundfont.md)).

### The window: actions out

The window doesn't hold the audio engine. It returns what it wants to
hear as a list of actions, and the app plays them:

```rust
pub enum EarAction {
    Play(Vec<Cue>),
    Stop,
}
```

Inside, the question and the answers given so far are one enum,
`Asked`. Drawing the answers needs to change them *and* call methods of
the window (to add to the score, to play a sound), which Rust doesn't
allow while the answers are borrowed out of `self`. The window takes
the question out of `self` for the time it is drawn, and puts it back:

```rust
match self.asked.take() {
    Some(Asked::Interval { question, mut answer }) => {
        self.interval_answers(ui, &question, &mut answer, actions);
        self.asked = Some(Asked::Interval { question, answer });
    }
    // ... chords and progressions the same way
    None => {}
}
```

`Option::take` leaves `None` behind, so for those few lines `self` and
the question are two separate values, and both can be borrowed freely.

The window's choices are remembered the way the Sounds menu's are:
a `#[serde(default)]` struct kept in egui's memory, which eframe saves
with the window's size.

## How it was tested

The core has 21 tests. The ones that guard the theory:

- **Every interval from every note.** For all 12 bottom notes and all
  12 intervals, the two written notes are the right keys, neither needs
  a double sharp or flat, and the name worked out from the two written
  notes (by `spelled_interval_name`, from their letters and semitones)
  is the interval's name; for the tritone, augmented fourth or
  diminished fifth.
- **Spellings by hand**: C# E, Db F, F B, E A#, C F#, E C, Eb D, F# F#,
  Bb Db, Bb E, Ab Eb.
- **Chords against the analyzer.** Five hundred chord questions in root
  position are fed to the analyzer the app uses to name what you play
  on a MIDI keyboard, which must name the same kind. (Not inverted:
  the analyzer rightly hears Am7/C as C6.)
- **Every chord question plays its chord with the right bass**, over
  2,000 seeds with inversions on.
- **Spelled explanations**: Bdim7, Ebmaj7/G, Fdim/Cb, Cdim7's Bbb,
  Caug/E (also Eaug), Ddim7/Cb (also Bdim7).
- **Numerals** match the harmony keys in major and minor, triads and
  sevenths.
- **Progressions** start on I, never repeat a chord, return to I only
  at the end, end on I or V, and use every chord chosen; every bar
  plays its own chord over its root.
- **Every key, root and bottom note comes up**: 24 keys, 12 roots, 16
  bottom notes.

Three tests in the audio engine check that a phrase plays with no song
loaded and is thrown away after it rings out, that a new phrase takes
the place of the last, and that a stopped phrase plays nothing more.
In the running app, a recording of the output after **Start** held two
notes, D4 and then F4: the minor third the window was asking.

## Exercises

1. **A tune for the major seventh.** The table has none. Find a song
   whose melody starts with a major seventh going up, check it on a
   piano, and add it to `Interval::tune`. Does the test
   `going_up_names_a_tune` need to change?
2. **Descending tunes.** Tunes help going down too: the start of
   Beethoven's Fifth Symphony, or "Swing Low, Sweet Chariot", for a
   major third down. Add a `tune_down` and use it in `explain` when the
   question went down.
3. **Sixth chords, fairly.** Sixth chords are left out because C6 and
   Am7 share their keys. They could be asked if the chord always
   played over its root in the bass, an octave below. Which notes would
   `notes_as` give then? Would inversions still make sense?
4. **What you get wrong.** The score counts right answers. Keep, for
   each interval, how often it was asked and how often it was right,
   and ask the weak ones more often. Where would the counts live, and
   how would `IntervalQuestion::new` use them without losing the
   "same seed, same question" rule?
5. **Your own progression.** Let the progression exercise play the
   chords of the song's chord sheet, with the names hidden, instead of
   a made-up one. What would it need from `chord_sheet.rs`?
