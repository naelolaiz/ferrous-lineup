# Instrumentos: batería, sampler y Lineup Osc

Archivos: [`crates/lineup-audio/src/instrument.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-audio/src/instrument.rs),
[`crates/lineup-audio/src/drums.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-audio/src/drums.rs),
[`crates/lineup-audio/src/dsp.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-audio/src/dsp.rs),
[`crates/lineup-audio/src/sample.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-audio/src/sample.rs)
y [`crates/lineup-audio/src/timbre.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-audio/src/timbre.rs)

Cada canal tiene un instrumento: la parte que convierte las notas en
sonido. Este capítulo trata de los dos que Ferrous Lineup hace por sí
mismo, el **sampler**, que toca un sonido grabado o sintetizado, y el
**Lineup Osc**, un sintetizador pequeño. Los otros dos tipos tocan a
través de otra cosa: una SoundFont ([SoundFonts y General
MIDI](soundfonts.md)) o un sintetizador externo por MIDI
([MIDI](midi.md)). Cómo se eligen los sonidos en la
aplicación está en
[Sonidos](https://naelolaiz.github.io/ferrous-lineup/es/manual/sounds.html),
del manual.

## Un enum, cuatro instrumentos

```rust
pub enum Instrument {
    Sampler(Sampler),
    Synth(Synth),
    SoundFont(SoundFontPlayer),
    MidiOut(MidiOut),
}
```

Un diseño habitual para «muchas clases de una misma cosa» es un trait,
`trait Instrument`, con valores de tipo `Box<dyn Instrument>`. Es
abierto: cualquiera podría añadir una clase sin tocar este archivo. Pero
cada `Box::new` reserva memoria, y cada llamada pasa por un puntero a una
tabla de funciones. Con cuatro clases, todas conocidas, un enum es más
sencillo. El hilo de audio hace `match` sobre él, crear uno nunca
reserva memoria, y cuando se añade una quinta clase el compilador
enumera cada `match` que tiene que enterarse.

Cada método de `Instrument` es un `match` que pasa la llamada a la clase
de dentro:

```rust
pub fn release_all(&mut self) {
    match self {
        Instrument::Sampler(sampler) => sampler.release_all(),
        Instrument::Synth(synth) => synth.release_all(),
        Instrument::SoundFont(player) => player.release_all(),
        Instrument::MidiOut(midi_out) => midi_out.release_all(),
    }
}
```

Lo que se le puede pedir a un instrumento:

| Método | Qué hace |
|---|---|
| `note_on` | Empieza una nota: una altura, una velocidad y un **gate**, cuántos frames se sostiene |
| `note_on_from` | Lo mismo, para una nota de la canción, que recuerda de dónde viene (`Origin`, ver [El secuenciador](sequencer.md)) |
| `note_off` | Suelta una nota sostenida con el gate `HELD` |
| `release_all` | Suelta todas las notas sostenidas, cuando se detiene la reproducción |
| `release_unless` | Suelta las notas de la canción que la canción ya no toca |
| `tempo_changed` | Estira las notas sostenidas después de un cambio de tempo ([Tempo, compás, swing y sensación](tempo.md)) |
| `next_sample` | El siguiente frame de sonido: la suma de todas las voces |

Una nota de la canción sabe su longitud cuando empieza, así que recibe
un gate en frames y termina sola; el hilo de audio nunca tiene que
acordarse de pararla. Una tecla pulsada en un teclado MIDI no tiene una
longitud conocida, así que recibe `HELD`, `u32::MAX`, que nunca se acaba:
solo `note_off` la termina.

El instrumento de un canal lo construye `Instrument::for_channel`, en el
hilo de la interfaz, a partir del `Channel` del proyecto:

```rust
Ok(match &channel.kind {
    // A drum sent over MIDI is its note of a General MIDI drum set.
    ChannelKind::Sampler(SampleSource::Builtin(drum)) => match channel.sent_on {
        Some(number) => Instrument::MidiOut(
            MidiOut::new(number).playing_as(Some(general_midi_drum(*drum))),
        ),
        None => {
            Instrument::Sampler(Sampler::new(Arc::new(drums::render(*drum, sample_rate))))
        }
    },
    ChannelKind::Sampler(SampleSource::File(path)) => {
        Instrument::Sampler(Sampler::new(samples.get(path)?))
    }
    ChannelKind::Synth => Instrument::Synth(Synth::with_timbre(
        sample_rate,
        Timbre::for_program(channel.program),
    )),
    ChannelKind::MidiOut { channel: number } => {
        Instrument::MidiOut(MidiOut::new(*number).with_program(channel.program))
    }
})
```

Aquí se sintetizan los tambores y se leen los archivos (a través de la
[`SampleCache`](../program/sync.md)), un trabajo lento que puede
reservar memoria: justo lo que el hilo de audio no debe hacer. El
instrumento terminado se envía en un comando.

## Una batería hecha con unas líneas de matemáticas

Ferrous Lineup no trae archivos de sonido. Sus trece tambores se
calculan cuando se crea un canal de batería, cada uno en unas pocas
líneas:

| Tambor | Receta |
|---|---|
| Bombo | Una senoide cuya altura cae deprisa de 150 a 45 Hz, apagándose en medio segundo |
| Caja | Un tono corto de 185 Hz (el casco) más ruido con paso alto (la bordonera) |
| Palmas | Ruido con paso alto en tres ráfagas separadas 11 ms, como varias manos, y una cola |
| Charles | Ruido muy brillante, con paso alto a 7 kHz, que se va en una décima de segundo |
| Charles abierto, ride, crash | «Metal» (ver abajo) y ruido, que resuenan cada vez más |
| Aro | Un clic de 1750 Hz, un cuerpo de 480 Hz y un chasquido de ruido |
| Tom grave y agudo | Como el bombo, más agudos y redondos, asentándose en 98 y 165 Hz |
| Cencerro | Dos ondas cuadradas a 540 y 800 Hz, a un poco menos de una quinta |
| Shaker | Ruido brillante que crece durante 15 ms y luego se apaga deprisa |
| Conga | Un tono de 320 Hz que baja un poco, y el golpe de la mano |

Casi todos comparten una forma, el **decaimiento exponencial**:

```rust
fn decay(t: f32, tau: f32) -> f32 {
    (-t / tau).exp()
}
```

Vale 1 al principio y cae a un 37 % después de `tau` segundos, a un 13 %
después del doble, y así. Las cosas golpeadas se apagan así.

### Closures que recuerdan: `FnMut`

Todos los tambores se construyen con una sola función auxiliar:

```rust
fn generate(seconds: f32, sample_rate: u32, f: impl FnMut(f32) -> f32) -> Vec<f32> {
    let frames = (seconds * sample_rate as f32) as usize;
    (0..frames)
        .map(|i| i as f32 / sample_rate as f32)
        .map(f)
        .collect()
}
```

Recibe un closure `f` que va de un tiempo en segundos a una muestra. El
closure del bombo guarda una `phase` que avanza entre llamadas:

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
closure que cambia lo que captura implementa el trait **`FnMut`**; `Fn`
es para los closures que solo leen, y `FnOnce` para los que solo se
pueden llamar una vez. `impl FnMut(f32) -> f32` en la lista de
parámetros significa «cualquier closure con esa forma», y el compilador
hace una copia de `generate` para el closure de cada tambor, como hace
con los tipos genéricos.

¿Por qué ir sumando una fase en vez de calcular `sin(t × frecuencia ×
2π)`? Porque la frecuencia cambia con el tiempo. La fase es la suma de
todos los pasitos dados hasta ahora, así que la altura se desliza sin
saltos; la fórmula directa saltaría de un lado a otro al cambiar la
frecuencia.

### Metal con seis ondas cuadradas

Los platillos usan un truco de las cajas de ritmos analógicas de
principios de los 80: seis ondas cuadradas a frecuencias que no son
múltiplos unas de otras, sumadas.

```rust
const METAL: [f32; 6] = [205.3, 304.4, 369.6, 522.7, 540.0, 800.0];

fn metal(t: f32, ratio: f32) -> f32 {
    METAL.iter().map(|&f| square(t, f * ratio)).sum::<f32>() / METAL.len() as f32
}
```

Las notas cuyas frecuencias son múltiplos sencillos unas de otras se
funden en un acorde. Estas no, así que juntas chocan y brillan como
metal. Un filtro de paso alto se queda con la parte brillante, y la
envolvente decide si es un charles cerrado, uno abierto, un ride o un
crash.

Después, cada tambor se escala con `normalize` para que su muestra más
fuerte sea 0,9, y dos tests recorren `Drum::ALL`: cada tambor llega a ese
pico y dura más de mil frames (`every_drum_is_audible_and_in_range`), y
cada uno termina en silencio en vez de cortarse
(`every_drum_fades_out`).

## Ruido y filtros

`dsp.rs` guarda las piezas pequeñas. El **ruido blanco** sale de
xorshift, un generador de números aleatorios diminuto:

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

Tres desplazamientos y tres o exclusivos. No sirve para seguridad, pero
suena igual que el ruido, nunca reserva memoria, no necesita ningún
crate, y da los mismos tambores «aleatorios» cada vez, lo que permite
probarlos. Su única trampa es el cero: xorshift desde 0 se queda en 0
para siempre, así que `Noise::new` empieza desde `seed.max(1)`.

Los filtros son de **un polo**, los más sencillos que hay. El de paso bajo
mueve su salida una fracción del camino hacia la entrada en cada
muestra, así que los cambios rápidos (las frecuencias altas) se suavizan:

```rust
pub fn process(&mut self, input: f32) -> f32 {
    self.state += (input - self.state) * self.coefficient;
    self.state
}
```

Es la misma aritmética que `Smoothed`, el volumen que se desliza en vez
de saltar ([Dos hilos](../program/threads.md)). El de paso alto sigue
en cambio los *cambios* de su entrada, así que una señal constante se
apaga y los cambios rápidos pasan. Los tests dicen exactamente eso: una
entrada constante atraviesa el paso bajo y el paso alto la elimina.

## El sampler

```rust
pub struct Sampler {
    sample: Arc<SampleData>,
    /// Boxed so an `Instrument` stays small: commands carrying one are
    /// copied into the command queue, and a pointer is cheaper to copy
    /// than eight voices. The box is created here, on the UI thread.
    voices: Box<[SamplerVoice; SAMPLER_VOICES]>,
    notes_started: u64,
}
```

`Arc<T>` («atomically reference-counted») es un puntero que comparten
varios dueños. Clonarlo no copia los datos, suma uno a un contador, y los
datos se liberan cuando se va el último `Arc`. El contador se actualiza
de forma atómica, así que es seguro entre hilos, y eso es lo que permite
construir una muestra en el hilo de la interfaz y tocarla en el hilo de
audio. El mismo archivo de sonido en tres canales está una sola vez en
memoria.

Cada una de las ocho voces toca un golpe:

```rust
fn next_sample(&mut self, frames: &[f32]) -> f32 {
    let Some(position) = self.position else {
        return 0.0;
    };
    let index = position as usize;
    if index + 1 >= frames.len() {
        self.position = None;
        return frames.get(index).copied().unwrap_or(0.0) * self.gain;
    }
    // Linear interpolation between the two nearest frames.
    let fraction = (position - index as f64) as f32;
    let sample = frames[index] + (frames[index + 1] - frames[index]) * fraction;
    self.position = Some(position + self.increment);
    // ...
}
```

`let ... else` encaja un patrón, o ejecuta el bloque `else`, que tiene
que salir de la función. Mantiene el camino principal sin sangría: «si no
suena, silencio; si no, sigue con `position`».

`position` es una fracción de frame, porque un sonido rara vez recorre
sus frames exactamente a uno por frame de salida. Un WAV grabado a 44,1
kHz y tocado en una tarjeta de sonido de 48 kHz tiene que avanzar un
poco más despacio, o sonaría más agudo y más corto. Y un sampler toca
alturas tocando más deprisa o más despacio: una octava arriba es el
doble de rápido, y cada semitono multiplica la velocidad por la raíz
duodécima de dos, la misma regla que usa `Note::frequency` ([Clases de
altura, notas y su escritura](../theory/pitch.md)):

```rust
// Every semitone up plays the sample 2^(1/12) times faster.
let semitones = pitch.midi() as f64 - STEP_NOTE.midi() as f64;
let pitch_ratio = 2.0_f64.powf(semitones / 12.0);
let rate_ratio = self.sample.sample_rate() as f64 / output_rate as f64;
```

El `increment` de la voz es el producto de las dos razones. En
`STEP_NOTE`, Do4, la nota que toca un botón de paso, la razón de altura
es 1 y el sonido se toca como se grabó. Entre dos frames guardados la voz
traza una recta, **interpolación lineal**: no es lo mejor que puede hacer
un sampler, pero es barata y basta para la batería.

La velocidad escala el volumen, `velocity / 127`. Un sampler ignora el
gate: como una caja de ritmos, toca su sonido entero cada vez, y como
tiene voces, un golpe nuevo no corta el anterior.

Eso está bien para la batería y mal para una cosa: una grabación larga,
como una canción entera importada como audio, seguiría sonando después
de Detener. Así que `release_all` apaga en unos 15 ms las voces de un
sonido de más de diez segundos, y deja resonar los golpes cortos:

```rust
pub fn release_all(&mut self) {
    let long = self.sample.frames().len() as u64
        > u64::from(self.sample.sample_rate()) * LONG_SAMPLE_SECONDS;
    if long {
        for voice in self.voices.iter_mut().filter(|voice| !voice.is_idle()) {
            voice.fading.get_or_insert(1.0);
        }
    }
}
```

`release_unless`, para un clip que se quita o una pista que se silencia
mientras suena, apaga los sonidos de la canción sea cual sea su
longitud.

### Cargar un archivo WAV

`SampleData::load_wav` lee archivos WAV con el crate
[`hound`](https://docs.rs/hound). Un WAV guarda las muestras como
números de coma flotante o como enteros de 8 a 32 bits, y los dos se
convierten en flotantes entre -1 y 1:

```rust
let interleaved: Vec<f32> = match spec.sample_format {
    hound::SampleFormat::Float => reader
        .samples::<f32>()
        .collect::<Result<_, _>>()
        .map_err(SampleError::Wav)?,
    hound::SampleFormat::Int => {
        let full_scale = (1_i64 << (spec.bits_per_sample - 1)) as f32;
        reader
            .samples::<i32>()
            .map(|sample| sample.map(|s| s as f32 / full_scale))
            .collect::<Result<_, _>>()
            .map_err(SampleError::Wav)?
    }
};
```

`samples()` da un `Result` por muestra, porque un archivo puede estar
cortado a la mitad. Recoger un iterador de `Result` en un `Result<Vec<_>,
_>` da `Ok` con todos los valores si todos eran `Ok`, o el primer `Err`,
y se detiene ahí. Los `_` dejan que el compilador complete los tipos.
Después, `interleaved.chunks(channels)` toma un frame cada vez
(izquierdo, derecho) y lo promedia, así que un archivo estéreo queda en
mono.

`SampleError` implementa `Display` con mensajes traducidos, así que «no
se pudo leer el archivo WAV» le llega al usuario en su idioma. El test
`loads_and_mixes_to_mono` escribe un WAV estéreo real de 16 bits en la
carpeta temporal del sistema con `hound::WavWriter` y lo vuelve a leer.

## Voces

Una **voz** es lo necesario para hacer sonar una nota: su propio punto
en el sonido, su propio oscilador, su propia envolvente. El Lineup Osc
tiene 16 voces y un sampler 8. Cada nota nueva toma una:

```rust
trait Voice {
    /// Whether the voice is silent and free to play a new note.
    fn is_idle(&self) -> bool;
    /// When the voice's note started, as a count of notes started so far.
    fn started(&self) -> u64;
    /// Whether the voice's note has ended and it is only fading out.
    fn is_releasing(&self) -> bool;
}

fn pick_voice<V: Voice>(voices: &mut [V]) -> &mut V {
    let oldest = |releasing: bool| {
        voices
            .iter()
            .enumerate()
            .filter(|(_, voice)| !releasing || voice.is_releasing())
            .min_by_key(|(_, voice)| voice.started())
            .map(|(index, _)| index)
    };
    let index = voices
        .iter()
        .position(Voice::is_idle)
        .or_else(|| oldest(true))
        .or_else(|| oldest(false))
        .unwrap_or(0);
    &mut voices[index]
}
```

El orden es: una voz en silencio si la hay; si no, la voz más antigua de
las que solo se están apagando; si no, la más antigua de todas. Tomar una
voz ocupada es **robar una voz**, y todo sintetizador de hardware lo hace
cuando se sostienen más notas de las voces que tiene. Una voz que se
apaga va primero porque cortar una cola se nota mucho menos que cortar
una nota todavía sostenida, como un bajo largo debajo de acordes rápidos
(el test `voices_that_are_fading_out_are_stolen_first` toca justo eso).

`Voice` es un trait privado: `SamplerVoice` y `SynthVoice` son structs
distintos, los dos lo implementan, y una sola función genérica sirve
para ambos. `V: Voice` es la restricción de trait; la función solo puede
llamar a lo que el trait promete.

¿Por qué buscar primero un índice y solo después devolver `&mut
voices[index]`? Las búsquedas toman prestado el slice para leerlo
(`iter()`). Una referencia mutable no puede salir de una búsqueda de
solo lectura, así que el código encuentra la posición, deja que terminen
los préstamos de lectura, y toma prestado de forma mutable una vez, al
final.

`started` sale de un contador que sube con cada nota, así que «la más
antigua» es «la menor». Un `u64` no se desborda: a mil notas por segundo
tardaría 584 millones de años.

### Un número fijo de voces, en un `Box`

```rust
voices: Box::new(std::array::from_fn(|_| {
    SynthVoice::new(sample_rate, timbre.cutoff)
})),
```

Las voces son un array de tamaño fijo, así que el hilo de audio nunca lo
hace crecer y nunca reserva memoria. `std::array::from_fn` construye un
array llamando al closure una vez por elemento; el closure recibe el
índice, que aquí no hace falta, de ahí `|_|`. Las voces del sampler
implementan `Default`, así que las construye `Box::default()`.

El array va en un `Box` por la cola de comandos hacia el hilo de audio.
Cada hueco de la cola es tan grande como el `Command` más grande, y un
comando que inserta un canal lleva un `Instrument`. Dieciséis voces
pasan del kilobyte; un `Box` es un puntero de 8 bytes. Un test fija el
tamaño:

```rust
let size = std::mem::size_of::<Command>();
assert!(size <= 80, "a command takes {size} bytes");
```

Meterlas en un `Box` significa que las voces viven en el montón, y
liberarlas es liberar memoria. Por eso un instrumento con el que el hilo
de audio ha terminado vuelve siempre al hilo de la interfaz por la cola
de basura ([Dos hilos](../program/threads.md)): el box se crea y se
libera en el hilo de la interfaz, y el hilo de audio solo mueve el
puntero.

## El Lineup Osc

El Lineup Osc toca un oscilador por voz, a través de un filtro de paso
bajo suave, moldeado por una **envolvente** de volumen:

```rust
pub struct Synth {
    sample_rate: f32,
    voices: Box<[SynthVoice; SYNTH_VOICES]>,
    /// Boxed, like the voices, so an `Instrument` stays small.
    envelope: Box<EnvelopeRates>,
    notes_started: u64,
}
```

### La onda

Un oscilador guarda una **fase**, de 0 a 1 a lo largo de un ciclo, y la
onda dice cómo suena cada fase:

```rust
pub fn at(self, phase: f32) -> f32 {
    match self {
        // Ramps from -1 to 1.
        Wave::Saw => phase * 2.0 - 1.0,
        // 1 for the first half of the cycle, -1 for the second.
        Wave::Square => {
            if phase < 0.5 {
                1.0
            } else {
                -1.0
            }
        }
        // Up from -1 to 1 and back down.
        Wave::Triangle => 1.0 - 4.0 * (phase - 0.5).abs(),
        Wave::Sine => (phase * std::f32::consts::TAU).sin(),
    }
}
```

En cada frame la fase avanza `frequency / sample_rate` y vuelve a empezar
con `fract()`. Una diente de sierra tiene todos los armónicos, así que
es brillante y zumbona; una cuadrada solo los impares, que suena hueco;
una triangular los tiene débiles y suena suave; una senoide no tiene
ninguno y suena pura.

Una diente de sierra hecha así contiene frecuencias por encima de lo que
puede representar la frecuencia de muestreo, y vuelven plegadas como
tonos metálicos débiles, lo que se llama **aliasing**. El filtro de paso
bajo esconde la mayor parte. Quitarlo de verdad (con correcciones
«PolyBLEP», o con tablas de onda) sería un buen proyecto.

### La envolvente: una máquina de estados

Una nota que solo se enciende y se apaga hace clic, y no se parece a
nada real. Una envolvente moldea su volumen a lo largo del tiempo. La
clásica tiene cuatro partes, **ADSR**:

```text
 volumen
   1 ┤  /\
     │ /  \___________          A: ataque, subiendo al volumen completo
     │/    D    S      \        D: decaimiento, bajando hasta...
     │                  \       S: sostenido, mientras la nota se sostiene
   0 ┼───────────────────\___   R: liberación, apagándose al terminar la nota
       A    fin del gate ^
```

Cada voz está en una **etapa**, y cada muestra puede moverla a la
siguiente. El código con esta forma es una **máquina de estados**:

```rust
match self.stage {
    Stage::Idle => return 0.0,
    Stage::Attack => {
        self.level += envelope.attack_step;
        if self.level >= 1.0 {
            self.level = 1.0;
            self.stage = Stage::Decay;
        }
    }
    Stage::Decay => {
        let sustain = envelope.sustain;
        self.level = sustain + (self.level - sustain) * envelope.decay_factor;
    }
    Stage::Release => {
        self.level *= envelope.release_factor;
        if self.level < SILENT {
            self.stage = Stage::Idle;
            return 0.0;
        }
    }
}

// Count down the gate while the note is held, then let go. A
// `HELD` gate is not counted: only `note_off` ends it.
if self.is_held() && self.gate != HELD {
    match self.gate.checked_sub(1) {
        Some(left) => self.gate = left,
        None => self.stage = Stage::Release,
    }
}
```

- El ataque sube en línea recta; el decaimiento y la liberación son
  **exponenciales**: cada frame conserva una fracción fija de la
  distancia que queda, como se apagan las cuerdas y las campanas. Las
  fracciones se calculan una vez por instrumento en
  `EnvelopeRates::new`, no en cada muestra: `decay_factor` es `e^(-1 /
  (segundos × frecuencia de muestreo))`.
- `checked_sub` devuelve `None` en vez de bajar de cero, así que se lee
  como «si no queda gate, suelta».
- No hay etapa `Sustain`. El decaimiento se acerca al nivel de sostenido
  y se queda ahí, así que una etapa propia no haría nada.
- `SILENT` es 0,0001, unos -80 dB. Un apagado exponencial nunca llega a
  cero, así que por debajo de ese valor la voz se da por callada y se
  libera.

`note_off` suelta las voces de una altura solo si se empezaron con el
gate `HELD`. Una tecla que se suelta en un teclado MIDI no debe terminar
la misma nota que la canción toca con su propio gate. `release_all` pasa
todas las voces sostenidas a `Release`, que es como Detener deja que las
notas largas se apaguen en vez de seguir sonando.

Una voz robada no se reinicia: `start` pone la altura, la ganancia y el
gate nuevos, y deja el nivel, la fase y el filtro como estaban.

```rust
voice.stage = Stage::Attack;
voice.started = self.notes_started;
voice.origin = origin;
// The level, phase and filter carry on from where they were, so a
// stolen voice does not jump (and click).
```

El ataque sube entonces desde donde estuviera el nivel. Cada voz se
baja además a `SYNTH_VOICE_GAIN`, 0,35, para que un acorde de cuatro
notas no salga cuatro veces más fuerte.

### Timbres: sonar como un instrumento General MIDI

El Lineup Osc nunca podrá sonar como un violín de verdad, pero puede
sonar más a violín que a piano. Un canal con un programa General MIDI y
sin SoundFont toca un **timbre** elegido según la familia del programa:

```rust
pub struct Timbre {
    pub wave: Wave,
    /// Seconds to reach full volume.
    pub attack: f32,
    /// How quickly the volume falls to the sustain level (time constant,
    /// in seconds).
    pub decay: f32,
    /// The volume a held note settles at, 0 to 1. 0 for a sound that dies
    /// away by itself, like a piano or a plucked string.
    pub sustain: f32,
    /// How quickly a note fades after it is let go (time constant).
    pub release: f32,
    /// The low-pass filter's cutoff, in Hz: lower is darker.
    pub cutoff: f32,
    /// How loud, compared with the Lineup Osc's own sound.
    pub gain: f32,
}
```

Dos pequeñas `const fn` dan nombre a las dos formas en que se comportan
los instrumentos:

```rust
/// Struck or plucked: starts at once and dies away over `decay`.
const fn struck(wave: Wave, decay: f32, release: f32) -> Timbre {
    Timbre::new(wave, 0.003, decay, 0.0, release)
}

/// Bowed, blown or sung: swells in over `attack` and keeps going.
const fn held(wave: Wave, attack: f32, release: f32) -> Timbre {
    Timbre::new(wave, attack, 0.3, 0.85, release)
}
```

y `for_program` es un solo `match` sobre el número de programa, en el
orden del propio General MIDI, donde cada familia es un rango de ocho
números:

```rust
let timbre = match program.min(127) {
    // Pianos.
    0..=3 => Timbre::struck(Wave::Saw, 0.9, 0.25).cutoff(1_800.0),
    // ...
    // Brass.
    56..=63 => Timbre::held(Wave::Saw, 0.04, 0.1).cutoff(2_000.0),
    // Reeds: saxophones, oboe, bassoon, clarinet.
    64..=71 => Timbre::held(Wave::Square, 0.03, 0.08).cutoff(1_800.0),
    // Pipes: flutes, recorder, ocarina.
    72..=79 => Timbre::held(Wave::Triangle, 0.05, 0.1).cutoff(3_000.0),
    // ...
    // Sound effects have no tune to play; the Lineup Osc stands in.
    _ => Timbre::LINEUP_OSC,
};
```

| Familia | Onda | Envolvente | Suena |
|---|---|---|---|
| Piano, guitarra, arpa, pizzicato | diente de sierra o triangular | empieza de golpe, se apaga aunque se sostenga | golpeado o pulsado |
| Cuerdas, metales | diente de sierra | entra creciendo, se mantiene | frotado o soplado |
| Órgano, acordeón, lengüetas | cuadrada | entera mientras se sostiene | hueco, de lengüeta |
| Flautas, voces | triangular | ataque suave, se mantiene | suave y redondo |
| Campanas, vibráfono, marimba | senoide | se apaga | puro |

Los patrones de rango (`0..=3`), las alternativas (`24..=28 | 31`) y un
`_` final hacen de toda la tabla una sola expresión, y el compilador
comprueba que cada número de 0 a 255 cae en algún sitio. Al final, la
ganancia de cada timbre se multiplica por la de su onda, para que una
cuadrada, la onda más fuerte, no destaque en la mezcla.
`Timbre::for_program(None)` es `LINEUP_OSC`, la diente de sierra
brillante propia del sintetizador, así que un canal sin programa suena
como siempre. Qué programa recibe cada canal está en [SoundFonts y
General MIDI](soundfonts.md).

## Cómo se prueba

Los tests tocan un instrumento en un búfer y miran los números, ya que
en un test nadie puede escuchar:

- `synth_plays_the_right_pitch` toca La4 durante un segundo y cuenta
  cuántas veces cambia de signo la salida: unas 880, dos por cada ciclo
  de 440 Hz.
- `synth_stops_after_the_gate_and_release` sostiene una nota una décima
  de segundo y comprueba que la salida es exactamente cero diez tiempos
  de liberación después, sin ninguna voz sonando.
- `synth_plays_chords`, `voices_are_stolen_when_all_are_busy` y
  `voices_that_are_fading_out_are_stolen_first` comprueban las voces.
- `sampler_resamples_to_the_output_rate` toca un segundo grabado a 24
  kHz en una salida de 48 kHz y espera 48 000 frames de sonido, con un
  margen de diez; `sampler_plays_higher_notes_faster` espera la mitad una
  octava arriba; `sampler_hits_can_overlap` toca dos golpes a la vez.
- `held_notes_last_until_note_off` comprueba el gate `HELD`.
- En `timbre.rs`, `plucked_instruments_die_away_and_bowed_ones_keep_going`
  comprueba las familias, y `every_wave_stays_between_minus_one_and_one`
  las ondas.
