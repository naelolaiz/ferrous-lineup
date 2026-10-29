# Swing and feel

Clap along to a jazz or blues record and say "da-da da-da" with the
eighth notes. It won't fit: the band plays "doo-ba doo-ba", a long note
and a short one, in every beat. That unevenness is **swing**, and it is
why a shuffle bounces and a pop song doesn't.

## The Swing control

**Swing** sits in the playlist's toolbar, next to **Loose**, and on the
chord sheet, next to the key and the tempo. Drag the number up or down,
from **straight** (even eighth notes) to **100%**. Try it on **File >
Demos**, *Midnight Shuffle*, while it plays: the drums, the piano, the
bass and the harmonica all change their feel together, and not a single
note moves in the piano roll. Swing is played, not written: the notes
stay on the grid, and playback moves the second eighth note of each beat
later. Saving keeps it, **Ctrl+Z** takes it back, and WAV and MIDI
exports play it.

How long is the long note?

| Swing | The two eighth notes of a beat | Ratio | Second eighth note at 120 BPM |
|---|---|---|---|
| straight | 1/2 + 1/2 of the beat | 1 : 1 | 250 ms after the beat |
| 50% | 7/12 + 5/12 | 7 : 5 | 292 ms |
| 100% | 2/3 + 1/3, a triplet | 2 : 1 | 333 ms |

The 100% row is how textbooks explain swing: split the beat into three
(a **triplet**) and play the first two parts as one note. That is why
jazz charts write plain eighth notes with "Swing" over the first bar:
writing every pair as a triplet would fill the page with brackets and
threes. Real players don't stick to one ratio. They swing harder at slow
tempos and even out at fast ones, where there is no time for a long-short
pair, so try the whole range.

A swung pair is not the same as a **dotted eighth and a sixteenth**
(3 : 1). That is a rhythm of its own, sharper, like the "Hap-py" of
*Happy Birthday*, and swing leaves it alone: it only moves notes that sit
on the second eighth note of a beat.

## Styles and swing

Each style brings its own swing: **Swing**, **Jazz ballad**, **Jazz
waltz** and **Shuffle** at 100%, **Rock and roll** at 50%, most others
straight. Change it, and the style remembers: the next time you pick it,
in any song, it plays with your amount. The Library's **Swing** option
(see [Styles and the Library](styles.md)) says whether a style brings its
swing to the song. The swing is the whole song's, so it swings the
melody too, as a jazz singer swings the tune.

A section in another style (see [Song form and sections](song-form.md))
swings as that style does: a swing bridge in a straight pop song swings
its eighth notes, and a rock section in a jazz song plays them even.

## Where swing doesn't apply

A **shuffle** is the blues' name for the same long-short feel, usually a
full triplet. Some blues are written in **12/8** instead, four beats of
three eighth notes each, the triplet feel written out (*Trouble in Mind*
in the demos). So the control is greyed out in 6/8, 9/8 and 12/8: the
beats already split in three, and there are no pairs to swing. The same
goes for 5/8 and 7/8, whose beats are the eighth notes themselves. It
works in every time signature counted in quarter notes or half notes,
such as 4/4, 3/4 or 2/2.

Hip hop, funk and "new jack swing" swing their **sixteenth notes**
instead, the pairs inside each eighth note. This control swings eighth
notes only.

## Loose timing

**Loose**, next to Swing in the playlist's toolbar, makes notes land a
little off the grid, as when a person plays: from **off**, as the app
starts, to 40 milliseconds. Each note starts up to that much late, a
different amount each time, and the notes a player strikes together
move mostly together, so a chord is a little late as a whole, not spread
out like a harp. The same song sounds the same each time you play it,
and a WAV export sounds like playback.
