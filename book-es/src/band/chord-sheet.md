# La hoja de acordes: modelo, distribución y texto

Archivos: [`crates/lineup-core/src/chord_sheet.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/chord_sheet.rs),
[`crates/lineup-core/src/chord_sheet/text.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/chord_sheet/text.rs),
[`crates/lineup-core/src/bars.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/bars.rs),
[`crates/lineup-app/src/chord_sheet.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/chord_sheet.rs)
y [`crates/lineup-app/src/chord_sheet/layout.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/chord_sheet/layout.rs)

Una banda de acompañamiento necesita conocer la canción, y los músicos
anotan una canción como una cuadrícula de compases con nombres de
acordes dentro: «C», Tab, «Am7 D7», Tab. Esa cuadrícula es la **hoja de
acordes**. Se abre con **F5**, desde la barra de herramientas o desde
**Ver > Hoja de acordes**, como una ventana sobre la lista de
reproducción, o en el lugar de la lista de reproducción, en la ventana
principal. La banda de los capítulos siguientes toca a partir de ella,
y las vistas de armonía de la Parte III nombran lo que hay en ella.

![La hoja de acordes con la demostración Rhythm changes: dos vueltas de ocho compases, C A7 | Dm7 G7 y así, con números romanos bajo cada acorde](../images/step-9-chord-sheet.png)

Este capítulo sigue la hoja desde lo que escribes en un compás hasta
cómo se guarda, de dónde vienen sus acordes, cómo sigue a la canción
cuando se mueven, insertan o eliminan compases, cómo la ventana
distribuye los compases, y cómo una hoja se convierte en texto plano y
vuelve. Cómo se usa está en el capítulo del manual [La hoja de
acordes](https://naelolaiz.github.io/ferrous-lineup/es/manual/chord-sheet.html).

## Qué cabe en un compás

Haz clic en un compás y escribe. Un compás admite cualquiera de estas
cosas, separadas por espacios:

| Se escribe | Qué es | Ejemplo |
|------------|--------|---------|
| Un símbolo de acorde | cualquier acorde que lea [Acordes](../theory/chords.md#símbolos-de-acorde-leer-una-partitura-guía) | `Am7`, `G/B`, `F#m7b5`, `C6/9` |
| Un poliacorde | dos acordes a la vez, el de arriba primero, unidos por `\|` | `Eb7\|Fb`, `C\|F#` |
| Un número romano | un acorde de la tonalidad, en una tonalidad de siete notas | `ii7 V7` en Do mayor se escribe Dm7 G7 |
| Un conjunto de clases de altura | por su nombre de Forte, movido con `Tn` o `TnI` | `3-11 T2` es Re Fa La |
| | o por sus notas, entre llaves | `{0,1,4}`, `{D F A}`, `{t e 0}` |
| Un micromodo | uno de los doce de Kröpfl, movido igual | `Menor 3`, `Menor 3 T4I` |
| Una forma de serie | una forma de la serie dodecafónica de la tonalidad | `P0`, `I5`, `R3`, `RI7` |
| Sin acorde | nada toca la armonía | `N.C.` o `NC` |
| Una prolongación | la armonía anterior sigue un pulso más | `/` o `.` |
| Nada, o `%` | el compás sigue con la armonía anterior | |

Una barra después de un acorde indica una nota en el bajo, así que las
dos capas de un poliacorde se unen con una barra vertical: `Eb7|Fb` es
Mi♭7 sobre Fa♭ mayor, el acorde con que empiezan los Augurios
primaverales. Los acordes también se pueden escribir en solfeo, «Lam7»,
nombre como nombre la aplicación las notas (ver [Escribir un acorde de
las dos
maneras](../program/languages.md#escribir-un-acorde-de-las-dos-maneras)),
y una armonía puede llevar las marcas de un golpe o de una
anticipación, «C.» o «^C» (ver [Golpes, sostenidos, silencios y
anticipaciones](stops.md)).

Lo que escribes se anota de una sola manera. Un número romano se
convierte en el acorde que es en la tonalidad, así que «V7» escrito en
Si♭ mayor se anota F7; un acorde se deletrea como lo deletrean las
teclas de armonía, «CM7» pasa a ser Cmaj7; un conjunto escrito por sus
notas pasa a su nombre de Forte, `{D F A}` es 3-11 T2. En una
tonalidad menor, un número romano que escribes se lee como la hoja
escribe los números, así que «VII» en La menor es Sol mayor, como lo
escribe el ajuste de [Tonalidades menores: bIII o
III](../theory/numerals.md#tonalidades-menores-biii-o-iii). Los
cifrados de la biblioteca se leen de la otra manera, desde la escala
mayor, porque así están escritos; `read_bar` y `read_typed_bar` solo se
diferencian en eso.

### Dónde caen las armonías

Las partituras guía tienen una convención para varios acordes en un
compás, y la hoja la sigue:

| Escrito en 4/4 | Pulsos |
|----------------|--------|
| `Am7` | Am7 en el 1, todo el compás |
| `Am7 D7` | Am7 en el 1, D7 en el 3: dos pulsos cada uno |
| `C F G` | C en el 1, F en el 3, G en el 4: dos, uno y uno |
| `C Am Dm G7` | uno en cada pulso |
| `C / / G7` | C en el 1, G7 en el 4 |
| `/ G7` | el acorde anterior sigue un pulso, G7 en el 2 |

En 3/4, dos acordes ocupan dos pulsos y uno: «C G7» pone G7 en el
pulso 3. La regla del reparto parejo es una línea: la armonía número
`k` de `n` en un compás de `beats` pulsos va en el pulso
`(k × beats) / n`, redondeado hacia arriba.

```rust
let beat = if holds {
    place
} else {
    (harmony_count * beats.len()).div_ceil(harmonies)
};
```

`div_ceil` es la división entera redondeada hacia arriba, así que para
tres armonías en cuatro pulsos los pulsos son 0, ⌈4/3⌉ = 2 y ⌈8/3⌉ = 3.
Cuando el compás tiene algún `/`, cada palabra es un pulso (`place` es
la posición de la palabra), así que puedes poner un acorde en el pulso
que quieras. Un compás con más armonías que pulsos se rechaza: «este
compás tiene 3 pulsos, y tiene 4 armonías escritas.»

### Nombres de varias palabras

«Menor 3 T2» es una armonía escrita en tres palabras, y «C Am» son dos
armonías en dos palabras, así que un compás no se puede partir por los
espacios y leer cada palabra por separado. En cambio, en cada palabra
el lector prueba primero el tramo de palabras más largo, hasta cuatro,
y se queda con el primero que se lee como una armonía:

```rust
let holds_at = words[at..]
    .iter()
    .position(|word| matches!(*word, "/" | "."))
    .map_or(words.len(), |offset| at + offset);
let longest = (at + 1..=holds_at.min(at + MOST_WORDS))
    .rev()
    .find_map(|end| {
        read_marked(&words[at..end].join(" "), key, numerals)
            .ok()
            .map(|marked| (marked, end))
    });
```

`(a..=b).rev()` cuenta hacia atrás de `b` a `a`, y `find_map` se detiene
en el primer `Some`. Así, «Menor 3 T2 C» lee «Menor 3 T2 C» (no es una
armonía), después «Menor 3 T2» (un micromodo), y sigue desde «C». Un
tramo nunca pasa de un `/` suelto (`holds_at` es donde está el
siguiente), porque esa barra es un pulso: «A E / B» es La, Mi sostenido
dos pulsos y después Si, no La y el acorde con bajo E/B.

Si una palabra no es ninguna armonía, el compás queda abierto con el
cursor al final, y el motivo se escribe sobre la hoja en rojo:

> Compás 3: «Hm7» no es un acorde (Am7, G/B), dos acordes a la vez
> (Eb7|Fb), un conjunto (3-11 T2, {0,4,7}), un micromodo (Menor 3), una
> forma de serie (P0, RI7) ni N.C.

No se escribe nada en la canción hasta que se lee el compás entero.

### Volver a escribir un compás

Al hacer clic en un compás, su campo de texto se llena con lo que
tiene, así que `write_bar(markers, bar)` tiene que dar un texto que
`read_bar` vuelva a leer como los mismos marcadores. Escribe solo los
nombres cuando caen donde los pondría la regla del reparto, «Am7 D7», y
si no, una palabra por pulso, con `/` en los pulsos que siguen,
«C / / G7». Un `/` al final no dice nada y se quita, salvo cuando es el
último que queda:

```rust
while words.last().is_some_and(|word| word == "/")
    && words[..words.len() - 1].iter().any(|word| word == "/")
{
    words.pop();
}
```

«C G7 C /» pone tres acordes en los tres primeros pulsos. Sin su barra,
«C G7 C» serían tres acordes repartidos por el compás, en los pulsos 1,
3 y 4.

## Cómo se guarda la hoja

La hoja no es una cuadrícula de compases en memoria. Es una lista de
**marcadores de acorde**, cada uno diciendo «desde este paso, la
armonía es esta», como los marcadores de compás, de tempo y de
tonalidad:

```rust
pub struct ChordMarker {
    /// The step it starts on, counted from the start of the song.
    pub start: usize,
    pub harmony: HarmonyName,
    /// Whether the band plays it early.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub push: Option<Push>,
    /// Whether the band stops playing its style here.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stop: Option<Stop>,
}
```

La armonía es una de las seis clases que admite un compás:

```rust
pub enum HarmonyName {
    /// A chord symbol: "Am7", "G/B".
    Chord(ChordSymbol),
    /// Two chords at once, one over the other, the upper one first:
    /// "Eb7|Fb" is E♭7 over F♭ major, the chord that opens the Augurs of
    /// Spring. Lead sheets draw the two chords as a fraction, with a
    /// line between them; a slash would mean a bass note.
    Polychord(ChordSymbol, ChordSymbol),
    /// No chord, written "N.C.": nothing plays the harmony here.
    NoChord,
    /// A pitch-class set on these very notes, named by its Forte number
    /// and how it is moved from the prime form: "3-11 T2" is D F A.
    Set(PitchSet),
    /// One of Kröpfl's micromodes on these three notes: "Menor 3 T2".
    Micromode(PitchSet),
    /// A form of the twelve-tone row of the key here: "RI7".
    Row(RowLabel),
}
```

Una armonía dura hasta el siguiente marcador, como un acorde en una
partitura guía dura hasta que se escribe el siguiente. Por eso un
compás sin nada no necesita guardar nada, y por eso «la armonía en el
paso 200» es una búsqueda del último marcador en el 200 o antes:

```rust
pub fn chord_at(&self, step: usize) -> Option<ChordMarker> {
    let markers = &self.playlist.chord_markers;
    let after = markers.partition_point(|marker| marker.start <= step);
    after.checked_sub(1).map(|index| markers[index])
}
```

`partition_point` hace una búsqueda binaria en la lista ordenada y
devuelve cuántos marcadores empiezan en `step` o antes; el anterior a
ese índice es la respuesta. `checked_sub(1)` es `None` cuando no hay
ninguno, antes del primer acorde.

Todo cambio de la hoja pasa por un único método, que escribe las
armonías de un tramo de la canción, normalmente un compás:

```rust
pub fn set_chords(&mut self, start: usize, end: usize, markers: &[ChordMarker]) -> bool {
```

Los marcadores que ya había entre `start` y `end` se van, y `markers`
ocupan su lugar. Escribir un compás entero de una vez, en lugar de
añadir o mover marcadores sueltos, coincide con lo que haces en la
hoja: escribes un compás, y lo que escribiste es lo que tiene el
compás. Después se ordena la lista:

```rust
pub(crate) fn tidy(markers: &mut Vec<ChordMarker>, end: usize) {
    markers.retain(|marker| marker.start < end);
    markers.reverse();
    markers.sort_by_key(|marker| marker.start);
    markers.dedup_by_key(|marker| marker.start);
}
```

Dos marcadores en un mismo paso: debe ganar el último escrito.
`dedup_by_key` se queda con el *primero* de cada serie de claves
iguales, así que la lista se invierte antes de ordenarla. `sort_by_key`
es un orden **estable**, que mantiene los elementos iguales en el orden
en que estaban, así que después de invertir, el más nuevo de cada paso
queda primero y sobrevive. Deshacer no necesita nada nuevo: el `edit`
de la aplicación guarda una copia del proyecto anterior a cada cambio
(ver [Deshacer y rehacer](../program/undo.md)).

Guardar marcadores en la línea de tiempo, y no una lista de compases,
permite que la hoja comparta el código que mueve los otros marcadores:
cuando una herramienta desplaza todo lo que hay después de un paso, los
marcadores de acorde se desplazan en el mismo bucle que los de compás,
tempo y tonalidad. En qué compás cae un marcador se calcula al
dibujarlo.

### En el archivo

La armonía se guarda como el texto que escribirías, no como números:

```ron
chord_markers: [
    (start: 0, harmony: "C"),
    (start: 8, harmony: "A7"),
    (start: 16, harmony: "Dm7"),
    (start: 64, harmony: "Menor 3 T2I"),
],
```

Eso requiere dos atributos en el enum:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(into = "String", try_from = "String")]
```

`into = "String"` le dice a serde que guarde un `HarmonyName`
convirtiéndolo primero en un `String`, con
`impl From<HarmonyName> for String`, y `try_from = "String"` que cargue
uno leyendo un `String` y convirtiéndolo de vuelta con
`impl TryFrom<String> for HarmonyName`. `try_from` y no `from`, porque
leer puede fallar: un archivo con una armonía que no se puede leer se
rechaza con el motivo, como cualquier archivo roto. Las dos conversiones
trabajan en letras, nombre como nombre la aplicación las notas, para
que un archivo sea el mismo para todos (ver [Siempre en letras y en
inglés](../program/saving.md#siempre-en-letras-y-en-inglés)):

```rust
impl From<HarmonyName> for String {
    fn from(harmony: HarmonyName) -> Self {
        NoteNaming::Letters.with(|| harmony.to_string())
    }
}
```

Un proyecto con acordes en la hoja se guarda con el formato 12 o
posterior, y uno con un poliacorde con el formato 25 (ver [El número de
versión](../program/saving.md#el-número-de-versión)).

## De dónde vienen los acordes

Una canción se abre con sus acordes ya en la hoja, así que la hoja
nunca está en blanco para una canción que toca acordes.

Una **demostración hecha a partir de un cifrado** escribe su propio
cifrado, parte por parte, cada parte en su tonalidad. La tercera parte
de El Choclo, en La mayor, dice A E7 | E7 A | F#7 Bm | B7 E7: el
VI7 ii | II7 V7 del cifrado nombrado en La mayor.

```rust
fn write_chart(project: &mut Project, at: usize, spans: &[ChordSpan], key: Scale) {
    let Some(end) = spans.last().map(|span| at + span.start + span.length) else {
        return;
    };
    let markers: Vec<ChordMarker> = spans
        .iter()
        .map(|span| {
            ChordMarker::new(
                at + span.start,
                HarmonyName::Chord(span.chord.symbol_in(key)),
            )
            .pushed(span.push)
            .stopped(span.stop)
        })
        .collect();
    project.set_chords(at, end, &markers);
}
```

`let ... else` toma el final del último tramo, o vuelve enseguida
cuando el cifrado está vacío (`spans.last()` es `None`). `symbol_in(key)`
es la misma función que usa el menú Progresión para nombrar «V7» como
A7 en Re menor, así que la hoja deletrea los acordes de una
demostración exactamente como los lee la banda. `pushed` y `stopped`
toman `self` por valor y devuelven una copia cambiada, así que se
encadenan.

**Cualquier otra canción**, un archivo guardado sin acordes, un archivo
MIDI o una grabación importados, recibe los acordes que la aplicación
oye en ella, al abrirse y mientras su hoja esté vacía. El botón
**Escribir los acordes que toca la canción** lo vuelve a hacer cuando
se lo pides, en lugar de lo que dice la hoja; con una banda tocando a
partir de una hoja que tiene acordes, el botón se oculta, ya que las
notas de la banda salen de la hoja. Cómo se oye la canción, un compás
cortado en tramos con programación dinámica y cada tramo nombrado
entero, está en [Cortar un compás en
acordes](../theory/hearing.md#cortar-un-compás-en-acordes). Dos
reglas hacen que el resultado se lea como un cifrado y no como un
análisis. La nota después de la barra es la nota más grave de toda la
banda, la línea de bajo incluida, así que unos acordes de piano
dispuestos Do Mi La sobre un bajo que toca La son Am, no Am/C. Y una
sección empieza con su acorde escrito de nuevo, como un cifrado vuelve
a escribir el acorde al comienzo de cada parte:

```rust
let section_starts = self.playlist.sections.iter().any(|s| s.start == start);
if section_starts || markers.last().is_none_or(|last| last.harmony != harmony) {
    markers.push(ChordMarker::new(start, harmony));
}
```

`is_none_or` es verdadero para una lista vacía, así que la primera
armonía siempre se escribe.

## Seguir a la canción

Como los marcadores están en la línea de tiempo de la lista de
reproducción, las herramientas que mueven cosas en ella también los
mueven a ellos:

- **Copiar una sección**, **escribir sus repeticiones** o **poner una
  introducción delante** hacen sitio con `Project::make_room`, que
  desplaza cada clip, sección y marcador desde un paso en adelante, los
  marcadores de acorde con el resto. Cada copia de una sección recibe
  sus acordes, empezando por aquel con el que abre la sección, aunque
  ese acorde se haya escrito en un compás anterior.
- **Pasar la canción a otro compás** mueve cada marcador con su compás
  y lo deja en el pulso más cercano del compás nuevo, ya que un acorde
  cambia en un pulso (ver [Pasar una canción a otro
  compás](../sound/tempo.md#pasar-una-canción-a-otro-compás)).
- **Cambiar la tonalidad** mueve la hoja como se mueven las notas, así
  que sigue nombrando lo que suena.

Un cambio de tonalidad es un método de `HarmonyName`:

```rust
match self {
    HarmonyName::Chord(symbol) if from.kind == to.kind => {
        HarmonyName::Chord(symbol.transposed(from.spell(from.root), to.spell(to.root)))
    }
    HarmonyName::Chord(symbol) => HarmonyName::Chord(chord_moved(symbol, from, to)),
    // ...
    HarmonyName::NoChord => HarmonyName::NoChord,
    HarmonyName::Set(set) => HarmonyName::Set(set.transpose(up)),
    HarmonyName::Micromode(set) => HarmonyName::Micromode(set.transpose(up)),
    HarmonyName::Row(label) => {
        HarmonyName::Row(RowLabel::new(label.form, label.number + up))
    }
}
```

El primer brazo tiene una **guarda de match**, `if from.kind == to.kind`:
acepta un acorde solo cuando el modo no cambia. Entonces el acorde se
mueve por letras, como en [Llevar una hoja de acordes a otra
tonalidad](../theory/chords.md#llevar-una-hoja-de-acordes-a-otra-tonalidad):
G7/B en Do mayor es A7/C# en Re mayor. Cuando cambia el modo, el
segundo brazo lleva cada nota del acorde al mismo grado de la nueva
escala, como se mueven las notas de los patrones: el acorde de Do mayor
en Do mayor pasa a ser Do menor en Do menor, y G7 pasa a ser Gm7,
porque su Si baja a Si♭. Los conjuntos y los micromodos no tienen
grados, así que conservan su forma y solo se mueven con la
fundamental; una forma de serie se mueve con la serie.

## Insertar y eliminar compases

Un cifrado que estás escribiendo rara vez sale bien a la primera. Al
puente le falta un compás antes de la vuelta, o una transcripción tiene
un compás de más. Haz clic derecho en un compás de la hoja, o en un
número de compás de la regla de la lista de reproducción, para
**Insertar un compás antes**, **Insertar un compás después** y
**Eliminar este compás** (o **Eliminar los compases 5–8** cuando hay
varios seleccionados). Cada uno es un solo paso de deshacer, y toda la
canción después de ese lugar se mueve:

```text
before:  | C    | F    | G7   | C    |
insert a bar after bar 2:
after:   | C    | F    | %    | G7   | C    |
```

![El menú sobre el compás 3 de Embraceable You: escribir en él, vaciarlo, llevar allí el cursor de reproducción, insertar un compás antes o después, o eliminarlo.](../images/step-9-insert-bar-menu.png)

### Qué es un compás insertado

Un compás nuevo está **vacío**, y vacío quiere decir «aquí no cambia
nada»: la armonía que venía antes sigue a través de él, y también la
tonalidad, el tempo y la variación de la banda. Eso es lo que significa
también un compás vacío en un cifrado, y por eso el menú no pregunta
qué poner en él.

Está en el **compás** (la métrica) del compás junto al que se inserta,
y en la **sección** de ese compás. Un compás insertado antes del primer
compás de una sección de vals es un compás de vals, aunque el anterior
esté en 4/4: estabas mirando el compás de vals cuando pediste otro como
él. Todo lo que hay después se mueve un compás más tarde: clips,
secciones, el bucle, los marcadores de compás, tempo, tonalidad y
acordes, y las variaciones y los estilos por sección de la banda.

### Qué se lleva un compás eliminado

Un compás eliminado se lleva sus notas y sus acordes, y todo lo que hay
después se mueve un compás antes. La pregunta es qué pasa con **lo que
sonaba al final de él**. Toma C, F, G7, C con la tonalidad cambiando a
Sol mayor en el compás 3 y el tempo a 90 en el compás 4, y elimina el
compás 3. Los marcadores del compás 3 se van con él, pero la tonalidad
de Sol mayor debía seguir después, y lo sigue debiendo. Así que, antes
de mover nada, `close_bars` lee lo que sonaba al final de los compases
eliminados:

```rust
// What goes on at the end of the bars goes on from their start.
let time = meter.time_at(end);
let tempo = self.tempo_map().tempo_at(end);
let key = self.key_at(end);
```

y después vuelve a poner cada cosa donde estaban los compases. Los
marcadores que dicen lo mismo que el anterior se quitan después, así
que eliminar un compás en medio de un tramo largo en una sola tonalidad
no deja una marca de tonalidad de más.

La armonía es la delicada. Si un G7 empieza en un compás eliminado y se
sostiene en el siguiente, el G7 debe seguir allí. Pero si el compás
anterior al eliminado ya es un G7, escribir otro G7 sería un segundo
marcador que dice lo mismo. El código comprueba las dos cosas:

```rust
if let Some(marker) = going_on
    && self.playlist.chord_markers.iter().all(|m| m.start != start)
    && start
        .checked_sub(1)
        .and_then(|step| self.chord_at(step))
        .is_none_or(|before| before.harmony != marker.harmony)
{
    self.playlist
        .chord_markers
        .push(ChordMarker::new(start, marker.harmony));
}
```

Esto es una **cadena de let** (*let chain*): `if let` y condiciones
simples unidas con `&&`, estables desde Rust 1.88 con la edición 2024.
Cada parte puede usar lo que ligó un `let` anterior: `marker` se usa en
la última línea de la condición. Sin cadenas de let esto necesita un
`if let` con un `if` anidado dentro. `start.checked_sub(1)` es `None`
en el primer compás, donde no hay «antes»; `and_then` deja pasar el
`None` y si no busca el acorde un paso antes; e `is_none_or` dice que
sí cuando no hay acorde antes, o cuando el que hay es distinto.

### Mover cada paso: un closure

Insertar usa `make_room`, el mismo método que copiar una sección.
Eliminar necesita lo contrario, y un caso más: lo que estaba **dentro**
de los pasos eliminados. Un closure dice adónde va cualquier paso de la
canción:

```rust
// Where a step of the song goes: the deleted bars close up.
let place = |step: usize| match step {
    step if step <= start => step,
    step if step >= end => step - steps,
    _ => start,
};
```

Los pasos anteriores a los compases eliminados se quedan; los
posteriores se mueven `steps` antes; los de dentro caen donde estaban
los compases. Después se aplica a todo lo que tiene un lugar en la
canción. El final de una sección también pasa por `place`, así que una
sección que contenía los compases se acorta, y una hecha solo de ellos
queda con largo 0 y se quita. El bucle se conserva solo si queda algo
de él:

```rust
playlist.loop_region = playlist.loop_region.and_then(|region| {
    let (from, to) = (place(region.start), place(region.end));
    (from < to).then_some(crate::LoopRegion::new(from, to))
});
```

`bool::then_some` convierte `true` en `Some(value)` y `false` en
`None`: aquí, «el bucle si todavía tiene largo».

### Las notas: abrir el patrón o cortar el clip

Un clip toca un **patrón**, y las notas están en el patrón, no en el
clip. Insertar un compás en medio de un clip tiene que hacer sitio en
las notas que toca. Hay dos casos:

- **El patrón lo toca solo este clip, una vez.** Entonces el propio
  patrón puede cambiar: recibe los pasos vacíos o pierde los
  eliminados, y el clip toca el patrón más largo o más corto.
- **El patrón se toca también en otro lugar**, en otro clip o en este
  mismo repetido. Entonces el patrón no debe cambiar, o los otros
  lugares cambiarían con él. El clip se **corta en dos** donde entra o
  sale el compás, y cada mitad toca un patrón propio: «Riff (1)» y
  «Riff (2)».

![La lista de reproducción después de insertar: A (1) toca los compases 1 a 3, el compás 4 está vacío, y A (2) toca el resto del viejo A desde el compás 5, con la batería cortada igual.](../images/step-9-insert-bar-playlist.png)

```rust
/// Whether clip `index` plays its pattern once and no other clip
/// plays it, so the pattern can change with it.
fn only_clip(&self, index: usize) -> bool {
    let clip = self.playlist.clips[index];
    clip.repeats == 1
        && self
            .playlist
            .clips
            .iter()
            .enumerate()
            .all(|(other, playing)| other == index || playing.pattern != clip.pattern)
}
```

`enumerate` empareja cada clip con su índice, así que `all` puede
saltarse el propio clip (`other == index`) y preguntar por cada uno de
los demás. Un corte que cae entre dos repeticiones de un clip no
necesita ningún patrón nuevo: un clip que toca cuatro veces un ritmo de
un compás, al eliminarse su segundo compás, pasa a ser un clip que lo
toca una vez y otro que lo toca dos.

Las notas que se sostienen a través del lugar se **cortan** allí, y las
de después se mueven:

```rust
/// `notes` with `steps` empty steps put in at step `at`: the notes from
/// there on move later, and a note held across it ends there.
fn opened(notes: &[NoteEvent], at: usize, steps: usize) -> Vec<NoteEvent> {
    let at_tick = at * TICKS_PER_STEP;
    notes
        .iter()
        .map(|note| match note.start_tick() < at_tick {
            true => cut_short(note, at_tick),
            false => NoteEvent {
                start: note.start + steps,
                ..*note
            },
        })
        .collect()
}
```

Una nota sostenida sobre el compás nuevo se corta donde empieza el
compás, porque el compás nuevo debe estar vacío: una nota de antes
sonando debajo de lo que escribas allí sería una sorpresa.
`NoteEvent { start: …, ..*note }` es la sintaxis de actualización de
structs, una copia de `*note` con `start` cambiado; funciona con
`*note` porque `NoteEvent` es `Copy`. Las mitades de un patrón cortado
conservan su color, y una mitad cortada solo para eliminarla se quita
otra vez al final, así que eliminar no deja patrones vacíos.

### Todo o nada

Dos cosas pueden hacer imposible insertar o eliminar: la canción
pasaría de su largo máximo, o no hay sitio para los patrones que
necesita un corte (una canción tiene como mucho 999). Cualquiera de las
dos se descubre a mitad de camino, cuando algunos clips ya cambiaron.
Así que los dos empiezan guardando una copia del proyecto:

```rust
let before = self.clone();
if !self.open_clips(at, steps) || !self.make_room(at, steps) {
    *self = before;
    return false;
}
```

y la vuelven a poner si algo falla. `*self = before` reemplaza el
proyecto entero, moviendo `before` a su lugar. La aplicación dice
entonces «No hay lugar para otro compás en la canción.» o «No hay lugar
para los patrones que necesita el corte.», y nada cambió. `||` se
detiene en el primer `true`, así que `make_room` solo se ejecuta si
`open_clips` salió bien.

### La banda sigue

Las partes de la banda se escriben a partir de la hoja de acordes.
Mover todo un compás mueve también sus notas, y la aplicación no debe
confundir eso con una edición tuya: una parte que editas queda
**congelada**, como la dejaste, y deja de seguir la hoja. Así que los
dos métodos se ejecutan dentro de `moving_the_band`, que también usan
las herramientas que transponen la canción entera o le cambian el
compás:

```rust
pub fn insert_bars(&mut self, bar: usize, after: bool, count: usize) -> bool {
    self.moving_the_band(|project| project.open_bars(bar, after, count))
}
```

`moving_the_band` recibe el closure como
`impl FnOnce(&mut Project) -> R`, lo ejecuta sobre el proyecto y
después toma las partes movidas como propias de la banda (ver [Cómo
toca la banda una hoja de acordes](band.md)).

Los dos menús, el de la hoja y el de la regla, son una única función,
`bar_edits`, así que siempre ofrecen las mismas opciones con las mismas
pistas. Devuelve lo que se pulsó como un valor, un `BarEdit`; la hoja
lo envuelve en su propia acción y la lista de reproducción en la suya,
y la aplicación aplica cualquiera de las dos con `edit_bars`, que además
selecciona el compás nuevo en la hoja para que puedas escribir en él
enseguida.

## La ventana

La ventana está en `lineup-app`, y casi todo es dibujo. Sus filas de
arriba tienen los controles de la banda, la tonalidad, el menú
**Mostrar** que nombra la armonía como acordes, conjuntos o micromodos
(ver [Una armonía, cuatro
nombres](../theory/sets.md#una-armonía-cuatro-nombres)), la forma de
la canción y las herramientas de práctica; los capítulos de cada una
las explican. Debajo vienen los compases.

Cada sección empieza una línea nueva bajo su nombre, «Estrofa ×3» para
una sección que se toca tres veces. Una doble barra marca dónde empieza
una sección, y la barra final (fina y después gruesa) dónde termina la
canción. Cada armonía está donde cae su pulso, con su número romano
debajo en la tonalidad, sus notas debajo si es un conjunto o un
micromodo y, para un acorde de fuera de la tonalidad, lo que hace allí
(«V7/ii», «iv · prestado»; ver [Qué hace un
acorde](../theory/numerals.md#qué-hace-un-acorde-las-funciones-de-la-hoja-de-acordes)).
Una inversión lleva las cifras del bajo cifrado: G7/B es V⁶₅. Un
compás sin nada escrito muestra, tenue, la armonía que sigue, para que
no parezca vacío. Un cambio de tonalidad se escribe arriba a la derecha
de su compás, «→ Sol mayor». Al señalar una armonía se dice qué es:
«La7: La séptima de dominante, V7/ii en Do mayor … Sus notas: La Do#
Mi Sol.» Mantener pulsado el botón del ratón sobre ella la toca, con su
nota del bajo debajo.

Mientras suena la canción, el compás que suena lleva un marco blanco y
el acorde que suena es el nombre más brillante de la hoja; los demás se
atenúan. La hoja se desplaza junto con la canción.

![La misma hoja mientras suena la canción: el compás 9 enmarcado en blanco, su C brillante, el resto atenuado](../images/step-9-chord-sheet-playing.png)

### Un rectángulo por compás

La hoja no está hecha de botones. La ventana toma el espacio que
necesitan todas las líneas, después calcula un `Rect` para cada compás
(una `Cell`, con su línea y su columna) y le pregunta a egui por cada
uno:

```rust
let id = ui.scope_id().with(("chord-sheet-bar", bar.number));
let response = ui.interact(rect, id, Sense::click_and_drag());
```

`ui.interact` convierte cualquier rectángulo en algo sobre lo que se
puede pasar el ratón, hacer clic o arrastrar, y devuelve el mismo
`Response` que devolvería un botón. El id tiene que ser el mismo de un
frame al siguiente, porque egui recuerda por id sobre qué widget se
pulsó el ratón; el número de compás lo asegura. Los nombres, los textos
de debajo y las barras se pintan después con `painter.text`,
`painter.vline` y `painter.rect_stroke`.

### Un campo de texto sobre un compás

Se escribe en un `TextEdit` corriente, puesto sobre el compás con
`ui.put(rect, field)`. Intro toma el texto y selecciona el compás
siguiente, Tab lo toma y sigue escribiendo en el compás siguiente,
Mayús+Tab vuelve uno atrás, y Escape deja el compás como estaba. Tres
cosas requirieron cuidado.

**La primera letra.** Seleccionas un compás y escribes «A»: el campo
se abre en ese frame, así que la hoja ya leyó la «A». El campo se abre
con la «A» dentro y el cursor detrás, escribiendo el cursor en el
estado guardado del campo antes de dibujarlo. Intro o F2 abren un
compás con lo que tiene, todo seleccionado, para que escribir lo
reemplace:

```rust
let mut state = egui::text_edit::TextEditState::load(ui.ctx(), id).unwrap_or_default();
let range = match fresh {
    Fresh::SelectAll => egui::text::CCursorRange::select_all_str(&editing.text),
    Fresh::CursorAtEnd => egui::text::CCursorRange::one(egui::text::CCursor::new(
        editing.text.chars().count(),
    )),
};
state.cursor.set_char_range(Some(range));
state.store(ui.ctx(), id);
ui.memory_mut(|memory| memory.request_focus(id));
```

El cursor cuenta caracteres, no bytes: un «♯» escrito en un compás
ocupa tres bytes en UTF-8, y es un solo lugar para el cursor.

**Tab y las flechas.** egui usa Tab para pasar de un widget a otro, y
un campo de texto suelta el teclado con él. En la hoja, Tab quiere
decir «compás siguiente», así que el campo saca el Tab de la entrada
antes de que egui lo vea:

```rust
let (tab, back) = if had_focus {
    ui.input_mut(|input| {
        let back = input.consume_key(Modifiers::SHIFT, Key::Tab);
        (input.consume_key(Modifiers::NONE, Key::Tab), back)
    })
} else {
    (false, false)
};
```

y le dice a egui qué teclas se queda la hoja mientras tiene el teclado:

```rust
const KEEP_KEYS: egui::EventFilter = egui::EventFilter {
    tab: true,
    horizontal_arrows: true,
    vertical_arrows: true,
    escape: false,
};
```

**El Intro que cierra el campo** sigue «pulsado» en ese frame, y las
teclas propias de la hoja leen Intro como «abrir este compás». Sin una
protección, Intro cerraría un compás y abriría el siguiente a la vez.
La hoja recuerda si se estaba escribiendo en un compás cuando empezó el
frame, y solo lee sus teclas si no:

```rust
let was_editing = self.editing.is_some();
self.edit_field(ui, project, &cells, &keys, sheet.id, actions);
// ...
if has_keyboard && self.editing.is_none() && !was_editing {
    ui.memory_mut(|memory| memory.set_focus_lock_filter(sheet.id, KEEP_KEYS));
    self.keys(ui, project, &cells, typing_keys, actions);
}
```

Mientras la hoja tiene el teclado, las cifras son parte de lo que
escribes, «3-11 T2» o «C6/9», así que las teclas de armonía no las
tocan. Con el teclado del equipo encendido (las letras tocan notas),
una letra no puede empezar a escribir en un compás; Intro o un doble
clic lo abren primero.

### Acciones, no cambios

Como los editores de la [Parte
II](../program/editors.md#dibujar-devuelve-acciones), la hoja nunca
cambia el proyecto ella misma. Dibuja a partir de `&Project` (un
préstamo compartido, de solo lectura) y devuelve una lista de lo que se
hizo:

```rust
pub enum SheetAction {
    // ...
    /// Write the harmonies of the steps from `start` to `end`: one bar.
    SetChords {
        start: usize,
        end: usize,
        markers: Vec<ChordMarker>,
    },
    /// Write the chords the song plays on the empty sheet.
    WriteHeardChords,
    // ...
    /// Insert or delete bars.
    Bars(BarEdit),
    /// Move the playhead to this step of the playlist's timeline.
    Seek(usize),
    /// Play these notes now, as a harmony key does.
    NoteOn(Vec<Note>),
    NoteOff(Vec<Note>),
    // ...
}
```

`LineupApp::apply_sheet` hace después cada una: `SetChords` a través de
`edit`, para que se pueda deshacer, `Seek` moviendo el cursor de
reproducción, y las notas por el mismo camino que las teclas de
armonía. La ventana no puede olvidarse de registrar un paso de
deshacer, porque no tiene ninguna manera de cambiar el proyecto; el
borrow checker lo rechazaría.

## Líneas que se ajustan a los nombres

Un cifrado impreso suele tener cuatro compases por línea. La hoja
escribe nombres mucho más largos que «C» y «G7»: poliacordes como
«Eb7|Fb», conjuntos como «3-11 T7I», micromodos como «Acorde
menor-mayor T7I», cada uno con un número romano o sus notas debajo. Así
que el número de compases por línea es una elección, **Compases por
línea**, junto a **Mostrar**. Por defecto es **Ajustar a los nombres**:
tantos compases como quepan con cada nombre, y el número o las notas de
debajo, a tamaño completo. El menú dice cuántos son ahora, como en
**Ajustar (4)**. O elige un número del 1 al 16; entonces la línea tiene
esos, y los nombres que no caben se escriben más pequeños. **⛶ Llenar
la ventana** lleva la hoja a la ventana principal, al lugar de la lista
de reproducción, donde una línea tiene más sitio.

![Embraceable You en la hoja de acordes, llenando la ventana principal junto a los patrones: cuatro compases por línea, donde su ventana tenía sitio para dos, y cada nombre y número a tamaño completo.](../images/step-9-chord-sheet-fills-window.png)

### Por qué 1, 2, 4, 8, 12 o 16

Ajustar podría poner cualquier número de compases en una línea: cinco,
siete, once. Un cifrado impreso nunca lo hace, y por una razón. La
mayoría de las canciones están hechas de **frases de cuatro compases**,
y un cifrado pone el primer compás de una frase al comienzo de una
línea, así que el ojo encuentra los compases 1, 5, 9 y 13 en la misma
columna. Con cinco compases por línea, la segunda frase empezaría en la
segunda columna de la segunda línea, la tercera en la tercera: se
perdería la forma de la canción. Así que los números que ajusta son los
que mantienen las frases juntas: 1 y 2 entran justos en una frase, y 4,
8, 12 y 16 están hechos de frases enteras. Doce es un blues de doce
compases en una línea.

```rust
/// The counts of bars a line holds when they fit, from the most: each
/// divides a four-bar phrase or is made of them.
const FITS: [usize; 6] = [16, 12, 8, 4, 2, 1];

/// No bar is made narrower than this to fit more on a line, however
/// short its names.
const NARROWEST_BAR: f32 = 110.0;
```

`NARROWEST_BAR` está para las canciones de nombres cortos. Una hoja de
«C» y «G» pondría si no dieciséis compases en una ventana que muestra
cuatro con holgura, cada uno demasiado estrecho para hacer clic o
escribir en él.

La elección es un tipo:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum Columns {
    /// As many as fit with every name at its full size.
    #[default]
    Fit,
    /// This many, from 1 to [`MOST_BARS`]: names that don't fit shrink.
    Bars(usize),
}
```

`#[default]` sobre una variante le dice a `#[derive(Default)]` qué
variante es `Columns::default()`. El número sale de un único método:

```rust
pub fn count(self, width: f32, needed: f32) -> usize {
    match self {
        Columns::Bars(bars) => bars.clamp(1, MOST_BARS),
        Columns::Fit => {
            let bar = needed.max(NARROWEST_BAR);
            FITS.into_iter()
                .find(|&bars| width / bars as f32 >= bar)
                .unwrap_or(1)
        }
    }
}
```

`FITS` va de más compases a menos, así que `find` se detiene en el
número más grande cuyos compases siguen siendo lo bastante anchos. Si
ni siquiera uno cabe (un nombre muy largo en una ventana muy estrecha),
sigue siendo un compás por línea, y sus nombres se escriben más
pequeños.

`FITS.into_iter()` recorre el array **por valor**, así que sus
elementos son `usize`. `find` le pasa cada elemento a su closure por
referencia, como un `&usize`, para que pueda probarlo sin tomarlo y
devolverlo si es el que busca. El patrón `&bars` en el parámetro del
closure saca el `usize` de la referencia.

`count` es una función pura de dos números, así que su prueba no
necesita ninguna ventana:

```rust
// Short names: the narrowest bar decides.
assert_eq!(Columns::Fit.count(760.0, 60.0), 4);
assert_eq!(Columns::Fit.count(1300.0, 60.0), 8);
// Long names take more room, and whole phrases still start lines:
// room for three bars makes lines of two.
assert_eq!(Columns::Fit.count(760.0, 240.0), 2);
```

A 1300 píxeles, doce compases serían de 108 píxeles cada uno, por
debajo del compás más estrecho; ocho son de 162. A 760 píxeles con
nombres que necesitan 240, cabrían tres compases (253 cada uno), pero
tres no es un número que mantenga juntas las frases, así que son dos.

### Qué tan ancho debe ser un compás

Antes de dibujar, la hoja le pregunta a cada compás qué tan ancho debe
ser, y toma el más ancho. Lo que tiene que caber es cada **nombre** y
la **primera palabra de debajo**: el número romano de un acorde, o las
notas de un conjunto («Do Mi Sol»), que suele ser la más ancha de las
dos. Las palabras después del número, como «prestado» en
«bVII7 · prestado», se escriben solo donde hay sitio. El color del
número ya dice «prestado», y la pista lo dice todo; contar esas
palabras haría que un acorde prestado en el último pulso de un compás
dejara en la mitad los compases de todas las líneas.

```rust
fn first_word(caption: &str) -> &str {
    caption.split(" · ").next().unwrap_or(caption)
}
```

`split` da los trozos entre los separadores, uno por vez, y `next`
toma el primero. Devuelve un `Option`, porque un iterador puede estar
vacío; `split` nunca lo está, pero el tipo no lo sabe, y `unwrap_or`
dice qué usar si lo estuviera.

Cada nombre se coloca **en su pulso, o justo después del nombre
anterior** cuando ese necesita el sitio. En un compás de 4/4 con
«C A7», C empieza en el pulso 1 y A7 en el 3. Pero en un compás donde
Dm7/C dura una semicorchea y G7 el resto, G7 se escribe justo después
de Dm7/C, un poco tarde, como hace un cifrado impreso cuando los
acordes se apiñan. Un nombre nunca empieza antes de su pulso, así que
el orden de los nombres es siempre el de los acordes. Un `Row` guarda,
para cada nombre de un compás, su pulso (desde 0 al comienzo del compás
hacia 1 al final) y su ancho, medido con las fuentes del painter
(`layout_no_wrap`) como se dibujarán los nombres:

```rust
struct Row {
    /// For each name, its beat, from 0 at the start of the bar towards 1
    /// at its end, and its width.
    names: Vec<(f32, f32)>,
}
```

¿Qué tan estrecho puede ser el interior del compás? Como ningún nombre
empieza antes de su pulso, **cada nombre, con todos los que vienen
después, tiene que caber entre su pulso y el final del compás**:

```rust
fn packed_width(&self) -> f32 {
    (0..self.names.len())
        .map(|first| {
            let (beat, _) = self.names[first];
            let from = &self.names[first..];
            let widths: f32 = from.iter().map(|(_, width)| width).sum();
            let gaps = NAME_GAP * (from.len() - 1) as f32;
            (widths + gaps) / (1.0 - beat)
        })
        .fold(0.0, f32::max)
}
```

En «C / / A7», A7 empieza en el pulso 4 y tiene la última negra del
compás: un A7 de 30 píxeles de ancho necesita un compás de 120. C
empieza en el pulso 1 y necesita los dos nombres, con el espacio entre
ellos, en el compás entero. El compás necesita el mayor de los dos.
Para la semicorchea de Dm7/C, son los dos nombres uno junto al otro, no
un compás dieciséis veces más ancho que «Dm7/C». `&self.names[first..]`
es un **slice**: una vista del vector desde `first` hasta el final, sin
copiarlo. A `sum` hay que decirle qué suma, aquí `f32`, porque muchos
tipos se pueden sumar a partir de los mismos elementos.

El mismo `Row` coloca los nombres al dibujar el compás:

```rust
fn lefts(&self, left: f32, right: f32) -> Vec<f32> {
    let on_beat = |beat: f32| left + (right - left) * beat;
    let mut free = left;
    let packed: Vec<f32> = self
        .names
        .iter()
        .map(|&(beat, width)| {
            let start = on_beat(beat).max(free);
            free = start + width + NAME_GAP;
            start
        })
        .collect();
    if free - NAME_GAP <= right + 0.5 {
        packed
    } else {
        self.names.iter().map(|&(beat, _)| on_beat(beat)).collect()
    }
}
```

El closure que recibe `map` cambia `free`, una variable de fuera, cada
vez que se lo llama: es un `FnMut`, que `map` acepta porque lo llama
una vez por elemento, en orden. Cuando los nombres no caben todos así,
en una línea con un número de compases elegido por ti, cada uno empieza
en su pulso y se dibujan más pequeños. Medir y dibujar comparten un
`Row`, así que la hoja nunca hace sitio para algo que no dibuja.

El compás más ancho de la canción es un **fold**:

```rust
.map(|bar| {
    self.needed_width(ui.painter(), project, &meter, &keys, &bar, marks, &heard)
})
.fold(0.0, f32::max),
```

¿Por qué no `.max()`? `Iterator::max` necesita `Ord`, un orden total, y
`f32` no lo tiene: `NaN` no es ni menor ni mayor que ningún número.
`f32::max` es una función corriente que toma dos `f32` y devuelve el
mayor (ignorando un `NaN`), así que hacer fold con ella desde `0.0` da
el compás más ancho, o 0 para una canción sin compases. Pasar
`f32::max` por su nombre funciona porque una función es un valor que se
puede llamar, como lo sería un closure `|a, b| a.max(b)`. Medir cada
compás cuesta un poco en cada frame, pero egui guarda en caché el texto
que distribuyó hace poco, así que volver a medir el mismo nombre es una
consulta.

Con un número elegido, un compás puede ser demasiado estrecho para sus
nombres. Entonces se escriben más pequeños, con tres reglas: todos los
nombres de un compás a un mismo tamaño, el que permite el más apiñado,
porque un compás que mezcla nombres grandes y pequeños se lee como si
los pequeños importaran menos; un nombre que sigue de un compás
anterior solo se achica a sí mismo; y un conjunto o un micromodo cuyo
nombre no cabe ni al tamaño más pequeño se escribe solo por su clase
(«Menor 3» por «Menor 3 T8I»), o, donde ni eso cabe, como «…»: un
nombre cortado se leería como otro nombre.

### Ocupar el lugar de la lista de reproducción

La ventana principal tiene un panel central, donde se dibuja la lista
de reproducción. Cuando la hoja llena la ventana, ese panel dibuja la
hoja en su lugar, junto a los patrones y la Biblioteca:

```rust
// The chord sheet can take the playlist's place, beside the
// pattern picker and the library.
self.chord_sheet.remember(ctx);
let sheet_fills = self.chord_sheet.fills_window();
// ...
let playlist = egui::CentralPanel::default_margins().show(ui, |ui| {
    if sheet_fills {
        let actions = self
            .playlist
            .browser(ui, &self.project, self.pattern, tried);
        let sheet = self.chord_sheet.main(
            ui,
            &self.project,
            playing,
            typing,
            (practice, self.playlist.library_browser_mut()),
        );
        (actions, sheet)
    } else {
        let actions = self.playlist.show(
            ui,
            &self.project,
            self.pattern,
            song_position,
            tried,
            playlist_zoom,
        );
        (actions, Vec::new())
    }
});
```

Las dos ramas devuelven un par de listas de acciones: lo que pide la
lista de reproducción (o el navegador junto a la hoja), y lo que pide la
hoja. El `if` es una expresión, así que las dos ramas tienen que tener
el mismo tipo; la rama de la lista de reproducción devuelve un `Vec`
vacío para las acciones de la hoja. `ChordSheet::main` y la ventana
propia de la hoja dibujan el mismo contenido con el mismo método,
`contents`; solo cambia el marco que lo rodea, así que todo lo que hace
la hoja en su ventana lo hace en la ventana principal, sin una segunda
copia del código.

`remember` se llama **antes** que `fills_window`. Lee las elecciones
guardadas por la ejecución anterior, la primera vez que se lo llama.
Llamado después, el primer frame dibujaría la lista de reproducción y
el segundo la hoja: un destello de la vista equivocada cada vez que
arranca la aplicación.

Las teclas de zoom amplían lo que está bajo el puntero (ver [¿Qué hay
bajo el puntero?](../program/egui.md#qué-hay-bajo-el-puntero)). Con
la hoja en el lugar de la lista de reproducción, el área de la lista
tiene que contar como nada, o las teclas ampliarían una lista que
nadie ve:

```rust
let playlist = match self.chord_sheet.fills_window() {
    true => egui::Rect::NOTHING,
    false => self.playlist.area(),
};
```

`Rect::NOTHING` no contiene ningún punto, así que «¿está el puntero
sobre la lista de reproducción?» es siempre falso, y las teclas cambian
el tamaño de la interfaz. La hoja en la ventana principal conserva el
sitio que tiene, así que con nombres más pequeños caben más compases
por línea. En su propia ventana no: la ventana se achica con todo lo
demás, y tiene los mismos compases.

### Recordar las elecciones

Las dos elecciones son un struct pequeño, guardado con el estado de la
aplicación:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
struct Choices {
    columns: Columns,
    /// Whether the sheet takes the playlist's place in the main window.
    fills: bool,
}
```

`#[serde(default)]` sobre el struct dice: un campo que falta en lo
guardado toma su valor de `Choices::default()`, así que una tercera
elección añadida más tarde sigue cargando las dos primeras. `remember`
guarda una copia de lo último que guardó, y solo escribe cuando las
elecciones difieren de ella:

```rust
match self.saved {
    None => {
        self.choices = ctx
            .data_mut(|data| data.get_persisted(choices_id()))
            .unwrap_or_default();
        self.saved = Some(self.choices);
    }
    Some(saved) if saved != self.choices => {
        let choices = self.choices;
        ctx.data_mut(|data| data.insert_persisted(choices_id(), choices));
        self.saved = Some(choices);
    }
    Some(_) => {}
}
```

El brazo `None` se ejecuta una vez, en el primer frame. El segundo
brazo tiene una guarda de match: acepta solo cuando el patrón coincide y
la condición se cumple. Todo lo demás cae en `Some(_) => {}`, nada que
hacer, que es casi todos los frames.

### Las flechas caen en la columna correcta

Tenga una línea los compases que tenga, las flechas arriba y abajo van
al compás de arriba o de abajo. Nunca cuentan compases: la hoja guarda
cada compás que dibujó como una celda con su **línea** y su
**columna**, y la flecha abajo busca la celda de la línea siguiente
cuya columna está más cerca:

```rust
let in_line = |line: usize, column: usize| {
    cells
        .iter()
        .filter(|cell| cell.line == line)
        .min_by_key(|cell| cell.column.abs_diff(column))
        .map(|cell| cell.bar.number)
};
```

Las celdas salen de lo que se dibujó, así que están bien con cuatro
compases por línea, con dieciséis y en una línea corta al final de una
sección. `min_by_key` sirve aquí, a diferencia de `max` más arriba,
porque la clave es un `usize`, que es `Ord`.

## Hojas de acordes como texto

Los músicos se mandan cifrados como texto plano: en un correo antes de
un ensayo, en un foro, en una aplicación de notas. Nadie acordó un
formato, pero la mayoría de los cifrados se parecen, porque copian la
manera en que se imprime un cifrado:

```text
Title: Autumn Song
Key: C major
Time: 4/4
Tempo: 120
Style: Bossa nova

[Verse ×2]
| C      | Am7 | Dm7 G7 | C  |
| F      | G7  | G7     | C  |

[Chorus]
| F      | G   | Em7 A7 | Dm |
```

**Archivo > Importar hoja de acordes…** abre un archivo así (o uno
soltado sobre la ventana) como una canción nueva con banda: la
tonalidad, el compás, el tempo y el estilo del cifrado, sus secciones en
la lista de reproducción, sus acordes en la hoja. Si algo no se puede
leer, no se importa nada, y la barra de estado dice dónde: «línea 4:
compás 3: «Qx7» no es un acorde …». **Archivo > Exportar hoja de
acordes…** escribe la hoja de cualquier canción en el mismo formato,
con los compases alineados en columnas. En la hoja, **Ctrl+C** copia
como texto los compases seleccionados (o la hoja entera, con sus
ajustes y secciones, cuando están todos o ninguno seleccionados),
**Ctrl+X** los corta, y **Ctrl+V** escribe el texto del portapapeles
desde el compás seleccionado. Una línea sin ningún `|` se lee como un
compás, así que «Dm7 G7» copiado de un mensaje va directo a un compás.
Pegar toma solo los acordes: la canción conserva su tonalidad, su
compás, su tempo, su banda y sus secciones.

![Autumn Leaves importado desde texto: la hoja de acordes muestra el estilo de la banda (Swing), la tonalidad Sol mayor, 132 BPM, la sección A ×2 sobre los compases 1 a 8 y la sección B desde el compás 9, cada acorde con su número romano; la barra de estado dice «Se importó autumn.txt: 24 compases»](../images/step-9-text-chart-imported.png)

### El formato

Todo es opcional salvo los compases. Las palabras del formato son en
inglés, como en el texto que exporta la aplicación.

| Línea | Significa |
|-------|-----------|
| `Title: Autumn Leaves` | El nombre de la canción. Una primera línea que no es otra cosa también es el título, como arriba en un cifrado impreso. |
| `Key: G major` | La tonalidad: `G`, `Em`, `E minor`, `A Dorian`, `C harmonic minor` y las demás escalas tonales (no las series ni los conjuntos, que se eligen en la aplicación). Una tonalidad conserva el nombre escrito: `Gb major` sigue siendo Sol♭ mayor. |
| `Time: 3/4` | El compás, cualquiera que acepte la aplicación: `6/8`, `5/4`, `2+2+3/8`. |
| `Tempo: 132` | Negras por minuto; `132 BPM` también se lee. |
| `Style: Bossa nova` | El estilo de la banda, por su nombre. |
| `[Verse]` | Una sección empieza en el compás siguiente. `[Chorus ×2]`, `[Chorus x2]` o `[Chorus] x2` la toca dos veces. |
| `\| C \| Am7 \| Dm7 G7 \|` | Compases. Una línea puede tener cualquier número de ellos. |
| `# Preguntarle a Ana por el puente` | Una nota para las personas, que se salta. |

Dentro de un compás va exactamente lo que escribes en un compás de la
hoja, marcas incluidas. `||`, `|:` y `:|` se leen como barras simples;
las repeticiones se escriben con `×2` en una sección, que es como las
guarda la lista de reproducción. Una línea `Key:` o `Time:` entre
compases cambia la tonalidad o el compás desde el compás siguiente. Los
números romanos se leen en la tonalidad del cifrado, así que

```text
Key: F major
| I | vi | ii7 V7 | I |
```

da F, Dm, Gm7 C7, F, y el mismo texto con `Key: D major` da D, Bm,
Em7 A7, D: una progresión en cualquier tonalidad. Sin línea `Style:`,
la banda toca un estilo en el compás del cifrado, y sin compás
tampoco, el estilo con que empieza una canción nueva.

Un compás vacío, o `%`, se lee como en la hoja: la armonía anterior
sigue a través de él. Eso es lo que significa el signo de repetición
después de un compás de un solo acorde. Después de un compás de dos
acordes, «| C G | % |», un músico vuelve a tocar C G, mientras que la
aplicación sostiene el G.

### Leer el texto

`text::read` convierte el texto en un `TextSheet`, y solo comprueba la
disposición. Todavía no puede leer los compases: cuántos pulsos tiene
un compás depende del compás que rige en ese punto, que solo la
canción conoce cuando las líneas `Time:` anteriores están en su lugar.

```rust
pub struct TextSheet {
    pub title: Option<String>,
    /// The key the song starts in.
    pub key: Option<Scale>,
    /// The time signature the song starts in.
    pub time: Option<TimeSignature>,
    /// In quarter notes per minute.
    pub tempo: Option<f32>,
    /// The name of the band style to play it in, as written, and its
    /// line.
    pub style: Option<(String, usize)>,
    /// The bars, sections and changes, in order.
    pub items: Vec<Item>,
}

pub enum Item {
    /// A bar, as it is typed in the chord sheet: "Dm7 G7".
    Bar { text: String, line: usize },
    /// A section starts with the next bar.
    Section {
        name: String,
        repeats: usize,
        line: usize,
    },
    /// The key changes from the next bar on.
    Key(Scale),
    /// The time signature changes from the next bar on.
    Time(TimeSignature),
}
```

`Item` es un enum cuyas variantes guardan cosas distintas: dos tienen
campos con nombre, como un struct, y dos guardan un valor, como una
tupla. Un `match` sobre un `Item` tiene que decir qué hace con cada una
de las cuatro, así que cuando se añada una quinta clase de línea, el
compilador listará cada lugar que tiene que aprenderla. Los compases y
las secciones guardan el número de su línea, para que un problema
encontrado después, al leer el compás, todavía pueda nombrarla. Las
líneas se numeran con `enumerate`, que cuenta desde 0:

```rust
for (index, line) in text.lines().enumerate() {
    let number = index + 1;
    let line = line.trim();
    if line.is_empty() || line.starts_with('#') {
        continue;
    }
```

El error es un struct pequeño con la línea y el mensaje, e
`impl fmt::Display` dice cómo se imprime, «línea 4: …». Toda función
que puede fallar devuelve `Result<_, TextError>`, y `?` pasa el error
hacia arriba enseguida. `map_err` convierte otro error en este por el
camino:

```rust
let key = read_key(value).map_err(|message| error(number, message))?;
```

Una línea es un ajuste cuando tiene dos puntos y la palabra de antes es
uno de los cinco ajustes, con mayúsculas o minúsculas.
`split_once(':')` parte «Key: G major» en los primeros dos puntos, en
`("Key", " G major")`, y devuelve `None` cuando no los hay:

```rust
fn setting(line: &str) -> Option<(String, &str)> {
    let (name, value) = line.split_once(':')?;
    let name = name.trim().to_lowercase();
    ["title", "key", "time", "tempo", "style"]
        .contains(&name.as_str())
        .then_some((name, value))
}
```

Un encabezado de sección puede poner su cuenta dentro de los corchetes
o después. `read_times` lee «×2», «x2» o «X 2»:

```rust
fn read_times(text: &str) -> Option<Option<usize>> {
    let number = text
        .strip_prefix(['×', 'x', 'X'])?
        .trim()
        .parse::<usize>()
        .ok()?;
    Some((1..=MAX_REPEATS).contains(&number).then_some(number))
}
```

`strip_prefix` recibe un array de caracteres y quita el que esté al
comienzo del texto. La respuesta es una opción dentro de una opción,
porque hay tres casos que distinguir: `None` es «esto no es una cuenta»
(el encabezado sigue con otras palabras), `Some(None)` es «una cuenta,
pero demasiadas veces» (un error), y `Some(Some(2))` es dos veces. Donde
se lee el encabezado, una cadena de let busca la cuenta al final del
nombre:

```rust
if repeats.is_none()
    && let Some((first, last)) = name.rsplit_once(char::is_whitespace)
    && let Some(times) = read_times(last)
{
    repeats = Some(times);
    name = first.trim().to_string();
}
```

### Hacer la canción

`text::import` empieza desde `Project::with_band()`, la canción que
hace **Archivo > Nuevo**, y pone lo que dice el cifrado, en el orden
que necesita la banda: primero el estilo (que puede pasar la canción
vacía a su propio compás), después el compás, la tonalidad y el tempo
del cifrado por encima. Luego recorre los elementos una vez, llevando
el paso donde empieza el compás siguiente:

```rust
Item::Bar { text, line } => {
    if step + time.steps_per_bar() > MAX_SONG_STEPS {
        return Err(error(*line, tr("the song cannot be this long.")));
    }
    bars.push((step, text, *line));
    step += time.steps_per_bar();
}
Item::Time(new) => {
    time = *new;
    project.set_time_marker(TimeMarker::new(step, time));
}
Item::Key(key) => {
    project.set_key_marker(KeyMarker::new(step, *key));
}
Item::Section { name, repeats, .. } => sections.push((name.clone(), *repeats, step)),
```

Una sección termina donde empieza la siguiente, o al final de la
canción. Con cada marcador en su lugar, cada compás se lee con
`read_typed_bar`, que recibe el compás (para saber sus pulsos) y la
tonalidad al comienzo (para leer los números romanos). Todos los
acordes entran en la canción en un solo `set_chords`, y se le dice a la
banda que toque la hoja. `text::paste` hace la misma lectura en una
canción que ya existe, desde un compás, y devuelve los marcadores en
lugar de escribirlos; la ventana los convierte en un único
`SheetAction::SetChords`, la misma acción que escribir en un compás,
así que deshacer y la banda no necesitan nada nuevo.

### Escribir el texto

`text::write` recorre los compases con `write_bar`, la función que
llena el campo de texto de un compás al hacer clic en él, y junta
líneas: palabras («[Verse ×2]», «Key: D minor») o una fila de hasta
cuatro compases. Una sección, un cambio de tonalidad o de compás
empieza una fila nueva, y un compás vacío se escribe `%`. Escribe en
letras y en inglés, muestre lo que muestre la aplicación, para que el
texto se lea igual dondequiera que se pegue:

```rust
pub fn write(project: &Project, title: Option<&str>) -> String {
    Language::English.with(|| NoteNaming::Letters.with(|| write_sheet(project, title)))
}
```

Después `layout` alinea los compases. Busca el compás más ancho de cada
una de las cuatro columnas, y luego rellena cada compás hasta el ancho
de su columna:

```rust
for (width, bar) in widths.iter_mut().zip(bars) {
    *width = (*width).max(bar.chars().count());
}
```

```rust
.map(|(bar, width)| format!("{bar:<width$}"))
```

`{bar:<width$}` escribe `bar` alineado a la izquierda (`<`) en un campo
tan ancho como la variable `width` (el `$` dice que el ancho sale de
una variable). Los anchos se cuentan con `chars().count()`, no con
`len()`: `len()` cuenta los bytes del texto UTF-8, y una letra fuera
del ASCII simple (ø, ×, ♭) ocupa dos o tres bytes pero un solo lugar en
la pantalla. Una tonalidad que el texto no puede decir, una serie
dodecafónica o un conjunto, se escribe como una nota para las personas,
«# Key: … (chosen in the app)», para que el cifrado se siga leyendo y
nadie se confunda.

### Copiar, cortar y pegar

egui convierte Ctrl+C, Ctrl+X y Ctrl+V en eventos propios, con el texto
pegado ya tomado del portapapeles del sistema. Es igual en Linux, en
macOS (donde las teclas son Cmd+C, Cmd+X y Cmd+V) y en Windows. La hoja
los busca entre los eventos del frame mientras tiene el teclado y no se
está escribiendo en ningún compás; mientras se escribe en un compás, van
al campo de texto como siempre.

```rust
for event in &input.events {
    match event {
        egui::Event::Copy => copy = true,
        egui::Event::Cut => cut = true,
        egui::Event::Paste(text) => paste = Some(text.clone()),
        _ => {}
    }
}
```

Para copiar, `ctx.copy_text(text)` pone el texto en el portapapeles al
final del frame. Cortar es copiar seguido de un `SetChords` sin
marcadores sobre los compases seleccionados. Sin ningún compás
seleccionado, copiar toma la hoja entera, pero cortar no vaciaría nada,
así que deja el portapapeles como estaba.

## Cómo se prueba

La mejor prueba de un par de funciones como `write` y `read` es un
**viaje de ida y vuelta**: escribir algo, volver a leerlo y comprobar
que se obtiene lo mismo. `every_song_demo_comes_back_from_its_text`
hace eso con cada demostración de canción: la hoja de acordes de cada
una se escribe como texto, se importa, y sus acordes, tonalidades y
compases se comparan con los de la demostración. Esa prueba es la razón
de que «C G7 C /» conserve su barra y de que un nombre nunca pase por
encima de un `/`. Unas pocas demostraciones tienen un acorde entre dos
pulsos, que una hoja de acordes no puede escribir, así que la prueba
compara los acordes que caen en los pulsos, y comprueba que menos de una
demostración de cada diez tiene un acorde así. Junto a ella,
`a_bar_is_written_back_as_it_reads` y
`every_name_is_written_back_the_same_way` hacen lo mismo con un compás y
con un nombre.

En `chord_sheet.rs`, `harmonies_spread_over_the_beats_as_on_a_lead_sheet`
comprueba la tabla de más arriba, `a_bar_cannot_hold_more_than_its_beats`
el rechazo, `every_way_of_naming_harmony_is_read` y
`nonsense_is_refused_with_a_reason` los nombres,
`numerals_are_chords_in_the_key` y
`typed_numerals_in_minor_follow_the_way_numerals_are_written` los
números romanos, y `harmonies_move_with_the_key` un cambio de
tonalidad. En `text.rs`, `a_chart_is_read_line_by_line`,
`bar_lines_are_read_however_they_are_drawn`,
`a_mistake_says_its_line`, `numerals_are_read_in_the_charts_key`,
`pasted_bars_go_from_the_bar_chosen_on` y
`the_sheet_is_written_in_columns` cubren el texto.

Las pruebas de `bars.rs` trabajan sobre una canción pequeña: un patrón
de cuatro compases que toca C, F, G7, C, una estrofa sobre todo él, un
bucle sobre los compases 2 y 3, la tonalidad de Sol desde el compás 3 y
90 BPM desde el compás 4. `an_inserted_bar_moves_everything_after_it`
comprueba cada nota, acorde, tonalidad, tempo, la estrofa y el bucle;
`a_bar_inserted_before_a_change_of_time_takes_the_new_time`,
`a_clip_of_a_pattern_played_elsewhere_too_is_cut_in_two`,
`a_deleted_bar_closes_up`, `a_deleted_bar_leaves_no_pattern_behind`,
`a_harmony_from_a_deleted_bar_goes_on_where_it_was` y
`a_repeated_clip_loses_the_repeats_in_the_deleted_bars` siguen los casos
de más arriba, y `the_band_follows_the_bars` comprueba que ninguna
parte de la banda quedó congelada. Para oír que nada más se movió, la
[traza de reproducción](../sound/trace.md) de una demostración antes y
después de insertar muestra cada nota anterior al compás en el mismo
momento, y cada nota posterior exactamente un compás más tarde.

En la aplicación, `lines_fit_as_many_whole_phrases_as_there_is_room_for`
comprueba `count`. `lines_hold_as_many_bars_as_show_every_name_in_full`
empieza una canción cuyo primer compás es «Dm7/C F/C G7/D C/E» y
comprueba que cada nombre se dibuja a tamaño completo, 20 puntos;
después elige 8 compases y encuentra «Dm7/C» más pequeño.
`the_words_after_a_numeral_are_written_where_there_is_room` y
`a_short_chord_keeps_its_name_whole_with_the_next_just_after_it`
comprueban la colocación,
`the_zoom_keys_over_the_sheet_in_the_main_window_zoom_the_sheet` y
`the_sheet_fills_the_main_window_and_goes_back` la ventana principal, y
`a_bars_menu_inserts_a_bar_and_deletes_it` y
`the_bar_numbers_insert_and_delete_bars_too` los dos menús.

Los ejercicios sobre la hoja de acordes están en [Ejercicios: la
banda](exercises.md).
