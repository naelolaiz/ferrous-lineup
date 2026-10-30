# Decirle al hilo de audio lo que ha cambiado

Archivos: [`crates/lineup-core/src/diff.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/diff.rs),
[`crates/lineup-app/src/app/edit.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/edit.rs),
[`crates/lineup-audio/src/engine.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-audio/src/engine.rs)
y [`crates/lineup-audio/src/sample.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-audio/src/sample.rs)

## Una sola entrada para cada cambio

El hilo de audio guarda su propia copia de todo lo que reproduce: el
instrumento, el volumen, el paneo y las notas de cada canal, las
longitudes de los patrones, la canción (ver [Dos
hilos](threads.md)). La ventana debe mantener esa copia al
día, y un cambio puede tocar cualquier cosa. Un clic en un paso cambia
una lista de notas. Borrar un patrón renumera todos los clips que vienen
después. Escribir un acorde hace que la banda escriba patrones nuevos.
Deshacer sustituye todo el proyecto por uno más antiguo, que puede
diferir en cualquier cosa.

Hay tres formas de contarle un cambio al hilo de audio:

1. **Enviar el proyecto entero otra vez**: todos los patrones y canales,
   como al abrir un archivo. Sencillo, pero cada instrumento se
   construiría de nuevo, cortando toda nota que esté sonando, y un
   archivo de muestra se leería otra vez por un deshacer de un solo
   paso.
2. **Escribir, para cada tipo de edición, los comandos que necesita**,
   y para deshacer, los comandos que la invierten. Es un par escrito a
   mano para cada edición, y cada par es una ocasión de equivocarse al
   invertir (el [capítulo siguiente](undo.md) cuenta más
   sobre esta elección).
3. **Comparar el proyecto antes y después**, y enviar solo lo que
   difiere.

Ferrous Lineup hace lo tercero, para *todos* los cambios, no solo para
deshacer. Ninguna edición envía sus propios comandos. Todo cambio pasa
por un único método, `edit`:

```rust
pub(super) fn edit<R>(
    &mut self,
    label: &'static str,
    change: impl FnOnce(&mut Project) -> R,
) -> R {
    if self.history.is_open() {
        // A drag or a typing session is under way, and its first change
        // made the undo step. Keep changing the project without copying
        // the whole of it every frame; the audio catches up steadily
        // from the last state it was given.
        let (patterns, channels) =
            (self.project.patterns().len(), self.project.channels().len());
        let result = change(&mut self.project);
        self.project.follow_chord_sheet();
        self.file.project_edited();
        self.recovery.project_changed();
        self.step_unsynced = true;
        if self.project.patterns().len() != patterns
            || self.project.channels().len() != channels
        {
            // A pattern or channel came or went: the views and the
            // audio thread's rows have to move now, not in 50 ms.
            self.sync_step();
        } else {
            self.sync_step_if_due();
        }
        return result;
    }
    // A step left open by a direct history call: send it before the
    // audio's state is assumed to be this project.
    self.finish_step_sync();
    let before = self.project.clone();
    let result = change(&mut self.project);
    // The band plays what the chord sheet says now, in the same step.
    self.project.follow_chord_sheet();
    if self.project != before {
        self.sync_audio(&before);
        self.synced = self.project.clone();
        self.step_unsynced = false;
        self.last_sync = Instant::now();
        self.file.project_reverted();
        self.recovery.project_changed();
        // `before` is moved into the history here, after its last use.
        self.history.record(label, before);
    }
    result
}
```

Lee primero la segunda mitad, el caso normal. `edit` guarda una copia
del proyecto, deja que el closure lo cambie y, si algo cambió, hace
cuatro cosas en un solo sitio:

- `sync_audio(&before)` le dice al hilo de audio qué difiere. El resto
  de este capítulo trata de eso.
- `follow_chord_sheet` deja que la banda vuelva a escribir sus patrones
  si cambiaron la hoja de acordes, la tonalidad o los compases, como
  parte del mismo paso (ver [Cómo toca la banda una hoja de
  acordes](../band/band.md)).
- `file` y `recovery` se enteran de que el proyecto cambió: la barra de
  título recibe su asterisco de cambios sin guardar, y se escribirá una
  copia de recuperación (ver [Guardar](saving.md)).
  `project_reverted` vuelve a calcular el asterisco desde cero, porque
  un cambio hecho de una vez puede dejar exactamente lo que estaba
  guardado, como un paso que se activa y se vuelve a desactivar.
- `history.record` guarda `before` como el paso de deshacer llamado
  `label` (ver [Deshacer y rehacer](undo.md)).

Tener un único camino compensa dos veces. Solo hay un fragmento de
código que convierte cambios del proyecto en comandos, así que se puede
probar a fondo por separado. Y se usa en cada clic, así que si tuviera
un error lo oirías mientras editas, no solo en el raro momento en que
deshaces algo inusual.

### Un arrastre es un paso, no uno por fotograma

La primera mitad, la rama `is_open`, es el detalle de rendimiento
importante. Un arrastre cambia notas en cada fotograma, sesenta veces
por segundo, pero solo su primer fotograma necesita la copia para
deshacer y la comparación completa. Mientras el paso está abierto, el
proyecto se cambia directamente, y `synced` guarda el último estado que
recibió el hilo de audio. `sync_step_if_due` compara el proyecto con ese
estado y envía las diferencias como mucho cada 50 ms:

```rust
/// How often, at most, a drag's changes reach the audio thread. The
/// window still draws every frame; only the sound waits this long, and
/// the whole project is copied only when this comes round.
const SYNC_INTERVAL: Duration = Duration::from_millis(50);

fn sync_step_if_due(&mut self) {
    if self.last_sync.elapsed() >= SYNC_INTERVAL {
        self.sync_step();
    }
}

fn sync_step(&mut self) {
    let before = std::mem::replace(&mut self.synced, Project::empty());
    self.sync_audio(&before);
    self.synced = self.project.clone();
    self.step_unsynced = false;
    self.last_sync = Instant::now();
}
```

`std::mem::replace` saca el `synced` antiguo de la estructura y deja un
proyecto vacío en su lugar, así que `sync_audio` puede tomarlo prestado
como `before` sin copiarlo (no podría tomar prestado `self.synced`
mientras también toma prestado `self` de forma mutable). Cuando un
patrón o un canal aparece o desaparece en mitad de un paso, esperar no
basta: las vistas cuentan filas que el hilo de audio aún no tiene, así
que `sync_step` se ejecuta enseguida.

Al soltar el ratón, `finish_step_sync` envía los últimos cambios que
faltan, y el paso se cierra. Un arrastre es un paso de deshacer, una
copia y un puñado de mensajes.

Deshacer y rehacer sustituyen el proyecto entero, así que siempre hacen
la comparación completa:

```rust
pub(super) fn undo(&mut self) {
    self.finish_step_sync();
    let before = self.project.clone();
    if self.history.undo(&mut self.project).is_some() {
        self.sync_audio(&before);
        self.synced = self.project.clone();
        self.step_unsynced = false;
        self.last_sync = Instant::now();
        self.file.project_reverted();
        self.recovery.project_changed();
        self.restore_song_library();
        self.keep_place_in_song(&before);
    }
}
```

Las dos últimas líneas son para la ventana, no para el sonido: la
biblioteca muestra el estilo y el ritmo que toca esta versión del
proyecto, y si deshacer cambió el compás, la reproducción sigue en el
mismo compás de la canción.

### Leer `edit`

`edit` recibe un **closure** que cambia el proyecto, y el closure recibe
el proyecto como argumento:

```rust
self.edit("move clip", |project| project.move_clip(index, track, start));
```

¿Por qué no dejar que el closure alcance `self.project` por sí mismo?
Porque `edit` ya tiene `&mut self`, y un closure que tomara prestado
`self` también sería un segundo préstamo del mismo valor mientras el
primero sigue en uso. El borrow checker lo rechaza. Entregarle al closure
`&mut self.project` como argumento le presta exactamente la parte que
necesita, durante exactamente lo que dura la llamada.

El tipo del closure es `impl FnOnce(&mut Project) -> R`. **`FnOnce`**
es el trait de los closures que se llaman exactamente una vez: deja que
el closure mueva un valor al proyecto, como un `channel` nuevo, porque
nadie lo llamará una segunda vez. (Los closures `Fn` se pueden llamar
muchas veces y solo leen lo que capturan; los `FnMut` pueden cambiarlo.)

`R` es lo que devuelva el closure, y `edit` se lo devuelve a quien lo
llama. Muchos métodos de `Project` devuelven algo: `add_clip` devuelve
el índice del clip nuevo como `Option<usize>`, `remove_pattern` el
patrón que quitó, `insert_bars` si había sitio. Como `R` puede ser
cualquier tipo, un closure puede ser simplemente la llamada al método,
`|project| project.add_clip(clip)`, y quien llama recibe la respuesta.
Si `edit` pidiera `impl FnOnce(&mut Project)`, que debe devolver `()`,
cada una de esas necesitaría llaves y un punto y coma para tirar el
valor.

`edit_pattern` es un pequeño envoltorio para el caso más común, un
cambio en el patrón seleccionado:

```rust
fn edit_pattern(&mut self, label: &'static str, change: impl FnOnce(&mut Pattern)) {
    let index = self.pattern;
    self.edit(label, |project| {
        if let Some(pattern) = project.pattern_mut(index) {
            change(pattern);
        }
    });
}
```

`self.project != before` compara los dos proyectos por completo: el
`PartialEq` derivado compara cada campo, cada lista, cada nota. Si nada
cambió, como un clip arrastrado de vuelta a la misma casilla o una nota
borrada donde no había ninguna, no se envía nada y no se registra
ningún paso de deshacer.

## `diff`: la lista de cambios

La comparación vive en `lineup-core`, que conoce los proyectos pero nada
del hilo de audio. Describe las diferencias con las propias palabras del
proyecto:

```rust
pub enum Change {
    RemovePattern(usize),
    InsertPattern(usize),
    RemoveChannel(usize),
    InsertChannel(usize),
    Instrument(usize),
    Volume(usize),
    Muted(usize),
    Pan(usize),
    ReverbSend(usize),
    Effects(usize),
    PatternLength(usize),
    Notes { pattern: usize, channel: usize },
    Tempo,
    LooseTiming,
    Mixer,
    Song,
}

pub fn diff(before: &Project, after: &Project) -> Vec<Change>
```

Los cambios son pasos que hay que seguir **en orden**, y cada número es
una posición en el momento en que se aplica su cambio. Después de
`RemovePattern(0)`, el patrón que solía ser el número 1 es el número 0,
y cualquier cambio posterior lo llama 0. Los valores (un volumen, una
lista de notas) no están en el `Change`: se leen del proyecto más nuevo
cuando el cambio se convierte en un comando. Eso mantiene `Change`
pequeño y `Copy`.

Solo se compara lo que produce sonido. Los nombres de patrón, de canal,
de pista y de sección y la tonalidad no dan ningún cambio, porque el
hilo de audio nunca los ve. Silenciar una pista sí da uno: saca los
clips de esa pista de la canción, así que los clips que reproduce el
hilo de audio cambian.

El mezclador también se oye. Mover un canal entre los altavoces da
`Pan`, cambiar cuánto va de él a la reverberación da `ReverbSend`,
cambiar su ecualizador, su compresor o su delay da `Effects`, y cambiar
el volumen maestro o la propia reverberación da `Mixer`. Poner un canal
en Solo es un cambio de mezclador *y* cambia qué canales se oyen, así
que `diff` también informa de `Muted` para cada canal cuyo silencio
efectivo cambió. La Soltura da `LooseTiming`: decide cuánto puede
alejarse de la cuadrícula el comienzo de una nota.

### Alinear dos listas

La parte difícil son las listas. Digamos que los patrones eran
`[Beat, Chords, Fill]` y ahora son `[Beat, Fill]`. Comparar elemento a
elemento diría que el patrón 1 cambió de Chords a Fill y que el patrón 2
desapareció, lo cual es cierto pero torpe: enviaría otra vez todas las
notas de Fill. Lo que pasó de verdad es que «se quitó el patrón 1».

`Alignment` lo encuentra. Cuenta cuántos elementos son iguales al
principio de ambas listas, y cuántos al final. Lo que queda en medio se
quitó de la lista antigua y se insertó en la nueva:

```text
  antes:    Beat │ Chords │ Fill       start = 1 (Beat es el mismo)
  después:  Beat │        │ Fill       end   = 1 (Fill es el mismo)
                                       eliminados = 3 - 1 - 1 = 1
                                       insertados = 2 - 1 - 1 = 0
```

```rust
fn new<T>(before: &[T], after: &[T], same: impl Fn(&T, &T) -> bool) -> Self {
    if before.len() == after.len() {
        return Alignment { start: 0, removed: 0, inserted: 0 };
    }
    // The same at the start...
    let start = before
        .iter()
        .zip(after)
        .take_while(|(old, new)| same(old, new))
        .count();
    // ...and the same at the end, without counting an item twice.
    let room = before.len().min(after.len()) - start;
    let end = before[start..]
        .iter()
        .rev()
        .zip(after[start..].iter().rev())
        .take_while(|(old, new)| same(old, new))
        .count()
        .min(room);
    Alignment {
        start,
        removed: before.len() - start - end,
        inserted: after.len() - start - end,
    }
}
```

La cadena de iteradores se lee bien cuando conoces las piezas: `zip`
recorre dos listas lado a lado, dando pares; `take_while` conserva pares
mientras son iguales y se detiene en el primero que no lo es; `count`
dice cuántos se conservaron. `rev` recorre las listas hacia atrás, para
el final.

`.min(room)` es fácil de pasar por alto e importante. Con `[A, A]`
convirtiéndose en `[A]`, el principio encuentra un elemento igual, y el
final también, *el mismo `A`*. Las dos cuentas juntas dirían que se
conservan dos elementos de una lista de uno, y
`after.len() - start - end` sería `1 - 1 - 1`, que para un `usize` no es
-1 sino un panic («attempt to subtract with overflow»). Limitar el final
a lo que queda después del principio mantiene honesta la aritmética.

Cuando ambas listas miden lo mismo, no se añadió ni se quitó nada, así
que los elementos se comparan posición por posición. Eso cubre las
ediciones más comunes: un clic en un paso cambia un patrón, en su sitio.

`Alignment` funciona para cualquier `T`, porque quien llama dice qué
significa «el mismo» con el closure `same`. Para los canales es la
igualdad completa: `|old, new| old == new`. Para los patrones **no** lo
es, y esa es la parte sutil. Las filas de un patrón cambian cuando los
canales van y vienen: cuando entra la banda, cada patrón recibe una fila
para cada uno de sus canales, así que comparar patrones enteros diría
que todos los patrones cambiaron. El alineado enviaría entonces otra vez
las notas de cada patrón, y la aplicación seleccionaría un patrón que
solo parece nuevo.

Así que los patrones se alinean por las cosas que los identifican, no por
su contenido:

```rust
/// Whether two patterns are the same pattern, perhaps with other notes.
fn same_pattern(old: &Pattern, new: &Pattern) -> bool {
    old.name == new.name && old.color == new.color && old.player == new.player
}
```

Las notas y la longitud son lo que el alineador deja cambiar en su
sitio. El nombre, el color y el intérprete (`player`) dicen *qué* patrón
es; las notas que difieren se envían después como cambios `Notes` y
`PatternLength` de todos modos.

### Punteros `fn`: las variantes de un enum son funciones

Una vez alineadas las dos listas, los cambios son: quitar el medio de la
lista antigua y luego insertar el medio de la nueva.

```rust
fn changes(
    self,
    remove: fn(usize) -> Change,
    insert: fn(usize) -> Change,
) -> impl Iterator<Item = Change> {
    let removals = (self.start..self.start + self.removed).rev().map(remove);
    let insertions = (self.start..self.start + self.inserted).map(insert);
    removals.chain(insertions)
}
```

y se llama así:

```rust
patterns.changes(Change::RemovePattern, Change::InsertPattern)
```

`Change::RemovePattern` no es aquí un valor, es una **función**. Toda
variante de un enum que guarda datos entre paréntesis es también una
función que la construye: `Change::RemovePattern` tiene el tipo
`fn(usize) -> Change`. El `fn(usize) -> Change` en minúscula es un tipo
de **puntero a función**: la dirección de una función normal, sin
variables capturadas. Un closure que no captura nada también puede
serlo. `map(remove)` lo llama para cada número, construyendo
`RemovePattern(2)`, `RemovePattern(1)` y así sucesivamente.

Las eliminaciones van desde el **último** hacia atrás. Quitar primero el
patrón 2 deja los patrones 0 y 1 donde estaban, así que
`RemovePattern(1)` sigue significando el patrón correcto. Hacia delante,
quitar primero el patrón 1 bajaría el patrón 2 al 1, y la segunda
eliminación daría con el equivocado. Las inserciones van hacia delante
por la misma razón: una vez llena la posición 1, la posición 2 está justo
después.

`chain` une dos iteradores en uno: primero todos los de `removals`, luego
todos los de `insertions`. La función devuelve
`impl Iterator<Item = Change>`, «algún iterador de cambios», sin nombrar
su largo tipo real (`Chain<Map<Rev<Range<usize>>, fn(usize) -> Change>, ...>`).

### El orden de todo

```rust
pub fn diff(before: &Project, after: &Project) -> Vec<Change> {
    let patterns = Alignment::new(before.patterns(), after.patterns(), same_pattern);
    let channels = Alignment::new(before.channels(), after.channels(), |old, new| old == new);

    let mut changes: Vec<Change> = patterns
        .changes(Change::RemovePattern, Change::InsertPattern)
        .chain(channels.changes(Change::RemoveChannel, Change::InsertChannel))
        .collect();
    ...
```

Primero los patrones, luego los canales, y luego todo lo que hay dentro.
El orden importa por lo que hace el hilo de audio con cada cambio:

- Un **patrón** insertado llega al hilo de audio sin notas en ningún
  canal; sus notas vienen después como cambios `Notes`.
- Un **canal** insertado llega completo: su instrumento, su volumen y sus
  notas en *cada* patrón. Así que los patrones ya deben estar correctos
  cuando llegue, o traería notas para un número equivocado de patrones.

Luego, para cada canal que ya existía (`old_index` dice dónde estaba, o
`None` si es nuevo), se comparan el instrumento y la mezcla:

```rust
for (index, channel) in after.channels().iter().enumerate() {
    let Some(old_index) = channels.old_index(index) else {
        continue; // a new channel arrives complete
    };
    let old = &before.channels()[old_index];
    if old.kind != channel.kind
        || old.program != channel.program
        || old.sent_on != channel.sent_on
    {
        changes.push(Change::Instrument(index));
    }
    if old.volume != channel.volume {
        changes.push(Change::Volume(index));
    }
    if old.pan != channel.pan {
        changes.push(Change::Pan(index));
    }
    if old.reverb_send != channel.reverb_send {
        changes.push(Change::ReverbSend(index));
    }
    if old.effects != channel.effects {
        changes.push(Change::Effects(index));
    }
    if before.channel_muted(old_index) != after.channel_muted(index) {
        changes.push(Change::Muted(index));
    }
}
```

El instrumento se construye a partir de tres campos: el tipo (batería,
archivo de muestra, Lineup Osc, MIDI Out), el programa General MIDI y
`sent_on`, el puerto MIDI al que se envía una batería en lugar de sonar
aquí. `channel_muted` no es simplemente el botón de silencio del canal:
también tiene en cuenta el Solo del mezclador. Por eso un cambio de Solo
aparece como un cambio `Muted` en los demás canales, además de un cambio
`Mixer`.

**`let ... else`** o saca el valor del `Some`, o ejecuta el bloque
`else`, que debe salir: aquí `continue` salta al canal siguiente.

Luego las notas, para cada patrón y cada canal antiguo:

```rust
for (index, pattern) in after.patterns().iter().enumerate() {
    let old_index = patterns.old_index(index);
    let old_pattern = old_index.map(|old| &before.patterns()[old]);
    if let Some(old) = old_pattern
        && old.length() != pattern.length()
    {
        changes.push(Change::PatternLength(index));
    }
    for channel in 0..after.channels().len() {
        let Some(old_channel) = channels.old_index(channel) else {
            continue; // a new channel brought its notes with it
        };
        // The notes as they are played, with the song's swing: a
        // new amount, or a pattern now in a time signature that does
        // not swing, plays other notes. A new pattern starts with no
        // notes in the audio thread.
        let old_notes = match old_index {
            Some(old) => before.played_notes(old, old_channel),
            None => Cow::Borrowed(&[][..]),
        };
        if old_notes != after.played_notes(index, channel) {
            changes.push(Change::Notes { pattern: index, channel });
        }
    }
}
```

El `if let ... && ...` es una **let chain**: el bloque se ejecuta solo
si el patrón es antiguo *y* su longitud cambió.

Las notas que se comparan son `played_notes`, no las escritas. El swing
retrasa las notas a contratiempo sin cambiar lo escrito (ver [Tempo,
compás, swing y feel](../sound/tempo.md)), y al hilo de audio hay que
enviarle lo que se *oye*. Así que subir el swing no cambia ninguna nota
escrita, pero da un cambio `Notes` por cada lista que mueve.
`played_notes` devuelve un **`Cow`** («clone on write», copiar al
escribir): un `Cow::Borrowed` cuando las notas escritas suenan tal cual,
lo que no cuesta nada, y un `Cow::Owned` con una copia cambiada solo
cuando el swing las mueve. Dos `Cow` se comparan por su contenido, sea
cual sea su tipo.

`&[][..]` es un slice vacío: `[]` es un array vacío, y `[..]` lo toma
entero como slice. Un patrón nuevo se compara con «sin notas», que es
exactamente lo que el hilo de audio tiene para él, así que un cambio
`Notes` se envía solo para los canales que tienen notas en él.

Por último vienen el tempo, la soltura, el mezclador y la canción. La
canción se compara a través del arreglo que el modo canción reproduce
de verdad:

```rust
let (old_song, new_song) = (before.playback_arrangement(), after.playback_arrangement());
if old_song.plays != new_song.plays
    || old_song.length != new_song.length
    || before.played_tempos(&old_song) != after.played_tempos(&new_song)
{
    changes.push(Change::Song);
}
```

Un `Arrangement` son los clips con sus repeticiones y las repeticiones
de las secciones ya desenrolladas, recortado a la región de bucle si la
hay (ver [El proyecto](project.md)). Compararlo detecta un
clip movido, un silencio, un número de repeticiones, un bucle cambiado y
un marcador de tempo en un solo cambio `Song`. Los marcadores de tempo
forman parte de la canción porque la longitud de un paso cambia a mitad
de ella.

### Cuando la historia es ambigua

Digamos que hay tres canales Lineup Osc con los mismos ajustes pero
notas distintas, y borras el del medio. Comparando solo los *canales*
(nombre, instrumento, volumen, silencio), la lista antigua es
`[Osc, Osc, Osc]` y la nueva `[Osc, Osc]`, y el alineado no puede saber
cuál se fue: encuentra dos iguales al principio y dice que se quitó el
*último*.

No pasa nada. Las notas del segundo canal ahora difieren (tiene las que
solían ser las notas del tercer canal), así que viene un cambio `Notes`,
y el hilo de audio acaba con exactamente los canales y las notas
correctos. El diff no necesita adivinar lo que hiciste de verdad.
Necesita una lista de pasos que acabe en el mismo sitio, y corta donde
pueda.

## Probar con un modelo

¿Cómo se prueba una función como `diff`? Comprobar su salida exacta
para cada caso sería frágil, y no demostraría que la salida es
*correcta*. Lo que de verdad queremos saber es: *si el hilo de audio
sigue estos cambios, ¿acaba con el proyecto nuevo?*

Así que las pruebas construyen un pequeño **modelo** de lo que sabe el
hilo de audio: las longitudes de los patrones, el instrumento, la
mezcla, los efectos y las notas por patrón de cada canal, el tempo y la
canción. Es lo bastante sencillo como para fiarse de un vistazo:

```rust
struct ModelChannel {
    kind: ChannelKind,
    program: Option<u8>,
    volume: f32,
    muted: bool,
    pan: f32,
    reverb_send: f32,
    effects: crate::Effects,
    /// One list per pattern.
    notes: Vec<Vec<NoteEvent>>,
}

struct Model {
    lengths: Vec<usize>,
    channels: Vec<ModelChannel>,
    tempo: Tempo,
    loose_timing: u8,
    plays: Vec<Play>,
    song_length: usize,
    tempos: Vec<Tempo>,
}
```

`Model::apply` le hace al modelo lo que el hilo de audio hace con cada
cambio, y `check` lo junta todo:

```rust
/// Diffs two projects, checks that following the changes turns the
/// first into the second, and returns the changes.
fn check(before: &Project, after: &Project) -> Vec<Change> {
    let changes = diff(before, after);
    let mut model = Model::of(before);
    for &change in &changes {
        model.apply(change, after);
    }
    assert_eq!(model, Model::of(after), "changes: {changes:?}");
    changes
}

/// Checks both directions, as undo and then redo would.
fn check_both_ways(before: &Project, after: &Project) -> Vec<Change> {
    check(after, before);
    check(before, after)
}
```

Cada prueba comprueba primero esta propiedad, y solo después, donde
importa, los cambios exactos: que un clic en un paso da un único
`Notes`, que los nombres y la tonalidad no dan nada, que deshacer un
patrón borrado empieza con `InsertPattern(0)`, que el swing cambia las
notas que suenan. La última prueba monta un lío a propósito, quitando
un patrón, añadiendo un canal, activando un paso, cambiando un volumen y
el tempo a la vez, y también compara un proyecto vacío con la canción
de demostración, como haría abrir un archivo.

Este tipo de prueba se llama **basada en propiedades**: comprueba una
regla que debe cumplirse para cualquier entrada, en lugar de una
respuesta para una entrada. Uno de los
[ejercicios](exercises.md) va más allá, con proyectos
cambiados al azar.

## El lado de la aplicación: `sync_audio`

`sync_audio` convierte cada `Change` en un comando, leyendo los valores
del proyecto actual:

```rust
fn sync_audio(&mut self, before: &Project) {
    let sample_rate = self.sample_rate();
    let mut patterns_moved = false;
    let mut channels_changed = false;
    for change in diff(before, &self.project) {
        match change {
            Change::RemovePattern(index) => {
                self.send(Command::RemovePattern(index));
                // Keep the same pattern selected. If it was the one
                // removed, the next one takes its place.
                if index < self.pattern {
                    self.pattern -= 1;
                }
                patterns_moved = true;
            }
            Change::InsertPattern(index) => {
                let pattern = &self.project.patterns()[index];
                let (length, bands) = (pattern.length(), pattern.player.is_some());
                self.send(Command::InsertPattern { index, length });
                if bands {
                    // A part the band wrote, as you typed a chord:
                    // the pattern you had stays selected.
                    if index <= self.pattern {
                        self.pattern += 1;
                    }
                } else {
                    // A new pattern (added, cloned, or brought back
                    // by undo) is the one you want to see, from its
                    // start.
                    self.pattern = index;
                    self.rewind_for_new_pattern();
                }
                patterns_moved = true;
            }
            Change::Notes { pattern, channel } => {
                let notes = self.project.played_notes(pattern, channel).into_owned();
                self.send(Command::SetNotes { pattern, channel, notes });
            }
            ...
        }
    }
    ...
}
```

Las demás ramas son igual de directas. `InsertChannel` construye el
instrumento del canal y lo envía con sus notas para cada patrón.
`Instrument` construye un instrumento nuevo y envía `SetInstrument`.
`Volume`, `Pan`, `ReverbSend` y `Effects` envían su único valor,
`Muted` envía el silencio efectivo del canal, `LooseTiming` envía la
cantidad nueva, `Mixer` envía el volumen maestro y los ajustes de la
reverberación, y `Song` envía los tres comandos que sustituyen la
canción, sus tempos y los clics del metrónomo. `Tempo` envía el tempo a
la velocidad de práctica, si ralentizaste la canción para practicar (ver
[Herramientas de práctica](../band/practice.md)).

`into_owned` convierte el `Cow` de `played_notes` en un `Vec` que el
comando puede llevar: clona una lista prestada, y simplemente entrega
una que ya es propia.

`sync_audio` también mantiene los propios números de la ventana
apuntando a lo correcto. El patrón seleccionado es una posición, así que
baja cuando se va un patrón que estaba antes, y sube cuando la banda
inserta uno antes. El piano roll recuerda qué canal muestra, así que se
le avisa cuando los canales van y vienen (`channel_removed` y
`channel_inserted`). Como estas actualizaciones están aquí, añadir,
clonar y borrar patrones selecciona el patrón correcto tanto si lo
hiciste tú como si lo hizo deshacer.

## El hilo de audio inserta en su sitio

Deshacer «delete pattern 0» debe volver a poner un patrón en la
posición 0, no al final. Por eso los comandos del hilo de audio reciben
una posición:

```rust
/// Add an empty pattern at position `index`; the patterns from there
/// on move up one place. An index past the end adds it at the end.
InsertPattern { index: usize, length: usize },

/// Add a channel at position `index`, with its notes in every pattern.
/// The channels from there on move down one row. An index past the
/// end adds it at the bottom.
InsertChannel {
    index: usize,
    instrument: Instrument,
    notes: ChannelNotes,
    volume: f32,
    muted: bool,
},
```

`Vec::insert(index, value)` mueve cada elemento desde `index` un lugar
hacia arriba y pone `value` en el hueco. ¿Reservaría eso memoria en el
hilo de audio? Solo si el `Vec` estuviera lleno. El sitio para
`MAX_CHANNELS` canales y `MAX_PATTERNS` patrones se reserva al crear el
hilo de audio (ver [Espacio reservado de
antemano](threads.md#espacio-reservado-de-antemano)), y
ambos comandos comprueban primero el límite, así que `insert` siempre
cabe en memoria que ya está ahí. Mover elementos es solo copiar, lo cual
está permitido en tiempo real:

```rust
fn insert_pattern(&mut self, index: usize, length: usize) {
    if self.pattern_lengths.len() < MAX_PATTERNS {
        let index = index.min(self.pattern_lengths.len());
        self.pattern_lengths.insert(index, length);
        for channel in &mut self.channels {
            channel.notes.insert_empty(index);
        }
        ...
    }
}
```

Un índice más allá del final se limita (`index.min(len)`) en lugar de
rechazarse. El hilo de audio nunca debe entrar en pánico, y «insertar después
del último» es la lectura más sensata de un número demasiado grande.

`InsertChannel` es uno de los comandos más grandes. Cada ranura de la
cola de comandos es tan grande como el comando más grande, así que una
prueba mantiene los comandos en 80 bytes como mucho. Las voces de un
instrumento están detrás de un `Box`, así que un comando lleva un
puntero a ellas, nunca todas ellas.

## Los instrumentos vuelven de una caché

Deshacer «cargar muestra» le devuelve a un canal su instrumento antiguo,
y puede que sea un archivo de muestra. Leer un archivo en el hilo de la
ventana para un deshacer haría tartamudear a la ventana, y el archivo
podría incluso haber desaparecido ya.

Así que la aplicación guarda una **`SampleCache`**: cada archivo de
muestra leído hasta ahora, por ruta.

```rust
pub struct SampleCache {
    loaded: HashMap<PathBuf, Arc<SampleData>>,
}

impl SampleCache {
    pub fn get(&mut self, path: &Path) -> Result<Arc<SampleData>, SampleError> {
        // A `HashMap<PathBuf, _>` can be searched with a `&Path`, because
        // `PathBuf` implements `Borrow<Path>`: no `PathBuf` is built here.
        if let Some(sample) = self.loaded.get(path) {
            return Ok(Arc::clone(sample));
        }
        self.reload(path)
    }

    pub fn reload(&mut self, path: &Path) -> Result<Arc<SampleData>, SampleError> {
        let sample = Arc::new(SampleData::load_wav(path)?);
        self.loaded.insert(path.to_path_buf(), Arc::clone(&sample));
        Ok(sample)
    }
}
```

Un **`HashMap`** encuentra un valor por su clave en más o menos el mismo
tiempo tenga las entradas que tenga. Sus claves son `PathBuf`s con
propiedad, pero `get` recibe un `&Path` prestado. Eso funciona gracias
al **trait `Borrow`**: `PathBuf` implementa `Borrow<Path>`, una promesa
de que un `PathBuf` y el `Path` que contiene hacen hash y se comparan de
la misma manera. Así el mapa puede buscar un `&Path` directamente, sin
construir un `PathBuf` solo para buscar. (Un `HashMap<String, _>` se
puede buscar con un `&str` por la misma razón.)

La caché entrega clones de `Arc`, así que la caché y cada sampler que
usa el sonido comparten una copia en memoria. Una prueba lo comprueba
con **`Arc::ptr_eq`**, que pregunta si dos `Arc` apuntan a exactamente
la misma memoria, no solo a valores iguales:

```rust
assert!(Arc::ptr_eq(&first, &second), "the second get shares the first");
assert!(!Arc::ptr_eq(&first, &reloaded), "reload reads the file again");
```

Soltar un archivo sobre el rack llama a `reload`, no a `get`: puede que
hayas cambiado el archivo en un editor de audio desde entonces, y
volverlo a soltar debe recogerlo. Un archivo que no se puede leer no se
recuerda, así que el siguiente intento lo lee otra vez. Abrir otro
proyecto vacía la caché.

`Instrument::for_channel` recibe la caché como argumento
(`&mut SampleCache`) en lugar de leer los archivos por sí mismo.
Exportar vuelve a construir cada instrumento (ver
[Exportar](../sound/export.md)), y gracias a la caché tampoco lee
archivos.
