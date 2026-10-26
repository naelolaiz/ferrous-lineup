# The mixer in score order: the drums together

Files: [`crates/lineup-core/src/score_order.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/score_order.rs)
and [`crates/lineup-app/src/app/mixer.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/mixer.rs)

The mixer used to list the channels in the order they were added to
the channel rack. A new song starts with Kick, Clap, Hat, Snare and a
Lineup Osc; a style that needs a rim shot, a bass, a cowbell and a
conga adds them at the end, so the mixer read Kick, Clap, Hat, Snare,
Lineup Osc, **Rim**, Bass, **Cowbell**, **Conga**: the drums scattered
around the piano and the bass.

Now the mixer lists the channels the way a conductor's score lists the
instruments, in sections with a heading each, and all the drums sit
together under **Drums**. Every channel keeps its own volume, mute and
solo, exactly as before. Under the drums, the Instrument column now
says **Built-in drum** or **Sample** (a sound file) instead of "Sample
/ drum".

![The mixer of the starting song after Bossa nova and Salsa montuno: under "Drums", Cowbell, Clap, Conga, Hat, Snare, Rim and Kick; under "Harp and guitars", the Bass (Electric Bass, finger); under "Keyboards", the Lineup Osc playing an Acoustic Grand Piano. Each row has its own volume slider, mute box and S for solo.](../images/step-9-mixer-score-order.png)

## Trying it

1. Start from **File > New**, or the song that opens with the app.
2. In the **Library**, click **Bossa nova**, then **Salsa montuno**.
   The channel rack now ends with Rim, Bass, Cowbell and Conga.
3. Press **F9** for the mixer. The drums are under **Drums**, the bass
   under **Harp and guitars**, the piano under **Keyboards**.
4. Click **S** on the Rim row: in the channel rack (**F6**), only Rim's
   light stays on. The mixer shows the rows in another order, but each
   row still belongs to its own channel.

Only the mixer changes its order. The channel rack, the piano roll and
the saved file keep the channels in the order they were added, and
nothing sounds different.

## Score order

An orchestral score has one staff per instrument, and every score puts
them in the same order, so a conductor finds the oboes or the cellos
without searching. The order goes by family, and inside a family
mostly from high to low:

| Section | Instruments, top to bottom |
|---|---|
| Woodwinds | piccolo, flutes, oboes, English horn, clarinets, saxophones, bassoons |
| Brass | horns, trumpets, trombones, tuba |
| Percussion | timpani; glockenspiel, xylophone, vibraphone, marimba, tubular bells; the rest |
| Drums | hand percussion, then the drum kit |
| Harp and guitars | harp, guitars, banjo, koto, bass guitar |
| Keyboards | celesta, piano, harpsichord, organ, accordion and bandoneon |
| Synths | synth leads, pads and effects; a Lineup Osc with no instrument |
| Voices | a solo voice, then the choir |
| Strings | string sections, violins, violas, cellos, double basses |

The horns come above the trumpets, although they sound lower, because
the horns blend with the woodwinds and were long written next to them.
The strings come last, so the double basses, the lowest part of the
orchestra, end the page. Voices go just above the strings, the way a
choir appears in a requiem or an oratorio.

The mixer doesn't look at a channel's name to place it. It looks at the
instrument: the General MIDI program the channel plays (see
[Real instrument sounds](soundfont.md)). A channel called "Lead" that
plays a flute goes with the woodwinds; one that plays a violin goes
with the strings. The program numbers mostly come in families already
(the brass are 56 to 63), but not always the way a score wants them:
General MIDI puts the timpani with the strings and the celesta with the
mallets, and the score puts the timpani first in the percussion and the
celesta with the keyboards.

### The drums

A drum kit is written on one staff, with each drum on its own line or
space: the cymbals on and above the top line (the crash highest, then
the hi-hat, then the ride), the high tom in the top space, the snare in
the middle, the floor tom below it and the kick in the bottom space. The
**Drums** section follows that staff from top to bottom: Crash, Hat,
Open hat, Ride, High tom, Snare, Rim (a cross-stick, written where the
snare is), Low tom, Kick. Hand percussion (Cowbell, Shaker, Clap,
Conga) usually has staves of its own, above the kit, so it comes
first. A channel playing a sound file, and a MIDI Out channel sending
to MIDI channel 10 (the General MIDI drums), go with the drums too.

## The code

### Where a channel goes

`score_order.rs`, in `lineup-core`, has no UI code, so it can be tested
with plain `cargo test`. A `Section` is an enum in score order:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Section {
    Woodwinds,
    Brass,
    Percussion,
    Drums,
    HarpAndGuitars,
    Keyboards,
    Synths,
    Voices,
    Strings,
}
```

Deriving `PartialOrd` and `Ord` on an enum orders its variants as they
are written: `Section::Woodwinds < Section::Brass`. The order of the
score is the order of the source code.

Each section lists its programs, top of the score first, and `place`
finds a channel's section and its position in it:

```rust
pub fn place(channel: &Channel) -> (Section, usize) {
    match &channel.kind {
        ChannelKind::Sampler(SampleSource::Builtin(drum)) => {
            let rank = DRUMS.iter().position(|d| d == drum).unwrap_or(DRUMS.len());
            (Section::Drums, rank)
        }
        ChannelKind::Sampler(SampleSource::File(_)) => (Section::Drums, DRUMS.len() + 1),
        ChannelKind::MidiOut {
            channel: DRUM_CHANNEL,
        } => (Section::Drums, DRUMS.len()),
        ChannelKind::Synth | ChannelKind::MidiOut { .. } => match channel.program {
            Some(program) => program_place(program),
            None => (Section::Synths, usize::MAX),
        },
    }
}
```

`ChannelKind::MidiOut { channel: DRUM_CHANNEL }` matches only MIDI Out
channels whose field equals the constant `DRUM_CHANNEL` (9, shown as
10). A `const` can stand in a pattern like a literal number. The next
arm, `MidiOut { .. }`, catches every other MIDI channel; `match` tries
the arms from the top, so the drum channel never gets that far.

`program_place` looks for the program in every section's list:

```rust
fn program_place(program: u8) -> (Section, usize) {
    Section::ALL
        .into_iter()
        .find_map(|section| {
            let rank = section.programs().iter().position(|&p| p == program)?;
            Some((section, rank))
        })
        .unwrap_or((Section::Synths, usize::MAX))
}
```

`find_map` runs the closure on each section and stops at the first one
that returns `Some`. Inside, the `?` after `position` returns `None`
from the closure when the program is not in that section, which tells
`find_map` to try the next. A test checks that each of the 128
programs is in exactly one list, so none is forgotten and none is in
two places.

### Sorting and grouping

```rust
pub fn score_order(channels: &[Channel]) -> Vec<(Section, Vec<usize>)> {
    let mut order: Vec<usize> = (0..channels.len()).collect();
    order.sort_by_key(|&index| place(&channels[index]));
    let mut sections: Vec<(Section, Vec<usize>)> = Vec::new();
    for index in order {
        let section = place(&channels[index]).0;
        match sections.last_mut() {
            Some((last, indices)) if *last == section => indices.push(index),
            _ => sections.push((section, vec![index])),
        }
    }
    sections
}
```

It sorts the channels' **numbers**, not the channels: the mixer needs
to know which channel each row is, to change that channel's volume and
to solo the right one. `sort_by_key` sorts by a key worked out for each
item; here the key is the pair `(Section, usize)`, and tuples compare
field by field, so the section decides first and the place inside it
breaks the tie. `sort_by_key` is a *stable* sort: two channels with the
same key, such as "Lead", "Voice 2" and "Voice 3" all singing Voice
Oohs, keep the rack's order.

The loop then walks the sorted list and starts a new group whenever
the section changes. `last_mut` gives a mutable reference to the last
group, if there is one; the `if` after the pattern (a *match guard*)
only takes the first arm when that group is the same section.

### The mixer window

`mixer.rs` draws a heading row for each section and a row for each of
its channels, using the channel's own number for everything else:

```rust
for (section, indices) in score_order(&levels) {
    ui.label(egui::RichText::new(section.name()).color(theme::ACCENT));
    ui.end_row();
    for index in indices {
        let channel = &mut levels[index];
        // name, instrument, volume, mute ...
        if ui.selectable_label(mix.solo == Some(index), "S").clicked() {
            mix.solo = if mix.solo == Some(index) { None } else { Some(index) };
        }
        ui.end_row();
    }
}
```

`levels` is the mixer's copy of the channels. Rows change it in place
through `levels[index]`; when the window is done, the app compares the
copy with the project and makes one undo step for whatever changed, as
before. The test `the_mixer_keeps_the_drums_together_and_each_one_still_solos_alone`
draws the mixer, checks that a Ride added after the band is drawn
between the Hat and the Snare, clicks the **S** on its row and checks
that the solo went to the Ride's channel.

## Exercises

1. Open *In the Mood*, then *Sing, Sing, Sing*, with the mixer open.
   In which section is each song's "Horns" channel? Look at the
   Instrument column to see why.
2. Open *Danny Boy*, right-click its Lead in the channel rack and pick
   **Instrument: Violin > Pipe > 74 Flute**. Where does it move in
   the mixer?
3. In `score_order.rs`, the saxophones are between the clarinets and
   the bassoons, as most orchestral scores have them. In a big band
   score the saxophones come first, above the trumpets. What would you
   change to get the big band order, and which test would tell you that
   you broke the orchestral one?
