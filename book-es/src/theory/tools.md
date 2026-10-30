# Herramientas de edición como funciones simples

Archivos: [`crates/lineup-core/src/tools.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/tools.rs)
y [`crates/lineup-app/src/piano_roll.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/piano_roll.rs)

El menú **Herramientas** del piano roll cambia muchas notas a la vez:
las sube un semitono, una octava o una nota de la tonalidad, las
adelanta o las atrasa, las pone en la cuadrícula o dentro de la
tonalidad, invierte acordes y los convierte en arpegios. Cada
herramienta es una función de `lineup_core::tools` que recibe unas notas
y devuelve otras nuevas. Ninguna sabe nada de ventanas, selecciones ni
deshacer, y eso es lo que las hace fáciles de leer y de probar. Este
capítulo las recorre una por una; cómo eligen una herramienta el menú y
las flechas está en [Los editores: rack de canales, piano roll y lista
de reproducción](../program/editors.md#un-enum-para-el-menú-y-las-teclas).

## La música, en pocas palabras

**Transportar** mueve la música hacia arriba o hacia abajo sin cambiar
su forma: cada intervalo sigue igual, así que una melodía subida un tono
es la misma melodía, más aguda. Do, Mi, Sol subido un tono es Re, Fa♯,
La: sigue siendo un acorde mayor.

Mover notas **dentro de la tonalidad** es otra cosa. Cada nota va a la
nota siguiente de la *escala*, no un número fijo de semitonos más
arriba, así que los intervalos cambian para encajar en la tonalidad. En
Do mayor, Do, Mi, Sol subido una nota de la tonalidad es Re, Fa, La: el
acorde del grado siguiente, Re menor, no Re mayor. Los compositores lo
han usado siempre: una figura corta repetida una nota más arriba cada
vez, sin salir de la tonalidad, es una **secuencia** (también llamada
progresión o marcha), y la música barroca está llena de ellas. Copia una
frase, pégala más adelante y súbela una nota de la tonalidad, y habrás
escrito una.

**Cuantizar** pone las notas en la cuadrícula. Nadie toca exactamente
sobre el pulso, y un archivo MIDI de una interpretación guarda cada
pequeña diferencia; cuantizar deja las notas tan exactas como una caja
de ritmos.

Una **inversión** de un acorde tiene las mismas notas con otra en el
bajo, como explica [Acordes: construirlos, nombrarlos e
invertirlos](chords.md#inversiones). Un **arpegio** toca las
notas de un acorde una detrás de otra, como un guitarrista que puntea un
acorde o un arpa, en lugar de todas a la vez.

## Notas que entran, notas que salen

Todas las herramientas tienen la misma forma. Toman prestadas las notas
como un slice, `&[NoteEvent]`, y devuelven otras nuevas; no cambian nada
por sí mismas. El módulo lo dice al principio:

```rust
//! Each tool is a plain function: it takes some notes and returns new
//! ones, and changes nothing itself. The piano roll hands the result to
//! [`Pattern::replace_notes`](crate::Pattern::replace_notes), through
//! the app's `edit`, so every tool can be undone like any other change.
```

El piano roll elige la función de cada herramienta, con un nombre para
el historial de deshacer:

```rust
let (label, new) = match tool {
    // ...
    Tool::Transpose(semitones) => {
        ("transpose notes", tools::transpose(&targets, semitones))
    }
    Tool::InKey(steps) => (
        "move notes in key",
        tools::transpose_in_key(&targets, scale, steps),
    ),
    Tool::Shift(steps) => ("move notes", tools::shift(&targets, steps, length)),
    Tool::Quantize => ("quantize notes", Some(tools::quantize(&targets, length))),
    Tool::SnapToKey => (
        "snap notes to key",
        Some(tools::snap_to_scale(&targets, scale)),
    ),
    Tool::Invert { up } => ("invert chords", tools::invert(&targets, up)),
    // ...
};
```

`targets` es la selección, o todas las notas del canal si no hay nada
seleccionado. Las notas nuevas reemplazan a las viejas en una sola
acción `ReplaceNotes`, que es un paso de deshacer; cómo funciona eso, y
por qué la selección guarda notas y no posiciones, está en [Los
editores: rack de canales, piano roll y lista de
reproducción](../program/editors.md#la-selección-guarda-notas-no-posiciones).
Tener las herramientas fuera de la aplicación también permite que otras
partes usen las mismas funciones: las pruebas de las series
dodecafónicas agrupan en acordes una serie escrita con `tools::chords`,
y el teclado MIDI ajusta las teclas con `tools::nearest_in_key`.

La mayoría de las herramientas construyen cada nota nueva a partir de
una vieja con la **sintaxis de actualización de structs**:

```rust
Some(NoteEvent {
    pitch: note.pitch.transpose(semitones)?,
    ..*note
})
```

`..*note` significa «todos los demás campos, de `*note`»: el inicio, la
duración, la velocidad, el `nudge` y el `trim` quedan como estaban, y
solo la altura es nueva. `NoteEvent` es `Copy`, así que `*note` copia la
nota vieja del slice sin quitarla de allí. Si algún día se añade un
campo a `NoteEvent`, todas las herramientas lo conservan sin cambiar
nada.

## Agrupar notas en acordes: `chords`

Invertir y arpegiar trabajan con acordes, así que la primera herramienta
los busca. Aquí un acorde es simplemente las notas que empiezan en el
mismo paso:

```rust
pub fn chords(notes: &[NoteEvent]) -> Vec<Vec<NoteEvent>> {
    let mut sorted = notes.to_vec();
    sorted.sort_by_key(|note| (note.start, note.pitch));
    // `chunk_by` cuts the sorted list wherever the start changes.
    sorted
        .chunk_by(|a, b| a.start == b.start)
        .map(<[NoteEvent]>::to_vec)
        .collect()
}
```

Ordenar por la tupla `(note.start, note.pitch)` ordena por el inicio, y
por la altura entre las notas que empiezan juntas, así que cada acorde
sale con la nota más grave primero. Después `chunk_by` corta la lista en
tramos, como hace el carril de acordes (ver [El carril de acordes:
`chunk_by`, y luego
medir](../program/editors.md#el-carril-de-acordes-chunk_by-y-luego-medir)).

Cada tramo es un slice que toma prestado de `sorted`, y `sorted` se
libera cuando la función termina. Devolver los slices los dejaría
apuntando a memoria liberada, cosa que el comprobador de préstamos
rechaza, así que cada uno se copia a un `Vec` propio.
`<[NoteEvent]>::to_vec` es el método de slice `to_vec` nombrado como
función, para que `map` lo reciba directamente en lugar de
`|run| run.to_vec()`. Los ángulos hacen falta porque `[NoteEvent]` es un
tipo, no un nombre: `[NoteEvent]::to_vec` no se podría analizar.

## Cuando una herramienta no puede: recolectar `Option`s

Transportar un acorde cerca de lo más alto del teclado puede llevar una
nota más allá de la nota MIDI 127. Dejar esa nota donde está rompería el
acorde; quitarla perdería una nota. Así que la herramienta no hace
ninguna de las dos cosas: rechaza el movimiento entero.

```rust
pub fn transpose(notes: &[NoteEvent], semitones: i32) -> Option<Vec<NoteEvent>> {
    // Collecting an iterator of `Option`s into `Option<Vec<_>>` gives
    // `None` as soon as one item is `None`, and the whole `Vec` otherwise.
    notes
        .iter()
        .map(|note| {
            Some(NoteEvent {
                pitch: note.pitch.transpose(semitones)?,
                ..*note
            })
        })
        .collect()
}
```

`Note::transpose` devuelve `None` para una nota fuera del teclado (ver
[Clases de altura, notas y su
escritura](pitch.md#mover-una-nota-option-para-puede-que-no-exista)).
El `?` dentro del closure devuelve `None` desde el *closure*, no desde
`transpose`, así que el iterador produce un `Option<NoteEvent>` por cada
nota. `collect` sabe construir muchas clases de colección, y el tipo de
retorno pide `Option<Vec<NoteEvent>>`: esa colección se detiene en el
primer `None` y es `None` ella misma, o guarda todas las notas. `Result`
funciona igual: recolectado en un `Result<Vec<_>, E>`, se detiene en el
primer error.

El piano roll no hace nada cuando una herramienta devuelve `None`, así
que pulsar ↑ sobre un acorde con una nota en la fila de arriba
simplemente no lo mueve:

```rust
let Some(new) = new.filter(|new| fits_view(new)) else {
    return;
};
```

`Option::filter` convierte `Some` en `None` cuando la prueba falla; aquí
la prueba es que todas las notas quepan en las filas que muestra el
piano roll, que son menos que las 128 notas de MIDI.

### Moverse en el tiempo: `checked_add_signed` y `then_some`

← y → mueven las notas un paso antes o después, y con Mayús cuatro
pasos, un pulso de 4/4. Los inicios son `usize`, que no puede ser
negativo, mientras que el movimiento puede ir en los dos sentidos, así
que es un `isize`:

```rust
pub fn shift(notes: &[NoteEvent], steps: isize, pattern_length: usize) -> Option<Vec<NoteEvent>> {
    notes
        .iter()
        .map(|note| {
            let start = note.start.checked_add_signed(steps)?;
            (start < pattern_length).then_some(NoteEvent { start, ..*note })
        })
        .collect()
}
```

`usize::checked_add_signed` suma un número con signo a uno sin signo y
devuelve `None` si el resultado quedaría por debajo de cero, así que una
nota en el paso 0 no puede adelantarse. `then_some` convierte el `bool`
en un `Option`: `Some` de la nota movida si todavía empieza dentro del
patrón, `None` si empezaría en su final o después. En los dos casos la
selección entera se queda donde estaba, con el mismo `collect` de antes.

### De vuelta a la cuadrícula: `quantize`

**Cuantizar a los pasos** lleva el inicio y el final de cada nota al
paso más cercano. Una nota guarda su lugar exacto como un paso más un
`nudge` y un `trim`, en ticks, 24 por paso (ver [El proyecto: patrones,
canales, pistas, clips y
marcas](../program/project.md#notas-pasos-y-patrones)), y el redondeo
está en la propia nota:

```rust
/// The note moved to the nearest steps, both its start and its end:
/// what *quantizing* does. A note shorter than a step keeps one.
pub fn quantized(self) -> NoteEvent {
    let nearest = |tick: usize| (tick + TICKS_PER_STEP / 2) / TICKS_PER_STEP;
    let start = nearest(self.start_tick());
    let end = nearest(self.end_tick()).max(start + 1);
    NoteEvent {
        velocity: self.velocity,
        ..NoteEvent::new(self.pitch, start, end - start)
    }
}
```

La división entera redondea hacia abajo, así que sumar antes medio paso
redondea al paso más cercano. `NoteEvent::new` crea una nota en la
cuadrícula, sin `nudge` ni `trim`, y la actualización de struct lo toma
todo de ella menos la velocidad. La herramienta añade una regla: una
nota que ahora empezaría en el final del patrón empieza en su último
paso.

```rust
pub fn quantize(notes: &[NoteEvent], pattern_length: usize) -> Vec<NoteEvent> {
    let last = pattern_length.saturating_sub(1);
    notes
        .iter()
        .map(|note| {
            let quantized = note.quantized();
            let start = quantized.start.min(last);
            NoteEvent {
                start,
                length: quantized.end() - start,
                ..quantized
            }
        })
        .collect()
}
```

Cuantizar es una elección, no algo que la aplicación haga por su cuenta,
porque puede cambiar la música. Los tresillos de corcheas empiezan cada
32 ticks, a un paso y un tercio de distancia, y ninguna cuadrícula de
semicorcheas los contiene: cuantizados, caen en los pasos 0, 1 y 3 y ya
no son regulares. Por eso las notas importadas de un archivo MIDI
conservan su `nudge` hasta que lo pidas. (Las notas que grabas se ponen
en la cuadrícula al grabarlas, con una función propia: ver [MIDI:
mensajes, dispositivos, grabación y
archivos](../sound/midi.md#cuantizar-recorded_note).)

## Quedarse en la tonalidad

Dos herramientas trabajan con la tonalidad: una mueve las notas una nota
de la tonalidad hacia arriba o hacia abajo, la otra lleva dentro de la
tonalidad todas las notas que están fuera. Las dos usan la tonalidad en
la que está la canción donde suena el patrón, después de las marcas de
tonalidad (ver [Escalas, modos y
tonalidades](scales.md#la-tonalidad-de-un-patrón)).

### Una nota de la tonalidad más arriba: `find_map`

La nota siguiente de la tonalidad por encima de una nota es la primera
nota, un semitono más arriba, luego dos, y así, que esté en la escala:

```rust
pub fn step_in_key(note: Note, scale: Scale, up: bool) -> Option<Note> {
    let direction = if up { 1 } else { -1 };
    (1..=12).find_map(|distance| {
        note.transpose(direction * distance)
            .filter(|candidate| scale.contains(candidate.pitch_class()))
    })
}
```

`find_map` llama al closure para cada distancia y se detiene en el
primer `Some`. El closure transporta, cosa que puede fallar en el borde
del teclado, y `filter` se queda con la nota solo si la escala la
contiene. Doce semitonos alcanzan todas las clases de altura, así que si
la escala tiene alguna nota, se encuentra una: en el peor caso la misma
nota una octava más allá, cuando la escala no tiene otra. Un rango es
perezoso, y `find_map` se detiene en el primer acierto, así que un paso
en Do mayor prueba una o dos distancias, no doce.

Mover varias notas de la tonalidad es ese paso, repetido:

```rust
pub fn transpose_in_key(notes: &[NoteEvent], scale: Scale, steps: i32) -> Option<Vec<NoteEvent>> {
    notes
        .iter()
        .map(|note| {
            let mut pitch = note.pitch;
            for _ in 0..steps.unsigned_abs() {
                pitch = step_in_key(pitch, scale, steps > 0)?;
            }
            Some(NoteEvent { pitch, ..*note })
        })
        .collect()
}
```

`unsigned_abs` da el tamaño de `steps` como número sin signo, así que el
bucle se repite tantas veces bajando como subiendo. Una nota de fuera de
la tonalidad cae en la nota de la tonalidad más cercana en la dirección
del movimiento: Do♯ subido un paso en Do mayor es Re. **Alt+↑** y
**Alt+↓** usan esta herramienta.

### La nota de la tonalidad más cercana: ajustar

Ajustar mira en los dos sentidos a la vez:

```rust
pub fn nearest_in_key(note: Note, scale: Scale) -> Note {
    if scale.contains(note.pitch_class()) {
        return note;
    }
    (1..=6)
        .find_map(|distance| {
            [-distance, distance].into_iter().find_map(|semitones| {
                note.transpose(semitones)
                    .filter(|candidate| scale.contains(candidate.pitch_class()))
            })
        })
        .unwrap_or(note)
}
```

Para cada distancia, el `find_map` de dentro prueba la nota de abajo
antes que la de arriba, así que cuando dos notas de la tonalidad están
igual de cerca, gana la más grave: en Do mayor, Do♯ se ajusta a Do, Fa♯ a
Fa y Si♭ a La. Hace falta una regla, porque ninguna de las dos es más
correcta, y esta es fácil de recordar. Seis semitonos hacia arriba o
hacia abajo alcanzan las doce clases de altura, así que cualquier escala
con alguna nota se encuentra; solo en los extremos del teclado pueden
fallar los dos intentos, y `unwrap_or(note)` deja entonces la nota donde
está. Ajustar nunca rechaza como lo hace un movimiento; en el peor caso
una nota se queda quieta.

`snap_to_scale` la aplica a muchas notas, para **Ajustar a la
tonalidad** del menú Herramientas:

```rust
pub fn snap_to_scale(notes: &[NoteEvent], scale: Scale) -> Vec<NoteEvent> {
    notes
        .iter()
        .map(|note| NoteEvent {
            pitch: nearest_in_key(note.pitch, scale),
            ..*note
        })
        .collect()
}
```

Las mismas dos funciones hacen el botón **Ajustar a la escala** del
piano roll: una nota que dibujas cae en `nearest_in_key` de la fila en la
que hiciste clic, y un grupo que arrastras se ajusta con `snap_to_scale`
una vez que se ha movido (ver [Mover
un grupo](../program/editors.md#mover-un-grupo)). Una tecla tocada en
el teclado de escribir o en un teclado MIDI se ajusta del mismo modo
(ver [MIDI: mensajes, dispositivos, grabación y
archivos](../sound/midi.md#ajustar-las-teclas-a-la-tonalidad)).

## Inversiones

**Invertir los acordes hacia arriba** sube la nota más grave de cada
acorde por encima de la más aguda; **Invertir los acordes hacia abajo**
baja la más aguda por debajo de la más grave. Las dos mueven una nota
por octavas hasta que pasa el otro extremo del acorde:

```rust
/// `note` moved up by octaves until it is above `ceiling`.
fn above(note: Note, ceiling: Note) -> Option<Note> {
    let mut note = note;
    while note <= ceiling {
        note = note.transpose(12)?;
    }
    Some(note)
}
```

«Hasta que esté por encima», no «una octava más arriba»: en Do, Mi, Sol,
Do, el Do grave tiene que subir dos octavas para quedar por encima del
Do agudo. El `?` se rinde si la nota saldría del teclado.

```rust
pub fn invert(notes: &[NoteEvent], up: bool) -> Option<Vec<NoteEvent>> {
    let mut result = Vec::with_capacity(notes.len());
    for mut chord in chords(notes) {
        if let (Some(&lowest), Some(&highest)) = (chord.first(), chord.last())
            && chord.len() >= 2
        {
            if up {
                chord[0].pitch = above(lowest.pitch, highest.pitch)?;
            } else {
                let last = chord.len() - 1;
                chord[last].pitch = below(highest.pitch, lowest.pitch)?;
            }
            chord.sort_by_key(|note| note.pitch);
        }
        result.extend(chord);
    }
    Some(result)
}
```

`chords` devuelve cada acorde con la nota más grave primero, así que
`first` y `last` son el bajo y la nota de arriba. Devuelven `Option`s,
porque un `Vec` vacío no tiene ninguna de las dos, y hacer match sobre
una tupla de ambas las toma en un solo patrón. El `&` de `Some(&lowest)`
copia la nota de la referencia que devuelve `first`, así que `lowest` y
`highest` son `NoteEvent`s normales y no préstamos de `chord`. El `&&`
forma una **cadena de let** (*let chain*): el cuerpo se ejecuta solo si
existen las dos notas *y* el acorde tiene dos notas o más, así que una
nota sola se deja como está. `for mut chord` hace de cada acorde una
variable que se puede cambiar, sin un `let mut` dentro del bucle.

`inversion`, una hermana para una lista de alturas, da un acorde en
cualquier inversión de una vez. El sello de acordes del piano roll la usa
para el menú de inversión que está junto a **Dibujar** (ver [Acordes:
construirlos, nombrarlos e
invertirlos](chords.md#sellos-de-acorde)):

```rust
pub fn inversion(pitches: &[Note], times: usize) -> Vec<Note> {
    let mut notes = pitches.to_vec();
    notes.sort();
    for _ in 0..times % notes.len().max(1) {
        let (Some(&lowest), Some(&highest)) = (notes.first(), notes.last()) else {
            break;
        };
        let Some(moved) = above(lowest, highest) else {
            break; // no room above: leave the chord as it is
        };
        notes.remove(0);
        notes.push(moved);
    }
    notes
}
```

Un acorde de tres notas tiene tres posiciones, así que girarlo tres
veces vuelve al estado fundamental: `times % notes.len()` se salta las
vueltas completas. `.max(1)` evita que una lista vacía divida por cero,
lo que provocaría un pánico. El `let ... else` toma los dos extremos o
sale del bucle. A diferencia de `invert`, esta no rechaza cuando una
nota no puede subir más: un sello cerca de lo más alto del teclado sigue
dibujando un acorde, en la última inversión que cabe.

## Arpegios: `cycle`, `step_by` y `zip`

**Arpegiar los acordes** toca las notas de cada acorde una detrás de
otra, mientras duraba el acorde. El menú Herramientas elige el orden
(**arriba**, **abajo**, **arriba y abajo**) y cuánto dura cada nota
(**1 paso**, **2 pasos**, **1 pulso**). El orden es un enum:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ArpeggioOrder {
    #[default]
    Up,
    Down,
    UpDown,
}
```

`#[derive(Default)]` en un enum necesita que se le diga qué variante es
la predeterminada, con `#[default]`. Cada orden sabe qué nota del acorde
viene después, contando desde la más grave, para siempre:

```rust
fn indices(self, count: usize) -> impl Iterator<Item = usize> {
    let once: Vec<usize> = match self {
        ArpeggioOrder::Up => (0..count).collect(),
        ArpeggioOrder::Down => (0..count).rev().collect(),
        ArpeggioOrder::UpDown => (0..count)
            .chain((1..count.saturating_sub(1)).rev())
            .collect(),
    };
    // `cycle` repeats an iterator forever. It is lazy, so "forever"
    // costs nothing: the caller takes only as many as it needs.
    once.into_iter().cycle()
}
```

Para tres notas, arriba y abajo es 0, 1, 2, 1, y luego otra vez desde
0: la nota de arriba y la de abajo se tocan una vez en cada vuelta,
nunca dos veces seguidas. `saturating_sub` se detiene en cero en lugar
de pasar por debajo, así que `count - 1` no puede desbordar el `usize`
para un acorde vacío. El tipo de retorno `impl Iterator<Item = usize>`
dice «algún iterador de `usize`» sin nombrar su tipo, que aquí sería
`Cycle<std::vec::IntoIter<usize>>`; a quien lo llama solo le hace falta
saber que puede recorrerlo.

```rust
pub fn arpeggiate(notes: &[NoteEvent], order: ArpeggioOrder, rate: usize) -> Vec<NoteEvent> {
    let rate = rate.max(1);
    let chords = chords(notes);
    let mut result = Vec::new();
    for (index, chord) in chords.iter().enumerate() {
        if chord.len() < 2 {
            result.extend(chord);
            continue;
        }
        let start = chord[0].start;
        let mut end = chord.iter().map(NoteEvent::end).max().unwrap_or(start + 1);
        if let Some(next) = chords.get(index + 1) {
            end = end.min(next[0].start);
        }
        // `step_by` takes every `rate`-th step; `zip` pairs each of those
        // steps with the next note of the order, and stops when the steps
        // run out (the order never does).
        for (step, pick) in (start..end).step_by(rate).zip(order.indices(chord.len())) {
            result.push(NoteEvent {
                start: step,
                length: rate.min(end - step),
                ..chord[pick]
            });
        }
    }
    result
}
```

El arpegio dura hasta que termina la nota más larga del acorde, pero se
detiene donde empieza el siguiente acorde o nota sola, así que nunca
suena encima. `map(NoteEvent::end)` pasa el método como función, como
`to_vec` más arriba; `max` devuelve un `Option`, porque una lista vacía
no tiene elemento mayor. `zip` se detiene cuando se acaba cualquiera de
los dos lados, y el orden, que no termina nunca, no se acaba, así que los
pasos deciden cuántas notas hay. Cada nota se construye `..chord[pick]`,
a partir de la nota del acorde que toca, así que conserva la velocidad de
esa nota: una nota de arriba fuerte sigue siendo fuerte en el arpegio.
La última nota se corta si el acorde termina en medio de ella
(`rate.min(end - step)`).

`result.extend(chord)` añade una nota sola tal como está. `chord` es un
`&Vec<NoteEvent>`, y un `Vec` de un tipo `Copy` se puede extender a
partir de referencias, copiando cada elemento.

## Oír lo que se movió

Después de una herramienta que cambia alturas, el piano roll toca el
primer acorde del resultado, para que oigas adónde fue sin poner en
marcha la canción:

```rust
if matches!(
    tool,
    Tool::Transpose(_) | Tool::InKey(_) | Tool::Invert { .. }
) && let Some(first) = tools::chords(&new).into_iter().next()
{
    actions.push(RollAction::Preview {
        channel: self.channel,
        pitches: first.iter().map(|note| note.pitch).collect(),
    });
}
```

`matches!` pregunta si un valor encaja en un patrón y da un `bool`; `_`
y `{ .. }` aceptan cualquier valor dentro. Es otra vez una cadena de
let: la escucha previa se envía solo para esas herramientas *y* si hay
un primer acorde. `Preview` es una acción más que la aplicación atiende
después de dibujar.

## Cómo se prueba

Como las herramientas son funciones simples, las pruebas también lo son:
unas notas entran, salen las notas esperadas, sin ventana. Una función
auxiliar crea una nota a partir de un número MIDI, un inicio y una
duración:

```rust
fn note(midi: u8, start: usize, length: usize) -> NoteEvent {
    NoteEvent::new(Note::new(midi), start, length)
}
```

y las pruebas se leen como la música:

```rust
#[test]
fn moving_in_key_changes_the_chord() {
    // C major one step up in C major is D minor: D, F, A.
    let c_major = [note(60, 0, 4), note(64, 0, 4), note(67, 0, 4)];
    let up = transpose_in_key(&c_major, C_MAJOR, 1).unwrap();
    assert_eq!(pitches(&up), [62, 65, 69]);
    // Two steps down from C, E, G: A, C, E (A minor).
    let down = transpose_in_key(&c_major, C_MAJOR, -2).unwrap();
    assert_eq!(pitches(&down), [57, 60, 64]);
    // ...
}
```

Las demás comprueban que una sola nota demasiado aguda estropea todo el
transporte, que un desplazamiento se queda dentro del patrón, que los
tresillos de corcheas se cuantizan a los pasos 0, 1 y 3, que ajustar
mueve solo las notas de fuera de la tonalidad (y que en un empate gana
la más grave), que invertir tres veces vuelve al estado fundamental, y
que un arpegio se detiene en el acorde siguiente y conserva la velocidad
de cada nota. Las pruebas de `change_key`, la función que usan los menús
**Tonalidad**, que está en el mismo archivo, están en [Escalas, modos y
tonalidades](scales.md#cómo-se-prueba).

Los ejercicios sobre las herramientas están en [Ejercicios: la teoría
musical en código](exercises.md).
