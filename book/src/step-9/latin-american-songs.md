# Latin American songs, from Mexico to the Río de la Plata

Files: [`crates/lineup-core/src/library/song/tango.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/song/tango.rs),
[`crates/lineup-core/src/library/song/latin.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/song/latin.rs),
[`crates/lineup-core/src/library/song/folklore.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/song/folklore.rs),
[`crates/lineup-core/src/library/chart.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/chart.rs)
and [`crates/lineup-core/src/library/comping.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/comping.rs)

The demos had a handful of tangos, a few Andean songs and some Cuban
and Brazilian ones. This chapter adds **44 Latin American songs**,
makes **six older ones play their whole score**, and adds five dance
rhythms the library did not have. Every new song was
written down from a score, never from memory, and every chord was
checked by listening to the rendered band.

## Trying it

Open **File > Demos** and type "cabeza" in the search box: **Por una
cabeza**, Gardel's tango of 1935. Then try these, one per kind of
music:

- **Carioca** (Brazil): a maxixe, the "Brazilian tango" of 1913, in E minor.
- **Sobre las olas** (Mexico): all four of its waltzes.
- **Papel de plata** (the Andes): a huayno whose refrain turns to 3/4.
- **La gata golosa** (Colombia): a pasillo, a waltz with its second beat late.
- **Aquellos ojos verdes** (Cuba): a bolero.
- **Luna de Xelajú** (Guatemala): a waltz on the marimba.

The folklore genre in **File > Demos** is **Latin American folklore**,
since it reaches from Mexico to Argentina.

The new songs:

| Where | Songs |
|---|---|
| Tango | Lágrimas y sonrisas, A media luz, Adiós muchachos, Caminito, Boedo, Alma en pena, El amanecer, A la gran muñeca, Por una cabeza, El día que me quieras, Volver |
| Andes (huayno) | Papel de plata, La Quebrada de Humahuaca, Huayta huaytucha, Kicharirillaway, A las orillas del Titicaca, Chunguinada |
| Mexico | Jesusita en Chihuahua, De colores, Sobre las olas, La golondrina, Estrellita, Jarabe tapatío, La raspa, La bamba, La cucaracha (complete), La Llorona, María Elena, Te quiero dijiste, Los machetes |
| Cuba | Quiéreme mucho, Siboney, Aquellos ojos verdes |
| Colombia | Diana triste, La gata golosa |
| Guatemala | Luna de Xelajú |
| Brazil | Apanhei-te, cavaquinho, Carioca, Atraente, Sons de carrilhões, Ó abre alas, Pezinho, Balaio, Luar do sertão |

Played in full now: **El Choclo**, **La Cumparsita**, **La Paloma**,
**Gallo ciego**, **El cóndor pasa** and **Alma llanera**.

## The music

### Strains, repeats and "D.C."

A tango, a choro or a march of 1900 is not verse and chorus: it is two
or three **strains** of 16 bars (in 2/4), each played twice, often with
the last one in another key. The score saves paper with repeat signs,
first and second endings, and **D.C.** (*da capo*, "from the head":
go back to the start) or **D.S.** (*dal segno*, back to a sign), up to
**Fine** ("end") or a jump to a **coda** ("tail").

A demo plays all of it written out, in the order the score says. In
the playlist you can see the form: **El Choclo** is Pickup, A, B, B,
C, then the D.C. takes it back to A and B once more (repeats are not
taken a second time after a D.C.), and C with its second ending
closes the piece. Before, the demo stopped after C and never came back.

### Key changes you can see

Each key change of these scores is a key marker on the ruler (see
[Key changes](../step-8/key-changes.md)):

- **El Choclo** turns from A minor to **A major** for its third strain
  (the score's key signature changes from none to three sharps), goes
  back to A minor with the D.C., and ends in A major.
- **Gallo ciego** visits the **relative major**, F, in its second
  strain (same key signature as D minor, but the cadences are C7 to F),
  and its third strain is in **B flat**, a real change of signature.
- **Volver** has its verse in G minor and its refrain in **G major**:
  the same tonic, the other mode. Listen for B flat turning into B.
- **Carioca** is in E minor, with a second strain in G (the relative
  major) and a trio in **C**, a third away: Nazareth wrote it like a
  little piano rondo.
- **Alma llanera** has its verse in D minor and its chorus in **D
  major**, the joropo's famous lift.

### Five new dance rhythms

The library's Rhythms and Styles menus have five new entries. Four of
them are in 2/4, written two of their bars to a bar of the app:

- **Polca**: the polka as Mexico's corridos and norteño bands and Rio's
  choro players took it up: the bass on beat 1, the guitars on beat 2
  and its "and".
- **Maxixe**: the dance Rio's piano composers called "Brazilian
  tango": the habanera's long, short, long, long in the left hand, and
  a syncopation (short, long, short) in the tune.
- **Marchinha**: the carnival march, sung in the street since Ó abre
  alas of 1899: the bass drum on both beats, the second stronger.
- **Son jalisciense** (6/8): the son of Jalisco and the mariachi,
  switching between two groups of three eighths and three groups of
  two, like the Spanish *sesquiáltera*.
- **Pasillo** (3/4): the quick waltz of Colombia and Ecuador, with the
  second beat played late, on its "and".

### Appoggiaturas

Play **El día que me quieras** and watch the Lead channel against the
chord name: many phrases start on a note that is **not** in the chord
(F over C7, E flat over B flat minor) and only then move to one that
is. That is an **appoggiatura**, a "leaning" note: the tension on the
strong beat and its resolution a step away are what make the melody
sound like longing. A tune does not have to sit on chord tones; it has
to explain the notes that are not.

## The code

### Every chord is struck

`chart::accompany` turns a chord chart into the keys' and the bass's
notes, and starts each chord's pattern again at the chord. That is how
a chart whose chords change on the bar line should sound. But take the
refrain of **Papel de plata**, in 3/4:

```text
i bVII7 bIII | bIII bVII7 bIII | ...
```

One chord per beat. The waltz pattern of the keys strikes on beats 2
and 3, so restarted at each chord, its first hit falls a beat after
the chord starts, which is already the next chord: the one-beat D7
(`bVII7`) was never played at all.

Song demos now use `chart::accompany_in_bars`, which keeps the keys'
pattern **in step with the bars**, the way a band keeps its groove
whatever the harmony does. Each hit plays the chord sounding at that
moment, and a chord too short for any hit is struck once where it
starts:

```rust
pub fn accompany_in_bars(
    spans: &[ChordSpan],
    tonic: PitchClass,
    comping: Comping,
    bass: Bass,
    time: TimeSignature,
) -> Backing {
    let mut backing = accompany(spans, tonic, comping, bass);
    // ... build one `Played` per chord and the song's bars ...
    backing.chords = comping.play_in_bars(&bars, &harmonies);
    backing
}
```

It starts from `accompany`'s result and replaces only `chords`, so the
bass line stays exactly as it was. Keeping the bass in step with the
bars too was tried, but it changed the walking bass of the jazz demos
enough that the key finder (see
[Finding the key](../step-8/finding-the-key.md)) heard more wrong
keys, so the bass still starts its line at every chord.

`Comping::play_in_bars` finds the chord under each hit with
`partition_point`, a binary search on a sorted slice: it returns how
many harmonies start at or before the hit, so the one before that
count is the chord sounding.

### A part played its own way

A song had one comping, one bass line and one rhythm. A part can now
override them, for a section in another meter:

```rust
pub struct Part {
    // ...
    pub played: Option<(Comping, Bass, &'static str)>,
}

impl Song {
    pub fn part_playing(&self, part: &Part) -> (Comping, Bass, &'static str) {
        part.played.unwrap_or((self.comping, self.bass, self.rhythm))
    }
}
```

`Option::unwrap_or` returns the part's own triple when there is one,
the song's otherwise. Papel de plata's refrain says
`.in_time(THREE_FOUR).played(Comping::Waltz, Bass::Waltz, "Vidala")`:
the huayno's 2/4 patterns would not fit a bar of three beats.

### A test that met the full forms

The key finder has a test, `clear_modulations_are_found_on_their_bar`,
that checked it found every key change of a few songs exactly. El
Choclo and Gallo ciego were two of them. Played in full, each now goes
back to its first key after a **single eight-bar strain** in another,
and the finder hears so short a strain as passing through: it adds a
cost, `CHANGE_COST`, for every change of key, and two changes cost
more than eight bars of a slightly better fit gain. Lowering the cost
finds them, but on the whole library it found more changes that are
not there (89.5% of bars right instead of 91.3%). So those two songs
moved to a weaker test that still holds: every change the finder hears
must be one the score makes. Alma llanera, whose D major chorus is
long, joined the exact test.

### Copyright

A song is included if it is traditional, or was published in 1930 or
earlier, or all its authors died before 1956. A comment above each says
which. Some famous songs are missing because no score of them could be
read from here (Desde el alma, Mi Buenos Aires querido, La morocha,
O samba e o tango); attach a score and they can be added.

## Exercises

1. Open **Volver** and find the key marker where the refrain starts.
   Which note of the G minor scale changes in G major? Find it in the
   tune of the refrain's first bars.
2. In **El Choclo**, mute every channel but Bass and play the third
   strain. Find the circle of fifths F#7, B minor, B7, E7: each root
   is a fifth above the next. Where does it land?
3. In `folklore.rs`, change Papel de plata's refrain to
   `.played(Comping::Sway, Bass::Waltz, "Vidala")`, run the app and
   play the refrain. Sway strikes once, on beat 2: which chords of
   `i bVII7 bIII` do you hear struck, and which only start?
4. Play **Los machetes** (Polca) and then **Ó abre alas** (Marchinha),
   both in 2/4. Which beat does the drum stress in each?
