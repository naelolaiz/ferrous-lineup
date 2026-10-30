# El secuenciador: temporización a la muestra

Archivos: [`crates/lineup-core/src/sequencer.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/sequencer.rs)
y [`crates/lineup-audio/src/engine.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-audio/src/engine.rs)

El hilo de audio toca un patrón o la canción contando frames de audio.
Quien cuenta es un tipo pequeño de `lineup-core`, el **secuenciador**,
que no sabe nada de sonido: solo dice en qué frame empieza cada paso.
Todo lo que suena en un paso, las notas de cada canal en cada clip, es
cosa del hilo de audio, en `engine.rs`. Este capítulo sigue un paso
desde la cuenta de frames hasta las notas que empieza. Cómo funciona el
hilo de audio en sí está en [Dos hilos](../program/threads.md); cómo
doblan los pasos los cambios de tempo, los compases y el swing está en
[Tempo, compás, swing y sensación](tempo.md).

## La aritmética

Un paso es una semicorchea, y un pulso una negra, así que un pulso tiene
cuatro pasos (`STEPS_PER_BEAT`). A un tempo de *T* pulsos por minuto y
una frecuencia de muestreo de *R* frames por segundo, un pulso dura
`R × 60 / T` frames y un paso un cuarto de eso:

```text
frames por paso = R × 60 / T / 4

120 BPM, 48 kHz:  48000 × 60 / 120 / 4 = 6000 frames       (exacto)
130 BPM, 44.1 kHz: 44100 × 60 / 130 / 4 = 5088.46... frames (no entero)
```

```rust
pub fn frames_per_step(tempo: Tempo, sample_rate: u32) -> f64 {
    tempo.samples_per_beat(sample_rate) / STEPS_PER_BEAT as f64
}
```

Si 5088.46 se redondeara a 5088, cada paso se quedaría 0.46 frames
corto. Son unos 10 microsegundos, nada que fueras a oír una sola vez,
pero se acumula: al cabo de unos minutos la batería iría audiblemente
por delante de cualquier otra cosa que corra a 130 BPM. Por eso el
secuenciador conserva la fracción, en un `f64`.

## El tick

El hilo de audio llama a `tick` una vez por cada frame que produce:

```rust
pub fn tick(&mut self) -> Option<usize> {
    let fired = if self.frames_until_next <= 0.0 {
        let step = self.next_step;
        if self.coming_round {
            self.laps += 1;
        }
        self.next_step = (self.next_step + 1) % self.length;
        self.coming_round = self.next_step == 0;
        self.frames_until_next += self.frames_per_step;
        Some(step)
    } else {
        None
    };
    self.frames_until_next -= 1.0;
    fired
}
```

`frames_until_next` cuenta hacia atrás de uno en uno cada frame. Cuando
llega a cero, se dispara el siguiente paso y se *suma* al contador la
longitud de un paso entero, no se asigna. La fracción que sobrara se
queda en el contador, de modo que los límites de los pasos caen en 0,
5088.46, 10176.92... redondeados al frame más cercano cada vez, pero sin
derivar nunca. `%` es el operador de resto: devuelve el número de paso a
0 al final del bucle.

Como un paso empieza en el frame exacto donde empieza, y no al principio
del bloque de audio que lo contiene, la temporización no depende del
tamaño del búfer de la tarjeta de sonido. Eso es lo que significa
**exacto a la muestra**.

### Qué más puede decir el secuenciador

Otras partes del programa le preguntan al secuenciador algo más que «qué
paso suena ahora»:

- **`position`** da el paso con la fracción que ha pasado: 4.5 es la
  mitad del paso 4. La grabación la usa para poner una tecla tocada
  entre dos pasos en el más cercano ([MIDI](midi.md)).
- **`laps`** cuenta cuántas veces volvió el bucle al principio por
  seguir tocando después de su último paso. Un salto al principio no
  cuenta. El entrenador de velocidad acelera en cada vuelta
  ([Herramientas de práctica](../band/practice.md)).
- **`due`** dice qué paso empieza en el *próximo* tick, sin avanzar. El
  hilo de audio lo pregunta antes de `tick`, así que una canción cuyo
  tempo cambia puede poner el tempo nuevo antes de que empiece ese paso,
  y el paso entero dura lo que debe.
- **`start_at`** salta, de modo que el paso dado se dispara en el
  próximo tick; un paso más allá del final da la vuelta, como lo haría la
  reproducción.

`set_tempo` cambia la longitud de los pasos que vienen; el paso que ya
suena conserva los frames que le quedan. `set_length` vuelve al paso 0
si el siguiente paso queda más allá del nuevo final.

## Por qué vive en `lineup-core`

El secuenciador no tiene nada que ver con tarjetas de sonido. Solo
cuenta. Tenerlo en `lineup-core` significa que se prueba en
milisegundos, sin audio, y la cuenta previa usa otro propio.

## Probar con iteradores

```rust
fn run(sequencer: &mut Sequencer, frames: usize) -> Vec<(usize, usize)> {
    (0..frames)
        .filter_map(|frame| sequencer.tick().map(|step| (frame, step)))
        .collect()
}
```

Esta función auxiliar hace avanzar el secuenciador `frames` veces y
lista cada paso que se disparó junto con el frame en que se disparó:

- `0..frames` es un **rango**, que es un iterador sobre números.
- `filter_map` llama al closure para cada frame y se queda solo con los
  resultados `Some`, desenvueltos.
- `Option::map` convierte `Some(step)` en `Some((frame, step))`, una
  tupla.
- `collect()` reúne los resultados en un `Vec`. Rust deduce qué
  colección construir a partir del tipo de retorno.

Los tests comprueban luego números de frame exactos:

```rust
assert_eq!(
    fired,
    vec![(0, 0), (6_000, 1), (12_000, 2), (18_000, 3), (24_000, 0)]
);
```

y que un minuto a 130 BPM da exactamente 520 pasos (130 pulsos × 4), el
último a menos de un frame de donde lo ponen 519 pasos exactos. Los
guiones bajos en literales numéricos como `6_000` los ignora el
compilador y solo sirven para que los números grandes se lean mejor.

## Modo patrón y modo canción

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PlayMode {
    /// Loop the selected pattern, to work on it.
    #[default]
    Pattern,
    /// Play the playlist from the start, and loop at the end of the song.
    Song,
}
```

El secuenciador es el mismo en los dos modos. Cuenta pasos de 0 a cierta
longitud y repite. Solo cambia la longitud: la del patrón seleccionado
en modo patrón, la de la canción en modo canción.

```rust
fn update_loop_length(&mut self) {
    let length = match self.mode {
        PlayMode::Pattern => self.pattern_length(self.pattern),
        PlayMode::Song => self.song_length,
    };
    let length = if length == 0 { STEPS_PER_BAR } else { length };
    self.sequencer.set_length(length);
}
```

Una canción vacía tendría longitud 0, y un bucle de cero pasos no tiene
sentido, así que se convierte en un compás en silencio.

La canción llega como `Command::SetSong { plays, length }`: los trozos
de patrones que hay que tocar, con cada repetición ya desplegada, y la
longitud. El hilo de audio podría calcular la longitud, pero `Project`
ya lo hace, y dos copias de un mismo cálculo pueden separarse. Qué es un
`Play`, y cómo los construye el proyecto, está en [Secciones,
repeticiones y bucles](../program/project.md#secciones-repeticiones-y-bucles).

## Notas para cada patrón: `ChannelNotes`

Cada canal necesita una lista de notas por patrón. Un `Vec` de `Vec`s lo
resuelve, pero hay una trampa. Cuando se añade un patrón, el hilo de
audio tiene que añadir una lista vacía a cada canal. `Vec::new()` en sí
no cuesta nada: un `Vec` vacío todavía no tiene memoria propia. Pero
añadirlo al `Vec` *exterior* reserva memoria si el `Vec` exterior está
lleno. Para ir sobre seguro, el `Vec` exterior tiene que tener sitio para
todos los patrones que pueda llegar a haber, reservado antes de llegar al
hilo de audio.

Esa es una regla sobre cómo se construyó el valor, y Rust puede hacer
cumplir reglas así con un **newtype**, un struct con un único campo
privado:

```rust
pub struct ChannelNotes(Vec<Vec<NoteEvent>>);

impl ChannelNotes {
    /// One list per pattern, in pattern order.
    pub fn new(lists: impl IntoIterator<Item = Vec<NoteEvent>>) -> Self {
        let mut patterns = Vec::with_capacity(MAX_PATTERNS);
        patterns.extend(lists.into_iter().take(MAX_PATTERNS));
        ChannelNotes(patterns)
    }

    /// Adds an empty list for a new pattern at position `pattern`, without
    /// allocating. The lists from there on move up one place.
    fn insert_empty(&mut self, pattern: usize) {
        if self.0.len() < self.0.capacity() {
            self.0.insert(pattern.min(self.0.len()), Vec::new());
        }
    }
}
```

Como el campo es privado, el código de fuera de `engine.rs` no puede
escribir `ChannelNotes(some_vec)`. La única forma de crear uno es
`ChannelNotes::new`, que siempre reserva sitio para `MAX_PATTERNS`
listas. Así que cada `ChannelNotes` que recibe el hilo de audio tiene el
sitio, e `insert_empty` nunca puede reservar memoria. La comprobación
`len < capacity` lo asegura del todo: si alguna vez estuviera lleno,
preferiría saltarse la inserción antes que reservar memoria. `.0` es la
forma de llegar al campo de un struct tupla (uno cuyos campos no tienen
nombre). El test `channel_notes_have_room_for_every_pattern` deja fijada
la promesa.

Del lado de la ventana, enviar un canal construye uno con un iterador,
una lista por patrón:

```rust
// The notes as they are played, with the song's swing.
let notes = ChannelNotes::new(
    (0..self.project.patterns().len())
        .map(|pattern| self.project.played_notes(pattern, index).into_owned()),
);
```

`new` recibe `impl IntoIterator<Item = Vec<NoteEvent>>`, así que acepta
este iterador, un array como `[steps(&[0]), steps(&[2])]` en los tests,
o un `Vec` de listas. Las notas son `played_notes`, no las escritas: con
swing, la segunda corchea de cada pulso llega tarde ([Tempo, compás,
swing y sensación](tempo.md#swing)).

## Los patrones van y vienen

`RemovePattern` saca la lista del patrón de cada canal. Las listas que
vienen después bajan un lugar, exactamente como `Project::remove_pattern`
renumera los clips, así que los dos lados siguen de acuerdo en qué
significa «el patrón 2». Cada lista quitada va a la cola de basura, para
liberarla en el hilo de la ventana:

```rust
fn remove_pattern(&mut self, pattern: usize) {
    if pattern < self.pattern_lengths.len() {
        self.pattern_lengths.remove(pattern);
        for index in 0..self.channels.len() {
            if let Some(notes) = self.channels[index].notes.remove(pattern) {
                self.throw_away(Garbage::Notes(notes));
            }
        }
        self.update_loop_length();
        ...
    }
}
```

El bucle cuenta con un índice en lugar de `for channel in &mut
self.channels`, porque el cuerpo llama a `self.throw_away`, que necesita
`&mut self`. Un bucle `for` sobre `&mut self.channels` mantendría un
préstamo mutable de una parte de `self` durante todo el bucle, y
`self.throw_away` necesitaría otro. Con un índice, cada préstamo de
`self.channels[index]` termina en la misma línea.

`Clear` lo vacía todo antes de enviar un proyecto nuevo, al abrir un
archivo:

```rust
while let Some(mut channel) = self.channels.pop() {
    silence(&mut channel.instrument, &mut self.midi_out, &mut self.midi_backlog);
    self.throw_away(Garbage::Channel(channel));
}
// `clear` empties the list but keeps its memory: nothing is freed.
self.pattern_lengths.clear();
let old = std::mem::take(&mut self.plays);
self.throw_away(Garbage::Song(old));
```

- `pop` saca el último elemento de un `Vec`, o devuelve `None` cuando
  está vacío, así que `while let Some(..)` sigue hasta que no queda
  nada.
- `silence` le dice a un sintetizador externo que suelte las notas que
  tiene un canal MIDI Out, porque un sintetizador solo para una nota
  cuando se lo dicen.
- `clear` sobre un `Vec<usize>` pone su longitud a cero pero conserva su
  memoria, así que aquí no se libera nada, y los `push` posteriores no
  reservan memoria.
- `std::mem::take` saca el valor y deja en su lugar el valor por defecto
  del tipo, aquí un `Vec` vacío (que no posee memoria).

Abrir un proyecto tira hasta `MAX_CHANNELS` (32) canales, cada uno con
su instrumento y sus notas, y la canción, sus tempos y los golpes de su
metrónomo, y por eso la cola de basura tiene 1024 lugares.

## Empezar las notas de un paso

Cuando se dispara un paso, el hilo de audio empieza las notas que
comienzan en él. En modo patrón es un patrón. En modo canción, un paso de
la canción puede estar dentro de varios clips a la vez, uno por pista, y
cada clip tiene su propia idea de «qué paso»:

```rust
PlayMode::Song => {
    // `self.plays` is read while `self.channels` is changed: two
    // different fields, so the borrows do not overlap.
    for play in &self.plays {
        let length = self.pattern_lengths.get(play.pattern).copied().unwrap_or(0);
        if let Some(local_step) = play.pattern_step(step, length) {
            // Notes stop at the end of the pattern, or of the
            // piece, whichever comes first.
            let room = (length - local_step).min(play.end() - step);
            start_pattern_notes(
                &mut self.channels,
                Origin {
                    pattern: play.pattern,
                    pattern_step: local_step,
                    song_step: step,
                    over_song: false,
                },
                room,
                timing,
                &mut self.pending,
                self.trace.as_deref_mut(),
            );
        }
    }
    ...
}
```

Un clip que empieza en el paso 32 de la canción toca el paso 2 de su
patrón en el paso 34 de la canción. Si dos clips se solapan (un clip de
acordes y uno de ritmo, en dos pistas), los dos empiezan sus notas. Un
clip que repite su patrón es un solo `Play` largo, y `pattern_step` toma
el paso de la canción módulo la longitud del patrón, así que el patrón
vuelve a empezar una y otra vez dentro de él.

### Tomar prestado solo lo necesario

`start_pattern_notes` es una función normal, no un método:

```rust
fn start_pattern_notes(
    channels: &mut [ChannelState],
    origin: Origin,
    room: usize,
    timing: Timing,
    pending: &mut Vec<Pending>,
    mut trace: Option<&mut Tracer>,
) { ... }
```

Si fuera un método que recibe `&mut self`, llamarlo dentro de
`for play in &self.plays` no compilaría: el bucle toma prestado
`self.plays` para leer, y la llamada tomaría prestado *todo* `self` para
cambiarlo. Como función que recibe `&mut self.channels`, toma prestado
solo ese campo, que no se solapa con `self.plays`. Dar forma a los
argumentos de una función para que tome prestado solo lo que necesita es
una forma habitual de trabajar con el comprobador de préstamos en lugar
de contra él.

`Origin` dice de dónde vienen las notas: el patrón, el paso del patrón,
el paso de la canción, y si vienen de un patrón tocado sobre la canción
(más abajo). Cada voz guarda el origen de la nota que toca, y así el
hilo de audio sabe después si la canción todavía la toca.

`Timing` agrupa los números que siempre viajan juntos:

```rust
/// How long a step is, the sample rate the instruments run at, how
/// loose the timing is, and the frame the step starts on.
#[derive(Debug, Clone, Copy)]
struct Timing {
    step_frames: f64,
    sample_rate: u32,
    loose_frames: f64,
    now: u64,
}
```

Sin él, la función recibiría nueve argumentos, varios de ellos números
sueltos, y una llamada con cuatro números seguidos es fácil de escribir
en el orden equivocado. (El lint `too_many_arguments` de Clippy protesta
a partir de ocho, y la legibilidad sufre antes.)

### Cuánto se sostiene una nota

La longitud de una nota está en pasos; el instrumento quiere frames. El
hilo de audio la calcula cuando empieza la nota, y la llama **gate**:

```rust
let tick = timing.step_frames / TICKS_PER_STEP as f64;
let held = if note.length <= room {
    note.length as f64 * timing.step_frames - f64::from(note.nudge + note.trim) * tick
} else {
    room as f64 * timing.step_frames - f64::from(note.nudge) * tick
};
let gate = held as u32;
```

Una nota que se pasa del final de su patrón, o del trozo de la canción
que la toca, se corta ahí (`room`), para que el último acorde de un
patrón no quede colgando sobre el clip siguiente. `nudge` y `trim` son
fracciones de paso, en ticks: una nota de una interpretación MIDI puede
empezar un poco después de su paso y acabar un poco antes del último.
Con la temporización suelta, cada nota además empieza un poco tarde a
propósito. Una nota que empieza tarde espera en `pending`, una lista
reservada de antemano, hasta que llega su frame. Las dos cosas están en
[Tempo, compás, swing y sensación](tempo.md).

## Cuando la canción cambia bajo una nota que suena

La canción puede cambiar mientras suena: se mueve o se borra un clip, se
silencia una pista, la banda escribe patrones nuevos. Una nota que ya
empezó sonaría hasta el final de su gate, aunque la canción ya no la
toque. Así que cuando llega una canción nueva mientras se toca, el hilo
de audio suelta las notas que ya no toca:

```rust
fn release_notes_left_out(&mut self) {
    // The fields are borrowed one by one, so the closure can read the
    // song while the channels are changed.
    let plays = &self.plays;
    let lengths = &self.pattern_lengths;
    let still_played = |origin: Origin| {
        if origin.over_song {
            return true;
        }
        let length = lengths.get(origin.pattern).copied().unwrap_or(0);
        plays.iter().any(|play| {
            play.pattern == origin.pattern
                && play.pattern_step(origin.song_step, length) == Some(origin.pattern_step)
        })
    };
    for channel in &mut self.channels {
        channel.instrument.release_unless(still_played);
    }
    ...
}
```

Una nota se sigue tocando si algún trozo de la canción nueva toca el
mismo paso del mismo patrón en el mismo paso de la canción.
`release_unless` recibe la prueba como closure (`impl Fn(Origin) -> bool`),
así que cada clase de instrumento puede soltar a su manera: una voz del
sintetizador pasa a su release, un sampler apaga su sonido rápidamente,
un MIDI Out envía un note-off. Las teclas pulsadas en un teclado y las
previsualizaciones no tienen origen, así que nunca se cortan.
`notes_stop_when_the_song_no_longer_plays_them` lo comprueba.

El closure toma prestados `self.plays` y `self.pattern_lengths`, y el
bucle toma prestado `self.channels` para cambiarlo. Está permitido
porque son campos distintos: desde la edición 2021, un closure que usa
`self.plays` captura solo ese campo, no todo `self`, así que escribir
`self.plays` dentro del closure también funcionaría. Las dos referencias
locales solo dejan claro qué partes de `self` lee.

## Saltar, y cambiar de modo

`Command::Seek(step)` mueve la posición de inicio. Con la reproducción
parada, es donde empieza el siguiente `Play`. Mientras suena, la
reproducción salta allí en el acto:

```rust
fn seek(&mut self, step: usize) {
    self.start = step;
    self.pending.clear();
    ...
    if self.playing {
        // Notes that were playing would otherwise hang on
        // until the end they will now never reach.
        for channel in &mut self.channels {
            channel.instrument.release_all();
        }
        ...
        self.sequencer.start_at(step);
    }
}
```

Las notas que esperan en `pending` son de la posición anterior, así que
se descartan. Cambiar entre modo patrón y modo canción empieza el bucle
nuevo desde arriba, y suelta las notas sostenidas por la misma razón.
Parar suelta todas las notas, también las teclas pulsadas: cada una pasa
a su release, así que el sonido se apaga en un momento y no queda
colgando.

## Un patrón sobre la canción

**▶ Tocar sobre la canción**, en el rack de canales, toca el patrón
seleccionado sobre la canción mientras suena, para probarlo con el resto
de la banda (`Command::SetOverlay`). Empieza en la siguiente línea de
compás de la canción, para entrar a tiempo, y se repite mientras suena:

```rust
fn overlay_step(&mut self, step: usize) -> Option<(usize, usize)> {
    let bar_line = click_at(&self.song_clicks, step) == Click::Bar;
    let overlay = self.overlay.as_mut()?;
    // After a jump it waits for a bar line, to start again from its
    // top in time with the bars.
    if overlay.last.is_some_and(|last| step != last + 1)
        && let Some((pattern, _)) = overlay.playing.take()
    {
        overlay.coming.get_or_insert(pattern);
    }
    overlay.last = Some(step);
    if bar_line && let Some(pattern) = overlay.coming.take() {
        overlay.playing = Some((pattern, 0));
    }
    let (pattern, at) = overlay.playing?;
    let length = self.pattern_lengths.get(pattern).copied().unwrap_or(0);
    if length == 0 {
        return None;
    }
    let at = at % length;
    overlay.playing = Some((pattern, at + 1));
    Some((pattern, at))
}
```

Las líneas de compás salen de los golpes del metrónomo de la canción, que
ya dicen dónde empieza cada compás en cualquier compás. Un patrón
elegido mientras otro suena sobre la canción espera como `coming` hasta
la siguiente línea de compás. Un salto (un seek, o la canción que vuelve
a empezar) se nota porque el paso no es el siguiente al último, y el
patrón vuelve a empezar desde arriba en la siguiente línea de compás.

La ventana dibuja un segundo cursor en el rack para el patrón que suena
sobre la canción. Dos números, el patrón y su paso, tienen que leerse
juntos, o la ventana podría ver un patrón nuevo con un paso viejo. Así
que comparten un único `AtomicU64`, el patrón en los 32 bits altos y el
paso en los 32 bajos:

```rust
fn store_overlay(overlay: &AtomicU64, playing: Option<(usize, usize)>) {
    let bits = playing.map_or(NO_POSITION, |(pattern, step)| {
        ((pattern as u64) << 32) | step as u64
    });
    overlay.store(bits, Ordering::Relaxed);
}
```

`<<` desplaza los bits del patrón 32 lugares hacia arriba, y `|` (o bit
a bit) pone el paso en la mitad baja, que está vacía. Leer lo deshace con
`>> 32` y una máscara, `bits & u64::from(u32::MAX)`. Una carga, un
guardado: un entero atómico no se puede leer mitad viejo y mitad nuevo.
Un valor especial, `NO_POSITION`, significa «nada suena sobre la
canción».

## Cómo se prueba

Los tests hacen funcionar el hilo de audio sin tarjeta de sonido (mira
[Probar sin tarjeta de sonido](../program/threads.md#probar-sin-tarjeta-de-sonido)).
Una pequeña función auxiliar encuentra dónde empiezan los sonidos en un
bloque de muestras:

```rust
fn onsets(out: &[f32]) -> Vec<usize> {
    (0..out.len())
        .filter(|&i| out[i] != 0.0 && (i == 0 || out[i - 1] == 0.0))
        .collect()
}
```

El clic de prueba es una ráfaga corta después de silencio, así que cada
golpe es un inicio. Entonces una canción se comprueba en una línea:

```rust
#[test]
fn song_mode_plays_the_clips() {
    let mut h = two_patterns();
    // Pattern 1 at bar 1, pattern 0 at bar 2: a two-bar song.
    let plays = vec![whole(1, 0), whole(0, 16)];
    send(&mut h, Command::SetSong { plays, length: 32 });
    send(&mut h, Command::SetMode(PlayMode::Song));
    send(&mut h, Command::Play);
    let out = render(&mut h, 40 * STEP);
    // Step 2 of the first clip, step 0 of the second, then the song
    // loops back to the start: step 2 of the first clip again.
    assert_eq!(onsets(&out), vec![2 * STEP, 16 * STEP, 34 * STEP]);
    assert_eq!(
        h.playhead.load(Ordering::Relaxed),
        7,
        "step 39 is step 7 of the second time round"
    );
}
```

La última aserción lleva un mensaje que dice *por qué* 7 es correcto:
después de 40 pasos de una canción de 32, el secuenciador va por su
segunda vuelta. Quien espere 39 ve la razón en cuanto el test falla.

Otros tests cubren el resto del capítulo: clips que se solapan, trozos
que repiten su patrón, patrones insertados y quitados debajo de los
demás, `Clear` mandándolo todo a la basura, `Play` empezando donde lo
puso un seek, y siete tests sobre un patrón tocado sobre la canción.
