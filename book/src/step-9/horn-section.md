# A horn section for any style

Files: [`crates/lineup-core/src/band/horns.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/band/horns.rs),
[`crates/lineup-core/src/band.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/band.rs)
and [`crates/lineup-app/src/chord_sheet/band.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/chord_sheet/band.rs)

Soul and funk records have one. So do big bands, salsa orchestras, ska
bands and the brass bands of the Balkans: a **horn section**, trumpets,
saxophones and trombones playing the chords together. Until now only the
[big band songs](big-band-songs.md) had horns, written into each song.
Now the band can bring a horn section to any style. It plays the chord
sheet in a part that suits the style, with each horn moving as little as
it can from chord to chord.

![The Band menu with Horn section ticked, and its hint: "A horn section plays the chords in close harmony, each horn moving as little as it can from chord to chord. In Motown, the horns (Brass Section) play short punches in 3 + 3 + 2."](../images/step-9-horns-menu.png)

## Trying it

1. Open the chord sheet (**F5**), pick a style in the Library, such as
   **Motown**, and type a few chords: `C Am7 Dm7 G7 C Ab G7 C`.
2. Open **Band** and tick **Horn section**. The hint says what the
   horns will play in this style. A **Horns** track comes in with the
   band, and the status bar says "The horns play the chords with the
   band".
3. Play. Then try other styles with the horns on: **Swing**, **Pop
   ballad**, **Funk**, **March**, **Reggaeton**.

Untick **Horn section** to send them home ("No horns"). The **Horns**
line above it in the Band menu is the player, like Piano or Bass: click
its instrument to change the sound. The horns can be locked and edited
like any other player.

## What the horns play in each style

A horn section does not play the same thing in every style. The part
comes from the style:

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
| Tango, folk, classical, reggae | long notes |
| Other styles in 3/4 or 6/8 | long notes |
| Other styles in 5/4, 7/8 and other odd meters | a punch on each beat the bar is felt in |

The **3 + 3 + 2** rhythm splits the eight eighth notes of a 4/4 bar
into groups of three, three and two, with an accent at the start of each
group: on beat 1, on the "and" of 2, and on beat 4. Played this way,
the bar pushes forward. It is the tresillo of Cuban music, and it is
everywhere in pop and soul horn lines.

The **afterbeats** are the off-beats of a brass band. In a march in 2/4
the tuba plays "oom" on the beat and the horns play "pah" between. In a
waltz the bass has beat 1 and the horns beats 2 and 3.

A style with no drums, such as a chorale or a piano ballad, gets long
notes too: there is no groove to punch.

The instrument comes from the style's instruments: **Brass Section**
(General MIDI 61) for most styles, **French Horn** for classical ones
and **Synth Brass** for electronic ones.

If a bar is in another meter than the style's rhythm (a 3/4 bar in a
4/4 funk song, say), the horns punch the beats that bar is felt in, and
go back to the style's rhythm after it.

## Close harmony

Horns playing chords together voice them in **close position**: the
notes of the chord as near together as they go, all inside an octave,
under the highest horn, the **lead**. Arrangers write horn parts that
way because it makes three or four horns sound like one big instrument.

![The Horns part of the Motown example in the piano roll with Voice leading on. From Ab to G7, the line of moves reads "Horns: top C5→B4 · 2nd Ab4→G4 · 3rd Eb4→F4; outer parts in contrary motion". The coloured lines join each horn's notes from chord to chord.](../images/step-9-horns.png)

- **How many horns.** The horns play the notes the piano plays in
  the chord, whatever their octave: as many horns as the piano's
  voicing has different notes, up to four. In the example, C is E4 G4
  C5 and G7 is D4 F4 G4 B4.
- **A big chord.** A chord with more than four notes, such as a C13 (C
  E G B♭ D A), leaves out its fifth first, and then its root, which the
  bass plays anyway. What is left is what makes the chord sound as it
  does: the third, the seventh and the notes above. The horns play E,
  B♭, D, A.
- **The lead** stays between F4 and F5, where a trumpet section sounds
  bright without straining.

## Moving as little as possible

From one chord to the next, the horns take the close voicing that moves
least: each horn goes to the nearest note it can. In the example, C to
A♭ is E4 G4 C5 to E♭4 A♭4 C5. Two horns move a semitone down and up,
and the lead holds C. A♭ to G7 is the picture: the lead falls from C5 to
B4, the second horn from A♭4 to G4, and the third rises from E♭4 to F4,
while a fourth horn comes in on D4 for the seventh chord. Then from G7
to C, B4 rises to C5 and F4 falls to E4, as the leading
tone and the seventh of a dominant do.

The horns also avoid **parallel fifths** where they can. Two horns a
perfect fifth apart that move to another perfect fifth sound like one
horn, and the texture loses a line. Close voicings rarely have fifths
next to each other, but a voicing that moves least can still make them.
So each pair of horns that would move in parallel fifths counts as
24 semitones of movement, more than any other way to the next chord,
and the horns pick another voicing.

With **Voice leading** ticked in the piano roll, you can watch each horn
move, as [How each part moves](part-motion.md) describes.

## In the code

### Choosing the part

`HornPart` is what the horns play. Long notes, or short chords in the
rhythm of one of the ways the piano comps:

```rust
pub enum HornPart {
    Pads,
    Hits(Comping),
}
```

`Comping` already had every rhythm the horns need, Charleston, Tresillo,
Funk, Stabs, Dembow, Offbeat, Aksak (afterbeats) and Accents, so the
horns reuse them. `HornPart::of` reads the style: first two lists of
style names, then the meter, then the genre:

```rust
if style.rhythm.is_none() || HELD.contains(&style.name) {
    return HornPart::Pads;
}
if let Some(&(_, comping)) = BY_STYLE.iter().find(|(name, _)| *name == style.name) {
    return HornPart::Hits(comping);
}
```

`HELD` is a `const` array of the ballads, and `BY_STYLE` pairs a style
name with its rhythm: `("Ska", Comping::Offbeat)`. `iter().find` returns
an `Option<&(&str, Comping)>`, and the pattern `Some(&(_, comping))`
takes the pair apart in one go: `&` for the reference, `_` for the name
it does not need.

### A close voicing under a lead

`close` builds one voicing from the top down. It starts on the lead
and keeps taking the nearest note below that is a chord note no horn
plays yet:

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

`let ... else` is a pattern that must match, or else the `else` block
runs and has to leave (here with `break`). It keeps the happy path
unindented. `checked_sub` gives `None` instead of wrapping around below
note 0, and `filter_map` drops those.

### The voicing that moves least

`choose` tries every lead from F4 to F5 that is a chord note, builds the
close voicing under it, and keeps the cheapest:

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
down, and adds `PARALLEL_FIFTHS` (24) for each pair that moves in
parallel fifths. `min_by_key` returns the first of the cheapest, and
`unwrap_or_default` gives an empty voicing, silence, for a harmony with
no notes. At most thirteen leads are tried, each with four horns at
most, so trying them all is fast. There is no need for anything
cleverer.

`voiced` runs `choose` through the whole song, each chord from the one
before. A rest keeps the last voicing, so the horns come back in where
they left off.

## How we know it is right

- **Voicing tests** in `horns.rs`: C, A♭, G7, C moves as described
  above. No two horns ever move in parallel fifths in a long progression
  that tempts them to. C13 is played E, B♭, D, A. Swing gets kicks,
  Motown punches, a jazz ballad, a zamba and a chorale long notes, a
  rachenitsa afterbeats and a rock style in 5/4 a punch on each beat. A
  chorale gets French horns and Techno synth brass.
- **Band tests** in `band.rs` bring the horns to six styles: Motown,
  Swing, a chorale, Techno, a zamba and a rachenitsa in 7/8. Every
  horn note is a note of its chord, every voicing is in close position
  within an octave, and the lead is between F4 and F5. They check where
  the hits fall: a pop ballad holds each bar from its first step, swing
  kicks on beat 1 and just before beat 3, a march plays on the
  afterbeats. A song with horns saves in format 28 and comes back the
  same.
- **The playback trace** of the Motown example lists the horns' notes,
  bar by bar, as the piano roll shows them.

## Exercises

1. Turn the horns on in **Swing** and in **Pop ballad**. Point at the
   horns' notes in the piano roll. Which style's horns move more? Why do
   long notes need smooth voice leading even more than short ones?
2. Type `C Am F G` in a style with horn hits, and write down each
   chord's horn voicing from the piano roll. Which horn holds a note
   from one chord to the next? (That held note is a **common tone**.)
3. `cost` charges 24 semitones for each pair of parallel fifths. What
   could go wrong if it charged only 2? Write a test in `horns.rs` with
   a progression where the cheapest voicing would move in parallel
   fifths, and check what `voiced` picks.
4. `HELD` and `BY_STYLE` name styles by their names. What happens to a
   style renamed in the library? Find the test that would catch it.
