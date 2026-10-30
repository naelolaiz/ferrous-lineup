# The library: styles and rhythms as data

Files: [`crates/lineup-core/src/library/`](https://github.com/naelolaiz/ferrous-lineup/tree/main/crates/lineup-core/src/library),
[`crates/lineup-core/src/sources.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/sources.rs),
[`crates/lineup-app/src/library_browser.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/library_browser.rs)
and [`crates/lineup-app/src/app/library.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/library.rs)

A musician who sits down to play a bossa nova knows four things: the
chords, how the guitar plays them, what the bass does, and the drum
pattern. None of it is code. It is *knowledge*, and the best way to put
knowledge into a program is to write it down as data, in a form a
person can read and check, and to keep the code that uses it small.

The library does that: 177 harmony styles, 112 rhythms and 6 fills,
each a few lines in a table, plus the twelve-tone and micromode styles
of the theory chapters and the songs of [How a demo is
written](../songs/demos.md). This chapter goes through the notations,
then follows a style from the table into a song: which channel plays
what, how a whole song changes style, how each part of the song
remembers where it came from, and the Library tab that lists it all.
How to use the Library is in the manual's [Styles and the
Library](https://naelolaiz.github.io/ferrous-lineup/manual/styles.html).

## Chord charts: chords by the bar

A progression written as chord names, "C G Am F", only works in C.
Written as Roman numerals, "I V vi IV", it works in every key (see
[Reading a numeral back](../theory/numerals.md#reading-a-numeral-back)).
A **chord chart** writes the numerals bar by bar, the way a lead sheet
does, with `|` between bars:

```text
I | IV V | vi | ii7 V7
```

The chords in a bar share it equally, so "IV V" is half a bar each. A
chord that repeats the one before is held on, not played again: "I | I"
is two bars of one chord. `chart::parse_chart(text, bar)` turns a chart
into `ChordSpan`s, steps of a bar of `bar` steps:

```rust
pub struct ChordSpan {
    pub chord: RomanChord,
    pub start: usize,
    pub length: usize,
    /// Whether the band plays it early (see [`Push`]).
    pub push: Option<Push>,
    /// Whether the band stops playing its style here (see [`Stop`]).
    pub stop: Option<Stop>,
}
```

It returns `None` when a chord cannot be read, a bar is empty, or a bar
cannot be shared equally between its chords: a table entry that is
wrong fails a test instead of playing nonsense. `push` and `stop` are
the marks of [Shots, holds, rests and pushes](stops.md).

## Comping: how the chords are played

The same four chords can be a ballad, a reggae tune or a house track.
What changes is the rhythm the chords are struck in, which jazz
musicians call **comping** (short for accompanying), and what the bass
does. Each way of comping is a short table of **hits**: when a note
starts, how long it lasts, which notes of the chord it plays, and how
hard.

```rust
struct Hit<T> {
    /// When it starts, in steps from the start of the pattern's period.
    at: usize,
    /// How long it lasts, in steps.
    len: usize,
    what: T,
    velocity: u8,
}
```

For chords, `what` is a `Voices`: `All` the notes, `Upper` (every note
but the lowest, the lighter up-stroke of a strum) or `Nth(n)`, one note
counted from the bottom, for arpeggios. For the bass it is a `Tone`:
the root, the fifth, the octave, the third, the seventh, the next
chord's root played early (an **anticipation**) or the note a semitone
from it, leading in. One generic struct serves both, with `T` filled in
twice.

```rust
Comping::Pulse => (4, &[hit!(0, 2, All, 96), hit!(2, 2, All, 78)]),
Comping::Quarters => (4, &[hit!(0, 2, All, 100)]),
Comping::Offbeat => (4, &[hit!(2, 1, All, 92)]),
```

The first number is the **period**: the hits repeat every 4 steps, a
beat. `Pulse` is steady eighth notes, the second one softer; `Offbeat`
is the reggae skank, one short chord on the second eighth of every
beat. The habanera's long, short, long, long takes 8 steps:

```rust
Comping::Habanera => (
    8,
    &[
        hit!(0, 3, All, 96),
        hit!(3, 1, Upper, 74),
        hit!(4, 2, All, 86),
        hit!(6, 2, All, 84),
    ],
),
```

### `macro_rules!`, and why not a function

`hit!` is a small **declarative macro**:

```rust
macro_rules! hit {
    ($at:expr, $len:expr, $what:expr, $velocity:expr) => {
        Hit {
            at: $at,
            len: $len,
            what: $what,
            velocity: $velocity,
        }
    };
}
```

A macro is code that writes code. `macro_rules!` matches what it is
given against a pattern, here four expressions (`$at:expr` and so on),
and pastes them into the template on the right. `hit!(0, 2, All, 96)`
becomes `Hit { at: 0, len: 2, what: All, velocity: 96 }` before the
compiler checks anything.

Why not a `const fn hit(...) -> Hit`? Look at what the `match` returns:
`&'static [Hit<Voices>]`, a borrowed slice that lives for the whole
program. Writing `&[ ... ]` of plain values gives exactly that: the
compiler sees an expression made only of constants, puts it in the
program's read-only data, and lets the borrow live for ever. That is
called **constant promotion**. It only happens for plain values: a
*function call* inside the brackets, even a `const fn`, is not
promoted, and the compiler says the borrowed temporary is dropped at
the end of the statement. The macro keeps the table short to write and
still made of plain values. The comment above the macro says so, so
nobody "simplifies" it into a function.

## Styles: a table in a `static`

A style puts it together: a chart in a mode, a way of comping, a bass
line, a usual tempo and the rhythm that goes with it.

```rust
pub struct Style {
    pub name: &'static str,
    pub genre: Genre,
    /// The mode the chords are written in: the project's scale becomes
    /// this mode when the style is used.
    pub mode: ScaleKind,
    pub time: TimeSignature,
    /// The progression, as a chord chart (see [`chart::parse_chart`]):
    /// numerals counted from the tonic, bars separated by `|`.
    pub chart: &'static str,
    pub comping: Comping,
    pub bass: Bass,
    /// The tempo the style is usually played at, in BPM.
    pub tempo: f32,
    /// The name of the rhythm in [`super::RHYTHMS`] that goes with it, or
    /// "" for none.
    pub rhythm: &'static str,
    /// One or two sentences on what to listen for.
    pub about: &'static str,
}
```

Every field is `&'static str`, `Copy` or a small enum, so a `Style` is
`Copy` and the table needs no allocation at all. An entry reads like a
sentence:

```rust
style(
    "Four chords",
    Pop,
    Major,
    "I | V | vi | IV",
    Comping::Pulse,
    Bass::Pulse,
    110.0,
    "Pop beat",
    "The four chords behind hundreds of hits: home, the dominant, the sad relative minor, \
     and the subdominant.",
),
```

`style` is a `const fn` that fills in 4/4, with
`#[allow(clippy::too_many_arguments)]` because nine arguments in a
table row are clearer than nine field names. The styles in another
time signature add `.in_time(THREE_FOUR)`, another `const fn`, which
uses **struct update syntax**:

```rust
const fn in_time(self, time: TimeSignature) -> Style {
    Style { time, ..self }
}
```

`..self` means "every other field as in `self`". The table itself is a
`static`:

```rust
pub static STYLES: &[Style] = &[
```

A `const` is pasted into every place that uses it, like a macro; a
`static` is one value at one address for the whole program. For a big
table used in many places, `static` keeps one copy, and a
`&'static Style` pointing into it is as cheap to pass around as a
number and can never dangle.

`Style::render(tonic)` plays a style in any key: the chart becomes
spans, each numeral a chord in the key, the chords are voiced one after
another, moving as little as possible, and the comping and the bass
play them. That is the band's own work, in [How the band plays a chord
sheet](band.md).

## Rhythms, one line per drum

Drummers and drum machines think of a beat as a grid: one row per
drum, one column per step. The library writes exactly that, as text:

```rust
rhythm(
    "Pop beat",
    Pop,
    110.0,
    &[
        (Kick, "X... .... X.x. ...."),
        (Snare, ".... X... .... X..."),
        (HiHat, "x.x. x.x. x.x. x.x."),
    ],
    "Snare roll",
    "The backbeat: kick on 1 and 3, snare on 2 and 4, hi-hat on every eighth note.",
),
```

Each character is a step. `X` is a loud hit, `x` a normal one, `o` a
soft **ghost note** and `.` silence. `h` and `l` play the drum five
semitones higher or lower, for the open and muted tones of a conga or
two toms. Spaces only make the beats easy to see, and `|` separates
bars, for rhythms longer than one. "Snare roll" names the **fill**, a
one-bar break that ends a section.

One character becomes one hit:

```rust
fn hit(character: char) -> Option<Option<Hit>> {
    match character {
        '.' => Some(None),
        'X' => Some(Some((120, 0))),
        'x' => Some(Some((96, 0))),
        'o' => Some(Some((56, 0))),
        'h' => Some(Some((96, 5))),
        'l' => Some(Some((96, -5))),
        _ => None,
    }
}
```

An `Option` inside an `Option` says two different things: the outer
`None` is "this is not a character a rhythm may hold", so a line with a
typo is refused; the inner `None` is "a step of silence", a perfectly
good answer. Here `Hit` is a `type` alias, `(u8, i32)`: a velocity and
a shift in semitones.

The same two letters write the Turkish usuls of the Balkan rhythms. On
the darbuka, `l` is the low stroke, **düm**, and `h` the high one,
**tek**, and the comments cite where each pattern comes from:

```rust
rhythm(
    "Aksak",
    Balkan,
    90.0,
    &[
        (Kick, "X... .... X... ......"),
        (Conga, "l... h.h. l... h...h."),
        (Shaker, "x.o. x.o. x.o. x.o.o."),
    ],
    "",
    "The Turkish aksak, 9/8 counted 2 + 2 + 2 + 3, on the darbuka: düm, te-ke, \
     düm, tek-tek, the low düm on the first and third beats, the long last beat \
     split two and one.",
)
.in_time(TimeSignature::of_groups(&[2, 2, 2, 3], 8)),
```

When a rhythm goes into a pattern, it is laid out a bar at a time in
the pattern's time signature:

```rust
for start in (0..length).step_by(bar) {
    let steps = &bars[(start / bar) % bars.len()];
    for (step, hit) in steps.iter().enumerate().take(bar) {
```

`step_by(bar)` gives the start of every bar, and `take(bar)` stops a
long bar at the bar line, so a 4/4 beat in a 3/4 pattern loses its last
beat in each bar instead of drifting across the bar lines. A few
rhythms, the ones marked `follows_the_meter`, are written again for
every bar's own meter instead, for music whose meter changes all the
time (see [An orchestra for music that changes
meter](orchestra.md)).

### A rhythm for any time signature

When you change a song's time signature by hand, it needs a rhythm in
the new one. `Rhythm::for_time` asks, in order, until something
answers:

```rust
RHYTHMS
    .iter()
    .filter(in_time)
    .find(same_kind)
    .or_else(|| {
        USUAL
            .iter()
            .filter_map(|name| Rhythm::named(name))
            .find(in_time)
    })
    .or_else(|| RHYTHMS.iter().find(in_time))
    .or_else(|| Rhythm::grouped_for(time, same_kind))
    .unwrap_or_else(|| Rhythm::basic(time))
```

A rhythm of the same kind in the new time (a tango in 3/4 is a tango
waltz); else the usual one (a waltz for 3/4); else any in that time;
else one with as many of the same notes to the bar grouped its own way
(7/8 plays as 2+2+3/8, a rachenitsa); else a plain beat made for that
time signature. Each `or_else` runs only if everything before it found
nothing. How the song's notes move into the new meter is in [Putting a
song in another time signature](../sound/tempo.md#putting-a-song-in-another-time-signature).

That last plain beat has a problem: a `Rhythm` holds `&'static str`
lines, and 5/4 is in no table. `Rhythm::basic` makes the text when it
is first asked for, and keeps it for as long as the program runs:

```rust
pub fn basic(time: TimeSignature) -> &'static Rhythm {
    static MADE: Mutex<Vec<&'static Rhythm>> = Mutex::new(Vec::new());
    let mut made = MADE.lock().unwrap_or_else(PoisonError::into_inner);
    if let Some(beat) = made.iter().find(|beat| beat.time == time) {
        return beat;
    }
    let lines: Vec<(Drum, &'static str)> = basic_lines(time)
        .into_iter()
        .map(|(drum, line)| (drum, &*String::leak(line)))
        .collect();
```

`String::leak`, `Vec::leak` and `Box::leak` give up ownership of their
memory on purpose: it is never freed, and in exchange you get a
`&'static` reference to it. Leaking every time would grow without end,
so the beats made are kept in a `static Mutex<Vec<...>>`, and each time
signature is made once: a handful of small strings for the life of the
program. `Mutex::new` is a `const fn`, which is what lets a `static`
hold one. A poisoned lock (a thread panicked while holding it) still
holds good data here, so `unwrap_or_else(PoisonError::into_inner)` takes
it anyway.

## Where the notes go

The tables know nothing about channels. `library/apply.rs` decides
where their notes go: chords to a synth called "Keys" (or the channel
you selected), the bass line to a synth called "Bass", the melody of a
song to "Lead", and each drum to the channel that plays it. Missing
channels are added, with a piano for the Keys and a finger bass for the
Bass as their General MIDI sounds.

```rust
pub fn chords_channel(project: &mut Project, selected: Option<usize>) -> Option<usize> {
    let free = |(index, channel): (usize, &Channel)| {
        plays_notes(channel)
            && channel.name != BASS
            && channel.name != LEAD
            && !project.is_locked(index)
    };
    if let Some(index) = selected
        && project
            .channels()
            .get(index)
            .is_some_and(|channel| free((index, channel)))
    {
        return Some(index);
    }
    let other = project
        .channels()
        .iter()
        .enumerate()
        .position(|(index, channel)| channel.kind == ChannelKind::Synth && free((index, channel)));
    find_synth(project, KEYS)
        .or(other)
        .or_else(|| synth_named(project, KEYS))
}
```

`free` is a closure kept in a variable, so it can be used twice. A
**locked** channel is never picked for chords (see [Song form,
sections, variations and locks](song-form.md)). The chain at
the end shows the difference between `or` and `or_else`: `other` is
already worked out, so `or` takes it as a value; `synth_named` *adds a
channel*, so it must only run if nothing else was found, and goes in a
closure with `or_else`.

Drums are found by what they are, not by name: the channel whose kind
is `Sampler(SampleSource::Builtin(drum))`. A kick renamed "Bombo" is
still the kick. A synth called "Bass" sent to an external synth over
MIDI is still the bass, since `plays_notes` accepts MIDI Out channels
too (see [MIDI](../sound/midi.md)).

## Playing a song in another style

Trying a style in a pattern of its own is easy. Changing the style of
a whole song is harder: "let's do this one as a reggae" means the same
chords, played another way by everyone. But a pattern does not store
its chords; it stores notes. So the library first **hears** them, with
the second ear of [Hearing
harmony](../theory/hearing.md#a-second-ear-for-the-bands-own-patterns),
and then plays them again.

A rhythm says how it likes its chords played through the style whose
drums it is:

```rust
pub fn feel(rhythm: &Rhythm) -> (Comping, Bass) {
    let same_time = |style: &&Style| style.time == rhythm.time;
    STYLES
        .iter()
        .find(|style| style.rhythm == rhythm.name)
        .or_else(|| {
            STYLES
                .iter()
                .filter(same_time)
                .find(|style| style.genre == rhythm.genre)
        })
        .or_else(|| STYLES.iter().find(same_time))
        .map_or((Comping::Pulse, Bass::Pulse), |style| {
            (style.comping, style.bass)
        })
}
```

The "Salsa" rhythm is the drums of the "Salsa montuno" style, so it
plays its chords as a montuno over a tumbao bass. A rhythm no style
uses borrows a style of its genre in its time signature, so a rhythm in
3/4 never gets a way of playing meant for 4/4. Why `&&Style`?
`STYLES.iter()` yields `&Style`, and `filter` hands its closure a
reference to each item: a reference to a reference.

`recomp` plays one pattern again: it hears the chords of its chords
channel over its bass, voices them, and lets the new comping and bass
play each stretch. Stretches where no chord was heard keep their old
notes, so an intro without chords stays as it was. `play_song` does it
for every pattern, and leaves three things alone on purpose: the
melody, the patterns of the band (which follow the chord sheet, see
[How the band plays a chord sheet](band.md)), and endings, a
pattern whose notes all start on its first step, like a final held
chord with a crash. Where the old drums had a fill in their last bar,
the new rhythm's fill goes there. A fill is a last bar that plays like
no bar before it:

```rust
// A fill is a last bar that plays like no bar before it.
let fill = length >= 2 * bar && {
    let last = drums_in(project, pattern, length - bar, bar);
    (0..length - bar)
        .step_by(bar)
        .all(|start| drums_in(project, pattern, start, bar) != last)
};
```

A block `{ ... }` is an expression in Rust, whose value is its last
line, so it can sit after `&&`; and since `&&` stops as soon as its left
side is `false`, the block only runs for patterns of two bars or more.
A rhythm in another time signature first re-meters the whole song,
bar for bar, so a 4/4 song played as a waltz becomes a song in 3/4.

### Which parts to take

The window a style opens in the app asks which parts to take, in three
groups: under **Accompaniment**, **Drums**, **Chord and bass parts**
and **Voices / instruments**; under **Harmony**, **Chord progression**;
under **Timing**, **Time signature**, **Tempo** and **Swing**. They are
one small struct:

```rust
pub struct StyleParts {
    pub rhythm: bool,
    pub instruments: bool,
    pub time_signature: bool,
    pub progression: bool,
    /// Take the preset's usual tempo where the meter stays the same. Off
    /// by default: the song keeps its tempo, so its own notes last as
    /// long as before.
    pub tempo: bool,
    pub swing: bool,
    /// Copy the preset's chord, bass and other accompaniment parts (a
    /// rhythm's are those of its matching style). Off, the song's own
    /// stay, as its drums do with `rhythm` off.
    pub matching_style: bool,
}
```

`rhythm` is the **Drums** box and `matching_style` the **Chord and bass
parts** box. Every box stands on its own: what is left unticked, the
song keeps. In a pattern, each box is one step of `write_style_parts`,
and each step touches only its own notes:

```rust
if parts.time_signature {
    meter(project, index, style.time);
}
if parts.matching_style {
    recomp(project, index, style.comping, style.bass);
}
if parts.progression {
    progression(project, index, style);
}
// ...
if parts.rhythm
    && let Some(rhythm) = Rhythm::named(style.rhythm)
{
    write_rhythm(project, index, rhythm);
}
if parts.instruments {
    style_instruments(project, style, selected);
}
```

`recomp` plays the chord and bass channels again and leaves the drums;
`write_rhythm` writes the drum lanes and leaves the rest. So a song can
take one style's drums and another's chord and bass parts, in either
order. A band keeps the same split in a field of its own, `drums` (see
[How the band plays a chord sheet](band.md)). The `if parts.rhythm &&
let Some(rhythm) = ...` line is a *let chain*: the block runs only when
the box is ticked *and* the style's rhythm is found, with `rhythm`
bound inside it.

**Voices / instruments** gives the style's sounds to the channels that
play its parts: Keys and Bass, or the channel picked in the rack. A
song imported from a MIDI file has neither; its channels carry the
names the file gave them, such as "PIANO MELODIA", "BANDONEON" or
"CONTRABAJO". Then `own_players_instruments` gives the style's sounds
to the song's own players, each by the family of the instrument it
had, and leaves the tune alone:

```rust
let program = match channel.program? {
    general_midi::CONTRABASS => band.bass_program(),
    // Violins to tremolo strings, and the string ensembles.
    40..=44 | 48..=51 => extras.strings.as_ref()?.program,
    program => match general_midi::FAMILIES[general_midi::family(program)] {
        "Piano" => piano,
        "Organ" => extras.organ.as_ref().map_or(piano, |organ| organ.program),
        "Guitar" => extras
            .guitar
            .as_ref()
            .map_or(piano, |guitar| guitar.program),
        "Bass" => band.bass_program(),
        _ => return None,
    },
};
```

The code sits in a closure given to `filter_map`, which returns an
`Option`, so every `?` and the `return None` mean "this channel keeps
its sound": a channel with no General MIDI program, strings when the
style has none, a choir or a bandoneon, which no style has. A double
bass is in the General MIDI strings family, so it is matched first, by
its number, and takes the bass. The tune is any channel whose name
says so (Lead, Melody, Melodía, Voice, Voz, Canto), or else the one the
lead sheet would show.

Two small methods correct the choice for the kind of preset, each
taking `self` by value and giving back a changed copy: `for_rhythm`
turns off the progression, which a rhythm does not have; and
`following` keeps the song's time signatures and tempo for a style
that plays every bar in its own meter. They chain:
`parts.for_rhythm().following(rhythm.follows_the_meter)`.

The choice is kept between runs, in egui's memory, under the name
`apply-options`. Until you change it, it is everything but the chord
progression, so the song keeps its own chords:

```rust
const FIRST_OPTIONS: StyleParts = StyleParts {
    rhythm: true,
    instruments: true,
    time_signature: true,
    progression: false,
    tempo: true,
    swing: true,
    matching_style: true,
};
```

`#[serde(default)]` on the struct makes a field added later read as its
default from an older save. A `const` of a struct is written out in
full where it is used, so `FIRST_OPTIONS` costs nothing at run time.

### The same style, the same song

Try Bluegrass, then Celtic, then Bluegrass again: the third should play
exactly as the first. A style must not be applied on top of the last
one, with the last one's meter and tempo, or every click would drift a
little further from the song. So the app keeps, next to each version of
the song the Library made, the song as it came, in its own meter:

```rust
pub(super) struct SongSource {
    original: Project,
    library: Tried,
    /// Latest preset components, applied without changing the source meter.
    reference: Project,
}
```

Each click starts again from `reference`, and asks whether the style's
meter differs from *that* song's, not from the one the last style put
it in:

```rust
let changes_meter = !lineup_core::remeter::is_in(&source.reference, time);
```

The tempo follows the same idea. **Tempo** puts the style's tempo where
most of the song plays, with `take_tempo` (see [The tempo
strip](../sound/tempo.md#the-tempo-strip)): a song that starts with a
slow bar and then plays at 121 BPM, given a style made for 180, plays
at 180 where it played at 121, and its slow bar stays as much slower
as before. Giving the style's tempo to the song's first bar instead
would make a MIDI file with a slow lead-in race.

## Where each part came from

Taking only some parts means a song can have its drums from one style,
its chord and bass parts from another, its tempo from a third and its
chords from the demo it started as. The Library shows it, a line per
part: Tempo "Tango marcato (style)", Chords "El Choclo (demo)", and
**edited, was …** once you change that part yourself. `sources.rs`
keeps that record.

The nine parts are an enum, in the order the Library lists them: the
drums, the chord and bass parts, the melody, the instruments, the time
signature, the key, the chords, the tempo and the swing. A set of them
is nine bits of a `u16`, one bit a part:

```rust
pub struct PartSet(u16);
```

```rust
pub fn contains(self, part: SongPart) -> bool {
    self.0 & part.bit() != 0
}
```

Every change that brings parts in goes through one function, which
compares the song before and after:

```rust
pub fn bring_in<R>(
    &mut self,
    source: &Source,
    taken: PartSet,
    edited: PartSet,
    change: impl FnOnce(&mut Project) -> R,
) -> R {
    let before = SongPart::ALL.map(|part| self.part_fingerprint(part));
    let result = change(self);
    for (part, before) in SongPart::ALL.into_iter().zip(before) {
        let now = self.part_fingerprint(part);
        // A key that changed with what was taken came with it: a
        // minor progression puts the song in minor, a row style puts
        // it in its row.
        let came = taken.contains(part)
            || (part == SongPart::Key && now != before && taken != PartSet::NONE);
        if came && !edited.contains(part) {
            self.sources.set(
                part,
                Some(PartSource {
                    source: source.clone(),
                    print: Some(now),
                }),
            );
        } else if now != before && !edited.contains(part) {
            self.follow_part(part, before, now);
        }
    }
    result
}
```

The parts taken now come from `source`, even if they did not change: a
4/4 style applied to a song already in 4/4 still gives it its time
signature. A part that changed only along with them, such as chords
moved into a new meter, *follows along*: it still comes from where it
came from, and is as it came if it was before. The key is the one part
no box names: it comes with whatever brought it, so a minor
progression's key comes from its style.

The `print` is a **fingerprint**: a `u64` hash of the part as it came,
made with `std::hash::DefaultHasher` from its notes, its tempo, its
chords. Whether a part is edited is not stored anywhere; it is asked
each time, by comparing the fingerprint with the part as it is now:

```rust
edited: came.print != Some(self.part_fingerprint(part)),
```

So editing the tempo makes it "edited, was …", and setting it back, or
undoing, makes it as it came again, with no code to keep a flag up to
date. The project file saves only the source and an `edited` flag,
never the hash: `DefaultHasher` may change its algorithm in another
Rust release, and a stored hash would then call every part of every
old file edited. On loading, the print is worked out again from the
part as it is.

Three of the parts are notes of the same patterns: the drums, the
melody and the chord and bass parts, which are everything else. They
are told apart by channel, with one helper that hashes the notes of the
channels a closure picks:

```rust
SongPart::Melody => self.hash_notes(&mut hasher, is_lead),
```

and the chord and bass parts pass `|channel| !is_drum(channel) &&
!is_lead(channel)`. With a band, the drums and the chord and bass parts
are what the band plays, so their fingerprints hash the names of the
styles they come from instead.

Older versions knew only six of the parts, and would refuse a file
naming one they do not know. So the drums, the melody and the key are
saved in a field of their own, `more_sources`, which those versions
skip. It is written whenever any part came from somewhere, even empty,
so a file without it is from before; for such a file,
`sources_of_an_older_file` works out where those three came from: the
drums with the patterns, a demo's melody with the demo, and the key
with the chords.

One more detail. The record of sources is not the music, and two songs
that play the same should be equal: the app compares projects to know
whether there are unsaved changes. So its `PartialEq` says every two
records are equal:

```rust
impl PartialEq for Sources {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}
```

A hand-written `impl` is how a type takes part in a derived
`PartialEq` (the `Project`'s) without changing the answer.

## The Library tab

The Library lists harmony styles, rhythms and your own styles in one
catalogue, sorted by the names as shown in the language chosen:

```rust
pub enum Preset<'a> {
    Style(&'static Style),
    Rhythm(&'static Rhythm),
    User(usize, &'a UserStyle),
}
```

Two kinds of entry borrow from tables that live for ever; your own
styles live in the app, so `Preset` has a lifetime, `'a`, which says it
cannot outlive the list it borrows from. Every question the list asks
(name, time, tempo, whether it brings a progression) is a method with
one `match`, so the rest of the browser never asks which kind it is. A
rhythm that is the very drums of a style of the same name, tempo and
feel is listed once, as the style.

The tick boxes under the search keep only the presets like the song,
in its time signature or near its tempo, or those with what you want:
a chord progression, drums that swing, an instrument picked from a
menu. What they look at in a preset is a small `Copy` struct,

```rust
struct Traits {
    time: TimeSignature,
    /// The speed it was made for, in beats a minute.
    tempo: f32,
    /// Whether it plays every bar in its own meter, so it fits any time
    /// signature.
    any_meter: bool,
    /// Whether it brings a chord progression.
    chords: bool,
    /// Whether its drums swing.
    swing: bool,
}
```

and the decision is one method:

```rust
fn keeps(self, traits: Traits, instruments: impl FnOnce() -> Vec<Instrument>) -> bool {
    let filters = self.filters;
    // 7/8 as 2+2+3 is still 7/8.
    let same_time = traits.any_meter || traits.time.ungrouped() == self.time.ungrouped();
    let ratio = traits.tempo / self.tempo;
    let near = ratio.max(1.0 / ratio) <= NEAR_SPEED;
    (!filters.time || same_time)
        && (!filters.speed || near)
        && (!filters.chords || traits.chords)
        && (!filters.swing || traits.swing)
        && filters
            .instrument
            .is_none_or(|instrument| instruments().contains(&instrument))
}
```

Read the lines as rules joined by "and": `!filters.time || same_time`
is "either the box is not ticked, or the time is the same".
`ratio.max(1.0 / ratio)` turns "half as fast" (0.5) and "twice as fast"
(2.0) into the same number, 2, so faster and slower are judged alike;
with `NEAR_SPEED` at 1.15, a song at 130 BPM keeps presets made for
113 to 149.5. Comparing ratios rather than a difference in BPM treats
slow and fast songs alike: ±15 BPM would be a lot at 60 and little at
180. `keeps` takes `self` by value: the struct is `Copy` and a few
bytes, so that costs no more than a reference.

The instruments are the one question that costs something: a preset's
are those of the band that would play it, worked out from its style.
So they come as a closure, `instruments`, called only at the end, when
an instrument is chosen. `Option::is_none_or` is `true` for `None`, no
instrument chosen, and otherwise asks the closure about the one inside.
`&&` stops at the first `false`, so a preset already left out by an
earlier box never works its instruments out at all.

The list is grouped by genre, one `egui::CollapsingHeader` a genre with
its count and your own styles first under **User styles**, unless
**Settings > Style library** says **Alphabetical list**. A search opens
every group that has a match. When the song changes style, the group of
the new one opens for half a second, long enough for egui to scroll its
lit row into sight, and then leaves the groups as you had them.

### Trying, swapping and undoing

What a click does depends on the song. With a band, as in a song from
**File > New** or a demo, the style becomes the band's way of playing
the chord sheet (see [How the band plays a chord
sheet](band.md)). While the song plays without a band, the
click changes the whole song, so the music goes on. Otherwise it writes
the style into a **trial pattern** of its own and plays it there.

Clicking another style should *replace* the trial, not pile up
patterns, so you can flick through the library. But once you start
editing that pattern, it is yours. The app tells the two apart by
keeping a copy of what it wrote:

```rust
pub(super) struct Audition {
    pattern: usize,
    /// The pattern as the library left it. If it still looks like this,
    /// the next click may rewrite it.
    written: Pattern,
    style: Option<&'static Style>,
    rhythm: Option<&'static Rhythm>,
    row_style: Option<&'static RowStyle>,
    micromode_style: Option<&'static MicromodeStyle>,
}
```

```rust
fn unchanged_audition(&self) -> Option<&Audition> {
    let pattern = self.project.pattern(self.pattern)?;
    self.auditions
        .iter()
        .rev()
        .find(|audition| audition.pattern == self.pattern && &audition.written == pattern)
}
```

`==` on two `Pattern`s compares every note, thanks to the derived
`PartialEq`: instead of tracking every edit, keep a copy and compare,
the same idea as unsaved changes. The app keeps the last 64 trials, not
only the newest, and searches from the newest with `rev()`. That is for
undo: after Ctrl+Z brings the previous style back, the pattern matches
an older copy, and the next click still swaps it in place.

Harmony styles and rhythms take the parts you ticked (see [Which parts
to take](#which-parts-to-take)). Row styles and micromode styles have no such choice: they
are tried the same way, and differ only in the notes they write. So
their shared steps are one method, and the different step is a closure
passed in:

```rust
fn write_pattern_style(
    &mut self,
    name: &'static str,
    tempo: f32,
    rhythm: Option<&'static Rhythm>,
    write: impl FnOnce(&mut Project, usize),
) -> Option<(usize, Pattern)> {
```

`impl FnOnce(&mut Project, usize)` accepts any closure that can be
called once with those two arguments. The compiler makes a copy of the
method for each closure passed to it, so the call costs no more than
code written out. `FnOnce` asks the least of the three closure traits
(`Fn`, `FnMut`, `FnOnce`): every closure is at least `FnOnce`, so
asking for it accepts them all.

Every write goes through `edit`, so a trial is one step to undo, the
audio thread hears of it through the usual diff, and saving just
works. Inside it, `bring_in` records where the parts came from.

## Testing it

The tables are data, and data can be wrong as easily as code, without
crashing anything. So the tests read the whole library:
`every_style_parses_and_fits_in_a_pattern`,
`names_are_unique_and_rhythms_exist`, `every_genre_has_styles`,
`styles_play_in_any_key`, and for the rhythms
`every_line_is_whole_bars_of_known_characters`,
`names_are_unique_and_fills_exist` and `congas_play_high_and_low`.
`a_new_time_signature_keeps_the_kind_of_rhythm`,
`an_odd_meter_gets_a_rhythm_grouped_its_way` and
`a_time_signature_with_no_rhythm_gets_a_basic_beat` follow `for_time`
down its chain.

`apply.rs` checks where notes go (`styles_go_to_keys_and_bass`,
`chords_go_around_a_locked_channel`), that `recomping_keeps_the_chords`,
that a whole song takes a new rhythm or a style, that a waltz puts the
song in 3/4, and, the widest, that
`every_demo_takes_every_kind_of_library_click`. `sources.rs` checks a
part that comes, is edited, and is undone:

```rust
#[test]
fn a_part_brought_in_is_edited_once_changed_and_not_after_undoing_it() {
    let mut project = Project::demo();
    project.bring_in_all(&demo());
    assert_eq!(from(&project, SongPart::Tempo), Some(("El Choclo", false)));
    let tempo = project.tempo;
    project.scale_tempo(Tempo::new(tempo.bpm() + 10.0));
    assert_eq!(from(&project, SongPart::Tempo), Some(("El Choclo", true)));
    assert_eq!(from(&project, SongPart::Swing), Some(("El Choclo", false)));
    project.scale_tempo(tempo);
    assert_eq!(from(&project, SongPart::Tempo), Some(("El Choclo", false)));
}
```

For the boxes,
`a_styles_sounds_go_to_an_imported_songs_players_by_family_but_the_tune`
builds a song like an imported tango, with a "PIANO MELODIA", a
bandoneon, a guitar, a double bass, strings and a choir, and checks
which channels take the sounds of Tango and of Bossa nova and which keep
theirs; `a_songs_keys_and_bass_still_take_a_styles_sounds_alone` checks
a song with Keys and Bass is changed only there. In `band.rs`,
`drums_and_chord_parts_come_on_their_own_from_builtin_and_saved_styles`
gives a Rock ride band the drums of Bossa nova first, from the built-in
style and from a saved copy of it, and checks the piano and the bass
stay as they were, the file reads back the same, and once the chord and
bass parts follow, the band is plain Bossa nova again, in the format it
needed before; `a_styles_chord_parts_can_come_without_its_drums` takes
Bossa nova's chord and bass parts over Rock ride's drums, which needs
format 29. `a_style_tempo_goes_where_most_of_the_song_is` checks
`take_tempo` on a song with a slow first bar.

In the app, `the_library_tick_boxes_keep_only_presets_like_the_song`
counts the presets listed as the boxes are ticked;
`the_same_style_brings_the_same_tempo_whatever_was_tried_before` clicks
Bluegrass, Celtic, Bluegrass and Celtic on a song in 2/4 and in 4/4,
and checks each style brings the same tempo every time; and
`trying_row_and_micromode_styles_rewrites_one_pattern_while_it_is_unchanged`
checks that trials land in one pattern and each is one step to undo.

Exercises on the library are in [Exercises: the
band](exercises.md).
