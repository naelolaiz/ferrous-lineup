# El proyecto: patrones, canales, pistas, clips y marcas

Archivos: [`crates/lineup-core/src/pattern.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/pattern.rs),
[`crates/lineup-core/src/project.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/project.rs),
[`crates/lineup-core/src/playlist.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/playlist.rs)
y [`crates/lineup-core/src/arrangement.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/arrangement.rs)

Todo aquello con lo que trabaja el usuario es un solo valor de tipo
`Project`: los canales del rack de canales, los patrones de notas que
tocan, la lista de reproducción que ordena los patrones en una canción,
y las marcas a lo largo de ella (secciones, tempo, compás, tonalidad y
acordes). Es también exactamente lo que guarda un archivo de proyecto.
Este capítulo trata de su forma y de las reglas que mantiene. Las
ventanas que lo editan están en [Los editores](editors.md);
cómo usarlas está en el capítulo [Patrones y la lista de
reproducción](https://naelolaiz.github.io/ferrous-lineup/es/manual/patterns-and-playlist.html)
del manual de usuario.

## Notas, pasos y patrones

Un **patrón** es un tramo corto de música, a menudo un compás, con una
fila de notas para cada canal. Una nota es una altura, un comienzo, una
duración y una velocidad, todo contado en **pasos** (una semicorchea
cada uno en 4/4):

```rust
pub struct NoteEvent {
    pub pitch: Note,
    /// The step the note starts on, counted from 0.
    pub start: usize,
    /// How many steps the note is held for, at least 1.
    pub length: usize,
    /// How hard the note is played, from 1 to 127 (as in MIDI).
    pub velocity: u8,
    /// How many ticks after the start of its first step the note really
    /// starts, from 0 to [`TICKS_PER_STEP`] - 1.
    pub nudge: u8,
    /// How many ticks before the end of its last step the note really
    /// ends.
    pub trim: u8,
}
```

Los botones de paso del rack de canales y el piano roll editan las
mismas notas. Un paso «encendido» no es más que una nota de un paso de
largo en la altura por defecto, así que `is_on` pregunta si alguna nota
empieza ahí:

```rust
pub fn is_on(&self, channel: usize, step: usize) -> bool {
    self.notes(channel).iter().any(|note| note.start == step)
}
```

Una sola clase de datos con dos vistas es como funcionan los estudios
basados en patrones, y significa que el hilo de audio tiene una sola
clase de cosa que tocar. `nudge` y `trim` colocan una nota entre pasos,
para música tocada por una persona o importada de un archivo MIDI; las
notas en la cuadrícula los dejan en 0.

## Canales: enums con datos

```rust
pub enum SampleSource {
    /// One of the drums built into Ferrous Lineup.
    Builtin(Drum),
    /// A WAV file on disk.
    File(PathBuf),
}

pub enum ChannelKind {
    /// Plays a recorded or built-in sound from start to end for each
    /// note, faster for higher notes and slower for lower ones.
    Sampler(SampleSource),
    /// The built-in "Lineup Osc" synthesizer, which can play chords.
    Synth,
    /// Makes no sound itself: sends its notes out through MIDI, on
    /// MIDI channel `channel` (0 to 15, shown as 1 to 16).
    MidiOut { channel: u8 },
}
```

Un canal tiene un nombre, un volumen, un interruptor de silencio, un
número de instrumento General MIDI y un tipo. Los enums se anidan: un
canal es un sampler, el Lineup Osc o un canal de salida MIDI, y el
sonido de un sampler es o un tambor incorporado o un archivo. Cada hecho
se guarda una vez y toda combinación tiene sentido.
`MidiOut { channel: u8 }` muestra que una variante también puede tener
**campos con nombre**, como un struct.

`PathBuf` es el tipo estándar para una ruta de archivo poseída. No es un
`String`, porque los nombres de archivo en algunos sistemas no son texto
UTF-8 válido; `path.display()` convierte uno en algo imprimible.

`Drum::ALL` es una constante asociada que contiene todas las variantes,
para que los menús puedan iterar sobre los tambores en lugar de
enumerarlos a mano.

## La forma de un proyecto

Un proyecto tiene una lista de patrones, y una lista de reproducción que
dice dónde suena cada uno:

```rust
pub struct Project {
    pub mixer: crate::Mixer,
    pub tempo: Tempo,
    pub scale: Scale,
    pub time_signature: TimeSignature,
    pub loose_timing: u8,
    pub swing: u8,
    pub(crate) channels: Vec<Channel>,
    pub(crate) patterns: Vec<Pattern>,
    pub(crate) playlist: Playlist,
    pub(crate) band: Option<Band>,
    // ...where each part came from, and the locked channels
}
```

Cada patrón tiene una fila de notas por canal. Los estudios
basados en patrones funcionan así: un patrón es «lo que toca cada
instrumento durante estos compases», así que el patrón de batería tiene
filas vacías para el sintetizador.

La lista de reproducción son dos listas:

```rust
pub struct Track {
    pub name: String,
    pub muted: bool,
    // ...a display height, which older files leave out
}

pub struct Clip {
    pub pattern: usize,
    pub track: usize,
    /// The first step the clip plays.
    pub start: usize,
    /// How many times round the pattern the clip plays, one after the
    /// other. A trimmed time round counts.
    pub repeats: usize,
    /// Steps left out at the start of the first time round, and at the
    /// end of the last (see "Trimming a clip" below).
    pub skip: usize,
    pub cut: usize,
}

pub struct Playlist {
    pub(crate) tracks: Vec<Track>,
    pub(crate) clips: Vec<Clip>,
    // ...sections, a loop region, and time, tempo, key and chord markers
}
```

Un clip son tres números, un contador de repeticiones y dos recortes:
qué patrón, en qué pista, empezando en qué paso de la canción, cuántas
veces suena el patrón y cuánto de él se deja fuera en cada extremo.
Fíjate en lo que *no* está: la duración del clip. Es la duración del
patrón por el número de repeticiones, menos los recortes, así que se
calcula cuando hace falta:

```rust
pub fn clip_length(&self, clip: &Clip) -> usize {
    clip.length(self.patterns.get(clip.pattern).map_or(0, Pattern::length))
}

impl Clip {
    /// A clip that plays its pattern once, whole.
    pub fn new(pattern: usize, track: usize, start: usize) -> Self {
        Clip {
            pattern,
            track,
            start,
            repeats: 1,
            skip: 0,
            cut: 0,
        }
    }

    pub fn length(&self, pattern_length: usize) -> usize {
        let (skip, cut) = self.trims(pattern_length);
        (pattern_length * self.repeats).saturating_sub(skip + cut)
    }
}
```

Guardar también la duración supondría dos sitios que deben coincidir. Haz
el patrón más largo y la duración guardada de cada clip quedaría mal. Un
valor que se puede calcular a partir de otros casi siempre es mejor
calcularlo que guardarlo. Lo mismo vale para el final de un clip repetido:
es `start + clip_length`, no otro campo.

`Project::playback_arrangement` desenrolla las repeticiones en `Play`s
antes de que la canción llegue al hilo de audio (ver [Secciones,
repeticiones y bucles](#secciones-repeticiones-y-bucles) más abajo); el
clip es lo que dibujas, y su `repeats` es el pequeño «×2» que lleva
encima.

`map_or(0, Pattern::length)` dice «si hay un patrón, llama a
`Pattern::length` sobre él; si no, usa 0». Un método se puede pasar por
nombre, como una función, cuando su único argumento es `self`.

### Recortar un clip

Arrastrar hacia adentro el borde de un clip oculta parte de su patrón,
como un programa de audio recorta un sonido: las notas siguen en el
patrón, solo que no suenan, y arrastrar el borde de nuevo hacia afuera
las trae de vuelta. `skip` es cuántos pasos del patrón deja fuera la
primera vuelta, `cut` cuántos deja fuera la última. Los dos son menores
que la duración del patrón, así que la primera y la última vuelta
siempre tocan algo.

`start` sigue siendo «el primer paso que suena». Así, un clip recortado
al comienzo toca el paso `skip` de su patrón en el paso `start` de la
canción, y el lugar donde caería el paso 0 de su patrón, si no estuviera
recortado, es `start - skip`, el **origen** del clip. Ahí es donde un
editor busca el compás y la tonalidad en que suena por primera vez un
patrón.

Todos los que leen el clip calculaban «el patrón vuelve a empezar cada
`pattern_length` pasos desde `start`». Con recortes eso vale solo desde
el origen, y la primera y la última vuelta son más cortas. En vez de
que cada lector lo resuelva bien, el clip lo dice una sola vez:

```rust
pub struct Round {
    /// The song step `steps.start` is heard on.
    pub at: usize,
    /// The steps of the pattern heard.
    pub steps: Range<usize>,
}

pub fn rounds(&self, pattern_length: usize) -> impl Iterator<Item = Round> + '_ {
    let (skip, cut) = self.trims(pattern_length);
    let repeats = if pattern_length == 0 { 0 } else { self.repeats };
    (0..repeats).map(move |round| {
        let first = if round == 0 { skip } else { 0 };
        let last = if round + 1 == repeats { pattern_length - cut } else { pattern_length };
        Round {
            at: self.start + round * pattern_length - (skip - first),
            steps: first..last,
        }
    })
}
```

Una nota del patrón suena en una vuelta si empieza dentro de `steps`, y
`Round::place` la lleva a la línea de tiempo de la canción, cortada
donde termina la vuelta, así que una nota sostenida más allá de un final
recortado se detiene ahí. La reproducción, la banda, la partitura
guía, las tonalidades y los acordes que se oyen, insertar compases y
cambiar el compás leen todos los clips a través de `rounds()`.

Mover un borde debe dejar cada nota que sigue sonando donde estaba.
Mover el borde derecho solo cambia hasta dónde llega el clip dentro de
su patrón:

```rust
let reach = skip + (step - self.start);
let repeats = reach.div_ceil(pattern_length);
Clip { repeats, skip, cut: repeats * pattern_length - reach, ..*self }
```

`div_ceil` redondea hacia arriba: llegar 20 pasos dentro de un patrón de
16 son 2 vueltas, la segunda con 12 pasos recortados. Mover el borde
izquierdo deja el origen donde está, y calcula desde él cuántas vueltas
gana o pierde el clip y dónde empieza la nueva primera vuelta:

```rust
let origin = self.start as isize - skip as isize;
let from = step as isize - origin;
let rounds = from.div_euclid(pattern_length as isize);
let skip = from.rem_euclid(pattern_length as isize) as usize;
let repeats = self.repeats as isize - rounds;
```

`div_euclid` y `rem_euclid` son la división y el resto que redondean
hacia menos infinito, así que también sirven para un paso antes del
origen: arrastrado 4 pasos hacia afuera del comienzo de un patrón de 16
pasos, `from` es -4, `rounds` es -1 (se gana una vuelta) y `skip` es 12,
así que el clip toca una vez más los últimos 4 pasos del patrón antes de
él. Con `/` y `%`, -4 / 16 daría 0 y -4 % 16 daría -4.

La lista de reproducción lee los recortes de un clip para dibujarlo: las
notas de la primera vuelta se dibujan empezando `skip` pasos a la
izquierda del clip, y el dibujo se corta al rectángulo del clip, así que
lo oculto no se dibuja.

`skip` y `cut` se dejan fuera del archivo cuando valen 0 (ver
[Guardar](saving.md)), así que una canción sin clips recortados se
escribe como antes. Una con un clip recortado es formato 33: una versión
anterior tocaría el patrón entero.

## Cuatro reglas, un guardián

El comentario de `Project` enumera las reglas que mantiene:

- cada clip de la lista de reproducción se refiere a un patrón y a una pista que existen;
- las secciones están ordenadas y nunca se solapan, y las repeticiones no se salen de rango.

La mayoría de estas reglas tratan de dos listas a la vez, así que
ninguna lista puede mantenerlas por sí sola. El `Project` las posee
todas, mantiene los campos privados para `lineup-core` y los cambia solo
en métodos que respetan todas las reglas. Añadir un canal añade una fila
a cada patrón:

```rust
pub fn add_channel(&mut self, channel: Channel) -> Option<usize> {
    if self.channels.len() >= MAX_CHANNELS {
        return None;
    }
    self.channels.push(channel);
    for pattern in &mut self.patterns {
        pattern.push_row();
    }
    Some(self.channels.len() - 1)
}
```

Borrar un patrón es lo interesante. Los clips se refieren a los patrones
por su posición en la lista, así que cuando el patrón 1 desaparece, los
clips del patrón 1 deben desaparecer también, y los clips de los patrones
2, 3, 4... deben bajar uno cada uno para que sigan reproduciendo la misma
música:

```rust
pub fn remove_pattern(&mut self, index: usize) -> Option<Pattern> {
    if index >= self.patterns.len() || self.patterns.len() == 1 {
        return None;
    }
    self.playlist.clips.retain(|clip| clip.pattern != index);
    for clip in &mut self.playlist.clips {
        if clip.pattern > index {
            clip.pattern -= 1;
        }
    }
    Some(self.patterns.remove(index))
}
```

- `retain` conserva los elementos para los que el closure devuelve `true`
  y descarta los demás, en una sola pasada.
- `for clip in &mut self.playlist.clips` da una referencia mutable a cada
  clip por turno, así que `clip.pattern -= 1` cambia el clip de la lista.
- El método devuelve el patrón borrado, envuelto en `Some`. Quien lo llama
  puede ignorarlo, guardarlo, o
  comprobar que la eliminación llegó a ocurrir.

La prueba `removing_a_pattern_renumbers_its_followers` comprueba toda la
historia: tras borrar «Ritmo», los cinco clips de Ritmo desaparecen,
«Acordes» es el patrón 0, y el fill sigue sonando en el compás 8.

¿Por qué usar posiciones, en lugar de dar a cada patrón un id permanente?
Los ids harían más simple la eliminación (nada que renumerar) pero todo lo
demás más difícil: cada búsqueda tendría que buscar un id en lugar de
indexar un `Vec`. Para una lista de como máximo 999 patrones (normalmente
muchos menos), las posiciones más un método cuidadoso son la opción más
simple. Es el tipo de intercambio sin una única respuesta correcta; uno
de los [ejercicios](exercises.md) de esta parte te pide que
pienses en ello.

## Repartir `&mut Pattern` con seguridad

La app necesita cambiar notas de un patrón. `Project` podría copiar cada
método de `Pattern` (`toggle`, `add_note`, `replace_note`...) como método
propio. En su lugar reparte el patrón mismo:

```rust
pub fn pattern_mut(&mut self, index: usize) -> Option<&mut Pattern> {
    self.patterns.get_mut(index)
}
```

¿Es eso seguro? Con un `&mut Pattern`, ¿podría la app añadir una fila y
romper la regla de «una fila por canal»? No, por cómo están declarados los
métodos de filas en `pattern.rs`:

```rust
pub(crate) fn push_row(&mut self) { ... }
pub(crate) fn remove_row(&mut self, channel: usize) { ... }
```

`pub(crate)` significa «público solo dentro de este crate». `project.rs`
puede llamarlos, porque está en `lineup-core`; `lineup-app` no puede, ni
siquiera con un `&mut Pattern` en las manos. Cada método que `lineup-app`
*sí puede* llamar mantiene el número de filas tal como está. La privacidad
en Rust va de módulos y crates, no de quién tiene una referencia, y eso
permite a un tipo dar una referencia a otro sin regalar sus reglas.

En el lado de la app, cada edición de un patrón pasa por un pequeño ayudante:

```rust
fn edit_pattern(&mut self, label: &'static str, change: impl FnOnce(&mut Pattern)) {
    let index = self.pattern;
    self.edit(label, |project| {
        if let Some(pattern) = project.pattern_mut(index) {
            change(pattern);
        }
    });
}

// For example:
RackAction::ToggleStep { channel, step } => {
    self.edit_pattern("change steps", |pattern| {
        pattern.toggle(channel, step);
    });
}
```

- `impl FnOnce(&mut Pattern)` acepta cualquier closure que reciba un
  patrón. `FnOnce` es el más permisivo de los tres traits de closures:
  el closure solo tiene que poder llamarse una vez, así que puede
  incluso mover valores que capturó.
- `self.edit(label, ...)` es la única función por la que pasa cada
  cambio del proyecto. Guarda el cambio para deshacerlo y le dice al
  hilo de audio qué cambió ([Deshacer y rehacer](undo.md),
  [Avisar al hilo de audio de lo que cambió](sync.md)).
  `label` es lo que dirá **Edición > Deshacer**.

## Nombres, e `impl Into<String>`

Los patrones tienen nombre. El constructor recibe el nombre primero:

```rust
pub fn new(name: impl Into<String>, channels: usize, length: usize) -> Self {
    Pattern {
        name: name.into(),
        color: 0,
        time: None,
        player: None,
        length: length.clamp(1, MAX_STEPS),
        rows: vec![Vec::new(); channels],
    }
}
```

`impl Into<String>` acepta cualquier cosa que se pueda convertir en un
`String`: `Pattern::new("Beat", 5, 16)` con un `&str`, o
`Pattern::new(format!("Pattern {n}"), 5, 16)` con un `String`, sin que
quien llama escriba `.to_string()`. Dentro, `name.into()` hace la
conversión (para un `String`, simplemente lo mueve).

`name` es un campo público mientras que `length` y `rows` son privados.
Mezclar está bien en Rust: `name` no tiene ninguna regla que mantener, así
que no hay razón para esconderlo detrás de métodos. `color`, `time` y
`player` dicen de qué color se dibuja el patrón, en qué compás se
escribió, y cuál de los músicos de la banda lo escribió, si lo escribió
alguno.

Clonar un patrón usa el derive `Clone` de `Pattern`:

```rust
pub fn clone_pattern(&mut self, index: usize) -> Option<usize> {
    let mut copy = self.patterns.get(index)?.clone();
    copy.name = format!("{} copy", copy.name);
    // A copy is yours, not the band's.
    copy.player = None;
    self.push_pattern(copy)
}
```

`.clone()` copia el patrón entero, cada fila y cada nota, así que la copia
y el original nunca comparten nada; la prueba `cloned_patterns_are_independent`
comprueba que cambiar uno deja al otro en paz. El `?` después de
`get(index)` devuelve `None` de la función inmediatamente si no existe tal
patrón.

## Nunca dos notas en el mismo lugar

`add_note` se niega a añadir una nota en el mismo lugar que otra.
`replace_note`, usado al arrastrar, también debe negarse, o una nota
podría arrastrarse exactamente encima de otra, así que comprueba cada
nota excepto la que se está reemplazando:

```rust
let duplicate = row
    .iter()
    .enumerate()
    .any(|(other_index, other)| other_index != index && other.same_place(&note));
```

Así que «no hay dos notas de la misma altura que empiecen en el mismo
momento» es cierto para cada patrón, y cargar un archivo puede insistir
en ello. `same_place` compara la altura, el paso y el nudge, así que dos
notas que empiezan a unos pocos ticks de distancia siguen siendo dos
notas.

## ¿Cuánto dura la canción?

La canción termina donde termina su último clip, redondeado hacia arriba a
un compás entero:

```rust
pub fn song_length(&self) -> usize {
    let clips = self
        .playlist
        .clips
        .iter()
        .map(|clip| clip.start + self.clip_length(clip));
    let sections = self.playlist.sections.iter().map(Section::end);
    let end = clips.chain(sections).max().unwrap_or(0);
    self.meter().round_up_to_bar(end)
}
```

`max` devuelve un `Option`, porque una lista vacía no tiene máximo;
`unwrap_or(0)` convierte «no hay clips» en una duración de 0. `chain` une
los dos iteradores, porque una canción puede terminar con un clip o con una
sección que no tiene ningún clip justo al final. `self.meter()` conoce el
compás, y su `round_up_to_bar` es una división que redondea hacia arriba:
en 4/4, 17 pasos son `17.div_ceil(16) = 2` compases.

## ¿Dónde está un patrón mientras suena la canción?

En modo canción, el hilo de audio informa de un paso de la *canción*. El rack de canales y el piano roll muestran un patrón, así que necesitan saber
qué paso de *ese* patrón está sonando, si es que hay alguno:

```rust
pub fn pattern_step_in_song(&self, pattern: usize, song_step: usize) -> Option<usize> {
    self.playlist
        .audible_clips()
        .filter(|clip| clip.pattern == pattern)
        .find_map(|clip| self.step_in_clip(&clip, song_step))
}

/// The step of its pattern the clip plays on song step `step`.
fn step_in_clip(&self, clip: &Clip, step: usize) -> Option<usize> {
    let pattern_length = self.patterns.get(clip.pattern).map_or(0, Pattern::length);
    clip.rounds(pattern_length)
        .find(|round| (round.at..round.end()).contains(&step))
        .map(|round| round.steps.start + step - round.at)
}
```

Léelo como una frase: de los clips que se pueden oír, toma los de este
patrón, encuentra el primero que cubra este paso de la canción y dime qué
paso del patrón toca ahí esa vuelta. Las vueltas son lo que hace que un
clip repetido vuelva a empezar su patrón en cada vuelta: un clip de 16
pasos que se repite en el paso 16 de la canción está allí en el paso 0
del patrón. Un clip recortado al comienzo está en el paso `skip` en su
primer paso. `audible_clips` se salta los clips de pistas silenciadas:

```rust
pub fn audible_clips(&self) -> impl Iterator<Item = Clip> + '_ {
    self.clips
        .iter()
        .copied()
        .filter(|clip| self.tracks.get(clip.track).is_some_and(|track| !track.muted))
}
```

`.copied()` convierte un iterador de `&Clip` en uno de `Clip`. Funciona
porque `Clip` es `Copy` (unos pocos números), y significa que quien llama recibe
valores simples en lugar de referencias a la lista de reproducción.

## La canción de partida

`Project::demo` construye una canción de ocho compases, que tocan muchas
pruebas, con los mismos métodos que usa la app:

```rust
for bar in [0, 2, 4, 6] {
    project.add_clip(Clip::new(chords, chord_track, bar * 16));
}
for bar in 2..7 {
    project.add_clip(Clip::new(beat, drum_track, bar * 16));
}
project.add_clip(Clip::new(fill, drum_track, 7 * 16));
```

`2..7` es un rango de 2 hasta, pero sin incluir, el 7: compases 3 a 7
contando desde 1, como hace la regla. El fill ocupa el lugar del beat en
el compás 8. **Archivo > Nuevo ritmo** empieza en cambio desde
`Project::template`: la batería y un Lineup Osc, sin notas y con una
pista. **Archivo > Nuevo** empieza desde `Project::with_band`, una hoja
de acordes vacía con una banda ([Cómo toca la banda una hoja de
acordes](../band/band.md)).

## Secciones, repeticiones y bucles

Una canción tiene más forma que sus clips: una introducción, una
estrofa, un estribillo que se oye dos veces, un ritmo que se repite
durante ocho compases. La lista de reproducción guarda tres cosas más
para eso:

- **Secciones**: tramos con nombre de la canción (Introducción, Estrofa,
  Estribillo…) en una franja sobre los números de compás. Una sección se
  puede tocar varias veces seguidas: se dibuja una vez y se marca «×2».
- **Repeticiones de clip**: un clip puede tocar su patrón una y otra
  vez, como al estirar un clip de patrón en otros estudios basados en
  patrones.
- **Una región de bucle**: el modo canción puede repetir solo una parte
  de la canción.

Las secciones no son dueñas de clips. Una sección es solo un nombre y un
rango de pasos: los clips que quedan debajo forman parte de ella.
Eliminar una sección deja los clips como están.

### Dos tipos de paso

Aquí está la única idea que hace sencillo todo lo demás. Una vez que una
sección puede repetirse, la lista de reproducción tal como se *dibuja* y la canción
tal como se *toca* ya no miden lo mismo:

```text
timeline:   | Intro | Chorus ×2 | Outro |
canción:    | Intro | Chorus | Chorus | Outro |
```

Así que el código habla de dos tipos de paso:

- un **paso de timeline** es un lugar de la lista de reproducción, tal como se dibuja;
- un **paso de canción** es un momento de la canción, tal como se oye.

`Project::song_length` es la longitud del timeline. `Arrangement` es la
canción tal como se oye:

```rust
pub struct Arrangement {
    /// The pieces of patterns to play, in song steps.
    pub plays: Vec<Play>,
    /// The stretches of the timeline, in the order they play.
    pub passes: Vec<Pass>,
    /// How long the song is, in song steps.
    pub length: usize,
}

pub struct Play {
    pub pattern: usize,
    pub track: usize,  // the track of its clip
    pub start: usize,  // the song step it starts on
    pub skip: usize,   // how far into the clip it starts
    pub length: usize, // how many steps it lasts
}
```

Un `Pass` dice «los pasos de timeline 32 a 63 suenan en los pasos de
canción 64 a 95, y esta es la ronda 2 de 2 de la sección 1». Un `Play`
dice «toca el patrón 3 desde el paso de canción 64 durante 32 pasos,
empezando 0 pasos dentro de él». El patrón se repite en bucle mientras dura el
`Play`, así que un clip repetido tres veces es un `Play` de tres patrones
de largo. `skip` no es cero cuando una sección o la región de bucle
empieza a mitad de un clip, o cuando el clip está recortado al
comienzo: entonces es el `skip` del propio clip más cuánto se adentra en
el clip el trozo. `track` está ahí para una sola cosa: la grabación de
una toma suena solo bajo los clips de su pista (ver [Solo bajo sus
clips](../sound/instruments.md#solo-bajo-sus-clips)).

`Project::arrangement()` lo construye para la canción entera (las
exportaciones lo usan), y `Project::playback_arrangement()` para lo que
toca el modo canción: la región de bucle si la hay, la canción entera si
no.

#### Construir el arreglo

`arrange` en `arrangement.rs` corta el timeline en trozos en los bordes
de las secciones. Las secciones suenan `repeats` veces; los huecos entre
ellas suenan una vez:

```rust
fn pieces(project: &Project, from: usize, to: usize)
    -> Vec<(usize, usize, Option<usize>, usize)>
{
    let mut pieces = Vec::new();
    let mut cursor = from;
    for (index, section) in project.playlist().sections().iter().enumerate() {
        if section.end() <= cursor || section.start >= to {
            continue;
        }
        if section.start > cursor {
            pieces.push((cursor, section.start, None, 1)); // a gap
        }
        let start = section.start.max(cursor);
        let end = section.end().min(to);
        pieces.push((start, end, Some(index), section.repeats));
        cursor = end;
    }
    if cursor < to {
        pieces.push((cursor, to, None, 1));
    }
    pieces
}
```

Esto solo funciona porque las secciones se mantienen **ordenadas y sin
solaparse nunca**. Eso es una regla, así que, como con los clips, la
lista es privada y solo los métodos de `Project` la cambian.
`add_section` y `set_section` comprueban la regla y rechazan una sección
que la rompería; `insert_section` mantiene el orden con
`partition_point`, una búsqueda binaria que devuelve el primer lugar
donde la condición deja de ser cierta:

```rust
let index = sections.partition_point(|other| other.start < section.start);
sections.insert(index, section);
```

Luego, para cada trozo y cada pasada, todo clip que se solape con el
trozo añade un `Play` para la parte de él que está dentro del trozo.
`max` y `min` encajan el clip en el trozo:

```rust
let begin = clip.start.max(piece_from);
let end = clip_end.min(piece_to);
if begin < end {
    arrangement.plays.push(Play {
        pattern: clip.pattern,
        start: start + (begin - piece_from),
        skip: begin - clip.start,
        length: end - begin,
    });
}
```

### El hilo de audio no sabe nada de secciones

`Command::SetSong` lleva los `plays` del arreglo, y lo único que hace el
hilo de audio con ellos es un resto:

```rust
pub fn pattern_step(&self, step: usize, pattern_length: usize) -> Option<usize> {
    if pattern_length == 0 || !(self.start..self.end()).contains(&step) {
        return None;
    }
    Some((self.skip + step - self.start) % pattern_length)
}
```

`%` envuelve el paso alrededor del patrón, que es todo lo que significa
«repetir». Todo lo de nombres, rondas y bucles se queda en `lineup-core`,
donde puede probarse sin tarjeta de sonido. El hilo de audio sigue siendo
pequeño, y no reserva memoria: el `Vec<Play>` se construye en el hilo de
la UI, y el viejo se devuelve para liberarlo allí.

La exportación a WAV y la exportación a MIDI usan el mismo
`Arrangement`, así que las tres coinciden en qué es la canción. El
escritor de MIDI necesita la pregunta contraria: ¿en qué pasos de canción
suena una nota del paso de patrón 2? Eso es `Play::song_steps_of`, que
usa un iterador con `step_by`:

```rust
(first..self.length)
    .step_by(pattern_length.max(1))
    .map(move |offset| self.start + offset)
```

### De vuelta de la canción a la lista de reproducción

El hilo de audio cuenta pasos de canción, así que el cabezal de
reproducción que comunica es un paso de canción. Para dibujarlo, la app
lo convierte de nuevo en un paso de timeline con los passes del arreglo:

```rust
fn song_position(&self) -> Option<(usize, Pass)> {
    if self.mode != PlayMode::Song {
        return None;
    }
    let step = self.playhead()?;
    let song = self.project.playback_arrangement();
    let pass = *song.pass_at(step)?;
    Some((pass.from + (step - pass.start), pass))
}
```

El `Pass` viaja con él hasta la lista de reproducción, que es como una sección sabe
que debe mostrar «▶ 2 de 2». El rack de canales, el piano roll y las teclas de armonía reciben el paso de timeline, así que muestran el paso correcto del
clip correcto también durante una repetición.

### Guardar: formato 4

`Clip` y `Section` tienen las dos un campo `repeats`, y la mayoría de las
veces es 1. serde puede dejar un campo fuera del archivo cuando tiene su
valor habitual, y volver a rellenarlo al leer un archivo que no lo tiene:

```rust
#[serde(default = "once", skip_serializing_if = "is_once")]
pub repeats: usize,
```

`default` nombra una función que devuelve el valor que usar, y
`skip_serializing_if` una función que dice cuándo dejar el campo fuera.
Las listas del archivo (`sections`, `loop_region`) usan el
`#[serde(default)]` normal con `Vec::is_empty` y `Option::is_none`. Así
que un proyecto que no usa nada de esto se escribe byte a byte como
antes, y un Ferrous Lineup más antiguo lo abre.

Una versión más antigua también abriría un proyecto que *sí* usa
repeticiones: se salta los campos que no conoce. Pero tocaría cada
repetición una vez, que es una canción que suena mal sin una palabra de
aviso. Así que `format_for` devuelve **4** para cualquier proyecto con
secciones, una región de bucle o un clip repetido, y las versiones
antiguas lo rechazan con un mensaje claro
([Guardar](saving.md)).

### Dibujarlo: un módulo dentro de un módulo

Las franjas se dibujan en un archivo propio. Un módulo de archivo puede
tener hijos: `mod sections;` dentro de `playlist.rs` carga
`playlist/sections.rs`. Un módulo hijo puede ver los elementos privados
de su padre, así que `sections.rs` usa `Layout`, `Snap` y
`PlaylistAction` de `playlist.rs` con
`use super::{Layout, PlaylistAction, Snap};` sin hacerlos públicos para
el resto de la app.

Cada sección es su propia zona clicable, hecha con `ui.interact` sobre un
rectángulo que elegimos nosotros, que devuelve un `Response` como
cualquier widget:

```rust
let response = ui.interact(rect, ui.scope_id().with(("section", index)),
                           Sense::click_and_drag());
```

El id debe ser distinto para cada sección y el mismo de un frame a otro,
así que se construye a partir del número de la sección. El menú es un
`egui::Popup` atado a esa respuesta, que se abre con un clic y solo se
cierra con un clic fuera, porque contiene un campo de texto en el que
escribes.

Arrastrar una sección no cambia el proyecto en cada frame, como hace
arrastrar un clip. Solo se dibuja un contorno, y el proyecto cambia una
vez, cuando el botón sube. Una sección se mueve dentro de una lista
ordenada: si el movimiento de un frame la saltara por encima de una
vecina, su número cambiaría a mitad del arrastre, y el siguiente frame
movería la sección equivocada.

### Escribir las repeticiones

**Escribir las repeticiones** en el menú de una sección convierte «Estribillo ×3» en Estribillo, Estribillo 2 y Estribillo 3, uno detrás de otro, cada uno tocado
una vez, para que puedas cambiar el tercer estribillo sin cambiar los
otros. La canción suena igual antes y después; una prueba lo comprueba
comparando los dos arreglos.

![Antes (abajo) y después (arriba) de escribir las repeticiones de «Chorus ×2».](../images/step-8-write-out.png)

Comparte su código con **Duplicar**: `make_room` mueve todo lo que hay
después de la sección más tarde, y luego los clips que empiezan dentro de
la sección se copian en el hueco. Una cosa que hay que vigilar: los clips
se copian enteros. El relleno de la imagen empieza dentro del estribillo
y sigue más allá de su final, así que el original y su copia se solapan.
Ferrous Lineup no puede cortar un clip en dos (un clip no tiene «empieza
a esta distancia dentro del patrón»); uno de los
[ejercicios](exercises.md) de esta parte lo añade.

![Arrastrar por los números de compás hace loop de los compases 3 a 6.](../images/step-8-loop.png)
