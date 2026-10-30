# Exportar: WAV, MIDI y video

Archivos: [`crates/lineup-audio/src/render.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-audio/src/render.rs),
[`crates/lineup-app/src/app/export.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/export.rs),
[`crates/lineup-video/`](https://github.com/naelolaiz/ferrous-lineup/tree/main/crates/lineup-video),
[`crates/lineup-app/src/video.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/video.rs)
y [`crates/lineup-app/src/app/video.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/video.rs)

El menú **Archivo** saca una canción de la aplicación de varias formas:

- **Exportar WAV…** escribe el sonido, tal como lo oyes cuando la
  canción suena.
- **Exportar video…** escribe un archivo MP4 con el sonido y las vistas
  que elijas siguiendo la música: la hoja de acordes, la partitura guía,
  la lista de reproducción, el piano roll, las teclas de armonía.
- **Exportar canción como MIDI…** y **Exportar patrón como MIDI…**
  escriben las notas, para otro programa de música. Cómo se hacen los
  bytes de un archivo MIDI está en
  [MIDI](midi.md#escribir-un-archivo-midi).
- **Exportar hoja de acordes…** escribe los acordes como texto, y la
  partitura guía se imprime como PDF; ambas cosas están en los capítulos
  sobre la [hoja de acordes](../band/chord-sheet.md) y la
  [partitura guía](../band/lead-sheet.md).

Cómo usarlas está en el capítulo
[Exportar](https://naelolaiz.github.io/ferrous-lineup/es/manual/exporting.html)
del manual. Este capítulo trata de las dos exportaciones que tienen que
*tocar* la canción: el archivo WAV y el video. Tres ideas las sostienen:

1. **El mismo código, más rápido que el tiempo real.** El hilo de audio
   es una función que llena un búfer. No sabe que quien le pide es una
   tarjeta de sonido. Puede pedirle un bucle, tan rápido como dé el
   ordenador, y escribir lo que sale en un archivo. No hay un segundo
   motor que mantener de acuerdo con el primero, así que una exportación
   suena exactamente como la reproducción.
2. **Una instantánea, en un hilo propio.** La exportación recibe su
   propia copia de la canción y sus propios instrumentos, y corre al
   lado de la ventana. Puedes seguir tocando y editando; la exportación
   sigue con la canción tal como estaba cuando la elegiste.
3. **Un archivo terminado o ninguno.** Toda exportación escribe en un
   archivo temporal y lo renombra al final, así que una exportación que
   falla o se detiene nunca deja medio archivo donde debería haber uno
   terminado.

## El hilo de audio, sin tarjeta de sonido

Visto desde fuera, el hilo de audio es un método:

```rust
pub(crate) fn process<T>(&mut self, output: &mut [T])
where
    T: SizedSample + FromSample<f32>,
```

Llena `output` con los siguientes frames, en el tipo de muestra que
[pida la tarjeta de sonido](../program/threads.md#genéricos-una-función-muchos-tipos-de-muestra).
Con una tarjeta de sonido, cpal lo llama unos cientos de frames por vez,
tan rápido como se reproducen. Nada en él espera al reloj: simplemente
hace los siguientes frames. Así que un bucle que lo llama con un simple
`Vec<f32>` obtiene la canción tan rápido como el procesador puede
calcularla. Los tests del propio motor siempre han funcionado así: nunca
abren una tarjeta de sonido.

`lineup-audio` envuelve esto en un **`Renderer`**:

```rust
pub struct Renderer {
    audio: AudioThread,
    sample_rate: u32,
    channels: usize,
    /// The audio thread is built with a command queue, but the renderer
    /// hands it commands directly. Keeping this end of the queue alive
    /// keeps the queue valid.
    _commands: rtrb::Producer<Command>,
    trash: rtrb::Consumer<Garbage>,
    /// Where playback is, which the audio thread stores after every block.
    position: Arc<AtomicU64>,
    /// The trace being written, and its reader, once [`Renderer::trace`]
    /// has started one.
    trace: Option<(TraceReader, Trace)>,
}
```

`AudioThread` es `pub(crate)`: los otros crates no pueden construir uno,
llamar a su `process` ni meterse dentro. Reciben el `Renderer`, cuyos
métodos respetan las reglas del hilo de audio. Mantener la exportación
*dentro* de `lineup-audio` es lo que permite que el motor siga siendo
privado.

`AudioThread::new` quiere el extremo receptor de una cola de comandos,
porque en el hilo de audio real así llegan los comandos. El renderer no
tiene otro hilo del que recibir. Crea una cola con espacio para un
comando:

```rust
let (commands, command_consumer) = rtrb::RingBuffer::new(1);
```

y nunca la usa. Los comandos entran directamente:

```rust
/// Applies a command at once: there is no other thread to hand it to.
/// Volume changes take effect at once too, without smoothing: the
/// commands come before the first sample is made.
pub fn send(&mut self, command: Command) {
    self.audio.apply(command);
    self.audio.settle_volumes();
    self.empty_trash();
    self.read_trace();
}
```

El campo que guarda el otro extremo de la cola es `_commands`. Un guion
bajo al principio le dice al compilador (y al lector) que el campo no se
lee nunca a propósito, así que no avisa «field `commands` is never
read». El campo igualmente guarda el productor mientras vive el
renderer, así que la cola del hilo de audio tiene sus dos extremos, como
en la aplicación.

Los otros métodos son igual de cortos:

```rust
/// Fills `output` with the next frames, one sample per channel each.
pub fn render(&mut self, output: &mut [f32]) {
    self.audio.process(output);
    self.empty_trash();
    self.read_trace();
}
```

`read_trace` pertenece a la [traza de reproducción](trace.md),
que usa el mismo renderer para anotar cada nota en lugar del sonido.

### Dos detalles que solo aparecen fuera de línea

**El suavizado.** Los cambios de volumen se deslizan durante unos
milisegundos (`Smoothed`), porque un salto brusco produce un clic
audible. Con una tarjeta de sonido, ese deslizamiento ocurre mientras
suena la música. En una exportación, fijar el volumen principal justo
antes de la primera muestra haría que los primeros milisegundos del
archivo se deslizaran del volumen por defecto al tuyo. Por eso `send`
llama a `settle_volumes`, que pone cada valor suavizado en su objetivo
de una vez:

```rust
pub(crate) fn settle_volumes(&mut self) {
    self.master_volume.settle();
    self.reverb.settle();
    for channel in &mut self.channels {
        channel.volume.settle();
        channel.pan.settle();
        channel.reverb_send.settle();
    }
}
```

```rust
/// Jumps straight to the target, with no smoothing.
pub fn settle(&mut self) {
    self.current = self.target;
}
```

El primer test del renderer, `renders_like_the_audio_thread`,
detectaría que falta ese ajuste: fija el volumen principal y comprueba
que el primerísimo frame de un clic tiene el nivel completo.

**La basura.** El hilo de audio real nunca libera memoria. Lo que tira
(una lista vieja de notas, un canal eliminado) va a la
[cola de basura](../program/threads.md#liberar-memoria-en-otro-sitio-la-cola-de-basura),
y el hilo de la interfaz lo libera una vez por frame de la ventana. El
renderer no es un hilo de tiempo real, así que puede liberar las cosas
en el acto, y vacía la cola después de cada comando y de cada bloque:

```rust
/// Frees what the audio thread threw away. There is no real-time
/// thread here, so it can be done straight away.
fn empty_trash(&mut self) {
    while self.trash.pop().is_ok() {}
}
```

`pop` devuelve cada valor, y soltarlo lo libera. El cuerpo del bucle
está vacío porque no hay nada más que hacer con ellos.

## Poner la canción en el renderer

Una exportación construye su renderer con los mismos comandos que la
aplicación envía al arrancar y después de abrir un archivo:

```rust
/// The whole song, ready for `renderer` to play from the start.
fn song_render_with(&mut self, mut renderer: Renderer) -> SongRender {
    let sample_rate = renderer.sample_rate();
    for command in self.project_commands(sample_rate, 1.0) {
        renderer.send(command);
    }

    // The song loops when it reaches its end, so silent steps are
    // added after it: enough for the tail, and one more, so the
    // export ends before the song would start again.
    // The whole song, every repeat included, even when song mode only
    // loops part of it.
    let song = self.project.arrangement();
    let song_length = song.length;
    // The silent steps play at the tempo the song ends at: the
    // audio thread plays steps past the end of the tempo list at
    // its last tempo.
    let tempos = self.project.played_tempos(&song);
    let end_tempo = tempos.last().copied().unwrap_or(self.project.tempo);
    let step_frames = frames_per_step(end_tempo, sample_rate);
    let reverb = self.project.mixer.reverb;
    let tail_seconds = if reverb.enabled {
        TAIL_SECONDS + f64::from(reverb.decay + reverb.pre_delay_ms / 1000.0)
    } else {
        TAIL_SECONDS
    };
    let tail_frames = tail_seconds * f64::from(sample_rate);
    let silent_steps = (tail_frames / step_frames).ceil() as usize + 1;
    let song_frames = self.project.played_seconds(&song) * f64::from(sample_rate);
    renderer.send(Command::SetSong {
        plays: song.plays,
        length: song_length + silent_steps,
    });
    renderer.send(Command::SetSongTempos(tempos));
    renderer.send(Command::SetMode(PlayMode::Song));
    renderer.send(Command::Play);
    SongRender {
        renderer,
        frames: (song_frames + tail_frames) as usize,
    }
}
```

`project_commands` enumera el tempo y el tiempo suelto, el volumen
principal y la reverberación, cada patrón (`InsertPattern`), cada canal
(`InsertChannel`, con su instrumento y sus notas en todos los patrones,
y luego su balance, su envío a la reverberación y sus efectos) y la
canción. El segundo argumento es la velocidad de práctica. La
reproducción puede hacer más lenta la canción para practicar, pero una
exportación va siempre a la velocidad propia de la canción, así que pasa
`1.0`. `send_whole_project`, que habla con el hilo de audio real, llama
a la misma función con la velocidad de práctica, así que ambos se
construyen igual.

Lo que toca la exportación se deduce de esos comandos:

- **Todo lo que oyes**: la banda, los volúmenes del mezclador, el
  balance, la reverberación y los efectos. Los canales y pistas
  silenciados siguen callados, como cuando pulsas Reproducir. Con la
  [SoundFont](soundfonts.md) activada, suenan sus
  instrumentos.
- **La canción entera**, cada repetición y cada vuelta de un estribillo,
  aunque una región de bucle haga que el modo canción toque solo una
  parte: `arrangement()` es la canción entera, mientras que Reproducir
  usa `playback_arrangement()`.
- **Sin metrónomo.** `project_commands` nunca envía
  `Command::SetMetronome`, y un hilo de audio nuevo empieza con él
  apagado.

Los instrumentos se hacen para la frecuencia de muestreo de la
exportación, 44.100 frames por segundo, sea cual sea la de tu tarjeta de
sonido. La batería incorporada se sintetiza de nuevo a esa frecuencia,
el Lineup Osc se hace para ella, y los samplers toman sus sonidos de la
[`SampleCache`](instruments.md), así que no se vuelve a leer
ningún archivo.

El renderer tiene sus propios instrumentos y su propia copia de cada
nota. Es una instantánea, como las copias del
[historial de deshacer](../program/undo.md): la exportación sigue con
la canción tal como estaba, hagas lo que hagas en la ventana mientras
tanto.

### ¿Cuánto dura una canción?

La duración de la canción se conoce en pasos; la del archivo, en frames.
`played_seconds` suma cada paso al tempo vigente en él, así que una
canción con cambios de tempo, ritardandos y calderones también obtiene
la duración correcta. Después viene una **cola** de dos segundos, para
dejar sonar las últimas notas. Con la reverberación activada, la cola
espera además su decaimiento y su pre-delay, para que la sala se apague
en lugar de cortarse.

La canción inicial tiene 128 pasos a 130 BPM. Hay 4 pasos por pulso, así
que un paso dura 60 / 130 / 4 = 0,115 segundos, 5.088,46 frames a 44.100
(un paso no es un número entero de frames; el
[secuenciador](sequencer.md) conserva la fracción). Los 128
pasos ocupan 651.323 frames; con una cola de dos segundos, 88.200, el
archivo tiene 739.523 frames, 16,77 segundos.

### Pasos en silencio después del final

Hay una trampa. En modo canción, el hilo de audio *repite en bucle*:
cuando el cursor llega al final de la canción, vuelve a empezar desde el
compás 1. La cola terminaría con el primer acorde de la canción
empezando otra vez. La solución es decirle a este renderer, y solo a
este, que la canción es más larga de lo que es, con algunos pasos en
silencio. Para la canción inicial son 88.200 / 5.088,46 = 17,33,
redondeado hacia arriba a 18, más uno: 19 pasos en silencio.

¿Por qué el paso de más? Un paso no es un número entero de frames, así
que el secuenciador empieza algunos pasos un frame antes o después de la
fracción exacta. El paso de más asegura que el final del archivo esté
siempre a un paso entero del bucle, se redondee como se redondee.

Los pasos en silencio suenan al tempo con el que termina la canción. El
hilo de audio toca los pasos que quedan más allá del final de la lista
de tempos a su último tempo, así que una canción que termina con un
ritardando recibe una cola medida en sus lentos pasos finales, no en los
más rápidos del principio.

## Escribir un archivo WAV

Un archivo WAV es una cabecera corta (aquí, 44 bytes) que dice cómo se
guarda el sonido, seguida de las muestras. La exportación escribe el
tipo más común:

- **44.100 frames por segundo**, la frecuencia de un CD, que todo
  programa entiende,
- **2 canales** (estéreo), guardados **intercalados**: izquierdo,
  derecho, izquierdo, derecho..., exactamente como los búferes que cpal
  le da al hilo de audio,
- **16 bits por muestra**: cada muestra es un `i16`, un número entero de
  -32.768 a 32.767.

Eso da 44.100 × 2 × 2 = 176.400 bytes por segundo de sonido, unos
10,6 MB por minuto. El archivo de la canción inicial ocupa 44 +
739.523 × 4 = 2.958.136 bytes.

Escribir la cabecera y las muestras a mano sería fácil, pero el crate
`hound` ya lo hace, y el sampler lo usa de todos modos para *leer*
archivos WAV:

```rust
pub fn export_wav_cancellable(
    renderer: &mut Renderer,
    path: &Path,
    frames: usize,
    mut progress: impl FnMut(usize),
    cancel: &AtomicBool,
) -> Result<WavExport, hound::Error> {
    let spec = hound::WavSpec {
        channels: renderer.channels() as u16,
        sample_rate: renderer.sample_rate(),
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut temporary = PathBuf::from(path);
    temporary.as_mut_os_string().push(".tmp");

    let mut writer = hound::WavWriter::create(&temporary, spec)?;
    let mut block = vec![0.0_f32; BLOCK_FRAMES * renderer.channels()];
    let mut done = 0;
    while done < frames {
        if cancel.load(Ordering::Relaxed) {
            drop(writer);
            let _ = fs::remove_file(&temporary);
            return Ok(WavExport::Cancelled);
        }
        let count = (frames - done).min(BLOCK_FRAMES);
        let samples = &mut block[..count * renderer.channels()];
        renderer.render(samples);
        for &sample in samples.iter() {
            writer.write_sample(to_i16(sample))?;
        }
        done += count;
        progress(done);
    }
    writer.finalize()?;
    fs::rename(&temporary, path)?;
    Ok(WavExport::Written)
}
```

El sonido se hace en bloques de 4.096 frames (`BLOCK_FRAMES`), en un
único búfer que se reutiliza para cada bloque. El último bloque suele
ser más corto, así que `samples` es un slice con solo la parte
necesaria. Después de cada bloque se llama a `progress` con el número de
frames hechos. Su tipo es `impl FnMut(usize)`: cualquier closure que
reciba un número y pueda cambiar lo que capturó. El closure de la
aplicación guarda el número donde la barra de estado puede leerlo.

Cada `?` aquí devuelve un `hound::Error`. `fs::rename`, en cambio,
devuelve un `std::io::Error`, y `?` funciona igual, porque `hound::Error`
implementa `From<std::io::Error>`: el operador `?` convierte el error a
la salida.

`export_wav` es un envoltorio fino para quien no puede cancelar. Pasa un
indicador que nunca se activa y devuelve un simple
`Result<(), hound::Error>`; lo usan los tests. La aplicación usa la
versión que se puede cancelar.

### Escribir en un archivo temporal y después renombrar

Las muestras van primero a `song.wav.tmp`, y solo cuando todo está
escrito se renombra a `song.wav`. Así se
[guarda](../program/saving.md#guardar-sin-perder-el-archivo-viejo)
también el archivo del proyecto. Exportar encima de un `song.wav`
anterior deja el archivo viejo entero hasta que el nuevo está completo:
si el disco se llena o algo falla a mitad de camino, te queda tu
exportación anterior, no la mitad de una nueva.

`as_mut_os_string().push(".tmp")` añade al nombre. `set_extension("tmp")`
*reemplazaría* `.wav`, dando `song.tmp`, que podría ser el nombre del
archivo de otra persona.

### De `f32` a `i16`

```rust
/// Turns a sample between -1.0 and 1.0 into a 16-bit integer sample.
fn to_i16(sample: f32) -> i16 {
    (sample.clamp(-1.0, 1.0) * f32::from(i16::MAX)).round() as i16
}
```

`clamp` mantiene la muestra entre -1,0 y 1,0; después se escala a
-32.767..32.767 y se redondea al entero más cercano. Una conversión con
`as` de un número de coma flotante a un entero ya se detiene en los
límites del entero en Rust (*satura*: `3e9 as i16` es 32.767), así que
el `clamp` no hace falta por seguridad, pero dice lo que se quiere.
`f32::from(i16::MAX)` es una conversión que nunca puede perder nada, así
que es un `From`, no un `as`.

El hilo de audio ya mantiene la salida principal dentro de -1,0..1,0 con
su [limitador suave](mixer.md) (`tanh`), así que en la
práctica el `clamp` nunca recorta nada.

## Un hilo propio

Renderizar lleva un rato. Con los sonidos incorporados, una compilación
de release en un ordenador de cuatro núcleos toca el Boléro (catorce
minutos, para una orquesta completa) a través de un renderer en unos
veinte segundos, unas cuarenta veces más rápido de lo que suena; la
canción inicial tarda una fracción de segundo. Una compilación de
depuración es muchas veces más lenta. Mientras tanto la ventana no debe
congelarse, así que la exportación corre en un **hilo** propio:

```rust
let done = Arc::new(AtomicUsize::new(0));
let cancel = Arc::new(AtomicBool::new(false));
// The thread gets its own `Arc`s pointing to the same counter and
// flag, and its own copy of the path. `move` gives the closure
// ownership of them, and of the renderer: from now on they belong
// to the thread.
let worker = {
    let done = Arc::clone(&done);
    let cancel = Arc::clone(&cancel);
    let path = path.clone();
    thread::spawn(move || {
        match export_wav_cancellable(
            &mut renderer,
            &path,
            frames,
            |frames_done| done.store(frames_done, Ordering::Relaxed),
            &cancel,
        ) {
            Ok(WavExport::Written) => Ok(()),
            Ok(WavExport::Cancelled) => Err(CANCELLED.to_string()),
            Err(err) => Err(err.to_string()),
        }
    })
};
self.export = Some(Export::new(path, frames, done, Some(cancel), worker));
```

`std::thread::spawn` arranca un hilo nuevo que ejecuta un closure, y le
pide dos cosas, las mismas dos que cpal le pide al
[callback de audio](../program/threads.md#el-callback-de-audio-es-un-closure):

- **`'static`**: el closure no puede tomar prestado nada del código que
  lo arrancó. El hilo nuevo puede sobrevivir a la función que lo creó
  (sin duda sobrevive a este frame de la interfaz), así que una
  referencia prestada podría apuntar a algo que ya no existe. Por eso el
  closure lleva **`move`**: toma *la propiedad* de todo lo que usa (el
  renderer, la ruta, el contador, el indicador) en lugar de tomarlo
  prestado.
- **`Send`**: todo lo que se mueve a él tiene que poder pasarse a otro
  hilo sin peligro. El renderer cumple, porque todo lo que contiene
  cumple: los instrumentos, los `Arc` con las muestras, los extremos de
  los ring buffers. Si contuviera un `Rc` (un contador de referencias
  que no es seguro entre hilos), esta línea no compilaría.

El bloque `{ let done = Arc::clone(&done); ... }` es un truco habitual.
Tanto la interfaz como el hilo necesitan el contador, así que cada uno
recibe su propio `Arc` que apunta a él. Dentro del bloque, el `done`
nuevo *oculta* (shadowing) al de fuera, y el closure `move` se lleva el
de dentro, dejando el `done` de fuera para la interfaz. Lo mismo pasa
con `cancel` y con `path`.

### El progreso: un contador atómico

El hilo escribe cuántos frames están hechos en un **`AtomicUsize`**, y
la barra de estado lo lee cuando dibuja la barra de progreso, como la
ventana lee el
[cursor de reproducción](../program/threads.md#el-cursor-de-reproducción-un-entero-atómico).
`Ordering::Relaxed` basta: ningún otro dato depende del número, y una
barra un frame atrasada no importa. El archivo solo se da por bueno
cuando el hilo ha terminado, y `join` (abajo) da la garantía completa.

### Saber cuándo ha terminado

`spawn` devuelve un **`JoinHandle`**. La aplicación lo guarda, junto con
el contador, en un `Export`:

```rust
pub(super) struct Export {
    path: PathBuf,
    /// How much there is to do: frames of sound, or pictures.
    frames: usize,
    /// How many are done. The export thread writes it and the UI
    /// thread reads it; an atomic needs no lock for that.
    done: Arc<AtomicUsize>,
    /// Set to ask the thread to stop, for exports that can be cancelled.
    cancel: Option<Arc<AtomicBool>>,
    /// The thread doing the work. Joining it gives back what its closure
    /// returned: here, whether the file was written.
    worker: JoinHandle<Result<(), String>>,
}
```

La exportación de video usa la misma estructura, contando imágenes en
lugar de frames. `JoinHandle<Result<(), String>>` dice qué devuelve el
closure del hilo: `Ok(())` si se escribió el archivo, o un mensaje. El
error se convierte en `String` en el hilo de la exportación
(`err.to_string()`), ya que un mensaje para la barra de estado es todo
lo que la ventana necesita. Una exportación detenida también vuelve como
`Err`, con la cadena especial `CANCELLED`.

En cada frame, la aplicación comprueba cómo va:

```rust
pub(super) fn check_export(&mut self, ctx: &egui::Context) {
    let Some(export) = &self.export else {
        return;
    };
    if !export.worker.is_finished() {
        ctx.request_repaint_after(Duration::from_millis(100));
        return;
    }
    // `take` moves the export out of the `Option` and leaves `None`,
    // so `join` can take ownership of the thread's handle.
    if let Some(export) = self.export.take() {
        let name = export.file_name();
        match export.worker.join() {
            Ok(Ok(())) => self.notice = Some(trf("Exported {}", &[&name])),
            Ok(Err(err)) if err == CANCELLED => {
                self.notice = Some(trf("Stopped exporting {}", &[&name]));
            }
            Ok(Err(err)) => {
                self.message = Some(trf("Could not export {}: {}", &[&name, &err]));
            }
            // `join` fails only if the thread panicked.
            Err(_) => self.message = Some(trf("Exporting {} failed", &[&name])),
        }
    }
    // ...and closes the window, if a quit was waiting for the export.
}
```

Algunas cosas en las que fijarse:

- egui solo dibuja un frame nuevo cuando pasa algo: un movimiento del
  ratón, una tecla. Mientras corre una exportación puede que no pase
  nada, y la barra de progreso se quedaría quieta.
  `request_repaint_after(100 ms)` pide otro frame pronto, diez veces por
  segundo, hasta que la exportación termina.
- `is_finished()` pregunta sin esperar. `join()` *espera* a que el hilo
  termine, pero solo se llama cuando `is_finished()` es verdadero, así
  que vuelve en el acto.
- `join` toma el handle por valor: lo consume, ya que un hilo solo se
  puede unir una vez. El handle está dentro de `self.export`, un
  `Option`, y no se puede sacar algo de un campo que solo se tiene
  prestado. `Option::take` saca el valor y deja `None` en su lugar, así
  que la exportación desaparece de `self` y pasa a ser de la variable
  local `export`.
- El `match` tiene dos capas: `join` devuelve un `Result` que es `Err`
  solo si el hilo entró en pánico, y dentro de su `Ok` está el propio
  `Result` del closure. Los patrones se anidan, así que `Ok(Ok(()))`
  coincide con «el hilo terminó normalmente y el archivo se escribió».
  El brazo con `if err == CANCELLED` es una *guarda de match*: solo
  coincide cuando se cumple la condición, y el brazo siguiente se queda
  con los demás errores.

### Detener una exportación

Mientras corre una exportación, la barra de estado muestra una barra de
progreso, **Exportando song.wav**, y un pequeño botón **Detener**:

```rust
if let Some(export) = &self.export {
    ui.separator();
    ui.add(
        egui::ProgressBar::new(export.progress())
            .desired_width(160.0)
            .text(trf("Exporting {}", &[&export.file_name()])),
    );
    if export.can_cancel() && ui.small_button(tr("Stop")).clicked() {
        export.cancel();
    }
}
```

`cancel` guarda `true` en el `AtomicBool`, y el hilo de la exportación
lo comprueba antes de cada bloque, como muestra
`export_wav_cancellable` más arriba. Una exportación detenida suelta el
escritor, borra su archivo temporal y devuelve `WavExport::Cancelled`,
así que nunca queda un WAV a medio escribir donde debería haber uno
terminado. Primero se suelta el escritor, lo que cierra el archivo:
Windows no borra un archivo que sigue abierto.

Cerrar la ventana mientras corre una exportación pregunta antes: **Una
exportación aún se está ejecutando**, con **Detener y salir** y
**Continuar exportando**. Funciona como la
[pregunta por los cambios sin guardar](../program/saving.md#cerrar-la-ventana),
que va primero si los hay: la aplicación cancela el cierre con
`ViewportCommand::CancelClose`, y una ventana modal pregunta. Elegir
**Detener y salir** activa el mismo indicador y recuerda que hay un
cierre esperando; cuando `check_export` ve que el hilo terminó, cierra
la ventana. Esperar al hilo le da tiempo para borrar su archivo temporal
antes de que termine el proceso.

## Archivos MIDI: se escriben al instante

Un archivo MIDI guarda solo las notas, unos pocos bytes cada una. Una
canción larga entera ocupa unos pocos kilobytes, hechos en un momento,
así que **Exportar canción como MIDI…** y **Exportar patrón como MIDI…**
no necesitan hilo ni barra de progreso: preguntan dónde guardar y
escriben los bytes con `std::fs::write`. Cómo `song_to_midi` y
`pattern_to_midi` convierten las notas en esos bytes está en
[MIDI](midi.md#escribir-un-archivo-midi).

Toda exportación propone un nombre de archivo hecho a partir del del
proyecto: `FileState::export_name` convierte `song.lineup` en
`song.wav`, `song.mp4` o `song.mid`, o propone `Sin título.wav` para un
proyecto que nunca se guardó. El archivo MIDI de un patrón lleva el
nombre del patrón, con cada `/` o `\` reemplazado, ya que un nombre de
archivo no puede contenerlos. Si escribes un nombre sin extensión, se
añade la correcta.

## Un video: la canción con las vistas acompañando

Un archivo WAV lleva la canción, pero no la armonía que ves mientras
suena. **Exportar video…** guarda un archivo MP4 con el sonido y las
vistas que elijas siguiendo la música: el compás que suena enmarcado en
la hoja de acordes, el acorde iluminado en las teclas de armonía, el
cursor avanzando por la lista de reproducción y el piano roll, y cada
cambio de tonalidad mostrado cuando llega.

![Un fotograma de El Choclo al minuto: el compás 29 enmarcado, E7 iluminado como V7](../images/step-9-video-chord-sheet.png)

Un diálogo pregunta qué vistas mostrar y qué tamaño tienen las
imágenes: **1280 × 720** (HD) o **1920 × 1080** (Full HD). Su vista
previa muestra una imagen del video en el compás que elijas con **Vista
previa en**. En ella, el cuadro de cada vista se puede arrastrar,
redimensionar por su esquina y ampliar girando la rueda encima, del 50 %
al 250 %. **Apilar** vuelve a poner las vistas una sobre otra.

![Partitura guía, lista de reproducción y piano roll juntos](../images/step-9-video-three-views.png)

Hacer un video son cuatro trabajos, y las secciones siguientes los
toman uno por uno:

1. **Dibujar** las vistas como imágenes, sin ventana ni tarjeta
   gráfica.
2. **Comprimir** las imágenes como H.264 y el sonido como MP3.
3. **Poner ambos** en un archivo MP4.
4. **Mantener la imagen con el sonido**, con cambios de tempo y todo.

### Nada que instalar

La forma habitual de hacer un video desde un programa es pasarle
imágenes a FFmpeg. Pero FFmpeg es un programa aparte que Windows y macOS
no traen, así que cada usuario tendría que instalarlo. En cambio, la
aplicación compila dos codificadores muy conocidos a partir de su
código fuente en C junto con ella misma, a través de dos crates:

| Qué | Formato | Codificador | Crate |
|-----|---------|-------------|-------|
| Imágenes | H.264, 30 por segundo | OpenH264 de Cisco (licencia BSD) | `openh264` |
| Sonido | MP3, 256 kbit/s estéreo | LAME (LGPL) | `mp3lame-encoder` |
| Ambos juntos | MP4 | el nuestro, unos cientos de líneas | (`lineup-video`) |

Un crate con C dentro tiene un `build.rs` que ejecuta un compilador de C
antes de que Rust compile el resto, con el crate
[`cc`](https://docs.rs/cc). La [CI](../shipping/ci.md) compila en
Linux, Windows y los dos tipos de Mac, así que demuestra que funciona en
todas partes. Ambos codificadores, y el crate que pinta las imágenes, se
optimizan incluso en las compilaciones de depuración, como el
sintetizador de SoundFonts, por la razón que se da en
[Compilaciones de depuración](soundfonts.md#compilaciones-de-depuración).

H.264 y MP3 no son los formatos más nuevos. Se eligieron porque todo
reproductor, teléfono, navegador y sitio de videos los lee.

### El crate: `lineup-video`

El crate no sabe nada de canciones. Recibe imágenes y sonido y escribe
un archivo. Sus únicas dependencias son los dos codificadores y
`epaint`, la parte de egui que describe las formas. Tiene cuatro
módulos:

- **`canvas`** pinta las formas de egui en una imagen en memoria.
- **`h264`** comprime cada imagen.
- **`mp3`** comprime el sonido.
- **`mp4`** escribe el archivo.

y un tipo que los usa a todos, `VideoWriter`. Le das una imagen, luego
el sonido que suena mientras se muestra, y así hasta el final:

```rust
let mut writer = VideoWriter::create(&path, settings)?;
```

```rust
match scene.draw(time, position) {
    Some(picture) => writer.add_picture(picture)?,
    None => writer.repeat_picture(),
}
renderer.render(&mut sound);
writer.add_sound(&sound)?;
```

```rust
match record() {
    Ok(()) => writer.finish(),
    Err(err) => {
        writer.cancel();
        Err(err)
    }
}
```

`create` escribe en `nombre.mp4.tmp`, `finish` lo renombra y `cancel` lo
borra, como con un archivo WAV. `cancel(self)` toma el escritor por
valor, así que un escritor cancelado no se puede volver a usar: el
compilador te lo impide.

## Dentro de un archivo MP4

Un archivo MP4 es un árbol de **cajas** (boxes). Cada caja es una
longitud de 4 bytes, un nombre de 4 letras y su contenido, que puede
ser más cajas:

```text
ftyp                 "esto es un archivo MP4"
mdat                 todas las imágenes y el sonido comprimidos, uno tras otro
moov                 el índice:
  mvhd                 cuánto dura el conjunto
  trak                 las imágenes:
    tkhd, mdia …         tamaño, reloj y las tablas de abajo
      avc1 + avcC          "H.264", y los conjuntos de parámetros del codificador
      stts                 cuánto tiempo se muestra cada imagen
      stss                 qué imágenes son imágenes clave
      stsz, stco           qué tamaño tiene cada una y dónde está en mdat
  trak                 el sonido:
    edts/elst            dónde empieza de verdad el sonido (ver abajo)
    mp4a + esds          "MP3"
    stts, stsz, stco     las mismas tablas para los frames MP3
```

Una imagen o un trozo de sonido en `mdat` es una *muestra* (sample), y
las tablas de `moov` enumeran cada muestra de una pista. El formato es
ISO/IEC 14496-12 y 14496-14; solo se escribe lo que necesita un video
con una pista H.264 y una pista MP3.

### Una caja cuyo tamaño va al final

Una caja empieza con su tamaño, pero el tamaño solo se conoce cuando su
contenido está escrito. `write_box` deja sitio para él, deja que un
closure escriba el contenido, y después lo rellena:

```rust
/// Writes a box: its size, its name, then whatever `contents` writes.
/// The size is only known afterwards, so it is filled in at the end.
fn write_box(out: &mut Vec<u8>, name: &[u8; 4], contents: impl FnOnce(&mut Vec<u8>)) {
    let start = out.len();
    out.extend_from_slice(&[0; 4]);
    out.extend_from_slice(name);
    contents(out);
    let size = (out.len() - start) as u32;
    out[start..start + 4].copy_from_slice(&size.to_be_bytes());
}
```

Las cajas dentro de cajas son entonces closures dentro de closures, y el
código tiene la forma del árbol:

```rust
write_box(out, b"trak", |out| {
    write_tkhd(out, 1, duration_ms, false, format.width, format.height);
    write_box(out, b"mdia", |out| {
        write_mdhd(out, format.timescale, self.video.duration());
        write_hdlr(out, b"vide", "Pictures");
```

`b"trak"` es una *cadena de bytes*: un `&[u8; 4]`, cuatro bytes, no un
`&str`. El tipo del parámetro, `&[u8; 4]`, hace que el compilador
compruebe que cada nombre tiene exactamente cuatro letras.
`to_be_bytes` da los bytes de un número en *big-endian*, el más
significativo primero, que es como MP4 (y MIDI) guardan los números.

### El índice va al final

`moov` tiene que ir después de los datos. Hasta el final, nadie sabe qué
tamaño tiene cada imagen ni dónde quedó. Así que `Mp4Writer` escribe
`mdat` sobre la marcha, guarda una lista de tamaños y posiciones, y
escribe `moov` en `finish`. Después vuelve atrás para rellenar la
longitud de `mdat`:

```rust
// A size of 1 says the real size follows as 64 bits, so the file
// can grow past 4 GB. It is filled in at the end.
head.extend_from_slice(&1_u32.to_be_bytes());
head.extend_from_slice(b"mdat");
head.extend_from_slice(&0_u64.to_be_bytes());
```

```rust
self.file.seek(SeekFrom::Start(self.mdat_start + 8))?;
self.file
    .write_all(&(self.position - self.mdat_start).to_be_bytes())?;
self.file.seek(SeekFrom::Start(self.position))?;
```

Un video largo en Full HD puede pasar de 4 GB, así que el tamaño es de
64 bits. Por la misma razón, la tabla de dónde empieza cada muestra es
`stco` (posiciones de 32 bits, que todo reproductor lee) mientras el
archivo es lo bastante pequeño, y `co64` en cuanto una posición pasa de
`u32::MAX`.

### Cuánto se muestra cada imagen

`stts` guarda *tramos*: «300 imágenes de 1/30 s cada una» ocupa una
entrada, no 300:

```rust
// How long the samples last, as runs of samples of the same length.
let mut runs: Vec<(u32, u32)> = Vec::new();
for &duration in &track.durations {
    match runs.last_mut() {
        Some((count, last)) if *last == duration => *count += 1,
        _ => runs.push((1, duration)),
    }
}
```

`last_mut` da una referencia mutable al último tramo, si lo hay. La
guarda de match `if *last == duration` lo alarga cuando la nueva
duración es la misma; si no, empieza un tramo nuevo.

El reloj de las imágenes va a 90.000 tics por segundo, el habitual en
video, que se divide exactamente en 24, 25, 30 o 60 imágenes por
segundo. A 30 por segundo, una imagen dura 3.000 tics. Cuando nada
cambia en pantalla, no hay ninguna imagen nueva: `repeat_picture` hace
que la última dure 3.000 tics más. Una pantalla donde solo se mueve un
marco una vez por compás no cuesta casi nada.

### Que el sonido empiece a tiempo

Un codificador MP3 pone 576 muestras de silencio antes del sonido, y un
decodificador añade 529 más. Sin una corrección, la música llegaría
1.105 muestras tarde, 25 ms. Eso es casi una imagen, y basta para ver un
acorde iluminarse antes de oírlo. La **lista de edición** (`elst`) de la
pista de sonido le dice al reproductor dónde empieza el sonido de
verdad:

```rust
// The edit list: play the track from `priming` on, for as long
// as the real sound lasts. The silence the encoder put first,
// and the padding after the last frame, are skipped.
write_box(out, b"edts", |out| {
    write_full_box(out, b"elst", 0, 0, |out| {
        out.extend_from_slice(&1_u32.to_be_bytes());
        out.extend_from_slice(&(duration_ms as u32).to_be_bytes());
        out.extend_from_slice(&format.priming.to_be_bytes());
        out.extend_from_slice(&1_u16.to_be_bytes()); // at normal speed
        out.extend_from_slice(&0_u16.to_be_bytes());
    });
});
```

Una edición: tocar durante lo que dura el sonido real, empezando 1.105
muestras dentro de la pista, a velocidad normal. Para comprobarlo, se
hizo un video de prueba con clics en 1,0, 2,5 y 4,0 s y un destello
blanco en las imágenes en los mismos momentos. Decodificado con FFmpeg,
los clics están exactamente en 1,0, 2,5 y 4,0 s, y los destellos en las
imágenes 30, 75 y 120.

## Imágenes: H.264

Las imágenes llegan como RGBA, cuatro bytes por píxel. H.264 las quiere
como **YUV**: brillo (Y) para cada píxel, y dos diferencias de color (U
y V) para cada cuadrado de cuatro píxeles. El ojo ve el detalle en el
brillo mucho mejor que en el color, así que solo con eso el tamaño se
reduce a la mitad. Después de la primera imagen, el codificador solo
guarda lo que cambió. Cada dos segundos hay una *imagen clave* que se
sostiene sola, para que un reproductor pueda saltar allí y empezar a
dibujar en el acto.

El codificador se configura para lo que muestra un video de la
aplicación: una pantalla, con bordes nítidos, colores planos y texto:

```rust
let config = EncoderConfig::new()
    // A screen recording: sharp edges, flat colours, text.
    .usage_type(UsageType::ScreenContentRealTime)
    .profile(Profile::Baseline)
    .rate_control_mode(RateControlMode::Bitrate)
    .bitrate(BitRate::from_bps(bitrate(width, height)))
    .max_frame_rate(FrameRate::from_hz(frame_rate as f32))
    // Every picture is kept: dropping one to save room would
    // make the playhead stutter.
    .skip_frames(false)
    .complexity(Complexity::High)
    // A key picture every two seconds, so a player can jump.
    .intra_frame_period(IntraFramePeriod::from_num_frames(frame_rate * 2))
    .vui(VuiConfig::bt709().matrix_coefficients(MatrixCoefficients::Smpte170M));
```

La tasa de bits es de 4,5 bits por píxel y por segundo: unos 4 Mbit/s a
1280×720 y 9 a 1920×1080, lo que mantiene el texto nítido y el archivo
pequeño.

Lo que sale es una lista de *unidades NAL*: dos conjuntos de parámetros
que describen el flujo (el tamaño de la imagen, el perfil), y después
las imágenes. El codificador pone `0 0 1` o `0 0 0 1` antes de cada
unidad para marcar dónde empieza. Un archivo MP4 quiere cada unidad
después de su longitud, así que las marcas se quitan, con los
[patrones de slice](midi.md#leer-un-mensaje-patrones-de-slice)
con los que se leen los mensajes MIDI:

```rust
/// A NAL unit without the 0 0 1 or 0 0 0 1 the encoder puts before it to
/// mark where it starts.
fn without_start_code(nal: &[u8]) -> &[u8] {
    match nal {
        [0, 0, 0, 1, rest @ ..] | [0, 0, 1, rest @ ..] => rest,
        _ => nal,
    }
}
```

`rest @ ..` da un nombre al resto del slice, y `|` une dos patrones en
un mismo brazo, ya que ambos ligan `rest`. El resultado toma prestado
del argumento, así que no se copia nada.

El tipo de cada unidad está en los cinco bits bajos de su primer byte.
Los conjuntos de parámetros se apartan, porque el archivo MP4 los guarda
una sola vez, en su caja `avcC`; el resto son la imagen:

```rust
match header & 0x1F {
    SEQUENCE_PARAMETERS => self.sps = Some(nal.to_vec()),
    PICTURE_PARAMETERS => self.pps = Some(nal.to_vec()),
    kind => {
        picture.key |= kind == KEY_PICTURE;
        picture
            .bytes
            .extend_from_slice(&(nal.len() as u32).to_be_bytes());
        picture.bytes.extend_from_slice(nal);
    }
}
```

Un brazo de `match` puede compararse con constantes
(`SEQUENCE_PARAMETERS` es `7`), y el último brazo, un simple nombre,
atrapa cualquier otro valor y lo liga a `kind`.

## Sonido: MP3

LAME recibe las muestras y devuelve *frames* MP3: cada uno contiene
1.152 muestras de cada canal y empieza con una cabecera de 4 bytes. El
archivo MP4 guarda cada frame como una muestra de la pista de sonido,
así que los frames se separan según llegan, leyendo la longitud de cada
uno en su cabecera:

```rust
fn frame_length(bytes: &[u8]) -> Option<usize> {
    let [first, second, third, ..] = *bytes else {
        return None;
    };
    // Sync, MPEG-1 (11), Layer III (01).
    if first != 0xFF || second & 0xFE != 0xFA {
        return None;
    }
    const KBPS: [usize; 15] = [
        0, 32, 40, 48, 56, 64, 80, 96, 112, 128, 160, 192, 224, 256, 320,
    ];
    const RATES: [usize; 3] = [44_100, 48_000, 32_000];
    let kbps = *KBPS.get(usize::from(third >> 4))?;
    let rate = *RATES.get(usize::from((third >> 2) & 0b11))?;
    let padding = usize::from((third >> 1) & 1);
    (kbps > 0).then(|| 144 * kbps * 1000 / rate + padding)
}
```

Los primeros once bits de la cabecera están todos a uno, para que un
decodificador pueda encontrar un frame; luego vienen la versión y la
capa, y en el tercer byte la tasa de bits y la frecuencia de muestreo,
como posiciones en una tabla. A 256 kbit/s y 44,1 kHz un frame ocupa
144 × 256.000 / 44.100 = 835,9 bytes: 835, o 836 cuando el bit de
relleno está activo, para que en promedio los frames salgan exactamente
a la tasa de bits.

`let [first, second, third, ..] = *bytes else { return None; }` es un
*let-else*: toma los tres primeros bytes si hay al menos tres, y si no,
vuelve. `KBPS.get(...)?` devuelve `None` para un número más allá del
final de la tabla, en lugar de entrar en pánico como haría `KBPS[...]`:
los bytes podrían ser cualquier cosa.

### El único `unsafe` del programa

LAME es C, y escribe su salida en la memoria que le damos. El crate
quiere esa memoria como `&mut [MaybeUninit<u8>]`: bytes que todavía
pueden contener cualquier cosa. Un `Vec` tiene espacio más allá de su
final una vez que ha reservado algo, y `spare_capacity_mut` entrega ese
espacio:

```rust
let frames = samples.len() / 2;
self.pending
    .reserve(mp3lame_encoder::max_required_buffer_size(frames));
let written = self
    .encoder
    .encode(InterleavedPcm(samples), spare(&mut self.pending))
    .map_err(|err| err.to_string())?;
// SAFETY: LAME wrote `written` bytes into the spare room.
unsafe { self.pending.set_len(self.pending.len() + written) };
Ok(self.cut())
```

Después hay que decirle al `Vec` que ahora es más largo. `set_len` es
`unsafe`: Rust no puede comprobar que los bytes que añade al `Vec` se
escribieron de verdad, y leer bytes que no se escribieron es
comportamiento indefinido. Un bloque `unsafe` es donde el programador
asume una promesa que el compilador no puede comprobar, y el comentario
`// SAFETY:` dice por qué se cumple: LAME dijo que escribió `written`
bytes allí. Los dos bloques de `mp3.rs` (este y el de `finish`) son el
único código `unsafe` de todo el programa.

La otra forma sería un búfer de ceros que LAME sobrescribe. Es seguro y
un poco más lento, ya que cada byte se escribe dos veces. Cualquiera de
las dos sirve aquí; esta muestra para qué sirve `unsafe`.

## Dibujar la aplicación sin ventana

Las vistas de un video son código egui normal. La hoja de acordes del
video *es* la hoja de acordes de la aplicación, a través de pequeñas
funciones `video` que llaman al mismo código de dibujo sin los menús.
Pero egui no dibuja píxeles por sí mismo. En cada frame convierte la
interfaz en una lista de **triángulos**, cada esquina con un color y un
punto en una textura (la textura contiene las letras de la fuente). En
la aplicación, `egui_glow` envía esos triángulos a la tarjeta gráfica.

La exportación no puede contar con una tarjeta gráfica, y corre en un
hilo propio. Así que cada vista recibe un **panel**: un `egui::Context`
propio, sin ventana, y un `Canvas` que pinta los triángulos con el
procesador. En cada imagen del video, un panel ejecuta el código de la
vista como eframe ejecuta el de la ventana, convierte lo que dibujó en
triángulos y los pinta:

```rust
let ctx = self.ctx.clone();
let mut output = ctx.run_ui(input, &mut contents);
let shapes = std::mem::take(&mut output.shapes);
let primitives = ctx.tessellate(shapes, output.pixels_per_point);
changed |= self.canvas.paint(
    primitives,
    &mut output.textures_delta,
    output.pixels_per_point,
    background,
);
```

`textures_delta` es lo que egui añadió a sus texturas en ese frame: la
imagen de la fuente crece a medida que hacen falta letras y tamaños
nuevos, y el canvas guarda su propia copia, como haría una tarjeta
gráfica.

### Pintar un triángulo

Para pintar un triángulo, se visita cada píxel cercano y se pregunta
«¿está este píxel dentro?». La prueba es la **función de arista**:

```rust
/// Twice the area of the triangle `from`, `to`, `p`: positive when `p`
/// is on the inside of the edge from `from` to `to`, zero on it.
fn edge(from: (i64, i64), to: (i64, i64), p: (i64, i64)) -> i64 {
    (to.0 - from.0) * (p.1 - from.1) - (to.1 - from.1) * (p.0 - from.0)
}
```

Para una arista de A a B, este producto vectorial es positivo de un lado
y negativo del otro. Un píxel está dentro cuando las tres aristas
coinciden. Los mismos tres números, divididos por su total, dicen lo
cerca que está el píxel de cada esquina. Con eso se mezclan los colores
y las coordenadas de textura de las esquinas, y así las letras toman su
forma. Algunos detalles hacen que coincida con la pantalla:

- **Números enteros, en 1/256 de píxel.** Dos triángulos que comparten
  una arista no deben pintar ambos los píxeles que caen sobre ella, o
  una forma semitransparente recibe una costura más oscura. Con enteros
  no hay dudas de redondeo, y la *regla de arriba a la izquierda*
  resuelve los empates: un píxel justo sobre una arista pertenece al
  triángulo solo si esa arista es superior o izquierda.
- **Colores premultiplicados.** egui guarda los colores ya
  multiplicados por su opacidad, y mezcla con
  `nuevo + viejo × (1 − opacidad)`, como hace su shader de la tarjeta
  gráfica.
- **Avanzar, no multiplicar.** Moverse un píxel a la derecha cambia cada
  función de arista en una constante, así que el bucle interior suma en
  lugar de multiplicar. Para los triángulos simples de un solo color, la
  mayor parte de una interfaz, un camino rápido se salta la textura por
  completo.

### Franjas de filas, en cada núcleo

La imagen se corta en franjas de filas, y cada franja se pinta en su
propio núcleo:

```rust
self.pixels.fill(background.to_array());
let width = self.width;
let cores = thread::available_parallelism().map_or(1, |n| n.get().min(8));
let rows_per_band = self.height.div_ceil(cores).max(1);
let textures = &self.textures;
thread::scope(|scope| {
    for (band, pixels) in self.pixels.chunks_mut(rows_per_band * width).enumerate() {
        let top = band * rows_per_band;
        let bottom = top + pixels.len() / width;
        scope.spawn(move || {
            let mut band = Band {
                pixels,
                width,
                top,
                bottom,
            };
            for clipped in primitives {
                if let Primitive::Mesh(mesh) = &clipped.primitive {
                    band.paint_mesh(mesh, clipped.clip_rect, pixels_per_point, textures);
                }
            }
        });
    }
});
```

Estos hilos *toman prestado*: cada uno recibe un slice `&mut` de la
imagen, y todos comparten `&self.textures` y los triángulos.
`thread::spawn` no podría hacerlo, ya que su closure debe ser
`'static`. **`std::thread::scope`** sí puede, porque no vuelve hasta que
cada hilo creado dentro de él ha terminado, así que los préstamos no
pueden sobrevivir a lo que apuntan. El compilador comprueba el resto:
`chunks_mut` da slices que no se solapan, así que dos hilos nunca pueden
escribir el mismo píxel.

Cada franja pinta todos los triángulos, pero solo sus propias filas de
cada uno. Un triángulo que cruza dos franjas lo pintan a medias entre
las dos.

### Un cuadro para cada vista, y el zoom

El zoom funciona como el propio zoom de egui. El panel le dice a su
contexto que hay más píxeles por punto, 1,4 veces más al 140 %, así que
egui distribuye la vista en menos puntos y dibuja cada letra al tamaño
mayor. Si la vista se dibujara pequeña y luego se estirara la imagen,
las letras saldrían borrosas:

```rust
let ppp = self.pixels_per_point;
let size = vec2(self.canvas.width() as f32, self.canvas.height() as f32) / ppp;
let mut input = egui::RawInput {
    screen_rect: Some(Rect::from_min_size(Pos2::ZERO, size)),
    time: Some(time),
    predicted_dt: 1.0 / frame_rate as f32,
    ..Default::default()
};
input
    .viewports
    .entry(egui::ViewportId::ROOT)
    .or_default()
    .native_pixels_per_point = Some(ppp);
```

`RawInput` es lo que eframe le da a egui en cada frame: el tamaño de la
pantalla, el tiempo, el ratón y las teclas. Aquí es inventado, sin ratón
ni teclas, y con el tiempo *del video*, así que lo que se mueve con el
tiempo se mueve con el video. `..Default::default()` rellena cada campo
que no se nombra.

Full HD distribuye las vistas exactamente como HD, 1280 por 720 puntos,
a 1,5 píxeles por punto: el texto es más nítido, pero no cabe nada más.
Cada panel pinta su propio canvas, y `Canvas::copy_from` copia los
paneles en la imagen, en sus cuadros, bajo una línea de título con el
nombre de la canción, el compás y el pulso, la tonalidad, el compás de
la partitura y el tempo. Los cuadros pueden solaparse: la vista más
abajo en la lista se copia la última, encima.

### A tiempo

Dos detalles mantienen las vistas a tiempo:

- **Sin deslizamiento.** En la ventana, una vista se desplaza suavemente
  hasta el compás que suena, en aproximadamente medio segundo. En un
  video eso es medio segundo tarde, así que cada panel desactiva la
  animación de desplazamiento de egui y salta allí:
  `style.scroll_animation = egui::style::ScrollAnimation::none()`.
- **Ni una imagen tarde.** Una vista que se desplaza solo se mueve en el
  frame *siguiente*, y le pide uno a egui de inmediato (un retraso de
  redibujado de cero). Cuando eso pasa, el panel hace una segunda pasada
  dentro de la misma imagen, así que el compás aparece en su primer
  pulso. Un panel cuyo tamaño o zoom acaba de cambiar también se dibuja
  siempre dos veces, porque egui mide algunas cosas en su primer frame.

### Imágenes que no cambiaron

Mientras suena una canción, la mayoría de las imágenes son iguales a la
anterior: la hoja de acordes solo se mueve cuando cambia el compás.
`Canvas::paint` guarda los triángulos de la última imagen, y pinta solo
si los nuevos son distintos:

```rust
/// Whether two lists of shapes paint the same picture.
fn same(new: &[ClippedPrimitive], old: &[ClippedPrimitive]) -> bool {
    new.len() == old.len()
        && new.iter().zip(old).all(|(new, old)| {
            new.clip_rect == old.clip_rect
                && match (&new.primitive, &old.primitive) {
                    (Primitive::Mesh(new), Primitive::Mesh(old)) => new == old,
                    _ => false,
                }
        })
}
```

Comparar dos mallas con `==` compara cada esquina de cada triángulo, lo
que es mucho más barato que pintarlas. Cuando ningún panel cambió y
ningún cuadro se movió, `Scene::draw` devuelve `None`, el codificador no
se ejecuta y `repeat_picture` hace que la última imagen dure más.
Saltarse las imágenes sin cambios y pintar los triángulos simples por el
camino rápido hacen que exportar un video sea varias veces más rápido
que pintar cada imagen entera.

### Ajustes que viven en un hilo

Los nombres de las notas, las traducciones y los números romanos en
menor son
[ajustes por hilo](../program/languages.md#dos-ajustes-guardados-como-el-de-los-números-romanos),
valores `thread_local!` que la ventana fija antes de dibujar cada frame.
La exportación dibuja en un hilo propio, que empieza con los valores por
defecto: el video de un usuario en español saldría en inglés y en
letras. Así que la exportación lee los tres ajustes en el hilo de la
interfaz, los mueve al closure (son pequeños valores `Copy`) y los fija
lo primero en su propio hilo:

```rust
let numerals = MinorNumerals::current();
let language = crate::i18n::Language::current();
let naming = lineup_core::NoteNaming::current();
```

```rust
thread::spawn(move || {
    // The views are made here, on the export's own thread,
    // and write numerals, words and note names as the app does.
    numerals.set();
    language.set();
    naming.set();
    // ...then the scene, the writer and the pictures, below.
})
```

## Mantener la imagen con el sonido

La canción la toca el mismo `Renderer` fuera de línea que una
exportación WAV, 1/30 s cada vez: a 44.100 muestras por segundo,
exactamente 1.470 muestras por imagen. Las imágenes tienen que mostrar
*dónde está la música*, y con cambios de tempo, ritardandos y calderones
eso no es una suma simple. Así que el renderer lo informa. Después de
cada bloque de sonido, el hilo de audio guarda su posición en pasos, con
una fracción (4,5 es la mitad del paso 4), como hace para la ventana, y
`Renderer::position` la devuelve:

```rust
for picture in 0..pictures {
    if cancel.load(Ordering::Relaxed) {
        return Err(CANCELLED.to_string());
    }
    let time = picture as f64 / f64::from(FRAME_RATE);
    // Before the first sound, the song is about to
    // start; after it, where the sound so far ended.
    let position = if picture == 0 {
        Some(0.0)
    } else {
        renderer.position()
    };
    match scene.draw(time, position) {
        Some(picture) => writer.add_picture(picture)?,
        None => writer.repeat_picture(),
    }
    renderer.render(&mut sound);
    writer.add_sound(&sound)?;
    done.store(picture + 1, Ordering::Relaxed);
}
```

Cada imagen se dibuja *antes* del sonido que suena mientras se muestra,
en la posición donde empieza ese sonido.

La posición cuenta los pasos *tal como suenan*: con una repetición, o un
estribillo que la banda toca tres veces, ese no es el lugar en la lista
de reproducción. `Scene::draw` la traduce con el arreglo de la canción,
como hace la ventana con su cursor:

```rust
let at = position.and_then(|position| {
    let step = position.max(0.0) as usize;
    let pass = *self.song.pass_at(step)?;
    Some((pass.from + (step - pass.start), pass))
});
```

Una *pasada* (pass) es un tramo de la lista de reproducción tocado una
vez: desde qué lugar de la lista (`from`), empezando en qué paso tocado
(`start`). Cómo el arreglo despliega las repeticiones en pasadas está en
[Forma de la canción](../band/song-form.md).

Para comprobarlo, se exportó El Choclo. A 112 BPM en 4/4, el compás 15
empieza a los 30,0 s, el compás 29 a los 60,0 s y el compás 43 a los
90,0 s, y la [traza de reproducción](trace.md), que toca a
través del mismo renderer, tiene las primeras notas de esos compases
exactamente en esos tiempos. Las imágenes del video en esos momentos
muestran «Compás 15, pulso 1», «Compás 29, pulso 1» y «Compás 43,
pulso 1».

## Organizar las vistas

Dónde va cada cuadro y a qué tamaño se dibuja su vista es un `Layout`:
un `Placement` para cada una de las cinco vistas.

```rust
/// Where a view goes in the picture, and how big it is drawn.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Placement {
    pub shown: bool,
    /// Its box: left, top, right and bottom, from 0 to 1 across the room
    /// under the title line.
    pub rect: [f32; 4],
    /// How big it is drawn: 1 as in the window, 2 twice as big.
    pub zoom: f32,
}
```

El cuadro se guarda en partes del espacio, de 0 a 1, no en píxeles, así
que la misma distribución sirve para los dos tamaños, y para la vista
previa del diálogo, que mide 640 puntos de ancho.

Las ubicaciones viven en un array, pero el código las lee por vista:
`layout[View::ChordSheet]`. Eso funciona porque `Layout` implementa los
traits de indexación, con un `View` como índice:

```rust
impl std::ops::Index<View> for Layout {
    type Output = Placement;
    fn index(&self, view: View) -> &Placement {
        &self.views[view.index()]
    }
}

impl std::ops::IndexMut<View> for Layout {
    fn index_mut(&mut self, view: View) -> &mut Placement {
        &mut self.views[view.index()]
    }
}
```

`Index` es lo que llama `layout[view]` para leer, e `IndexMut` lo que
llama para cambiar, como en `layout[View::HarmonyKeys].shown = true`.
`View` es un enum sin datos, así que `view as usize` da su posición en
la declaración, que es lo que devuelve `View::index`. Nadie puede
indexar el array con un número que no sea una vista.

Arrastrar un cuadro cambia su `rect` con `move_by`, su esquina con
`resize_by`, y la rueda del ratón su zoom con `zoom_by`, manteniendo
siempre el cuadro dentro del espacio y el zoom entre el 50 % y el 250 %.
La rueda amplía por `(scroll / 500.0).exp()`: girarla hacia arriba y
luego hacia abajo la misma cantidad multiplica por un número y después
por su inverso, así que vuelve exactamente a donde estaba. **Apilar**
llama a `Layout::stack`, que pone las teclas de armonía abajo, tan altas
como necesitan, y reparte el espacio de arriba por igual entre las demás
vistas mostradas.

El diálogo [recuerda](../program/egui.md#recordar-entre-ejecuciones)
la distribución y el tamaño entre ejecuciones, con `get_persisted` e
`insert_persisted` de egui. `Layout` y `Placement` derivan `Serialize` y
`Deserialize`, y `#[serde(default)]` rellena cualquier campo que falte
en lo guardado con su valor por defecto, así que los ajustes escritos
por otra versión de la aplicación se siguen leyendo.

![El diálogo Exportar video: la hoja de acordes movida a la derecha y ampliada al 250 %](../images/step-9-video-layout.png)

Aquí la hoja de acordes, ampliada al 140 %, está al lado de la partitura
guía, con las teclas de armonía al 120 % debajo:

![El Choclo a 1920×1080: hoja de acordes y partitura guía una al lado de la otra](../images/step-9-video-side-by-side.png)

## Cómo se prueba

Los tests del renderer tocan en un `Vec` y miran los números:

- `renders_like_the_audio_thread`: un clic hecho por el renderer es
  exactamente el clic que hace el hilo de audio, `tanh(0.5)` en los dos
  canales, y dura diez frames.
- `knows_where_playback_is`: a 120 BPM un paso dura 6.000 frames a
  48 kHz, así que después de 9.000 frames la posición es 1,5 pasos.
- `exports_a_wav_file`: el archivo tiene el formato, la duración y la
  primera muestra correctos, `progress` se llamó después de cada bloque,
  y no queda ningún archivo `.tmp`.
- `a_cancelled_export_leaves_no_file`: después de detenerla no existen
  ni el archivo ni el temporal.

Los tests del escritor MP4 comprueban los bytes:
`a_box_starts_with_its_size_and_name`,
`samples_of_the_same_length_are_listed_as_runs`, y
`writes_a_movie_the_tables_describe`, que vuelve a leer dónde quedó cada
muestra y cuánto dura la película. Los tests de la escena dibujan
imágenes y cuentan píxeles: `views_are_drawn_in_their_boxes` pone la
hoja de acordes en la mitad derecha y comprueba que la mitad izquierda
está vacía, y `zoomed_in_views_are_drawn_bigger` comprueba que una vista
al 200 % tiene mucha más tinta que al 100 %. Dos tests de la aplicación
exportan un video entero de la canción inicial con todas las vistas, y
detienen otro a mitad de camino para comprobar que no queda ningún
archivo.

Lo que los tests no pueden comprobar es si un archivo se reproduce bien
en todas partes. Para eso, abre una exportación en cualquier editor de
audio: la forma de la canción se ve fácilmente en la onda. El archivo
WAV de la canción inicial tiene exactamente 739.523 frames, el canal
izquierdo y el derecho son iguales (todos sus canales están en el centro
y la reverberación está apagada), los compases 1 y 2 tienen los acordes
solos, los compases 3 a 7 el ritmo (el patrón Pulso), el compás 8 el relleno, y la cola
está en silencio, sin ningún acorde de la canción que vuelva a empezar.
Un video debería abrirse en cualquier reproductor, y saltar de un lado a
otro debería empezar a dibujar en el acto, gracias a una imagen clave
cada dos segundos.

Los ejercicios sobre exportar están en
[Ejercicios: el sonido](exercises.md).
