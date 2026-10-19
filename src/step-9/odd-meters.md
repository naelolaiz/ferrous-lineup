# Odd meters: 5/4, 7/8 and the dances of the Balkans

Files: [`crates/lineup-core/src/meter.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/meter.rs),
[`crates/lineup-core/src/library/comping.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/comping.rs),
[`crates/lineup-core/src/library/rhythm.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/rhythm.rs),
[`crates/lineup-core/src/library/style.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/style.rs),
[`crates/lineup-core/src/library/chart.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/chart.rs),
[`crates/lineup-core/src/library/song.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/song.rs),
[`crates/lineup-core/src/library/song/balkan.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/song/balkan.rs),
[`crates/lineup-core/src/library/song/classical.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/song/classical.rs)
and [`crates/lineup-core/src/library/demo.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/demo.rs)

The app could already be set to 5/4 or 7/8 by hand (see
[Free time signatures](../step-8/free-time-signatures.md)), but the
library had nothing to play in them: changing a song's Time to 7/8
gave it a plain "basic beat", and no demo was in an odd meter. This
change adds **eleven rhythms and harmony styles** in odd meters, a way
for the band to **play** them, and songs to hear them in:
**Take Five** in 5/4; **Ajde Jano**, **Jovano Jovanke**, **Samiotissa** and the
**Svatbarska rachenitsa** in 7/8; **Paidushko horo** in 5/8; **Daichovo horo** and
**Entarisi ala benziyor** in 9/8; **Gankino horo** in 11/8; **Bučimiš** in 15/8; and
Mussorgsky's **Promenade**, whose meter changes from bar to bar.

## Trying it

Open **File > Demos > Songs > Balkan and Turkish > Ajde Jano** and
press Play. Count along with the drums: **1 2 3, 1 2, 1 2**. The bar
has seven eighth notes, but you do not count seven: you feel three
beats, a long one and two short ones. The big drum strikes the long
beat; the bass plays a note on every beat and the accordion plays its
chord on the eighths in between, "oom-pa-pa, oom-pa, oom-pa".

Now open **Take Five** (in Jazz). The bar has five quarter notes, and
the piano's vamp groups them **1 2 3, 1 2**: E flat minor for three
beats, B flat minor seven for two.

Then open any song in 4/4, say **Old MacDonald Had a Farm**, open the
**Time** list, keep **Use matching style** ticked and click **7/8**.
Before, the song got a basic beat; now it gets a **rachenitsa**, and the
Time list shows `2+2+3/8`: Bulgaria's way of counting seven.

## The music

### Long beats and short beats

In 4/4 or 3/4 every beat is as long as every other. In 6/8 the six
eighths make two beats of three. Many dances of the Balkans, Greece
and Turkey count their eighths in beats of **two and three mixed**:

| Meter | Counted | Dance |
|---|---|---|
| 5/8 | 2 + 3, quick, slow | Bulgarian paidushko |
| 7/8 | 3 + 2 + 2, slow, quick, quick | Greek kalamatianos, Macedonian lesnoto |
| 7/8 | 2 + 2 + 3, quick, quick, slow | Bulgarian rachenitsa |
| 9/8 | 2 + 2 + 2 + 3 | Bulgarian daichovo, Thracian karşılama |
| 9/8 | 2 + 3 + 2 + 2 | Turkish raks aksağı |
| 11/8 | 2 + 2 + 3 + 2 + 2 | Bulgarian kopanitsa (Gankino horo) |
| 15/8 | 2 + 2 + 2 + 2 + 3 + 2 + 2 | Bulgarian bučimiš |

A long beat is **half as long again** as a short one, three eighths
against two, so a dancer's steps go long, short, short, or short,
short, long: the dance seems to limp. Turkish musicians call these
meters **aksak**, "limping". The same 7/8 can limp two ways, which is
why the app writes the grouping in the time signature: `3+2+2/8` is a
kalamatianos, `2+2+3/8` a rachenitsa. Write the time signature the
way a dancer counts it, because the drums, the bass and the chords
all follow the grouping.

Jazz took the idea further in 1959, on the Dave Brubeck Quartet's
album *Time Out*: **Take Five** is in 5/4, counted 3 + 2, and Brubeck
wrote **Blue Rondo à la Turk** in a 9/8 he heard Turkish street
musicians play.

### Meters that change from bar to bar

Some music changes meter as it goes. **Mussorgsky's Promenade**, the
walk between the paintings of *Pictures at an Exhibition* (1874), puts
a bar of 5/4 next to a bar of 6/4: the tune is phrased like speech, or
a Russian folk song, not like a march. Its beats are all equal, five
and then six quarter notes, each leaned on: there is no long beat and
short beat here, only bars of different lengths. In the app each bar of such a
part has its own time signature, the demo puts a **time marker**
wherever it changes (look at the ruler), and the drums and the band
follow every one.

### The modes of the Balkans and Turkey

**Entarisi ala benziyor** is a Turkish folk song in **makam hicaz**.
A makam is more than a scale (it has its own way of moving and of
resting), but its notes on A are A, B flat, C sharp, D, E, F, G. The
step and a half between B flat and C sharp is what makes it sound
"eastern". Those are the notes of the Phrygian mode with its third
raised, the same scale as flamenco's, so the demo's key is A Phrygian.
**Jovano Jovanke** sings the same scale on E: E, F, G sharp, A, B, C,
D, and its first phrase falls G sharp, F, E across that step and a
half.
Turkish music divides the whole tone into nine **commas**; the B flat
and C sharp of hicaz are four commas from B and C, close enough to a
piano's semitone.

Turkish folk songs are sung and played in unison, over a drum. The
chords of this demo are **added**: D minor where the tune runs between
D and F, A major where it rests on C sharp and A, B flat major under
its long B flats.

**Gankino horo** has another colour: D, E, F, G sharp, A, B, C, a minor
scale with its fourth raised and a B natural, the Dorian mode with a
sharp fourth (Turkish musicians call it nikriz). The G sharp rubs
against the D minor chord and leads to E major (II), where each strain
stops.

### Triplets

Some tunes need **triplets**: three notes in the time of two. Jovano
Jovanke has one in its first line, F, E, F in the time of a quarter. A melody can now write
`(3` before three notes: `(3 G4:2 G4 G4` is three eighths in the time
of a quarter. Their starts fall between the sixteenth-note steps of
the grid, so they are placed to the tick.

## The code

### A time signature written as a constant

Songs and rhythms are `const` tables, built before the program runs.
`TimeSignature::grouped(&[3, 2, 2], 8)` returns an `Option`, which a
constant cannot unwrap. A second constructor is a **`const fn`**, a
function Rust can run at compile time:

```rust
pub const fn of_groups(groups: &[u8], unit: u8) -> Self {
    let mut mask = 0_u64;
    let mut at = 0;
    let mut index = 0;
    // `for` loops are not allowed in a `const fn` yet: `while` is.
    while index < groups.len() {
        assert!(groups[index] > 0, "a group of no notes");
        mask |= 1 << at;
        at += groups[index] as usize;
        index += 1;
    }
    ...
}

const THREE_TWO_TWO: TimeSignature = TimeSignature::of_groups(&[3, 2, 2], 8);
```

The grouping is a **bit mask**: bit `n` is set when a beat starts on
note `n`. For 3+2+2 that is bits 0, 3 and 5. An `assert!` in a `const
fn` that fails stops the **build**, not the program: a mistyped
grouping never reaches a user.

### The beats a player feels

`felt_groups()` says how a bar is felt: its grouping if it has one,
three-eighth beats in 6/8, and otherwise twos with a three at the end,
so an ungrouped 7/8 is felt 2 + 2 + 3 and 5/4 as 2 + 3.

### A band that limps

The piano's comping patterns were tables of hits that repeat every so
many steps, whatever the bar: that works in 4/4, but a pattern of two
beats would drift across a 7/8 bar. The new `Comping::Aksak` and
`Bass::Aksak` are **made from the bar**: the bass on the first eighth
of every felt beat, the chord on the others.

```rust
fn hits(self, time: TimeSignature) -> (usize, Cow<'static, [Hit<Voices>]>) {
    match self {
        Comping::Aksak => {
            let unit = time.unit_steps();
            // A chord a little shorter than its note, so the next
            // one is heard to start, but never under a step.
            let length = (unit - unit / 4).max(1);
            let mut hits = Vec::new();
            let mut at = 0;
            for group in time.felt_groups() {
                for note in 1..usize::from(group) {
                    let velocity = if note == 1 { 84 } else { 74 };
                    hits.push(hit!(at + note * unit, length, Voices::All, velocity));
                }
                at += usize::from(group) * unit;
            }
            (time.steps_per_bar(), Cow::Owned(hits))
        }
        _ => {
            let (period, hits) = self.pattern();
            (period, Cow::Borrowed(hits))
        }
    }
}
```

**`Cow`** ("clone on write") holds either a borrowed value or an owned
one. The old patterns are `'static` tables, so they are borrowed for
free (`Cow::Borrowed`); the aksak pattern is built for each time
signature, so it is owned (`Cow::Owned`). The code that plays the hits
reads both the same way. To build it, `accompany`, `Comping::play`
and `Bass::play` now take the time signature, and an aksak pattern
counts from the **bar line**, not from the chord: a chord that starts
on the second beat of a 7/8 bar still gets its bass on beat two.

### A part whose meter changes

A song's `Part` gets a list of meters, one per bar, and
`Song::part_runs` cuts the part into **runs** of bars in one meter,
each with its share of the melody and the chords. A pattern has one
time signature, so the demo makes a pattern per run, named "Promenade,
bar 2" and so on, and puts a time marker where each starts.

### Triplets in the melody

`parse_melody` used to count in steps. It now counts in **ticks**, 24 to
a step, so a triplet eighth is 32 ticks long (two steps' worth, 48,
times two thirds). A note is made with `NoteEvent::exact`, which places
it on any tick.

### The Time list picks a limping rhythm

`Rhythm::for_time` looks for a rhythm in the new time signature. 7/8
written without a grouping matched none, so the song got a basic beat.
Now, for an odd meter without a grouping, it looks among the rhythms
with **as many notes to the bar**: one of the song's own kind first
(a Balkan song in 7/8 gets a kalamatianos, its first one), then one
grouped the way the meter is felt (7/8: 2+2+3, a rachenitsa), then any.
9/8 is not odd in this sense: written plainly it means three beats of
three, so it does not get a daichovo.

### How the songs were written down

Each song has a `// Source:` comment with the transcription its notes
come from: ABC files of the folk-dance collections of John Chambers and
Henrik Norbeck, Paul Desmond's alto saxophone part for Take Five, the
Mutopia Project's LilyPond engraving of Pictures at an Exhibition, and
the SymbTr corpus of Turkish makam music. Where a source has no chords,
the comment says the chords are a plain harmonization. Many Balkan dances are
written in sixteenths (7/16, 11/16); here they are written in eighths,
every note twice as long, which sounds the same at the tempo given.

### Copyright

The folk songs and dances are traditional, and Mussorgsky's music is long in the public domain. **Take Five** (Paul
Desmond, 1959) is still under copyright everywhere; it is included at
the owner's request, and a comment above it says so. Blue Rondo à la
Turk was asked for too, but no written score of it could be reached.

## Exercises

1. Open **Ajde Jano** and, in the Time list, type `7` over `8`
   grouped as `2+2+3` and click **Use 2+2+3/8**. The melody
   keeps its notes, but the band now leans on other eighths. Which
   beats of the tune now land between the bass notes?
2. In the Library, play the **Kalamatianos** and the **Rachenitsa**
   rhythms one after the other. Clap along: where is the long beat in
   each?
3. Pick `5/8` in the Time list of a song in 4/4, with **Use matching
   style** ticked. Which rhythm does it get, and how does the Time list
   write the meter?
4. Write a style of your own in `style.rs`: `Comping::Aksak` and
   `Bass::Aksak` in `TimeSignature::of_groups(&[3, 3, 2], 8)`, 8/8
   counted like a tresillo. Does `every_style_is_heard_as_its_chart`
   still pass?
