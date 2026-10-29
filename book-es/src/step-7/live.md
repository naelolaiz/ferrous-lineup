# Live keys: hilos de entrada y de salida

Archivos: [`crates/lineup-app/src/midi.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/midi.rs),
[`crates/lineup-app/src/app/live.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/live.rs),
[`crates/lineup-audio/src/engine.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-audio/src/engine.rs)
y [`crates/lineup-audio/src/instrument.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-audio/src/instrument.rs)

Hasta ahora, cada nota venía del sequencer, que vive en el hilo de
audio y conoce la longitud de cada nota antes de que empiece. Una tecla
de un teclado es distinta en dos cosas: viene de fuera, en un hilo que
Ferrous Lineup no posee, y nadie sabe cuánto tiempo se mantendrá
pulsada. Este capítulo sigue a una tecla desde el cable hasta el
altavoz, y a una nota de salida MIDI desde el patrón hasta el cable.

## Por qué las teclas se saltan la UI

El camino obvio para una tecla sería el que toma cualquier otro cambio:
la UI se entera y envía un `Command` al hilo de audio. Pero la UI solo
se ejecuta una vez por frame. A 60 frames por segundo son hasta 17 ms
de espera, y en un equipo lento mucho más, antes de que el hilo de
audio se entere siquiera de la tecla. Un pianista lo nota.

El hilo de audio se ejecuta mucho más a menudo: una vez por *bloque*,
cada pocos milisegundos. Así que una tecla de un teclado MIDI va
directa allí, y la UI recibe después una copia, solo para mostrarla y
grabarla:

```text
  hilo de midir ──▶ LiveInput ──▶ hilo de audio        (lo reproduce)
        └────────▶ canal mpsc ──▶ hilo de la UI         (lo muestra y lo graba)
```

El teclado de escritura no puede hacer esto: egui lee el teclado, y
egui se ejecuta en el hilo de la UI. Así que las teclas escritas toman
el camino lento, como un `Command::Live(MidiMessage)`. En el hilo de
audio, ambas acaban en el mismo método, `play_live`.

## Un emisor por cola: `LiveInput`

La cola de comandos del [paso 1](../step-1/audio-engine.md#enviar-comandos-con-un-ring-buffer)
es un ring buffer del crate `rtrb`, y su `Producer` pertenece al
`AudioEngine`, en el hilo de la UI. rtrb es *un solo emisor, un solo
receptor*: un hilo mete, un hilo saca, y eso es lo que le permite
funcionar sin cerrojos. El hilo de midir es un segundo hilo que quiere
meter, así que necesita un ring buffer propio:

```rust
pub struct LiveInput {
    notes: rtrb::Producer<MidiMessage>,
    position: Arc<AtomicU64>,
}

impl LiveInput {
    pub fn send(&mut self, message: MidiMessage) -> bool {
        self.notes.push(message).is_ok()
    }
}
```

Como solo puede haber un emisor, esto no puede ser un método de
`AudioEngine` que cualquiera pueda llamar. Es un *valor*, y quien lo
posea es el emisor. El motor lo crea en `start` y lo guarda en un
`Option` hasta que alguien lo pide:

```rust
pub fn take_live_input(&mut self) -> Option<LiveInput> {
    self.live_input.take()
}
```

`Option::take` saca el valor y deja un `None` (el paso 5 lo usó para el
hilo de exportación), así que la segunda llamada no recibe nada. El
sistema de tipos hace imposible romper la regla de «un solo emisor»:
`LiveInput` no implementa `Clone`, así que no puede copiarse, solo
moverse. `MidiOutput`, la salida de los canales de salida MIDI, funciona
igual desde el otro extremo: guarda el `Consumer` de la cola.

En el hilo de audio, el receptor también es un `Option`, porque la
exportación WAV del paso 5 ejecuta un hilo de audio sin teclado alguno:

```rust
while let Some(message) = self.live_input.as_mut().and_then(|keys| keys.pop().ok()) {
    self.play_live(message);
}
```

Esa línea condensa tres pasos. `self.live_input` es un
`Option<Consumer>`, y llamar a `pop` necesita un `&mut Consumer`.
`as_mut()` convierte `&mut Option<Consumer>` en `Option<&mut Consumer>`,
tomando prestado lo de dentro en lugar de sacarlo. `and_then` ejecuta
el closure solo si hay un receptor, y el `Option` del propio closure
(de `pop().ok()`: `Some(key)` o `None` cuando la cola está vacía) se
convierte en el resultado. Así que el bucle sigue mientras haya un
receptor *y* tenga una tecla.

## Compartir un float: la posición en un `AtomicU64`

La grabación necesita saber *cuándo* llegó cada tecla, como un lugar
del patrón: «a mitad del paso 4». El cursor de reproducción del paso 2 es un paso
entero, un `AtomicUsize`. La posición es un `f64`, y Rust no tiene
floats atómicos.

Sí tiene `AtomicU64`, y un `f64` son 64 bits. `f64::to_bits` da esos
bits como un `u64` sin cambiarlos, y `f64::from_bits` los convierte de
vuelta:

```rust
// audio thread, after each block while playing:
self.position.store(self.sequencer.position().to_bits(), Ordering::Relaxed);

// any other thread:
fn load_position(position: &AtomicU64) -> Option<f64> {
    match position.load(Ordering::Relaxed) {
        NO_POSITION => None,
        bits => Some(f64::from_bits(bits)),
    }
}
```

«Detenido» es `NO_POSITION = u64::MAX`, igual que `NO_STEP` para el cursor de reproducción. Como float, esos bits son un NaN («not a number»), que
ninguna posición es nunca, así que los dos no pueden confundirse.

La posición en sí viene del sequencer, que cuenta hacia atrás los
frames hasta el siguiente paso:

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

`next_step` es el paso a punto de dispararse, así que el que está
sonando es el anterior. `next_step - 1` se iría por debajo de cero en
el paso 0, cosa que un `usize` no puede hacer (entraría en pánico en
una build de depuración). Sumar `length` primero y quedarse con el
resto retrocede un paso *dando la vuelta al bucle*: del paso 0 al último
paso. `passed` es cuánto de ese paso ha pasado, de 0 a 1.

La posición se guarda una vez por bloque, así que cuando llega una
tecla puede tener hasta un bloque de antigüedad: una centésima de
segundo. Un paso a 120 BPM dura un octavo de segundo, y la grabación
redondea al paso más cercano de todos modos, así que es demasiado poco
como para cambiar dónde cae una nota.

## Una gate que nunca se agota

Las notas del sequencer tienen una *gate*, el número de frames que se
mantienen, que la voz cuenta hacia atrás. Una tecla no tiene longitud
conocida, así que recibe una gate que significa «hasta que se diga lo
contrario»:

```rust
pub const HELD: u32 = u32::MAX;
```

La voz del sintetizador se salta la cuenta atrás para ella:

```rust
if matches!(self.stage, Stage::Attack | Stage::Decay) && self.gate != HELD {
    ...
}
```

y el nuevo `note_off` solo suelta las voces sostenidas:

```rust
pub fn note_off(&mut self, pitch: Note) {
    for voice in self.voices.iter_mut() {
        if voice.pitch == pitch
            && voice.gate == HELD
            && matches!(voice.stage, Stage::Attack | Stage::Decay)
        {
            voice.stage = Stage::Release;
        }
    }
}
```

La parte de `voice.gate == HELD` importa cuando tocas sobre el patrón.
Si el patrón toca un Do4 largo y tú pulsas Do4 en el teclado, soltar tu
tecla no debe terminar la nota del patrón. La prueba
`a_key_up_leaves_the_sequencers_notes_alone` comprueba exactamente eso.
Las voces ahora recuerdan su `pitch`, que antes no necesitaban. El
sampler ignora `note_off`: un golpe de batería siempre suena hasta el
final.

## En qué canal sube una tecla: `live_keys`

Las teclas tocan el *canal seleccionado*, el que muestra el piano roll.
La UI se lo dice al hilo de audio con `Command::SetLiveChannel` cada
vez que cambia. Pero la selección puede cambiar con una tecla pulsada.
Si la tecla subiera entonces en el canal nuevo, la nota del canal
antiguo sonaría para siempre. Así que el hilo de audio recuerda, para
cada una de las 128 notas, en qué canal bajó su tecla:

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
`if let ... && let ...` es una let chain, como en el paso 6.

Los canales se conocen por su posición en la lista, e insertar o quitar
un canal mueve los que vienen detrás. Un «canal 2» recordado podría ser
entonces otro canal, o ninguno. Así que antes de que cambie la lista,
`release_live_keys` suelta todas las teclas que están pulsadas. La
prueba `removing_channels_lets_go_of_their_keys` quita el canal 0 con
una tecla pulsada en el canal 1, que entonces *se convierte en* el
canal 0.

## Salida MIDI: un instrumento con bandeja de salida

Un canal de salida MIDI no suena. Su instrumento convierte las notas en
mensajes MIDI para otro sintetizador. Es un tercer tipo de
`Instrument`, junto al sampler y al sintetizador, así que todo lo demás
(el sequencer, las live keys, el volumen, el silencio) funciona con él
sin cambios:

```rust
pub enum Instrument {
    Sampler(Sampler),
    Synth(Synth),
    MidiOut(MidiOut),
}
```

El hilo de audio no debe enviar los mensajes él mismo. Enviar pasa por
la capa MIDI del sistema operativo, una llamada al sistema que puede
tardar lo que sea, y la [regla de oro](../step-1/audio-engine.md#la-regla-de-oro-del-hilo-de-audio)
prohíbe esperar. Así que `MidiOut` junta sus mensajes en una *bandeja
de salida*, un `Vec` con espacio reservado cuando se crea en el hilo de
la UI:

```rust
fn post(&mut self, message: MidiMessage) {
    let outbox = &mut self.state.outbox;
    if outbox.len() < outbox.capacity() {
        outbox.push(message);
    }
}
```

Comprobar `len() < capacity()` antes de meter significa que el `Vec`
nunca crece, así que nunca reserva memoria. Si alguna vez estuviera
lleno, se perdería un mensaje antes que romper la regla.

Un sintetizador solo termina una nota cuando se lo dicen, así que
`MidiOut` debe enviar un note-off cuando se agota la gate de cada nota.
Guarda, para cada nota, un enum que dice si la nota está sonando y
hasta cuándo:

```rust
enum Sounding {
    Off,
    Until(u64), // the frame when its note-off is due
    Held,       // until note_off, for live keys
}
```

`next_sample` no produce sonido, pero cuenta los frames en `now`, que
es el reloj de `Until`. Una vez por bloque, el hilo de audio pide a
cada instrumento que entregue sus mensajes:

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
    for message in self.state.outbox.drain(..) {
        send(message);
    }
}
```

`drain(..)` saca todos los elementos del `Vec` en orden, y el `Vec`
conserva su memoria para el siguiente bloque. `send` es un
`impl FnMut`: al instrumento ni le va ni le viene adónde van los
mensajes. El hilo de audio le pasa un closure que los mete en la cola
`MidiOutput`, y las pruebas le pasan uno que los mete en un `Vec`
normal para mirarlos:

```rust
// audio thread
fn send_midi(&mut self) {
    for channel in &mut self.channels {
        channel.instrument.send_midi(|message| {
            if let Some(out) = &mut self.midi_out {
                let _ = out.push(message);
            }
        });
    }
}
```

El bucle toma prestado `self.channels` mutablemente, y el closure toma
prestado `self.midi_out` mutablemente, los dos a la vez. Eso está
permitido: desde la edición de 2021, un closure captura solo los campos
que usa, no todo `self`, y dos campos distintos son [préstamos disjuntos](../step-3/voices.md).

En MIDI hay una de cada nota por canal. Si una nota empieza de nuevo
mientras aún suena, el sintetizador oiría «on, on, off», y la segunda
nota se detendría con el note-off de la primera. Así que `note_on`
envía antes un note-off en ese caso
(`midi_out_ends_a_repeated_note_before_starting_it_again`).

Dos detalles finales evitan que las notas se queden colgadas en el
sintetizador externo. Cuando se quita un canal de salida MIDI, o se sustituye
su instrumento, o se carga un proyecto nuevo, `silence` envía sus
note-off antes de desechar el instrumento:

```rust
fn silence(instrument: &mut Instrument, midi_out: &mut Option<rtrb::Producer<MidiMessage>>) {
    instrument.release_all();
    instrument.send_midi(|message| {
        if let Some(out) = midi_out {
            let _ = out.push(message);
        }
    });
}
```

Y el estado vive en un `Box`:

```rust
pub struct MidiOut {
    channel: u8,
    now: u64,
    state: Box<MidiOutState>, // 128 `Sounding`s and the outbox
}
```

128 `Sounding` ocupan 2 KB. Un enum es tan grande como su variante más
grande, así que sin el box, cada `Instrument`, y cada `Command` que
lleva uno por el ring buffer, ocuparía 2 KB. El motor tiene una prueba
que mantiene un comando por debajo de 80 bytes, y sin el box fallaría.

## midir: puertos y conexiones

[midir](https://docs.rs/midir) es el crate que habla con la capa MIDI
de cada sistema: ALSA en Linux, Core MIDI en macOS, Windows Multimedia
en Windows. Está construido como `cpal`: se listan los puertos, se abre
una conexión con uno, y la conexión sigue abierta mientras se conserve
el valor. Ferrous Lineup lo envuelve en un struct `Midi`, en
`lineup-app/src/midi.rs`.

Los puertos se vuelven a encontrar por un id que da midir, un `String`,
porque la lista de puertos cambia según se enchufan y desenchufan
dispositivos. El menú MIDI busca los puertos cada vez que se abre.

### Conectar consume, cerrar devuelve

```rust
let mut midi_in = MidiInput::new(CLIENT_NAME).map_err(|err| err.to_string())?;
midi_in.ignore(Ignore::All);
let found = midi_in
    .find_port_by_id(&port.id)
    .ok_or_else(|| format!("{} is not connected any more", port.name))?;
let connection = midi_in
    .connect(&found, "input", on_message, link)
    .map_err(|err| err.to_string())?;
```

`connect` recibe `self`, no `&self`: el `MidiInput` se consume y vive
dentro de la conexión desde entonces. `close` lo devuelve, con los
datos. Esto es la propiedad usada como máquina de estados: un
`MidiInput` conectado no puede conectarse dos veces, porque ya no lo
tienes.

`Ignore::All` deja fuera los mensajes que no son notas (tics de reloj,
datos system exclusive, y «active sensing», un teclado que dice «sigo
aquí» tres veces por segundo), así que el hilo de midir no se despierta
por ellos.

`on_message` es una función normal, y `link` son los datos que recibe
cada vez, como `&mut InputLink`:

```rust
fn on_message(_microseconds: u64, bytes: &[u8], link: &mut InputLink) {
    let Some(message) = MidiMessage::parse(bytes) else {
        return;
    };
    let mut position = None;
    if let Some(live) = &link.live {
        let mut live = lock(live);
        live.send(message);
        position = live.position();
    }
    let _ = link.to_ui.send(Played { message, position });
    link.ctx.request_repaint();
}
```

Se ejecuta en el hilo de midir, así que `connect` pide que sea
`Send + 'static`, como el callback de audio del paso 1. El tiempo que
pasa midir se ignora: la posición del hilo de audio es el reloj que
importa (mira [el siguiente capítulo](recording.md)).

### Por qué el `LiveInput` está detrás de un `Mutex`

`InputLink` guarda el `LiveInput` como `Arc<Mutex<LiveInput>>`, lo que
parece raro para un valor que solo usa un hilo a la vez. La razón son
los errores de midir. Si `connect` falla, el error devuelve el
`MidiInput` (`ConnectError::into_inner`), pero *no* los datos: `link`
desaparecería, y con él el único emisor de la cola live, para siempre.
No se podría volver a conectar ningún teclado jamás.

Así que el struct `Midi` guarda el `LiveInput` en sí, detrás de un
`Arc<Mutex<>>`, y cada link guarda un clon del `Arc`. Una conexión
fallida solo pierde un clon. Solo existe una conexión a la vez, así que
nadie espera nunca al cerrojo, y el hilo de midir no es el hilo de
audio, así que puede bloquear.

### Cerrojos envenenados

```rust
fn lock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex.lock().unwrap_or_else(PoisonError::into_inner)
}
```

Si un hilo entra en pánico mientras sostiene un cerrojo, Rust marca el
`Mutex` como **envenenado**, y cada `lock()` posterior devuelve un
`Err`: los datos de dentro pueden estar a medio cambiar, y Rust no los
entregará sin preguntar. `PoisonError::into_inner` es la manera de
decir «dámelo igualmente». Aquí es seguro, porque un extremo de cola y
una conexión nunca se quedan a medio cambiar. El helper es genérico
sobre `T`, así que sirve para los dos mutexes de este archivo.

### Una copia para la UI: `mpsc`

La copia de cada tecla para la UI pasa por un canal
`std::sync::mpsc` («multiple producer, single consumer»), de la
librería estándar:

```rust
let (to_ui, from_keyboard) = mpsc::channel();
...
pub fn played(&self) -> Vec<Played> {
    self.from_keyboard.try_iter().collect()
}
```

A diferencia de rtrb, reserva memoria y puede bloquear, lo que aquí
está bien: ni el hilo de midir ni el hilo de la UI son de tiempo real.
Y el `Sender` puede clonarse, así que cada conexión nueva recibe el
suyo. `try_iter` entrega los mensajes que esperan y se detiene cuando
no hay ninguno, sin esperar más: lo correcto para una UI que no debe
dejar de dibujar.

egui solo dibuja un frame cuando algo ocurre: el ratón se mueve, se
escribe una tecla. Una tecla de un teclado MIDI no es algo que egui
vea, así que `on_message` llama a `ctx.request_repaint()`.
`egui::Context` es un handle que puede clonarse y enviarse a otros
hilos, y `request_repaint` es una de las pocas llamadas de egui
pensadas para usarse desde ellos.

## El hilo de envío

Los mensajes que los canales de salida MIDI dejan en la cola `MidiOutput` los
envía un hilo propio, llamado `midi-out`:

```rust
fn start_sending(mut messages: MidiOutput, output: Arc<Mutex<Option<MidiOutputConnection>>>) {
    let sender = move || {
        while !messages.is_abandoned() {
            thread::sleep(SEND_INTERVAL);
            let mut output = lock(&output);
            while let Some(message) = messages.pop() {
                if let Some(connection) = output.as_mut() {
                    let _ = connection.send(&message.bytes());
                }
            }
        }
    };
    let _ = thread::Builder::new()
        .name("midi-out".to_string())
        .spawn(sender);
}
```

Duerme un milisegundo, envía lo que esté esperando, y vuelve a
empezar. Un hilo que espera trabajo normalmente dormiría hasta que lo
despierten, con un `Condvar`. Pero despertar a un hilo es una llamada
al sistema, que el hilo de audio no debe hacer. Mirar mil veces por
segundo cuesta casi nada, y añade como mucho un milisegundo.

El cerrojo se toma después de dormir, no antes, así que el hilo de la
UI nunca espera a un hilo que duerme. El guard `output` se suelta al
final de cada vuelta del bucle, lo que lo desbloquea.

`is_abandoned()` es true cuando el otro extremo de la cola desaparece:
el motor de audio se soltó, como al cerrar la aplicación. Entonces
termina el bucle, y con él el hilo. Nada tiene que decirle que se
detenga.

`thread::Builder` es la manera larga de lanzar un hilo. Puede darle un
nombre al hilo, que se ve en los depuradores y en herramientas como
`top -H`. Y su `spawn` devuelve un `Result` donde `thread::spawn`
entraría en pánico si el sistema se negara. Si eso llegara a pasar,
la salida MIDI se queda en silencio y todo lo demás funciona, lo que es mejor
que un cierre inesperado al arrancar.

### Cambiar de puerto sin notas colgadas

La conexión con el puerto de salida está detrás de un `Mutex` porque la
usan dos hilos: el hilo de envío envía, y el hilo de la UI la sustituye
cuando eliges otro puerto:

```rust
let old = std::mem::replace(&mut *lock(&self.output), new);
if let Some(mut old) = old {
    for channel in 0..16 {
        let _ = old.send(&all_notes_off(channel));
    }
    old.close();
}
```

`&mut *lock(&self.output)` bloquea el mutex, sigue el guard hasta el
`Option` de dentro (`*`) y lo toma prestado mutablemente, todo en una
expresión; el guard se suelta al final de la sentencia, así que el
cerrojo se mantiene justo lo necesario para el intercambio. El hilo de
envío manda entonces al puerto viejo o al nuevo, nunca a medias a cada
uno. Antes de cerrar la conexión vieja, recibe All Notes Off en los 16
canales, para que un sintetizador que estaba tocando una nota no la
siga tocando para siempre.

El lado de entrada tiene su propia versión: cuando eliges otro teclado
MIDI, se sueltan todas las teclas pulsadas en el viejo
(`release_all_keys`), porque sus key-up no llegarían nunca.

## El teclado de escritura

La lectura de las teclas de letras ocurre una vez por frame, en
`typing_keyboard`. egui guarda en una lista los eventos de teclado de
este frame, y el código elige los que quiere:

```rust
egui::Event::Key {
    key,
    physical_key,
    pressed,
    repeat: false,
    modifiers,
} => {
    let key = physical_key.unwrap_or(*key);
    let shortcut = modifiers.command || modifiers.alt;
    (!*pressed || !shortcut).then_some((key, *pressed))
}
```

Cuatro detalles hacen que se comporte como un piano:

- **`repeat: false`** en el patrón. Una tecla mantenida hace que el
  sistema la repita («aaaaa» en un campo de texto). Un campo escrito
  como valor en un patrón debe ser igual a ese valor, así que las
  repeticiones no encajan, y una tecla mantenida suena una vez.
- **`physical_key`**. `key` es la letra que la distribución del teclado
  imprime en la tecla; `physical_key` es dónde *está* la tecla. En un
  teclado francés, la tecla donde un teclado inglés tiene la Z dice W.
  Usar dónde está mantiene la forma del piano en cualquier
  distribución. Algunos sistemas no lo informan, así que `unwrap_or`
  recurre a la letra.
- **Los atajos no son notas.** Ctrl+Z es deshacer. Una tecla que baja
  con Ctrl (Cmd en un Mac) o Alt pulsado se omite. Una tecla que *sube*
  siempre cuenta, se tenga lo que se tenga pulsado, o una nota empezada
  antes de pulsar Ctrl no terminaría nunca.
- **El foco.** Si la ventana pierde el teclado (haces clic en otra
  ventana), o un campo de texto se lo queda (al renombrar un patrón),
  los eventos de «subir» de las teclas van a otra parte. Así que todas
  las teclas de escritura se sueltan de golpe cuando eso ocurre.

Los eventos se leen dentro de `ctx.input(|input| ...)`, que mantiene un
cerrojo sobre la entrada de egui mientras se ejecuta el closure. El
closure solo junta un `Vec` de `(key, pressed)`, y el bucle que toca
las notas se ejecuta después, con el cerrojo liberado.

Soltar todas las teclas usa otra vez `std::mem::take`:

```rust
fn release_typing_keys(&mut self) {
    for (_, pitch) in std::mem::take(&mut self.live.typing_down) {
        self.play_typed(MidiMessage::NoteOff { channel: 0, pitch });
    }
}
```

Recorrer `&self.live.typing_down` tomaría prestado `self` durante todo
el bucle, y `play_typed` necesita `&mut self`. Sacar antes la lista
deja una vacía en su lugar, y el bucle posee la vieja, así que las dos
nunca se solapan.

## El canal seleccionado

Las teclas necesitan un canal para tocar, y el piano roll ya tiene uno:
el canal que muestra. Ese es el *canal seleccionado*. Hacer clic en el
nombre de un canal en el rack, que ya reproducía una vista previa,
ahora también lo selecciona, y el rack lo resalta. Una vez por frame,
`live_input` envía `SetLiveChannel` si la selección cambió desde la
última vez.

## Un motor en un Box

Un cambio pequeño en `app.rs` vino de Clippy:

```rust
enum Audio {
    Running(Box<AudioEngine>),
    Unavailable(String),
}
```

El motor ganó dos campos nuevos en este paso, y el lint
`large_enum_variant` de Clippy notó que una variante se había vuelto
mucho más grande que la otra. Un enum es tan grande como su variante
más grande, así que un mensaje de error del tamaño de un `String`
ocupaba el espacio de un motor entero. `Box` mueve el motor al
montículo, y la variante solo guarda un puntero. La misma razón que el
estado en un box de `MidiOut`, esta vez encontrada por una herramienta.
