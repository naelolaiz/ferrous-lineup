# Muchos patrones, y un playlist para ordenarlos

Archivos: [`crates/lineup-core/src/project.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/project.rs),
[`crates/lineup-core/src/playlist.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/playlist.rs)
y [`crates/lineup-core/src/pattern.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/pattern.rs)

## La forma de un proyecto

En el paso 3 un proyecto tenía un patrón. Ahora tiene una lista de ellos,
y un playlist que dice dónde suena cada uno:

```rust
pub struct Project {
    pub tempo: Tempo,
    pub scale: Scale,
    channels: Vec<Channel>,
    patterns: Vec<Pattern>,
    playlist: Playlist,
    // ...time signature, mixer, band and the later chapters' fields
}
```

Cada patrón sigue teniendo una fila de notas por canal. Los estudios
basados en patrones funcionan así: un patrón es «lo que toca cada
instrumento durante estos compases», así que el patrón de batería tiene
filas vacías para el sintetizador.

El playlist son dos listas:

```rust
pub struct Track {
    pub name: String,
    pub muted: bool,
    // ...a display height, which older files leave out
}

pub struct Clip {
    pub pattern: usize,
    pub track: usize,
    pub start: usize,
    /// How many times the pattern plays, one after the other.
    pub repeats: usize,
}

pub struct Playlist {
    pub(crate) tracks: Vec<Track>,
    pub(crate) clips: Vec<Clip>,
    // ...sections, a loop region, and time, tempo, key and chord markers
}
```

Un clip son tres números y un contador de repeticiones: qué patrón, en qué
pista, empezando en qué paso de la canción y cuántas veces suena el patrón.
Fíjate en lo que *no* está: la duración del clip. Es la duración del patrón
por el número de repeticiones, así que se calcula cuando hace falta:

```rust
pub fn clip_length(&self, clip: &Clip) -> usize {
    self.patterns.get(clip.pattern).map_or(0, Pattern::length) * clip.repeats
}

impl Clip {
    /// A clip that plays its pattern once.
    pub fn new(pattern: usize, track: usize, start: usize) -> Self {
        Clip { pattern, track, start, repeats: 1 }
    }
}
```

Guardar también la duración supondría dos sitios que deben coincidir. Haz
el patrón más largo y la duración guardada de cada clip quedaría mal. Un
valor que se puede calcular a partir de otros casi siempre es mejor
calcularlo que guardarlo. Lo mismo vale para el final de un clip repetido:
es `start + clip_length`, no otro campo.

Los clips del paso 4 siempre reproducían su patrón una vez, y el hilo de
audio recibía los clips mismos. El paso 8 añadió repeticiones, y ahora
`Project::playback_arrangement` los desenrolla en `Play`s antes de que la
canción llegue al hilo de audio. El capítulo sobre [reproducir una
canción](song-playback.md#empezar-notas-en-modo-canción) tiene el fragmento
actual; el clip sigue siendo lo que dibujas, y su `repeats` es el pequeño
«×2» que lleva encima.

`map_or(0, Pattern::length)` dice «si hay un patrón, llama a
`Pattern::length` sobre él; si no, usa 0». Un método se puede pasar por
nombre, como una función, cuando su único argumento es `self`.

## Tres reglas, un guardián

El comentario de `Project` enumera las reglas que mantiene:

- cada patrón tiene una fila de notas por canal, en el mismo orden;
- siempre hay al menos un patrón;
- cada clip del playlist se refiere a un patrón y a una pista que existen.

Cada regla trata de dos listas a la vez, así que ninguna lista puede
mantenerla por sí sola. El `Project` las posee todas, mantiene los campos
privados y los cambia solo en métodos que respetan todas las reglas. Añadir
un canal añade una fila a cada patrón:

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
  puede ignorarlo, guardarlo para una función de deshacer más adelante, o
  comprobar que la eliminación llegó a ocurrir.

La prueba `removing_a_pattern_renumbers_its_followers` comprueba toda la
historia: tras borrar «Beat», los cinco clips de Beat desaparecen,
«Chords» es el patrón 0, y el fill sigue sonando en el compás 8.

¿Por qué usar posiciones, en lugar de dar a cada patrón un id permanente?
Los ids harían más simple la eliminación (nada que renumerar) pero todo lo
demás más difícil: cada búsqueda tendría que buscar un id en lugar de
indexar un `Vec`. Para una lista de como máximo 999 patrones (normalmente
muchos menos), las posiciones más un método cuidadoso son la opción más
simple. Es el tipo de intercambio sin una única respuesta correcta; el
[último ejercicio](exercises.md#7-piénsalo) de este paso te pide que
pienses en ello.

## Repartir `&mut Pattern` con seguridad

La app necesita cambiar notas de un patrón. `Project` podría copiar cada
método de `Pattern` (`toggle`, `add_note`, `replace_note`...) como método
propio, como hizo el paso 3 con el patrón único. En su lugar reparte el
patrón mismo:

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

En el lado de la app, cada edición de notas pasa por un pequeño ayudante:

```rust
fn edit_notes(&mut self, channel: usize, edit: impl FnOnce(&mut Pattern) -> bool) {
    let changed = self.project.pattern_mut(self.pattern).is_some_and(edit);
    if changed {
        self.send_notes(channel);
    }
}

// For example:
RackAction::ToggleStep { channel, step } => {
    self.edit_notes(channel, |pattern| pattern.toggle(channel, step).is_some());
}
```

- `impl FnOnce(&mut Pattern) -> bool` acepta cualquier closure que reciba
  un patrón y diga si cambió algo. `FnOnce` es el más permisivo de los tres
  traits de closures: el closure solo tiene que poder llamarse una vez, así
  que puede incluso mover valores que capturó (el closure de `AddNotes`
  del piano roll mueve su vector `notes` al loop).
- `is_some_and(edit)` llama a `edit` sobre el patrón si hay uno, y da
  `false` si no lo hay.

## Nombres, e `impl Into<String>`

Los patrones ahora tienen nombre. El constructor recibe el nombre primero:

```rust
pub fn new(name: impl Into<String>, channels: usize, length: usize) -> Self {
    Pattern {
        name: name.into(),
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
que no hay razón para esconderlo detrás de métodos.

Clonar un patrón usa el derive `Clone` que `Pattern` tiene desde el paso 2:

```rust
pub fn clone_pattern(&mut self, index: usize) -> Option<usize> {
    let mut copy = self.patterns.get(index)?.clone();
    copy.name = format!("{} copy", copy.name);
    self.push_pattern(copy)
}
```

`.clone()` copia el patrón entero, cada fila y cada nota, así que la copia
y el original nunca comparten nada; la prueba `cloned_patterns_are_independent`
comprueba que cambiar uno deja al otro en paz. El `?` después de
`get(index)` devuelve `None` de la función inmediatamente si no existe tal
patrón.

## Un duplicado menos

Guardar archivos (dos capítulos más adelante) encontró un agujero en las
reglas del paso 3. `add_note` se negaba a añadir una nota en el mismo lugar
que otra, pero `replace_note`, usado al arrastrar, no comprobaba, así que
podías arrastrar una nota exactamente encima de otra. El nuevo
`replace_note` comprueba cada nota excepto la que se está reemplazando:

```rust
let duplicate = row
    .iter()
    .enumerate()
    .any(|(other_index, other)| other_index != index && other.same_place(&note));
```

Ahora «no hay dos notas de la misma altura que empiecen en el mismo
momento» es cierto para cada patrón, y cargar un archivo puede insistir en
ello. El paso 8 dio después a las notas un `nudge` dentro de su paso, para
la interpretación importada y suelta; `same_place` compara la altura, el
paso y el nudge, así que dos notas que empiezan a unos pocos ticks de
distancia siguen siendo dos notas.

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

El paso 8 añadió secciones y compases; en el paso 4 la última línea era un
simple `round_up_to_bar(end)` sobre los clips solos. La regla es la misma:
la canción termina donde termina su última pieza, en una línea de compás.

## ¿Dónde está un patrón mientras suena la canción?

En modo canción, el hilo de audio informa de un paso de la *canción*. El
channel rack y el piano roll muestran un patrón, así que necesitan saber
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
    let end = clip.start + pattern_length * clip.repeats;
    (clip.start..end)
        .contains(&step)
        .then(|| (step - clip.start) % pattern_length)
}
```

Léelo como una frase: de los clips que se pueden oír, toma los de este
patrón, encuentra el primero que cubra este paso de la canción y dime
cuánto se adentra en el clip ese paso. El `% pattern_length` es lo que hace
que un clip repetido vuelva a empezar su patrón en cada vuelta: un clip de
16 pasos que se repite en el paso 16 de la canción está allí en el paso 0
del patrón. En el paso 4 cada clip sonaba una vez, así que el `%` no
estaba. `audible_clips` se salta los clips de pistas silenciadas:

```rust
pub fn audible_clips(&self) -> impl Iterator<Item = Clip> + '_ {
    self.clips
        .iter()
        .copied()
        .filter(|clip| self.tracks.get(clip.track).is_some_and(|track| !track.muted))
}
```

`.copied()` convierte un iterador de `&Clip` en uno de `Clip`. Funciona
porque `Clip` es `Copy` (tres números), y significa que quien llama recibe
valores simples en lugar de referencias al playlist.

## La canción de demostración

`Project::demo` construye la canción con los mismos métodos que usa la app:

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
contando desde 1, como hace la regla. El fill ocupa el lugar del beat en el
compás 8. Un proyecto nuevo (Archivo > Nuevo) empieza desde
`Project::template` en su lugar: los mismos cinco canales, un patrón vacío
y un playlist vacío.
