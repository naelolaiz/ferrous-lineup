# Borrowed chords, secondary dominants and pivot chords

Files: [`crates/lineup-core/src/chord_sheet/function.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/chord_sheet/function.rs),
[`crates/lineup-core/src/harmony/scale.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/harmony/scale.rs)
and [`crates/lineup-app/src/chord_sheet.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/chord_sheet.rs)

A Roman numeral says where a chord stands in the key. In C major, Dm is
ii and G7 is V7. Most chords of a song are the key's own, built only
from its seven notes. The chords that are not are often the ones you
remember: the minor iv that darkens the last line of a ballad, the D7
that leads into the bridge of a standard. Harmony has names for why those
chords are there, and the chord sheet now writes them under the chords:

- **Borrowed chords**, taken from the minor key on the same tonic.
- **Secondary dominants**, which make another chord of the key sound
  like home for a moment.
- **The Neapolitan chord**, a major chord on the lowered second degree.
- **Pivot chords**, where the music changes key through a chord that
  belongs to both keys.

![Sixteen bars in C major that move to G major. Fm and Ab are marked "borrowed" in blue, E7 and D7/F# are secondary dominants in yellow (V7/vi, V⁶₅/V), Db/F is the Neapolitan sixth in pink (N⁶), and Am in bar 14 is the pivot chord, "vi = ii", in orange. The key change to G major is marked on bar 15.](../images/step-9-chord-functions.png)

## Trying it

1. Make a new song, open the chord sheet (**F5**) and type these
   sixteen bars, one chord in each: C, Fm, Ab, G7, C, E7, Am, D7/F#,
   G, C, Db/F, G, C, Am, D7, G.
2. Right-click bar 15 in the playlist's bar numbers and pick the key
   **G major** from there on. The chord sheet writes "→ G major" on
   bar 15.
3. Point at a coloured numeral to read what the chord does. Point at
   "→ G major" to read how the key changes.

The numerals are coloured by what the chord does: **blue** for a
borrowed chord, **yellow** for a secondary dominant, **pink** for the
Neapolitan, and **orange**, the colour of key changes, for a pivot
chord. The key's own chords stay grey.

## Borrowed chords: modal mixture

C major and C minor share their tonic, C, and their dominant, G. They
are called **parallel keys**. Music in a major key often takes a chord
from its parallel minor without leaving the key. This is called
**borrowing**, or **modal mixture**: the two modes are mixed.

| Chord | In C major | Its notes | From C minor |
|---|---|---|---|
| Fm | iv | F A♭ C | A♭ |
| A♭ | ♭VI | A♭ C E♭ | A♭ and E♭ |
| B♭ | ♭VII | B♭ D F | B♭ |
| E♭ | ♭III | E♭ G B♭ | E♭ and B♭ |
| D° | ii° | D F A♭ | A♭ |
| Bdim7 | vii°7 | B D F A♭ | A♭ |

The ♭ in ♭VI says the chord is built on a lowered sixth degree: A♭,
where C major has A. The tooltip names the notes that come from the
minor key:

> Fm: F minor, iv in C major. A borrowed chord: C major takes it from C
> minor, the minor key on the same tonic (this is called modal
> mixture). Its Ab comes from C minor, not C major: it darkens the
> major key for a moment without leaving it.

Borrowing goes the other way too. A piece in A minor that ends on A
major borrows its last chord from A major. That ending has a name of
its own, the **Picardy third**, and the tooltip says so when a minor
piece ends that way.

Bdim7 is in the list because a major key also borrows from the
**harmonic** minor, the minor scale with a raised seventh. In C minor
that raised seventh is B, the leading tone, and Bdim7 (B D F A♭) is
the leading-tone chord of C minor.

## Secondary dominants: V of V

In C major, G7 is V7, the dominant. Its third, B, is the leading tone,
a semitone below C, and it pulls up to C. That pull is what makes C
sound like home.

Any major or minor chord of the key can get a dominant of its own. D7
is the dominant of G: its third, F♯, is a semitone below G and pulls up
to it. F♯ is not in C major, so for a moment G sounds like a tonic,
the home of a key of its own. This is called a **tonicization**. D7 in
C major is written **V7/V**, "five seven of five": the V7 of the chord
that is V in C.

![Pointing at D7/F#: "A secondary dominant: D7/F# is the dominant of G, the V of C major, as if G were a tonic. Its third, F#, is the leading tone of G, a semitone below it, and pulls up to it, so G sounds for a moment like a key of its own (a tonicization). Here it goes on to G, as a dominant goes to its tonic."](../images/step-9-chord-functions-secondary.png)

The figures of an inversion go on the V. D7 with F♯ in the bass is
V⁶₅/V, and the F♯ in the bass rises straight to G.

Every major and minor chord of the key can be tonicized, except the
tonic, which needs no help, and vii°, which is diminished and cannot be
a tonic. In C major:

| Chord | Its dominant | Written | Its leading tone |
|---|---|---|---|
| Dm (ii) | A7 | V7/ii | C♯ |
| Em (iii) | B7 | V7/iii | D♯ |
| F (IV) | C7 | V7/IV | E |
| G (V) | D7 | V7/V | F♯ |
| Am (vi) | E7 | V7/vi | G♯ |

C7 is the odd one. Its leading tone, E, is in C major already. What
makes C7 sound like a dominant is its seventh, B♭, which the tooltip
names: "Its Bb is not in C major."

A secondary dominant does not always go where it leads. E7 going to F
instead of Am is a **deceptive** move, like V going to vi. The chord
is still V7/vi, and the tooltip adds "Here it goes to F instead of
Am."

It may also go there by way of a **six-four chord**: a triad with its
fifth in the bass, such as C/G. D7 going to C/G and then to G7 is the
textbook way into a cadence. The bass has already reached G, and the C
and E above it fall to B and D a moment later. This is the **cadential
six-four**. The tooltip says that the bass has arrived and the G chord
comes when the two notes above it fall a step.

### Secondary leading-tone chords

A diminished chord can do a dominant's work too. In C major, Bdim is
vii°, built on the leading tone, and it pulls into C as V does. F♯dim7
is built on F♯, the leading tone of G, so it pulls into G. It is
written **vii°7/V**.

Unlike a secondary dominant, it is only named that way when it goes to
its chord, or to a six-four chord over it. Autumn Leaves has F♯m7♭5
going to B7 and then Em, a ii–V–i in E minor. Read in C major, as on
the page about [pasting chord sheets](text-chord-sheets.md), F♯m7♭5 is
still the ii of E minor there, not the vii of G. So the chord sheet
keeps its plain numeral, ♯ivø7.

### In a minor key

In A minor, the dominant is E or E7, with G♯ raised from the scale's G.
The chord sheet reads secondary dominants against the **natural** minor
scale. The dominant's own dominant is B7, with D♯, written V7/V. When
B7 goes to Em, a minor v, it is written V7/v.

A major tonic chord in minor is two things at once. A7 going to Dm is
V7/iv, the dominant of the subdominant. A major at the end of the
piece is the Picardy third, borrowed from A major.

## The Neapolitan sixth

The **Neapolitan** chord is a major chord on the lowered second degree:
D♭ major in C, where the key has D. It is named after the Neapolitan
opera composers of the 1700s, who used it often. It is most often
played with its third in the bass, D♭/F, so it is known as the
**Neapolitan sixth**, written N⁶ (a chord with its third in the bass is
a "six" chord in figured bass). It leads to the dominant, and its D♭
falls to B, the leading tone. That fall is a diminished third, a
striking melodic step.

The chord sheet writes N⁶ only for a major chord spelled on the second
letter of the key, D♭ in C. C♯ major is the same notes on the piano, but
spelled on C it is a chromatic chord on the tonic, not a Neapolitan.

## Pivot chords: changing key

A **modulation** is a move to a new key that the music stays in. The
smoothest way there goes through a chord the two keys share. In C
major, Am is vi. In G major, Am is ii. When the music goes C, Am, D7,
G, the ear hears Am as vi in C. Then D7 comes, which C major does not
have, and the ear hears that Am was really ii in G. Am is the **pivot
chord**: it turns the music from one key to the other.

The chord sheet looks for the pivot around each key change. It finds
the first chord near the change that belongs to the new key and not to
the old one. That chord is the **arrival** (D7 here). The chord before
it is the pivot if it belongs to both keys. The pivot gets both
numerals, old key first: `vi = ii · pivot`.

![Pointing at the pivot chord: "Am: A minor, vi in C major. The pivot chord of the modulation to G major: Am is vi in C major and ii in G major, a chord of both keys. The ear hears it in C major; D7, which C major does not have, comes next and shows that it was the ii of G major."](../images/step-9-chord-functions-pivot.png)

Where the key marker goes is up to you. Musicians mark the new key
where they feel it has arrived, which can be a bar or two after the
pivot. If the marker is on the G, the D7 before it already belongs to
G major. The chord sheet reads it in G major anyway and writes
`V7 · in G major` under it, in orange.

Not every key change has a pivot:

- **A direct modulation** goes straight to a chord of the new key from
  one that is not in it, as a song does when it moves up a semitone for
  its last chorus. The key change's tooltip says "a direct modulation,
  with no pivot chord".
- **A change of mode** keeps the tonic and changes the mode, from C
  major to C minor. No chord is needed to lead there.

A borrowed chord can be a pivot too. Fm is iv borrowed from C minor,
and it is vi in A♭ major. In C, F, Fm, E♭7, A♭, Fm is the pivot
`iv = vi`, and E♭7 is the arrival.

## In the code

### Reading the whole sheet at once

A chord's numeral used to depend on the chord and the key alone. Now it
depends on its neighbours too: a secondary leading-tone chord is named
by where it goes, and a pivot is found by looking at the chords around
a key change. So `Functions::read` reads the whole chord sheet in one
pass, and the sheet asks it about each chord:

```rust
pub struct Function {
    pub start: usize,
    pub symbol: ChordSymbol,
    pub key: Scale,
    pub role: Role,
    pub next: Option<ChordSymbol>,
    pub last: bool,
    pub early: bool,
    pub pivot: Option<Modulation>,
}
```

`next` is the next chord written that is not the same chord again: C in
one bar and C in the next have not moved on.

### An enum with data in its variants

What a chord does is a `Role`:

```rust
pub enum Role {
    Plain,
    Borrowed { from: Scale },
    SecondaryDominant { of: Chord },
    SecondaryLeadingTone { of: Chord },
    Neapolitan,
}
```

Each variant carries exactly what it needs. A borrowed chord knows the
key it comes from, a secondary chord knows the chord it leads to, and
the Neapolitan needs nothing more. A `match` gets the data out where it
is used, so `of` exists only in the arms where it means something:

```rust
match self.role {
    Role::SecondaryDominant { of } => Some(with_figures(
        &format!("V{}/{}", chord.quality.numeral_suffix(), self.target_numeral(of)?),
        figures,
    )),
    Role::Neapolitan => Some(with_figures("N", figures)),
    Role::Plain | Role::Borrowed { .. } => figured(self.key, self.symbol),
    // ...
}
```

`Role::Borrowed { .. }` matches a borrowed chord whatever key it comes
from: `..` ignores the fields.

### Which role comes first

Some chords could be two things. In A minor, A major is both the
dominant of Dm and the major tonic chord borrowed from A major. `role`
decides by where the chord goes:

```rust
let goes_to = |of: Chord| next.is_some_and(|next| arrives(next, of));
let dominant = key.secondary_dominant_target(chord);
let leading = key.secondary_leading_tone_target(chord);
let secondary = match (dominant, leading) {
    (Some(of), _) => Some(Role::SecondaryDominant { of }),
    (_, Some(of)) if goes_to(of) => Some(Role::SecondaryLeadingTone { of }),
    _ => None,
};
if let Some(of) = dominant.or(leading)
    && goes_to(of)
{
    return secondary.unwrap_or(Role::Plain);
}
if let Some(from) = key.borrowed_from(chord) {
    return Role::Borrowed { from };
}
if is_neapolitan(key, symbol) {
    return Role::Neapolitan;
}
secondary.unwrap_or(Role::Plain)
```

A chord that goes to the chord it leads to is a secondary chord. Then
comes borrowing, then the Neapolitan. Last comes a secondary dominant
that goes elsewhere. `match (dominant, leading)` matches on a tuple of
two `Option`s, and the guard `if goes_to(of)` adds a condition to a
pattern.

`is_some_and` asks a question about what is inside an `Option` and
answers `false` for `None`, so the last chord, with no next chord, goes
nowhere.

### Finding the pivot

`modulation` looks at the chords from the key change before to the one
after. Two closures say what each chord is to the two keys:

```rust
let new_only = |chord: &Function| {
    let chord = chord.symbol.chord();
    belongs(to, chord) && !heard_in(from, chord)
};
let common = |chord: &Function| {
    let chord = chord.symbol.chord();
    belongs(to, chord) && heard_in(from, chord)
};
```

`heard_in` counts the old key's borrowed chords as its own. That is why
Fm, borrowed in C major, can be a pivot into A♭ major. The arrival is
the first `new_only` chord. It is looked for just before the marker
first, in case the new key came early, and then after it. The pivot is
the chord before the arrival, if it is `common`:

```rust
change.pivot = arrival
    .checked_sub(1)
    .map(|before| range[before])
    .filter(|before| common(before))
    .map(|before| (before.start, before.symbol));
```

`checked_sub(1)` gives `None` when the arrival is the first chord, so a
change that starts on its arrival has no pivot, and no index ever goes
below zero.

## How we know it is right

The tests in `function.rs` are small chord sheets in the style of a
harmony textbook. Each one checks the numerals and what the tooltips
say:

- C Am A♭ Fm G7 C: ♭VI and iv are borrowed from C minor.
- C E7 Am D7/F♯ G7 C A7 Dm: V7/vi, V⁶₅/V and V7/ii, each going to its
  chord.
- E7 going to F: still V7/vi, and the tooltip says it goes to F instead
  of Am.
- C7 going to F: V7/IV, whose B♭ is named.
- F♯dim7, G♯dim and C♯m7♭5: vii°7/V, vii°/vi and viiø7/ii.
- Autumn Leaves in C: F♯m7♭5 going to B7 keeps ♯ivø7.
- D7 and F♯dim7 going to C/G: the cadential six-four.
- D♭/F in C minor is N⁶, and C♯ in C major is not a Neapolitan.
- B7 in A minor is V7/V, or V7/v going to Em.
- A7 in A minor going to Dm is V7/iv, and A major at the end is a
  Picardy third.
- C, Am, D7, G into G major with the marker on the Am, the D7 or the G:
  Am is the pivot each time.
- Fm as a borrowed pivot into A♭ major, a direct modulation to D♭ major,
  and a change of mode from C major to C minor.

## Exercises

1. Type `C Fm C` and then `C F C`. Listen to both. The only note that
   changes is A to A♭. What does it do to the mood? Find a song you
   know that does this.
2. In G major, which chord is V7/V? Which note of it is not in G major,
   and where does that note want to go? Type it on the chord sheet and
   check.
3. Write a modulation from C major to F major through a pivot. Which
   chords do the two keys share? Which chord shows that F major has
   arrived? (Hint: which note does F major have that C major does not?)
4. `is_neapolitan` checks the chord's spelling with
   `key.tonic_name().letter.steps_up_to(symbol.root.letter) == 1`.
   Explain why C♯ major in C fails that test while D♭ major passes,
   though both are the same keys of the piano.
5. The chord sheet does not name the **augmented sixth** chords
   (Italian, French and German), which also lead to the dominant. Look
   up their notes in C minor. Which of them sounds like a dominant
   seventh chord on A♭, and what in its spelling tells the two apart?
