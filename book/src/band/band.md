# How the band plays a chord sheet

Files: [`crates/lineup-core/src/band.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/band.rs),
[`crates/lineup-core/src/band/`](https://github.com/naelolaiz/ferrous-lineup/tree/main/crates/lineup-core/src/band),
[`crates/lineup-core/src/library/comping.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/comping.rs),
[`crates/lineup-core/src/library/follow.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/follow.rs),
[`crates/lineup-app/src/chord_sheet/band.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/chord_sheet/band.rs)
and [`crates/lineup-app/src/app/follow.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/follow.rs)

The chord sheet holds a song's harmony. The band plays it. Type `C`,
Tab, `Am`, Tab, `Dm7 G7`, Tab, `C`, press Space, and a piano, a bass
and drums play those chords, with a guitar, strings, an organ or a horn
section where the style has them. Change the style, the key or a chord,
and the band plays it again the new way.

![The chord sheet of a song with the Pop beat band: the Band and Progression buttons, "Key: C major · 4/4 · 110 BPM", the band's rows (Form, Choruses, Ending, and You play Piano, Bass, Drums, Guitar), and the bars C | Am | F | G7 with their numerals](../images/step-9-band-sheet.png)

The band's parts are ordinary patterns on tracks of their own, called
"Piano 1", "Bass 1", "Drums 1". Double-click one and the piano roll
shows every note the band plays; nothing is hidden, which matters in a
program you learn from: you can see how a bossa nova pianist spreads a
Dm7 over the bar, or where a walking bass puts its lead-in note.

![The playlist with the band's three tracks: Piano 1, Bass 1 and Drums 1, four bars each](../images/step-9-band-playlist.png)

This chapter covers how a style is split from the chords it plays, what
each player plays and how, how the parts are written again when the
sheet changes without losing what you edited, the bigger band (guitar,
strings, organ, horns), and the arranger-keyboard mode where the band
follows the chord you hold. Song form, variations and section styles
are in [Song form, sections, variations and
locks](song-form.md); breaks in the band's playing in
[Shots, holds, rests and pushes](stops.md). How to use the
band is in the manual's [Your first
song](https://naelolaiz.github.io/ferrous-lineup/manual/first-song.html)
and [A bigger band and the
horns](https://naelolaiz.github.io/ferrous-lineup/manual/bigger-band.html).

## The music, briefly

A few words come up all through the band's code:

- **Comp**: a chord played under the tune; as a verb, to play those
  chords. **Comping** is the rhythm and the voicings a chord player
  uses, such as a bossa pianist's syncopated chords.
- **Voicing**: which notes of a chord sound, and in which octaves.
  **Voice leading** moves each voice of a chord to the nearest voice of
  the next, so the chords join smoothly.
- **Changes**: a song's chord progression, bar by bar; what the chord
  sheet holds.
- **Walking bass**: one note per beat, the root first, notes of the
  chord, and a half step into the next root.
- **Pad**: chords held softly under the band, usually by strings or an
  organ.
- **Turnaround**: the chords at the end of a section that lead back to
  its start, such as I vi ii V.

## What to play, and how to play it

A style from the [library](library.md) is a bundle: "Bossa
nova" means Dm7 G7 Cmaj7 *and* the piano's syncopated chords *and* the
bass on 1 and the "and" of 2 *and* the rim clicks. A backing band
separates the two halves. The chords are yours, on the sheet; the style
only says how to play them.

| Half | What it is | Where it is chosen |
|------|------------|--------------------|
| What to play | a progression of numerals | the sheet's **Progression** menu, from each `Style`'s chart |
| How to play it | comping, bass, drums, the players it adds | the **Library**, as a `BandStyle` |

A `BandStyle` is a style with the chords taken out:

```rust
pub struct BandStyle {
    pub name: &'static str,
    pub genre: Genre,
    pub time: TimeSignature,
    /// The tempo it is usually played at, in BPM.
    pub tempo: f32,
    pub comping: Comping,
    pub bass: Bass,
    /// The drums, or `None` for a band without them.
    pub rhythm: Option<&'static Rhythm>,
    pub about: &'static str,
}
```

The band styles are not written out by hand. Each library rhythm
already knows how the piano and bass go with it (`apply::feel`, see
[Playing a song in another
style](library.md#playing-a-song-in-another-style)), so the
list is made from the rhythms the first time it is asked for, with four
styles without drums added: **Chorale** (each chord held, as an organ
plays a hymn), **Alberti bass** (the chord broken low, high, middle,
high, as in Mozart's sonatas), **Arpeggios** and **Sarabande** in 3/4.

```rust
pub fn band_styles() -> &'static [BandStyle] {
    static STYLES: OnceLock<Vec<BandStyle>> = OnceLock::new();
    STYLES.get_or_init(|| {
        let mut styles: Vec<BandStyle> = RHYTHMS
            .iter()
            .map(|rhythm| {
                let (comping, bass) = apply::feel(rhythm);
                BandStyle {
                    name: rhythm.name,
                    genre: rhythm.genre,
                    time: rhythm.time,
                    tempo: rhythm.tempo,
                    comping,
                    bass,
                    rhythm: Some(rhythm),
                    about: rhythm.about,
                }
            })
            .chain(WITHOUT_DRUMS)
            .collect();
        // A stable sort: within a genre, the library's order stays.
        styles.sort_by_key(|style| style.genre);
        styles
    })
}
```

A `static` must be built when the program is compiled, and a `Vec`
cannot be, because it needs memory from the heap. `std::sync::OnceLock`
is a box that starts empty and is filled once, the first time
`get_or_init` runs; every later call gets the same value back. Several
threads can race to call it and only one closure runs. The return type
`&'static [BandStyle]` says "this list lives as long as the program",
which is what lets a `&'static BandStyle` sit in the project as the
band's style with no lifetime to track. `sort_by_key` is stable, so
styles of one genre keep the library's order.

## A band in the song

**File > New** makes a song with an empty sheet and a band ready to play
it in **Pop beat**:

```rust
pub fn with_band() -> Self {
    let mut project = Project::empty();
    project.set_band_style(BandStyle::named(DEFAULT_STYLE));
    project.remove_unused_tracks();
    project
}
```

From there, three places shape the band:

- **The Library.** A click on a style or a rhythm makes it the band's
  way of playing, with the parts ticked in its window: the drums, the
  chord and bass parts, the voices, the chord progression, the time
  signature, the tempo and the swing (see [Which parts to
  take](library.md#which-parts-to-take)). A song with music of its own
  but no band gets one when the chord and bass parts are taken with the
  voices.
- **The sheet's 🎷 Band menu** chooses who plays and on which sound: an
  **Instrument lineup** ("Piano, acoustic guitar + strings", "Horns,
  timpani + strings" and six more), a tick box and an instrument for
  each player, the **Horn section**, the **Soloist** (see [The
  soloist](soloist.md)) and **No band**.
- **The sheet's Progression menu** is the Library again, showing only
  styles with a chord progression; a click writes that progression from
  the selected bar on, in the song's key: I vi ii V in C is Cmaj7 Am7
  Dm7 G7, the turnaround that ends so many standards. A chord already
  sounding is not written again, so "I | I | IV" writes C, nothing,
  then F, as a chart does. On an empty sheet, the song takes the
  progression's mode: "i bVII bVI" is Am G F in A minor.

Because the parts can come from different places, the band keeps them
apart:

```rust
pub struct Band {
    pub custom_rhythm: Option<Box<crate::library::UserStyle>>,
    pub custom_instruments: Option<Box<crate::library::UserStyle>>,
    pub style: &'static BandStyle,
    /// The instrumentation can come from a different style than the rhythm.
    pub instrument_style: &'static BandStyle,
    /// The exact library style supplying the comping, when one was picked.
    pub harmony_style: Option<&'static Style>,
    /// The chords and the bass played as plain accents on the beats: how
    /// songs saved so play. The Library no longer chooses it.
    pub accents_only: bool,
    /// Whose drums play, when not those of `style` or `custom_rhythm`.
    pub drums: BandDrums,
    // ...
}

pub enum BandDrums {
    /// Those of the style the band plays the chords and the bass in, or
    /// of the saved style it plays them from.
    Style,
    /// Those of another style.
    Of(&'static BandStyle),
    /// Those of a saved style.
    Saved(Box<crate::library::UserStyle>),
    /// None: the song's own drums play.
    Song,
}
```

`style` says how the chords and the bass are played; `drums` says whose
drums play with them; `instrument_style` says who plays and on what;
and `harmony_style` is the library style picked, when its comping
differs from its rhythm's. The two `custom_` fields hold styles of
your own (see [Your own styles](style-editor.md)). `accents_only` plays
the chords and the bass on the beats the bar is felt in, whatever the
style; only songs saved that way still play so.

Taking a style's chord and bass parts but not its drums keeps the drums
the band played: `drums` turns from `Style` into `Of` the old style,
so they no longer follow `style`. Taking the drums alone sets `Of` the
new one. When the two end up the same again, `tidy_drums` turns `Of`
back into `Style`, so a band has one way to say "this style's drums",
and its file stays one an older version can read. Each player asks the
band, not the style, how to play:

```rust
fn comping_in(&self, variation: Variation) -> Comping {
    if self.accents_only {
        Comping::Accents
    } else {
        self.playing_style().comping_in(variation)
    }
}
```

Picking a style can change three more things. A style in another time
signature puts the song in it, bar for bar, as a rhythm from the
library does (see [Putting a song in another time
signature](../sound/tempo.md#putting-a-song-in-another-time-signature)):
each bar keeps its length in seconds, so a song in 4/4 at 110 BPM
picked as a Jazz waltz becomes 3/4 at 82.5 BPM (4 × 60 / 110 =
3 × 60 / 82.5 = 2.18 seconds a bar). The tempo is the style's while the
sheet is empty, and yours once you have written chords. And the song
takes the style's swing, since swing is part of how a style is played.
The piano and the bass get the General MIDI instrument of the genre (an
electric piano for soul and electronic styles, a double bass for jazz
and tango, a synth bass for hip hop), so they sound right through a
SoundFont or MIDI out.

## What each player plays

A band style knows *how*; the sheet knows *what*. Putting them together
takes three steps.

### The harmonies, as notes

`played_harmonies` walks the sheet and turns each harmony into a
`Played`: when it starts and ends, the notes the chords are played with
(its voicing), and what the bass plays under it.

```rust
pub struct Played {
    pub start: usize,
    pub end: usize,
    /// The notes the chords are played with, from low to high. Empty for
    /// no chord: nothing plays.
    pub voicing: Vec<Note>,
    /// What the bass plays.
    pub bass: BassPart,
}

/// What the bass plays under a harmony.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BassPart {
    /// A chord: its line goes over the chord's notes from `note` (its
    /// root, or a slash chord's bass), and may lead into `next`, the
    /// bass note of the harmony after it.
    Chord {
        chord: Chord,
        note: Note,
        next: Note,
    },
    /// A group of notes with no root, such as a pitch-class set: the
    /// bass keeps the rhythm of its line on this one note and its octave.
    Pedal(Note),
    /// Nothing.
    Rest,
}
```

Chords are voiced the way the library voices a chart, each from the one
before, moving every voice as little as it can, so C Am F G7 does not
jump around the keyboard (see [Voice
leading](../theory/voice-leading.md)). Their bass notes make a line
(`bass_line`), so `G7/B` has B in the bass. The other ways Ferrous
Lineup names harmony are played by their notes:

- **A polychord** is played as written: the lower chord, the upper
  stacked right above it, over the lower chord's bass.
- **A pitch-class set or a micromode** has no root, so there is no
  "right" bass note. The band takes the first note of its normal form
  (its most compact order): 3-11 T2, D F A, is played from middle C up,
  over a D. The choice is arbitrary but fixed, so the same set always
  sounds the same.
- **A row form** is a line of twelve notes, not a chord. The band plays
  it as four chords of three notes, in the row's order, one after
  another, each over its first note. Cutting a row into groups and
  sounding each group as a chord is a common way to harmonise it;
  Webern built the row of his Concerto, Op. 24, from four such groups.
- **N.C.** plays nothing; the drums go on.

### The rhythm, kept to the bars

The library's comping patterns start again at every chord. That is
right for a chart where chords change on the bar line, but a sheet can
say `Dm7 G7` (a change on beat 3) or `C / / G7` (on beat 4). Starting
the pattern again on beat 3 would shift the groove every time the chord
changes, as if the drummer counted "one" in the middle of the bar. A
band keeps its groove and lets the chords fall into it. So
`Comping::play_in_bars` places the rhythm's hits in every bar, from its
first beat, and each hit plays whichever harmony is sounding when it
falls:

```rust
for bar in bars {
    let (period, hits) = self.hits(bar.time);
    let from = if self.follows_the_bar() {
        bar.start
    } else {
        period_start(bar, period)
    };
    for offset in (0..bar.end() - from).step_by(period) {
        for hit in hits.iter() {
            let at = from + offset + hit.at;
            if at < bar.start || at >= bar.end() {
                continue;
            }
            let index = harmonies.partition_point(|harmony| harmony.start <= at);
            let Some(index) = index.checked_sub(1) else {
                continue;
            };
            // ...
        }
    }
}
```

The harmonies are sorted by start, so `partition_point` counts, by
binary search, the harmonies that have started by `at`; the sounding one
is the one before. A chord so short that no hit falls in it (a G7 on
the "and" of 4, in a pattern that only strikes on 1 and 3) would go
unheard, and a skipped chord is a wrong harmony, so such a chord is
struck once where it starts. A pattern longer than a bar, like the
bossa nova's two-bar clave, counts from bar 1: `period_start` starts it
in odd bars and goes on in even ones, the way a player counts "one,
two" across the pair.

### Beats for any meter

Most comping patterns are a fixed table of hits. Two are not: **plain
accents** (`Comping::Accents`), a chord on each beat the bar is felt
in, and the **aksak** of the Balkan styles, the afterbeats inside each
beat. In 7/8 felt as
2+2+3 they fall in one place, in 5/4 felt as 2+3 in another, so their
hits are built from the bar's `felt_groups` (see [Beats of different
lengths](../sound/tempo.md#beats-of-different-lengths)):

```rust
fn hits(self, time: TimeSignature) -> (usize, Cow<'static, [Hit<Voices>]>) {
    match self {
        // ...
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

The function returns a **`Cow`**, "clone on write" (see [The order of
everything](../program/sync.md#the-order-of-everything)): the fixed patterns hand out a
borrow of their `static` table, `Cow::Borrowed`, at no cost, and the
two built ones an owned `Vec`, `Cow::Owned`. The caller reads both the
same way. These two patterns count from the bar line wherever the
chord starts (`follows_the_bar`), since the beats of an odd meter are
where the bar puts them.

### The bass

`Bass::play_in_bars` is the same idea for the bass line: it starts
again where each harmony starts, so the root lands on the change, and
again at each bar line, so a chord held for eight bars does not turn
into one long walk. Only the last piece of a harmony leads into the
next. A walking bass plays one note per beat:

```rust
fn walk(span: usize, down: bool) -> Vec<(usize, Tone)> {
    const UP: [Tone; 6] = [
        Tone::Third,
        Tone::Fifth,
        Tone::Seventh,
        Tone::Octave,
        Tone::Fifth,
        Tone::Third,
    ];
    const DOWN: [Tone; 6] = [
        Tone::LowSeventh,
        Tone::LowFifth,
        Tone::LowThird,
        Tone::LowFifth,
        Tone::LowSeventh,
        Tone::Root,
    ];
    let beats = span.div_ceil(4);
    (0..beats)
        .map(|beat| {
            let tone = if beat == 0 {
                Tone::Root
            } else if beat == beats - 1 {
                Tone::Approach
            } else if beats == 3 {
                if down { Tone::LowFifth } else { Tone::Fifth }
            } else if down {
                DOWN[(beat - 1) % DOWN.len()]
            } else {
                UP[(beat - 1) % UP.len()]
            };
            (beat * 4, tone)
        })
        .collect()
}
```

The root on 1, chord notes in the middle, and on the last beat the
**approach**, a note a half step above or below the next chord's bass
note. The line is given that note itself, not just its name: a root
is played between C2 and B2 (see [The harmonies, as
notes](#the-harmonies-as-notes)), so from G7 to C the next note is C2,
and the approach is the half step on the side nearer the note before
it, D♭2 coming down from D2. Knowing only "C", the line could pick B2
and then fall a seventh to the C2 that follows.

The middle notes go up the chord (`UP`) or down it (`DOWN`, the
chord's notes below the root). `Bass::play` makes both lines and keeps
the one that reaches the next chord in smaller steps, the line down
only while it stays above E1 (`BASS_FLOOR`), the lowest string of a
bass guitar.
So over C going to Am the walk goes up, C2 E2 G2 G♯2, into A2, and over
G7 going to C it comes down, G2 F2 D2 D♭2, into C2. In a bar of three
the walk goes root, fifth, approach: over C7 going to F, C2 G2 F♯2,
into F2. The third is left out there because the third of C *is* the
half step below F: the line would reach its lead-in a beat early.

### The drums

The drums lay the style's rhythm out over the song's parts, its
sections or the parts heard in its notes: a fill closes each part, a
small one marks every eight bars inside long ones, the part after a
fill starts with a crash cymbal, and a part of another kind moves the
hi-hat to the ride (see [Drums by part](song-form.md#drums-by-part)).
A fill also leads into a change of variation or style, which starts
with a crash. Variations, the intro and the ending are in [Song form,
sections, variations and locks](song-form.md).

### Each part, as patterns

A whole song does not fit in one pattern (a pattern holds at most 256
steps, 16 bars of 4/4), and one giant pattern would be hard to read
anyway. `chunks` cuts the song into stretches: whole bars of one time
signature, inside one of the song's parts (its sections, or the parts
heard in its notes), as long as a pattern may be. `write_part` cuts the
notes at those lines (a note that lasts past the end of a stretch goes
on at the start of the next, and a note held over the start of a part
joins the two stretches into one) and compares each stretch's notes
with those before:

```rust
let index = match contents.iter().position(|known| *known == content) {
    Some(index) => index,
    None => {
        contents.push(content);
        contents.len() - 1
    }
};
```

Stretches that play exactly the same notes share **one** pattern, with
two clips of it, as a producer copies a clip instead of playing a part
twice. A song of two identical choruses gets "Piano 1" twice in the
playlist. The patterns are written over the ones the player already
had, so "Piano 1" keeps its number while you edit the sheet, and a
piano roll open on it shows the new notes instead of closing. Every
pattern the band writes is tagged:

```rust
/// The band's player whose part it holds, if the band wrote it (see
/// [`crate::band`]). The band writes these patterns again when the
/// chord sheet changes.
pub player: Option<Player>,
```

That tag is how the band finds its own work again, and how the library
knows to leave it alone when it plays a whole song in another rhythm.

## Playing again when the sheet changes

The band plays what the sheet says *now*. Type a chord, change the key,
put the song in another meter, drag a section: the parts are written
again, in the same undo step as your change, so one Ctrl+Z takes back
both. Every change to the project goes through `LineupApp::edit` (see
[Undo and redo](../program/undo.md)), so that is where the band is
told:

```rust
let before = self.project.clone();
let result = change(&mut self.project);
// The band plays what the chord sheet says now, in the same step.
self.project.follow_chord_sheet();
```

Writing every part again after every click would be wasteful: moving a
clip of the melody changes nothing the band plays from. So
`follow_chord_sheet` first takes a **fingerprint** of what the parts
are played from (the style, the chord markers, the keys, the bars, the
sections, the form) and compares it with the one taken last time:

```rust
pub fn follow_chord_sheet(&mut self) -> bool {
    let Some(band) = &self.band else {
        return false;
    };
    if band.heard == Some(self.band_inputs()) {
        return false;
    }
    self.band_channels(false);
    self.write_band();
    true
}
```

A fingerprint, or hash, is a number computed from some data, such that
different data almost surely give different numbers. Comparing two
64-bit numbers is much cheaper than comparing two chord sheets, and the
band only has to keep the number, not a copy of the sheet.

## A part you edit stays yours

A band that writes its parts afresh every time would lose an edit to
the bass line the next time anything changed. Here, a part you change
by hand **freezes**: the band leaves it as you left it, and the sheet
says so, with a **Play it again** button that throws your version away
and has the band write the part from the sheet once more. The other
players go on following the sheet.

![The chord sheet with the notice "You edited the piano: the band keeps it as you left it." and a "Play it again" button](../images/step-9-band-frozen.png)

How does the band know you edited a part, whatever tool you used: the
piano roll, the channel rack, a dragged clip, the library? It does not
watch the tools. It keeps, for each player, a fingerprint of the part
it last wrote, and compares it with a fingerprint of the part as it is
now:

```rust
pub fn is_frozen(&self, player: Player) -> bool {
    self.band.as_ref().is_some_and(|band| {
        band.written[player.index()].is_some_and(|written| written != self.part_print(player))
    })
}
```

Two `is_some_and` nested read as one sentence: "there is a band, and it
has written this player's part, and that is not what is there now".
`part_print` mixes in where each of the player's clips starts, how often
it repeats, and every note of its pattern. Which track a clip is on and
which number its pattern has are left out, so moving the Piano track
down, or deleting a pattern of yours (which renumbers the ones after
it), freezes nothing.

### FNV-1a

The fingerprints are saved in the project file, so a part you froze
stays frozen when you open the song tomorrow. The standard library's
`DefaultHasher` is not promised to give the same number in the next
version of Rust, so it cannot be saved. The band uses **FNV-1a**, a
classic hash of a few lines:

```rust
struct Fingerprint(u64);

impl Fingerprint {
    fn new() -> Self {
        Fingerprint(0xCBF2_9CE4_8422_2325)
    }

    fn bytes(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            self.0 ^= u64::from(byte);
            self.0 = self.0.wrapping_mul(0x0100_0000_01B3);
        }
    }

    fn number(&mut self, number: usize) {
        self.bytes(&(number as u64).to_le_bytes());
    }
}
```

For each byte: mix it into the number with XOR, then multiply by a
large prime, `0x100000001B3`. The starting number and the prime are the
published constants of the algorithm, so every program that implements
FNV-1a gets the same result. Multiplying two large `u64`s overflows; in
a debug build plain `*` would stop the program with "attempt to
multiply with overflow", and `wrapping_mul` says that keeping only the
low 64 bits is what we want, as a hash does on purpose. `to_le_bytes`
writes a number's bytes lowest first ("little-endian"), whatever the
computer, so the same song gives the same print everywhere. A part with
no notes at all is the starting number itself, 14695981039346656037:
you will see it in a saved file, for strings that wait for variation b.

### Moving the whole song is not an edit

Changing the song's key transposes every note, the band's included.
The band's parts no longer match their fingerprints, but you did not
edit them: the tool moved everything. Putting the song in another
meter, copying a section and inserting a bar are the same. These tools
run inside `moving_the_band`:

```rust
pub(crate) fn moving_the_band<R>(&mut self, change: impl FnOnce(&mut Project) -> R) -> R {
    self.keeping_parts(|_, _| true, change)
}
```

```rust
fn keeping_parts<R>(
    &mut self,
    keep: impl Fn(&Project, Player) -> bool,
    change: impl FnOnce(&mut Project) -> R,
) -> R {
    let following: Vec<Player> = Player::ALL
        .into_iter()
        .filter(|&player| {
            self.band
                .as_ref()
                .is_some_and(|band| band.written[player.index()].is_some())
                && !self.is_frozen(player)
                && keep(self, player)
        })
        .collect();
    let result = change(self);
    let prints: Vec<(Player, u64)> = following
        .into_iter()
        .map(|player| (player, self.part_print(player)))
        .collect();
    if let Some(band) = &mut self.band {
        for (player, print) in prints {
            band.written[player.index()] = Some(print);
        }
    }
    result
}
```

It notes which parts were following the sheet before, runs the tool,
and then takes their new fingerprints as the band's own. A frozen part
is moved by the tool like any other notes, and stays frozen: your
version of the piano, now in D. `keep` is an `impl Fn`, called many
times, while `change` is an `impl FnOnce`, called once. The prints are
collected into a `Vec` before `self.band` is borrowed mutably, because
`part_print` needs `self` too, and the borrow checker allows one or the
other at a time. Deleting a channel uses the same method with another
`keep`: a part that played nothing on the deleted channel only had its
channel numbers shift, which is not an edit; a part that lost notes is
frozen.

## A band over a song of its own

Open a demo, such as **Canon in D**, and take a style's chord and bass
parts and its voices from the Library. A demo already plays its own
accompaniment, so first the band:

1. writes the chords the song plays on the sheet, if it is empty (see
   [Where the chords come
   from](chord-sheet.md#where-the-chords-come-from)), and
2. **mutes** the song's own chord and bass channels (the library calls
   them "Keys" and "Bass"), a bass of the song's own as a MIDI file
   brings it (a channel named for one, or with a General MIDI bass
   sound: `plays_the_bass`) and, when the style's drums come too, its
   tracks that hold only drums, which an imported file keeps on a track
   of their own,

so the two do not play over each other. The melody plays on, now with
the band behind it. Without the style's drums, the band comes in with
`drums: BandDrums::Song`, and the song's own drums play on. A style that
follows the song, such as the orchestra, comes in alongside the song's
parts instead of muting them (see [An orchestra for music that changes
meter](orchestra.md)).

![The chord sheet of Canon in D after the Pop ballad preset: the band's rows (Form, Choruses, Ending, and You play Piano, Bass, Drums, Guitar, Strings) over D A Bm F#m | G D G A, the progression of Pachelbel's Canon, written bar by bar](../images/step-9-band-demo.png)

**No band** removes the band's patterns, its tracks and its channels
(only the ones it added, and only if nothing else plays on them), and
plays again what it muted. It remembers what it muted by *name*, not by
number, so the right channels come back even after you added or removed
others in between. The chords stay on the sheet.

## A bigger band: guitar, strings and organ

A piano, a bass and drums is a jazz trio, and the rhythm section of most
bands, but a rock band without its guitar or a tango without its violins
does not sound like itself. So a style can bring up to three more
players, each playing the way that music uses them:

| Style | Who comes in | What they play |
|---|---|---|
| Pop beat | steel-string guitar, strings in b | strummed chords, held strings |
| Rock beat | overdriven guitar, rock organ in b | power chords in eighth notes |
| Shuffle, Rock and roll | clean electric guitar | the boogie shuffle |
| Swing | jazz guitar | four to the bar |
| Funk, Disco | muted guitar, and strings in disco | sixteenths, scratched |
| One drop, Steppers, Rocksteady | clean guitar, organ | the chop on 2 and 4, the organ bubble |
| Bossa nova | nylon-string guitar | the bossa rhythm |
| Tangos | strings | held chords |
| Chorale | church organ | held chords |
| Hip hop, most electronic styles | nobody | |

"Strings in b" means they wait for variation b: right-click a bar, pick
**Variation b**, and from there the strings come in, holding the chords
an octave above the piano, as strings lift a pop chorus.

### The music: how a guitarist plays a chord

A pianist can play any notes of a chord: ten fingers, every key in
reach. A guitarist has six strings, tuned from the lowest

| String | 6 | 5 | 4 | 3 | 2 | 1 |
|---|---|---|---|---|---|---|
| Open note | E2 | A2 | D3 | G3 | B3 | E4 |
| MIDI | 40 | 45 | 50 | 55 | 59 | 64 |

and a left hand with four fingers to press them against the **frets**,
one semitone each. Each string plays one note at a time, or none. A
string not pressed rings **open**, for free; that is why the keys of E,
A, D and G, whose chords use many open strings, are the guitar's home
keys. So a guitar chord is a **shape**: a fret (or nothing) for each
string. Chord books write it as six characters from the low E string,
with `x` for a string not played and `0` for an open one:

```text
C    x32010     the A string at fret 3 (C), D at 2 (E), G open (G),
                B at 1 (C), high E open (E): C E G C E
G    320003
Am   x02210
F    133211     the first finger flat across fret 1: a "barre"
```

The lowest string that sounds must be the chord's **bass**, the note the
bass player plays (the root, or under a slash chord the note after the
slash). That is why the C shape leaves out the low E string: an E there
would make it C/E, which chord books write `032010`.

Ferrous Lineup does not store a chord book. It finds the shapes the way
a guitarist learns them, by trying the frets in reach and keeping the
easiest. For each **position** on the neck (the fret the first finger
sits at), it tries every way to put a note of the chord on each string
within four frets, keeps the ones a hand can play, and scores them:

- every **needed** note must be there: the root, the third (or the
  second or fourth of a sus chord), the sixth or seventh, a fifth that is
  not perfect, and the highest extension, which names the chord (the 13
  of a G13);
- a perfect fifth and the lower extensions are **welcome**, and shapes
  with more of them are better;
- shapes low on the neck, with **open strings**, and on more strings
  are better;
- up to four pressed strings are fine. More need a **barre**, one
  finger flat across the lowest fret, which stops every string above it,
  so none of those can be open, and leaves three fingers for the rest.

One more rule keeps the sound clear. Two notes close together sound
muddy low down: a third between E2 and G♯2 rumbles, while a fifth there
is fine. Arrangers have a table of the lowest note each interval sounds
clear from, the **low interval limits**, and a shape that goes below
them costs a little more. A little, not a lot: the open G, `320003`,
starts with G2 and B2, a third below its limit (B♭2), and it is still
the G every guitarist plays. Its sound is part of the guitar's sound.

With those rules, the search finds the open chords of the chord books
(C `x32010`, G `320003`, D `xx0232`, Am `x02210`, G7 `320001`, Cmaj7
`x32000`), barre chords in other keys (F `133211`, the open E shape a
fret higher, and B `x24442`, the A shape), and a shape for every chord
type in every key.

A guitarist does not strike the strings together. The pick sweeps
across them: on a **down-stroke** from the low strings to the high ones,
on an **up-stroke** back from the high ones, and the hand coming up
rarely reaches the low strings. The library's rhythms already mark the
light "up" hits, between the beats. The guitar plays each down-stroke
on every string, low to high, a tick apart (about 5 ms at 120 BPM, so a
six-string strum takes about 25 ms, as a real one does), and each
up-stroke on the top four, high to low. How swing moves a strum as a
whole is in [Swing](../sound/tempo.md#swing).

What each style's guitarist plays:

- **Strummed chords** (pop, folk, country): the full shape in the
  style's rhythm.
- **Power chords** (rock, punk, metal): only the root, its fifth and the
  octave, `A5` = A2 E3 A3. Distortion adds new notes at the sums and
  differences of the frequencies played. For a fifth, whose frequencies
  are close to 3:2, they fall in line with the root, an octave or two
  below it; for a third they land on notes out of tune with the chord,
  and it buzzes and sours. So rock guitarists leave the third to the
  bass and the singer, and "A5" means "A without its third".
- **Four to the bar** (swing): three-note chords, the root, the third
  and the seventh, struck on every beat, the second and fourth a little
  harder, as Freddie Green kept time in Count Basie's band for almost
  fifty years. The shapes are the jazz **shells**: from a root on the
  low E string, the seventh comes next and the third an octave up
  (G7 = G2 F3 B3); from the A string, the third and then the seventh
  (C7 = C3 E3 B♭3).
- **The boogie shuffle** (blues, rock and roll): two strings only, the
  root with its fifth on one beat and with its sixth on the next, both
  eighths of each beat: A and E, A and E, A and F♯, A and F♯. Over a
  minor chord it stays on the fifth.
- **The chop** (reggae, soul, Motown, bluegrass): a short chord on the
  top four strings on beats 2 and 4, with the snare, the low strings
  left to the bass. Country calls it the "chick" of "boom-chick". In
  **ska** the guitar plays between all the beats instead.
- **Scratching** (funk, disco): sixteenth notes on a muted guitar, some
  accented, most only a percussive "chk".

**Strings** hold each chord for as long as it lasts, an octave above
the piano, softly: a pad. Ballads use the slower-starting strings,
synth styles the synthesizer ones. The **organ** holds chords too, as a
church organ in a chorale or a Hammond in a slow blues. In reggae it
plays the **bubble**: short chords between all the beats low down, and
on beats 2 and 4 an octave higher, with the guitar's chop.

Every player plays a General MIDI instrument chosen by the style. The
General MIDI table numbers them from 1, the MIDI messages from 0, so
the code says 24 for the nylon guitar:

| Program (from 0) | Instrument | Styles |
|---|---|---|
| 24 | Acoustic guitar (nylon) | bossa nova, samba, bolero, folklore, rumba flamenca |
| 25 | Acoustic guitar (steel) | pop, folk, country, bluegrass, waltz |
| 26 | Electric guitar (jazz) | swing |
| 27 | Electric guitar (clean) | shuffle, rock and roll, surf, Motown, reggae, ska |
| 28 | Electric guitar (muted) | funk, disco |
| 29, 30 | Overdriven, distortion guitar | rock, punk, metal |
| 48, 49, 50 | Strings, slow strings, synth strings | pop, tango, ballads, R&B, trance |
| 16, 17, 18, 19 | Drawbar, percussive, rock, church organ | blues, reggae, rock, chorale |

### Players in arrays

The band keeps one channel and one fingerprint per player, in arrays
indexed by the player:

```rust
pub const COUNT: usize = 8;

pub const ALL: [Player; Player::COUNT] = [
    Player::Piano,
    Player::Bass,
    Player::Drums,
    Player::Guitar,
    Player::Strings,
    Player::Organ,
    Player::Soloist,
    Player::Horns,
];
```

```rust
pub(crate) channels: [Option<usize>; Player::COUNT],
```

`band.channels[player.index()]` answers "which channel?" for every
player, and `index` is `self as usize`: a field-less enum converts to
its position. The array's length is part of its type,
`[Option<usize>; 8]`, so the compiler checks that `ALL` really lists
`COUNT` players. `write_band` loops over `Player::ALL`, and everything
after it, the patterns, the tracks and the fingerprints that keep an
edited part frozen, works for every player the same way.

### Who plays in which style: a table of constants

Each style's added players are a value of this type:

```rust
pub struct Extra<Part> {
    pub part: Part,
    pub program: u8,
    pub only_in_b: bool,
}
```

```rust
pub struct Extras {
    pub guitar: Option<Extra<GuitarPart>>,
    pub strings: Option<Extra<PadPart>>,
    pub organ: Option<Extra<PadPart>>,
}
```

`Extra` is **generic**: `Part` is a type parameter, filled in as
`GuitarPart` for the guitar and `PadPart` for strings and organ. The
three fields "what it plays, on which instrument, does it wait for b"
are written once for both. The styles are listed in a table built by
the compiler:

```rust
const NONE: Extras = Extras {
    guitar: None,
    strings: None,
    organ: None,
};

const fn with_guitar(part: GuitarPart, program: u8) -> Extras {
    Extras {
        guitar: guitar(part, program),
        ..NONE
    }
}
```

```rust
const BY_STYLE: &[(&str, Extras)] = &[
    // Pop: a strummed acoustic guitar, strings for the chorus.
    (
        "Pop beat",
        Extras {
            guitar: guitar(STRUMMED, STEEL_GUITAR),
            strings: pad_in_b(STRINGS),
            ..NONE
        },
    ),
    // ...
];
```

A `const fn` can be called where Rust needs a value at compile time,
such as in a `const`. `..NONE` is struct update syntax: the fields not
written take their values from `NONE`, so each style names only the
players it has. The test `every_style_listed_is_a_band_style` checks
that every name in the table is a real band style, so a typo cannot
quietly leave a style without its guitar.

### Enums that carry data, and a function as a parameter

What the guitar plays is an enum whose variants can carry a value:

```rust
pub enum GuitarPart {
    Strum(Comping),
    StylesRhythm,
    Chop,
    Power(Comping),
    FourToTheBar,
    Boogie,
}
```

`Strum(Comping::Bossa)` and `Strum(Comping::Waltz)` are one variant with
different rhythms inside. A `match` can look inside it, with `|`
patterns nested, as in variation b, where held and picked chords become
strummed ones:

```rust
pub fn in_variation(self, variation: Variation) -> GuitarPart {
    match (variation, self) {
        (Variation::B, GuitarPart::Power(Comping::Held)) => GuitarPart::Power(Comping::Pulse),
        (
            Variation::B,
            GuitarPart::Strum(Comping::Held | Comping::ArpUp | Comping::ArpUpDown),
        ) => GuitarPart::Strum(Comping::Strum),
        (_, part) => part,
    }
}
```

Strummed chords, power chords and shells differ only in which notes each
harmony gets; playing the rhythm and strumming it is the same. So
`strummed` takes the voicing as a parameter:

```rust
fn strummed(
    rhythm: Comping,
    bars: &[Bar],
    harmonies: &[Played],
    voice: fn(&Played) -> Vec<Note>,
) -> Vec<NoteEvent> {
```

`fn(&Played) -> Vec<Note>` is a **function pointer** type: any plain
function with that signature fits, and the caller passes `shape`,
`power_chord`, `shell` or `top_strings` by name. Finding a shape tries
thousands of fret combinations, and a song repeats its chords, so
`strummed` remembers the shapes it found in a `HashMap`:

```rust
let voicing = shapes.entry(key).or_insert_with(|| voice(harmony)).clone();
```

`entry` finds the place for `key` in the map, and `or_insert_with`
calls the closure only if that place is empty: each different chord is
searched once.

### Trying every fret: recursion with a callback

How many strings a shape uses is not known in advance, so the search
cannot be a fixed number of nested `for` loops. It is a function that
picks a fret for one string and calls itself for the next:

```rust
fn try_frets(
    choices: &[Vec<u8>],
    low: usize,
    top: usize,
    frets: &mut Vec<u8>,
    found: &mut impl FnMut(&[u8]),
) {
    let string = low + frets.len();
    if string > top {
        found(frets);
        return;
    }
    for &fret in &choices[string] {
        frets.push(fret);
        try_frets(choices, low, top, frets, found);
        frets.pop();
    }
}
```

`frets` is one `Vec` shared by every level: a level pushes its string's
fret, lets the deeper levels finish, and pops it again before trying
the next. When every string has a fret, `found` gets the whole shape.
`found` is a closure that scores it and keeps the best one, so it
changes something outside itself: that makes it an `FnMut`, and it is
passed as `&mut` so that every level of the recursion calls the same
closure, rather than each taking its own.

## A horn section

Soul and funk records have one. So do big bands, salsa orchestras, ska
bands and the brass bands of the Balkans: trumpets, saxophones and
trombones playing the chords together. Tick **Horn section** in the
Band menu, and a **Horns** track comes in with the band, playing the
chord sheet in a part that suits the style:

| Style | What the horns play |
|---|---|
| Ballads (Pop ballad, Jazz ballad, Slow blues, Neo soul, R&B, Bossa nova, Bolero…) | long notes held under the band |
| Swing, shuffle, blues, rock and roll | big-band kicks on beat 1 and just before beat 3 (the Charleston rhythm) |
| Pop, rock, soul, hip hop, Latin, second line | short punches in 3 + 3 + 2 (the tresillo) |
| Funk | tight funk stabs in sixteenth notes |
| House and other electronic styles | syncopated stabs, as in house music |
| Reggaeton, moombahton | short chords in the dembow's 3 + 3 + 2 |
| Ska | short chords between the beats |
| March, waltz, polka, Balkan dances | afterbeats, the "pah" of a brass band's oom-pah |
| Tango, folk, classical, reggae, styles without drums | long notes |
| Other styles in 3/4 or 6/8 | long notes |
| Other styles in 5/4, 7/8 and other odd meters | a punch on each beat the bar is felt in |

The **3 + 3 + 2** rhythm splits the eight eighth notes of a 4/4 bar into
groups of three, three and two, with an accent at the start of each: on
beat 1, on the "and" of 2, and on beat 4. Played this way, the bar
pushes forward. It is the tresillo of Cuban music, and it is everywhere
in pop and soul horn lines. The **afterbeats** are the off-beats of a
brass band: in a march in 2/4 the tuba plays "oom" on the beat and the
horns "pah" between; in a waltz the bass has beat 1 and the horns beats
2 and 3. The instrument is **Brass Section** (General MIDI 61) for most
styles, **French Horn** for classical ones and **Synth Brass** for
electronic ones.

![The Band menu with Horn section ticked, and its hint: "A horn section plays the chords in close harmony, each horn moving as little as it can from chord to chord. In Motown, the horns (Brass Section) play short punches in 3 + 3 + 2."](../images/step-9-horns-menu.png)

### Close harmony, moving as little as possible

Horns playing chords together voice them in **close position**: the
notes of the chord as near together as they go, inside an octave, under
the highest horn, the **lead**. Arrangers write horn parts that way
because it makes three or four horns sound like one big instrument. The
horns play the notes the piano plays in the chord, whatever their
octave: as many horns as the piano's voicing has different notes, up to
four. A chord with more than four notes, such as C13 (C E G B♭ D A),
leaves out its fifth first, and then its root, which the bass plays
anyway: the horns play E, B♭, D, A, what makes the chord sound as it
does. The lead stays between F4 and F5, where a trumpet section sounds
bright without straining.

From one chord to the next, the horns take the close voicing that moves
least: each horn goes to the nearest note it can. C to A♭ is E4 G4 C5
to E♭4 A♭4 C5: two horns move a semitone, down and up, and the lead
holds C. A♭ to G7: the lead falls from C5 to B4, the second horn from
A♭4 to G4, the third rises from E♭4 to F4, and a fourth comes in on D4
for the seventh chord. Then from G7 to C, B4 rises to C5 and F4 falls
to E4, as the leading tone and the seventh of a dominant do.

![The Horns part of the Motown example in the piano roll with Voice leading on. From Ab to G7, the line of moves reads "Horns: top C5→B4 · 2nd Ab4→G4 · 3rd Eb4→F4; outer parts in contrary motion". The coloured lines join each horn's notes from chord to chord.](../images/step-9-horns.png)

The horns also avoid **parallel fifths** where they can. Two horns a
perfect fifth apart that move to another perfect fifth sound like one
horn, and the texture loses a line (see [Voice
leading](../theory/voice-leading.md)). So each pair of horns that
would move in parallel fifths counts as 24 semitones of movement, more
than any other way to the next chord, and the horns pick another
voicing.

### In the code

What the horns play is long notes, or short chords in the rhythm of one
of the ways the piano comps:

```rust
pub enum HornPart {
    /// Long notes: each chord held for as long as it lasts.
    Pads,
    /// Short chords in the rhythm of a way of comping.
    Hits(Comping),
}
```

`Comping` already had every rhythm the horns need (Charleston,
Tresillo, Funk, Stabs, Dembow, Offbeat, Aksak for the afterbeats and
Accents), so the horns reuse them. `HornPart::of` reads the style: two
lists of style names first, then the meter, then the genre:

```rust
if style.rhythm.is_none() || HELD.contains(&style.name) {
    return HornPart::Pads;
}
if let Some(&(_, comping)) = BY_STYLE.iter().find(|(name, _)| *name == style.name) {
    return HornPart::Hits(comping);
}
```

`iter().find` returns an `Option<&(&str, Comping)>`, and the pattern
`Some(&(_, comping))` takes the pair apart in one go: `&` for the
reference, `_` for the name it does not need. `close` builds one
voicing from the top down, starting on the lead and taking the nearest
note below that is a chord note no horn plays yet:

```rust
fn close(lead: u8, classes: &[u8]) -> Vec<u8> {
    let mut notes = vec![lead];
    let mut above = lead;
    while notes.len() < classes.len() {
        let Some(below) = (1..12)
            .filter_map(|down| above.checked_sub(down))
            .find(|note| {
                classes.contains(&(note % 12)) && notes.iter().all(|other| other % 12 != note % 12)
            })
        else {
            break;
        };
        notes.push(below);
        above = below;
    }
    notes.reverse();
    notes
}
```

`checked_sub` gives `None` instead of wrapping around below note 0, and
`filter_map` drops those. `choose` tries every lead from F4 to F5 that
is a chord note, builds the close voicing under it, and keeps the
cheapest:

```rust
fn choose(classes: &[u8], last: Option<&[u8]>) -> Vec<u8> {
    (LEAD_LOW..=LEAD_HIGH)
        .filter(|lead| classes.contains(&(lead % 12)))
        .map(|lead| close(lead, classes))
        .min_by_key(|voicing| cost(voicing, last))
        .unwrap_or_default()
}
```

`cost` adds up how far each horn moves, matching the horns from the top
down, plus 24 for each pair in parallel fifths. `min_by_key` returns the
first of the cheapest, and `unwrap_or_default` gives an empty voicing,
silence, for a harmony with no notes. At most thirteen leads are tried,
each with four horns at most, so trying them all is fast; there is no
need for anything cleverer.

## Following the chords you hold

An **arranger keyboard**, the home keyboard with a "style" button,
plays drums, chords and a bass line, and the left hand only tells it
*which chord*. Hold C and the band plays in C; move to A minor and the
same groove carries on in A minor. **MIDI > Accompaniment follows
chords** does that with any song: play it, hold a chord on a MIDI
keyboard (or on the letter keys, with **Typing keyboard to piano**), and
from the next note they play, the chords and the bass line move into
it. The status bar says which chord the accompaniment is in.

![Holding A and C: the accompaniment plays A minor](../images/step-8-follow-status.png)

Like arranger keyboards, it does not need the whole chord:

| Keys held | Chord |
|-----------|-------|
| one key, such as G | its major chord: G |
| two keys a minor third apart, such as D and F | the minor chord on the lower one: Dm |
| two keys a major third or a fifth apart | the major chord on the lower one |
| two keys a minor seventh apart, such as G and F | the seventh chord: G7 |
| three keys or more | the chord they make |

The last chord stays when you let go, as the "memory" button of an
arranger keyboard does. Only a key going *down* asks for a new chord,
so letting go of the keys one by one does not count as playing the keys
left.

Before it can move a note, the app needs to know which chord the note
was written for. It hears each pattern once, with the second ear of
[Hearing harmony](../theory/hearing.md#a-second-ear-for-the-bands-own-patterns),
and keeps what it heard until the project changes. Then every note of a
stretch heard as, say, C major is moved into the chord held, say
A minor. Arranger keyboards have a table for this, written by whoever
made the style; Ferrous Lineup works it out from what each note *does*
in its chord, its role:

```rust
enum Role {
    Root,
    /// The third, or what stands for it in a "sus" chord.
    Third,
    Fifth,
    /// The seventh, or the sixth of a sixth chord.
    Seventh,
    /// A ninth, eleventh or thirteenth.
    Tension,
}
```

Block chords, three notes or more starting together, are voiced again
for the new chord as close as possible to where they were, so the hand
hardly moves. Single notes, as in arpeggios and bass lines, keep their
role: the root of C goes to the root of A minor, the third (E) to the
third (C), the fifth (G) to the fifth (E); a seventh goes to the new
seventh, or to the root if the new chord has none. Notes outside the
chord, such as the passing notes of a walking bass, move as far as the
root does. Each moved note lands in the octave nearest to where it was,
which keeps a bass line low and an arpeggio the same shape:

```rust
fn nearest(pitch_class: u8, near: Note) -> Note {
    let near = i32::from(near.midi());
    let up = (i32::from(pitch_class) - near).rem_euclid(12);
    // Up to a tritone up, or else down.
    let midi = if up <= 6 { near + up } else { near + up - 12 };
    Note::new(midi.clamp(0, 127) as u8)
}
```

`rem_euclid` is the remainder that is never negative: `-3 % 12` is `-3`
in Rust, but `(-3i32).rem_euclid(12)` is `9`, which is what "how far up
to the next A" means.

The song itself never changes: undo, saving and the WAV export see the
notes as written. The moved notes go to the audio thread only, with the
command every edit already uses to replace one channel's notes in one
pattern (see [Sending commands with a ring
buffer](../program/threads.md#sending-commands-with-a-ring-buffer)):

```rust
self.send(Command::SetNotes {
    pattern: moved.pattern,
    channel: moved.channel,
    notes: swing::swung(&moved.notes, swing, hits),
});
```

The audio thread swaps the list between two blocks of sound, so the
next note it starts is in the new chord; notes already sounding ring
out as they were. When playback stops or the option is turned off, the
project's own notes are sent back. "Only a key going down" is one flag,
set on each Note On and read with `std::mem::take`, which returns it
and leaves `false` behind:

```rust
if !std::mem::take(&mut self.live.follow.key_went_down) {
    return;
}
```

## In the file

A project with a band saves its style by name, where it ends, its
channels, the fingerprints, and what it muted. A new song from **File >
New** with C, Am, F and G7 on its sheet saves this:

```ron
band: Some((
        style: "Pop beat",
        end: 64,
        piano: Some(0),
        bass: Some(1),
        guitar: Some(2),
        strings: Some(3),
        written: (Some(14171519900600248065), Some(11668178586522433030), Some(3477506964485452990)),
        written_too: (Some(13661597678858777105), Some(14695981039346656037), None),
        heard: Some(6239919094066095261),
        muted: (),
        since: 0,
    )),
```

and each band pattern carries `player: Some(Piano)` and so on. The
fingerprints are split oddly: `written` holds three, `written_too`
three more. The band in memory has one array of eight, but the file has
to stay readable by older versions where it can. A band of a piano, a
bass and drums was saved as `written: (…, …, …)` before there were
other players, so those three stay where they were, and the others go
in fields of their own that are left out when empty:

```rust
/// The fingerprints of the piano, the bass and the drums.
pub written: [Option<u64>; 3],
/// The fingerprints of the guitar, the strings and the organ, left
/// out when there are none, so a band of three reads as before.
#[serde(default, skip_serializing_if = "none_written")]
pub written_too: [Option<u64>; 3],
/// The fingerprint of the soloist's part.
#[serde(default, skip_serializing_if = "Option::is_none")]
pub written_solo: Option<u64>,
```

`skip_serializing_if` leaves a field out of the file when the function
named says so, and `default` fills it in when a file does not have it.
Turning the array into the file's fields is one pattern:

```rust
let [piano, bass, drums, guitar, strings, organ, soloist, horns] = band.written;
```

An array pattern takes an array apart into names, one per element, and
the compiler checks that there are exactly eight. So a band of three is
written exactly as before, and its song keeps the format it needed; a
band needs format 13, a bigger band 17, a soloist 19, a horn section
28 and another style's drums 29, saved as `drums: Some(Of("Bluegrass"))`
or `drums: Some(Song)` (see [The version
number](../program/saving.md#the-version-number)), and an older
version refuses those with a message instead of silently dropping a
player. A file whose band plays a style this version does not know, or
on a channel that does not exist, is refused with the reason rather
than opened wrong.

## Testing it

`band.rs` has the widest tests of the band:
`a_new_song_is_an_empty_sheet_with_a_band`,
`the_band_plays_what_is_typed`,
`the_band_follows_every_change_of_the_sheet`,
`an_edited_part_stays_until_it_is_played_again`,
`a_frozen_part_stays_frozen_in_the_file`,
`sets_micromodes_and_rows_are_played_by_their_notes`,
`no_chord_silences_every_style`,
`a_style_in_three_puts_the_song_in_three`,
`a_walking_bass_leads_into_each_chord_by_a_semitone` (in Swing and in a
jazz waltz),
`a_band_in_a_demo_mutes_the_songs_own_accompaniment`,
`deleting_a_players_channel_freezes_only_that_part`,
`long_songs_are_cut_into_patterns_and_repeats_share_one`, and for the
style parts `instruments_can_change_without_changing_the_performance`,
`the_progression_is_independent_of_the_rhythm_and_sounds` and
`separate_style_components_survive_saving_and_undo`.
`every_rhythm_is_a_band_style_and_names_are_unique` reads the whole
list. In `comping.rs`, `a_walking_line_leads_into_the_next_chord`
checks C2, E2, G2, F♯2 over C7 going to F,
`a_walking_line_goes_the_way_of_the_next_chord` the four lines of C,
Am, F, G7 and that, from every chord to every root, the last step is a
half step and no step is wider than a sixth, and
`plain_accents_follow_the_grouping_without_a_styled_bass_line` the beats
of an odd meter.

The guitar's tests check the music: `open_chords_are_the_ones_in_chord_books`,
`other_keys_take_barre_chords`, `a_slash_chord_has_its_bass_lowest`,
`every_chord_in_every_key_has_a_shape_with_its_needed_notes`,
`power_chords_and_shells`,
`a_down_stroke_goes_up_the_strings_and_an_up_stroke_comes_down`,
`a_chop_strikes_the_top_four_strings_on_two_and_four` and
`the_boogie_alternates_fifth_and_sixth`. The horns':
`the_horns_move_as_little_as_they_can` (C, A♭, G7, C as above),
`the_horns_never_move_in_parallel_fifths_when_they_can_help_it`,
`a_big_chord_leaves_out_its_fifth_and_root` and
`every_style_has_horns_that_fit_it`; and in `band.rs`,
`horns_come_to_any_style_when_asked` brings them to six styles and
checks that every horn note is a note of its chord, every voicing is in
close position and the lead is between F4 and F5.

Following a held chord is checked by `keys_ask_for_chords`,
`notes_keep_their_role`, `a_seventh_goes_to_the_root_of_a_triad`,
`every_style_follows_any_chord_in_its_chords` (for every style, the
chord channel plays only notes of the chord held) and
`every_demo_song_follows_a_chord`. And the [playback
trace](../sound/trace.md) of a band song lists every note each player
plays, bar by bar, which is how a change to the band is heard without
listening.

Exercises on the band are in [Exercises: the
band](exercises.md).
