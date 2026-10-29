# Las herramientas son funciones puras

Archivos: [`crates/lineup-core/src/tools.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/tools.rs)
y [`crates/lineup-core/src/pattern.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/pattern.rs)

## Notas que entran, notas que salen

Cada herramienta del menú Herramientas del piano roll es una función de
`lineup_core::tools` con la misma forma: toma prestadas unas notas y
devuelve otras nuevas.

```rust
pub fn transpose(notes: &[NoteEvent], semitones: i32) -> Option<Vec<NoteEvent>>
pub fn shift(notes: &[NoteEvent], steps: isize, pattern_length: usize) -> Option<Vec<NoteEvent>>
pub fn quantize(notes: &[NoteEvent], pattern_length: usize) -> Vec<NoteEvent>
pub fn transpose_in_key(notes: &[NoteEvent], scale: Scale, steps: i32) -> Option<Vec<NoteEvent>>
pub fn snap_to_scale(notes: &[NoteEvent], scale: Scale) -> Vec<NoteEvent>
pub fn invert(notes: &[NoteEvent], up: bool) -> Option<Vec<NoteEvent>>
pub fn arpeggiate(notes: &[NoteEvent], order: ArpeggioOrder, rate: usize) -> Vec<NoteEvent>
```

Ninguna de ellas cambia nada. No ven el patrón, el proyecto, la
selección ni la ventana; no pueden, porque no se les da ninguno. Este
tipo de función se llama **función pura**: su resultado depende solo de
sus argumentos, y llamarla no tiene ningún otro efecto. Las funciones
puras son el código más fácil de probar que existe, porque una prueba
solo tiene que llamarlas y mirar lo que devuelven:

```rust
#[test]
fn moving_in_key_changes_the_chord() {
    // C major one step up in C major is D minor: D, F, A.
    let c_major = [note(60, 0, 4), note(64, 0, 4), note(67, 0, 4)];
    let up = transpose_in_key(&c_major, C_MAJOR, 1).unwrap();
    assert_eq!(pitches(&up), [62, 65, 69]);
}
```

Todas las decisiones sobre *qué* notas y *qué hacer después* se quedan
en el piano roll: le entrega a una herramienta las notas seleccionadas
(o todas), y convierte el resultado en un `RollAction`. La última
sección de este capítulo muestra cómo llega ese resultado al patrón.

Los argumentos son slices, `&[NoteEvent]`, no `&Vec<NoteEvent>`. Un
slice acepta un `Vec`, un array, o una parte de cualquiera de los dos
(gracias a la deref coercion), así que las pruebas pueden pasar un array
plano como `[note(60, 0, 4), ...]` mientras que el piano roll pasa su
`Vec`.

## Agrupar notas en acordes: `chords`

Invertir y arpegiar trabajan sobre acordes, no sobre notas sueltas, así
que la primera herramienta es una función auxiliar que encuentra los
acordes. En un piano roll, un acorde es simplemente las notas que
empiezan en el mismo step:

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

Tres pequeños detalles que conviene notar:

- `sort_by_key` ordena por una **tupla**. Las tuplas se comparan
  elemento a elemento, como las palabras en un diccionario: primero por
  `start`, y solo cuando dos inicios son iguales, por `pitch`. Así las
  notas quedan en orden temporal, y en cada acorde primero va la nota
  más grave.
- `chunk_by`, que el [carril de acordes del paso 3](../step-3/piano-roll-ui.md)
  ya usaba, corta un slice en tramos de vecinos para los que el closure
  dice «estos van juntos». Cada tramo es un `&[NoteEvent]`, un trozo
  prestado de `sorted`.
- `.map(<[NoteEvent]>::to_vec)` pasa un método como si fuera una
  función. `<[NoteEvent]>::to_vec` es el método `to_vec` de los slices
  de notas, escrito como una ruta. Es lo mismo que
  `.map(|chunk| chunk.to_vec())`, sin inventar un nombre para el tramo.
  Los corchetes angulares son necesarios porque `[NoteEvent]::to_vec`
  no compilaría.

## Cuando una herramienta no puede funcionar: recolectar `Option`s

Transponer un acorde una octava hacia arriba puede fallar: el rango MIDI
termina en la nota 127, y `Note::transpose` devuelve `None` para una
nota que lo dejaría atrás. ¿Qué debería pasar cuando una nota de un
acorde no puede moverse? Mover las demás cambiaría el acorde en
silencio. Las herramientas, en cambio, no hacen nada: devuelven `None`
para todo el grupo.

```rust
pub fn transpose(notes: &[NoteEvent], semitones: i32) -> Option<Vec<NoteEvent>> {
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

El closure devuelve un `Option<NoteEvent>` por cada nota. El `?` que hay
dentro devuelve `None` *desde el closure* (no desde `transpose`) cuando
`Note::transpose` falla. Así que `map` produce un iterador de `Option`s,
y `collect` los reúne.

El truco está en el tipo al que se recolecta. `Option<Vec<NoteEvent>>`
implementa `FromIterator<Option<NoteEvent>>`, lo que significa:
recolectar `Option`s en un `Option<Vec>` da `Some(vec)` si todos los
elementos eran `Some`, y `None` en cuanto un elemento es `None`. Incluso
se detiene en el primer `None`, sin mirar el resto. Rust sabe qué
`collect` usar por el tipo de retorno de la función, así que el código
no tiene que decirlo. (`Result` funciona igual: recolectar `Result`s en
un `Result<Vec<_>, E>` se detiene en el primer error.)

`..*note` es sintaxis de actualización de struct: «todos los demás
campos como en `*note`». Aquí `note` es un `&NoteEvent`, porque `iter()`
presta los elementos, así que se desreferencia con `*`. Eso solo
funciona porque `NoteEvent` es `Copy`: los campos se copian y el
original se queda donde estaba.

### Moverse en el tiempo: `checked_add_signed` y `then_some`

`shift` mueve las notas antes o después. Un inicio es un `usize` (nunca
negativo), pero el movimiento puede ser negativo, así que es un `isize`:

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

`usize::checked_add_signed` suma un número con signo a uno sin signo, y
devuelve `None` si el resultado fuera a quedar por debajo de cero (o a
pasarse del `usize` más grande). Escribir en su lugar
`(note.start as isize + steps) as usize` convertiría un -1 en un número
enorme sin decir nada.

`bool::then_some(value)` convierte una condición en un `Option`:
`Some(value)` si la condición es verdadera, `None` si no. Las dos líneas
se leen como la propia regla: el nuevo inicio debe existir y debe estar
dentro del patrón.

### De vuelta a la cuadrícula: `quantize`

Las notas tocadas a mano, o importadas de un archivo MIDI, pueden
empezar unos cuantos ticks fuera de la cuadrícula.
`NoteEvent::quantized` mueve tanto el inicio como el final al step más
cercano, y `quantize` lo aplica a un grupo entero, manteniendo el
resultado dentro del patrón:

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

El `..quantized` lleva la altura y la velocidad; `start` y `length` se
recalculan para que una nota situada justo al final no se salga del
patrón. Herramientas > Cuantizar a los pasos llama a esto sobre la
selección, o sobre todo el canal cuando no hay nada seleccionado.

## Mantenerse en la tonalidad

### Una nota de la tonalidad hacia arriba: `find_map`

«Subir una nota en la tonalidad» mueve cada nota a la siguiente nota por
encima de ella que pertenece a la tonalidad. En Do mayor, Mi va a Fa (un
semitono más arriba), pero Fa va a Sol (dos):

```rust
pub fn step_in_key(note: Note, scale: Scale, up: bool) -> Option<Note> {
    let direction = if up { 1 } else { -1 };
    (1..=12).find_map(|distance| {
        note.transpose(direction * distance)
            .filter(|candidate| scale.contains(candidate.pitch_class()))
    })
}
```

`find_map` prueba cada distancia por turnos y devuelve el primer `Some`
que da el closure. `Option::filter` conserva un `Some` solo si su valor
pasa la prueba, así que el closure dice «la nota que está a esta
distancia, si existe y está en la tonalidad». Toda escala tiene una nota
a 12 semitonos o menos (la octava de la propia nota, si no hay nada más
cerca), así que `None` solo vuelve en los bordes del rango MIDI.

`transpose_in_key` llama a `step_in_key` una vez por cada step del
movimiento, por cada nota, y recolecta los `Option`s igual que hace
`transpose`. Su efecto sobre los acordes es la parte interesante: cada
nota de Do, Mi, Sol se mueve a la siguiente nota de la tonalidad, así que el
acorde pasa a ser Re, Fa, La, que es Re *menor*. Mover un acorde dentro de
la tonalidad lo mantiene en ella, de modo que su tipo cambia de grado en
grado, exactamente como los siete acordes de la tonalidad en la fila
«Acordes en Do mayor».

### La nota más cercana de la tonalidad: el ajuste

El ajuste mueve una nota que está fuera de la tonalidad a la nota más
cercana dentro de ella. En Do mayor, Do♯ está a un semitono de Do y a uno
de Re: ¿cuál gana? No hay una respuesta correcta, así que el código toma
una decisión y lo dice: la nota más grave.

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

Dos `find_map`s, uno dentro del otro: el exterior prueba las distancias
1, 2, 3..., y para cada distancia el interior prueba abajo y luego
arriba. Como `-distance` va primero en el array, la nota más grave gana
el empate. La regla está en el *orden del array*, y es fácil pasarla por
alto, así que el comentario de documentación y una prueba la dicen en
voz alta:

```rust
// C stays. C#, F# and Bb are each one semitone from two notes of
// the key, so the lower one wins: C, F and A.
assert_eq!(pitches(&snap_to_scale(&notes, C_MAJOR)), [60, 60, 65, 69]);
```

`snap_to_scale` nunca falla, así que devuelve un `Vec` plano: una nota
que no puede moverse (no hay ninguna nota de la escala cerca, dentro del
rango MIDI) se queda donde está.

## Inversiones

Un acorde se **invierte** moviendo su nota más grave una octava hacia
arriba, por encima de las demás. Do, Mi, Sol pasa a ser Mi, Sol, Do (la primera
inversión), y hacerlo otra vez da Sol, Do, Mi (la segunda). Las notas, y por
tanto el acorde, siguen siendo las mismas; lo que cambia es la nota del
bajo, y eso cambia mucho el color del acorde. El carril de acordes lo
escribe con una barra: Mi, Sol, Do es «C/E», Do mayor sobre Mi.

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

- `times % notes.len()`: un acorde de tres notas invertido tres veces
  vuelve a donde empezó, una octava más arriba. En lugar de subir
  octavas, la función invierte `times` módulo el número de notas. El
  `.max(1)` está ahí porque `% 0` hace panic en Rust, y un acorde vacío
  no debe hacer caer nada: `0..0` simplemente no hace ningún bucle.
- `let (Some(&lowest), Some(&highest)) = (...) else { break; }` pone dos
  `Option`s en una tupla y hace match de ambos a la vez: «si hay una
  primera y una última nota». El `&` del patrón copia cada `Note` fuera
  de la referencia que devuelve `first()`.
- `above(lowest, highest)` sube una nota octava a octava hasta que queda
  por encima de `highest`. Para Do, Mi, Sol, el Do sube una octava, por
  encima del Sol. Para Do, Mi, Sol, Do' (con la octava arriba del todo), tiene
  que subir dos octavas para quedar por encima de Do'.

`invert`, la herramienta del menú, usa `chords` para encontrar todos los
acordes en las notas que recibe e invierte cada uno una vez, hacia
arriba o hacia abajo. Hacia abajo es la imagen especular: la nota más
aguda va por debajo de la más grave. Tras mover una nota vuelve a
ordenar el acorde por altura, porque la nota movida está ahora en el
extremo equivocado de la lista: la primera prueba de esta función
detectó exactamente eso, con la nota movida todavía en primer lugar.

## Arpegios: `cycle`, `step_by` y `zip`

Un **arpegio** toca las notas de un acorde una detrás de otra en lugar
de a la vez. «Arriba» toca Do, Mi, Sol, Do, Mi, Sol..., «abajo» Sol, Mi, Do..., y
«arriba y abajo» Do, Mi, Sol, Mi, Do, Mi, Sol, Mi...: las notas superior e
inferior no se tocan dos veces seguidas.

El orden es un enum, con un método que dice qué nota viene después,
contando desde la más grave:

```rust
fn indices(self, count: usize) -> impl Iterator<Item = usize> {
    let once: Vec<usize> = match self {
        ArpeggioOrder::Up => (0..count).collect(),
        ArpeggioOrder::Down => (0..count).rev().collect(),
        ArpeggioOrder::UpDown => (0..count)
            .chain((1..count.saturating_sub(1)).rev())
            .collect(),
    };
    once.into_iter().cycle()
}
```

Para tres notas, «arriba y abajo» es `0, 1, 2` encadenado con `1` (el
rango `1..2` invertido): `0, 1, 2, 1`. Después `cycle` repite eso **para
siempre**: `0, 1, 2, 1, 0, 1, 2, 1, 0...`. Un iterador sin fin suena
peligroso, pero los iteradores son perezosos: no se calcula nada hasta
que alguien pide el siguiente elemento, así que «para siempre» no cuesta
nada mientras quien llama deje de pedir.

El tipo de retorno `impl Iterator<Item = usize>` significa «un iterador
de `usize`s; cuál exactamente es asunto mío». El tipo real es
`Cycle<vec::IntoIter<usize>>`, que quien llama no necesita saber.

El propio arpegio llena el tiempo que duró el acorde:

```rust
let start = chord[0].start;
let mut end = chord.iter().map(NoteEvent::end).max().unwrap_or(start + 1);
if let Some(next) = chords.get(index + 1) {
    end = end.min(next[0].start);
}
for (step, pick) in (start..end).step_by(rate).zip(order.indices(chord.len())) {
    result.push(NoteEvent {
        start: step,
        length: rate.min(end - step),
        ..chord[pick]
    });
}
```

- El arpegio dura tanto como la nota más larga del acorde, pero se
  detiene donde empieza el acorde (o la nota) siguiente, de modo que
  nunca choca con él.
- `(start..end).step_by(rate)` da cada step número `rate`: con un acorde
  de 0 a 8 y un rate de 2, los steps 0, 2, 4 y 6.
- `zip` empareja esos steps con el orden de notas sin fin: `(0, C)`,
  `(2, E)`, `(4, G)`, `(6, C)`. Un zip se detiene en cuanto *cualquiera*
  de los dos lados se agota, y los steps siempre se agotan primero. Eso
  es lo que hace seguro al `cycle` sin fin.
- `rate.min(end - step)` acorta la última nota si el acorde termina en
  mitad de ella.
- `..chord[pick]` copia todo lo demás de la nota elegida, así que cada
  nota del arpegio conserva la velocidad que tenía en el acorde.

El resultado es una lista plana de notas. Las notas sueltas de la
entrada pasan sin tocar, así que arpegiar un canal entero solo cambia
sus acordes.

## De las notas nuevas al patrón: `replace_notes`

El resultado de una herramienta tiene que sustituir a las notas que
recibió. Un patrón mantiene una regla sobre sus notas desde el paso 3:
no hay dos notas de un canal que empiecen en el mismo momento con la
misma altura (sonarían como una sola, más fuerte, y nunca podrían
distinguirse). Desde el paso 8, «el mismo momento» incluye el `nudge`
de la nota dentro de su step, así que se permiten dos notas separadas
por unos pocos ticks. `same_place` compara la altura, el inicio y el
nudge. Sustituir un grupo debe mantener esa regla, y hacerlo todo de una
vez, así que `Pattern` tiene un método exactamente para eso:

```rust
pub fn replace_notes(&mut self, channel: usize, old: &[NoteEvent], new: &[NoteEvent]) -> bool
pub fn can_replace_notes(&self, channel: usize, old: &[NoteEvent], new: &[NoteEvent]) -> bool
```

Ambos se construyen sobre un método privado que calcula el resultado sin
cambiar nada:

```rust
fn replaced(&self, channel: usize, old: &[NoteEvent], new: &[NoteEvent]) -> Option<Vec<NoteEvent>> {
    let row = self.rows.get(channel)?;
    if !old.iter().all(|note| row.contains(note)) {
        return None;
    }
    let mut notes: Vec<NoteEvent> = row
        .iter()
        .filter(|note| !old.contains(note))
        .copied()
        .collect();
    let new = tidy_notes(new.iter().copied());
    let lands_on_another = new
        .iter()
        .any(|note| notes.iter().any(|other| other.same_place(note)));
    if lands_on_another {
        return None;
    }
    notes.extend(new);
    Some(notes)
}
```

Se niega (devuelve `None`) en dos casos: cuando una de las notas `old`
ya no está en el canal, y cuando una nota nueva caería sobre una nota
que se queda. `replace_notes` entonces guarda el resultado entero o no
cambia nada; no hay sustituciones a medias. `can_replace_notes` hace la
misma pregunta sin cambiar nada, para que el piano roll pueda comprobar
antes, y simplemente no mover una selección encima de otras notas.

`tidy_notes` limpia las notas nuevas antes de que entren: cada nota se
ajusta a los steps permitidos (se acorta si se pasa de `MAX_STEPS`, 256
steps, y se descarta si empieza más allá; `nudge` y `trim` se mantienen
por debajo de un step), y de varias notas nuevas en el mismo lugar solo
se conserva la primera. «El mismo lugar» es `same_place`: misma altura,
mismo inicio y mismo `nudge`, así que dos notas separadas por unos pocos
ticks sobreviven ambas. El caso duplicado es fácil de alcanzar: ajusta Do
y Do♯ tocadas juntas a Do mayor, y ambas se convierten en Do. En lugar de
hacer que cada herramienta sea cuidadosa, el patrón limpia lo que se le
dé.

¿Por qué `old` guarda notas y no posiciones en la lista? Porque la
selección del piano roll también guarda notas, por las razones que
explica el [capítulo del piano roll](piano-roll.md). Encontrar una nota
por valor en una lista de unos cientos es lo bastante rápido como para
hacerlo muchas veces por frame.
