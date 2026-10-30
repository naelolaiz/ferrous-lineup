# Acordes: construirlos, nombrarlos e invertirlos

Archivos: [`crates/lineup-core/src/harmony/chord.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/harmony/chord.rs),
[`crates/lineup-core/src/harmony/symbol.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/harmony/symbol.rs),
[`crates/lineup-core/src/harmony/analysis.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/harmony/analysis.rs)
y [`crates/lineup-core/src/harmony/scale.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/harmony/scale.rs)

Los acordes están por todas partes en Ferrous Lineup: el piano roll los
coloca con un clic y nombra lo que esté sonando, la hoja de acordes los
lee a medida que los escribes, las teclas de armonía los tocan y la
banda los convierte en partes. Este capítulo trata de los tipos que hay
detrás de todo eso: la clase de un acorde, un acorde, un símbolo de
acorde como lo escribe una partitura guía, y el análisis que nombra un
grupo de notas.

## La música, en pocas palabras

Un **acorde** son varias notas tocadas juntas. La mayoría de los
acordes se construyen con **terceras**: una tercera mayor son cuatro
semitonos, una tercera menor tres.

| Acorde | Desde la fundamental | Sobre Do | Suena |
|--------|----------------------|----------|-------|
| mayor | tercera mayor + tercera menor (0 4 7) | Do Mi Sol | luminoso, estable |
| menor | tercera menor + tercera mayor (0 3 7) | Do Mi♭ Sol | más oscuro |
| disminuido | dos terceras menores (0 3 6) | Do Mi♭ Sol♭ | tenso |
| aumentado | dos terceras mayores (0 4 8) | Do Mi Sol♯ | inestable, flotante |
| séptima de dominante | tríada mayor + tercera menor (0 4 7 10) | Do Mi Sol Si♭ | quiere seguir |
| séptima mayor | tríada mayor + tercera mayor (0 4 7 11) | Do Mi Sol Si | suave, jazzero |
| menor séptima | tríada menor + tercera menor (0 3 7 10) | Do Mi♭ Sol Si♭ | cálido |

Seguir subiendo por terceras más allá de la octava da los acordes
**extendidos** del jazz, el soul y el funk. Sus nombres cuentan las
notas de la escala desde la fundamental: la novena es la segunda una
octava más arriba (14 = 12 + 2 semitonos), la oncena la cuarta (17 = 12
+ 5), la decimotercera la sexta (21 = 12 + 9). C13 es Do Mi Sol Si♭ Re
La.

La nota más grave es la **fundamental** cuando el acorde está en
**estado fundamental**. Tocado como Mi Sol Do, con Mi abajo, sigue
siendo Do mayor, en su primera **inversión**. Un **símbolo de acorde**,
como lo escribe una partitura guía, pone el bajo después de una barra:
C/E. El capítulo [Los acordes y sus
nombres](https://naelolaiz.github.io/ferrous-lineup/es/manual/chords.html)
del manual lo tiene todo, con qué escuchar.

## Clases de acorde: una tabla en un `match`

Un acorde sin su fundamental es una **clase** de acorde: «menor
séptima», empiece en la nota que empiece. Hay treinta, una variante del
enum cada una:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChordQuality {
    Major,
    Minor,
    Diminished,
    Augmented,
    Sus2,
    Sus4,
    Major6,
    Minor6,
    Dominant7,
    Major7,
    Minor7,
    // ...
    Dominant13,
    Major13,
}
```

Todo lo que Ferrous Lineup sabe de una clase es un `match` sobre ella,
un método por dato. Las notas, en semitonos por encima de la
fundamental:

```rust
pub fn intervals(self) -> &'static [u8] {
    use ChordQuality::*;
    match self {
        Major => &[0, 4, 7],
        Minor => &[0, 3, 7],
        Diminished => &[0, 3, 6],
        // ...
        Dominant9 => &[0, 4, 7, 10, 14],
        // ...
        // Players leave the eleventh out of thirteenth chords, because
        // it clashes with the third; so does Ferrous Lineup.
        Dominant13 => &[0, 4, 7, 10, 14, 21],
        Major13 => &[0, 4, 7, 11, 14, 21],
    }
}
```

La novena se escribe 14, no 2. Como clase de altura es lo mismo, pero
un acorde colocado en el piano roll se construye con estos números, y
una novena un tono por encima de la fundamental sonaría a racimo, no a
acorde de novena.

Los otros datos son métodos de la misma forma: `symbol` (el «m7» de
«Am7»), `name` («minor seventh»), `tones` (qué nota del acorde es cada
intervalo, más abajo), `is_minor`, `is_dominant` y `numeral_suffix` (el
«°» de «vii°»). Escribir el conocimiento como ramas de `match` tiene una
gran ventaja frente a, por ejemplo, una lista de structs: añade una
variante y el compilador nombra cada método que todavía no la trata.

`numeral_suffix` termina con un **patrón que liga**:

```rust
match self {
    Major | Minor => "",
    Diminished => "°",
    // ...
    other => other.symbol(),
}
```

`other` encaja con todo lo que queda, como `_`, pero le da un nombre
para usarlo. `family`, que clasifica una clase en los tres tipos de
tríada, añade una guarda a uno:

```rust
pub fn family(self) -> ChordFamily {
    use ChordQuality::*;
    match self {
        Diminished | HalfDiminished7 | Diminished7 => ChordFamily::Diminished,
        quality if quality.is_minor() => ChordFamily::Minor,
        _ => ChordFamily::Major,
    }
}
```

Las tonalidades usan la familia para emparejar un acorde con sus
propios acordes: G7 es el acorde mayor sobre Sol, Bm7b5 el disminuido
sobre Si ([Números romanos, las teclas de armonía y las funciones de
los acordes](numerals.md)).

### ¿Qué clase es esta?

Las notas de una clase sobre Do son un
[`PitchSet`](pitch.md#un-conjunto-de-clases-de-altura-en-doce-bits),
y encontrar la clase de un grupo de notas visto desde su fundamental es
una búsqueda:

```rust
pub fn identify(relative: PitchSet) -> Option<ChordQuality> {
    Self::ALL
        .into_iter()
        .find(|quality| quality.pitch_set() == relative)
}
```

`find` devuelve la primera clase que encaja. Así que el orden de
`ChordQuality::ALL` importa, y pone primero los acordes más sencillos y
comunes. Para que `identify` encuentre todas las clases, no puede haber
dos con las mismas notas, y un test se asegura de ello:

```rust
#[test]
fn no_two_qualities_have_the_same_notes() {
    // Otherwise `identify` could never find the second one.
    for (i, a) in ChordQuality::ALL.iter().enumerate() {
        for b in &ChordQuality::ALL[i + 1..] {
            assert_ne!(a.pitch_set(), b.pitch_set(), "{a:?} and {b:?}");
        }
    }
}
```

`&ChordQuality::ALL[i + 1..]` es el slice del array desde `i + 1` hasta
el final, así que cada par se compara una vez.

## Un acorde: una fundamental y una clase

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Chord {
    pub root: PitchClass,
    pub quality: ChordQuality,
}
```

Sus clases de altura son las de la clase subidas hasta la fundamental, y
sus notas se apilan desde una nota fundamental dada, dejando fuera las
que se saldrían por arriba de MIDI:

```rust
pub fn pitch_set(self) -> PitchSet {
    self.quality.pitch_set().transpose(self.root.semitone())
}

pub fn notes_from(self, root_note: Note) -> Vec<Note> {
    self.quality
        .intervals()
        .iter()
        .filter_map(|&interval| root_note.transpose(interval as i32))
        .collect()
}
```

### Los acordes de una tonalidad: apilar terceras

Para construir el acorde sobre una nota de una tonalidad, se toma una
nota de la escala sí y otra no: la nota, la que está dos lugares más
arriba, la que está cuatro (y seis para un acorde de séptima). Eso es
**apilar terceras**. En Do mayor, empezar en Re da Re, Fa y La: Re
menor.

```rust
pub fn stack_thirds(self, note: Note, count: usize) -> Option<Vec<Note>> {
    if !self.is_heptatonic() {
        return None;
    }
    let intervals = self.kind.tonal_intervals();
    let degree = self.degree_of(note.pitch_class())?;
    let notes = (0..count)
        .map(|i| {
            let index = degree + 2 * i;
            // Going past the seventh note wraps into the next octave.
            let octave = (index / 7) as i32;
            let above_root = intervals[index % 7] as i32 + 12 * octave;
            above_root - intervals[degree] as i32
        })
        .filter_map(|semitones| note.transpose(semitones))
        .collect();
    Some(notes)
}
```

Se lee como una cadena: los números `0..count` se vuelven lugares de la
escala, los lugares se vuelven semitonos por encima de `note`, y
`filter_map` convierte los semitonos en notas, descartando las que
saldrían del rango MIDI. El `?` después de `degree_of` devuelve `None`
enseguida cuando la nota no está en la escala.

`Scale::triad(degree)` y `seventh(degree)` apilan tres o cuatro notas
sobre un grado y nombran el resultado con `identify`. Para los siete
grados de Do mayor, es la tabla que aprende todo compositor de
canciones:

| Grado | 1 | 2 | 3 | 4 | 5 | 6 | 7 |
|-------|---|---|---|---|---|---|---|
| Tríada | C | Dm | Em | F | G | Am | Bdim |
| Número | I | ii | iii | IV | V | vi | vii° |
| Séptima | Cmaj7 | Dm7 | Em7 | Fmaj7 | G7 | Am7 | Bm7b5 |

Cambia la tonalidad y la tabla la sigue, porque se calcula, no está
escrita. `ninth(degree)` apila cinco notas, y devuelve `None` en los
grados donde la novena queda solo un semitono por encima de la
fundamental una octava más arriba (Mi Sol Si Re Fa en Do mayor), que
choca, y ningún acorde con nombre encaja.

### Sellos de acorde

Lo que coloca un clic en el piano roll es un **sello**, un enum cuyas
variantes llevan lo que necesitan:

```rust
pub fn notes_at(self, note: Note, scale: Scale) -> Vec<Note> {
    match self {
        ChordStamp::Single => vec![note],
        ChordStamp::ScaleTriad => scale.stack_thirds(note, 3).unwrap_or_else(|| vec![note]),
        ChordStamp::ScaleSeventh => scale.stack_thirds(note, 4).unwrap_or_else(|| vec![note]),
        ChordStamp::Fixed(quality) => Chord::new(note.pitch_class(), quality).notes_from(note),
        ChordStamp::Set(set) => set
            .semitones()
            .into_iter()
            .filter_map(|up| note.transpose(i32::from(up)))
            .collect(),
    }
}
```

Los sellos «Acorde de la tonalidad» son donde el piano roll enseña
armonía: haz clic en cualquier fila iluminada y obtienes el acorde que
la tonalidad le da a esa nota, sin saber antes su nombre. El nombre
aparece luego en la franja de acordes sobre la cuadrícula. Un sello
`Set` coloca cualquier grupo de notas elegido en el explorador de
conjuntos ([Conjuntos de clases de altura, y una armonía con cuatro
nombres](sets.md)).

El menú **Dibujar** lista todos los sellos. `all()` encadena dos
iteradores: los tres sellos especiales, y luego un sello `Fixed` por
clase:

```rust
pub fn all() -> impl Iterator<Item = ChordStamp> {
    [
        ChordStamp::Single,
        ChordStamp::ScaleTriad,
        ChordStamp::ScaleSeventh,
    ]
    .into_iter()
    .chain(ChordQuality::ALL.into_iter().map(ChordStamp::Fixed))
}
```

`map(ChordStamp::Fixed)` pasa la propia variante como función: una
variante de tupla como `Fixed` *es* una función de `ChordQuality` a
`ChordStamp`.

Junto al menú Dibujar, un segundo menú elige la **inversión** de un
acorde sellado. Invertir sube la nota más grave una octava
(`tools::inversion`, en [Herramientas de edición como funciones
simples](tools.md)), lo que alejaría el acorde de donde
hiciste clic, así que el piano roll lo baja una octava y la nota en la
que hiciste clic sigue siendo la fundamental:

```rust
fn stamp_notes(&self, pitch: Note, scale: Scale) -> Vec<Note> {
    let notes = tools::inversion(&self.stamp.notes_at(pitch, scale), self.inversion);
    if notes.contains(&pitch) {
        return notes;
    }
    notes
        .iter()
        .map(|note| note.transpose(-12))
        .collect::<Option<Vec<Note>>>()
        .unwrap_or(notes)
}
```

Recoger un iterador de `Option`s en un `Option<Vec<_>>` da `None` en
cuanto un elemento es `None`. Aquí el tipo se escribe con un
**turbofish**, `collect::<...>()`, porque `unwrap_or` se llama sobre el
resultado enseguida, y Rust tiene que saber el tipo del resultado para
encontrar ese método. Si una nota se saliera por abajo de MIDI, el
acorde se queda como estaba. El contorno que sigue al puntero se dibuja
con la misma función, así que lo que ves es lo que coloca un clic.

## Escribir un acorde

C♯7 y D♭7 se tocan en las mismas teclas, pero se escriben distinto. Un
acorde se escribe hacia arriba desde su fundamental por letras: la
tercera está dos letras más arriba, la quinta cuatro, la séptima seis.

| Acorde | Fundamental | Tercera | Quinta | Séptima |
|--------|-------------|---------|--------|---------|
| C♯7 | Do♯ | Mi♯ | Sol♯ | Si |
| D♭7 | Re♭ | Fa | La♭ | Do♭ |

La séptima de D♭7 es Do♭, no Si: seis letras por encima de Re está Do.
Así que cada clase dice también qué nota del acorde es cada intervalo,
contada como cuentan los músicos:

```rust
pub fn tones(self) -> &'static [u8] {
    use ChordQuality::*;
    match self {
        Major | Minor | Diminished | Augmented => &[1, 3, 5],
        Sus2 => &[1, 2, 5],
        Sus4 => &[1, 4, 5],
        // ...
        Dominant13 | Major13 => &[1, 3, 5, 7, 9, 13],
    }
}
```

y `spell` recorre los intervalos y las notas juntos con `zip`, pidiendo
a [`NoteName::above`](pitch.md#los-intervalos-también-cuentan-letras)
cada nota: tantos semitonos, `tone - 1` letras más arriba:

```rust
pub fn spell(self, root: NoteName) -> Vec<NoteName> {
    self.quality
        .intervals()
        .iter()
        .zip(self.quality.tones())
        .map(|(&interval, &tone)| root.above(interval % 12, tone - 1))
        .collect()
}
```

Eso da La mayor como La Do♯ Mi, la séptima disminuida de Do como Do Mi♭
Sol♭ Si𝄫 (su séptima es una séptima, aunque suene como una sexta), y
E7♯9 como Mi Sol♯ Si Re Fa𝄪: la novena aumentada es una novena, una letra
por encima de la fundamental, así que es una clase de Fa, no Sol.

### Un nombre para la fundamental

Escribir necesita el nombre de la fundamental para empezar. En una
tonalidad, la tonalidad lo da: `Scale::spell_root` escribe una
fundamental de la escala como la escribe la escala, y una de fuera de la
escala desde la tónica, como hace
[`Scale::spell`](scales.md#notas-de-fuera-de-la-tonalidad)
(Mi♭ mayor en Do mayor, el acorde prestado de Do menor). Una regla más
mantiene legibles los nombres de acorde: si esa fundamental escribiría
el acorde con dos sostenidos o bemoles en una nota, se usa el otro
nombre de la fundamental. En Re♭ mayor, el acorde un semitono por
encima de la tónica se escribe Re mayor, no Mi𝄫 mayor.

```rust
let doubled = |root: NoteName| chord.spell(root).iter().any(|n| n.accidental.abs() >= 2);
if !doubled(strict) {
    return strict;
}
let other = chord.root_name();
if doubled(other) {
    strict.simplified()
} else {
    other
}
```

Sin tonalidad, como con una serie dodecafónica, `Chord::root_name` elige
el nombre sostenido o el bemol de la fundamental, el que escriba el
acorde entero con menos signos. Do♯ mayor es Do♯ Mi♯ Sol♯, tres
sostenidos; Re♭ mayor es Re♭ Fa La♭, dos bemoles; así que es Re♭.

```rust
let usual = NoteName::chromatic(self.root);
[Spelling::Sharps, Spelling::Flats]
    .into_iter()
    .map(|spelling| NoteName::plain(self.root, spelling))
    .fold(usual, |best, other| {
        if accidentals(other) < accidentals(best) {
            other
        } else {
            best
        }
    })
```

`fold` lleva un valor a lo largo del iterador: empieza con `usual` y,
para cada elemento, el closure devuelve el valor que sigue. Aquí se
queda con el nombre que tenga menos signos, y en caso de empate con el
que ya tiene, el nombre habitual de la tecla negra.

`Scale::chord_name(chord)` y `Scale::chord_notes(chord)` lo juntan todo:
el acorde sobre la tercera nota de La mayor es «C♯m», y en Re menor las
notas de La mayor son La Do♯ Mi.

## Símbolos de acorde: leer una partitura guía

Una canción que quieres que toque la banda suele llegar como **partitura
guía** (*lead sheet*), y una partitura guía nombra sus acordes: «C | G/B
| Am | Am/G». La hoja de acordes lee exactamente eso ([La hoja de
acordes: modelo, distribución y texto](../band/chord-sheet.md)), a
través de `ChordSymbol`:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChordSymbol {
    /// The root, spelled as written: Db7 keeps its Db, it is not C#7.
    pub root: NoteName,
    pub quality: ChordQuality,
    /// The note under the chord, after the slash: the B of "G/B". `None`
    /// when the root is in the bass, as usual.
    pub bass: Option<NoteName>,
}
```

Un símbolo guarda su fundamental como está escrita, un `NoteName`, así
que una hoja en Re♭ se queda en Re♭. Para tocarlo, `chord()` lo convierte
en un `Chord`, una clase de altura y una clase de acorde.

### Muchas maneras de escribir un acorde

Los músicos escriben la misma clase de varias maneras: un menor séptima
es Cm7, Cmin7 o C-7; un séptima mayor Cmaj7, CM7 o CΔ7; un
semidisminuido Cm7b5 o Cø. Se leen todas, desde una tabla de tuplas:

```rust
const SPELLINGS: [(ChordQuality, &[&str]); 30] = {
    use ChordQuality::*;
    [
        (Major, &["", "maj", "M"]),
        (Minor, &["m", "min", "mi", "-"]),
        (Diminished, &["dim", "°", "o"]),
        // ...
        (Major13, &["maj13", "M13", "Δ13"]),
    ]
};
```

Cada entrada empareja una clase con un slice de cadenas, `&[&str]`. Los
slices pueden tener longitudes distintas, así que una clase puede tener
una escritura y otra seis en el mismo array. El bloque `{ use
ChordQuality::*; [...] }` es una expresión cuyo valor es su última
línea, así que el `use` solo alcanza a la tabla. Leer una clase limpia
el texto y busca:

```rust
fn read_quality(text: &str) -> Option<ChordQuality> {
    let plain: String = text
        .chars()
        .filter(|c| !matches!(c, '(' | ')' | ' '))
        .map(|c| match c {
            '♭' => 'b',
            '♯' => '#',
            other => other,
        })
        .collect();
    SPELLINGS
        .iter()
        .find(|(_, spellings)| spellings.contains(&plain.as_str()))
        .map(|(quality, _)| *quality)
}
```

Los paréntesis y los espacios se quitan, así que «C7(b9)» se lee como
«C7b9», y ♭ y ♯ funcionan como `b` y `#`. Las mayúsculas y minúsculas
importan en un lugar, uno importante: CM7 es un séptima mayor (Do Mi Sol
Si) y Cm7 un menor séptima (Do Mi♭ Sol Si♭). Cuando Ferrous Lineup
escribe un acorde de vuelta, usa una manera para cada uno, la que da
`symbol`: Cmaj7, Cm7, Cm7b5, Cdim7. Un test escribe cada clase sobre
cada fundamental, con y sin bajo, y la vuelve a leer, así que la tabla
y `symbol` no pueden separarse.

### Acordes con barra, y el acorde de sexta y novena

Una barra pone otra nota en el bajo. Hay dos tipos:

- **Inversiones**: la nota del bajo está en el acorde. C/E es Do mayor
  con su tercera en el bajo, C/G con su quinta, G7/F con la séptima de
  G7. `ChordSymbol::bass_in_chord` dice cuál: 3, 5 o 7.
- **Un acorde sobre una nota del bajo**: el bajo no está en el acorde.
  F/G es Fa mayor sobre un Sol, un sonido abierto que los músicos de pop
  y gospel usan a menudo en lugar de G11. `bass_in_chord` dice `None`.

Una barra no es una nota del bajo: la de C6/9, el acorde de sexta y
novena. Así que solo lo que sigue a la **última** barra, y solo si es un
nombre de nota, es un bajo:

```rust
fn split_bass(rest: &str) -> (&str, Option<NoteName>) {
    match rest.rsplit_once('/') {
        Some((before, after)) => match NoteName::parse(after.trim()) {
            Some(bass) => (before, Some(bass)),
            None => (rest, None),
        },
        None => (rest, None),
    }
}
```

`rsplit_once('/')` parte el texto por su última barra en las partes de
antes y de después, o da `None` cuando no hay ninguna. «C6/9» es un
acorde de sexta y novena, y «C6/9/E» un acorde de sexta y novena sobre
Mi.

### Leer un símbolo entero

`ChordSymbol::parse` lee la fundamental con
[`NoteName::read_either`](../program/languages.md#leer-un-nombre-de-las-dos-maneras),
en letras o en solfeo, quedándose con la lectura cuyo resto es un
acorde, así que «Faug» es Fa aumentado y «Lam7» es La menor séptima:

```rust
let read = NoteName::read_either(text, |rest| read_rest(rest).is_some());
let Some((root, rest)) = read else {
    return Err(match NoteName::read_either(text, |_| true) {
        None => trf(
            "\"{}\" does not start with a note: chords start with a letter from A \
             to G, like C, F#m or Bb7, or with a syllable, like Do, Fa#m or Sib7.",
            &[&text],
        ),
        Some((_, rest)) => trf(
            "\"{}\": \"{}\" is not a kind of chord Ferrous Lineup knows, \
             such as m, 7, maj7, m7b5, sus4 or 13.",
            &[&text, &split_bass(rest).0],
        ),
    });
};
```

El error dice qué salió mal, para que la hoja de acordes pueda mostrarlo
junto al compás donde se escribió: o el texto no empieza con una nota, o
lo que sigue a la nota no es ningún acorde que Ferrous Lineup conozca.

`ChordSymbol` implementa además el trait estándar `FromStr`, «se puede
leer de un texto», llamando a `parse`. Con él, `str::parse` funciona para
acordes igual que para números:

```rust
impl FromStr for ChordSymbol {
    type Err = String;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        ChordSymbol::parse(text)
    }
}
```

así que `let chord: ChordSymbol = "Am7".parse()?;` lee un acorde.

### Llevar una hoja de acordes a otra tonalidad

Para llevar una canción a otra tonalidad, un músico mueve cada acorde la
misma distancia, contada en letras además de en semitonos. De Mi menor a
Re menor cada letra baja una:

| Mi menor | F♯m7b5 | B7 | Em |
|----------|--------|----|----|
| Re menor | Em7b5 | A7 | Dm |
| Do menor | Dm7b5 | G7 | Cm |
| Si♭ menor | Cm7b5 | F7 | B♭m |
| Sol♯ menor | A♯m7b5 | D♯7 | G♯m |

Es un **ii–V–i**, la forma más común de llegar a una tonalidad menor: en
cada tonalidad el ii está una letra por encima de la tónica y el V
cuatro letras por encima. Contar por letras lo mantiene así: en Sol♯
menor el V es D♯7, nunca E♭7. `transposed(from, to)` lo hace, moviendo la
fundamental y el bajo con `NoteName::above` y tomando el nombre más
sencillo para una fundamental que necesitaría dos signos, porque los
símbolos de acorde nunca los llevan ([Clases de altura, notas y su
escritura](pitch.md#los-intervalos-también-cuentan-letras)).
Un test comprueba el ii–V–i en las doce tonalidades, escritas con
sostenidos y con bemoles.

## Nombrar lo que suena

El piano roll nombra lo que suena en cada paso: nada, una nota, dos
notas, un acorde que conoce, o un grupo sin nombre de acorde. Cada
respuesta necesita datos distintos, que es trabajo para un enum cuyas
variantes llevan datos:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Harmony {
    /// No notes at all.
    Silence,
    /// One pitch class, maybe in several octaves.
    Single(PitchClass),
    /// Two pitch classes. `low` is the one in the bass, and `semitones`
    /// counts up from it to the other.
    Interval {
        low: PitchClass,
        high: PitchClass,
        semitones: u8,
    },
    /// A chord we know. When `bass` is not the chord's root, the chord is
    /// an inversion, written with a slash: C/E is C major with E in the bass.
    Chord { chord: Chord, bass: PitchClass },
    // ...
    /// Three or more pitch classes that are not a chord we know, named
    /// by their set class, such as "4-23" for a stack of fourths.
    Unknown(PitchSet),
}
```

Una variante más, `Polychord`, son dos acordes a la vez, como el Mi♭7
sobre Fa♭ mayor que abre los Augurios primaverales de La consagración de
la primavera. Solo la escucha de acordes los encuentra, por cómo se
agrupan las notas ([Oír la armonía: acordes y tonalidades a partir de
las notas](hearing.md)); las notas solas no pueden decir
cuáles pertenecen a cada acorde.

`analyze` recibe cualquier conjunto de notas y devuelve un `Harmony`:

```rust
pub fn analyze(notes: impl IntoIterator<Item = Note>) -> Harmony {
    let mut pitch_set = PitchSet::EMPTY;
    let mut lowest: Option<Note> = None;
    for note in notes {
        pitch_set.insert(note.pitch_class());
        lowest = Some(match lowest {
            Some(low) => low.min(note),
            None => note,
        });
    }

    let Some(lowest) = lowest else {
        return Harmony::Silence;
    };
    let bass = lowest.pitch_class();

    match pitch_set.len() {
        1 => Harmony::Single(bass),
        2 => {
            let high = pitch_set.iter().find(|&p| p != bass).unwrap_or(bass);
            Harmony::Interval {
                low: bass,
                high,
                semitones: bass.semitones_up_to(high),
            }
        }
        _ => match find_chord(pitch_set, bass) {
            Some(chord) => Harmony::Chord { chord, bass },
            None => Harmony::Unknown(pitch_set),
        },
    }
}
```

`impl IntoIterator<Item = Note>` en un *argumento* significa «cualquier
cosa que se pueda recorrer y dé notas»: un `Vec<Note>`, un array, o un
iterador directamente desde un patrón
([`sounding_at`](../program/editors.md#notas-fantasma-oír-los-otros-canales)),
sin recoger nada por el camino. Un solo bucle construye el conjunto y
encuentra la nota más grave, y un `match` sobre el número de clases de
altura elige el tipo de respuesta.

### Encontrar el acorde

Cualquier nota de un acorde podría ser su fundamental. `find_chord`
prueba cada una, empezando por el bajo, porque los acordes se tocan casi
siempre con la fundamental abajo:

```rust
fn find_chord(pitch_set: PitchSet, bass: PitchClass) -> Option<Chord> {
    let roots = || {
        (0..12)
            .map(move |semitones| bass.transpose(semitones))
            .filter(move |&root| pitch_set.contains(root))
    };

    let exact = roots().find_map(|root| {
        ChordQuality::identify(pitch_set.relative_to(root)).map(|quality| Chord::new(root, quality))
    });

    exact.or_else(|| {
        roots().find_map(|root| {
            let mut with_fifth = pitch_set;
            with_fifth.insert(root.transpose(7));
            if with_fifth == pitch_set {
                return None; // the fifth was already there
            }
            ChordQuality::identify(with_fifth.relative_to(root))
                .filter(|quality| quality.intervals().len() >= 4)
                .map(|quality| Chord::new(root, quality))
        })
    })
}
```

- `roots` es un **closure que fabrica un iterador**. Un iterador se gasta
  a medida que avanza, así que para buscar dos veces el código llama
  otra vez a `roots()` para tener uno nuevo.
- `find_map` es `find` y `map` en uno: llama al closure con cada
  elemento y devuelve el primer `Some`.
- `or_else` sobre un `Option` ejecuta su closure solo si la opción es
  `None`. La segunda búsqueda, que permite que falte la quinta, solo se
  ejecuta cuando la exacta no encontró nada. Los músicos suelen dejar
  fuera la quinta de los acordes de séptima, y Do, Mi y Si♭ se sigue
  oyendo como C7.
- `Option::filter` conserva un `Some` solo si el closure está de
  acuerdo. Evita que «Do y Mi, más un Sol adivinado» se llame acorde de
  Do mayor completo: adivinar solo se permite en acordes de cuatro notas
  o más.

Empezar por el bajo resuelve ambigüedades reales. Do Mi Sol La es C6 y
también Am7: sobre un Do es C6, sobre un La es Am7. Do Re Sol es Csus2 y
también Gsus4. Los acordes grandes se solapan todo el tiempo: Am11 y
Cmaj13 son las mismas seis clases de altura, La Si Do Re Mi Sol, y el
bajo les da nombre.

```rust
// C E G A is both C6 and Am7: the lowest note decides.
assert_eq!(label(&[60, 64, 67, 69]).unwrap(), "C6");
assert_eq!(label(&[57, 60, 64, 67]).unwrap(), "Am7");
```

### Etiquetas: guardas de match y patrones anidados

La franja de acordes sobre el piano roll muestra una etiqueta corta para
cada paso:

```rust
pub fn label(&self, spelling: Spelling) -> Option<String> {
    match *self {
        Harmony::Silence => None,
        Harmony::Single(pitch_class) => Some(pitch_class.name(spelling).to_string()),
        // A root and its fifth is a "power chord", written C5.
        Harmony::Interval {
            low, semitones: 7, ..
        } => Some(format!("{}5", low.name(spelling))),
        // A dash, not a plus: "C+" would read as C augmented.
        Harmony::Interval { low, high, .. } => {
            Some(format!("{}–{}", low.name(spelling), high.name(spelling)))
        }
        Harmony::Chord { chord, bass } if bass != chord.root => {
            Some(format!("{}/{}", chord.name(spelling), bass.name(spelling)))
        }
        Harmony::Chord { chord, .. } => Some(chord.name(spelling)),
        // ...
        Harmony::Unknown(set) => Some(set.set_class().name()),
    }
}
```

- `semitones: 7` dentro de un patrón encaja solo con ese valor. Los
  otros intervalos pasan a la rama siguiente.
- `..` significa «y los demás campos, sean cuales sean».
- `if bass != chord.root` es una **guarda de match**: las inversiones
  llevan barra, los acordes en estado fundamental no.
- Las ramas se prueban desde arriba, así que la más específica va
  primero. Si el `Harmony::Chord { chord, .. }` simple fuera primero, la
  rama de la barra nunca encajaría.
- `match *self` hace el match sobre el valor que hay detrás de la
  referencia. Funciona porque todos los campos son `Copy`, así que los
  patrones los copian.

Un grupo sin nombre de acorde recibe el nombre de su **clase de
conjunto**, como 3-1 para tres semitonos vecinos ([Conjuntos de clases
de altura, y una armonía con cuatro nombres](sets.md)).

`label` nombra las teclas negras con sostenidos o bemoles simples.
`label_in(scale)` escribe la misma respuesta en una tonalidad, con
`Scale::spell` y `Scale::spell_root`: E♭ en lugar de D♯ en Do mayor, A/C♯
en Re menor, E♯dim en Fa♯ mayor.

`describe(scale)` escribe una frase para la línea del piano roll sobre
el paso bajo el puntero, y para la barra de estado mientras mantienes
teclas pulsadas, como «Am: La menor, acorde vi de Do mayor.» o «C/E: Do
mayor con Mi en el bajo, acorde I⁶ de Do mayor.» Dice si un acorde es
de la tonalidad, **prestado** de la tonalidad homónima, o una
**dominante secundaria**; esas palabras se explican en [Números
romanos, las teclas de armonía y las funciones de los
acordes](numerals.md).

### Nombrar un intervalo

Dos notas son un **intervalo**, y un intervalo tiene dos nombres. Su
tamaño en semitonos da el nombre de todos los días, una tabla de doce
(`interval_name`: 3 es una «tercera menor», 6 un «tritono»). Pero los
músicos lo nombran por cómo está escrito: las letras dan el número, los
semitonos la especie. Do a Mi es una tercera (Do, Re, Mi), y también lo
son Do a Mi♭ y Do♯ a Mi; una tercera de cuatro semitonos es mayor, de
tres menor. Así que Do a Re♯ es una segunda aumentada, aunque suene como
la tercera menor Do a Mi♭.

```rust
let letters = usize::from(low.letter.steps_up_to(high.letter));
let semitones = low.pitch_class().semitones_up_to(high.pitch_class()) as i8;
// How far it is from the major or perfect interval, from -6 to 5.
let off = (semitones - MAJOR[letters] + 18).rem_euclid(12) - 6;
let perfect = matches!(letters, 0 | 3 | 4);
let quality = match (perfect, off) {
    (true, 0) => "perfect",
    (false, 0) => "major",
    (false, -1) => "minor",
    (_, 1) => "augmented",
    (true, -1) | (false, -2) => "diminished",
    (_, 2) => "doubly augmented",
    _ => "doubly diminished",
};
```

Los unísonos, las cuartas y las quintas son «justos» y no tienen mayor
ni menor; un semitono menos que justo es disminuido, mientras que para
los demás disminuido es uno menos que menor. Sumar 18 y tomar
`rem_euclid(12)` antes de restar 6 lleva cualquier diferencia al rango
de -6 a 5, así que una séptima que pasa de la octava se sigue comparando
bien.

En una tonalidad, las dos notas toman los nombres de la tonalidad, así
que en La menor armónica Fa a Sol♯ es de verdad una segunda aumentada,
el famoso hueco de esa escala. Una nota de fuera de la tonalidad que
formaría un intervalo raro se nombra en cambio de la manera habitual
para el tamaño del intervalo: en Do mayor, Do y Mi♭ forman una tercera
menor.

## Inversiones

Un símbolo de acorde escribe una inversión con una barra. Un número
romano la escribe con las cifras del **bajo cifrado**, la abreviatura de
los teclistas barrocos, que son los intervalos sobre el bajo:

| Acorde | Nota más grave | Símbolo | Número en Do mayor |
|--------|----------------|---------|--------------------|
| Do mayor | Do, la fundamental | C | I |
| Do mayor | Mi, la tercera | C/E | I⁶ (primera inversión) |
| Do mayor | Sol, la quinta | C/G | I⁶₄ (segunda inversión) |
| G7 | Sol, la fundamental | G7 | V7 |
| G7 | Si, la tercera | G7/B | V⁶₅ (primera inversión) |
| G7 | Re, la quinta | G7/D | V⁴₃ (segunda inversión) |
| G7 | Fa, la séptima | G7/F | V⁴₂ (tercera inversión) |

C/E tiene Do una sexta por encima de Mi y Sol una tercera por encima,
«6/3», abreviado ⁶. Un acorde de séptima tiene tres intervalos sobre su
bajo; las cifras conservan los dos que distinguen sus inversiones, y
ocupan el lugar del 7: G7/B es V⁶₅, no V7⁶₅.

`Chord::tone_of` encuentra qué nota del acorde es una altura, recorriendo
juntos los intervalos y sus números:

```rust
pub fn tone_of(self, pitch_class: PitchClass) -> Option<u8> {
    let semitones = self.root.semitones_up_to(pitch_class);
    self.quality
        .intervals()
        .iter()
        .zip(self.quality.tones())
        .find(|(interval, _)| **interval % 12 == semitones)
        .map(|(_, &tone)| tone)
}
```

e `inversion_figures` lo convierte en cifras con un solo `match` sobre
una tupla:

```rust
match (self.tone_of(bass), triad, seventh) {
    (Some(3), true, _) => "⁶",
    (Some(5), true, _) => "⁶₄",
    (Some(3), _, true) => "⁶₅",
    (Some(5), _, true) => "⁴₃",
    (Some(7), _, true) => "⁴₂",
    _ => "",
}
```

Los acordes sus, de sexta y de novena no tienen cifras; se quedan con el
nombre con barra. Cómo van las cifras sobre un número romano, y qué
cambia una inversión sobre adónde va un acorde después (la cuarta y
sexta cadencial), está en [Números romanos, las teclas de armonía y las
funciones de los acordes](numerals.md).

### Una pista, o todas a la vez

La franja de acordes del piano roll tiene dos filas. La primera nombra
los acordes de la pista que se está editando, paso a paso, con
`analyze`. El piano de la banda puede tocar G7 como Si Re Fa Sol, así
que por sí solo es G7/B. La segunda fila, **Todo**, nombra lo que toca
la banda entera ahí, pulso a pulso: si el bajo toca Fa bajo ese G7, la
banda toca G7/F, V⁴₂. Nombrar la banda entera es más difícil que nombrar
una pista, porque la melodía y la línea del bajo tienen notas de paso
que no están en el acorde, y un arpegio toca su acorde nota a nota. Ese
es el trabajo de la escucha de acordes, en [Oír la armonía: acordes y
tonalidades a partir de las notas](hearing.md).

## Cómo se prueba

Los tests se leen como un libro de armonía. Las notas de cada clase son
distintas, cada clase tiene un número de nota para cada intervalo, y los
acordes se escriben por sus letras:

```rust
assert_eq!(
    spelled(name(Letter::C, 0), ChordQuality::Diminished7),
    "C Eb Gb Bbb"
);
assert_eq!(
    spelled(name(Letter::E, 0), ChordQuality::Dominant7Sharp9),
    "E G# B D F##"
);
```

El análisis nombra disposiciones reales, en cualquier orden y octava, y
sin su quinta:

```rust
assert_eq!(label(&[48, 64, 67, 69, 74]).unwrap(), "C6/9");
assert_eq!(label(&[55, 59, 62, 65, 68]).unwrap(), "G7b9");
// E, G#, B, D and G: the "Hendrix chord".
assert_eq!(label(&[52, 56, 59, 62, 67]).unwrap(), "E7#9");
```

```rust
assert_eq!(label(&[48, 64, 70]).unwrap(), "C7");
```

Las descripciones usan la tonalidad, inversiones incluidas:

```rust
assert_eq!(describe(&[57, 60, 64]), "Am: A minor, chord vi of C major.");
```

Y los símbolos de acorde se leen en todas sus maneras de escribirlos,
de «C» y «Am7» a «C7(b9)», «Bø» y «Cm(maj7)», y se llevan a las doce
tonalidades por sus letras.

Los ejercicios sobre acordes están en [Ejercicios: la teoría musical en
código](exercises.md).
