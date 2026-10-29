# Chords from outside the key

Most chords of a song are the key's own, built from its seven notes.
The ones that are not are often the ones you remember: the minor iv
that darkens the last line of a ballad, the D7 that leads into the
bridge of a standard. Harmony has names for why they are there, and the
chord sheet writes them under the chords, each in a colour:

| Colour | Kind | Written |
|---|---|---|
| blue | a chord **borrowed** from the minor key on the same tonic | iv · borrowed |
| yellow | a **secondary dominant**, the dominant of another chord | V7/vi |
| pink | the **Neapolitan** chord | N⁶ |
| orange | a **pivot chord** into a new key | vi = ii · pivot |

The key's own chords stay grey. Point at a numeral and its tooltip
explains what the chord does and which of its notes are from outside
the key.

![Sixteen bars on the chord sheet, in C major moving to G major: C, Fm (iv · borrowed) and Ab (bVI · borrowed) in blue, G7, C, E7 (V7/vi) in yellow, Am, D7/F# (V⁶₅/V) in yellow, G, C, Db/F (N⁶) in pink, G, C, Am (vi = ii · pivot) in orange, D7 with "→ G major" over it, and G](images/outside-the-key.png)

To try it, make a new song, open the chord sheet and type these sixteen
bars: C, Fm, Ab, G7, C, E7, Am, D7/F#, G, C, Db/F, G, C, Am, D7, G.
Then right-click bar 15 in the playlist's bar numbers and choose **G**
under **Key from bar 15**. The chord sheet colours the numerals as in
the picture.

## Borrowed chords

C major and C minor share their tonic, C, and their dominant, G: they
are **parallel keys**. Music in a major key often takes a chord from
its parallel minor for a moment without leaving the key. This is
**borrowing**, or **modal mixture**:

| Chord | In C major | Its notes | From C minor |
|---|---|---|---|
| Fm | iv | F A♭ C | A♭ |
| A♭ | ♭VI | A♭ C E♭ | A♭ and E♭ |
| B♭ | ♭VII | B♭ D F | B♭ |
| E♭ | ♭III | E♭ G B♭ | E♭ and B♭ |
| D° | ii° | D F A♭ | A♭ |
| Bdim7 | vii°7 | B D F A♭ | A♭ |

The ♭ in ♭VI says the chord is built on a lowered sixth note: A♭ where
C major has A. A borrowed chord darkens the major key without leaving
it. **iv to I**, Fm to C, is the minor plagal cadence: the A♭ falls to
G. **♭VII to I**, B♭ to C, is the rock cadence. The **Minor plagal**
demo is built on the first.

Borrowing goes the other way too. A piece in A minor that ends on A
major borrows its last chord from A major: that bright ending is the
**Picardy third**, and has a demo of its own. The tooltip names it when
a minor piece ends that way.

## Secondary dominants

In C major, G7 is the dominant: its B is a semitone below C and pulls up
to it, which makes C sound like home. Any major or minor chord of the
key can get a dominant of its own. **D7** is the dominant of G: its F♯
is a semitone below G and pulls up to it. F♯ is not in C major, so for
a moment G sounds like a home of its own. This is a **tonicization**,
and D7 in C major is written **V7/V**, "five seven of five".

| Chord | Its dominant | Written | The note from outside |
|---|---|---|---|
| Dm (ii) | A7 | V7/ii | C♯ |
| Em (iii) | B7 | V7/iii | D♯ |
| F (IV) | C7 | V7/IV | B♭ |
| G (V) | D7 | V7/V | F♯ |
| Am (vi) | E7 | V7/vi | G♯ |

C7 is the odd one: its leading tone, E, is in C major already, and it
is the seventh, B♭, that makes it sound like a dominant.

An inversion's figures go on the V: D7 with F♯ in the bass is **V⁶₅/V**,
and the F♯ in the bass rises straight to G. A secondary dominant
doesn't always go where it leads: E7 going to F instead of Am is a
deceptive move, like V to vi, and the tooltip says so.

A **diminished** chord can do a dominant's work too. F♯dim7 is built on
F♯, the leading tone of G, so it pulls into G: it is written
**vii°7/V**.

### In a minor key

A minor key takes its chords from all three of its scales (natural,
harmonic and melodic), so in A minor, C, Dm, E7, F, G and G♯dim7 are
all its own. A secondary dominant there brings a note none of the
three has: C♯, D♯ or A♯. A7 going to Dm is **V7/iv**.

In a **mode**, such as D Dorian or E Phrygian, the chord sheet writes
numerals only. Borrowed chords and secondary dominants are names from
major and minor keys, and a mode's chords do other things: E major in
E Phrygian is the home chord of much flamenco, not the dominant of A
minor.

## The Neapolitan sixth

The **Neapolitan** chord is a major chord on the lowered second note:
D♭ major in C, where the key has D. It is named after the opera
composers of Naples in the 1700s, who used it often. It is usually
played with its third in the bass, D♭/F, and so is called the
**Neapolitan sixth**, written **N⁶**. It leads to the dominant, and its
D♭ falls to B, the leading tone: a striking step down. The
**Neapolitan** demo plays it in a minor key, where it is most at home.

## Pivot chords: changing key

A **modulation** is a move to a new key that the music stays in. The
smoothest way there goes through a chord the two keys share. In C
major, Am is vi; in G major, it is ii. When the music goes C, Am, D7, G,
the ear hears Am as vi in C. Then D7 comes, which C major doesn't have,
and the ear hears that Am was really ii in G. Am is the **pivot**: the
chord sheet writes both numerals, the old key first, **vi = ii ·
pivot**, and **→ G major** where the key changes.

Not every key change has a pivot:

- a **direct modulation** goes straight into the new key, as a song does
  when it moves up a semitone for its last chorus;
- a **change of mode** keeps the tonic and changes the scale, from C
  major to C minor, and needs no chord to lead there.

The tooltip on the key change says which it is. Key changes themselves
are set on the playlist's bar numbers: see [Keys and
modes](keys-and-modes.md#key-changes-in-a-song).

## Hearing them in the demos

The demos under **File > Demos > Styles > Classical** include short
progressions that show these chords: **Minor plagal** (iv to I), **Picardy third**,
**Neapolitan** and **Chromatic mediants** (major chords a third apart,
as in film music). Songs are full of them too. Open the chord sheet of
a jazz standard such as *Rhythm changes* and look for the yellow
numerals.
