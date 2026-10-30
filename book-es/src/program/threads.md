# Dos hilos: la ventana y el audio

Archivos: [`crates/lineup-audio/src/engine.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-audio/src/engine.rs),
[`crates/lineup-audio/src/dsp.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-audio/src/dsp.rs)
y [`crates/lineup-app/src/midi.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/midi.rs)

La ventana y el sonido viven en hilos distintos. El hilo de la ventana
dibuja unas sesenta veces por segundo y responde a clics y teclas; el
hilo de audio rellena el búfer de la tarjeta de sonido cada pocos
milisegundos y no debe llegar tarde nunca. Cómo se hablan los dos es la
decisión más importante del programa, y es donde las reglas de
propiedad de Rust empiezan a ganarse el sueldo.

```text
  UI thread (lineup-app)                         Audio thread (lineup-audio)
  ─────────────────────                        ─────────────────────────
  Project (lineup-core)       commands         AudioThread
   ├─ channels       ───────────────────▶       ├─ channels: Vec<ChannelState>
   ├─ patterns                                  │    └─ Instrument + notes
   └─ playlist                                  ├─ Sequencer
                                                │
  views             ◀── playhead, position ─────┤  (atomic numbers)
                                                │
  collect_garbage() ◀── trash queue ────────────┘  old values to free
```

## Cómo funciona el audio digital

El sonido es una vibración. Una tarjeta de sonido lo reproduce moviendo
un altavoz según una lista de números llamados **muestras**, típicamente
44.100 o 48.000 por segundo (la **frecuencia de muestreo**). Cada muestra
es un valor entre −1.0 y 1.0, la posición del altavoz en ese instante.
Con dos **canales** (estéreo), las muestras van **entrelazadas**:
izquierda, derecha, izquierda, derecha...

La tarjeta de sonido no nos espera. Reproduce desde un búfer pequeño y,
cada pocos milisegundos, pide al programa que lo rellene llamando a una
función que nosotros proporcionamos: el **callback de audio**. Se ejecuta
en un **hilo de audio** separado y de alta prioridad.

## La regla de oro del hilo de audio

Si el callback llega tarde, aunque sea una vez, el búfer se vacía y se
oye un clic o un hueco (un *underrun* o *xrun*). Por eso el código del
hilo de audio nunca debe hacer nada que pueda tardar un tiempo
impredecible:

- no usar **cerrojos** (un `Mutex` retenido por la interfaz podría
  bloquearlo),
- no **reservar ni liberar memoria** (`Vec::push` más allá de su
  capacidad, `String`, `Box::new`, ni soltar el último dueño de
  cualquiera de ellos),
- no hacer **I/O de archivos, red o consola** (`println!` incluido),
- no esperar a otros hilos.

Todo lo demás en este capítulo se deduce de esta regla.

## Enviar comandos con un ring buffer

La interfaz nunca cambia el estado del audio directamente. Envía
mensajes, valores de un enum con casi cuarenta variantes. Algunas de
ellas:

```rust
pub enum Command {
    Play,
    Seek(usize),
    Stop,
    SetTempo(Tempo),
    SetMasterVolume(f32),
    InsertChannel {
        index: usize,
        instrument: Instrument,
        notes: ChannelNotes,
        volume: f32,
        muted: bool,
    },
    RemoveChannel(usize),
    SetNotes {
        pattern: usize,
        channel: usize,
        notes: Vec<NoteEvent>,
    },
    // ...
}
```

Es un **enum** de Rust. A diferencia de los enums de C o Java, cada
variante puede llevar sus propios datos: `Seek` lleva un número de paso,
`SetNotes` toda una lista de notas. Un valor de enum es exactamente una
de sus variantes.

Los mensajes viajan por un **ring buffer** del crate
[`rtrb`](https://docs.rs/rtrb): una cola de tamaño fijo, reservada una
sola vez, con un escritor (el `Producer`) y un lector (el `Consumer`).
Añadir y extraer nunca bloquea ni reserva memoria, así que el hilo de
audio puede leerlo con seguridad.

```rust
let (command_producer, command_consumer) = rtrb::RingBuffer::new(COMMAND_QUEUE_SIZE);
let (trash_producer, trash_consumer) = rtrb::RingBuffer::new(TRASH_QUEUE_SIZE);
let (live_producer, live_consumer) = rtrb::RingBuffer::new(LIVE_QUEUE_SIZE);
let (midi_producer, midi_consumer) = rtrb::RingBuffer::new(MIDI_OUT_QUEUE_SIZE);
```

`RingBuffer::new` devuelve una **tupla** de dos valores, que separamos
con un **patrón** `let (a, b) = ...`. Hay cuatro colas: los comandos de
la interfaz, los valores que la interfaz debe liberar (la basura, más
abajo), las teclas de un teclado MIDI ([MIDI](../sound/midi.md)) y
los mensajes para los canales de salida MIDI. En cada una, un extremo se
queda en un hilo y el otro se mueve al otro.

### La propiedad lo hace seguro

Aquí es donde brilla Rust. `Producer` se puede usar desde un hilo y
`Consumer` desde otro, pero solo hay uno de cada, y las reglas de
**propiedad** de Rust lo garantizan:

- Cada valor tiene exactamente un **dueño**. Cuando el dueño desaparece,
  el valor se libera (se hace drop).
- Asignar o pasar un valor lo **mueve** al nuevo dueño, y el nombre
  antiguo ya no se puede usar.

Una vez que el `consumer` se mueve al callback de audio, el código de la
interfaz físicamente ya no puede tocarlo: el compilador lo rechazaría.
Las condiciones de carrera no son «improbables»; son errores de
compilación.

### Comandos que llevan datos

`Command` no es `Copy`, porque un `Instrument` puede tener un
`Arc<SampleData>` y `SetNotes` tiene un `Vec`. Moverlo por el ring
buffer sigue estando bien: el valor se mueve a la casilla ya reservada
de la cola y sale de nuevo, sin reservar memoria. El `Vec` de dentro se
reservó en el hilo de la interfaz, antes de enviarlo.

### Cuando la cola está llena

La cola guarda 4096 comandos, suficientes para un proyecto grande
entero. Si alguna vez se llena, la interfaz se adelantó al hilo de
audio, que la vacía al principio de cada bloque. `send_wait` no suelta
el comando: cede y vuelve a intentarlo, hasta mil veces.

```rust
pub fn send_wait(&mut self, mut command: Command) -> bool {
    for _ in 0..SEND_WAIT_ROUNDS {
        match self.commands.push(command) {
            Ok(()) => return true,
            Err(rtrb::PushError::Full(returned)) => command = returned,
        }
        // A stream that has reported an error will not empty the
        // queue, so waiting for room would only freeze the window.
        if self.problems.broken.load(Ordering::Relaxed) {
            return false;
        }
        std::thread::yield_now();
    }
    false
}
```

`push` recibe el comando por valor. Cuando la cola está llena lo
devuelve dentro del error, `PushError::Full(returned)`, para que el
bucle pueda intentarlo de nuevo con él: no se copia nada ni se pierde
nada.

## Abrir la tarjeta de sonido

`AudioEngine::start` usa el crate [`cpal`](https://docs.rs/cpal), que
funciona en Linux (ALSA), macOS (CoreAudio) y Windows (WASAPI), así que
el mismo código funciona en los tres.

```rust
pub fn start() -> Result<Self, EngineError> {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .ok_or(EngineError::NoOutputDevice)?;
    let supported = device
        .default_output_config()
        .map_err(EngineError::Config)?;
    ...
}
```

### `Option`, `Result` y el operador `?`

Rust no tiene `null` ni excepciones. En su lugar:

- **`Option<T>`** es `Some(value)` o `None`.
  `default_output_device()` devuelve `Option<Device>` porque puede no
  haber tarjeta de sonido.
- **`Result<T, E>`** es `Ok(value)` o `Err(error)`.
  `default_output_config()` devuelve `Result<SupportedStreamConfig, cpal::Error>`.

Ambos son enums normales de la librería estándar. Al ser tipos, no
puedes olvidar manejar el caso que falta o falla: el compilador no te
dejará usar un `Device` que quizá no tienes.

- `.ok_or(EngineError::NoOutputDevice)` convierte un `Option` en un
  `Result`, usando nuestro error para el caso `None`.
- `.map_err(EngineError::Config)` envuelve el error de cpal en nuestra
  propia variante. Observa que `EngineError::Config` se usa aquí como
  una función: las variantes de enum con datos se pueden llamar como
  constructores.
- **`?`** significa «si esto es un `Err`, devuélvelo de la función ahora
  mismo; si no, dame el valor `Ok`». Sustituye largas cadenas de
  `if error then return`.

### Un tipo de error propio

```rust
#[derive(Debug)]
pub enum EngineError {
    NoOutputDevice,
    Config(cpal::Error),
    UnsupportedFormat(SampleFormat),
    BuildStream(cpal::Error),
    Play(cpal::Error),
}

impl fmt::Display for EngineError { ... }
impl std::error::Error for EngineError {}
```

Un enum enumera todas las formas en que puede fallar el arranque del
motor. Implementar `Display` da a cada una un mensaje legible, y el
`impl std::error::Error` vacío lo marca como un tipo de error en regla
que funciona con el resto del ecosistema de Rust. La aplicación lo
convierte en texto con `err.to_string()` y sigue funcionando sin
sonido: todas las ventanas funcionan, y **Edición > Ajustes… > Audio**
ofrece volver a intentarlo.

## Genéricos: una función, muchos tipos de muestra

Distintas tarjetas de sonido quieren muestras en distintos formatos:
floats de 32 bits, enteros de 16 bits, etc. Generamos `f32` internamente
y convertimos en el último momento. En lugar de escribir el código del
stream seis veces, lo escribimos una vez como **función genérica**:

```rust
fn build_stream<T>(
    device: &cpal::Device,
    config: StreamConfig,
    mut audio_thread: AudioThread,
    problems: Arc<StreamProblems>,
) -> Result<cpal::Stream, EngineError>
where
    T: SizedSample + FromSample<f32>,
{ ... }
```

`T` es un tipo marcador de posición, y la cláusula `where` enumera los
**trait bounds**: `T` puede ser cualquier tipo que sea una muestra de
cpal (`SizedSample`) y que se pueda crear a partir de un `f32`
(`FromSample<f32>`). Dentro, `T::from_sample(x)` hace la conversión.

Luego `start` elige la versión correcta con un `match`:

```rust
let stream = match sample_format {
    SampleFormat::F32 => {
        build_stream::<f32>(&device, config, audio_thread, Arc::clone(&problems))
    }
    SampleFormat::I16 => {
        build_stream::<i16>(&device, config, audio_thread, Arc::clone(&problems))
    }
    ...
    other => return Err(EngineError::UnsupportedFormat(other)),
}?;
```

- `match` compara un valor con **patrones** y ejecuta la primera rama
  que encaja. El compilador comprueba que todos los valores posibles
  están cubiertos. `other` es un patrón comodín que además da nombre al
  valor.
- `build_stream::<f32>` (la sintaxis «turbofish» `::<>`) elige
  `T = f32`. El compilador genera una copia separada y totalmente
  optimizada de la función para cada tipo usado. Esto se llama
  **monomorfización**, y significa que los genéricos no cuestan nada en
  tiempo de ejecución.
- `audio_thread` se mueve a la rama que se ejecute. El compilador sabe
  que solo una rama se ejecuta, así que esto está permitido.

## El callback de audio es un closure

Dentro de `build_stream`:

```rust
let data_callback = move |output: &mut [T], _: &cpal::OutputCallbackInfo| {
    audio_thread.process(output);
};

let error_callback = move |err: cpal::Error| {
    problems.broken.store(true, Ordering::Relaxed);
    if let Ok(mut slot) = problems.error.lock() {
        *slot = Some(err.to_string());
    }
    eprintln!("audio stream error: {err}");
};

device
    .build_output_stream(config, data_callback, error_callback, None)
    .map_err(EngineError::BuildStream)
```

`|args| cuerpo` es un **closure**: una función anónima que puede usar
variables del lugar donde se escribió. La palabra clave `move` hace que
el closure tome la **propiedad** de `audio_thread`, y con ella del
consumer de comandos y de todo lo demás con lo que trabaja el hilo de
audio. Eso es necesario porque el closure se ejecutará en otro hilo,
mucho después de que `build_stream` haya devuelto, así que no puede
limitarse a tomar prestadas variables locales.

La firma de cpal exige que el callback sea
`FnMut(&mut [T], &OutputCallbackInfo) + Send + 'static`:

| Límite | Significado |
|--------|-------------|
| `FnMut` | un closure que puede cambiar los valores que posee (los instrumentos avanzan su estado) |
| `Send` | seguro de mover a otro hilo |
| `'static` | posee todo lo que usa, sin tomar prestado nada que pueda desaparecer |

Si intentáramos compartir algo inseguro, como una referencia simple a
los datos de la interfaz, el programa no compilaría. Así convierte Rust
la regla de oro del audio en comprobaciones que el compilador hace por
ti.

`&mut [T]` es un **slice mutable**: una vista prestada del búfer de la
tarjeta de sonido, que rellenamos pero no poseemos.

El callback de error es el único lugar que puede tomar un cerrojo e
imprimir: cpal lo llama fuera del callback de audio, cuando algo falla
mientras el stream funciona, como una interfaz desenchufada. Guarda el
problema más reciente para que la ventana lo muestre.

## `AudioThread::process`

El callback de audio entrega cada búfer a un solo método. Su esquema:

```rust
pub(crate) fn process<T>(&mut self, output: &mut [T])
where
    T: SizedSample + FromSample<f32>,
{
    self.flush_trash();
    self.flush_midi_backlog();
    while let Ok(command) = self.commands.pop() {
        self.apply(command);
    }
    while let Some(message) = self.live_input.as_mut().and_then(|keys| keys.pop().ok()) {
        self.play_live(message);
    }

    for frame in output.chunks_mut(self.output_channels) {
        if self.playing
            && let Some(step) = self.sequencer.tick()
        {
            self.playhead.store(step, Ordering::Relaxed);
            self.start_notes(step);
        }
        // ... mix the channels, the reverb and the metronome ...
    }
    self.send_midi();
}
```

- `while let Ok(command) = ...` sigue en bucle mientras el patrón
  encaja, de modo que vacía todos los comandos en espera y se detiene
  cuando la cola está vacía.
- `as_mut` convierte `&mut Option<Consumer>` en
  `Option<&mut Consumer>`, y `and_then` le pide una tecla solo si hay un
  consumer.
- `chunks_mut(channels)` divide el búfer en **frames**, una muestra por
  canal de salida.
- `if a && let Some(x) = b { ... }` es una **let chain**: una condición
  y un patrón combinados en un solo `if`.

El estado del hilo de audio vive en una struct, `AudioThread`, en lugar
de en variables sueltas dentro del closure. Así las pruebas pueden
ejecutarlo con un `Vec` simple en lugar de una tarjeta de sonido, y el
[renderizador](../sound/export.md) puede tocar una canción a un
archivo con el mismo código.

## Mezclar un frame

En cada frame, el instrumento de cada canal hace una muestra. La muestra
pasa por los efectos y el volumen del canal, y luego va a los lados
izquierdo y derecho según su balance, y a la reverberación tanto como
dice su envío ([El mezclador](../sound/mixer.md)):

```rust
for channel in &mut self.channels {
    let volume = channel.volume.next_value();
    let sample = channel.effects.process(channel.instrument.next_sample()) * volume;
    ...
    dry[0] += kept * gains[0];
    dry[1] += kept * gains[1];
    send += sample * amount;
}
...
// `tanh` gently rounds off peaks above 1.0 instead of clipping them hard.
let stereo = stereo.map(f32::tanh);
```

- `for channel in &mut self.channels` itera sobre referencias mutables,
  para que cada instrumento pueda avanzar su propio estado.
- Mezclar es simplemente sumar: los sonidos en el aire se suman, y las
  muestras también.
- Varios tambores fuertes a la vez pueden sumar más de 1.0, y una
  tarjeta de sonido lo recortaría con dureza. `tanh` (la tangente
  hiperbólica) deja pasar los valores pequeños casi sin cambios y dobla
  los grandes suavemente hacia ±1.0, un **soft clipper** (recortador
  suave).
- `volume.next_value()` es un valor
  [`Smoothed`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-audio/src/dsp.rs).
  Un volumen que salta hace saltar el altavoz, y eso se oye como un
  clic, así que en cambio se desliza hacia su objetivo: en cada muestra
  recorre una pequeña fracción de la distancia que queda. Es un
  *suavizador de un polo*, que se asienta en pocos milisegundos,
  demasiado rápido para oírse como un fundido pero suficiente para
  quitar el clic.

## Espacio reservado de antemano

```rust
channels: Vec::with_capacity(MAX_CHANNELS),
```

Un `Vec` tiene una **longitud** (cuántos elementos tiene) y una
**capacidad** (cuántos caben en la memoria que tiene). `push` solo
reserva memoria cuando la longitud superaría la capacidad. Reservar
espacio para `MAX_CHANNELS` al arrancar, en el hilo de la interfaz,
significa que `push` en el hilo de audio nunca reserva memoria. La
interfaz se niega a añadir más canales que esos, así que el límite no se
alcanza nunca. La lista de notas que esperan para empezar tarde, las
longitudes de los patrones y el backlog de la basura se reservan del
mismo modo.

`Vec::remove(index)` desplaza hacia abajo los elementos siguientes, lo
que tampoco reserva memoria. Así es como `RemoveChannel` mantiene los
números de canal del hilo de audio iguales a los de la lista de la
interfaz.

## Liberar memoria en otro sitio: la cola de basura

Cuando se elimina un canal o se sustituyen sus notas, el valor viejo
puede tener el último `Arc` de una muestra, o un `Vec` de notas.
Liberarlo soltaría esa memoria, y liberar puede tomar un cerrojo dentro
del asignador de memoria. Así que el hilo de audio no lo libera. Lo
devuelve:

```rust
fn set_instrument(&mut self, channel: usize, instrument: Instrument) {
    match self.channels.get_mut(channel) {
        Some(state) => {
            let mut old = std::mem::replace(&mut state.instrument, instrument);
            silence(&mut old, &mut self.midi_out, &mut self.midi_backlog);
            ...
            self.throw_away(Garbage::Instrument(old));
        }
        None => self.throw_away(Garbage::Instrument(instrument)),
    }
}
```

`std::mem::replace(&mut place, new)` pone `new` en `place` y devuelve lo
que había. Es la forma de sacar un valor de un campo de un struct del
que solo tienes un `&mut`, ya que Rust nunca deja un campo vacío.
(`silence` envía los note-offs que el instrumento viejo todavía le debe
a un sintetizador de salida MIDI.)

`throw_away` empuja el valor a un segundo ring buffer, que va en la
dirección contraria. `Garbage` es un enum con una variante para cada
clase de cosa que devuelve el hilo de audio: instrumentos, listas de
notas, canales, canciones, listas de tempo, frases, líneas de retardo.
El hilo de la interfaz vacía la cola una vez por frame:

```rust
pub fn collect_garbage(&mut self) {
    while let Ok(garbage) = self.trash.pop() {
        // Dropping a value frees its memory. Here, on the UI thread,
        // that is allowed to take as long as it needs.
        match garbage {
            Garbage::Instrument(instrument) => drop(instrument),
            Garbage::Notes(notes) => drop(notes),
            ...
        }
    }
}
```

`drop(value)` es una función normal del preludio que toma la propiedad y
deja que el valor salga de ámbito. Escribirla explícitamente documenta
la intención: aquí es donde se libera la memoria.

Una ráfaga de cambios entre dos frames podría llenar la cola de basura.
Entonces `throw_away` guarda el valor en un backlog reservado de
antemano, y `flush_trash` lo pasa a la cola al principio del bloque
siguiente. Solo si también se llena el backlog, lo que significa que la
interfaz lleva mucho tiempo sin recoger, el valor se libera en el hilo de
audio, como último recurso.

## El cursor de reproducción: un entero atómico

La interfaz quiere resaltar el paso que se está reproduciendo. El hilo
de audio lo sabe. Compartirlo con un `Mutex` significaría que el hilo de
audio podría esperar por el cerrojo, así que usamos un **atómico**:

```rust
let playhead = Arc::new(AtomicUsize::new(NO_STEP));
// audio thread:
self.playhead.store(step, Ordering::Relaxed);
// UI thread:
match self.playhead.load(Ordering::Relaxed) { NO_STEP => None, step => Some(step) }
```

Un `AtomicUsize` es un número que el procesador puede leer y escribir en
una sola operación indivisible, de modo que dos hilos pueden usarlo sin
cerrojo y sin ver nunca un valor a medio escribir. Ambos hilos tienen un
`Arc` al mismo (`Arc::clone(&playhead)` crea el segundo puntero).

`Ordering::Relaxed` dice que solo nos importa este valor, no el orden de
otras escrituras de memoria a su alrededor. Es el orden más barato y
basta para mostrarlo. Los órdenes más estrictos (`Acquire`, `Release`,
`SeqCst`) son para cuando un valor protege el acceso a otros.

Rust no tiene un paso «nulo», así que `usize::MAX` representa «parado»,
y `playhead()` lo convierte enseguida en un `Option<usize>` honesto para
que ningún otro código vea nunca el número mágico.

La posición con fracción de paso, que necesitan la grabación y las
vistas, es un `f64`, y no hay float atómico. Así que se guarda como los
bits del `f64` en un `AtomicU64`: `position.to_bits()` en el hilo de
audio, `f64::from_bits(bits)` del otro lado. Dos números que siempre
deben leerse juntos, como el patrón que suena sobre la canción y su
paso, comparten un `AtomicU64`: uno en los 32 bits altos, el otro en
los bajos.

## Mantener vivo el stream: `Drop`

```rust
pub struct AudioEngine {
    // The stream is never read, but it must be kept: dropping it stops audio.
    _stream: cpal::Stream,
    commands: rtrb::Producer<Command>,
    trash: rtrb::Consumer<Garbage>,
    playhead: Arc<AtomicUsize>,
    position: Arc<AtomicU64>,
    ...
}
```

El `cpal::Stream` no se vuelve a usar después de arrancar, pero hay que
conservarlo: cuando un valor se libera, Rust ejecuta su código de
limpieza (su implementación de `Drop`), y liberar un `Stream` cierra la
tarjeta de sonido. Guardarlo en el motor ata la vida del sonido a la del
motor. El guion bajo inicial le dice al compilador «sé que este campo no
se lee nunca».

Este patrón, en el que poseer un valor significa poseer un recurso y
liberarlo lo suelta, se llama **RAII**. Rust lo usa en todas partes: los
archivos se cierran, los cerrojos se desbloquean y la memoria se libera
sola cuando su dueño desaparece.

## Cuando la tarjeta de sonido desaparece

Una interfaz desenchufada, un cambio de dispositivo predeterminado o un
servidor de sonido que se reinició hacen que el stream informe de un
error. **Edición > Ajustes… > Audio** ofrece entonces **Reintentar** (y
**Reiniciar audio** mientras el stream funciona), que vuelve a abrir
el dispositivo de salida predeterminado del sistema. El proyecto se
queda: se envía entero al hilo de audio nuevo, los puertos MIDI se
vuelven a conectar al motor nuevo, y una canción que estaba sonando
vuelve a la misma posición y sigue.

El motor viejo se suelta antes de que arranque el nuevo, porque un
dispositivo que se vuelve a abrir no puede seguir retenido por el
stream muerto.

Dos cosas necesitan cuidado cuando se va un hilo de audio:

**Notas que suenan en un sintetizador externo.** Una nota enviada por
salida MIDI recibe su note-off del hilo de audio que la tocó; cuando ese
hilo ya no está, se van con él los note-offs que todavía debía. Los
mensajes salen por un hilo pequeño, `midi-out`, que vacía la cola del
hilo de audio cada milisegundo. Cuando su cola queda abandonada, su
bucle termina, y antes de acabar envía **All Notes Off** (controlador
123) en los dieciséis canales:

```rust
let sender = move || {
    while !messages.is_abandoned() {
        // ... send what the audio thread queued ...
    }
    // The engine is gone, and with it the note offs of notes still
    // sounding on the synth.
    if let Some(connection) = lock(&output).as_mut() {
        for channel in 0..16 {
            let _ = connection.send(&all_notes_off(channel));
        }
    }
};
```

El hilo de envío del motor nuevo no debe arrancar antes de que el viejo
haya enviado su All Notes Off, o ese mensaje podría cortar las primeras
notas del motor nuevo. Así que `reconnect_audio` espera al hilo viejo,
consultando `JoinHandle::is_finished` como mucho 100 ms (en la práctica
cerca de un milisegundo, porque el motor viejo ya no está). No llama a
`join()`, que podría esperar para siempre a un puerto MIDI colgado: la
ventana nunca debe congelarse por un puerto.

**Mensajes que deben llegar.** Un note-off perdido deja una nota sonando
para siempre en un sintetizador externo, así que la salida MIDI nunca
pierde ninguno. Los mensajes que encuentran la cola de salida llena
esperan en un backlog reservado y salen al principio del bloque
siguiente. Quitar un canal o sustituir un instrumento envía sus
note-offs por el mismo camino, y desconectar un puerto envía All Notes
Off en los dieciséis canales antes de que se cierre la conexión.

La [traza de reproducción](../sound/trace.md) es el caso contrario:
graba cada nota para un informe, no para el sonido, así que si su cola
se llena, el hilo de audio cuenta los eventos perdidos y sigue. Esperar
ahí pondría la traza antes que la música. Cuando el audio se reinicia,
la traza sigue en el mismo archivo: se escribe lo que dejó el hilo
viejo, y se envía una cola nueva al hilo nuevo.

## Probar sin tarjeta de sonido

Las pruebas al final de `engine.rs` construyen un `AudioThread` con ring
buffers, empujan comandos, renderizan en un `Vec<f32>` y comprueban el
resultado: un golpe en el frame 0 y otro 6000 frames después a 120 BPM,
silencio al silenciar, ediciones que surten efecto durante la
reproducción e instrumentos viejos que llegan a la basura. Una pequeña
struct `Harness` guarda las piezas para que cada prueba sea corta.
