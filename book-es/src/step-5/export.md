# Exportar la canción como archivo WAV

Archivos: [`crates/lineup-audio/src/render.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-audio/src/render.rs)
y [`crates/lineup-app/src/app/export.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/export.rs)

## Qué hay en un archivo WAV

Un archivo WAV es una cabecera corta (aquí, 44 bytes) que dice cómo está
guardado el sonido, seguida de las propias muestras. Ferrous Lineup
escribe el tipo más común:

- **44,100 fotogramas por segundo**, la frecuencia de un CD, que entiende
  cualquier programa,
- **2 canales** (estéreo), guardados **intercalados**: izquierda,
  derecha, izquierda, derecha..., exactamente como los búferes que cpal
  entrega al hilo de audio desde el paso 1,
- **16 bits por muestra**: cada muestra es un `i16`, un número entero de
  -32,768 a 32,767.

Eso da 44,100 × 2 × 2 = 176,400 bytes por segundo de sonido, unos
10.6 MB por minuto. La canción de demostración con su cola de dos
segundos son 739,523 fotogramas, 16.77 segundos, y su archivo son
44 + 739,523 × 4 = 2,958,136 bytes.

Escribir la cabecera y las muestras a mano sería fácil, pero el crate
`hound` ya lo hace, y la aplicación lo usa desde el paso 2 para *leer*
archivos WAV para el sampler.

## El hilo de audio, sin tarjeta de sonido

Mira el hilo de audio desde fuera, y es una función que llena un búfer:

```rust
pub(crate) fn process(&mut self, output: &mut [f32])
```

No sabe ni le importa que sea cpal quien pregunta. Con una tarjeta de
sonido, cpal pide unos cientos de fotogramas cada vez, tan rápido como se
reproducen. Para hacer un archivo, un loop los pide tan rápido como el
ordenador puede producirlos, y los escribe. Es el mismo código
produciendo el mismo sonido, así que una exportación suena exactamente
como la reproducción: no hay un segundo motor que mantener de acuerdo con
el primero. (Las pruebas del motor hacen esto desde el paso 2: tampoco
abren nunca una tarjeta de sonido.)

`lineup-audio` lo envuelve en un **`Renderer`**:

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
}
```

`AudioThread` sigue siendo `pub(crate)`: otros crates no pueden construir
uno, llamar a `process` sobre él ni alcanzar sus campos. Reciben el
`Renderer`, cuyos métodos mantienen las reglas. Hacer de la exportación
una función *dentro de* `lineup-audio` (el ejercicio 6 del paso 4
preguntaba por esto) es lo que deja que el motor siga siendo privado.

`AudioThread::new` quiere el extremo receptor de una cola de comandos,
porque en el hilo de audio real es así como llegan los comandos. El
renderer no tiene otro hilo del que recibir, así que hace una cola con
sitio para un comando, no la usa nunca, y llama a `apply` directamente:

```rust
/// Applies a command at once: there is no other thread to hand it to.
/// Volume changes take effect at once too, without smoothing: the
/// commands come before the first sample is made.
pub fn send(&mut self, command: Command) {
    self.audio.apply(command);
    self.audio.settle_volumes();
    self.empty_trash();
}
```

El campo que guarda el otro extremo de la cola se llama `_commands`. Un
guion bajo al principio le dice al compilador (y al lector) que el campo
no se lee a propósito, para que no avise "field `commands` is never
read". El campo sigue importando: mantiene vivo al producer mientras viva
el renderer.

### Dos detalles que solo aparecen sin tarjeta

**El suavizado.** Desde el paso 2, los cambios de volumen se deslizan a
lo largo de unos milisegundos (`Smoothed`), porque un salto brusco
produce un clic audible. En una tarjeta de sonido, ese deslizamiento
ocurre mientras suena la música. En una exportación, poner el volumen
maestro justo antes de la primera muestra haría que los primeros
milisegundos del archivo se deslizaran desde el volumen por defecto
hasta el tuyo. La primera prueba del renderer lo detectó: el primer clic
salía demasiado flojo. Así que `send` llama a `settle_volumes`, que pone
cada volumen en su objetivo de golpe, con un nuevo `Smoothed::settle`:

```rust
/// Jumps straight to the target, with no smoothing.
pub fn settle(&mut self) {
    self.current = self.target;
}
```

**La basura.** El hilo de audio real nunca libera memoria: lo que tira
(una lista antigua de notas, un canal quitado) va a la cola de basura, y
el hilo de la interfaz lo libera una vez por fotograma. El renderer no es
un hilo de tiempo real, así que puede liberar las cosas al momento, y
vacía la cola después de cada comando y de cada bloque. Nada se rompería
si se olvidara (el hilo de audio libera las cosas él mismo, como último
recurso, cuando la cola está llena), pero así el renderer se comporta
como el de verdad.

## Escribir el archivo

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

`export_wav` es un envoltorio fino para quienes no pueden cancelar: pasa
un flag que nunca se pone y devuelve el antiguo
`Result<(), hound::Error>`. La aplicación usa la versión cancelable.

El sonido se hace en bloques de 4,096 fotogramas, en un búfer que se
reutiliza para cada bloque. El último bloque suele ser más corto, así que
`samples` es un slice de solo la parte que se necesita. Después de cada
bloque, se llama a `progress` con el número de fotogramas hechos. Es un
`impl FnMut(usize)`: cualquier closure que reciba un número, y que puede
cambiar lo que capturó (el closure de la aplicación guarda el número en
algún sitio que la ventana pueda leer).

Cada `?` de aquí devuelve un `hound::Error`. `fs::rename` devuelve en su
lugar un `std::io::Error`, y `?` sigue funcionando, porque
`hound::Error` implementa `From<std::io::Error>`: el operador `?`
convierte el error al salir, como hacía el `FileError` del paso 4.

### Escribir en un archivo temporal y luego renombrar

Las muestras van primero a `song.wav.tmp`, y solo cuando está todo
escrito se renombra a `song.wav`. Exportar sobre un `song.wav` más
antiguo significa que el archivo viejo queda entero hasta que el nuevo
está completo: si el disco se llena o algo falla a mitad de camino, te
quedas con tu exportación antigua, no con la mitad de una nueva.
Renombrar un archivo dentro de una misma carpeta sustituye al antiguo
tanto en Linux como en macOS y Windows, y en Linux y macOS lo hace en un
solo paso indivisible.

`as_mut_os_string().push(".tmp")` añade al nombre.
`set_extension("tmp")` *sustituiría* `.wav`, dando `song.tmp`, que
podría ser el nombre del archivo de otra persona.

### De `f32` a `i16`

```rust
/// Turns a sample between -1.0 and 1.0 into a 16-bit integer sample.
fn to_i16(sample: f32) -> i16 {
    (sample.clamp(-1.0, 1.0) * f32::from(i16::MAX)).round() as i16
}
```

`clamp` mantiene la muestra entre -1.0 y 1.0, luego se escala a
-32,767..32,767 y se redondea al número entero más cercano. Un cast con
`as` de un float a un entero ya se detiene en los límites del entero en
Rust (*satura*: `3e9 as i16` es 32,767), así que el clamp no es
estrictamente necesario para la seguridad, pero dice lo que se quiere
decir. `f32::from(i16::MAX)` es una conversión que nunca puede perder
nada, así que es un `From`, no un `as`.

El hilo de audio ya mantiene la salida maestra dentro de -1.0..1.0 con
su recortador suave (`tanh`, del paso 2), así que en la práctica el
clamp nunca recorta nada.

## ¿Cuánto dura una canción?

La longitud de la canción se conoce en pasos; la del archivo, en
fotogramas:

```rust
// The whole song, every repeat and section included, even when song
// mode only loops part of it.
let song = self.project.arrangement();
let song_length = song.length;
let tempos = self.project.played_tempos(&song);
let end_tempo = tempos.last().copied().unwrap_or(self.project.tempo);
let step_frames = frames_per_step(end_tempo, EXPORT_SAMPLE_RATE);
let reverb = self.project.mixer.reverb;
let tail_seconds = if reverb.enabled {
    TAIL_SECONDS + f64::from(reverb.decay + reverb.pre_delay_ms / 1000.0)
} else {
    TAIL_SECONDS
};
let tail_frames = tail_seconds * f64::from(EXPORT_SAMPLE_RATE);
let song_frames = self.project.played_seconds(&song) * f64::from(EXPORT_SAMPLE_RATE);
...
let frames = (song_frames + tail_frames) as usize;
```

A 130 BPM hay 4 pasos por pulso, así que un paso dura 60 / 130 / 4 =
0.115 segundos, 5,088.46 fotogramas a 44,100. Los 128 pasos de la demo
ocupan 651,323 fotogramas, más una cola de dos segundos de 88,200, hacen
739,523. `played_seconds` suma cada paso al tempo que rige allí, así que
una canción con cambios de tempo sigue teniendo la longitud correcta. Con
la reverberación activada, la cola también espera su decay y su
pre-delay, para que la sala pueda apagarse en lugar de cortarse.

La cola deja que las últimas notas resuenen. Pero hay una trampa: en el
modo canción el hilo de audio hace *loop*. Cuando el playhead llega al
final de la canción, empieza otra vez desde el compás 1, así que la cola
terminaría con el primer acorde de la canción empezando de nuevo. La
solución es decirle a este renderer, y solo a este, que la canción es más
larga de lo que es en realidad, con unos pasos en silencio:

```rust
// The song loops when it reaches its end, so silent steps are
// added after it: enough for the tail, and one more, so the
// export ends before the song would start again.
let silent_steps = (tail_frames / step_frames).ceil() as usize + 1;
renderer.send(Command::SetSong {
    plays: song.plays,
    length: song_length + silent_steps,
});
renderer.send(Command::SetSongTempos(tempos));
```

Para la demo eso es 17.33, redondeado hacia arriba a 18, más uno: 19
pasos en silencio. ¿Por qué el extra? Un paso no es un número entero de
fotogramas, así que el secuenciador empieza algunos pasos un fotograma
antes o después de la fracción exacta. El paso extra significa que el
final del archivo queda siempre a un paso entero del loop, redondee como
redondee.

## Un hilo propio

Renderizar lleva un rato. En el ordenador donde se escribió esto, la demo
se exporta en una fracción de segundo, y una canción de 15 minutos tardó
unos tres segundos (en una compilación release; una debug es muchas veces
más lenta). La ventana no debe congelarse mientras tanto. Así que la
exportación se ejecuta en un **hilo** propio:

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

`std::thread::spawn` lanza un hilo nuevo que ejecuta un closure, y quiere
dos cosas de él:

- **`'static`**: el closure no puede tomar prestado nada del código que
  lo lanzó. El hilo nuevo puede sobrevivir a la función que lo lanzó
  (desde luego sobrevive a este fotograma de la interfaz), así que una
  referencia prestada podría apuntar a algo que ya no está. Por eso el
  closure se marca como **`move`**: toma la *propiedad* de todo lo que
  usa (el renderer, la ruta, el contador) en lugar de tomarlo prestado.
- **`Send`**: todo lo que se mueve a él debe ser seguro de entregar a
  otro hilo. El renderer cumple, porque todo lo que contiene también: los
  instrumentos, los `Arc`s con las muestras, los extremos del ring
  buffer. Si tuviera un `Rc` (un recuento de referencias que no es seguro
  para hilos), esta línea no compilaría.

El bloque `{ let done = Arc::clone(&done); ... }` es un truco habitual.
Tanto la interfaz como el hilo necesitan el contador, así que cada uno
recibe su propio `Arc` apuntando a él. El `done` interior ensombrece al
exterior dentro del bloque, y el closure `move` toma el interior,
dejando el `done` exterior para la interfaz. Lo mismo vale para `path`.

### El progreso: un contador atómico

El hilo escribe cuántos fotogramas están hechos en un **`AtomicUsize`**,
y la ventana lo lee cuando dibuja la barra de progreso. Un atómico se
puede leer y escribir desde dos hilos a la vez sin cerrojo, como el
playhead desde el paso 2.

`Ordering::Relaxed` es la promesa más débil (y barata) sobre cómo ven
este escrito los demás hilos: el propio número nunca se parte ni se
mezcla, pero no se garantiza que nada más se vea en orden con él. Es todo
lo que necesita una barra de progreso. Nada más se lee a causa del valor
del contador; del archivo solo se fía uno cuando el hilo ha terminado, y
`join` (más abajo) da la garantía completa.

### Saber cuándo ha terminado

`spawn` devuelve un **`JoinHandle`**. La aplicación lo guarda, con el
contador, en un `Export`:

```rust
pub(super) struct Export {
    path: PathBuf,
    frames: usize,
    done: Arc<AtomicUsize>,
    /// Set to ask the thread to stop, for exports that can be cancelled.
    cancel: Option<Arc<AtomicBool>>,
    worker: JoinHandle<Result<(), String>>,
}
```

`JoinHandle<Result<(), String>>` dice lo que devuelve el closure del
hilo: `Ok(())` si el archivo se escribió, o un mensaje de error. El error
se convierte en un `String` en el hilo de exportación
(`map_err(|err| err.to_string())`), porque un mensaje para la barra de
estado es todo lo
que necesita la ventana. Una exportación cancelada también vuelve como
`Err`, con el string especial `CANCELLED`; la barra de estado lo
convierte en "Stopped exporting song.wav" en lugar de mostrarlo como un
fallo.

Cada fotograma, la aplicación lo comprueba:

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
            Ok(Ok(())) => self.notice = Some(format!("Exported {name}")),
            Ok(Err(err)) => self.message = Some(format!("Could not export {name}: {err}")),
            // `join` fails only if the thread panicked.
            Err(_) => self.message = Some(format!("Exporting {name} failed")),
        }
    }
}
```

Unas cuantas cosas que notar:

- egui solo redibuja cuando pasa algo (un movimiento del ratón, una
  tecla). Mientras corre una exportación, puede que no pase nada, y la
  barra de progreso se quedaría quieta. `request_repaint_after(100 ms)`
  pide otro fotograma pronto, diez veces por segundo, hasta que termina
  la exportación.
- `is_finished()` pregunta sin esperar. `join()` *espera* a que termine
  el hilo, pero solo se llama cuando `is_finished()` es true, así que
  vuelve al momento.
- `join` toma el handle por valor (lo consume: un hilo solo se puede
  unir una vez). El handle está dentro de `self.export`, un `Option`, y
  no puedes mover algo fuera de un campo que solo tomas prestado.
  **`Option::take`** lo resuelve: mueve el valor fuera y deja `None` en
  su lugar, así que la exportación desaparece de `self` y pasa a ser
  propiedad del `export` local.
- El `match` tiene dos capas: `join` devuelve un `Result` que es `Err`
  solo si el hilo hizo panic, y dentro de su `Ok` está el propio
  `Result` del closure. Los patrones se pueden anidar, así que
  `Ok(Ok(()))` encaja con "el hilo terminó con normalidad, y el archivo
  se escribió".

### Detener una exportación

La barra de estado dibuja un pequeño botón Stop mientras
`Export::can_cancel()` es true. Llama a `export.cancel()`, que pone un
`AtomicBool`; el hilo de exportación comprueba ese flag entre bloques:

```rust
while done < frames {
    if cancel.load(Ordering::Relaxed) {
        drop(writer);
        let _ = fs::remove_file(&temporary);
        return Ok(WavExport::Cancelled);
    }
    // ...render and write one block...
}
```

`export_wav_cancellable` recibe el flag, y devuelve un `WavExport`:
`Written` o `Cancelled`. El `export_wav` normal construye un flag que
nunca se pone y mantiene su antigua interfaz `Result<(), hound::Error>`
para las pruebas. Una exportación cancelada borra el archivo temporal,
para que un WAV a medio escribir no quede nunca donde debería estar uno
terminado. La aplicación convierte `Cancelled` en el mensaje `CANCELLED`
de arriba.

Cerrar la ventana mientras corre una exportación pregunta primero: "Stop
it and quit?" o "Keep exporting". Es un segundo uso del mismo flag, y le
da al hilo de exportación tiempo para quitar su archivo temporal antes de
que termine el proceso.

## Meter la canción en el renderer

La exportación construye su renderer con los mismos comandos que la
aplicación envía al arrancar y después de abrir un archivo, pero con
instrumentos hechos para 44,100 fotogramas por segundo, sea cual sea la
frecuencia a la que corra tu tarjeta de sonido:

```rust
let mut renderer = Renderer::new(EXPORT_SAMPLE_RATE, EXPORT_CHANNELS);
for command in self.project_commands(EXPORT_SAMPLE_RATE, 1.0) {
    renderer.send(command);
}
// ...the longer song...
renderer.send(Command::SetMode(PlayMode::Song));
renderer.send(Command::Play);
```

`project_commands` es nuevo en este paso: enumera el tempo y el timing
suelto, el volumen maestro y la reverberación, cada patrón
(`InsertPattern`), cada canal (`InsertChannel`, con sus notas en todos
los patrones) y la canción, sus tempos y sus clics. El segundo argumento
es la velocidad de práctica: la reproducción puede ralentizar la canción
para practicar, pero una exportación va siempre a la velocidad propia de
la canción, así que pasa `1.0`. `send_whole_project`, que habla con el
hilo de audio real, usa la misma función con `self.practice.speed()`,
así que ambas se construyen igual. La batería integrada se sintetiza de
nuevo a 44,100, el Lineup Osc se hace para esa frecuencia, y los samplers
obtienen sus sonidos de la `SampleCache` del capítulo anterior, así que
no se lee ningún archivo.

El renderer posee sus propios instrumentos y su propia copia de cada
nota. Es una instantánea, como las copias del historial de deshacer:
puedes seguir reproduciendo y editando mientras corre, y la exportación
continúa con la canción tal como estaba cuando elegiste Export WAV.

El nombre del archivo viene del del proyecto: `FileState::export_name`
convierte `song.lineup` en `song.wav`, o sugiere `Untitled.wav` para un
proyecto que nunca se guardó. Si escribes un nombre sin extensión, se
añade `.wav`.

## Contra qué se comprobó el resultado

Además de las pruebas unitarias (un clic renderizado por el renderer es
exactamente el clic que produce el hilo de audio; un archivo WAV tiene el
formato, la longitud y la primera muestra correctos, y no queda ningún
archivo `.tmp`), la canción de demostración se exportó desde la
aplicación en marcha y el archivo se leyó de vuelta:

- exactamente 739,523 fotogramas, como se calculó arriba,
- la izquierda y la derecha son iguales (los canales de la demo están
  todos en el centro y la reverberación está apagada),
- los compases 1 y 2 tienen solo acordes, los compases 3 a 7 el pulso, el
  compás 8 el fill,
- la cola queda en silencio, sin que ningún acorde de la canción empiece
  de nuevo.

Puedes hacer lo mismo abriendo el archivo en cualquier editor de audio:
la forma de la canción se ve fácil en la forma de onda.
