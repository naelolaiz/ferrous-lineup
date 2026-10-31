# Beyond keys: sets, rows and micromodes

Everything in the chapters before assumed a key: a home note, and
chords that lean towards it. Much of the music of the 20th century has
no key at all, and chord names don't fit its harmonies. Composers and
theorists found other ways to think about it, and Ferrous Lineup has
three of them:

- **pitch-class sets**, a name for every group of notes, chord or not;
- **twelve-tone rows**, all twelve notes in an order that a whole piece
  is made of;
- **micromodes**, the twelve colours of three notes that the composer
  Francisco Kröpfl taught in Buenos Aires.

Each has a window in the piano roll's toolbar (**Row…**, **Sets…**,
**Micromodes…**), and each can be the key of a song.

## Pitch-class sets

**Pitch classes.** Number the twelve notes from C: C is 0, C♯ is 1, up
to B, 11. Octaves and spellings don't count: every C is 0, and C♯ and
D♭ are both 1. A **set** is a choice of some of the twelve.

**Moving a set.** Moving every note up *n* semitones is **T*n***: C E G
moved up 3 is E♭ G B♭. Turning every interval upside down is
**inverting**: C E G, up four semitones and then three, becomes C A♭ F,
down four and then three. That is F minor: upside down, a major triad
is a minor triad.

**Set classes.** A set with all its transpositions and inversions is a
**set class**. All 24 major and minor triads are one class. The 4,096
possible sets fall into 224 classes, and each has a **Forte number**,
from the list the theorist Allen Forte made in 1973: **3-11** is the
class of major and minor triads (3 notes, the 11th class of 3 notes).
A class is also written by its **prime form**, its most compact shape
moved to start on 0: **[037]** for 3-11, C E♭ G.

| Set | Forte number | Prime form |
|---|---|---|
| a major or minor triad | 3-11 | [037] |
| a diminished triad | 3-10 | [036] |
| an augmented triad | 3-12 | [048] |
| a dominant seventh or half-diminished chord | 4-27 | [0258] |
| a diminished seventh chord | 4-28 | [0369] |
| the pentatonic scale | 5-35 | [02479] |
| the whole-tone scale | 6-35 | [02468T] |
| the major scale, and its modes | 7-35 | [013568T] |
| the octatonic scale | 8-28 | [0134679T] |

(T stands for 10, so that each number is one character.)

The **interval vector** counts the intervals between every pair of
notes in a set: how many minor seconds, major seconds, minor thirds,
major thirds, fourths and tritones. A fifth counts as a fourth, being a
fourth upside down. For the triad 3-11 it is **<001110>**: one minor
third, one major third, one fourth. It is a fair summary of how a set
sounds.

### In the app

The piano roll names the set of the notes under the pointer, or of the
notes you select, on the line under the chord buttons:

> Set: 3-11 · normal form C E G · prime form [037] · intervals <001110>
> · major or minor triad · micromode Acorde menor-mayor

**Sets…** opens a list of every class of the size you choose, with its
number, prime form, interval vector and, for the famous ones, a name.
Search it by number or name ("4-Z15", "octatonic"), or click **Show
it** for the class of the selected notes. Pick one, then **Move it up**
and **Inverted (I)** choose which of its forms: **T3** is the set three
semitones up, **T3I** its inversion moved up three. **▶ Hear it** plays
it, **Draw with it** puts the whole set down with each click in the
piano roll, and **Use as the key** makes it the song's key. On the
chord sheet, a set is typed by its name or its notes: `3-11 T2`,
`{D F A}` or `{0,1,4}`.

### A set as the key: its forms

When the song's key is a pitch-class set (after **Use as the key**, or
after picking **pitch-class set** in the Key menu), the harmony keys can
play two things, chosen at the left:

- **Chords**: chords stacked from the set's notes, three or four at a
  time.
- **Forms**: the set's own **forms**, the whole set moved or turned
  upside down, each named by its **degree** (T0, T2, T2I…), as for a
  micromode (see [A micromode as the key](#a-micromode-as-the-key)). A
  set of three notes always shows its forms.

The forms come home first, then by how many notes they keep of home,
brighter the more they keep. When a form is something tonal music has a
name for, its key says so. In a key that is C E G B♭ as a set (4-27, the
notes of a dominant seventh), T2I is E G B♭ D, **Em7b5**, the only form
that keeps three of its notes. In C major as a set (7-35), T7 is **G
major**, which keeps six. **Near home** shows the forms that keep at
least half the notes of home, **All forms** shows them all.

![The harmony keys in a key that is the set C E G B♭, with Forms on, after T2I was played: T0 (C7), T2I (Em7b5), then T3 (E♭7), T6 (F♯7), T9 (A7) and the half-diminished sevenths that keep two notes; T0 is outlined because it keeps three notes of E G B♭ D](images/set-forms-keys.png)

**A progression of forms.** In a set key, the chord sheet's
**Progression** button offers ways to chain the set's forms, in place
of the Library's progressions, and writes eight bars of them from the
bar you chose:

- **Keep all notes but one**: each form keeps every note of the one
  before but one, the smoothest way on. C major as a set walks round
  the circle of fifths: F major, B♭ major, E♭ major… For a micromode it
  is called **Keep two notes, move one**.
- **Keep as many notes as it can**, for a set whose forms keep all
  notes but one only in pairs, as the dominant seventh and the
  half-diminished seventh do: C7, Em7b5, A7, C♯m7b5… It never goes
  straight back to the chord before.
- **Keep one note, move two** (micromodes): a pivot, a wider move.
- **All twelve notes, every *N* forms**: forms that share no note, so
  that every few of them hold all twelve notes between them: four forms
  of a micromode, the three diminished seventh chords, or a hexachord
  and the six notes it leaves out. Then the next round starts from a
  form that keeps notes of the last. It is there only for the sets that
  can do it.
- **Sequence**: every note up a semitone, a whole tone, a minor third,
  a fourth or a fifth each bar.

Each bar shows its degree and how many notes it keeps of the bar before
("T2I · 3 common"), and you can type a degree in any bar: `T2`, `T5I`.
The band plays each form as a chord whose shared notes stay on the same
keys, over its root when the form is a chord (C under C7) and otherwise
over the first note of its most compact order.

Switching to another set, with **Use as the key** in the Sets window,
keeps every form's degree, as switching micromodes does (see [A
micromode as the key](#a-micromode-as-the-key)): from C7 as a set to
Cmaj7 as a set, C7 Em7b5 A7 becomes Cmaj7 E♭maj7 Amaj7.

**A micromode style over your forms.** In a song, a style from the
Library's **Micromodes (Kröpfl)** group plays the forms written on the
chord sheet, in your order, the way a style plays the chords of a tonal
song: change a bar and the style plays the new form. It works in any
set key, not only a micromode: **Viennese points** can play C7, Em7b5,
A7… in a key that is C E G B♭ as a set. The style's keys, bass and line
play in place of the band's piano, bass and soloist, and the guitar,
strings and organ rest. When the sheet holds no forms of the key (a
song in a major or minor key, or an empty sheet), or when the Library's
**Chord progression** box is ticked, the style writes its own walk of
forms on the sheet instead, and the key becomes its micromode, starting
on the key note. A style of your own, played over a set, plays only the
set's notes.

## One harmony, four names

A set is another way to **name** notes, not other notes. C major's
chord C is C, E and G however you name it. The chord sheet's **Show**
menu writes the same harmony four ways, without changing a note:
**As written**, as **Chords**, as **Pitch-class sets** or as
**Micromodes**.

![The chord sheet of the demo Classical cadence with Show set to Pitch-class sets: C, F, G7, C, Am, Dm, G7, C written as 3-11 T7I (C E G), 3-11 T0I (F A C), 4-27 T7I (B D F G), 3-11 T7I, 3-11 T9 (A C E), 3-11 T2 (D F A), 4-27 T7I and 3-11 T7I, each with its notes under it](images/sets-view.png)

| As written | Chords | Pitch-class sets | Micromodes |
|---|---|---|---|
| C | C | 3-11 T7I | Acorde menor-mayor T7I |
| Dm | Dm | 3-11 T2 | Acorde menor-mayor T2 |
| G7 | G7 | 4-27 T7I | 4-27 T7I |
| 3-11 T2 | Dm | 3-11 T2 | Acorde menor-mayor T2 |
| Menor 3 | Menor 3 | 3-3 | Menor 3 |

How to read them:

- **T*n*** is the prime form moved up *n* semitones: 3-11's prime form
  is C E♭ G, a C minor triad, so **3-11 T2** is D F A, D minor.
- **T*n*I** is the prime form turned upside down first, then moved up:
  C E♭ G upside down is C A F, and moved up 7 it is G E C. So every
  major triad has an I in its name, and no minor triad has one.
- A micromode is always three notes, so G7, with four, keeps its set
  name in the Micromodes view.
- **Chords** names a set as a chord when its notes make one, and keeps
  the set's name when they don't.

For sets and micromodes, a second menu chooses which notes make each
group: **The sheet's harmonies**, **Notes together (vertical)**, or
**Notes in a line (serial)**, with how many notes each group holds and
which track to read. That reads the song's own notes, so you can look
at a tune as a chain of three-note groups.

The key works the same way. Pick **pitch-class set** in the **Key**
menu of a song in C major, and nothing moves: the key is now named
7-35, and the harmony keys show the same seven chords with their set
classes, six of them 3-11 and B D F 3-10. Notes move only when you
pick a different set.

## Twelve-tone rows

Around 1921, Arnold Schoenberg looked for a way to write music in which
no note is home. His answer was to use all twelve notes in a fixed
order, chosen for each piece: the **row**, or series. No note comes
back until the other eleven have sounded, so none becomes the centre.
This is the row of his Suite for Piano, Op. 25 (1925):

E F G C♯ F♯ D♯ G♯ D B C A A♯

A row comes in four forms:

| Form | How | Schoenberg's row |
|---|---|---|
| **P**, prime | as written | E F G C♯ F♯ D♯ G♯ D B C A A♯ |
| **I**, inversion | every interval upside down | E D♯ C♯ G D F C F♯ A G♯ B A♯ |
| **R**, retrograde | backwards | A♯ A C B D G♯ D♯ F♯ C♯ G F E |
| **RI**, retrograde inversion | the inversion backwards | A♯ B G♯ A F♯ C F D G C♯ D♯ E |

Each can start on any of the twelve notes, which makes 48 forms. They
are named by a letter and the number of the note they start on, with C
as 0: Schoenberg's row is **P4**, because it starts on E. An R form
takes the number of the form it reverses, so **R4** ends on E. (Some
books number the forms from the row's own first note instead, so that
the row as written is always P0. The forms are the same; only their
numbers move.)

### The Row window

**Row…** in the piano roll opens it. Type a row in the box at the top,
as note names (in letters or in solfège) or numbers, and click **Use**; or draw twelve notes in the
piano roll and click **From the notes**; or take one from **🎲 Random**,
**🎲 All-interval** or **Famous and example rows**: Schoenberg's Suite,
Berg's Lyric Suite, Webern's Symphony and Concerto, and four rows
written for Ferrous Lineup.

![The Twelve-tone row window for the row of Webern's Concerto, Op. 24, on C: C B E♭ E A♭ G A F F♯ C♯ D B♭, with its matrix of 48 forms, P0 selected, and under it "Steps up, in semitones: 11 4 1 4 11 2 8 1 7 1 8", which forms are combinatorial with it, and its trichords, all four 3-3 (Menor 3)](images/row-window.png)

The **matrix** holds all 48 forms in a square. Each line, read left to
right, is a P form, and right to left an R form; each column, read
down, is an I form, and up an RI form. The labels around the square say
which: click one to choose that form. The diagonal from the top left is
the same note all the way down, a quick check that the square is right.

Under the matrix, the window says what the row is made of: its steps in
semitones, its groups of three notes, and which forms are
**combinatorial** with it, whose first six notes are the six the row
leaves out, so the two played together give all twelve notes in each
half.

Then write a form into the pattern: **Together** says how many of its
notes sound at once, as a chord; **Then** what comes after the form,
**Once**, **Repeat** or **P, R, I, RI**, or **A plan of forms**: the
forms you type in the **Plan** box, such as P0 RI5 I7 R0, one after
another, as a chord progression is the plan of a tonal piece. Clicking
a label around the matrix adds that form at the end of the plan. Then
the register, **Close** (all in
one octave), **Smooth** (each note near the one before) or **Wide**
(leaping over three octaves, as in Webern). **Serialism** goes further,
as Messiaen, Boulez and Babbitt did around 1950: it lets the row
choose the **Durations**, **Dynamics** and **Octaves** too. When the
row is combinatorial, **With its partner, I3** (or whichever form it is)
writes the form's partner with it, an octave lower and note for note in
the same rhythm, so the first six notes of the two make all twelve, and
so do the last six: Schoenberg's favourite way of keeping all twelve
notes going in two voices at once. A row without a partner leaves the
box grey. **Write P0 into current pattern** (or **Write P0 RI5 I7…**,
with a plan) replaces the channel's notes, and **Save to new pattern**
puts them in a pattern of their own.

### A row as the key

Pick **twelve-tone row** in the **Key** menu, or click **Use as the
key** in the Row window for the form you chose, and the row becomes the
key. The piano roll numbers its
notes 1 to 12 on the keyboard, in the row's order, so you can draw the
row by following the numbers. The harmony keys play the row's groups
instead of chords: **3 notes** gives four keys, 1–3, 4–6, 7–9 and
10–12; **4 notes** three, and **6 notes** two.

Take another row in the Row window and click **Use as the key**, and
the piece is played on the new row: every
note keeps its place in its form, so the fifth note of RI5 becomes the
fifth note of RI5 of the new row. Ferrous Lineup finds which form each
note is part of the way the voice-leading check does, so the plan of
forms, the rhythm and the chords stay, and only the row changes. Some
rows, such as Berg's, give two names to every form, and going back to
the first row may then pick the other name: **Ctrl+Z** is the sure way
back.

With **Voice leading** ticked, the piano roll checks the rules of
twelve-tone writing as Schoenberg's students learnt them: the notes
follow forms of the row, each in its order; notes next to each other in
the row may sound together as a chord, and a note may be repeated
straight away, but it comes back only after the other eleven; a form
is played whole; no note is doubled in octaves; and no chord sounds
like tonal harmony, such as a major or minor triad or a dominant
seventh. A **Row** line under the chord names shows the forms it reads
in the notes.

The Library's **Twelve-tone rows** group has seven styles, and **File > Demos > Twelve-tone** plays each as a short
piece.

### A row style plays your row and your plan

When the key is a row, the Library's row styles play that row, yours,
each in its own way: the Triad chorale as chords over a slow bass, the
Twelve-tone lo-fi over a beat, and so on. In a key that is not a row,
or with **Chord progression** ticked, a style brings its own row,
starting on the key note.

In a song, the chord sheet holds the **plan** of the piece: which form
of the row plays where. Click a bar and type a form, such as **P0**,
**RI5**, **I7** or **R0**, and the band plays that form there, for as
long as it lasts on the sheet, in the style's textures: each of the
style's voices reads its own form the same way, so where the sheet says
I5 a bass that plays the row backwards plays RI5. When the sheet has no
forms of the row yet, the style writes its own plan there, one part
each: the row as written, backwards, upside down, and both. Change any
bar and the band plays the new form.

The style's chords, bass and melody take the place of the piano, the
bass and the soloist; the song's own chords and bass go quiet, and its
melody keeps playing. The Hexachord pads style plays its melody from
the partner of each form (see [The Row window](#the-row-window)), so
every bar holds all twelve notes with any combinatorial row.

## Micromodes

**Francisco Kröpfl** (1931–2021) was an Argentine composer, a pioneer
of electronic music who founded the first university studio for it in
Latin America, in Buenos Aires in 1958, and a teacher of composition
for decades. He taught his students to hear groups of three notes by
their colour. Any three notes have three intervals between them, and
those intervals give the group its sound, in any order, octave or
spelling, moved up or turned upside down. By those rules, which are
the rules of pitch-class sets, there are exactly twelve groups of
three. Kröpfl called them **micromodes** (*micromodos*):

| Micromode | Prime form | Forte |
|---|---|---|
| **Menor 1** | do - do♯ - re | 3-1 |
| **Menor 2** | do - do♯ - re♯ | 3-2 |
| **Menor 3** | do - do♯ - mi | 3-3 |
| **Menor 4** | do - do♯ - fa | 3-4 |
| **Menor 5** | do - do♯ - fa♯ | 3-5 |
| **Mayor 1** | do - re - mi | 3-6 |
| **Mayor 2** | do - re - fa | 3-7 |
| **Mayor 3** | do - re - fa♯ | 3-8 |
| **Mayor 4** | do - re - sol | 3-9 |
| **Acorde disminuido** | do - mi♭ - sol♭ | 3-10 |
| **Acorde menor-mayor** | do - mi♭ - sol | 3-11 |
| **Acorde aumentado** | do - mi - sol♯ | 3-12 |

They come in three families. The **menores** all have a minor second, a
semitone; the **mayores** have a major second and no semitone; and the
rest are the three triads, named after them. The minor and the major
triad are one micromode, **Acorde menor-mayor**, because each is the
other upside down: every chord of a I, IV, V, I is the same colour,
and tonal music works with *which* forms it uses.

**Micromodes…** in the piano roll shows the three families, each
micromode drawn on a clock of the twelve notes, with its prime form,
its intervals, its set and where it is heard:

![The Micromodes window: the three families, Menores (Menor 1 to 5), Mayores (Mayor 1 to 4) and Acordes (Acorde disminuido, Acorde menor-mayor, Acorde aumentado), and Menor 3 drawn on a clock as a triangle C, D♭, E, with its prime form do - do# - mi, its intervals, the set 3-3 [014] with a button to the Sets window, and where it is heard](images/micromodes.png)

Select some notes in the piano roll, and the window lists the
micromodes inside them: the seven notes of C major hold 35 groups of
three. **Move it up** and **Inverted (I)** choose a form; **▶ Hear
it**, **Draw with it** and **Use as the key** work as in the Sets
window, and the set's button opens the Sets window on the same class.
On the chord sheet, type a micromode by name: `Menor 3`, or `Menor 3
T4I` for one of its forms.

The four groups of three in the row of Webern's Concerto, Op. 24, in the
picture of the Row window, are all Menor 3. The Library's **Micromodes
(Kröpfl)** group has seven styles, and **File > Demos > Micromodes**
plays each of them. In a song, the band plays them from the chord sheet
(see [A micromode style over your forms](#a-set-as-the-key-its-forms)).

### A micromode as the key

When the song's key is a micromode (after **Use as the key**, or after
a micromode style from the Library), its chords are the micromode's
**forms**: the same three notes moved, or turned upside down and moved.
They are named by their **degree**, counted from the key the way a
Roman numeral is:

- **T0** is the key's own form, home. In Menor 3 on C it is C C♯ E.
- **T2** is home moved up 2 semitones: D D♯ F♯.
- **T0I** is home turned upside down around the key's note, each note
  as far below C as it was above: C B A♭.
- **T1I** is that moved up a semitone: A C C♯.

Move the whole song to D and every form keeps its degree, as ii stays
ii in any major key. Home is always T0. With the triads, the parallel
chord is T7I: C minor in C major, and C major in C minor.

![The harmony keys in a key of Menor 3 on C after T1I was played: T0, T1I, T4I and T5I, then the forms that keep one note of home, each with its notes and its name; T0, T8 and T9 are outlined because they keep two notes of A C C♯, and the hint below says so](images/micromode-keys.png)

The **harmony keys** show the key's forms, home first, then the forms
that keep two of its notes, then the ones that keep one; the more a
form keeps of home, the brighter its key. **Near home** leaves out the
forms that keep no note of home, and **All forms** shows them all (24
for Menor 3). After you play a form, the forms that keep two of *its*
notes are outlined: they are the smoothest ways on, one note moving.
Hover a key to read what stays and what moves, such as "keeps A and C;
C♯ goes to A♭"; for the major and minor triads it also names the move
(P, L or R).

On the **chord sheet**, each bar shows its degree, and from the second
bar on how many notes it keeps of the bar before: "T1I · 2 common".
Type a degree to write that form: `T2`, `T5I`. The micromode demos
write their forms on the sheet, so you can read the order of each.

**Another colour, the same degrees.** The **Key** menu lists the twelve
micromodes under the scales, in three short rows: **Menor** 1 to 5,
**Mayor** 1 to 4, and the triads as **°**, **m/M** and **+** (point at
one for its whole name). It opens at the key's own micromode. Pick
another, and the piece keeps its plan in the new colour, as a song in
C major keeps its chords' degrees when it goes to C minor: every form
on the chord sheet keeps its degree (T2 of Menor 3 becomes T2 of
Menor 5) and every written note moves to the same place in its new
form. A micromode style the band plays follows the new forms. The
smooth links change with the colour: the Menor 3 demo keeps two notes
from bar to bar, and in Menor 5 the same degrees keep none, which the
sheet shows ("T4I · 0 common").

A few micromodes are the same upside down (Menor 1, Mayor 1, and the
diminished and augmented triads), so in them a form upside down is
just a form moved; going into one of them and back cannot tell which
forms were upside down. From a major or minor key to a micromode, the
notes stay as they are, as with **pitch-class set**.
