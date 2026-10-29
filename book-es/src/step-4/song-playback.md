# Reproducir una canción en el hilo de audio

Archivo: [`crates/lineup-audio/src/engine.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-audio/src/engine.rs)

El hilo de audio mantiene su propia copia de todo lo que reproduce, como
en los pasos 2 y 3, y las reglas no han cambiado: nada de reservar memoria,
nada de liberar, nada de locks. Lo nuevo es que hay más que copiar: las
notas de cada patrón, la duración de cada patrón y la canción.

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

El step sequencer del paso 2 no cambia en absoluto. Sigue contando pasos
de 0 hasta una cierta duración y repitiéndose en bucle. Solo cambia la duración:
la del patrón seleccionado en modo patrón, la de la canción en modo
canción.

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

Una canción vacía tendría duración 0, y un bucle de cero pasos no tiene
sentido, así que se convierte en un compás en silencio.

## Notas para cada patrón: `ChannelNotes`

Cada canal necesita ahora una lista de notas por patrón. Un `Vec` de
`Vec`s lo hace, pero hay una trampa. Cuando el usuario añade un patrón, el
hilo de audio debe añadir una lista vacía a cada canal:

```rust
channel.notes.push(Vec::new());
```

`Vec::new()` en sí es gratis: un `Vec` vacío todavía no tiene memoria
propia (Rust solo pide memoria cuando llega el primer elemento). Pero
`push` sobre el `Vec` *externo* reserva memoria si el `Vec` externo está
lleno. Para ir seguros, el `Vec` externo debe tener sitio para todos los
patrones que puedan existir, reservado antes de llegar al hilo de audio.

Esa es una regla sobre cómo se hizo el valor, y Rust puede imponer tales
reglas con un **newtype**, un struct con un único campo privado:

```rust
pub struct ChannelNotes(Vec<Vec<NoteEvent>>);

impl ChannelNotes {
    /// One list per pattern, in pattern order.
    pub fn new(lists: impl IntoIterator<Item = Vec<NoteEvent>>) -> Self {
        let mut patterns = Vec::with_capacity(MAX_PATTERNS);
        patterns.extend(lists.into_iter().take(MAX_PATTERNS));
        ChannelNotes(patterns)
    }

    /// Adds an empty list for a new pattern at `pattern`, without
    /// allocating. The lists from there on move up one place.
    fn insert_empty(&mut self, pattern: usize) {
        if self.0.len() < self.0.capacity() {
            self.0.insert(pattern.min(self.0.len()), Vec::new());
        }
    }
}
```

Como el campo es privado, el código fuera de `engine.rs` no puede escribir
`ChannelNotes(some_vec)`. La única forma de hacer uno es `ChannelNotes::new`,
que siempre reserva sitio para `MAX_PATTERNS` listas. Así que todo
`ChannelNotes` que recibe el hilo de audio tiene el sitio, y `insert_empty`
nunca puede reservar memoria. La comprobación `len < capacity` asegura aún
más: si algún día estuviera lleno, preferiría saltarse la inserción antes
que reservar memoria. Un patrón se inserta en una posición, no siempre al
final, porque el deshacer del paso 5 puede devolver uno al medio de la
lista.

`.0` es como se llega al campo de un tuple struct (uno cuyos campos no
tienen nombre). Una prueba clava la promesa:

```rust
#[test]
fn channel_notes_have_room_for_every_pattern() {
    let notes = ChannelNotes::new([Vec::new()]);
    assert!(notes.0.capacity() >= MAX_PATTERNS);
}
```

En el hilo de UI, enviar un canal construye uno con un iterador, una lista
por patrón:

```rust
let notes = ChannelNotes::new(
    self.project
        .patterns()
        .iter()
        .map(|pattern| pattern.notes(index).to_vec()),
);
```

`new` recibe `impl IntoIterator<Item = Vec<NoteEvent>>`, así que acepta
este iterador, un array como `[steps(&[0]), steps(&[2])]` en las pruebas,
o un `Vec` de listas.

## Los nuevos comandos

```rust
SetMode(PlayMode),
SelectPattern(usize),
InsertPattern { index: usize, length: usize },
RemovePattern(usize),
SetPatternLength { pattern: usize, length: usize },
SetSong { plays: Vec<Play>, length: usize },
SetNotes { pattern: usize, channel: usize, notes: Vec<NoteEvent> },
Clear,
```

`RemovePattern` saca la lista del patrón de cada canal. Las listas que
vienen detrás bajan un puesto, igual que `Project::remove_pattern` renumera
los clips, para que ambos lados sigan estando de acuerdo sobre lo que
significa «patrón 2». Cada lista retirada va a la cola de basura, para
liberarse en el hilo de UI:

```rust
Command::RemovePattern(pattern) => {
    if pattern < self.pattern_lengths.len() {
        self.pattern_lengths.remove(pattern);
        for index in 0..self.channels.len() {
            if let Some(notes) = self.channels[index].notes.remove(pattern) {
                self.throw_away(Garbage::Notes(notes));
            }
        }
        self.update_loop_length();
    }
}
```

El bucle cuenta con un índice en lugar de `for channel in &mut
self.channels`, porque el cuerpo llama a `self.throw_away`, que necesita
`&mut self`. Un bucle `for` sobre `&mut self.channels` mantendría un
préstamo mutable de una parte de `self` durante todo el bucle, y
`self.throw_away` necesitaría otro. Con un índice, cada préstamo de
`self.channels[index]` termina en la misma línea.

`SetSong` trae la duración de la canción junto con sus piezas. El hilo de
audio podría calcularla, pero `Project::song_length` ya lo hace, y dos
copias del mismo cálculo pueden acabar divergiendo. La UI envía una canción
nueva cada vez que un clip se mueve, una pista se silencia o cambia la
duración de un patrón.

**Cambiado más tarde.** En el paso 4 el hilo de audio recibía los clips
mismos (`SetSong { clips }`) y calculaba él mismo el paso del patrón de
cada clip. El paso 8 añadió clips que se repiten y secciones que se
repiten, así que `Project::playback_arrangement` desenrolla todo eso
*antes* de enviar. Las piezas son `Play`s: un patrón, un inicio en la
canción, cuánto adentrarse en el clip para empezar (`skip`, para un bucle
que empieza a mitad del clip) y una duración. El hilo de audio sigue sin
saber nada de repeticiones ni secciones; simplemente recorre las piezas
que le dan.

`Clear` vacía todo antes de que se envíe un proyecto nuevo, al abrir un
archivo:

```rust
while let Some(channel) = self.channels.pop() {
    self.throw_away(Garbage::Channel(channel));
}
self.pattern_lengths.clear();
let old = std::mem::take(&mut self.plays);
self.throw_away(Garbage::Song(old));
let old = std::mem::take(&mut self.song_tempos);
self.throw_away(Garbage::Tempos(old));
```

- `pop` saca el último elemento de un `Vec`, o devuelve `None` cuando está
  vacío, así que `while let Some(..)` se ejecuta hasta que no queda nada.
- `clear` sobre un `Vec<usize>` pone su longitud a cero pero conserva su
  memoria, así que aquí no se libera nada, y los `push` posteriores no
  reservan memoria.
- `std::mem::take` saca el valor y deja en su lugar el valor por defecto
  del tipo, aquí un `Vec` vacío (que, de nuevo, no posee memoria).

Cargar un proyecto tira hasta 32 canales, cada uno con su instrumento y
sus notas, así que la cola de basura creció de 256 plazas a 1024.

## Empezar notas en modo canción

Cuando se dispara un paso, el hilo de audio arranca las notas que empiezan
en él. En modo canción, un paso de la canción puede estar dentro de varios
clips a la vez, uno por pista, y cada clip tiene su propia idea de «qué
paso»:

```rust
PlayMode::Song => {
    // `self.plays` is read while `self.channels` is changed: two
    // different fields, so the borrows do not overlap.
    for play in &self.plays {
        let length = self.pattern_lengths.get(play.pattern).copied().unwrap_or(0);
        if let Some(local_step) = play.pattern_step(step, length) {
            // Notes stop at the end of the pattern, or of the piece,
            // whichever comes first.
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
}
```

Un clip que empieza en el paso 32 de la canción reproduce el paso 2 de su
patrón en el paso 34 de la canción. Si dos clips se solapan (el clip de Acordes y un clip de Ritmo, en dos pistas), ambos arrancan sus notas. Un clip
que repite su patrón es un `Play` largo, y `pattern_step` toma el paso de
la canción módulo la duración del patrón, de modo que el patrón vuelve a
pasar una y otra vez dentro de él.

`start_pattern_notes` es una función simple, no un método:

```rust
fn start_pattern_notes(
    channels: &mut [ChannelState],
    origin: Origin,
    room: usize,
    timing: Timing,
    pending: &mut Vec<Pending>,
    trace: Option<&mut Tracer>,
) { ... }
```

`Origin` agrupa los tres números que dicen de dónde vienen las notas: el
patrón, el paso del patrón y el paso de la canción. Los dos argumentos
extra son para el timing suelto (las notas que empiezan un poco tarde
esperan en `pending`) y para el trace. Si esto fuera un método que recibe
`&mut self`, llamarlo dentro de `for play in &self.plays` no compilaría: el
bucle presta `self.plays` para leer, y la llamada prestaría *todo* `self`
para cambiar. Como función que recibe `&mut self.channels`, presta solo ese
campo, que no se solapa con `self.plays`. Es la misma regla de «campos
disjuntos» del paso 3, usada a propósito: da forma a los argumentos de la
función para que preste solo lo que necesita.

`Timing` agrupa los números que siempre viajan juntos:

```rust
#[derive(Debug, Clone, Copy)]
struct Timing {
    step_frames: f64,
    sample_rate: u32,
    loose_frames: f64,
    now: u64,
}
```

Sin él, la función recibiría nueve argumentos, varios de ellos simples
números, y una llamada con cuatro números seguidos es fácil de poner en el
orden equivocado. (El lint `too_many_arguments` de Clippy se queja a partir
de ocho, y la legibilidad sufre antes de eso.) Los dos campos extra son
para el timing suelto: cuánto puede llegar tarde una nota, y el reloj del
hilo de audio en frames.

## Probarlo

Las pruebas ejecutan el hilo de audio sin tarjeta de sonido, como antes.
Un pequeño ayudante encuentra dónde empiezan los sonidos en un bloque de
muestras:

```rust
fn onsets(out: &[f32]) -> Vec<usize> {
    (0..out.len())
        .filter(|&i| out[i] != 0.0 && (i == 0 || out[i - 1] == 0.0))
        .collect()
}
```

El clic de la prueba dura 10 frames, así que cada golpe es una ráfaga
corta tras el silencio. Entonces una canción se puede comprobar en una
línea:

```rust
#[test]
fn song_mode_plays_the_clips() {
    let mut h = two_patterns();
    // Pattern 1 at bar 1, pattern 0 at bar 2: a two-bar song.
    let plays = vec![
        Play { pattern: 1, start: 0, skip: 0, length: 16 },
        Play { pattern: 0, start: 16, skip: 0, length: 16 },
    ];
    send(&mut h, Command::SetSong { plays, length: 32 });
    send(&mut h, Command::SetMode(PlayMode::Song));
    send(&mut h, Command::Play);
    let out = render(&mut h, 40 * STEP);
    // Step 2 of the first clip, step 0 of the second, then the song
    // loops back to the start: step 2 of the first clip again.
    assert_eq!(onsets(&out), vec![2 * STEP, 16 * STEP, 34 * STEP]);
    assert_eq!(h.playhead.load(Ordering::Relaxed), 7, "step 39 is step 7 of the second time round");
}
```

La primera versión de esta prueba esperaba el cursor de reproducción en 39 y fallaba:
después de 40 pasos de una canción de 32, el step sequencer está en su
segunda vuelta. La que estaba mal era la prueba, no el código, y el arreglo
fue decir por qué en el mensaje del assert. Una prueba que falla es una
pregunta, no siempre un bug.

## Comprobarlo de oído, sin oídos

Como en el paso 3, la salida de la app real se grabó a través de un
dispositivo de sonido de «archivo» y se midió compás a compás. Los
compases 1 y 2 casi no tenían energía por encima de 6 kHz (solo el
sintetizador), los compases 3 a 7 tenían unas cuatro veces más (llegaban
los hats y las palmas), el compás 8 tenía golpes en los pasos 12 a 15 (el
redoble de la caja), y el compás 9 sonaba otra vez como el compás 1: la
canción había hecho bucle.
