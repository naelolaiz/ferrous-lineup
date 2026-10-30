# El mezclador: niveles, balance, reverberación, efectos y orden de partitura

Archivos: [`crates/lineup-core/src/mixer.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/mixer.rs),
[`crates/lineup-core/src/effects.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/effects.rs),
[`crates/lineup-core/src/score_order.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/score_order.rs),
[`crates/lineup-audio/src/reverb.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-audio/src/reverb.rs),
[`crates/lineup-audio/src/effects.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-audio/src/effects.rs),
[`crates/lineup-audio/src/levels.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-audio/src/levels.rs),
[`crates/lineup-app/src/app/mixer.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/mixer.rs)
y [`crates/lineup-app/src/meters.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/meters.rs)

El mezclador (**F9**) es donde se equilibra la banda. Cada canal tiene
una línea: su instrumento, un medidor de nivel, su volumen, dónde se
ubica entre los altavoces (**Balance**), cuánto de él va a la sala
(**Reverberación**), sus propios **Efectos** (un ecualizador, un
compresor y un delay), **Silenciar** y **Solo**. Encima están el volumen
**Principal**, con un medidor para cada altavoz, y la reverberación que
comparten todos los canales. Las líneas aparecen como una partitura de
director presenta los instrumentos, por secciones.

![El mezclador tocando El Choclo con la reverberación activada: los Teclados con balance I30, la Melodía (un acordeón) en D30 y la batería enviando un 40 % a la reverberación.](../images/step-9-mixer.png)

El trabajo se reparte como en todo el programa. `lineup-core` guarda los
ajustes, que se guardan con la canción; `lineup-audio` los convierte en
sonido en el hilo de audio; `lineup-app` dibuja la ventana. Cómo usar el
mezclador está en el capítulo
[El mezclador](https://naelolaiz.github.io/ferrous-lineup/es/manual/mixer.html)
del manual.

## Qué se guarda

El proyecto tiene un `Mixer`:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Mixer {
    pub master: f32,
    pub reverb: ReverbSettings,
    pub solo: Option<usize>,
}
```

```rust
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ReverbSettings {
    pub enabled: bool,
    /// Wet fraction, from 0 to 1.
    pub mix: f32,
    /// Approximate time to decay by 60 dB, in seconds.
    pub decay: f32,
    /// High frequency absorption, from 0 to 1.
    pub damping: f32,
    pub pre_delay_ms: f32,
}
```

y cada `Channel` tiene su propia mezcla: `volume` y `muted`, y tres
campos más:

```rust
#[serde(default, skip_serializing_if = "is_centred")]
pub pan: f32,
#[serde(default = "full_send", skip_serializing_if = "is_full_send")]
pub reverb_send: f32,
#[serde(default, skip_serializing_if = "Effects::is_plain")]
pub effects: Effects,
```

`skip_serializing_if` deja un campo fuera del archivo mientras tiene el
valor de un canal nuevo (en el centro, todo enviado a la reverberación,
sin efectos), y `default` lo vuelve a poner cuando falta. Una canción
cuyos canales son todos así se escribe exactamente igual que con un
Ferrous Lineup sin balances ni efectos, así que una versión más antigua
la sigue abriendo. Solo una canción que los usa necesita el formato 26
(balance y envío) o el 28 (efectos) ([El número de
versión](../program/saving.md#el-número-de-versión)). Al abrir se
comprueban los rangos: un archivo editado a mano con un balance de −3, o
un delay cuyos ecos no se apagarían nunca, se rechaza con un mensaje en
lugar de tocar un canal más fuerte que el máximo.

El **Solo** es un número de canal, no una marca en cada canal. El
proyecto responde «¿se oye este canal?» en un solo sitio, y así todo lo
que suena (el hilo de audio, una exportación, un archivo MIDI) está de
acuerdo:

```rust
pub fn channel_muted(&self, channel: usize) -> bool {
    self.channels.get(channel).is_none_or(|state| state.muted)
        || self.mixer.solo.is_some_and(|solo| solo != channel)
}
```

Por eso poner un canal en solo llega al hilo de audio como un cambio de
silencio en todos los demás ([`diff`: la lista de
cambios](../program/sync.md#diff-la-lista-de-cambios)).

**Restablecer mezcla** devuelve el principal, la reverberación, el solo y
el volumen, el balance, el envío y los efectos de cada canal a como
están en una canción nueva. Los músicos de la banda recuperan sus
volúmenes de siempre, y los canales que la banda deja callados siguen
silenciados (`Project::reset_mix`, en `band.rs`).

## El camino de una muestra

En cada frame, el hilo de audio lleva cada canal por los mismos pasos.
Este es el corazón de la mezcla, en `engine.rs`:

```rust
let wet = self.reverb.wet();
let mut dry = [0.0; 2];
let mut send = 0.0;
for channel in &mut self.channels {
    let volume = channel.volume.next_value();
    let sample = channel.effects.process(channel.instrument.next_sample()) * volume;
    channel.peak = channel.peak.max(sample.abs());
    let amount = channel.reverb_send.next_value();
    let gains = lineup_core::Channel::pan_gains(channel.pan.next_value());
    let kept = sample * (1.0 - wet * amount);
    dry[0] += kept * gains[0];
    dry[1] += kept * gains[1];
    send += sample * amount;
}
// ...
let click = self.click.next_sample();
let stereo = self
    .reverb
    .next(dry, send, wet)
    .map(|sample| sample + click);
let volume = self.master_volume.next_value();
let stereo = stereo.map(|sample| sample * volume);
master_peak = [0, 1].map(|side| master_peak[side].max(stereo[side].abs()));
// `tanh` gently rounds off peaks above 1.0 instead of clipping them hard.
let stereo = stereo.map(f32::tanh);
```

Por orden:

1. El instrumento hace una muestra, mono.
2. Los **efectos** del canal le dan forma: ecualizador, compresor,
   delay.
3. Su **volumen** la escala, y su **pico** se guarda para el medidor.
4. Su **balance** la reparte entre los dos altavoces, y su **envío**
   manda una parte a la reverberación.
5. La **reverberación** añade la sala a la suma de todos los canales.
6. El clic del metrónomo se suma después de la reverberación: la sala es
   de la música, y un clic con cola es más difícil de seguir.
7. El **volumen principal** escala el conjunto, sus picos van a los
   medidores del principal, y `tanh` redondea lo que pase del máximo
   ([Mezclar un frame](../program/threads.md#mezclar-un-frame)).

El volumen, el balance, el envío y el nivel húmedo de la reverberación
son todos `Smoothed`: al moverlos en la ventana, se deslizan hasta el
nuevo valor en unos milisegundos en lugar de saltar, porque un salto en
mitad de una onda se oye como un clic. Los efectos van antes que el
volumen, como en el canal de una mesa de mezclas, así que un compresor
oye el instrumento tal como toca, diga lo que diga el fader.

## El balance: dos altavoces, un canal

El balance es un número de −1 (todo a la izquierda) a 1 (todo a la
derecha), con 0 en el centro. Se convierte en una ganancia para cada
altavoz:

```rust
/// How loud it is in the left and the right speaker for its pan: both
/// full in the middle, and the far side fading out as it moves away.
pub fn pan_gains(pan: f32) -> [f32; 2] {
    [(1.0 - pan).min(1.0), (1.0 + pan).min(1.0)]
}
```

```text
balance  −1.0  −0.5   0.0   0.5   1.0
izq.      1.0   1.0   1.0   0.5   0.0
der.      0.0   0.5   1.0   1.0   1.0
```

En el centro los dos altavoces tocan el canal a todo volumen, como suena
un sonido mono en un equipo estéreo. Mover un canal hacia un lado baja
el **otro** altavoz, como la perilla de balance de un equipo de música.

Esta es una de varias **leyes de panorama**. Las mesas de mezclas suelen
usar en cambio una ley de *potencia constante*, en la que el centro toca
cada altavoz a alrededor de 0,71 (−3 dB), para que un canal suene igual
de fuerte esté donde esté. Con la ley que se usa aquí, un canal llevado
del todo a la izquierda suena un poco más bajo que en el centro, porque
solo lo toca un altavoz. La razón para mantenerla es que una canción en
la que nada tiene balance toca cada canal exactamente a su volumen en
los dos lados.

El deslizador escribe su valor como `I30`, `C` o `D100` en español (`L30`
y `R100` en inglés), y lee lo que escribes de la misma manera:
`pan_value` acepta `L30`, `R100`, `C`, un número simple de −100 a 100, y
los españoles `I30` (izquierda) y `D30` (derecha). Un doble clic lo
devuelve al centro. Un deslizador solo detecta arrastres, así que el
doble clic se lee del puntero y se compara con el rectángulo del
deslizador.

## La reverberación: una sala hecha de retardos

Una reverberación imita una sala. Un sonido rebota en las paredes, el
suelo y el techo y vuelve, más tarde y más suave, miles de veces, hasta
que los ecos se funden en una cola que se apaga en uno o dos segundos.
La reverberación del mezclador es una **reverberación de Schroeder**, el
diseño digital más antiguo (Manfred Schroeder, 1962), hecho con dos
tipos de retardo.

Un **filtro peine** es un retardo cuya salida vuelve a su entrada, un
poco más suave cada vez: un sonido vuelve cada 31 ms, por ejemplo, más
bajo en cada vuelta. Un solo peine suena como un aleteo metálico, pero
cuatro en paralelo, con longitudes que nunca coinciden, dan un patrón
denso de ecos. Un **filtro pasatodo** es un retardo que esparce cada eco
en un racimo sin cambiar el tono general; tres seguidos convierten los
ecos de los peines en una capa uniforme.

```rust
fn new(rate: u32, spread: usize) -> Self {
    let length =
        |samples| ((samples + spread) as f32 * rate as f32 / 48_000.0).round() as usize;
    Self {
        combs: [1493, 1601, 1747, 1867].map(|samples| Comb {
            delay: Delay::new(length(samples)),
            feedback: 0.0,
            filtered: 0.0,
        }),
        diffusers: [241, 503, 719].map(|samples| Delay::new(length(samples))),
    }
}
```

Las longitudes se cuentan en muestras a 48 kHz (1493 muestras son 31 ms)
y se escalan a la frecuencia real. Todas son números primos, así que
ningún par comparte un factor y sus ecos casi nunca caen en el mismo
instante, lo que sonaría como una nota que resuena. Cada lado de la sala
estéreo es un `Side` con sus propios peines y pasatodos; el derecho se
crea con `spread` 37, así que sus retardos son 37 muestras más largos y
los dos lados difieren, lo que el oído percibe como amplitud.

`[...].map(...)` sobre un array da un array de la misma longitud, así
que `combs` es un `[Comb; 4]` construido en una sola expresión, sin
`Vec`.

### Cuánto resuena la sala

El **Decaimiento** es el tiempo que tarda la cola en bajar 60 dB, a una
milésima de su nivel: la medida habitual de una sala, llamada RT60. La
realimentación de cada peine sale de ahí:

```rust
fn configure(&mut self, rate: u32, decay: f32) {
    for comb in &mut self.combs {
        comb.feedback =
            10.0_f32.powf(-3.0 * comb.delay.samples.len() as f32 / (rate as f32 * decay));
    }
}
```

En `decay` segundos el sonido da `rate × decay / longitud` vueltas a un
peine, y en cada vuelta se multiplica por `feedback`. Para que el total
sea 10⁻³ (−60 dB), cada vuelta multiplica por
10^(−3 × longitud / (rate × decay)). Los peines más largos dan menos
vueltas, así que reciben una realimentación más fuerte, y los cuatro se
apagan a la vez.

La **Amortiguación** imita el aire y las paredes blandas de una sala
real, que se tragan las frecuencias agudas antes que las graves. Dentro
de cada peine, el eco pasa por un filtro pasabajos de un polo antes de
volver a entrar:

```rust
fn next(&mut self, input: f32, damping: f32) -> f32 {
    let out = self.delay.read();
    self.filtered = flushed(out * (1.0 - damping) + self.filtered * damping);
    self.delay.write(input + self.filtered * self.feedback);
    out
}
```

`filtered` se mueve solo una parte del camino hacia cada muestra nueva,
lo que suaviza las oscilaciones rápidas: los agudos. Cada vuelta los
suaviza un poco más, así que la cola se oscurece mientras se apaga, como
en una sala de conciertos.

El **Predelay** es el hueco antes de que la sala responda, hasta 200 ms:
en una sala grande la primera reflexión llega más tarde que en una
habitación pequeña. Es un buffer circular más delante de los peines, de
una quinta parte de segundo.

### Nunca demasiado pequeña, nunca un estorbo

`flushed` convierte cualquier valor menor que 10⁻²⁰ en exactamente 0.
Una cola que se apaga se hace cada vez más pequeña sin llegar nunca a
cero, y los números tan diminutos (**denormales**) hacen que muchos
procesadores vayan decenas de veces más lentos. Una reverberación que
se deja sonar en silencio podría entonces pasarse del tiempo del hilo de
audio.

Toda la memoria de los retardos se reserva en `Reverb::new`, antes de
que arranque el stream; `configure` solo cambia números, así que mover un
deslizador nunca reserva memoria en el hilo de audio. Un ajuste fuera de
rango se ignora en lugar de aplicarse. Desactivada, el nivel húmedo baja
hasta cero y entonces `Reverb::next` devuelve el sonido seco enseguida,
sin hacer ningún trabajo. Activada de nuevo, empieza desde el silencio
(`clear`), no desde la cola que tenía cuando se apagó.

### El envío: cuánto de cada canal está en la sala

Cada canal tiene un **envío**, de 0 a 1: cuánto de él va a la sala. La
suma de todos los envíos es la única entrada de la reverberación, así que
veinte canales cuestan una reverberación, no veinte, como con un envío
en una mesa real. La reverberación devuelve solo la sala, que `next`
suma al sonido seco:

```rust
[
    dry[0] + left.next(delayed, damping) * wet,
    dry[1] + right.next(delayed, damping) * wet,
]
```

La línea que hay que mirar en la mezcla es `kept = sample * (1.0 - wet *
amount)`: lo que entra en la sala se quita del sonido seco, en
proporción al ajuste **Húmedo / seco**. Comprueba los dos extremos:

- **Envío 1** (todo, como en cada canal nuevo): `kept = sample × (1 −
  wet)`, y toda la muestra va a la sala. Es una mezcla húmedo / seco
  clásica de toda la canción.
- **Envío 0**: `kept = sample`, y nada va a la sala. El canal suena seco
  y cercano, como con la reverberación desactivada, mientras el resto
  de la banda sigue en la sala.

La sala es mono a la entrada y estéreo a la salida, así que un canal
llevado del todo a la izquierda sigue teniendo su reverberación en los
dos lados, como un guitarrista a la izquierda del escenario llena toda
la sala.

## Los efectos de cada canal

Una mesa de mezclas real tiene, en cada canal, unos cuantos efectos que
dan forma a un instrumento antes de que se una a los demás. Cada línea
del mezclador tiene tres, en el orden que tienen en la mayoría de las
mesas: un **ecualizador**, un **compresor** y un **delay**.

![El mezclador de una canción de Motown con vientos. El botón Efectos de la línea de los vientos dice «EQ, delay», y su menú está abierto: Ecualizador con Graves a -4,0 dB, Medios y Agudos a 0 dB; Compresor con Umbral -18,0 dB, Relación 1,0:1 y Ganancia 0,0 dB; Delay con Tiempo 1/8 con puntillo, Realimentación 35 % y Mezcla 30 %; y un botón Sin efectos.](../images/step-9-effects.png)

### El ecualizador: subir o bajar frecuencias

Todo sonido es una mezcla de frecuencias: la fundamental de una nota, y
sus armónicos por encima. Un **ecualizador** (EQ) sube o baja rangos de
ellas. No añade notas; cambia el color de las que hay. El EQ del
mezclador tiene tres bandas, cada una de −12 dB a +12 dB:

| Banda | Dónde | Forma | Qué vive allí |
|---|---|---|---|
| **Graves** | por debajo de 200 Hz | un estante | el peso de un bajo, un bombo, la mano izquierda de un piano |
| **Medios** | alrededor de 1 kHz | una campana, de una octava y media de ancho | el cuerpo de la mayoría de los instrumentos, y de las voces |
| **Agudos** | por encima de 4 kHz | un estante | el aire y el brillo: platillos, una voz con aire, un arco sobre una cuerda |

Un **estante** (*shelf*) sube o baja todo lo que pasa de su frecuencia en
la misma cantidad, como un escalón. Una **campana** sube o baja su
frecuencia más que nada, y cada vez menos a cada lado. Los decibelios
miden una proporción: +6 dB hace las ondas de una banda casi el doble de
altas, −6 dB casi la mitad, 0 dB la deja como está.

**Bajar** es el uso más común. Cuando dos instrumentos tocan en el mismo
registro, se tapan: los graves de un piano y el bajo se vuelven un solo
sonido turbio. Quitar los graves al piano deja sitio al bajo, y cuando el
bajo toca, el piano no suena más delgado.

En el hilo de audio cada banda es un **biquad**, un filtro de segundo
orden, la pieza básica de la mayoría de los ecualizadores. Cada muestra
de salida es una mezcla de las tres últimas muestras de entrada y las
dos últimas de salida:

```rust
let output = b0 * input + b1 * self.inputs[0] + b2 * self.inputs[1]
    - a1 * self.outputs[0]
    - a2 * self.outputs[1];
```

Los cinco números `b0`, `b1`, `b2`, `a1` y `a2` deciden qué hace el
filtro. `Biquad::set` los calcula para un estante o una campana, una
frecuencia y una ganancia, con las fórmulas del *Audio EQ Cookbook* de
Robert Bristow-Johnson, que usa casi todo programa de audio: una
pendiente de 1 para los estantes y un Q de 1 para la campana. Los tres
filtros van uno detrás de otro, graves, medios, agudos, y sus números
solo se recalculan cuando cambian los ajustes, nunca en cada muestra.
`set` conserva las muestras que el filtro recuerda, así que mover una
banda mientras suena no hace clic. El mismo suelo de 10⁻²⁰ que en la
reverberación mantiene fuera los denormales.

### El compresor: igualar lo fuerte que suena una parte

Algunas partes salen desiguales: una línea de bajo en la que unas notas
saltan y otras se pierden. Un **compresor** baja un sonido cuando se
vuelve fuerte:

- Por debajo de su **umbral** no hace nada.
- Por encima, baja el sonido en una cantidad que fija su **relación**. A
  4:1, un sonido 8 dB por encima del umbral sale solo 2 dB por encima:
  cuatro decibelios de entrada por cada uno de salida. A 1:1 no baja
  nada, así que el compresor está apagado. A 20:1 casi nada pasa del
  umbral.
- Su **ganancia** (la «ganancia de compensación») sube toda la parte
  después, ya que las notas fuertes bajaron. La parte suena tan fuerte
  como antes, pero más pareja.

Esa regla es aritmética simple, así que vive en `lineup-core`, donde un
test puede comprobarla sin ningún sonido:

```rust
pub fn reduction(&self, level: f32) -> f32 {
    let over = level - self.threshold;
    if over <= 0.0 || self.ratio <= 1.0 {
        0.0
    } else {
        over * (1.0 - 1.0 / self.ratio)
    }
}
```

A 4:1, 8 dB por encima del umbral se bajan 8 × (1 − ¼) = 6 dB, y salen
2 dB por encima.

Un compresor no baja cada onda. Sigue lo fuerte que ha sonado el sonido
en los últimos milisegundos: baja deprisa cuando un sonido pasa del
umbral (su **ataque**, 10 ms) y vuelve a subir más despacio cuando
vuelve a estar por debajo (su **liberación**, 150 ms). Si siguiera cada
onda, deformaría su forma y añadiría un zumbido. El `Follower` hace
esto:

```rust
let peak = sample.abs();
let speed = if peak > self.level {
    self.attack
} else {
    self.release
};
self.level += (peak - self.level) * speed;
let level = (20.0 * self.level.log10()).max(FLOOR);
let decibels = compressor.gain - compressor.reduction(level);
10f32.powf(decibels / 20.0)
```

`attack` y `release` se calculan una vez a partir de sus tiempos, como
`1 − e^(−1 / (segundos × frecuencia de muestreo))`: con ese número, el
nivel recorre unos dos tercios del camino hacia una nueva intensidad en
el tiempo dado. `20 × log10` convierte el nivel en decibelios,
`reduction` dice cuánto bajar, y `10^(dB / 20)` convierte la respuesta
de nuevo en un número por el que multiplicar la muestra. `.max(FLOOR)`
mantiene el silencio, cuyo logaritmo es menos infinito, en −100 dB.

### El delay: ecos a tiempo con la canción

Un **delay** vuelve a tocar el sonido un poco más tarde, y otra vez,
cada eco más bajo que el anterior. Su **tiempo** es una figura de la
canción, contada en pasos (semicorcheas):

```rust
pub const TIMES: [(u8, &'static str); 6] = [
    (1, "1/16"),
    (2, "1/8"),
    (3, "dotted 1/8"),
    (4, "1/4"),
    (6, "dotted 1/4"),
    (8, "1/2"),
];
```

Así los ecos caen en el ritmo de la canción, y siguen el tempo cuando
cambia. La **corchea con puntillo** («1/8 con puntillo» en el menú) es
la favorita: sus ecos caen tres semicorcheas después de cada nota, entre
los pulsos, y una guitarra o una voz que toca corcheas suena como si
tocara semicorcheas.

```rust
pub fn seconds(&self, tempo: Tempo) -> f32 {
    let step = tempo.seconds_per_beat() / crate::STEPS_PER_BEAT as f32;
    (f32::from(self.steps) * step).min(Delay::MOST_SECONDS)
}
```

Una corchea con puntillo a 120 BPM son 3 × 0,125 = 0,375 s. Un eco
llega como mucho 2 segundos después del sonido: una blanca a 40 BPM
serían 3 segundos, así que a tempos tan lentos los tiempos más largos se
quedan en 2.

- La **Realimentación** dice cuánto de cada eco vuelve como el
  siguiente. Al 50 %, cada eco suena la mitad que el anterior; al 0 % hay
  uno solo. Se detiene en el 90 %, así que los ecos siempre se apagan.
- La **Mezcla** dice lo fuertes que son los ecos al lado del propio
  sonido. 0 % apaga el delay; 100 % hace el primer eco tan fuerte como la
  nota.

El delay recuerda el sonido que vuelve a tocar en un **buffer
circular**: un array fijo que se escribe dando vueltas, donde la muestra
de `delay_frames` atrás siempre va detrás de la que se está escribiendo:

```rust
let frames = line.len();
let read = (self.written + frames - self.delay_frames) % frames;
let echo = line.0[read];
line.0[self.written] = sample + echo * self.settings.delay.feedback;
self.written = (self.written + 1) % frames;
sample += echo * mix;
```

Lo que entra en la línea es el sonido más una parte del eco; esa parte
vuelve otra vez `delay_frames` más tarde, y eso es la realimentación. Lo
que sale es el sonido más el eco a la mezcla. Sumar `frames` antes del
`%` impide que la resta baje de cero, lo que un `usize` no puede
guardar.

### Memoria hecha al otro lado

Dos segundos de sonido son 96 000 muestras a 48 kHz, una reserva grande,
y el hilo de audio nunca reserva memoria. Así que el hilo de la ventana
crea la `DelayLine` y la envía con los ajustes:

```rust
SetEffects {
    channel: usize,
    effects: lineup_core::Effects,
    line: Option<DelayLine>,
},
```

Envía una línea cada vez que envía ajustes con el delay activado, porque
solo el hilo de audio sabe si ese canal ya tiene una.
`ChannelEffects::set` se queda con la primera línea que recibe y
devuelve cualquier otra, que vuelve por la cola de basura para liberarse
del lado de la ventana:

```rust
let spare = match self.channels.get_mut(channel) {
    Some(state) => state.effects.set(effects, line, tempo),
    None => line,
};
if let Some(spare) = spare {
    self.throw_away(Garbage::DelayLine(spare));
}
```

Un canal conserva su línea una vez que la tiene. Cuando su delay se
apaga y se vuelve a encender, `set` llena antes la línea de silencio, o
volverían los ecos de la vez anterior. Cuando cambia el tempo, el motor
llama a `set_tempo` en los efectos de cada canal, así que los ecos se
mueven con él.

`process` mira primero si algún efecto está activado. Si ninguno lo
está, devuelve la muestra intacta: un canal sin efectos suena
exactamente como sonaría sin este código, y no cuesta nada.

## Medidores de nivel: del hilo de audio a la pantalla

Un medidor muestra lo fuerte que suena un canal **ahora**. El hilo de
audio lo sabe, porque hace el sonido; el hilo de la ventana dibuja el
medidor. Van a velocidades distintas (el hilo de audio llena un bloque
de unos cientos de muestras muchas veces por segundo, la ventana dibuja
unos 60 frames por segundo), y el hilo de audio nunca debe esperar a la
ventana, así que un `Mutex` queda descartado.

### Picos en atómicos

El hilo de audio guarda la muestra más fuerte de cada canal durante un
bloque (`channel.peak` más arriba). Al final del bloque sube un pico
compartido hasta ella, si es más fuerte. Una vez por frame, el medidor
toma el pico compartido y lo devuelve al silencio. Así no se pierde
ningún pico, se alineen como se alineen bloques y frames, y ningún lado
espera:

```rust
/// The peaks since the UI last took them, as the bits of `f32`s.
///
/// For numbers that are not negative, comparing their bits compares the
/// numbers, so `fetch_max` on the bits keeps the louder peak.
pub struct Levels {
    channels: [AtomicU32; MAX_CHANNELS],
    master: [AtomicU32; 2],
}
```

```rust
fn raise(slot: &AtomicU32, peak: f32) {
    // `peak > 0.0` is false for NaN too, which must not get in.
    if peak > 0.0 {
        slot.fetch_max(peak.to_bits(), Ordering::Relaxed);
    }
}

fn take(slot: &AtomicU32) -> f32 {
    f32::from_bits(slot.swap(0, Ordering::Relaxed))
}
```

La biblioteca estándar de Rust no tiene `AtomicF32`. Pero un `f32` son
32 bits, y `to_bits` y `from_bits` lo convierten en un `u32` y de vuelta
sin cambiar un bit. Mejor aún: para números **no negativos**, el orden de
los bits como enteros sin signo es el orden de los números, porque el
exponente está en los bits altos y la mantisa debajo. Así que
`fetch_max` sobre los bits, un «quédate con el mayor» atómico, se queda
con el pico más fuerte. Los picos son valores absolutos, nunca
negativos, y el `if` deja fuera los NaN (sus bits se compararían como
mayores que cualquier número).

`swap(0, …)` lee el valor y pone 0 (los bits de 0.0) en su lugar en un
solo paso, así que un pico que llegue entre una lectura y un reinicio no
se puede perder. `Ordering::Relaxed` basta porque cada pico va solo:
nada más en memoria tiene que verse en un orden determinado junto con
él.

`std::array::from_fn(|_| AtomicU32::new(0))` construye los arrays, ya que
un `AtomicU32` no es `Copy` y `[AtomicU32::new(0); N]` no compilaría.

### Decibelios, y un medidor que cae despacio

El oído percibe la intensidad por proporciones, no por diferencias, así
que el medidor se dibuja en **decibelios**, una escala de proporciones:

```text
decibelios = 20 × log10(nivel)      1.0 → 0 dB (el máximo)
                                    0.5 → −6 dB
                                   0.25 → −12 dB
                                  0.004 → −48 dB (el suelo del medidor)
```

Cada 6 dB menos reducen el nivel a la mitad, en cualquier punto de la
escala. Un medidor se ilumina desde −48 dB a la izquierda hasta 0 dB a
la derecha: verde, ámbar desde −6 dB, rojo en el máximo y por encima.
Los medidores del principal miden después del volumen principal y antes
de `tanh`, así que el rojo en el principal significa que el redondeo
está actuando. Al pasar el ratón por un medidor, dice su nivel en
decibelios.

Un medidor que siguiera cada pico exactamente parpadearía demasiado
rápido para leerlo. Así que **salta hacia arriba** a un pico nuevo
enseguida y **cae** 24 dB por segundo:

```rust
pub fn update(&mut self, levels: &Levels, channels: usize, seconds: f32) {
    let fall = gain(-FALL_DB_PER_SECOND * seconds);
    self.channels.resize(channels, 0.0);
    for (index, shown) in self.channels.iter_mut().enumerate() {
        *shown = levels.take_channel(index).max(*shown * fall);
    }
    for (shown, peak) in self.master.iter_mut().zip(levels.take_master()) {
        *shown = peak.max(*shown * fall);
    }
}
```

`seconds` es el tiempo desde el frame anterior. Caer 24 dB por segundo es
multiplicar por 10^(−24 × segundos / 20) en cada frame, sea cual sea la
frecuencia de frames. Medio segundo después de un pico, el medidor lo
muestra 12 dB más abajo, a una cuarta parte, más o menos.

egui solo dibuja un frame cuando pasa algo. Mientras un medidor sigue
cayendo, el mezclador pide el siguiente frame enseguida
(`request_repaint`); cuando todos los medidores están apagados, vuelve a
mirar cada décima de segundo (`request_repaint_after`), lo que basta para
captar las notas de un teclado MIDI tocado con la canción parada.

## Orden de partitura: la batería junta

El rack de canales guarda los canales en el orden en que se añadieron.
Un estilo que trae un aro, un bajo, un cencerro y una conga los añade al
final, así que una lista en el orden del rack desperdiga la batería
alrededor del piano y del bajo. Una partitura de director nunca hace
eso: cada instrumento tiene su lugar, el mismo en todas las partituras,
para que quien la lee encuentre las flautas arriba y los contrabajos
abajo sin buscar. El mezclador lista los canales en ese orden, por
secciones con un título cada una:

| Sección | Instrumentos, de arriba abajo |
|---|---|
| Maderas | flautín, flautas, oboes, corno inglés, clarinetes, saxofones, fagotes |
| Metales | trompas, trompetas, trombones, tuba |
| Percusión | timbales; glockenspiel, xilófono, vibráfono, marimba, campanas tubulares; el resto |
| Batería | percusión de mano, luego la batería |
| Arpa y guitarras | arpa, guitarras, banjo, koto, bajo eléctrico |
| Teclados | celesta, piano, clavecín, órgano, acordeón y bandoneón |
| Sintetizadores | solistas, colchones y efectos de sintetizador; un Lineup Osc sin instrumento |
| Voces | una voz solista, luego el coro |
| Cuerdas | secciones de cuerda, violines, violas, violonchelos, contrabajos |

Las trompas van encima de las trompetas aunque suenan más graves,
porque se funden con las maderas y durante mucho tiempo se escribieron
junto a ellas. Las cuerdas van al final, así que los contrabajos, la
parte más grave de la orquesta, cierran la página; un coro va justo
encima de ellas, como en un réquiem o un oratorio.

La batería sigue el **pentagrama de batería**, donde cada tambor tiene
su línea o su espacio: los platillos en la línea superior y por encima
(el crash el más alto, luego el charles, luego el ride), el tom agudo en
el espacio superior, la caja en el tercer espacio, el tom de pie debajo
y el bombo en el espacio inferior. Así que la sección **Batería** se lee
Crash, Charles, Charles abierto, Ride, Tom agudo, Caja, Aro (un golpe de
aro, escrito donde va la caja), Tom grave, Bombo. La percusión de mano
(cencerro, shaker, palmas, conga) suele tener sus propios pentagramas
encima de la batería, así que va primero.

Solo el mezclador cambia su orden. El rack de canales, el piano roll y
el archivo guardado mantienen los canales como se añadieron, y nada
suena distinto.

### Dónde va un canal

`score_order.rs` está en `lineup-core`, sin código de interfaz, así que
se prueba con un simple `cargo test`. Un `Section` es un enum en orden de
partitura:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Section {
    Woodwinds,
    Brass,
    Percussion,
    Drums,
    HarpAndGuitars,
    Keyboards,
    Synths,
    Voices,
    Strings,
}
```

Derivar `PartialOrd` y `Ord` en un enum ordena sus variantes tal como
están escritas: `Section::Woodwinds < Section::Brass`. El orden de la
partitura es el orden del código fuente.

El mezclador no mira el nombre de un canal para ubicarlo, sino su
instrumento, el programa General MIDI ([SoundFonts y General
MIDI](soundfonts.md)). Un canal llamado «Melodía» que toca una
flauta va con las maderas. Los números de programa ya vienen casi
siempre por familias, pero no siempre como los quiere una partitura:
General MIDI pone los timbales con las cuerdas y la celesta con las
láminas, y la partitura pone los timbales los primeros de la percusión y
la celesta con los teclados. Así que cada sección lista sus programas,
de arriba abajo, y `place` encuentra la sección de un canal y su puesto
en ella:

```rust
pub fn place(channel: &Channel) -> (Section, usize) {
    match &channel.kind {
        ChannelKind::Sampler(SampleSource::Builtin(drum)) => {
            let rank = DRUMS.iter().position(|d| d == drum).unwrap_or(DRUMS.len());
            (Section::Drums, rank)
        }
        // A sound file usually takes a drum's row in the rack.
        ChannelKind::Sampler(SampleSource::File(_)) => (Section::Drums, DRUMS.len() + 1),
        ChannelKind::MidiOut {
            channel: DRUM_CHANNEL,
        } => (Section::Drums, DRUMS.len()),
        ChannelKind::Synth | ChannelKind::MidiOut { .. } => match channel.program {
            Some(program) => program_place(program),
            // The Lineup Osc, or an outside synth with no instrument named.
            None => (Section::Synths, usize::MAX),
        },
    }
}
```

`ChannelKind::MidiOut { channel: DRUM_CHANNEL }` solo encaja con canales
de salida MIDI cuyo campo es igual a la constante `DRUM_CHANNEL` (9, que
se muestra como 10): una `const` puede ir en un patrón como un número
literal. El brazo siguiente, `MidiOut { .. }`, atrapa cualquier otro
canal MIDI; `match` prueba los brazos desde arriba, así que el canal de
batería nunca llega hasta ahí.

```rust
fn program_place(program: u8) -> (Section, usize) {
    Section::ALL
        .into_iter()
        .find_map(|section| {
            let rank = section.programs().iter().position(|&p| p == program)?;
            Some((section, rank))
        })
        .unwrap_or((Section::Synths, usize::MAX))
}
```

`find_map` ejecuta el closure en cada sección y se detiene en la primera
que devuelve `Some`. Dentro, el `?` después de `position` devuelve
`None` desde el closure cuando el programa no está en esa sección, lo
que le dice a `find_map` que pruebe la siguiente. Un test comprueba que
cada uno de los 128 programas está en exactamente una lista, y otro que
cada tambor integrado tiene un lugar en la batería.

### Ordenar y agrupar

```rust
pub fn score_order(channels: &[Channel]) -> Vec<(Section, Vec<usize>)> {
    let mut order: Vec<usize> = (0..channels.len()).collect();
    order.sort_by_key(|&index| place(&channels[index]));
    let mut sections: Vec<(Section, Vec<usize>)> = Vec::new();
    for index in order {
        let section = place(&channels[index]).0;
        match sections.last_mut() {
            Some((last, indices)) if *last == section => indices.push(index),
            _ => sections.push((section, vec![index])),
        }
    }
    sections
}
```

Ordena los **números** de los canales, no los canales: cada fila del
mezclador tiene que saber qué canal es, para cambiar el volumen de ese
canal y poner en solo el correcto. La clave es el par `(Section, usize)`,
y las tuplas se comparan campo a campo, así que decide primero la
sección y el puesto desempata. `sort_by_key` es un ordenamiento
*estable*: los canales con la misma clave, como «Melodía», «Voz 2» y
«Voz 3» tocando todos violines, conservan el orden del rack.

Luego el bucle recorre la lista ordenada y empieza un grupo nuevo cada
vez que cambia la sección. `last_mut` da una referencia mutable al
último grupo, si lo hay; el `if` después del patrón (una *guarda de
match*) solo toma el primer brazo cuando ese grupo es de la misma
sección.

## La ventana

`mixer_window` dibuja a partir de copias: `mix` es una copia del `Mixer`
del proyecto, y `levels` una copia de sus canales. Los deslizadores
cambian las copias, y cuando la ventana termina la aplicación las compara
con el proyecto y hace un solo `edit` con lo que haya cambiado, así que
un arrastre es un paso de deshacer ([Deshacer y
rehacer](../program/undo.md)). El volumen principal pasa en cambio por
`set_master_volume`, que reabre el último paso si también era el volumen
principal, así que moverlo varias veces es un solo paso ([Varios clics,
un paso](../program/undo.md#varios-clics-un-paso)).

```rust
for (section, indices) in score_order(&levels) {
    ui.label(egui::RichText::new(tr(section.name())).color(theme::ACCENT));
    ui.end_row();
    for index in indices {
        let channel = &mut levels[index];
        // name, instrument, meter, volume, pan, send, effects, mute ...
        if ui
            .selectable_label(mix.solo == Some(index), tr("S"))
            // ...
            .clicked()
        {
            mix.solo = if mix.solo == Some(index) {
                None
            } else {
                Some(index)
            };
        }
        ui.end_row();
    }
}
```

La columna **Instrumento** es un menú para un canal de sintetizador o de
salida MIDI, con las mismas opciones que el del rack: el rack y el
mezclador llaman los dos a `channel_rack::instrument_choices`. Un tambor
integrado no tiene instrumento que elegir, pero su menú puede enviarlo
por MIDI ([MIDI](midi.md)); la celda de un archivo de muestra
es texto simple. El menú se dibuja dentro de closures que ya toman
prestadas partes de la aplicación, así que no puede cambiar el proyecto
mientras se dibuja. Guarda lo que elegiste en una lista, y la aplicación
lo aplica cuando la ventana termina:

```rust
for action in picked {
    self.apply(action);
}
```

Es el mismo patrón que usan el rack de canales y la hoja de acordes: el
código de la interfaz dice **qué** hiciste, como un valor
(`RackAction::SetProgram { channel, program }`), y un solo sitio de la
aplicación lo hace.

El botón **Efectos** dice qué efectos están activados («EQ, delay»), o
«ninguno», y abre un menú con los tres. Ese menú se crea con
`PopupCloseBehavior::CloseOnClickOutside`, así que sigue abierto mientras
arrastras sus deslizadores, hasta que haces clic en otro sitio. Los
deslizadores del ecualizador vuelven a 0 dB con un doble clic, como el
balance vuelve al centro.

La ventana se ajusta a sus columnas, y se vuelve a ajustar cuando
cambian los canales o sus instrumentos, porque un nombre de instrumento
más corto puede estrechar una columna ([Ventanas del tamaño de lo que
muestran](../program/egui.md#ventanas-del-tamaño-de-lo-que-muestran)).

## Cómo se prueba

- `reverb.rs`: `bypass_preserves_samples_exactly` (desactivada, las
  muestras salen idénticas bit a bit);
  `impulse_has_a_stereo_tail_that_decays_and_can_be_cleared` a 22,05, 48
  y 96 kHz (silencio durante el predelay, una cola distinta a izquierda
  y derecha, menos de una milésima de la energía del primer medio
  segundo pasado un segundo, y silencio después de `clear`);
  `a_tail_ends_in_zeros_not_denormals`.
- `effects.rs` de `lineup-audio` hace pasar ondas senoidales por los
  efectos y mide lo que sale: Graves a +6 dB sube 50 Hz en 6 dB y deja
  5 kHz igual; un compresor a −20 dB y 4:1 baja 15 dB una onda al
  máximo, 20 dB por encima, y deja pasar una suave; un delay de corchea
  a 120 BPM con un 50 % de realimentación repite un clic a los 0,25 s a
  nivel completo, luego a la mitad, luego a un cuarto; una línea se
  guarda una vez y una segunda se devuelve.
- `effects.rs` de `lineup-core`: la aritmética de la reducción, el
  tiempo del delay a 120 BPM y su límite de 2 segundos, ajustes fuera de
  rango, el resumen del botón («EQ, comp»), y que unos efectos sin
  cambios se guardan como `()`.
- `render.rs` hace pasar un clic por todo el motor:
  `a_panned_channel_plays_more_on_its_side` con cinco balances, contra
  la tabla de arriba; `a_channel_sent_nowhere_plays_dry_in_a_room` (un
  envío de 0 suena tan fuerte como con la reverberación desactivada y no
  deja cola); `a_channel_with_a_delay_echoes_in_time_with_the_song`;
  `the_meters_see_each_channel_and_the_master` (un clic a volumen 0,5,
  a la izquierda, con el principal a 0,8: el medidor del canal marca
  0,25, el izquierdo del principal 0,2 y el derecho 0).
- `levels.rs` y `meters.rs`: el pico más fuerte se guarda hasta que se
  toma y un NaN nunca entra; un medidor salta hacia arriba, cae 12 dB en
  medio segundo y se apaga por debajo del suelo.
- `score_order.rs`: cada programa y cada tambor tienen exactamente un
  lugar; la batería añadida después de la banda se junta con el resto
  de la batería; una orquesta se lee de las flautas a los contrabajos;
  el canal MIDI de batería y los archivos de sonido cuentan como
  batería; los canales en el mismo lugar conservan el orden del rack.
- En la aplicación: `the_mixer_pans_a_channel_and_sets_its_reverb_send`,
  `the_mixer_sets_a_channels_effects` (abre el menú, elige «dotted
  1/4» y luego «No effects»), `a_click_on_an_instrument_in_the_mixer_changes_it`
  (y un Deshacer recupera el piano),
  `the_mixer_keeps_the_drums_together_and_each_one_still_solos_alone`,
  y dos exportaciones: `an_exported_song_keeps_each_channel_where_the_mixer_put_it`
  y `an_exported_song_plays_each_channels_effects`.
- En `file.rs`: `pan_and_reverb_sends_need_format_twenty_six` y
  `effects_need_format_twenty_eight`, cada uno con un archivo editado a
  mano que debe rechazarse.
