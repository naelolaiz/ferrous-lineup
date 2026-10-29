# La hoja de acordes: escribir una canción compás a compás

Archivos: [`crates/lineup-core/src/chord_sheet.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/chord_sheet.rs),
[`crates/lineup-core/src/project.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/project.rs),
[`crates/lineup-core/src/file.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/file.rs),
[`crates/lineup-core/src/remeter.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/remeter.rs)
y [`crates/lineup-app/src/chord_sheet.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/chord_sheet.rs)

En un programa de banda de acompañamiento, lo primero que ves es una
cuadrícula de compases, y en ella escribes una canción: "C", Tab,
"Am7 D7", Tab. Este cambio le da a Ferrous Lineup esa cuadrícula, la
**hoja de acordes**. Se abre con **F5**, el botón **📝 Acordes** de la barra
de herramientas o **Ver > Hoja de acordes**, como una ventana sobre la
lista de reproducción, que se queda como estaba.

![El chord sheet con la demo de Rhythm changes: dos choruses de ocho compases, C A7 | Dm7 G7 y así, con números romanos bajo cada acorde](../images/step-9-chord-sheet.png)

En este cambio la hoja es algo para leer y escribir; el cambio
siguiente, [La banda toca la hoja de acordes](band.md), pone una banda
detrás. Lo que ya hace es contener la armonía de la canción, nombrarla
de todas las maneras que conoce la app, seguir a la canción cuando
cambia y escribirse a partir de lo que toca la canción.

## Leer la hoja

La hoja se organiza como un chart impreso:

- **Cuatro compases por línea.** Cada compás muestra su número en la
  esquina, y la indicación de compás donde cambia (el "4/4" del compás
  1). (Más tarde la hoja aprendió a poner en una línea tantos compases
  como permiten sus nombres: véase [La hoja de acordes llena la
  ventana](chord-sheet-layout.md).)
- **Cada sección empieza una línea nueva**, bajo su nombre: «Estribillo 1»,
  «Estribillo 2». Una sección tocada tres veces se encabeza «Estrofa ×3». Una
  doble línea de compás marca dónde empieza una sección, y la línea de
  compás final (fina y después gruesa) dónde acaba la canción.
- **Cada armonía se sienta donde cae su pulso.** "C A7" en un compás de
  cuatro pulsos pone C en el borde izquierdo y A7 a mitad de camino, en
  el pulso 3.
- **Bajo cada acorde, su número romano en la tonalidad**: Fm en Do
  mayor es iv, y A7, la dominante de Dm, es V7/ii (véase [Acordes
  prestados, dominantes secundarias y acordes
  pivote](chord-functions.md)). Una inversión lleva las cifras del bajo
  cifrado: G7/B es V⁶₅, C/E es I⁶. Bajo un conjunto o un micromodo, sus
  notas.
- **Un compás sin nada escrito sigue con la armonía anterior**, que se
  muestra tenue para que el compás no parezca vacío.
- **Un cambio de tonalidad** se escribe arriba a la derecha del compás
  donde ocurre, «→ Sol mayor», en naranja como los nombres de las
  secciones.

Señalar una armonía dice qué es, con las mismas palabras que las
teclas de armonía: «A7: séptima de dominante de La, V7/ii en Do mayor: una
dominante secundaria, la dominante de Dm. Sus notas: La Do♯ Mi Sol».
Mantener pulsado el botón del ratón sobre ella la toca, con su nota de
bajo debajo.

Mientras suena la canción, el compás que suena recibe un marco blanco,
como la tecla de armonía que suena, y el acorde que suena es el nombre más
brillante de la hoja; los demás se atenúan. La hoja se desplaza con la
canción.

![El mismo sheet mientras suena la canción: el compás 9 enmarcado en blanco, su Do brillante, el resto atenuado](../images/step-9-chord-sheet-playing.png)

## Escribir en un compás

Haz clic en un compás para seleccionarlo (un marco naranja) y escribe.
La primera tecla que pulsas abre un campo de texto sobre el compás con
esa letra dentro; Intro acepta el texto y selecciona el compás
siguiente, Tab lo acepta y sigue escribiendo directamente en el compás
siguiente, Mayús+Tab vuelve uno atrás, y Esc deja el compás como
estaba. Un doble clic, Intro o F2 en un compás seleccionado lo abre con
lo que contiene, para cambiarlo. Supr lo vacía. Las flechas se mueven
de compás en compás, arriba y abajo por líneas.

Con el teclado de escritura activado (las letras tocan notas), una
letra no puede empezar a escribir en un compás; haz doble clic en el
compás o pulsa Intro primero.

Un compás puede contener cualquiera de estos, separados por espacios:

| Lo que escribes | Qué es | Ejemplo |
|-----------------|--------|---------|
| Un símbolo de acorde | cualquier acorde que sepa leer [Nombres de acordes](chord-names.md) | `Am7`, `G/B`, `F#m7b5`, `C6/9` |
| Un polychord | dos acordes a la vez, primero el de arriba, unidos por `\|` | `Eb7\|Fb`, `C\|F#` |
| Un número romano | un acorde de la tonalidad, en una tonalidad de siete notas | `ii7 V7` en Do mayor se escribe Dm7 G7 |
| Un conjunto de clases de alturas | por su nombre de Forte, movido con `Tn` o `TnI` | `3-11 T2` es Re Fa La |
| | o por sus notas, entre llaves | `{0,1,4}`, `{D F A}`, `{t e 0}` |
| Un micromodo | uno de los doce de Kröpfl, movido de la misma manera | `Menor 3`, `Menor 3 T4I` |
| Una forma de serie | una forma de la serie dodecafónica de la tonalidad | `P0`, `I5`, `R3`, `RI7` |
| Sin acorde | nada toca la armonía | `N.C.` o `NC` |
| Un sostenido | la armonía anterior sigue un pulso más | `/` o `.` |
| El compás anterior | el compás sigue con la armonía anterior | `%`, o nada |

Dos acordes a la vez llegaron más tarde, con el formato 25: `Eb7|Fb` es
E♭7 sobre Fa♭ mayor, el acorde que abre los Augurios primaverales. Una barra
significaría una nota de bajo, así que las dos capas se unen con una
pleca ([Oír acordes](chord-hearing.md)).

Lo que escribes se anota de una sola manera. Un número romano se
convierte en el acorde que es en la tonalidad, de modo que "V7" escrito
en Si♭ mayor se anota como F7, y un acorde se escribe como lo escriben
las teclas de armonía: "CM7" se convierte en Cmaj7. Un conjunto escrito por
sus notas se convierte en su nombre de Forte: `{D F A}` es 3-11 T2.

### Dónde caen las armonías

Las partituras guía tienen una convención para varios acordes en un compás,
y la hoja la sigue:

| Escrito en 4/4 | Pulsos |
|----------------|--------|
| `Am7` | Am7 en el 1, durante todo el compás |
| `Am7 D7` | Am7 en el 1, D7 en el 3: dos pulsos cada uno |
| `C F G` | C en el 1, F en el 3, G en el 4: dos, uno y uno |
| `C Am Dm G7` | uno en cada pulso |
| `C / / G7` | C en el 1, G7 en el 4 |
| `/ G7` | el acorde anterior sigue un pulso, G7 en el 2 |

En 3/4, dos acordes se reparten dos pulsos y uno: "C G7" pone G7 en el
pulso 3. La regla detrás del reparto a partes iguales es una línea: la
armonía `k`-ésima de `n` en un compás de `beats` pulsos va en el pulso
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
Cuando el compás contiene algún `/`, cada palabra es un pulso (`place`
es la posición de la palabra), de modo que puedes poner un acorde en el
pulso que quieras.

### Nombres de varias palabras

"Menor 3 T2" es una armonía escrita en tres palabras, y "C Am" son dos
armonías en dos palabras. `read_bar` no puede partir por espacios y
leer cada palabra por separado. En cambio, en cada palabra prueba
primero la serie de palabras más larga, hasta cuatro, y toma la primera
que se lea como una armonía:

```rust
let holds_at = words[at..]
    .iter()
    .position(|word| matches!(*word, "/" | "."))
    .map_or(words.len(), |offset| at + offset);
let longest = (at + 1..=holds_at.min(at + MOST_WORDS))
    .rev()
    .find_map(|end| {
        HarmonyName::parse_in(&words[at..end].join(" "), key)
            .ok()
            .map(|harmony| (harmony, end))
    });
```

`(a..=b).rev()` cuenta hacia atrás desde `b` hasta `a`, y `find_map` se
detiene en el primer `Some`. Así, "Menor 3 T2 C" lee "Menor 3 T2 C" (no
es una armonía), luego "Menor 3 T2" (un micromodo) y sigue desde "C".

Una serie nunca pasa de un `/` que está solo (`holds_at` es dónde está
el siguiente), porque esa barra es un pulso: "A E / B" es A, E
sostenido dos pulsos, luego B, no A y el acorde con barra E/B. Esa
regla llegó más tarde, cuando leer charts como texto encontró el error
([Hojas de acordes como texto](text-chord-sheets.md#dos-errores-viejos-encontrados-por-un-viaje-de-ida-y-vuelta)).

### Cuando el texto no se puede leer

Si una palabra no es una armonía, o hay más armonías que pulsos, el
campo sigue abierto con el cursor al final, y el motivo se escribe
encima de la hoja en rojo:

> Compás 3: "Hm7" no es un acorde (Am7, G/B), un conjunto (3-11 T2,
> {0,4,7}), un micromodo (Menor 3), una forma de serie (P0, RI7) ni
> N.C.

> Compás 5: este compás tiene 3 pulsos, y en él hay 4 armonías
> escritas.

No se escribe nada en la canción hasta que el compás entero se lee.

## La misma hoja en conjuntos y micromodos

Ferrous Lineup no es solo para canciones en tonalidades mayores y
menores. Las demos de micromodos están en una tonalidad que es un
conjunto de clases de alturas (el conjunto 3-3 sobre Do), y allí el
hoja escribe micromodos, cada uno con sus notas debajo, en el morado
con el que las teclas de armonía dibujan los grupos de notas sin función en
una tonalidad:

![El chord sheet de una demo de micromodos: Menor 3, Menor 3 T4I, Menor 3 T3 y así, con sus notas debajo](../images/step-9-chord-sheet-micromodes.png)

Menor 3 es el nombre de Kröpfl para las notas do, do♯, mi: Do Do♯ Mi, el
3-3 de Forte. `T4I` lo pone del revés (Do Si La♭) y mueve eso cuatro
semitonos hacia arriba (Mi Mi♭ Do), de modo que Menor 3 T4I es Do Mi♭ Mi. Son
los mismos nombres que usan las ventanas Micromodos y Conjuntos (véase
[Micromodos](../step-8/micromodes.md)).

## Escribir lo que toca la canción

Una canción se abre con sus acordes ya en la hoja, de modo que el
hoja nunca está vacía para una canción que toca acordes. De dónde
salen depende de la canción:

- **Una demo construida a partir de un chart** (las canciones y estilos
  del menú Demostraciones) escribe su propio chart en la hoja, parte a parte,
  cada parte en su propia tonalidad. La tercera sección de El Choclo,
  en La mayor, dice A E7 | E7 A | F#7 Bm | B7 E7: el VI7 ii | II7 V7
  del chart nombrado en La mayor. El acorde sostenido del final
  también se escribe.
- **Cualquier otra canción** (la canción inicial, un archivo que
  guardaste antes de que existiera la hoja, un archivo MIDI importado
  o una grabación) recibe los acordes que la app oye en ella, como se
  describe abajo.

Escribirlos cuando la canción se abre, antes de que la app recuerde el
proyecto como guardado, hace que una demo no aparezca como modificada
(sin `*` en el título).

Unas pocas piezas tienen su acompañamiento escrito nota a nota (Para
Elisa, Aria en la cuerda de Sol, el Preludio en do sostenido menor de
Rachmaninoff, Shchedryk). No tienen chart, y sus arpegios juntan
demasiado pocas notas a la vez para que se oiga un acorde pulso a
pulso, así que sus hojas se abren aún casi vacías.

```rust
fn write_chart(project: &mut Project, at: usize, spans: &[ChordSpan], key: Scale) {
    let Some(end) = spans.last().map(|span| at + span.start + span.length) else {
        return;
    };
    let markers: Vec<ChordMarker> = spans
        .iter()
        .map(|span| ChordMarker::new(at + span.start, HarmonyName::Chord(span.chord.symbol_in(key))))
        .collect();
    project.set_chords(at, end, &markers);
}
```

`let ... else` toma el final del último span, o vuelve enseguida cuando
el chart está vacío (`spans.last()` es `None`). `symbol_in(key)` es la
misma función que usa el menú Progresión para nombrar "V7" como A7 en
Re menor, de modo que la hoja escribe los acordes de una demo
exactamente como los leería la banda.

El botón **Escribir los acordes que toca la canción**, arriba de la hoja, las
escribe otra vez a partir de lo que la canción toca ahora, en lugar de
lo que dice la hoja: úsalo después de cambiar las notas. Recorre la
canción pulso a pulso, pregunta qué acorde toca allí la banda y escribe
una armonía donde eso cambia. Un deshacer lo revierte todo. Mientras
una banda toca desde la hoja, el botón está oculto, ya que las notas
de la banda salen de la hoja para empezar.

Tres detalles hacen que el resultado se lea como un chart y no como un
análisis:

1. **El bajo sale de la línea de bajo.** Las teclas de armonía oyen los
   canales que contienen acordes, y un voicing de piano de La menor como
   Do Mi La sería "Am/C". Una partitura guía escribe la nota que toda la banda
   tiene abajo del todo, así que el bajo después de la barra es la nota
   más grave de todos los canales, incluida la línea de bajo, cuando
   esa nota está en el acorde. Una nota de paso del bajo que no está en
   el acorde se omite, y el acorde se escribe sobre su fundamental.
   "Cuatro acordes" sale como C G Am F, sin las inversiones del piano como
   G/B y Am/C.
2. **Una sección empieza con su acorde escrito otra vez.** Rhythm
   changes acaba sus ocho compases en Do y empieza los ocho siguientes
   en Do. Sin una regla, el segundo chorus se abriría con un compás
   vacío que sigue al primero. Los charts escriben el acorde otra vez
   al principio de cada parte, y la hoja hace lo mismo.
3. **En un conjunto o una serie, escribe conjuntos.** En una tonalidad
   que es un conjunto o una serie dodecafónica, un nombre de acorde
   como "C#m(maj7)/B#" no dice nada verdadero sobre la música. Allí las
   notas oídas juntas se escriben como un conjunto por su nombre de
   Forte, o como un micromodo cuando la tonalidad es uno de los
   tricordios de Kröpfl y suenan tres notas.

```rust
let harmony = if chord_sheet::names_groups(key) {
    let heard = chord_notes(parts)
        .iter()
        .map(|note| note.pitch_class())
        .collect();
    match chord_sheet::heard_group(heard, key) {
        Some(group) => group,
        None => continue,
    }
} else {
    match heard_chord(parts, key) {
        Some(symbol) => HarmonyName::Chord(symbol),
        None => continue,
    }
};
let section_starts = self.playlist.sections.iter().any(|s| s.start == step);
if section_starts || markers.last().is_none_or(|last| last.harmony != harmony) {
    markers.push(ChordMarker::new(step, harmony));
}
```

`continue` salta al pulso siguiente del bucle `for`: un pulso sin
acorde, o con notas que no forman un acorde que la app conozca, sigue
con el acorde anterior. `is_none_or` es true para una lista vacía, así
que la primera armonía siempre se escribe.

## Cómo se guarda la hoja

La hoja no es una cuadrícula de compases en memoria. Es una lista de
**marcadores de acordes**, cada uno diciendo «de este paso en adelante,
la armonía es esta», como los marcadores de compás, tempo y tonalidad
que lo preceden:

```rust
pub struct ChordMarker {
    /// The step it starts on, counted from the start of the song.
    pub start: usize,
    pub harmony: HarmonyName,
}

pub enum HarmonyName {
    Chord(ChordSymbol),
    /// Two chords at once, the upper one first: "Eb7|Fb". Added later,
    /// with format 25.
    Polychord(ChordSymbol, ChordSymbol),
    NoChord,
    Set(PitchSet),
    Micromode(PitchSet),
    Row(RowLabel),
}
```

`Polychord` llegó después de este capítulo, con el formato 25. Los
acordes que escribe este capítulo se guardan como formato 12; un
proyecto con `Eb7|Fb` en su hoja necesita al menos el formato 25
([Oír acordes](chord-hearing.md)).

Una armonía dura hasta el marcador siguiente, igual que un acorde en una partitura guía dura hasta que se escribe el acorde siguiente. Por eso un
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
devuelve cuántos marcadores empiezan en `step` o antes; el que está
antes de ese índice es la respuesta. `checked_sub(1)` es `None` cuando
no hay ninguno, antes del primer acorde.

Mantener los marcadores en el timeline, y no una lista de compases,
deja que la hoja comparta el código que ya mueve los demás marcadores:
cuando una herramienta desplaza todo lo que hay después de un paso, los
marcadores de acordes se desplazan en el mismo bucle que los marcadores
de compás, tempo y tonalidad (véase «Seguir la canción», más abajo). En
qué compás cae un marcador se calcula cuando se dibuja.

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
#[derive(Serialize, Deserialize)]
#[serde(into = "String", try_from = "String")]
pub enum HarmonyName { … }
```

`into = "String"` le dice a serde que guarde un `HarmonyName`
convirtiéndolo primero en un `String` (con `impl From<HarmonyName> for
String`, que usa el texto de `Display`), y `try_from = "String"` que
cargue uno leyendo un `String` y convirtiéndolo de vuelta con
`impl TryFrom<String> for HarmonyName`, que es `HarmonyName::parse`. Si
el texto no se puede leer, el archivo se rechaza con el motivo, como
cualquier otro archivo roto.

El formato de archivo es ahora el 12. Un proyecto con acordes en su
hoja se guarda como formato 12; uno sin ellos se queda en el formato
que necesitaba antes, de modo que sigue abriéndose en la versión
anterior a esta. El número importa porque una versión antigua, al leer
el archivo, saltaría los `chord_markers` que no conoce y perdería el
hoja la próxima vez que guardara. En su lugar, rechaza cualquier
archivo cuyo formato sea más nuevo que el suyo, con un mensaje que lo
dice (la prueba `files_from_the_future_are_refused`).

### Escribir un compás

Todo cambio en la hoja pasa por un método, que escribe las armonías de
un tramo de la canción, normalmente un compás:

```rust
pub fn set_chords(&mut self, start: usize, end: usize, markers: &[ChordMarker]) -> bool
```

Los marcadores que ya están entre `start` y `end` se van, y `markers`
ocupan su lugar. Escribir un compás entero de una vez, en lugar de
añadir o mover marcadores sueltos, coincide con lo que haces en el
hoja: escribes un compás, y lo que escribiste es lo que el compás
contiene. Deshacer no necesita nada nuevo: el `edit` de la app guarda
una copia del proyecto de antes de cada cambio, así que «escribir
acordes» es un paso que deshacer como cualquier otro.

## Seguir la canción

Como los marcadores están en el timeline de la lista de reproducción, toda
herramienta que mueve cosas en el timeline los mueve también:

- **Duplicar una sección** (⧉ Duplicar en el menú de la sección) o
  **desplegar sus repeticiones**: todo lo que viene después se mueve, y
  cada copia recibe los acordes de la sección, empezando por el acorde
  con el que se abre la sección, aunque ese acorde estuviera escrito en
  un compás anterior.
- **Cambiar el compás de toda la canción** ([Cambiar la
  métrica](../step-8/re-metering.md)): cada marcador se mueve con su
  compás y cae en el pulso más cercano del compás nuevo, ya que un
  acorde cambia en un pulso. Un acorde en el pulso 3 de un compás de
  4/4, a mitad de camino, cae a mitad del compás de 3/4, pulso y medio
  dentro, y va al pulso 3: en una ligadura, gana el pulso posterior.
- **Cambiar la tonalidad** ([Cambiar de
  tonalidad](../step-8/changing-key.md)): la hoja se mueve como lo
  hacen las notas, de modo que sigue nombrando lo que suena. Una
  fundamental nueva mueve cada acorde, contando nombres de nota: G7/B en Do
  mayor es A7/C# en Re mayor. Un modo nuevo mueve cada nota de un
  acorde al mismo grado de la escala nueva, como se mueven las notas en
  los patrones: Do mayor en Do mayor pasa a ser Do menor en Do menor, y
  G7 pasa a ser Gm7 porque Si se mueve a Si♭. Los conjuntos y los
  micromodos conservan su forma y se mueven con la fundamental.

Cada una de estas cosas tiene una prueba en `project.rs` o `remeter.rs`
que escribe una hoja, hace el cambio y comprueba adónde fue cada
marcador.

## La ventana

La ventana está en `lineup-app`, y casi todo es dibujar. Unas pocas
partes merecen un vistazo.

### Un rectángulo por compás

La hoja no está hecho de botones. La ventana toma el espacio que
necesitan todas las líneas con `ui.allocate_exact_size`, calcula un
`Rect` para cada compás (un `Cell`) y le pregunta a egui por cada uno:

```rust
let id = ui.scope_id().with(("chord-sheet-bar", bar.number));
let response = ui.interact(rect, id, Sense::click_and_drag());
```

`ui.interact` convierte cualquier rectángulo en algo que se puede
sobrevolar, clicar o arrastrar, y devuelve el mismo `Response` que
devolvería un botón. El id tiene que ser el mismo de un frame al
siguiente, porque egui recuerda por id sobre qué widget bajó el ratón;
el número de compás lo hace. Los nombres, los rótulos y las líneas de
compás se pintan después con `painter.text`, `painter.vline` y
`painter.rect_stroke`.

Los nombres de cada compás se dibujan a un solo tamaño, el mayor con el
que el nombre más apretado aún cabe, de modo que un compás nunca mezcla
nombres grandes y pequeños. Un nombre que viene de un compás anterior
solo se encoge a sí mismo.

### Un campo de texto sobre un compás

Escribir ocurre en un `TextEdit` normal, colocado sobre el compás con
`ui.put(rect, field)`. Dos cosas requirieron cuidado:

- **La primera letra.** Seleccionas un compás y escribes "A": el campo
  se abre en ese frame, así que la "A" ya la ha leído la hoja. El
  campo se abre con la "A" dentro y el cursor detrás, escribiendo el
  cursor en el estado guardado del campo antes de dibujarlo:

  ```rust
  let mut state = egui::text_edit::TextEditState::load(ui.ctx(), id).unwrap_or_default();
  state.cursor.set_char_range(Some(range));
  state.store(ui.ctx(), id);
  ui.memory_mut(|memory| memory.request_focus(id));
  ```

- **Tab y las flechas.** egui usa Tab para moverse entre widgets, y un
  campo de texto suelta el teclado con él. En la hoja, Tab significa
  «compás siguiente», así que el campo saca Tab de la entrada antes de
  que egui lo vea (`input.consume_key(Modifiers::NONE, Key::Tab)`) y le
  dice a egui qué teclas conserva la hoja mientras tiene el teclado:

  ```rust
  const KEEP_KEYS: egui::EventFilter = egui::EventFilter {
      tab: true,
      horizontal_arrows: true,
      vertical_arrows: true,
      escape: false,
  };
  ui.memory_mut(|memory| memory.set_focus_lock_filter(id, KEEP_KEYS));
  ```

Una más: el Intro que cierra el campo sigue «pulsado» en ese frame, y
las propias teclas de la hoja leen Intro como «abrir este compás». Sin
una protección, Intro cerraría un compás y abriría el siguiente a la
vez. La hoja recuerda si se estaba escribiendo en un compás cuando
empezó el frame, y lee sus propias teclas solo si no:

```rust
let was_editing = self.editing.is_some();
self.edit_field(ui, project, &cells, &keys, sheet.id, actions);
// …
if has_keyboard && self.editing.is_none() && !was_editing {
    self.keys(ui, project, &cells, typing_keys, actions);
}
```

### Los dígitos pertenecen a la hoja mientras escribes

Las teclas de armonía tocan un acorde cuando pulsas del 1 al 7. En la hoja de acordes, los dígitos son parte de lo que escribes: "3-11 T2", "{0,1,4}",
"C6/9". Así que mientras la hoja tiene el teclado,
`ChordSheet::has_keyboard()` es true y las teclas de armonía dejan en paz a
los dígitos.

### Acciones, no cambios

Como el rack de canales y el piano roll, la hoja nunca cambia el proyecto
por sí mismo. Dibuja desde `&Project` (un préstamo compartido y de solo
lectura) y devuelve una lista de lo que se hizo:

```rust
pub enum SheetAction {
    SetChords { start: usize, end: usize, markers: Vec<ChordMarker> },
    WriteHeardChords,
    Seek(usize),
    NoteOn(Vec<Note>),
    NoteOff(Vec<Note>),
}
```

`LineupApp::apply_sheet` hace después cada una: `SetChords` a través de
`edit`, para que se pueda deshacer, `Seek` moviendo el cursor de reproducción, y las
notas por el mismo camino que las teclas de armonía. La ventana no puede
olvidarse de registrar un paso de deshacer, porque no tiene ninguna
manera de cambiar el proyecto; el borrow checker se lo negaría.

## Ejercicios

1. **Copiar y pegar compases.** Añade Ctrl+C y Ctrl+V en un compás
   seleccionado: copia sus marcadores (con su `start` hecho relativo al
   compás) y pégalos en otro compás con `set_chords`. ¿Qué debería
   pasar cuando los compases tienen distinta métrica? (`snap_to_beat`
   puede ayudar.)
2. **El número romano de un compás, en todas partes.** Añade un
   interruptor que muestre el número romano en grande y el nombre del
   acorde pequeño debajo, como escribe una progresión un libro de
   teoría. `HarmonyName::caption` ya da el número romano; ¿qué debería
   mostrar un conjunto?
3. **Compases por línea.** Los charts de canciones en 3/4 suelen tener
   ocho compases por línea. Haz que `BARS_PER_LINE` sea un ajuste de la
   ventana, con una opción de 2, 4 u 8. ¿Dónde se usa, y las flechas
   arriba y abajo siguen cayendo en la columna correcta? (La app ya lo
   hace; pruébalo antes de leer [cómo](chord-sheet-layout.md).)
4. **Releer lo que toca la canción.** Después de «Escribir los acordes que toca la canción», cambia unas pocas notas en el piano roll. Escribe un
   método que compare la hoja con lo que se oye ahora, compás a
   compás, y devuelva los compases que difieren. ¿Cómo los mostrarías
   en la hoja?
