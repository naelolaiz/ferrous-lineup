# Instrumentos: batería, sampler y sintetizador

Archivos:
[`drums.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-audio/src/drums.rs),
[`dsp.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-audio/src/dsp.rs),
[`sample.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-audio/src/sample.rs)
y
[`instrument.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-audio/src/instrument.rs)
en `crates/lineup-audio/src/`

## Sintetizar una batería

Ferrous Lineup no distribuye archivos de sonido: su batería se calcula
cuando se crea un canal de batería. Cada tambor son unas pocas líneas de
matemáticas.

| Tambor | Receta |
|--------|--------|
| Bombo | Una onda senoidal cuya altura cae rápido de 150 Hz a 45 Hz, desvaneciéndose en medio segundo |
| Caja | Un tono corto de 185 Hz (el casco) más ruido filtrado en paso alto (los bordones) |
| Palmas | Ruido filtrado en paso alto en tres ráfagas rápidas separadas 11 ms, como varias manos, más una cola corta |
| Hi-hat | Ruido muy brillante (filtrado en paso alto a 7 kHz) que se apaga en unos 0.1 s |

La mayoría comparten una forma, la **caída exponencial**:

```rust
fn decay(t: f32, tau: f32) -> f32 {
    (-t / tau).exp()
}
```

Vale 1.0 al principio y cae a un 37% al cabo de `tau` segundos, un 13%
tras `2 × tau`, y así. Los sonidos naturales se desvanecen así.

### Closures que recuerdan: `FnMut`

Todos los tambores se construyen con un solo ayudante:

```rust
fn generate(seconds: f32, sample_rate: u32, f: impl FnMut(f32) -> f32) -> Vec<f32> {
    let frames = (seconds * sample_rate as f32) as usize;
    (0..frames)
        .map(|i| i as f32 / sample_rate as f32)
        .map(f)
        .collect()
}
```

Recibe un closure `f` que convierte un tiempo (en segundos) en una
muestra. El closure del bombo mantiene una `phase` en curso entre
llamadas:

```rust
fn kick(sample_rate: u32) -> Vec<f32> {
    let mut phase = 0.0;
    generate(0.5, sample_rate, |t| {
        let frequency = 45.0 + 105.0 * decay(t, 0.04);
        phase += frequency / sample_rate as f32;
        (phase * TAU).sin() * decay(t, 0.15)
    })
}
```

El closure *cambia* `phase`, una variable que capturó de `kick`. Un
closure que muta lo que captura implementa el trait **`FnMut`** (`Fn` es
para closures que solo leen, `FnOnce` para los que solo se pueden llamar
una vez). `impl FnMut(f32) -> f32` en la lista de parámetros significa
«cualquier closure con esa forma». El compilador genera un `generate`
especializado para el closure de cada tambor, igual que hace con los
tipos genéricos.

¿Por qué acumular una fase en lugar de calcular directamente
`sin(t × frequency × 2π)`? Porque la frecuencia cambia con el tiempo. La
fase es la suma de todos los pasos pequeños dados hasta ahora, lo que da
un barrido de altura suave; la fórmula directa daría saltos.

### Ruido blanco sin crate

```rust
pub fn next_sample(&mut self) -> f32 {
    let mut x = self.state;
    x ^= x << 13;
    x ^= x >> 17;
    x ^= x << 5;
    self.state = x;
    (x as f32 / u32::MAX as f32) * 2.0 - 1.0
}
```

Esto es **xorshift**, un generador de números aleatorios diminuto: tres
desplazamientos (`<<`, `>>`) y o-exclusivas (`^`). No es seguro, pero
suena igual que el ruido, nunca reserva memoria y da los mismos tambores
«aleatorios» cada vez, lo que los hace fáciles de probar.

### Filtros

`dsp.rs` tiene dos **filtros de un polo**, los filtros más simples que
existen. Un **paso bajo** conserva las frecuencias bajas moviendo su
salida una fracción del camino hacia la entrada en cada muestra
(exactamente las mismas matemáticas que el volumen `Smoothed`). Un
**paso alto** conserva las frecuencias altas siguiendo los *cambios* de
la entrada. Sus pruebas muestran el comportamiento básico: una entrada
constante pasa el paso bajo y el paso alto la elimina.

## El sampler

```rust
pub struct Sampler {
    sample: Arc<SampleData>,
    position: Option<f64>,
    increment: f64,
}
```

### `Arc`: propiedad compartida

`Arc<T>` (*atomically reference-counted*, con conteo de referencias
atómico) es un puntero que varios dueños pueden compartir. Clonar un
`Arc` no copia los datos; suma uno a un contador. Cuando se libera el
último `Arc`, se liberan los datos. La parte «atómica» significa que el
contador se puede actualizar con seguridad desde varios hilos, que es lo
que permite a una muestra cruzar del hilo de la interfaz al hilo de
audio.

### Reproducir a otra frecuencia de muestreo

Un archivo WAV grabado a 44.1 kHz y reproducido en una tarjeta de sonido
de 48 kHz debe avanzar por el archivo un poco más lento que un frame por
frame, o sonaría más agudo y más corto. `increment` es la razón entre
las dos frecuencias, y `position` es fraccionaria. Entre dos frames
guardados, el sampler dibuja una línea recta (**interpolación lineal**):

```rust
let fraction = (position - index as f64) as f32;
let sample = frames[index] + (frames[index + 1] - frames[index]) * fraction;
```

### `let ... else`

```rust
let Some(position) = self.position else {
    return 0.0;
};
```

`let`-`else` desestructura un patrón o, si no encaja, ejecuta el bloque
`else`, que debe abandonar la función (con `return`, `break`, etc.).
Mantiene el camino principal de la función sin indentar: «si no estamos
reproduciendo, saca silencio; si no, sigue con `position`».

## Cargar archivos WAV

`SampleData::load_wav` usa el crate [`hound`](https://docs.rs/hound).
Los archivos WAV guardan las muestras como floats o como enteros de 8,
16, 24 o 32 bits; ambas se convierten en floats entre -1.0 y 1.0, y los
archivos estéreo se mezclan a mono promediando los canales de cada
frame.

```rust
reader
    .samples::<i32>()
    .map(|sample| sample.map(|s| s as f32 / full_scale))
    .collect::<Result<_, _>>()
    .map_err(SampleError::Wav)?
```

`samples()` produce un `Result` por muestra, porque el archivo podría
estar cortado a la mitad. Recoger un iterador de `Result`s en
`Result<Vec<_>, _>` da `Ok(vec)` si todos los elementos eran `Ok`, o el
primer `Err`. Los marcadores `_` dejan que el compilador rellene los
tipos.

La prueba escribe un archivo WAV real en la carpeta temporal del sistema
con `hound::WavWriter`, lo vuelve a cargar y comprueba la mezcla mono.

## El Lineup Osc

El canal de sintetizador genera una onda **diente de sierra**, una rampa
de -1 a 1 una vez por ciclo, que es brillante y zumbante porque es rica
en armónicos. Un filtro paso bajo a 2.5 kHz la suaviza, y una
**envolvente** moldea su volumen como una pulsación: una entrada de 3 ms
(el *ataque*, para evitar un clic) seguida de una caída exponencial (la
*caída*).

De momento toca Do4 en cada paso. En el paso 3 el piano roll dará a cada
paso su propia nota.

> Una sierra simple calculada así contiene frecuencias por encima de lo
> que la frecuencia de muestreo puede representar, que se pliegan como
> tenues tonos metálicos (*aliasing*). Arreglarlo bien (con «PolyBLEP» o
> wavetables) es un buen ejercicio posterior.

## Un enum en lugar de un trait object

```rust
pub enum Instrument {
    Sampler(Sampler),
    Synth(Synth),
}
```

Otro diseño habitual es un trait `Instrument` con valores
`Box<dyn Instrument>`. Es más abierto, ya que cualquiera podría añadir
un tipo nuevo, pero cada `Box::new` reserva memoria y las llamadas pasan
por un puntero a una tabla de funciones. Con solo dos tipos, un enum es
más simple: el hilo de audio hace `match` sobre él, crear uno nunca
reserva memoria y el compilador comprueba que todos los tipos están
manejados. Si Ferrous Lineup algún día carga plugins, el diseño con
trait volverá.
