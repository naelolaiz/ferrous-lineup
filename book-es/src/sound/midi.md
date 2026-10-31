# MIDI: mensajes, dispositivos, grabación y archivos

Archivos: [`crates/lineup-core/src/midi.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/midi.rs),
[`crates/lineup-core/src/midi_file.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/midi_file.rs),
[`crates/lineup-core/src/midi_file/read.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/midi_file/read.rs),
[`crates/lineup-core/src/midi_file/import.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/midi_file/import.rs),
[`crates/lineup-core/src/record.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/record.rs),
[`crates/lineup-audio/src/engine.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-audio/src/engine.rs),
[`crates/lineup-audio/src/instrument.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-audio/src/instrument.rs),
[`crates/lineup-app/src/midi.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/midi.rs),
[`crates/lineup-app/src/app/live.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/live.rs)
y [`crates/lineup-app/src/channel_rack.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/channel_rack.rs)

MIDI es como se hablan los teclados, los sintetizadores y los programas
de música. Es el mismo desde 1983: unos pocos bytes que dicen «esta
tecla bajó, con esta fuerza» o «esta tecla subió». Ferrous Lineup lo
habla en los dos sentidos:

- las teclas de un **teclado MIDI**, o del propio teclado del equipo
  dispuesto como un piano, tocan el *canal seleccionado*, el que
  muestra el piano roll, y la barra de estado nombra el acorde que
  forman;
- **grabar** convierte las teclas tocadas mientras un patrón se repite
  en notas de ese patrón;
- la **salida MIDI** envía canales a un sintetizador externo en lugar
  de a los altavoces: todos a la vez, o los que elijas;
- los **archivos MIDI** (`.mid`) se escriben a partir de una canción o
  de un patrón, y se leen de vuelta en un proyecto nuevo.

Cómo se usa todo esto está en las páginas del manual
[Teclados y sintetizadores MIDI](https://naelolaiz.github.io/ferrous-lineup/es/manual/midi.html),
[Abrir archivos MIDI y grabaciones](https://naelolaiz.github.io/ferrous-lineup/es/manual/importing.html)
y [Exportar](https://naelolaiz.github.io/ferrous-lineup/es/manual/exporting.html).
Este capítulo sigue a una tecla en su camino: los bytes, los hilos que
cruza, la nota en que se convierte y el archivo que la guarda.

Cuatro hilos se reparten el trabajo. Una tecla de un teclado MIDI hace
este camino:

```text
  teclado MIDI
     │ 3 bytes
     ▼
  hilo de midir: on_message ──▶ LiveInput (ring buffer) ──▶ hilo de audio ──▶ tarjeta de sonido
     │                                                          │
     │ una copia, con la posición (canal mpsc)                  │ mensajes de los canales de salida MIDI
     ▼                                                          ▼
  hilo de la interfaz: teclas pulsadas, acorde, grabación   MidiOutput (ring buffer)
     │                                                          │
     │ teclado del equipo: Command::Live ──▶ hilo de audio      ▼
                                                    hilo de envío ──▶ puerto MIDI ──▶ sintetizador
```

Cuatro ideas sostienen el diseño:

1. **Las teclas se saltan la interfaz.** La interfaz dibuja una vez por
   frame, y una tecla podría esperar allí un frame entero antes de que
   pase nada. Así que una tecla va del hilo MIDI directamente al hilo de
   audio, y suena dentro de un bloque de audio, unos pocos
   milisegundos. La interfaz recibe una copia, para mostrarla y
   grabarla.
2. **El hilo de audio sigue sin esperar nunca.** Enviar un mensaje MIDI
   es una llamada al sistema, que puede tardar lo que sea. Así que el
   hilo de audio deja los mensajes en una cola, y un hilo propio los
   envía.
3. **Un solo reloj para todo.** El hilo de audio publica por dónde va
   en el patrón, hasta una fracción de paso. Cada tecla se marca con
   esa posición al llegar, y grabar solo tiene que redondear.
4. **Un archivo se lee como datos que pueden estar mal.** Un archivo
   MIDI de internet puede estar cortado o dañado. Cada lectura puede
   fallar sin un panic, y la importación se queda con lo que pudo leer
   y lo dice.

## Tres bytes por tecla

Un teclado MIDI dice casi todo con mensajes de tres bytes. El primero,
el *byte de estado*, dice qué clase de mensaje es y en cuál de los 16
*canales*. Los otros dos son *bytes de datos*. Para una tecla que baja:

```text
  1001 0000   0011 1100   0110 0100
  │    │      │           │
  │    │      │           └ velocidad 100: con qué fuerza se pulsó la tecla
  │    │      └ nota 60: Do central (Do4)
  │    └ canal 0 (la gente lo llama canal 1)
  └ 1001 = note-on
```

Un byte de estado siempre tiene el bit superior a 1 (de 128 en
adelante), y un byte de datos nunca (de 0 a 127). Por eso un número de
nota MIDI y una velocidad nunca pasan de 127, y por eso `Note` usa los
mismos números: Do4 es 60 en Ferrous Lineup porque es 60 en MIDI.

Hay muchas clases de mensaje: potenciómetros y pedales («control
change»), pitch bends, tics de reloj para mantener las máquinas a
tiempo. La aplicación necesita tres:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MidiMessage {
    NoteOn {
        channel: u8,
        pitch: Note,
        /// How hard the key was struck, 1 to 127.
        velocity: u8,
    },
    NoteOff {
        channel: u8,
        pitch: Note,
    },
    /// From now on, `channel` plays General MIDI program `program`, 0 to
    /// 127. Only sent, never read: a keyboard choosing its own sound does
    /// not change the channel it plays.
    ProgramChange {
        channel: u8,
        program: u8,
    },
}

/// The kinds of message, as the top four bits of the status byte.
const NOTE_OFF: u8 = 0x80;
const NOTE_ON: u8 = 0x90;
const CONTROL_CHANGE: u8 = 0xB0;
const PROGRAM_CHANGE: u8 = 0xC0;
```

El byte de estado guarda dos cosas a la vez, así que el código lo
divide con operaciones de bits, las mismas que usan los
[conjuntos de clases de altura](../theory/sets.md): `status & 0xF0` se
queda con los cuatro bits superiores, la clase, y `status & 0x0F` con
los cuatro inferiores, el canal.

### Leer un mensaje: patrones de slice

```rust
pub fn parse(bytes: &[u8]) -> Option<MidiMessage> {
    let [status, pitch, velocity, ..] = *bytes else {
        return None;
    };
    // Data bytes have the top bit clear; anything else is malformed.
    if pitch > 127 || velocity > 127 {
        return None;
    }
    let channel = status & 0x0F;
    let pitch = Note::new(pitch);
    match status & 0xF0 {
        NOTE_ON if velocity > 0 => Some(MidiMessage::NoteOn {
            channel,
            pitch,
            velocity,
        }),
        NOTE_ON | NOTE_OFF => Some(MidiMessage::NoteOff { channel, pitch }),
        _ => None,
    }
}
```

`let [status, pitch, velocity, ..] = *bytes else` es un **patrón de
slice**. Un slice puede tener cualquier longitud, así que un patrón
para él describe una *forma*: «al menos tres elementos; llama a los tres
primeros `status`, `pitch` y `velocity`, y `..` para lo que siga». Si el
slice es más corto, el patrón no encaja y el bloque `else` retorna. Sin
patrones de slice, esto sería una comprobación de longitud y tres
indexaciones, cada una de las cuales podría entrar en pánico si la
comprobación estuviera mal. El `*` encaja con el slice mismo, no con la
referencia a él; se permite porque las partes que se sacan son `u8`,
que son `Copy`.

El `match` usa dos cosas más. `NOTE_ON` y `NOTE_OFF` son constantes, y
una constante en un patrón significa «igual a este valor», no «una
variable nueva». Y `NOTE_ON if velocity > 0` tiene una **guarda de
match**: el brazo solo encaja cuando además se cumple la condición. Un
note-on con velocidad 0 cae al brazo siguiente y se convierte en un
note-off. Muchos teclados envían de verdad eso en lugar de un note-off:
una serie de mensajes de la misma clase puede omitir el byte de estado
repetido («running status»), lo que ahorraba tiempo en los cables
lentos de 1983.

`parse` solo lee notas. Los program changes solo se envían, y cualquier
otro mensaje, desde un pedal de sustain hasta un tic de reloj, da
`None`.

### Escribir uno: dos o tres bytes

Una nota son tres bytes, un program change dos. `bytes` devuelve un
valor pequeño que guarda tres y dice cuántos cuentan:

```rust
/// The bytes of one message: two or three of them, as a slice.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MidiBytes {
    bytes: [u8; 3],
    len: usize,
}

impl std::ops::Deref for MidiBytes {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        &self.bytes[..self.len]
    }
}
```

**`Deref`** es el trait detrás del operador `*`. Implementarlo dice «un
`MidiBytes` se puede usar como un `[u8]`»: `*bytes` da el slice, y
también lo da pasar `&bytes` donde se espera un `&[u8]`, porque Rust
sigue `Deref` por su cuenta ahí (la *coerción por deref*). Así, el
código que envía puede pasar `&message.bytes()` a un puerto, un test
puede comparar `*off.bytes()` con `[0x80, 60, 64]`, y no se reserva
memoria: los tres bytes viven en el valor, en la pila.

```rust
pub fn bytes(self) -> MidiBytes {
    let (bytes, len) = match self {
        MidiMessage::NoteOn {
            channel,
            pitch,
            velocity,
        } => (
            [
                NOTE_ON | (channel & 0x0F),
                pitch.midi(),
                velocity.clamp(1, 127),
            ],
            3,
        ),
        // Velocity 64 is the usual "no particular speed" for a release.
        MidiMessage::NoteOff { channel, pitch } => {
            ([NOTE_OFF | (channel & 0x0F), pitch.midi(), 64], 3)
        }
        MidiMessage::ProgramChange { channel, program } => {
            ([PROGRAM_CHANGE | (channel & 0x0F), program & 0x7F, 0], 2)
        }
    };
    MidiBytes { bytes, len }
}
```

El `clamp(1, 127)` importa. Un note-on con velocidad 0 lo leería el
sintetizador como un note-off, así que una nota tocada «lo más suave
posible» no sonaría en absoluto. `& 0x0F` y `& 0x7F` evitan que un
número de canal o de programa equivocado se derrame en los bits de otro
campo.

Hay un mensaje que se escribe y nunca se lee: el control change 123,
«All Notes Off». Le dice a un sintetizador que pare todas las notas de
un canal:

```rust
pub fn all_notes_off(channel: u8) -> [u8; 3] {
    [CONTROL_CHANGE | (channel & 0x0F), 123, 0]
}
```

La aplicación lo envía en los 16 canales cada vez que suelta un
sintetizador, porque un note-off perdido por el camino dejaría una nota
sonando para siempre: cuando se elige otro puerto de salida, y cuando
desaparece el hilo de audio que debía los note-offs
([Dos hilos](../program/threads.md#cuando-la-tarjeta-de-sonido-desaparece)).

## El camino de entrada de una tecla

La mayoría de las notas vienen del secuenciador, que vive en el hilo de
audio y conoce la duración de cada nota antes de que empiece. Una tecla
de un teclado es distinta en dos cosas: viene de fuera, en un hilo que
no es de Ferrous Lineup, y nadie sabe cuánto tiempo se mantendrá
pulsada.

### Por qué las teclas se saltan la interfaz

El camino obvio para una tecla sería el que siguen todos los demás
cambios: la interfaz se entera y envía un `Command` al hilo de audio.
Pero la interfaz solo se ejecuta una vez por frame. A 60 frames por
segundo eso son hasta 17 ms de espera, y en un equipo lento mucho más,
antes de que el hilo de audio se entere siquiera de la tecla. Un
pianista lo nota.

El hilo de audio se ejecuta mucho más a menudo: una vez por *bloque*,
cada pocos milisegundos. Así que una tecla de un teclado MIDI va
directamente allí, y la interfaz recibe una copia después, solo para
mostrarla y grabarla:

```text
  hilo de midir ──▶ LiveInput ──▶ hilo de audio          (la toca)
        └────────▶ canal mpsc ──▶ hilo de la interfaz     (la muestra y la graba)
```

El teclado del equipo no puede hacer esto: egui lee el teclado, y egui
se ejecuta en el hilo de la interfaz. Así que las teclas escritas
toman el camino lento, como un `Command::Live(MidiMessage)`. En el hilo
de audio las dos acaban en el mismo método, `play_live`.

### Un productor por cola: `LiveInput`

La cola de comandos es un ring buffer del crate `rtrb`
([Dos hilos](../program/threads.md#enviar-comandos-con-un-ring-buffer)),
y su `Producer` pertenece al `AudioEngine`, en el hilo de la interfaz.
rtrb es de *un productor y un consumidor*: un hilo mete, un hilo saca,
y eso es lo que le permite funcionar sin cerrojos. El hilo de midir es
un segundo hilo que quiere meter, así que necesita un ring buffer
propio:

```rust
pub struct LiveInput {
    notes: rtrb::Producer<MidiMessage>,
    position: Arc<AtomicU64>,
}

impl LiveInput {
    /// Plays (or lets go of) a key on the live channel. Returns `false`
    /// if the queue was full and the key was lost.
    pub fn send(&mut self, message: MidiMessage) -> bool {
        self.notes.push(message).is_ok()
    }

    /// Where playback is, in steps (see [`Sequencer::position`]), or
    /// `None` when stopped. Recording uses it to place the key.
    pub fn position(&self) -> Option<f64> {
        load_position(&self.position)
    }
}
```

Como solo puede haber un productor, esto no puede ser un método de
`AudioEngine` que cualquiera pueda llamar. Es un *valor*, y quien lo
posee es el único productor. El motor lo crea al arrancar y lo guarda
en un `Option` hasta que alguien lo pide:

```rust
pub fn take_live_input(&mut self) -> Option<LiveInput> {
    self.live_input.take()
}
```

`Option::take` saca el valor y deja `None` en su lugar, así que la
segunda llamada no recibe nada. El sistema de tipos hace imposible
romper la regla «un productor»: `LiveInput` no implementa `Clone`, así
que no se puede copiar, solo mover. `MidiOutput`, la salida de los
canales de salida MIDI, funciona igual desde el otro extremo: guarda el
`Consumer` de la cola, y `take_midi_output` lo entrega una vez.

En el hilo de audio, el consumidor también es un `Option`, porque el
hilo de audio que [exporta una canción](export.md) no tiene
teclado:

```rust
while let Some(message) = self.live_input.as_mut().and_then(|keys| keys.pop().ok()) {
    self.play_live(message);
}
```

Esa línea empaqueta tres pasos. `self.live_input` es un
`Option<Consumer>`, y llamar a `pop` necesita un `&mut Consumer`.
`as_mut()` convierte `&mut Option<Consumer>` en `Option<&mut Consumer>`,
prestando lo de dentro en lugar de sacarlo. `and_then` ejecuta el
closure solo si hay consumidor, y el `Option` del propio closure (de
`pop().ok()`: `Some(key)`, o `None` cuando la cola está vacía) es el
resultado. Así que el bucle sigue mientras haya consumidor *y* tenga
una tecla.

### Dónde en el patrón: la posición

Grabar necesita saber *cuándo* llegó cada tecla, como un lugar en el
patrón: «a mitad del paso 4». El secuenciador lo sabe, porque cuenta
hacia atrás los frames hasta el siguiente paso
([El secuenciador](sequencer.md)):

```rust
pub fn position(&self) -> f64 {
    if self.frames_until_next <= 0.0 {
        return self.next_step as f64;
    }
    let current = (self.next_step + self.length - 1) % self.length;
    let passed = 1.0 - self.frames_until_next / self.frames_per_step;
    current as f64 + passed.clamp(0.0, 1.0)
}
```

`next_step` es el paso a punto de dispararse, así que el que suena es el
anterior. `next_step - 1` bajaría de cero en el paso 0, y un `usize` no
puede (haría panic en una compilación de depuración). Sumar `length`
primero y tomar el resto retrocede un paso *alrededor del bucle*: del
paso 0 al último. `passed` es cuánto de ese paso ha pasado, de 0 a 1.

El hilo de audio la guarda después de cada bloque, como los bits del
`f64` en un `AtomicU64`, ya que no hay float atómico
([Dos hilos](../program/threads.md#el-cursor-de-reproducción-un-entero-atómico)).
«Parado» es `NO_POSITION = u64::MAX`; como float, esos bits son un NaN
(«no es un número»), que ninguna posición es nunca. `LiveInput` guarda
un clon del mismo `Arc`, así que el hilo de midir puede leer la
posición en el mismo momento en que llega una tecla. Para entonces
tiene como mucho un bloque de antigüedad, una centésima de segundo; un
paso a 120 BPM dura un octavo de segundo, y grabar redondea al paso
más cercano de todos modos.

### Una tecla en el hilo de audio

Una nota de la canción recibe un gate, el número de frames que se
mantiene. Una tecla no tiene duración conocida, así que recibe `HELD`,
un gate que nunca se agota: solo `note_off` la termina
([Instrumentos](instruments.md)). `play_live` las empieza y
las termina:

```rust
fn play_live(&mut self, message: MidiMessage) {
    match message {
        MidiMessage::NoteOn {
            pitch, velocity, ..
        } => {
            // A key that is already down (on both keyboards at once,
            // say) comes up first.
            self.release_live(pitch);
            if let Some(channel) = self.live_channel
                && let Some(state) = self.channels.get_mut(channel)
            {
                state
                    .instrument
                    .note_on(pitch, velocity, HELD, self.sample_rate);
                self.live_keys[usize::from(pitch.midi())] = Some(channel);
            }
        }
        MidiMessage::NoteOff { pitch, .. } => self.release_live(pitch),
        MidiMessage::ProgramChange { .. } => {}
    }
}
```

Las teclas tocan el *canal en vivo*, que la interfaz fija con
`Command::SetLiveChannel` cada vez que cambia el canal seleccionado.
Pero la selección puede cambiar mientras una tecla está pulsada. Si la
tecla subiera entonces en el canal nuevo, la nota del antiguo sonaría
para siempre. Así que el hilo de audio recuerda, para cada una de las
128 notas, en qué canal bajó su tecla:

```rust
live_keys: [Option<usize>; 128],

fn release_live(&mut self, pitch: Note) {
    if let Some(channel) = self.live_keys[usize::from(pitch.midi())].take()
        && let Some(state) = self.channels.get_mut(channel)
    {
        state.instrument.note_off(pitch);
    }
}
```

Un array de 128 `Option` es un mapa de nota a canal que nunca reserva
memoria, y `take()` lee una entrada y la vacía de una vez. El
`if let ... && let ...` es una **let chain**: los dos patrones tienen que
encajar para que se ejecute el bloque.

Los canales se conocen por su posición en la lista, e insertar o quitar
un canal mueve los que vienen detrás. Un «canal 2» recordado podría ser
entonces otro canal, o ninguno. Así que antes de que cambie la lista,
`release_live_keys` suelta todas las teclas pulsadas. El test
`removing_channels_lets_go_of_their_keys` quita el canal 0 mientras hay
una tecla pulsada en el canal 1, que entonces *pasa a ser* el canal 0.

Las notas en vivo suenan como una vista previa: también en un canal
silenciado, y esté o no tocando el secuenciador. Silenciar detiene las
notas de la canción en un canal, no la tecla que estás pulsando
(`muting_a_channel_stops_its_notes_but_not_keys_held_down`). Detener la
reproducción suelta todo, teclas incluidas.

## midir: puertos y conexiones

[midir](https://docs.rs/midir) es el crate que habla con la capa MIDI
de cada sistema: ALSA en Linux, Core MIDI en macOS, Windows Multimedia
en Windows. Está hecho como `cpal`: listar los puertos, abrir una
conexión a uno, y la conexión queda abierta mientras se conserve el
valor. Ferrous Lineup lo envuelve en un struct `Midi`, en
`lineup-app/src/midi.rs`.

Si hay MIDI o no se averigua probando: crear un `MidiInput` falla en un
sistema sin capa MIDI, y el error se guarda para mostrarlo en el menú.
En Linux eso es un equipo sin el *secuenciador* de ALSA (el módulo del
kernel `snd-seq`, que crea `/dev/snd/seq`), y la ayuda del menú lo dice.

Los puertos se vuelven a encontrar por un id que da midir, un `String`
que no cambia mientras el dispositivo esté enchufado, porque la lista de
puertos cambia a medida que los dispositivos van y vienen. El menú MIDI
busca puertos cada vez que se abre (`refresh_ports`), y deja fuera los
de Ferrous Lineup: en Linux, el puerto desde el que envía la aplicación
aparece también como *entrada*, y elegirlo haría dar vueltas en círculo
a cada nota.

```rust
fn is_own(name: &str) -> bool {
    name.starts_with(&format!("{CLIENT_NAME}:"))
}
```

Los dos puntos importan: un sintetizador llamado «Ferrous Lineupish
Synth» no es la aplicación (el test `its_own_ports_are_left_out` lo
comprueba).

### Conectar consume, cerrar devuelve

```rust
let mut midi_in = MidiInput::new(CLIENT_NAME).map_err(|err| err.to_string())?;
// Clock ticks, "I am still here" messages and system exclusive
// data are not notes. Leaving them out saves waking up for them.
midi_in.ignore(Ignore::All);
let found = midi_in
    .find_port_by_id(&port.id)
    .ok_or_else(|| crate::i18n::trf("{} is not connected any more", &[&port.name]))?;
let link = InputLink {
    live: self.live.clone(),
    to_ui: self.to_ui.clone(),
    ctx: self.ctx.clone(),
    snap: Arc::clone(&self.snap),
    sounding: Vec::new(),
};
let connection = midi_in
    .connect(&found, "input", on_message, link)
    .map_err(|err| err.to_string())?;
```

`connect` toma `self`, no `&self`: el `MidiInput` se consume y vive
dentro de la conexión desde entonces. `close` lo devuelve, con los
datos. Es la propiedad usada como máquina de estados: un `MidiInput`
conectado no se puede conectar dos veces, porque ya no lo tienes.

`on_message` es una función normal, y `link` son los datos que recibe
cada vez, como `&mut InputLink`:

```rust
fn on_message(_microseconds: u64, bytes: &[u8], link: &mut InputLink) {
    let Some(message) = MidiMessage::parse(bytes) else {
        return;
    };
    let message = link.snapped(message);
    let mut position = None;
    if let Some(live) = &link.live {
        let mut live = lock(live);
        live.send(message);
        position = live.position();
    }
    // `send` fails only if the UI is gone, and then nobody needs the copy.
    let _ = link.to_ui.send(Played { message, position });
    // egui only redraws when something happens. This is something, and
    // `request_repaint` is one of the few egui calls meant for other threads.
    link.ctx.request_repaint();
}
```

Se ejecuta en el hilo de midir, así que `connect` exige que ella y sus
datos sean `Send + 'static`, como el callback de audio. El tiempo que
pasa midir se ignora: el reloj que importa es la posición del hilo de
audio.

### Por qué el `LiveInput` está detrás de un `Mutex`

`InputLink` guarda el `LiveInput` como `Arc<Mutex<LiveInput>>`, lo que
parece raro para un valor que solo usa un hilo a la vez. La razón son
los errores de midir. Si `connect` falla, el error devuelve el
`MidiInput`, pero *no* los datos: `link` se perdería, y con él el único
productor de la cola en vivo, para siempre. Ya no se podría conectar
ningún teclado.

Así que el struct `Midi` guarda el `LiveInput` él mismo, detrás de un
`Arc<Mutex<>>`, y cada enlace guarda un clon del `Arc`. Una conexión
fallida solo pierde un clon. Solo existe una conexión a la vez, así que
nunca se espera por el cerrojo, y el hilo de midir no es el hilo de
audio, así que puede bloquear. El mismo `Arc` permite a la aplicación
poner el `LiveInput` de un motor nuevo después de abrir otra vez la
tarjeta de sonido (`reconnect_audio`), sin volver a conectar el teclado.

### Cerrojos envenenados

```rust
fn lock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex.lock().unwrap_or_else(PoisonError::into_inner)
}
```

Si un hilo entra en pánico mientras tiene un cerrojo, Rust marca el `Mutex`
como **envenenado**, y cada `lock()` posterior devuelve un `Err`: los
datos de dentro pueden estar a medio cambiar, y Rust no los entrega sin
preguntar. `PoisonError::into_inner` es la forma de decir «dámelos de
todos modos». Aquí es seguro, porque un extremo de cola, una conexión o
una tecla nunca quedan a medio cambiar. La función es genérica sobre
`T`, así que sirve para todos los mutex del archivo.

### Una copia para la interfaz: `mpsc`

La copia de cada tecla para la interfaz va por un canal
`std::sync::mpsc` («varios productores, un consumidor»), de la
biblioteca estándar:

```rust
let (to_ui, from_keyboard) = mpsc::channel();
// ...
pub fn played(&self) -> Vec<Played> {
    self.from_keyboard.try_iter().collect()
}
```

A diferencia de rtrb, reserva memoria y puede bloquear, lo que aquí no
importa: ni el hilo de midir ni el de la interfaz son de tiempo real. Y
el `Sender` se puede clonar, así que cada conexión nueva recibe el suyo.
`try_iter` da los mensajes que esperan y se detiene cuando no hay más,
sin esperar: lo correcto para una interfaz que no debe dejar de dibujar.

egui solo dibuja un frame cuando pasa algo: se mueve el ratón, se
escribe una tecla. Una tecla de un teclado MIDI no es algo que egui vea,
así que `on_message` llama a `ctx.request_repaint()`. `egui::Context` es
un handle que se puede clonar y enviar a otros hilos, y
`request_repaint` es una de las pocas llamadas de egui pensadas para
usarse desde ellos.

### Ajustar las teclas a la tonalidad

**Ajustar las notas que tocas**, en el piano roll, lleva cada tecla que
tocas a la nota más cercana de la tonalidad en que está la canción en
ese punto, así que no puede salir una nota equivocada
([Escalas, modos y tonalidades](../theory/scales.md)). Para un teclado
MIDI eso tiene que ocurrir en el hilo de midir, antes de que la tecla
vaya al hilo de audio. La interfaz comparte la tonalidad a la que
ajustar mediante un `Arc<Mutex<Option<Scale>>>` más, que fija en cada
frame, y el enlace ajusta cada tecla con `tools::nearest_in_key`
([Herramientas de edición como funciones
simples](../theory/tools.md)).

La tonalidad de la canción puede cambiar mientras se mantiene una
tecla: la reproducción cruza una marca de tonalidad. Entonces la subida
de la tecla, ajustada a la tonalidad nueva, podría nombrar otra nota, y
la nota que empezó al bajar no terminaría nunca. Así que el enlace
recuerda, para cada tecla pulsada, la nota que toca:

```rust
MidiMessage::NoteOff {
    channel,
    pitch: key,
} => {
    let at = self.sounding.iter().position(|&(k, _)| k == key);
    let pitch = at.map_or(key, |at| self.sounding.remove(at).1);
    MidiMessage::NoteOff { channel, pitch }
}
```

`map_or(key, ...)` se lee «la nota que empezó esta tecla, o la tecla
misma si no empezó ninguna que conozcamos». El teclado del equipo guarda
los mismos pares para sus teclas, en `typing_down`.

## El teclado del equipo

¿No hay un teclado MIDI a mano? **MIDI > Teclado del equipo como piano**
convierte las teclas de letras en uno, dispuesto como las teclas negras
de un piano, entre las blancas y por encima de ellas:

```text
  teclas negras:  S  D     G  H  J      2  3     5  6  7     9  0
  teclas blancas: Z  X  C  V  B  N  M   Q  W  E  R  T  Y  U  I  O  P
  notas:          Do4 Re4 Mi4 Fa4 Sol4 La4 Si4  Do5 Re5 Mi5 Fa5 Sol5 La5 Si5 Do6 Re6 Mi6
```

La disposición es una tabla, `TYPING_KEYS: &[(Key, u8)]`, y
`typing_note` busca una tecla en ella con `find`. Las teclas se leen
una vez por frame. egui guarda los eventos de teclado de este frame en
una lista, y el código elige los que quiere:

```rust
egui::Event::Key {
    key,
    physical_key,
    pressed,
    repeat: false,
    modifiers,
} => {
    // The *physical* key, where it is on the keyboard,
    // so the piano keeps its shape on any layout:
    // on a French keyboard, W is where Z is in English.
    let key = physical_key.unwrap_or(*key);
    // Ctrl+Z is undo, not a note. Key releases always
    // count, or a note could stay on.
    let shortcut = modifiers.command || modifiers.alt;
    (!*pressed || !shortcut).then_some((key, *pressed))
}
```

Cuatro detalles hacen que se comporte como un piano:

- **`repeat: false`** en el patrón. Una tecla mantenida hace que el
  sistema la repita («aaaaa» en un campo de texto). Un campo escrito
  como un valor en un patrón tiene que ser igual a ese valor, así que
  las repeticiones no encajan, y una tecla mantenida toca una vez.
- **`physical_key`**. `key` es la letra que la distribución del teclado
  imprime en la tecla; `physical_key` es dónde *está* la tecla. Algunos
  sistemas no la informan, así que `unwrap_or` vuelve a la letra.
- **Los atajos no son notas.** Una tecla que baja con Ctrl (Cmd en un
  Mac) o Alt pulsado se salta. Una tecla que *sube* siempre cuenta, se
  pulse lo que se pulse, o una nota empezada antes de pulsar Ctrl no
  terminaría nunca. Los atajos de una sola tecla de la aplicación
  (Espacio, L, Inicio, Fin, Supr...) no están en las teclas del piano,
  así que siguen funcionando mientras está activo.
- **El foco.** Si la ventana pierde el teclado (haces clic en otra
  ventana), o se lo queda un campo de texto, un menú o un diálogo, los
  eventos de «subida» de las teclas van a otra parte. Así que todas las
  teclas del equipo se sueltan a la vez cuando eso ocurre.

Los eventos se leen dentro de `ctx.input(|input| ...)`, que mantiene un
cerrojo sobre la entrada de egui mientras se ejecuta el closure. El
closure solo reúne un `Vec` de `(key, pressed)`, y el bucle que toca las
notas se ejecuta después, con el cerrojo ya liberado.

Soltar todas las teclas usa `std::mem::take`:

```rust
fn release_typing_keys(&mut self) {
    // `take` leaves an empty list behind, so the loop can call `&mut self` methods.
    for (_, pitch) in std::mem::take(&mut self.live.typing_down) {
        self.play_typed(MidiMessage::NoteOff { channel: 0, pitch });
    }
}
```

Recorrer `&self.live.typing_down` prestaría `self` durante todo el
bucle, y `play_typed` necesita `&mut self`. Sacar la lista primero deja
una vacía en su lugar, y el bucle es dueño de la antigua, así que nunca
se solapan.

`play_typed` envía la tecla al hilo de audio como `Command::Live`, y
luego se la pasa a `key_played` con la posición, exactamente como llega
una tecla de midir. Las [teclas de armonía](../theory/numerals.md)
tocan sus acordes del mismo modo. A partir de ahí, la aplicación no
distingue los teclados.

### El canal seleccionado

Las teclas necesitan un canal que tocar, y el piano roll ya tiene uno:
el canal que muestra. Una vez por frame, `live_input` avisa al hilo de
audio si cambió:

```rust
let channel = self
    .piano_roll
    .selected_channel()
    .filter(|&channel| channel < self.project.channels().len());
if self.live.channel_sent != Some(channel) {
    self.send(Command::SetLiveChannel(channel));
    self.live.channel_sent = Some(channel);
}
```

`channel_sent` es un `Option<Option<usize>>`, y las dos capas
significan algo. El `None` exterior es «aún no se envió nada», así que
el primer frame, y el primero después de un proyecto nuevo, siempre
envían. `Some(None)` es «enviado: ningún canal», que el hilo de audio
entiende como «suelta todas las teclas e ignora las nuevas». Juntar las
dos cosas en un solo `Option` haría imposible decir una de ellas.

Elegir otro canal también termina la toma que se esté grabando, y
suelta todas las teclas pulsadas, así que una tecla nunca termina en un
canal en el que no empezó.

### Las teclas pulsadas

Cada tecla pulsada, de cualquiera de los dos teclados, se guarda en una
lista de `Held`, en el orden en que bajaron:

```rust
struct Held {
    pitch: Note,
    velocity: u8,
    /// The channel it plays, and will be recorded into.
    channel: usize,
    /// Where playback was when it went down, if it is being recorded.
    recorded_from: Option<f64>,
}
```

Esta lista es lo que el resto de la aplicación entiende por «las teclas
que pulsas». La barra de estado nombra su acorde («🎹 Am: La menor,
acorde vi de Do mayor»), el piano roll las ilumina en su teclado, las
teclas de armonía y el buscador de tonalidad las escuchan
([Oír la armonía](../theory/hearing.md)), y **El acompañamiento sigue
los acordes** las convierte en el acorde de la banda
([Cómo toca la banda una hoja de acordes](../band/band.md)). Grabar
es un uso más.

## Grabar: de una tecla a una nota

El botón **⏺** junto a Reproducir solo *arma* la grabación. Las teclas
se graban mientras se cumplen cuatro cosas a la vez:

```rust
pub(super) fn is_recording(&self) -> bool {
    self.live.recording
        && self.playing
        && self.mode == PlayMode::Pattern
        && self.piano_roll.selected_channel().is_some()
}
```

Armada, reproduciendo, en modo patrón (una toma va al patrón que se
repite) y con un canal en el que grabar. Armarla pasa al modo patrón
por ti, así que basta ⏺ y luego Espacio.

### Recordar dónde bajó una tecla

Cuando una tecla baja, `key_played` añade un `Held`:

```rust
self.live.held.retain(|held| held.pitch != pitch);
self.live.follow.key_down();
let recorded_from = played.position.filter(|_| self.is_recording());
self.live.held.push(Held {
    pitch,
    velocity,
    channel,
    recorded_from,
});
```

`retain` quita primero la misma altura si ya estaba pulsada (en el otro
teclado, por ejemplo), así que cada altura está una sola vez en la
lista.

`Option::filter` conserva un `Some` solo si el closure dice que sí, y si
no lo convierte en `None`. Así que `recorded_from` es la posición cuando
la tecla se está grabando, y `None` cuando solo se toca. El closure
ignora su argumento (`|_|`): la pregunta no es sobre la posición, sino
sobre si se está grabando.

Cuando la tecla sube, su `Held` sale de la lista, y si tiene posición de
inicio y hay posición final, se convierte en una nota:

```rust
let held = self.live.held.remove(index);
if let (Some(from), Some(until)) = (held.recorded_from, played.position) {
    self.record(held, from, until);
}
```

Hacer match con una tupla de dos `Option` comprueba los dos a la vez:
el bloque solo se ejecuta si los dos son `Some`.

### Cuantizar: `recorded_note`

Nadie toca exactamente en el paso. Una tecla que bajó en 3,9 y subió en
6,2 quería ser una nota del paso 4 al 6. Mover los extremos de una nota
a la cuadrícula se llama **cuantizar**, y es una función normal en
`lineup-core`, sin teclado ni interfaz a la vista:

```rust
pub fn recorded_note(pitch: Note, velocity: u8, from: f64, until: f64, length: usize) -> NoteEvent {
    let length = length.max(1);
    let until = if until < from {
        until + length as f64
    } else {
        until
    };
    let start = from.round();
    let end = until.round().max(start + 1.0);
    // `as usize` turns a float into an integer, dropping the fraction
    // (there is none left after `round`). Negative numbers become 0.
    let start_step = start as usize % length;
    let steps = ((end - start) as usize).min(length - start_step);
    NoteEvent {
        velocity,
        ..NoteEvent::new(pitch, start_step, steps)
    }
}
```

Línea a línea:

1. `length.max(1)` protege el `%` de abajo: un resto entre cero hace
   panic. Un patrón nunca está vacío, pero una función no debería
   depender de quien la llama para eso.
2. El patrón se repite. Si la tecla subió *antes* de donde bajó (bajó en
   14,5, subió en 1,5), el bucle volvió a empezar en medio, y el final
   es en realidad 17,5, una vuelta más allá.
3. `round()` lleva cada extremo al paso más cercano. `f64::round`
   redondea las mitades alejándose de cero, así que 2,5 pasa a 3.
4. `.max(start + 1.0)`: una pulsación rápida redondearía los dos
   extremos al mismo paso, una nota de duración 0. Una nota dura al
   menos un paso.
5. Una tecla pulsada justo antes de que el bucle vuelva a empezar (en
   15,7 en un patrón de 16 pasos) redondea a 16, que es el paso 0 de la
   vuelta siguiente: `% length` lo pliega de nuevo a 0. Ahí es donde lo
   quería quien toca.
6. Una nota no puede pasar del final del patrón, así que se corta ahí
   con `.min(length - start_step)`.

Las últimas líneas usan la **sintaxis de actualización de structs**:
`..NoteEvent::new(...)` significa «todos los campos no nombrados, como
en este valor», así que la nota recibe su velocidad, y `nudge` y `trim`
a 0 de `new`. Una nota grabada cae en la cuadrícula.

Los tests se leen como esas reglas:

```rust
#[test]
fn ends_go_to_the_nearest_step() {
    assert_eq!(recorded(2.0, 4.0), (2, 2));
    assert_eq!(recorded(1.8, 4.3), (2, 2), "a little early, a little late");
    assert_eq!(recorded(2.4, 4.6), (2, 3));
}

#[test]
fn just_before_the_loop_is_step_zero() {
    assert_eq!(recorded(15.7, 0.4), (0, 1));
    assert_eq!(recorded(15.6, 1.9), (0, 2));
}

#[test]
fn a_note_across_the_loop_end_is_cut_there() {
    assert_eq!(recorded(14.0, 2.0), (14, 2));
}
```

`recorded(from, until)` es una función auxiliar del módulo de tests que
graba un Do4 en un patrón de 16 pasos y devuelve `(start, length)`.
Poder escribir `15.7` en lugar de tener que tocar una tecla justo en ese
momento es la razón de que la función viva en `lineup-core` y reciba
números simples.

`start as usize` convierte un `f64` en un `usize`. Para los floats,
`as` descarta la parte fraccionaria y **satura**: un número negativo
pasa a 0, uno demasiado grande a `usize::MAX`, y NaN a 0. Nunca hace
panic ni da la vuelta. Es distinto de `as` entre enteros, que se queda
solo con los bits bajos: `300_i32 as u8` es 44.

Las posiciones dicen *en qué punto del bucle* estaba una tecla, no *en
qué vuelta*. Una tecla mantenida más de un bucle entero parece igual que
una corta: bajar en 2 y subir en 3 una vuelta después se lee como un
paso de duración. Y una nota que cruza el final del bucle se corta ahí,
en lugar de seguir en el paso 0. Las dos cosas quedan como
[ejercicios](exercises.md).

### Un paso de deshacer por toma

`record` añade la nota al patrón seleccionado mediante `edit_pattern`,
como cualquier otro cambio, así que se puede deshacer y el hilo de
audio se entera
([Decirle al hilo de audio lo que ha cambiado](../program/sync.md)):

```rust
fn record(&mut self, held: Held, from: f64, until: f64) {
    let Some(length) = self.project.pattern(self.pattern).map(Pattern::length) else {
        return;
    };
    let note = recorded_note(held.pitch, held.velocity, from, until, length);
    self.edit_pattern("record notes", |pattern| {
        pattern.add_note(held.channel, note);
    });
}
```

`.map(Pattern::length)` pasa un método como la función de `map`:
`Pattern::length` es una función que recibe un `&Pattern`, justo lo que
entrega `map`. `add_note` se salta una nota con la misma altura y el
mismo inicio que otra que ya está, así que tocar lo mismo en la vuelta
siguiente no apila notas unas encima de otras.

Una toma puede tener docenas de notas, y Ctrl+Z debería deshacerlas
todas, no de una en una. El historial ya mantiene un paso abierto
mientras el ratón está pulsado, así que un arrastre es un solo paso
([Deshacer y rehacer](../program/undo.md#varios-clics-un-paso)).
Grabar se suma a esa regla:

```rust
pub(super) fn end_frame_for_undo(&mut self, ctx: &egui::Context) {
    let holding = ctx.input(|input| input.pointer.any_down());
    if !holding && !ctx.text_edit_focused() && !self.is_recording() {
        self.close_step();
    } else if self.step_unsynced {
        // ...
    }
}
```

Mientras se graba, el paso sigue abierto, y cada nota se une a él. La
toma termina, y el paso con ella, cuando se detiene la reproducción, se
desarma la grabación, cambia el modo o se elige otro canal.

### Terminar una toma: antes de Detener

Cuando se detiene la reproducción, puede haber teclas aún pulsadas. Se
graban como si hubieran subido en ese momento, con `finish_recording`:

```rust
pub(super) fn finish_recording(&mut self) {
    let until = self.position();
    for index in 0..self.live.held.len() {
        let held = self.live.held[index];
        if let (Some(from), Some(until)) = (held.recorded_from, until) {
            self.record(held, from, until);
        }
        self.live.held[index].recorded_from = None;
    }
}
```

Las teclas siguen en la lista (siguen pulsadas, y siguen sonando), pero
su `recorded_from` se borra, así que su subida más tarde no graba nada.
El bucle va por índice, no `for held in &self.live.held`, porque
`record` necesita `&mut self` mientras la lista está prestada. `Held`
es `Copy`, así que `let held = self.live.held[index];` lo copia, y el
préstamo termina en esa línea.

El orden en `halt`, por donde pasa cualquier forma de detener, importa:

```rust
fn halt(&mut self) {
    if self.playing {
        // Before the audio thread forgets where it was.
        self.finish_recording();
    }
    self.playing = false;
    self.send(Command::Stop);
}
```

`finish_recording` necesita la posición, y `position()` da `None` en
cuanto `self.playing` es falso, o en cuanto el hilo de audio se ha
detenido y ha guardado `NO_POSITION`. Así que la toma se termina
primero, mientras los dos aún dicen «reproduciendo», y solo después se
envía Stop.

### Latencia: lo que oyes ya pasó

Cuando pulsas una tecla «en el tiempo», la pulsas en el tiempo que
*oíste*. Pero el sonido tarda en llegarte. El hilo de audio prepara cada
bloque por adelantado y se lo da a la tarjeta de sonido, que lo toca un
poco más tarde; una tarjeta suele guardar uno o dos bloques, así que con
bloques de 512 frames a 48 kHz un sonido sale de 11 a 21 ms después de
que el hilo de audio lo hiciera. Cuando oyes empezar el paso 4, el hilo
de audio ya va así de adentrado en él, y una tecla pulsada justo
entonces se marca así de tarde.

A 120 BPM un paso dura 125 ms, así que 21 ms son un sexto de paso, y
redondear al paso más cercano lo oculta, salvo que ya llegaras un poco
tarde. Un teclado MIDI apenas añade nada: sus teclas se marcan en el
hilo de midir en cuanto llegan. El teclado del equipo añade más: egui
lee el teclado una vez por frame, así que una tecla escrita puede
esperar hasta un frame antes de marcarse, 17 ms a 60 frames por
segundo, y mucho más en un equipo que dibuja sus frames despacio.

La solución es restar la latencia a la posición de cada tecla antes de
redondear. cpal puede decir cuánta es: cada llamada del callback de
audio recibe un `OutputCallbackInfo` con dos marcas de tiempo, cuándo se
ejecutó el callback y cuándo sonará su sonido. Es uno de los
[ejercicios](exercises.md).

## Salida MIDI: un instrumento con bandeja de salida

Un canal de salida MIDI no suena. Su instrumento convierte las notas en
mensajes MIDI para otro sintetizador. Es una clase más de `Instrument`,
junto al sampler, el Lineup Osc y el reproductor de SoundFont, así que
todo lo demás (el secuenciador, las teclas en vivo, el volumen,
silenciar) funciona con él sin cambios.

El hilo de audio no debe enviar los mensajes él mismo. Enviar pasa por
la capa MIDI del sistema operativo, una llamada al sistema que puede
tardar lo que sea, y la
[regla de oro](../program/threads.md#la-regla-de-oro-del-hilo-de-audio)
prohíbe esperar. Así que `MidiOut` junta sus mensajes en una *bandeja de
salida*, un `Vec` con espacio reservado al crearlo en el hilo de la
interfaz:

```rust
fn post(&mut self, message: MidiMessage) {
    let outbox = &mut self.state.outbox;
    if outbox.len() < outbox.capacity() {
        outbox.push(message);
    }
}
```

Comprobar `len() < capacity()` antes de meter hace que el `Vec` nunca
crezca, así que nunca reserva memoria. El espacio es `OUTBOX_SIZE`, 256
mensajes para un bloque de pocos milisegundos; incluso un acorde de
dieciséis notas que empieza mientras terminan otras dieciséis son solo
32. Si alguna vez se llenara, se perdería un mensaje antes que romper la
regla.

```rust
pub struct MidiOut {
    /// The MIDI channel, 0 to 15.
    channel: u8,
    /// The note every note goes out as, for a drum.
    pitch: Option<Note>,
    /// The General MIDI program the synth should play, if any.
    program: Option<u8>,
    /// Whether the program change still has to be sent: before the first
    /// note, and again after every stop, in case the synth's sound was
    /// changed on the synth itself in between.
    program_due: bool,
    /// Frames played so far: the clock that says when a note-off is due.
    now: u64,
    /// Boxed so an `Instrument` stays small, as the voices of the others are.
    state: Box<MidiOutState>,
}
```

Un sintetizador solo termina una nota cuando se le dice, así que
`MidiOut` tiene que enviar un note-off cuando se agota el gate de cada
nota. Guarda, para cada una de las 128 notas, un enum que dice si la
nota suena y hasta cuándo:

```rust
enum Sounding {
    Off,
    /// Until this frame, when its note-off is due.
    Until(u64),
    /// Until `note_off`.
    Held,
}
```

`next_sample` no suena, pero cuenta los frames en `now`, que es el reloj
de `Until`. Empezar una nota envía primero un program change cuando toca
uno, y luego el note-on:

```rust
let pitch = self.pitch.unwrap_or(pitch);
let slot = usize::from(pitch.midi());
// MIDI has one of each note per channel. If this one is already
// sounding, end it first, or the synth would hear "on, on, off"
// and the new note would stop at the old one's note-off.
if self.state.notes[slot] != Sounding::Off {
    self.post(self.note_off_message(pitch));
}
```

`self.pitch.unwrap_or(pitch)` se lee «la nota del tambor si la hay, y si
no la nota misma», y escribir `let pitch = ...` otra vez *oculta* el
parámetro: desde esa línea, `pitch` es el valor nuevo, y el antiguo no
se puede usar por error. `note_off` solo termina notas `Held`: si el
patrón toca un Do4 largo y tú pulsas Do4 en el teclado, soltar tu tecla
no debe terminar la nota del patrón.

Una vez por bloque, el hilo de audio pide a cada instrumento que entregue
sus mensajes:

```rust
pub fn flush(&mut self, mut send: impl FnMut(MidiMessage)) {
    for index in 0..128 {
        if let Sounding::Until(end) = self.state.notes[index]
            && end <= self.now
        {
            self.post(self.note_off_message(Note::new(index as u8)));
            self.state.notes[index] = Sounding::Off;
        }
    }
    // `drain(..)` empties the `Vec` but keeps its memory for next time.
    for message in self.state.outbox.drain(..) {
        send(message);
    }
}
```

Los note-offs se comprueban una vez por bloque, no en cada frame, así
que pueden llegar unos milisegundos tarde, lo que nadie oye al final de
una nota. `drain(..)` saca todos los elementos del `Vec` en orden, y el
`Vec` conserva su memoria para el bloque siguiente. `send` es un
`impl FnMut`: al instrumento no le importa adónde van los mensajes. El
hilo de audio pasa un closure que los pone en la cola `MidiOutput` (o,
cuando está llena, en una reserva preparada de antemano, porque un
note-off perdido deja una nota sonando para siempre; ver
[Dos hilos](../program/threads.md#cuando-la-tarjeta-de-sonido-desaparece)).
Los tests pasan uno que los mete en un `Vec` normal para mirarlos.

El estado vive en un `Box`. 128 `Sounding` y 128 `Option<Origin>`
ocupan unos pocos kilobytes, y un enum es tan grande como su variante
más grande: sin el box, cada `Instrument`, y cada `Command` que lleva
uno por el ring buffer, sería así de grande. Un test del motor mantiene
un comando en 80 bytes o menos, y fallaría sin el box.

Un instrumento de SoundFont también es un `MidiOut`, cuyos mensajes van
a un sintetizador dentro de la aplicación en lugar de a un puerto
([SoundFonts y General MIDI](soundfonts.md#una-salida-midi-que-toca-dentro-de-la-aplicación)).

### El hilo de envío

Los mensajes que los canales de salida MIDI dejan en la cola
`MidiOutput` los envía un hilo propio, llamado `midi-out`:

```rust
let sender = move || {
    while !messages.is_abandoned() {
        thread::sleep(SEND_INTERVAL);
        let mut output = lock(&output);
        while let Some(message) = messages.pop() {
            // With no port connected, the messages are dropped. A send
            // that fails (a cable pulled out) loses only that message.
            if let Some(connection) = output.as_mut() {
                let _ = connection.send(&message.bytes());
            }
        }
    }
    // ...
};
// If the thread cannot start, MIDI Out stays silent; nothing else breaks.
thread::Builder::new()
    .name("midi-out".to_string())
    .spawn(sender)
    .ok()
```

Duerme un milisegundo (`SEND_INTERVAL`), envía lo que esté esperando y
vuelve a empezar. Un hilo que espera trabajo normalmente dormiría hasta
que lo despertaran, con un `Condvar`. Pero despertar un hilo es una
llamada al sistema, que el hilo de audio no debe hacer. Mirar mil veces
por segundo no cuesta casi nada, y añade como mucho un milisegundo.

El cerrojo se toma después de dormir, no antes, así que el hilo de la
interfaz nunca espera por un hilo dormido. El guard `output` se suelta
al final de cada vuelta del bucle, lo que libera el cerrojo.

`is_abandoned()` es verdadero en cuanto el otro extremo de la cola ha
desaparecido: se soltó el motor de audio, como al cerrar la aplicación
o al abrir otra vez la tarjeta de sonido. Entonces el bucle termina, el
hilo envía All Notes Off por las notas que el motor antiguo aún debía, y
acaba. Nadie tiene que decirle que pare.

`thread::Builder` es la forma larga de empezar un hilo. Puede darle un
nombre, que aparece en los depuradores y en herramientas como `top -H`.
Y su `spawn` devuelve un `Result` donde `thread::spawn` haría panic si
el sistema se negara. `.ok()` lo convierte en un `Option<JoinHandle>`:
si el hilo no puede empezar, la salida MIDI se queda callada y todo lo
demás funciona, que es mejor que un fallo al arrancar. Sin audio no hay
hilo de envío, ya que las notas se hacen en el hilo de audio, y el menú
MIDI dice **No se envía nada mientras el audio no esté disponible**.

### Cambiar de puerto sin notas colgadas

La conexión al puerto de salida está detrás de un `Mutex` porque la
usan dos hilos: el hilo de envío envía, y el hilo de la interfaz la
sustituye cuando eliges otro puerto:

```rust
// Swap the connections while holding the lock, so the sending
// thread never sends half to one and half to the other.
let old = std::mem::replace(&mut *lock(&self.output), new);
if let Some(mut old) = old {
    // Notes that were playing there would hang: tell them all to stop.
    for channel in 0..16 {
        let _ = old.send(&all_notes_off(channel));
    }
    // `close` gives back the `MidiOutput`, which is not needed.
    old.close();
}
```

`&mut *lock(&self.output)` bloquea el mutex, sigue el guard hasta el
`Option` de dentro (`*`) y lo presta de forma mutable, todo en una
expresión; el guard se suelta al final de la sentencia, así que el
cerrojo se mantiene justo lo necesario para el intercambio. El hilo de
envío envía entonces al puerto antiguo o al nuevo, nunca la mitad a
cada uno.

La entrada tiene su propia versión de esto: cuando eliges otro teclado
MIDI, se sueltan todas las teclas pulsadas en el antiguo
(`release_all_keys`), porque sus subidas no llegarían nunca.

## Enviar canales por MIDI

Un canal de salida MIDI, añadido con **Añadir canal > Salida MIDI
(sintetizador externo)**, es la forma clásica: un canal propio que
envía sus notas al sintetizador. También es una trampa. Conecta un
sintetizador, elígelo en la **Salida** del menú **MIDI**, abre una
demostración, pulsa Reproducir... y si no se hiciera nada al respecto,
el sintetizador no diría nada, porque ninguna demostración tiene un
canal de salida MIDI. Así que hay dos formas más: todos los canales a
la vez, y un canal desde su menú de instrumento.

### Todos los canales a la vez: rutas

**MIDI > Enviar todos los canales** está activado desde el principio.
Mientras haya una salida conectada, cada canal suena en el sintetizador
en lugar de en los altavoces, en los mismos canales MIDI que escribe
**Archivo > Exportar canción como MIDI…**, así que una canción suena
igual tocada en vivo que desde su archivo MIDI. Una *ruta* es por dónde
salen las notas de un canal:

```rust
pub type Route = (u8, Option<Note>);

pub fn routes(channels: &[Channel]) -> Vec<Option<Route>> {
    channels
        .iter()
        .zip(midi_channels(channels))
        .map(|(channel, midi_channel)| match &channel.kind {
            _ if channel.sent_over_midi().is_some() => None,
            ChannelKind::Sampler(SampleSource::Builtin(drum)) => {
                Some((midi_channel, Some(general_midi_drum(*drum))))
            }
            _ => Some((midi_channel, None)),
        })
        .collect()
}
```

Una ruta es un canal MIDI, y la nota del tambor si la hay. `type` solo
da un nombre corto a un tipo largo; un `Route` sigue siendo una tupla
normal. `zip` recorre dos listas a la par: cada canal con su canal MIDI,
de `midi_channels` (más abajo). El primer brazo, `_ if ...`, es un
comodín con guarda: encaja con cualquier clase, pero solo para un canal
que ya se envía por MIDI, que envía sus notas él mismo y no necesita
ruta.

Mientras se envían todos los canales, cada sampler y sintetizador se
*sustituye* sin más por un `MidiOut`, y se repone cuando se deja de
enviar:

```rust
pub fn to_midi(self, channel: u8, pitch: Option<Note>, program: Option<u8>) -> Self {
    match self {
        Instrument::MidiOut(_) => self,
        _ => Instrument::MidiOut(
            MidiOut::new(channel)
                .playing_as(pitch)
                .with_program(program),
        ),
    }
}
```

`to_midi` toma `self` por valor: el instrumento antiguo se *mueve* a la
función y se suelta allí, y quien llama recibe el nuevo. Una salida MIDI
ya envía sus notas, así que vuelve sin cambios.

Las rutas dependen del menú (una salida elegida, la casilla marcada) y
del rack: añadir un sintetizador encima de otro mueve el de abajo al
canal MIDI siguiente. En lugar de perseguir cada lugar que puede
cambiarlas, la aplicación las vuelve a calcular en cada frame y compara:

```rust
let wanted = if self.live.send_all && self.live.midi.output().is_some() {
    midi::routes(channels)
} else {
    vec![None; channels.len()]
};
if self.live.routes_sent.as_ref() == Some(&wanted) {
    return;
}
```

Calcular 32 rutas como mucho cuesta mucho menos que dibujar la ventana,
y comparar dos `Vec` con `==` los compara elemento a elemento, porque
las tuplas, `Option` y `Note` implementan `PartialEq`. Solo cuando
difieren recibe un canal un instrumento nuevo, una salida MIDI o el
suyo, mediante `Command::SetInstrument`, y solo los canales cuya ruta
cambió.

`routes_sent` es un `Option<Vec<...>>`: `None` significa «no se sabe».
Cuando se añaden, quitan o recargan canales, el hilo de audio recibe
instrumentos normales para ellos, y los que se movieron conservan lo
que tuvieran. Si alguno era una salida MIDI en lugar de su propio
instrumento, la aplicación olvida lo que envió, y el frame siguiente
vuelve a enviar todos los canales. Eso es raro (hace falta un cambio en
el rack), y no puede salir mal.

### Un canal desde su menú de instrumento

A menudo se quiere algo intermedio: el bajo en un sintetizador de
hardware y el resto de la banda en la aplicación, o la batería en una
caja de ritmos. Así que cada menú de instrumento, en el mezclador y en
el rack de canales, empieza con **Enviar por MIDI**: un canal MIDI del
1 al 16, y la salida.

![El menú de instrumento de los Teclados en el mezclador: Enviar por MIDI, con sus dieciséis canales MIDI y la salida (este equipo no tiene ninguna), encima de los instrumentos General MIDI.](../images/step-9-midi-out-menu.png)

Un sintetizador enviado por MIDI **se convierte** en un canal de salida
MIDI, y conserva su instrumento:

```rust
pub fn send_over_midi(&mut self, midi_channel: u8) {
    let midi_channel = midi_channel.min(15);
    match self.kind {
        ChannelKind::Sampler(SampleSource::Builtin(_)) => {
            self.sent_on = Some(midi_channel);
            return;
        }
        ChannelKind::Sampler(SampleSource::File(_)) => return,
        _ => {}
    }
    let plain = match self.kind {
        ChannelKind::Synth => self.name == Channel::synth().name,
        ChannelKind::MidiOut { channel } => self.name == Channel::midi_out(channel).name,
        _ => false,
    };
    if plain {
        self.name = Channel::midi_out(midi_channel).name;
    }
    self.kind = ChannelKind::MidiOut {
        channel: midi_channel,
    };
}
```

Un nombre que solo decía qué lo tocaba, «Lineup Osc» o «Salida MIDI
2», sigue el cambio y pasa a «Salida MIDI 3». Un nombre que le diste
tú, como «Bajo», se queda. `play_here` hace lo contrario. Una muestra de
un archivo no tiene ninguna nota que un sintetizador pueda tocar en su
lugar, así que no se puede enviar.

Un **tambor** no cambia de clase. Recibe un campo, `sent_on`, con el
canal MIDI, y sigue siendo un `Sampler` que toca su tambor incorporado.
Eso le importa a la banda: los estilos encuentran el bombo, la caja y
los charles por lo que son (`Sampler(Builtin(Drum::Kick))`), para
escribir sus ritmos en ellos. Un bombo convertido en un canal de salida
MIDI ya no se encontraría, y el estilo siguiente añadiría un segundo
bombo. Por la misma razón, los estilos encuentran un «Bajo» o unos
«Teclados» enviados por MIDI por su nombre, como el bajo y los teclados
([La biblioteca](../band/library.md)). En el hilo de audio,
`Instrument::for_channel` construye un `MidiOut` para un tambor con
`sent_on`, que toca su propia nota de una batería General MIDI
([Instrumentos](instruments.md)), así que la parte de
batería, la banda y los estilos quedan exactamente igual; solo el
sonido se va a otra parte.

`Channel::sent_over_midi` responde «¿se envía, y en qué canal?» para
las dos clases a la vez, así que el resto del código no tiene que
conocer la diferencia.

El mezclador nombra el tambor que envía un canal, con la tabla de
tambores General MIDI de `general_midi.rs`:

```rust
/// The drum General MIDI plays for `note` on channel 10, such as "Bass
/// Drum 1" for 36, if the drum set has one there.
pub fn drum_name(note: u8) -> Option<&'static str> {
    DRUMS
        .get(usize::from(note.checked_sub(FIRST_DRUM)?))
        .copied()
}
```

Aquí se juntan dos clases de «quizá». `checked_sub` devuelve `None` en
lugar de dar la vuelta cuando la nota está por debajo de 35 (un `u8` no
puede bajar de 0), y el `?` devuelve ese `None` de la función en el
acto. `get` devuelve `None` más allá del final de la tabla, para las
notas por encima de 81. `copied` convierte el `Option<&&str>` que da
`get` en un `Option<&str>`.

### El menú necesita la lista de salidas

Los menús de instrumento los dibujan funciones de `channel_rack.rs`, que
conocen un canal y nada de las conexiones MIDI de la aplicación.
Pasárselas a través del rack, el mezclador y el editor de estilos
añadiría un parámetro a muchas funciones. En su lugar, la aplicación
pone la lista de salidas en la memoria de egui una vez por frame, y el
menú la lee de ahí:

```rust
pub fn set_midi_outputs(ctx: &egui::Context, outputs: MidiOutputs) {
    ctx.data_mut(|data| data.insert_temp(midi_outputs_id(), outputs));
}
```

```rust
let outputs: MidiOutputs = ui
    .ctx()
    .data(|data| data.get_temp(midi_outputs_id()))
    .unwrap_or_default();
```

La memoria de egui puede guardar un valor de cualquier tipo que sea
`Clone`, `Send`, `Sync` y `'static`, uno por `Id` y tipo. Los valores
guardados con `insert_temp` no se guardan al cerrar la aplicación, lo
que está bien para una lista de puertos que mañana puede ser otra. El
menú no cambia la conexión él mismo: añade un
`RackAction::ConnectMidiOutput`, que lleva a cabo la aplicación, como
cualquier otro clic en el rack.

Un test de este menú, `the_mixer_sends_a_channel_over_midi_and_back`,
muestra por qué un test tiene que preguntarle a la máquina qué tiene.
Abre **Enviar por MIDI** y comprueba las salidas de la lista. Esperar
**No se encontraron salidas** pasa en las máquinas de test de Linux y
macOS, que no tienen salida MIDI, y falla en Windows, que siempre tiene
una: su «Microsoft GS Wavetable Synth» incorporado. Así que el test lee
las salidas que encontró la aplicación, y espera ese texto solo cuando
no hay ninguna. Es la clase de diferencia que hay que pillar ejecutando
los tests en tres sistemas
([Tres sistemas e integración continua](../shipping/ci.md)).

### Qué canal MIDI usa cada canal

Un programa que lee un archivo MIDI, o un sintetizador que oye la
aplicación, usa el canal MIDI para elegir el instrumento. General MIDI,
el estándar que siguen la mayoría de los sintetizadores, reserva el
**canal 10** (9, contando desde 0) para la batería, con una nota fija
para cada tambor: 36 es un bombo, 38 una caja, 39 unas palmas, 42 un
charles cerrado. Una sola función decide el canal MIDI de cada canal,
para el archivo MIDI y para las rutas por igual:

```rust
pub fn midi_channels(channels: &[Channel]) -> Vec<u8> {
    let taken = |number: u8| {
        channels
            .iter()
            .any(|channel| channel.sent_over_midi() == Some(number))
    };
    let free: Vec<u8> = (0..16u8)
        .filter(|&number| number != DRUM_CHANNEL && !taken(number))
        .collect();
    let mut melodic = if free.is_empty() {
        (0..16u8).filter(|&number| number != DRUM_CHANNEL).collect()
    } else {
        free
    }
    .into_iter()
    .cycle();
    channels
        .iter()
        .map(|channel| match channel.kind {
            ChannelKind::Sampler(SampleSource::Builtin(_)) => {
                channel.sent_on.unwrap_or(DRUM_CHANNEL)
            }
            ChannelKind::MidiOut { channel } => channel,
            _ => melodic.next().unwrap_or(0),
        })
        .collect()
}
```

Los tambores incorporados comparten el canal 10, salvo que alguno se
haya enviado a otro. Los canales enviados por MIDI conservan el canal
que se les dio. Cada uno de los demás toma el siguiente canal *libre*:
no el 10, ni uno en el que ya toque un canal enviado por MIDI, o los dos
compartirían un sonido en el sintetizador y se cortarían las notas.
`cycle` repite los canales libres para siempre, y el iterador se crea
una vez, fuera del closure, que lo toma prestado de forma mutable: cada
`next()` lo hace avanzar, así que los canales reciben 1, 2, 3… por
turno, y si hay más canales que canales libres, empiezan a compartir.

## Escribir un archivo MIDI

Un Standard MIDI File guarda los mismos mensajes, con sus tiempos.
Guarda notas, no sonido: «Do4 empieza aquí, termina allí, en el canal
1». El programa que lo abre elige los instrumentos. Eso lo convierte en
la lengua común de los programas de música, y en la forma de llevar una
canción de Ferrous Lineup a otro estudio. `song_to_midi` escribe la
canción tal como la toca la lista de reproducción, y `pattern_to_midi`
un patrón, una vez.

El archivo es una lista de *chunks*. Cada uno empieza con cuatro letras
y su longitud:

```text
  MThd  longitud 6  formato 1  3 pistas  96 ticks por tiempo     la cabecera
  MTrk  longitud …  nombre "Ferrous Lineup", tempo, compás, tonalidad, fin
  MTrk  longitud …  nombre "Lineup Osc", un program change, las notas, fin
  MTrk  longitud …  nombre "Kick", las notas del bombo, fin
```

Este es el **formato 1**: varias pistas tocadas a la vez. La primera
pista solo guarda el tempo, el compás y la tonalidad, y cada canal del
proyecto recibe una pista propia, con su nombre, que empieza con un
program change cuando el canal tiene un instrumento General MIDI. Un
canal sin notas, o uno que no se oye (silenciado, o callado por el solo
de otro), no recibe pista.

Los números de un archivo MIDI son big-endian: el byte más
significativo primero. Los enteros de Rust tienen un método para eso,
que da un array de bytes:

```rust
let mut file = Vec::new();
file.extend_from_slice(b"MThd");
file.extend_from_slice(&6u32.to_be_bytes()); // the header's length
file.extend_from_slice(&1u16.to_be_bytes()); // format 1
file.extend_from_slice(&(tracks.len() as u16).to_be_bytes());
file.extend_from_slice(&TICKS_PER_BEAT.to_be_bytes());
```

`b"MThd"` es una **cadena de bytes**: un `&[u8; 4]` con los códigos
ASCII de las letras, en lugar de un `&str`. `6u32` es el número 6 como
`u32` (el tipo escrito tras el número), así que `to_be_bytes` da cuatro
bytes: `[0, 0, 0, 6]`. Un `u16` da dos.

### Ticks y tiempos delta

El tiempo en un archivo MIDI se cuenta en *ticks*, y la cabecera dice
cuántos ticks hacen un tiempo. Ferrous Lineup escribe 96, los mismos
ticks en los que coloca sus notas: un paso, una semicorchea, son 24
ticks, y el `nudge` y el `trim` de una nota cuentan los ticks que empieza
después o termina antes de sus pasos
([El proyecto](../program/project.md)). Así que una nota entre dos
pasos se escribe exactamente donde suena.

```rust
pub const TICKS_PER_BEAT: u16 = 96;
pub const TICKS_PER_STEP: u32 = crate::pattern::TICKS_PER_STEP as u32;

// A check made while compiling: if someone changed one of the two
// numbers and not the other, the build would stop here.
const _: () = assert!(TICKS_PER_BEAT as u32 == TICKS_PER_STEP * STEPS_PER_BEAT);
```

`const _: () = assert!(...)` es una constante sin nombre (`_`) cuyo
valor se calcula al compilar. `assert!` funciona en un `const` desde
Rust 1.57, y una aserción fallida ahí es un error de compilación, no un
panic al ejecutar. No cuesta nada en el programa, y convierte «acuérdate
de cambiar los dos» en algo que comprueba el compilador.

Dentro de una pista, cada evento se escribe tras un *tiempo delta*: el
número de ticks desde el evento anterior, no desde el principio. La
mayoría de los deltas son pequeños (0 para las notas de un acorde, 24
para el paso siguiente), así que se escriben en un formato donde los
números pequeños ocupan un byte, la **cantidad de longitud variable**:
siete bits del número por byte, el más significativo primero, con el bit
superior a 1 en todos los bytes menos el último.

```text
  200  = 1 1001000            ->  1000 0001  0100 1000   (0x81 0x48)
         │ └ últimos 7 bits             │         └ bit superior a 0: el último byte
         └ el resto: 1                  └ bit superior a 1: viene más
  384  = 11 0000000           ->  0x83 0x00
```

```rust
fn write_variable_length(out: &mut Vec<u8>, value: u32) {
    let mut groups = [0u8; 5];
    let mut count = 0;
    let mut rest = value;
    loop {
        groups[count] = (rest & 0x7F) as u8;
        count += 1;
        rest >>= 7;
        if rest == 0 {
            break;
        }
    }
    for index in (0..count).rev() {
        let more = if index > 0 { 0x80 } else { 0 };
        out.push(groups[index] | more);
    }
}
```

El bucle corta el número en grupos de siete bits desde el extremo bajo,
porque ese es el extremo al que llegan `& 0x7F` y `>>= 7`. El archivo
los quiere desde el extremo alto, así que el segundo bucle recorre los
grupos hacia atrás con `(0..count).rev()`. Cinco grupos bastan para
cualquier `u32`, así que caben en un array, y no se reserva memoria. El
test `variable_length_numbers` le da los ejemplos impresos en la
especificación de los archivos MIDI.

Un `TrackWriter` guarda el tick del último evento, para que el resto del
código pueda pensar en ticks absolutos:

```rust
#[derive(Default)]
struct TrackWriter {
    data: Vec<u8>,
    last_tick: u32,
}

impl TrackWriter {
    /// Writes the time since the last event. Events must come in order.
    fn delta(&mut self, tick: u32) {
        let delta = tick.saturating_sub(self.last_tick);
        write_variable_length(&mut self.data, delta);
        self.last_tick = self.last_tick.max(tick);
    }

    // ...

    /// Ends the track at `end` (or at its last event, if that is later)
    /// and wraps it in its chunk header.
    fn finish(mut self, end: u32) -> Vec<u8> {
        self.meta(end.max(self.last_tick), END_OF_TRACK, &[]);
        let mut chunk = Vec::with_capacity(8 + self.data.len());
        chunk.extend_from_slice(b"MTrk");
        chunk.extend_from_slice(&(self.data.len() as u32).to_be_bytes());
        chunk.extend(self.data);
        chunk
    }
}
```

`#[derive(Default)]` da `TrackWriter::default()`, un `Vec` vacío y el
tick 0. `finish` toma `mut self`, no `&mut self`: consume el escritor, y
sus datos se mueven al chunk sin copia. Un escritor terminado no se
puede seguir escribiendo por error, porque ya no existe.

### Metaeventos: nombres, tempo, compás y tonalidad

Los eventos que no son notas empiezan con `0xFF`, una clase y una
longitud:

| Bytes | Significa |
|-------|-----------|
| `FF 03 len texto` | el nombre de la pista |
| `FF 51 03 tt tt tt` | el tempo, como microsegundos por tiempo |
| `FF 58 04 nn dd cc 08` | el compás: 4/4 es `04 02 18 08` |
| `FF 59 02 sf mi` | la armadura: sostenidos (negativos para bemoles), mayor o menor |
| `FF 7F len datos` | datos para un solo programa (sequencer-specific) |
| `FF 2F 00` | el final de la pista |

El tempo es el raro: no tiempos por minuto, sino cuánto dura un tiempo,
en microsegundos. Una canción cuyo tempo cambia recibe un evento de
tempo en cada paso donde cambia el tempo tocado, y los compases de la
canción entran en cada cambio; las dos cosas están en
[Tempo, compás, swing y sensación](tempo.md#en-archivos-y-en-archivos-midi).

La armadura dice cuántos sostenidos o bemoles, y si la tonalidad es
menor. `Scale::key_signature` la calcula dando la vuelta al círculo de
quintas ([Escalas, modos y tonalidades](../theory/scales.md)). Una
armadura solo conoce mayor y menor, así que un modo toma la armadura de
la tonalidad mayor de la que viene: Re dórico no tiene ninguna, como Do
mayor, y Mi dórico los dos sostenidos de Re mayor. Eso pierde algo: un
archivo que dice «dos sostenidos, mayor» no puede decir si quiere decir
Re mayor o Mi dórico. Así que junto a cada armadura el archivo recibe la
tonalidad exacta, en un evento **sequencer-specific**, el único lugar
que el estándar deja para los datos propios de un programa:

```rust
/// The start of our own event: 0x7D is the maker's number set aside for
/// programs that are not sold, then our name, so another program's
/// 0x7D event is never mistaken for ours.
const SCALE_EVENT_START: &[u8] = b"\x7DFerrous Lineup key ";

pub fn scale_event(scale: Scale) -> Vec<u8> {
    let mut data = SCALE_EVENT_START.to_vec();
    data.extend_from_slice(ron::to_string(&scale).unwrap_or_default().as_bytes());
    data
}

/// The key a sequencer-specific event holds, if [`scale_event`] wrote it.
pub fn scale_from_event(data: &[u8]) -> Option<Scale> {
    let key = data.strip_prefix(SCALE_EVENT_START)?;
    let text = std::str::from_utf8(key).ok()?;
    ron::from_str(text).ok()
}
```

La tonalidad se escribe como la escribe un archivo de proyecto, en RON,
así que el evento dice `7D "Ferrous Lineup key (root:E,kind:Dorian)"`,
y el código que guarda los proyectos la vuelve a leer
([Guardar](../program/saving.md)). `\x7D` en una cadena de bytes es el
byte 0x7D. `strip_prefix` da el resto del slice después del comienzo, o
`None` cuando el slice no empieza así; los `?` hacen que cada paso que
puede fallar termine la función con `None`. Los demás programas se
saltan el evento, y Ferrous Lineup recupera exactamente Mi dórico, Do♯
menor armónica o una serie dodecafónica. Una serie o un conjunto no
tienen armadura, así que para ellos este evento es todo lo que hay.

### Notas, batería y orden

Cada nota se convierte en dos eventos, su note-on y su note-off. Las
notas son las que toca la reproducción: `played_notes` las da con el
swing de la canción, `song_steps_of` cada paso de la canción donde un
trozo de la canción toca ese paso (una vez por repetición), y una nota
que pasa del final de su patrón, o del trozo que se toca, se corta ahí,
exactamente como la toca el hilo de audio:

```rust
let room = (length - note.start).min(play.end() - song_step);
let step = song_step as u32 * TICKS_PER_STEP;
let on = step + u32::from(note.nudge);
let off = if note.length <= room {
    on + note.ticks() as u32
} else {
    step + room as u32 * TICKS_PER_STEP
};
let pitch = drum.map_or(note.pitch, general_midi_drum);
```

`map_or(note.pitch, general_midi_drum)` da la altura propia de la nota
para un canal melódico, y para un tambor incorporado, su nota General
MIDI: los pasos de un tambor se escriben todos en Do4, que una batería
General MIDI toca como un bongó. Pasar la función `general_midi_drum`
misma, por su nombre, funciona porque su tipo, `fn(Drum) -> Note`, es lo
que pide `map_or`.

Después los eventos se ordenan por tiempo. En el mismo tick, los
note-offs tienen que ir antes que los note-ons: si una nota termina
justo donde la misma nota vuelve a empezar, «on, off» terminaría la
nota nueva enseguida, y solo «off, on» se oye dos veces.

```rust
events.sort_by_key(|&(tick, message)| (tick, matches!(message, MidiMessage::NoteOn { .. })));
```

La clave es una tupla, `(tick, is_note_on)`, y las tuplas se comparan
elemento a elemento. Para el mismo tick, `false < true`, así que ganan
los note-offs. `sort_by_key` es un orden *estable*: los eventos con
claves iguales conservan su orden, así que las notas de un acorde se
quedan en el orden en que se escribieron. El test
`a_repeated_note_ends_before_it_starts_again` comprueba el orden en el
tick donde un Do4 termina y vuelve a empezar.

En la aplicación, **Archivo > Exportar canción como MIDI…** y **Exportar
patrón como MIDI…** solo preguntan dónde guardar, y escriben los bytes
con `std::fs::write`. El nombre de un patrón podría llevar una `/`, que
un nombre de archivo no puede, así que el nombre que se ofrece para un
patrón la sustituye.

## Leer un archivo MIDI

**Archivo > Importar archivo MIDI…**, abrir un archivo `.mid` desde la
línea de órdenes, o soltarlo en la ventana, convierte un archivo MIDI en
un proyecto nuevo, listo para tocar. Los archivos MIDI son la forma más
fácil de meter música real en la aplicación: casi cualquier pieza
escrita se encuentra como uno, a menudo copiado de la partitura, a veces
tocado en un teclado.

Leer es escribir recorrido al revés, con una diferencia que cambia cómo
se escribe el código: **un archivo que leemos puede estar mal.** Puede
estar cortado por una descarga rota, puede decir que un chunk es más
largo de lo que es, puede no ser un archivo MIDI en absoluto. Sean
cuales sean los bytes, el programa no debe fallar.

### Un cursor que no puede caerse del final

El lector recorre los bytes con un pequeño *cursor*: un slice y una
posición en él.

```rust
struct Bytes<'a> {
    data: &'a [u8],
    at: usize,
}

impl<'a> Bytes<'a> {
    // ...

    fn byte(&mut self) -> Option<u8> {
        let byte = *self.data.get(self.at)?;
        self.at += 1;
        Some(byte)
    }

    /// The next `count` bytes, or `None` if there are fewer left.
    fn take(&mut self, count: usize) -> Option<&'a [u8]> {
        let end = self.at.checked_add(count)?;
        let slice = self.data.get(self.at..end)?;
        self.at = end;
        Some(slice)
    }
}
```

Dos ideas de Rust hacen esto seguro:

- **`'a` es un lifetime.** `Bytes<'a>` toma prestados los bytes del
  archivo; no los copia. El lifetime dice «un `Bytes` no puede vivir más
  que los bytes a los que apunta», y el compilador lo comprueba. `take`
  devuelve `&'a [u8]`: un slice de los bytes *originales*, que sigue
  siendo válido aunque el cursor avance.
- **`get` en lugar de indexar.** `data[at]` entra en pánico cuando `at` pasa
  del final; `data.get(at)` devuelve `None`. El `?` que le sigue
  devuelve ese `None` de `byte` en el acto. `checked_add` hace lo mismo
  con una longitud tan grande que sumarla desbordaría. Así que cada
  lectura da un valor o `None`, y quien llama decide qué significa un
  valor que falta.

Un tiempo delta se lee de vuelta pegando los grupos de siete bits:

```rust
fn variable_length(&mut self) -> Option<u32> {
    let mut value = 0u32;
    for _ in 0..4 {
        let byte = self.byte()?;
        value = (value << 7) | u32::from(byte & 0x7F);
        if byte & 0x80 == 0 {
            return Some(value);
        }
    }
    None
}
```

`81 00` es `1 << 7 | 0` = 128. El estándar permite cuatro bytes como
mucho; un quinto significaría que el archivo está dañado, así que tras
cuatro el bucle se rinde.

### La cabecera y los chunks

`read` comprueba `MThd`, lee el formato y la *división*, y luego lee
cada chunk hasta el final del archivo:

```rust
while let Some(id) = file.take(4) {
    let Some(length) = file.u32() else {
        damaged = true;
        break;
    };
    let length = length as usize;
    // A chunk that claims to be longer than what is left of the file
    // was cut short: read what there is.
    let data = file.take(length.min(file.remaining()));
    let data = data.unwrap_or_default();
    if data.len() < length {
        damaged = true;
    }
    // Chunks of other kinds may be added by future versions of the
    // standard, or by one program for itself: they are skipped.
    if id == b"MTrk" {
        let (track, complete) = read_track(data);
        damaged |= !complete;
        tracks.push(track);
    }
}
```

La cabecera también dice cuántas pistas hay, pero algunos programas
escriben mal ese número, así que se confía en los chunks mismos. Los
archivos reales a menudo están un poco rotos, y todos los programas que
los abren son indulgentes con ello. Este también: una pista cortada
conserva los eventos leídos hasta ahí, un chunk más largo que el archivo
se lee hasta donde llega, los chunks desconocidos se saltan. Pero el
resultado lo dice: `MidiFile::damaged` se activa, y el aviso tras la
importación termina con «El archivo está dañado: se leyó todo lo
anterior al daño». `damaged |= !complete` es el «o» booleano escrito
como asignación, como `+=`: una vez dañado, siempre dañado.

Dos cosas más que un lector encuentra por ahí:

- **Archivos RMID** (`.rmi`). Windows envolvía los archivos MIDI en
  RIFF, el contenedor de los archivos WAV: `RIFF`, un tamaño, `RMID`, y
  luego chunks. El archivo MIDI es el chunk llamado `data`. A diferencia
  de MIDI, RIFF escribe sus números con el byte menos significativo
  primero, así que `unwrap_riff` usa `u32::from_le_bytes` donde MIDI usa
  `u32::from_be_bytes`, y un chunk de longitud impar va seguido de un
  byte de relleno.
- **Ticks por segundo.** La división normalmente cuenta ticks por tiempo
  (480 y 960 son habituales). Con el bit superior a 1 cuenta en cambio
  ticks por fotograma de cine o video (24, 25, 29,97 o 30 fotogramas por
  segundo), como `Timing::PerSecond`. Un archivo así no tiene tiempos en
  absoluto; la importación cuenta su tiempo en tiempos de 120 BPM, para
  que suene a la velocidad correcta.

### Running status

Una pista de una pieza de piano es casi todo note on, note off, note
on... Para ahorrar espacio, un archivo puede omitir el byte de estado
cuando es el mismo que el anterior. El lector siempre puede saberlo,
porque un byte de estado tiene el bit superior a 1 y un byte de datos
no:

```rust
let (status, first_data) = if first & 0x80 != 0 {
    (first, bytes.byte())
} else {
    (running.unwrap_or(0), Some(first))
};
```

Así que `90 3C 64 00 40 64` son dos note-ons: Do4 y Mi4, los dos con
velocidad 100. Los program changes y el channel pressure tienen un byte
de datos, el resto de los mensajes de canal dos. Un metaevento (`FF`) y
un mensaje system exclusive (`F0` o `F7`, ajustes para los
sintetizadores de un fabricante, que se saltan) llevan su propia
longitud. Cualquier otro mensaje de sistema solo viaja por cables, nunca
en archivos, así que encontrar uno significa que la pista está dañada.

### Los eventos como enum

Cada evento que necesita la importación se convierte en una variante
de un enum, con sus datos dentro:

```rust
pub enum EventKind {
    NoteOn { channel: u8, key: u8, velocity: u8 },
    NoteOff { channel: u8, key: u8 },
    Controller { channel: u8, number: u8, value: u8 },
    Program { channel: u8, program: u8 },
    Tempo(u32),
    TimeSignature { beats: u8, unit: u8 },
    KeySignature { sharps: i8, minor: bool },
    ExactKey(Scale),
    TrackName(String),
    InstrumentName(String),
}
```

Todo lo demás (pitch bend, aftertouch, letras) se lee y se descarta. Un
note-on con velocidad 0 se convierte en un `NoteOff`, como en un cable.
`meta_event` usa patrones de slice para comprobar que los datos de un
metaevento tienen la forma correcta antes de fiarse de ellos:

```rust
// Sharps (negative for flats), then 0 for major or 1 for minor.
(KEY_SIGNATURE, &[sharps, mode]) => {
    let sharps = sharps as i8;
    ((-7..=7).contains(&sharps) && mode <= 1).then_some(EventKind::KeySignature {
        sharps,
        minor: mode == 1,
    })
}
```

Una armadura son exactamente dos bytes. `&[sharps, mode]` solo encaja
con un slice de longitud dos y nombra sus bytes; una armadura de tres
bytes, o una que dice tener 9 sostenidos, no es una armadura y se salta.
`sharps as i8` lee los mismos ocho bits como un número con signo, así
que el byte `0xFE` es -2: dos bemoles. `bool::then_some` da
`Some(valor)` cuando el `bool` es verdadero y `None` si no.

Los nombres de pistas e instrumentos son texto, y el estándar no dice
nada de cómo se escriben las letras que no son del inglés básico: los
programas nuevos usan UTF-8, los antiguos Latin-1, un byte por letra.
`text` prueba primero UTF-8, ya que un texto en Latin-1 casi nunca es
UTF-8 válido, y si no, convierte cada byte en el `char` con ese número,
que es exactamente Latin-1.

## Importar: de los eventos a un proyecto

`import(&file, title)` convierte un `MidiFile` en un `Project` y un
`Report` de lo que hizo. El informe se convierte en el aviso de la barra
de estado, como *«Clementi: 2 canales, 812 notas, 38 compases, 4/4, 120
BPM, Do mayor»*.

En la aplicación, leer el archivo, leer el MIDI e importar son tres
pasos que pueden fallar cada uno con un tipo de error distinto. Se
encadenan en un solo `Result`:

```rust
let imported = std::fs::read(path)
    .map_err(|err| err.to_string())
    .and_then(|bytes| midi_file::read::read(&bytes).map_err(|err| err.to_string()))
    .and_then(|file| {
        midi_file::import::import(&file, &title).map_err(|err| err.to_string())
    });
```

`and_then` ejecuta el paso siguiente solo si el anterior tuvo éxito, y
le pasa su valor. `and_then` conserva el tipo de error, así que cada
paso convierte primero su propio error en un `String` con `map_err`. Un
solo `match` al final muestra el aviso o el error. Leer tarda unas
milésimas de segundo, así que, a diferencia de
[importar una grabación](../theory/listen.md), se hace en el acto en
el hilo de la interfaz.

### Los ticks del archivo en los nuestros

Un archivo cuenta en sus propios ticks, a menudo 480 o 960 por tiempo;
un proyecto cuenta 96. `Clock` convierte unos en otros, al tick más
cercano:

```rust
Timing::PerBeat(per_beat) => {
    let per_beat = u128::from(per_beat);
    (u128::from(tick) * u128::from(TICKS_PER_BEAT) + per_beat / 2) / per_beat
}
```

Sumar la mitad del divisor antes de dividir redondea al entero más
cercano en lugar de hacia abajo. La cuenta se hace en `u128` porque un
archivo dañado puede decir que un tick está cerca de `u64::MAX`, y
multiplicarlo por 96 desbordaría un `u64`; luego el resultado se limita,
mucho más allá del final de cualquier canción.

### Notas entre pasos

Un patrón cuenta el tiempo en pasos. Redondear cada nota al paso más
cercano estropearía mucha música:

- **Los tresillos** dividen un tiempo en tres. Los tresillos de corcheas
  empiezan en 0, ⅓ y ⅔ de tiempo: ninguno es una semicorchea.
- **El swing** toca tarde la segunda corchea de cada tiempo.
- Un archivo **tocado en vivo** en un teclado tiene cada nota un poco
  adelantada o atrasada, y eso es gran parte de lo que lo hace sonar
  vivo.

Así que una nota guarda sus ticks exactos en `nudge` y `trim`. ¿Por qué
24 por paso? Porque 24 se divide entre 2, 3, 4, 6, 8 y 12. Un tresillo
de corcheas dura 32 ticks, uno de semicorcheas 16: los dos números
enteros, así que los tresillos caen *exactos*. `NoteEvent::exact`
construye una nota a partir de sus ticks exactos:

```rust
pub fn exact(pitch: Note, start: usize, ticks: usize) -> Self {
    let end = start + ticks.max(1);
    let first = start / TICKS_PER_STEP;
    let last = end.div_ceil(TICKS_PER_STEP);
    NoteEvent {
        nudge: (start % TICKS_PER_STEP) as u8,
        trim: (last * TICKS_PER_STEP - end) as u8,
        ..NoteEvent::new(pitch, first, last - first)
    }
}
```

El tresillo del tick 32 está en el paso 1 (`32 / 24`), desplazado 8
ticks (`32 % 24`). `div_ceil` divide redondeando hacia arriba, así que
`last` es el paso siguiente a aquel en que termina la nota. El piano
roll dibuja una nota desplazada donde empieza de verdad, el hilo de
audio la toca ahí
([El secuenciador](sequencer.md#cuánto-se-sostiene-una-nota)),
y **Herramientas > Cuantizar a los pasos** lleva las notas a la
cuadrícula cuando las quieres ahí.

![Un archivo cuyas notas están un poco fuera de la cuadrícula, como cuando una pieza se toca en un teclado: en el piano roll las notas empiezan un poco después o antes de las líneas de paso, y las barras de velocidad de abajo tienen todas alturas distintas.](../images/step-8-midi-played-in.png)

### Emparejar note on con note off

Un archivo no guarda notas. Guarda *tecla abajo* y *tecla arriba*, y la
importación tiene que emparejarlas. Mientras se lee una pista, cada
canal MIDI guarda las notas que aún suenan en cada tecla:

```rust
struct ChannelState {
    /// For each key, the notes sounding on it, oldest first. There is
    /// usually one at most, but a file can strike a key again before
    /// letting it go; each note off then ends the oldest note.
    open: Vec<Vec<Open>>,
    pedal: bool,
    notes: Vec<Played>,
}
```

Una tecla puede tener dos notas abiertas: dos voces de una fuga en la
misma nota, o un canon donde las partes se solapan. Entonces cada subida
termina la nota **más antigua** de esa tecla, la primera que entró es
la primera que sale. Terminar la más nueva cortaría notas allí donde las
partes se solapan: los cánones de las demostraciones no volverían como
están escritos.

Dos cosas más que la importación hace como lo haría un piano:

- **El pedal de sustain** (controlador 64). Mientras está abajo, una
  tecla soltada sigue sonando (`held_by_pedal`); la nota termina cuando
  sube el pedal, o cuando se vuelve a tocar la misma tecla. Sin esto,
  una pieza de piano tocada con pedal suena seca y entrecortada, porque
  los dedos dejaron las teclas mucho antes de que el sonido debiera
  parar.
- **Una nota soltada en el mismo tick en que empezó** (una nota de
  adorno, un golpe de batería) dura igualmente un tick, para que se oiga.

Levantar el pedal divide en dos la lista de cada tecla:

```rust
let (held, sounding): (Vec<Open>, Vec<Open>) =
    open.drain(..).partition(|note| note.held_by_pedal);
*open = sounding;
for note in held {
    self.end(key, note, tick);
}
```

`partition` manda cada elemento a una de dos colecciones, según el
closure, y conserva su orden. `drain(..)` vacía la lista a medida que
avanza, así que las notas se mueven, no se copian, y `*open = sounding`
vuelve a poner las que siguen sonando. Las teclas aún pulsadas cuando
termina una pista se sueltan ahí.

Una nota tocada dos veces en el mismo tick por error es una sola nota:
las notas se ordenan con `sort_by_key(|note| (note.start, note.pitch,
std::cmp::Reverse(note.end)))`, y `dedup_by_key` se queda con la primera
de cada `(start, pitch)`. `Reverse` da la vuelta a un orden, así que la
más larga de las dos va primero y es la que se queda.

### Pistas, canales y batería

Cada canal MIDI de cada pista que toca notas se convierte en un canal
del rack. La mayoría de los archivos son de **formato 1**, con una
pista por instrumento, así que cada pista se convierte en un canal, con
el nombre de la pista («Violin I»), o con el de su instrumento General
MIDI cuando la pista no tiene nombre. Un archivo de **formato 0** lo
pone todo en una pista, con los instrumentos en canales MIDI distintos;
ahí, cada canal MIDI se convierte en un canal del rack. Un rack tiene 32
canales; cuando un archivo tiene más partes de las que caben junto a su
batería, sus partes se juntan por canal MIDI.

El **program change** que eligió el instrumento de cada parte se guarda
como el `program` del canal
([SoundFonts y General MIDI](soundfonts.md)), y su
**volumen** (controlador 7) fija el volumen del canal, para que un
acompañamiento suave siga siéndolo. Los programas y los volúmenes a
veces se fijan en otra pista que las notas, así que el primero que se
fija en cada canal en cualquier parte del archivo es el de reserva:

```rust
EventKind::Program { channel, program } => {
    programs[usize::from(channel)].get_or_insert(program);
}
```

`Option::get_or_insert` pone el valor solo si aún no hay ninguno, así
que gana el primer program change sin un `if`.

El canal MIDI 10 es el canal de batería: cada tecla es un sonido de
batería. La importación da a cada tambor incorporado que usa el archivo
un canal propio, como en un proyecto nuevo, así que la batería aparece
en el rack de canales como un secuenciador por pasos, cada golpe de un
paso. Los sonidos que no tienen los tambores incorporados van al más
cercano (`general_midi_drum_sound`: un triángulo al ride, una pandereta
al shaker, los bongós a la conga); un sonido sin nada cercano, como un
silbato, se deja fuera, y el aviso lo cuenta.

Unos pocos archivos de internet tienen millones de notas, escritos como
proeza, y harían el piano roll demasiado lento para usarlo. La
importación se queda con las 50 000 primeras (`MAX_NOTES`).
`keep_first_notes` ordena los ticks de inicio, toma el inicio de la
última nota que cabe como corte, y usa `partition_point` (una búsqueda
binaria del primer elemento que no pasa una prueba) para contar cuántas
notas del tick de corte aún caben.

### Tempo, compás y tonalidad

Los cambios de tempo, los compases y las armaduras se aplican a toda la
canción, sea cual sea la pista que los lleve. Una función genérica reúne
los eventos de una clase de todas las pistas:

```rust
fn song_events<T>(tracks: &[Track], pick: impl Fn(&EventKind) -> Option<T>) -> Vec<(u64, T)> {
    let mut events: Vec<(u64, T)> = tracks
        .iter()
        .flat_map(|track| &track.events)
        .filter_map(|event| pick(&event.kind).map(|value| (event.tick, value)))
        .collect();
    // A stable sort keeps events on the same tick in the order they were
    // read, so the last one written wins.
    events.sort_by_key(|&(tick, _)| tick);
    events
}
```

`pick` dice qué eventos conservar y qué sacar de ellos, así que la misma
función sirve para los tempos (`u32`), los compases y las tonalidades.
Cada clase va luego a un `BTreeMap` por paso, donde un evento posterior
en el mismo paso sustituye a uno anterior, como en la reproducción. Cada
cambio de tempo se convierte en una marca de tempo, cada compás en una
marca de compás
([Tempo, compás, swing y sensación](tempo.md)), y `tidy` deja
fuera las marcas que no cambian nada: los archivos a menudo repiten el
compás al comienzo de cada sección. Un compás que la aplicación no puede
contar, con compases que no son un número entero de semicorcheas, se
reduce cuando se puede: 6/32 pasa a 3/16.

La tonalidad sale de las armaduras, tal como están escritas: dos bemoles
y menor es Sol menor, y los sostenidos o bemoles de la armadura deciden
la escritura, así que seis bemoles son Sol♭ mayor y seis sostenidos Fa♯
mayor. Cada armadura posterior que nombra otra tonalidad se convierte en
una marca de tonalidad en la lista de reproducción. La tonalidad exacta
de un archivo que escribió Ferrous Lineup gana a la armadura que tiene
al lado. Dos casos se dejan a las notas:

- **Un archivo sin ninguna armadura.**
- **Un archivo que dice Do mayor pero no lo es.** Muchos programas
  escriben «sin sostenidos, mayor» en cada archivo, lo haya elegido
  alguien o no. Así que cuando todas las armaduras del archivo dicen Do
  mayor, la importación lo contrasta con las notas, y si otra tonalidad
  encaja *claramente* mejor (por `CLEARLY_BETTER`, 0,1 de correlación)
  se queda con esa, y el aviso dice «deducido de las notas».

Cómo deciden las notas una tonalidad, con perfiles de tonalidad y el
método de Krumhansl-Schmuckler, está en [Oír la armonía](../theory/hearing.md). La
importación suma cuánto suena cada una de las doce clases de altura, y
le pregunta a `key_fits` qué tonalidad encaja mejor.

### Cortar la canción en patrones

Un patrón tiene como mucho 256 pasos (16 compases de 4/4), y una canción
puede durar 200 compases. Así que la importación corta la canción en
patrones en las **barras de compás**, de 16 compases como mucho cada
uno, y los coloca uno tras otro en la lista de reproducción.

¿Dónde cortar? Una nota que suena a través del corte tiene que partirse
en dos, lo que no afecta al sonido pero es feo de editar. Así que entre
las barras de compás de la segunda mitad de cada tramo, `chunks` elige
la que menos notas cruzan, la última de ellas. Para contarlas en todos
los pasos a la vez usa un **array de diferencias**:

```rust
// How many notes cross each step's start: a difference array, +1
// where a note starts crossing and -1 after it stops.
let mut change = vec![0i64; length + 2];
for note in notes {
    let first = note.start / TICKS_PER_STEP + 1;
    let last = note.end.div_ceil(TICKS_PER_STEP).saturating_sub(1);
    if first <= last && first <= length {
        change[first] += 1;
        change[(last + 1).min(length + 1)] -= 1;
    }
}
let mut crossing = Vec::with_capacity(length + 1);
let mut count = 0;
for &delta in change.iter().take(length + 1) {
    count += delta;
    crossing.push(count);
}
```

Para cada nota, +1 en el primer límite de paso que cruza y -1 después
del último; sumar los números desde la izquierda da, en cada paso,
cuántas notas suenan a través de su comienzo. Es una pasada por las
notas y otra por los pasos, en lugar de comprobar cada nota en cada
paso. La elección es luego `(earliest..=last).rev().min_by_key(...)`:
`min_by_key` devuelve el *primero* de varios mínimos iguales, y recorrer
los compases hacia atrás hace que el primero sea el último compás.

`place_patterns` llena los patrones, partiendo las notas que cruzan un
corte. Un tramo que vuelve **nota por nota** (una estrofa repetida, un
da capo) toca otra vez el mismo patrón en lugar de una copia, como
construirías la canción a mano. Cada patrón lleva el nombre de sus
compases, como «Compases 1–16». La batería de cada trozo va en un
patrón propio, «Drums, bars 1–16», en una segunda pista llamada Drums:
así la batería de una banda puede ocupar su lugar, porque la banda
silencia las pistas que solo tienen batería (ver [La
banda](../band/band.md)).

![Eine kleine Nachtmusik de Mozart, importada de un archivo MIDI, en el piano roll. Encima de las notas, el carril de acordes nombra cada acorde: G, D, G, D, G, C, A, D... El cuadro de tonalidad dice Sol mayor. Debajo, el carril de velocidad muestra con qué fuerza se tocó cada nota.](../images/step-8-midi-mozart.png)

Como las notas caen en patrones normales, todo lo que ya entiende de
notas funciona con ellas: el carril de **Acordes** del piano roll nombra
el acorde de cada paso, y las teclas de armonía iluminan el acorde que
suena mientras se toca la canción.

## Cómo se prueba

- `midi.rs`: `parse` con note-ons, note-offs, un note-on silencioso, y
  mensajes que debe ignorar (un pedal de sustain, un pitch bend, un tic
  de reloj, un mensaje demasiado corto, un byte de datos fuera de
  rango), y cada mensaje a bytes y de vuelta.
- `record.rs`: las reglas de cuantización, con posiciones inventadas.
- `engine.rs`: `keys_play_the_live_channel_until_they_come_up`,
  `a_key_comes_up_on_the_channel_it_went_down_on`,
  `a_key_up_leaves_the_sequencers_notes_alone`,
  `keys_from_a_midi_keyboard_skip_the_command_queue`,
  `removing_channels_lets_go_of_their_keys`,
  `midi_out_channels_send_their_notes`,
  `removing_a_midi_out_ends_its_notes`, y los dos tests de que un
  mensaje MIDI espera a que haya sitio en lugar de perderse, en orden.
- `instrument.rs`: note-offs tras el gate, una nota repetida, el program
  change antes de la primera nota y después de cada parada, un
  sintetizador o tambor enviado a MIDI que no suena y envía sus notas.
- `midi_file.rs`: los números de longitud variable de la especificación,
  y luego archivos enteros, leídos por un pequeño lector escrito en el
  módulo de tests: pistas, program changes, cambios de tempo, compases y
  armaduras, la tonalidad exacta, la batería en el canal 10, clips y
  repeticiones, una nota repetida, una sección tocada una vez, y los
  canales MIDI de cada clase de canal.
- `read.rs`: running status, metaeventos, system exclusive, un archivo
  RMID, ticks por segundo, y `a_file_cut_short_keeps_what_was_read`,
  que corta un archivo en *cada* longitud de 0 bytes al archivo entero y
  lee cada una sin un panic.
- `import.rs`: archivos pequeños hechos a mano para lo que una
  demostración nunca tiene: tresillos, el pedal de sustain, un archivo
  de formato 0, batería, tonalidades con y sin armadura, una canción
  larga cortada en barras de compás, un tramo que vuelve, un archivo sin
  notas.

La mejor prueba de una importación es una exportación.
`every_demo_comes_back_note_for_note` exporta cada demostración de
canción como archivo MIDI, la importa, exporta otra vez el resultado y
compara los dos archivos evento a evento: cada note on con su tecla y
su velocidad, cada note off, cada cambio de tempo, compás, armadura y
program change, cada uno en el mismo tick. También comprueba que el
proyecto importado tiene exactamente la tonalidad y las marcas de
tonalidad de la demostración.

Aun así, un lector escrito por la misma persona que escribió el escritor
puede compartir sus errores. Así que vale la pena abrir los archivos
exportados en un programa que no sabe nada de Ferrous Lineup (la
biblioteca `mido` de Python lee uno en pocas líneas), y archivos de
otros programas en la aplicación.

### Probar sin sintetizador

Un test puede comprobar los bytes que enviaría un instrumento, pero no
que lleguen a un puerto. Para eso, Linux tiene el secuenciador de ALSA,
y dos módulos del kernel que crean dispositivos MIDI de mentira:
`snd-seq-dummy` (un puerto «Midi Through» que lo deja pasar todo) y
`snd-virmidi` (puertos que parecen un teclado o un sintetizador USB).
`aseqdump` imprime cada mensaje que llega a un puerto:

```sh
sudo modprobe snd-seq-dummy snd-virmidi
aseqdump -p "Midi Through"
```

Elige **Midi Through** en la **Salida** del menú **MIDI**, pulsa
Reproducir, y cada nota aparece al enviarse:

```text
129:0   Note on                 9, note 36, velocity 100
129:0   Note on                 0, note 60, velocity 100
129:0   Note off                9, note 36, velocity 64
```

`aseqdump` cuenta los canales desde 0, así que su 9 es el canal MIDI
10, la batería. Escribir bytes en el dispositivo en bruto de un puerto
virtual toca una tecla en un teclado de mentira: elige **Virtual Raw
MIDI 0-0** como entrada, y luego

```sh
printf '\x90\x45\x50' > /dev/snd/midiC0D0   # A4 down, velocity 80
printf '\x80\x45\x00' > /dev/snd/midiC0D0   # and up
```

![El menú MIDI: las entradas y salidas de un equipo con puertos MIDI virtuales, Enviar todos los canales, Teclado del equipo como piano y El acompañamiento sigue los acordes.](../images/step-8-midi-out-menu.png)
