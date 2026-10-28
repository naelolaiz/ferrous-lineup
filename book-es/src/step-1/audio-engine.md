# lineup-audio: el motor en tiempo real

Este es el capítulo más importante del paso 1. La forma en que funciona
el audio moldea todo el programa, y también es donde las reglas de
propiedad de Rust empiezan a demostrar su valor.

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
- no **reservar memoria** (`Vec::push`, `String`, `Box::new`...),
- no hacer **I/O de archivos, red o consola** (`println!` incluido),
- no esperar a otros hilos.

Esta regla decide cómo se comunica la interfaz con el motor de audio.

## Enviar comandos con un ring buffer

Archivo:
[`crates/lineup-audio/src/engine.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-audio/src/engine.rs)

La interfaz nunca cambia el estado del audio directamente. Envía
mensajes pequeños:

```rust
pub enum Command {
    SetToneOn(bool),
    SetFrequency(f32),
    SetVolume(f32),
}
```

Es un **enum** de Rust. La diferencia de los enums de Do o Java, cada
variante puede llevar sus propios datos: `SetToneOn` lleva un `bool`,
`SetFrequency` un `f32`. Un valor de enum es exactamente una de sus
variantes.

Los mensajes viajan por un **ring buffer** del crate
[`rtrb`](https://docs.rs/rtrb): una cola de tamaño fijo, reservada una
sola vez, con un escritor (el `Producer`) y un lector (el `Consumer`).
Añadir y extraer nunca bloquea ni reserva memoria, así que el hilo de
audio puede leerlo con seguridad.

```rust
let (producer, consumer) = rtrb::RingBuffer::new(COMMAND_QUEUE_SIZE);
```

`RingBuffer::new` devuelve una **tupla** de dos valores, que separamos
con un **patrón** `let (a, b) = ...`. El producer se queda en el
`AudioEngine`, del lado de la interfaz, y el consumer se mueve al
callback de audio.

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

## Abrir la tarjeta de sonido

`AudioEngine::start` usa el crate [`cpal`](https://docs.rs/cpal), que
funciona en Linux (ALSA), macOS (CoreAudio) y Windows (WASAPI). Por eso
el plan de empezar por Linux no necesitará una reescritura para los otros
sistemas.

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
que funciona con el resto del ecosistema de Rust. La interfaz lo
convierte en texto para la barra de estado con `err.to_string()`.

## Genéricos: una función, muchos tipos de muestra

Distintas tarjetas de sonido quieren muestras en distintos formatos:
floats de 32 bits, enteros de 16 bits, etc. Generamos `f32` internamente
y convertimos en el último momento. En lugar de escribir el código del
stream seis veces, lo escribimos una vez como **función genérica**:

```rust
fn build_stream<T>(device: &cpal::Device, config: StreamConfig,
                   commands: rtrb::Consumer<Command>)
    -> Result<cpal::Stream, EngineError>
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
    SampleFormat::F32 => build_stream::<f32>(&device, config, consumer),
    SampleFormat::I16 => build_stream::<i16>(&device, config, consumer),
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
- El `consumer` se mueve a la rama que se ejecute. El compilador sabe
  que solo una rama se ejecuta, así que esto está permitido.

## El callback de audio es un closure

Dentro de `build_stream`:

```rust
let mut audio_thread = AudioThread::new(config.sample_rate, config.channels as usize, commands);

let data_callback = move |output: &mut [T], _: &cpal::OutputCallbackInfo| {
    audio_thread.process(output);
};

device.build_output_stream(config, data_callback, error_callback, None)
```

`|args| cuerpo` es un **closure**: una función anónima que puede usar
variables del lugar donde se escribió. La palabra clave `move` hace que
el closure tome la **propiedad** de `audio_thread` (y, con ella, del
consumer de comandos y del tono de prueba que contiene). Eso es
necesario porque el closure se ejecutará en otro hilo, mucho después de
que `build_stream` haya devuelto, así que no puede limitarse a tomar
prestadas variables locales.

La firma de cpal exige que el callback sea
`FnMut(&mut [T], &OutputCallbackInfo) + Send + 'static`:

| Límite | Significado |
|--------|-------------|
| `FnMut` | un closure que puede cambiar los valores que posee (nuestro oscilador avanza su fase) |
| `Send` | seguro de mover a otro hilo |
| `'static` | posee todo lo que usa, sin tomar prestado nada que pueda desaparecer |

Si intentáramos compartir algo inseguro, como una referencia simple a
los datos de la interfaz, el programa no compilaría. Así convierte Rust
la regla de oro del audio en comprobaciones que el compilador hace por
ti.

`&mut [T]` es un **slice mutable**: una vista prestada del búfer de la
tarjeta de sonido, que rellenamos pero no poseemos.

## `AudioThread::process`

```rust
fn process<T>(&mut self, output: &mut [T])
where
    T: SizedSample + FromSample<f32>,
{
    while let Ok(command) = self.commands.pop() {
        match command {
            Command::SetToneOn(on) => self.tone.set_on(on),
            Command::SetFrequency(hz) => self.tone.set_frequency(hz),
            Command::SetVolume(volume) => self.tone.set_volume(volume),
        }
    }
    for frame in output.chunks_mut(self.channels) {
        let sample = T::from_sample(self.tone.next_sample());
        frame.fill(sample);
    }
}
```

- `while let Ok(command) = ...` sigue en bucle mientras el patrón
  encaja, de modo que vacía todos los comandos en espera y se detiene
  cuando la cola está vacía.
- El `match` **desestructura** cada variante: en
  `Command::SetFrequency(hz)`, `hz` queda ligado al número que
  contiene.
- `chunks_mut(channels)` divide el búfer en frames, una muestra por
  canal, y `fill` escribe el mismo valor en todos los canales.

Aquí nada reserva memoria, bloquea ni imprime. La función vive en su
propia struct en lugar de dentro del closure, para que las pruebas al
final de `engine.rs` puedan llamarla con un `Vec` simple en lugar de una
tarjeta de sonido.

## Mantener vivo el stream: `Drop`

```rust
pub struct AudioEngine {
    _stream: cpal::Stream,
    commands: rtrb::Producer<Command>,
    info: EngineInfo,
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

## El tono de prueba

En este paso el tono de prueba vivía en
`crates/lineup-audio/src/synth.rs`. El paso 2 lo sustituyó por los
canales de instrumentos: su ganancia sin clics se convirtió en
[`dsp::Smoothed`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-audio/src/dsp.rs),
y su oscilador creció hasta convertirse en el
[`Synth`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-audio/src/instrument.rs)
que hoy reproduce las notas del channel rack. El código de abajo es el
tono de prueba tal como era.

`TestTone` generaba una onda senoidal con un **acumulador de fase**:
`phase` iba de 0.0 a 1.0 una vez por ciclo, y cada muestra lo avanzaba
en `frequency / sample_rate`. La muestra era `sin(phase × 2π)`.

```rust
pub fn next_sample(&mut self) -> f32 {
    let target = if self.on { self.volume } else { 0.0 };
    self.current_gain += (target - self.current_gain) * self.smoothing;

    let sample = (self.phase * TAU).sin() * self.current_gain;

    self.phase += self.frequency / self.sample_rate;
    if self.phase >= 1.0 {
        self.phase -= 1.0;
    }
    sample
}
```

Encender o apagar un sonido de golpe hace saltar el altavoz, y eso se
oye como un clic. Por eso la ganancia se desliza hacia su objetivo: en
cada muestra recorre una fracción pequeña (`smoothing`) de la distancia
restante. Es un *suavizador de un polo*, y con el valor que elegimos se
asienta en unos 5 ms, demasiado rápido para oírse como un fundido pero
suficiente para eliminar el clic.

Observa que `if` es una **expresión** en Rust:
`if self.on { a } else { b }` produce un valor, como el operador `?:` de
otros lenguajes.

`&mut self` significa que el método **toma prestado** el tono de forma
mutable: puede cambiar los campos, y mientras se ejecuta nadie más puede
leer o escribir el tono. Rust permite o bien un préstamo mutable *o
bien* cualquier número de préstamos compartidos (`&`) a la vez, nunca
ambos. Esta regla, comprobada por el **borrow checker**, es la que evita
las condiciones de carrera y muchos otros errores.
