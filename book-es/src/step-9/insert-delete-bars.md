# Insertar y borrar compases

Archivos: [`crates/lineup-core/src/bars.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/bars.rs),
[`crates/lineup-core/src/project.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/project.rs),
[`crates/lineup-app/src/chord_sheet.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/chord_sheet.rs),
[`crates/lineup-app/src/playlist/sections.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/playlist/sections.rs)
y [`crates/lineup-app/src/app/edit.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/edit.rs)

Un chart que estás escribiendo rara vez sale bien a la primera. El
puente necesita un compás más antes del turnaround; una transcripción
tiene un compás de más donde se copiaron dos veces dos líneas de la
partitura. Hasta ahora la única forma de arreglarlo era mover a mano
cada clip, marcador y sección posteriores al lugar, y confiar en que las
partes de la banda y los acordes siguieran alineados.

Ahora un compás se puede insertar o borrar con un clic, y toda la
canción que viene después se mueve con él:

```text
antes:   | C    | F    | G7   | C    |
insertar un compás después del compás 2:
después: | C    | F    | %    | G7   | C    |
```

Haz clic derecho en un compás del **chord sheet**, o en un número de
compás de la **regla de la playlist**, y el menú tiene:

- **Insert a bar before**
- **Insert a bar after**
- **Delete this bar**, o **Delete bars 5–8** cuando hay varios compases
  seleccionados en el sheet.

Cada uno es un único paso para deshacer ("Undo insert bar").

![El menú sobre el compás 3 de Embraceable You: escribir en él, vaciarlo, mover allí el cabezal de reproducción, insertar un compás antes o después, o borrarlo.](../images/step-9-insert-bar-menu.png)

## Pruébalo

1. Abre **File > New**, que da un chord sheet vacío con una banda, y
   teclea `C`, `F`, `G7` y `C` en los cuatro primeros compases.
2. Haz clic derecho en el Fa y elige **Insert a bar after**. Aparece un
   compás 3 nuevo, con el Fa escrito tenue dentro: el Fa continúa a través
   del compás vacío, como hace `%` en un chart impreso. El G7 ahora está
   en el compás 4.
3. Teclea `D7` en el compás nuevo. La banda toca C, F, D7, G7, C.
4. Haz clic derecho en el D7 y elige **Delete this bar**. La canción
   queda como estaba.
5. En la playlist, haz clic derecho en el número de compás **2** de la
   regla y elige **Insert a bar before**. Los clips, las secciones y los
   marcadores posteriores al compás 1 se mueven todos un compás más
   tarde.

![Después de Insert a bar after en el compás 3 de Embraceable You: el nuevo compás 4 está vacío, así que el Am7 continúa a través de él, escrito tenue, y el D7 se ha movido al compás 5. El patrón de la sección A también se toca en otro sitio de la canción, así que su clip se ha cortado en dos, A (1) y A (2), y su batería con él.](../images/step-9-insert-bar-after.png)

## Qué es un compás insertado

Un compás nuevo está **vacío**, y vacío significa "aquí no cambia nada":
la armonía que venía sonando sigue a través de él, y lo mismo la
tonalidad, el tempo y la variación de la banda. Eso es también lo que
significa un compás vacío en un chart, y por eso el menú no pregunta qué
poner en él.

Tiene el **compás** del compás junto al que se inserta, y la **sección**
de ese compás. Un compás insertado antes del primer compás de una
sección de vals es un compás de vals, aunque el compás anterior esté en
4/4: estabas mirando el compás de vals cuando pediste uno más como él.

**Todo lo que hay después se mueve un compás más tarde**: clips,
secciones, el loop, los marcadores de compás, tempo, tonalidad y
acordes, y las variaciones de la banda y los estilos de sección.

## Qué se lleva un compás borrado

Un compás borrado se lleva sus notas y sus acordes, y todo lo que hay
después se mueve un compás antes. La pregunta es qué pasa con **lo que
estaba sonando al final**. Toma C, F, G7, C con la tonalidad cambiando a
Sol mayor en el compás 3 y el tempo a 90 en el compás 4, y borra el compás
3. Los marcadores del compás 3 se van con él, pero la tonalidad de Sol
mayor debía continuar después, y debe seguir haciéndolo. Así que, antes
de mover nada, `close_bars` lee lo que estaba sonando al final de los
compases borrados:

```rust
// What goes on at the end of the bars goes on from their start.
let time = meter.time_at(end);
let tempo = self.tempo_map().tempo_at(end);
let key = self.key_at(end);
```

y después vuelve a poner cada uno donde estaban los compases. Los
marcadores que dicen lo mismo que el anterior se limpian, así que borrar
en mitad de un tramo largo en una tonalidad no deja un marcador de
tonalidad detrás.

La armonía es la sutil. Si un G7 empieza en un compás borrado y se
mantiene hasta el siguiente (el compás siguiente no tiene acorde
propio), el G7 debe continuar allí. Pero si el compás anterior al
borrado ya es un G7, escribir otro G7 sería un segundo marcador diciendo
lo mismo. El código comprueba ambas cosas:

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

Esto es una **let chain**: un `if let` y condiciones normales unidos con
`&&`, estable desde Rust 1.88 (con la edición 2024). Cada parte puede
usar lo que ha ligado un `let` anterior: `marker` se usa en la última
línea de la condición. Antes de las let chains, esto necesitaba un
`if let` con un `if` anidado dentro.

`start.checked_sub(1)` es `None` para el primer compás, donde no hay
"antes"; `and_then` deja pasar el `None` y si no busca el acorde un step
antes; e `is_none_or` dice que sí cuando no hay acorde antes, o cuando
el que hay difiere.

## Mover cada step: un closure

Insertar usa `Project::make_room(at, steps)`, que la canción ya tenía
para copiar una sección y para poner una intro delante: cada clip,
sección y marcador en `at` o después se mueve `steps` más tarde. Borrar
necesita lo contrario, y un caso más: lo que estaba **dentro** de los
steps borrados. Un closure dice adónde va cualquier step de la canción:

```rust
// Where a step of the song goes: the deleted bars close up.
let place = |step: usize| match step {
    step if step <= start => step,
    step if step >= end => step - steps,
    _ => start,
};
```

Los steps anteriores a los compases borrados se quedan; los posteriores
se mueven `steps` antes; los de dentro aterrizan donde estaban los
compases. Luego se aplica a todo lo que tiene un lugar en la canción. El
final de una sección también pasa por `place`, así que una sección que
contenía los compases se acorta, y una hecha solo de ellos queda con
longitud 0 y se elimina. El loop se conserva solo si queda algo de él:

```rust
playlist.loop_region = playlist.loop_region.and_then(|region| {
    let (from, to) = (place(region.start), place(region.end));
    (from < to).then_some(crate::LoopRegion::new(from, to))
});
```

`bool::then_some` convierte `true` en `Some(value)` y `false` en `None`:
aquí, "el loop si todavía tiene longitud".

## Las notas: abrir el patrón, o cortar el clip

Los clips son la parte difícil. Un clip reproduce un **patrón**, y las
notas están en el patrón, no en el clip. Insertar un compás en mitad de
un clip tiene que hacer sitio en las notas que reproduce. Dos casos:

- **El patrón lo reproduce solo este clip, una vez.** Entonces el propio
  patrón puede cambiar: recibe los steps vacíos (`opened`) o pierde los
  borrados (`closed`), y el clip simplemente reproduce el patrón más
  largo o más corto.
- **El patrón también se reproduce en otro sitio**, por otro clip o por
  este repetido. Entonces el patrón no debe cambiar, o los otros sitios
  cambiarían con él. El clip se **corta en dos** donde el compás entra o
  sale, y cada mitad reproduce un patrón propio: "Riff (1)" y
  "Riff (2)".

![La playlist después de insertar: A (1) toca los compases 1 a 3, el compás 4 está vacío, y A (2) toca el resto del viejo A desde el compás 5, con la batería cortada igual.](../images/step-9-insert-bar-playlist.png)

La comprobación es `only_clip`:

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

`enumerate` empareja cada clip con su índice, para que `all` pueda
saltarse el propio clip (`other == index`) y preguntar por todos los
demás.

Cortar un clip que está **repetido** es más suave. Un corte que cae
entre dos repeticiones no necesita ningún patrón nuevo: un clip que
reproduce cuatro veces un groove de un compás, con su segundo compás
borrado, se convierte en un clip que lo reproduce una vez y un clip que
lo reproduce dos. Solo un corte en mitad de una repetición necesita las
dos mitades de esa repetición como patrones.

Las notas mantenidas a través del lugar se **cortan** allí, y las notas
posteriores se mueven:

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

Una nota mantenida sobre el compás nuevo se corta donde empieza el
compás, en lugar de mantenerse a través de él, porque el compás nuevo
debe quedar vacío: escribirás algo allí, y que suene debajo una nota de
antes sería una sorpresa. `NoteEvent { start: …, ..*note }` es
**sintaxis de actualización de struct**: una copia de `*note` con
`start` cambiado. Funciona con `*note` porque `NoteEvent` es `Copy`.

Las mitades de un patrón cortado conservan su **color**, para que los
clips se vean como se veían en la playlist, y una mitad cortada solo
para ser borrada se elimina de nuevo al final, de modo que un borrado no
deja patrones vacíos detrás.

## Todo o nada

Dos cosas pueden hacer imposible un insert o un delete: la canción
crecería más allá de su longitud máxima, o no hay sitio para los
patrones que necesita un corte (una canción admite como mucho 999).
Cualquiera de las dos se descubre a mitad de camino, cuando algunos
clips ya han cambiado. Así que ambas empiezan guardando una copia del
proyecto:

```rust
let before = self.clone();
if !self.open_clips(at, steps) || !self.make_room(at, steps) {
    *self = before;
    return false;
}
```

y la devuelven si algo falla. `*self = before` reemplaza el proyecto
entero, moviendo `before` dentro de él. La app dice entonces "There is
no room for another bar in the song" o "There is no room for the
patterns the cut needs", y nada ha cambiado.

`||` se detiene en el primer `true`, así que `make_room` se ejecuta solo
si `open_clips` tuvo éxito. `make_room` comprueba antes de mover nada,
pero `open_clips` puede fallar en el tercer clip después de cortar los
dos primeros; la copia cubre eso.

## La banda sigue

Las partes de la banda (piano, bajo, batería) se escriben a partir del
chord sheet. Mover todo un compás mueve también sus notas, y la app no
debe confundir eso con una edición tuya: una parte que editas queda
**congelada**, conservada tal como la dejaste, y deja de seguir el
sheet. Así que ambos métodos se ejecutan dentro de `moving_the_band`,
que también usan las herramientas que transponen o cambian el compás de
toda la canción:

```rust
pub fn insert_bars(&mut self, bar: usize, after: bool, count: usize) -> bool {
    self.moving_the_band(|project| project.open_bars(bar, after, count))
}
```

`moving_the_band` recibe el closure como `impl FnOnce(&mut Project) ->
R` y lo ejecuta sobre el proyecto, y luego adopta las partes movidas
como propias de la banda. En la app, la edición pasa por el mismo método
`edit` que cualquier otro cambio: guarda el proyecto como estaba para
deshacer, ejecuta el cambio, deja que la banda toque el sheet nuevo y
dice al hilo de audio qué ha cambiado.

## El menú, en dos sitios

Ambos menús son la misma función, así que siempre ofrecen los mismos
elementos con los mismos atajos:

```rust
/// The menu items that insert a bar next to bar `bar` or delete it, or
/// delete the bars of `selection` when it holds more than one.
pub fn bar_edits(ui: &mut Ui, bar: usize, selection: Option<(usize, usize)>) -> Option<BarEdit> {
```

Dibuja los botones y devuelve lo que se hizo clic, si acaso, como un
valor:

```rust
pub enum BarEdit {
    /// An empty bar next to bar `bar`, after it or before it.
    Insert { bar: usize, after: bool },
    /// Bars `first` to `last` taken out.
    Delete { first: usize, last: usize },
}
```

El chord sheet lo envuelve en su propia action, `SheetAction::Bars`, y
la playlist en `PlaylistAction::Bars`. Ninguno cambia el proyecto
mientras dibuja; la app aplica la acción después con `edit_bars`, que
además selecciona el compás nuevo en el sheet para que puedas teclear en
él de inmediato. La mayoría de las vistas de la app funcionan así:
dibujar, recoger acciones, aplicarlas después.

## Las pruebas

Las pruebas de `bars.rs` trabajan sobre una canción pequeña: un patrón
de cuatro compases que toca C, F, G7, C, una estrofa sobre todo él, un
loop sobre los compases 2 y 3, la tonalidad de Sol desde el compás 3 y 90
BPM desde el compás 4.

- `an_inserted_bar_moves_everything_after_it` inserta después del compás
  2 y comprueba cada nota, acorde, tonalidad, tempo, la estrofa y el
  loop: la nota mantenida hasta el compás nuevo ahora termina donde este
  empieza.
- `a_bar_inserted_before_a_change_of_time_takes_the_new_time` hace del
  compás 3 un compás de vals e inserta antes de él: el compás nuevo
  tiene doce steps.
- `a_clip_of_a_pattern_played_elsewhere_too_is_cut_in_two` comprueba
  "Riff (1)" y "Riff (2)", y que el "Riff" del otro clip no se ha
  tocado.
- `a_deleted_bar_closes_up`, `a_deleted_bar_leaves_no_pattern_behind`,
  `a_harmony_from_a_deleted_bar_goes_on_where_it_was` y
  `a_repeated_clip_loses_the_repeats_in_the_deleted_bars` hacen lo mismo
  para borrar.
- `the_band_follows_the_bars` inserta y borra en una canción con banda y
  comprueba que ninguna parte quedó congelada.

En la app, `a_bars_menu_inserts_a_bar_and_deletes_it` hace clic derecho
en el G de C, G, Am, F, inserta un compás después, teclea D7 dentro y lo
borra otra vez; `the_bar_numbers_insert_and_delete_bars_too` lo hace
desde la regla de la playlist y lo deshace con Ctrl+Z.

Las pruebas comprueban los datos de la canción. Para oír que nada más se
movió, la [traza de reproducción](playback-trace.md) de una demo antes y
después de un insert muestra cada nota anterior al compás en el mismo
instante, y cada nota posterior exactamente un compás después.

## Ejercicios

1. `insert_bars` recibe un `count`, pero el menú siempre inserta un
   compás. Añade **Insert bars…** con un número. ¿Qué parte de
   `open_bars` ya maneja más de uno?
2. Escribe una prueba que inserte un compás en mitad de la segunda
   repetición de un clip repetido tres veces. Antes de ejecutarla,
   apunta los clips que esperas, con sus inicios, repeticiones y
   patrones.
3. `close_bars` lee el compás, el tempo y la tonalidad al final de los
   compases borrados **antes** de mover nada. ¿Qué saldría mal si los
   leyera después?
