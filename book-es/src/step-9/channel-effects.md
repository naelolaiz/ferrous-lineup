# EQ, compresión y delay en cada canal

Archivos: [`crates/lineup-core/src/effects.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/effects.rs),
[`crates/lineup-audio/src/effects.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-audio/src/effects.rs),
[`crates/lineup-audio/src/engine.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-audio/src/engine.rs)
y [`crates/lineup-app/src/app/mixer.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/mixer.rs)

El [mixer](mixer.md) pone a toda la banda en una misma sala con su
reverberación, y coloca cada canal entre los altavoces con su pan. Pero
en una mesa de mezclas real cada tira de canal también tiene sus propios
**efectos**, que dan forma a un instrumento antes de que se una a los
demás. Ahora cada canal del mixer tiene tres, en el orden en que los
tienen la mayoría de las mesas: un **ecualizador**, un **compresor** y
un **delay**.

![El mixer de una canción Motown con trompas. El botón Effects de la línea Horns dice "EQ, delay", y su menú está abierto: Equalizer con Low a -4.0 dB, Mid y High a 0 dB; Compressor con Threshold a -18.0 dB, Ratio 1.0:1 y Gain a 0.0 dB; Delay con Time dotted 1/8, Feedback al 35% y Mix al 30%; y un botón No effects.](../images/step-9-effects.png)

## Pruébalo

1. Abre una demo desde **File > Demos**, pulsa **F9** para el mixer y
   **Space** para reproducir.
2. La columna **Effects** dice "none" en todas las líneas. Haz clic en
   ella en la línea del Piano: se abre un menú con los tres efectos. Se
   queda abierto mientras los cambias, hasta que haces clic en otro
   sitio.
3. Arrastra el deslizador **Low** del **Equalizer** hasta −6 dB. El
   piano se vuelve más delgado, y el bajo se oye mejor debajo de él. El
   botón ahora dice "EQ". **Flat** devuelve todas las bandas a 0 dB, y
   un doble clic en un deslizador devuelve ese.
4. En la línea del Bass, pon el **Ratio** del **Compressor** a 4:1 y su
   **Threshold** a −24 dB. Las notas fuertes bajan. Sube su **Gain**
   unos decibelios para devolver toda la parte a su nivel.
5. En la línea del Lead, pon el **Mix** del **Delay** al 30% y elige
   **dotted 1/8**. Cada nota vuelve tres semicorcheas después, y otra
   vez, más suave cada vez. Cambia el tempo de la canción: los ecos lo
   siguen.

**No effects** al final del menú los apaga todos para ese canal, y
**Reset mix** los apaga en todos los canales. Cada cambio es un paso
para deshacer. Los efectos también suenan en un WAV exportado, y la
canción los guarda.

## El ecualizador: subir o bajar frecuencias

Todo sonido es una mezcla de frecuencias: la fundamental de una nota
grave, y sus armónicos por encima. Un **ecualizador** (EQ) sube o baja
rangos de ellas. No añade notas. Cambia el color de las que hay. El EQ
del mixer tiene tres bandas:

| Banda | Dónde | Forma | Qué vive allí |
|---|---|---|---|
| **Low** | por debajo de 200 Hz | una shelf | el peso de un bajo, un bombo, la mano izquierda de un piano |
| **Mid** | alrededor de 1 kHz | una campana, de una octava y media de ancha | el cuerpo de la mayoría de los instrumentos, y de las voces |
| **High** | por encima de 4 kHz | una shelf | el aire y el brillo: platillos, una voz con aire, un arco sobre una cuerda |

Una **shelf** sube o baja todo lo que pasa de su frecuencia en la misma
cantidad, como un escalón. Una **campana** sube o baja su frecuencia al
máximo, y cada vez menos a cada lado.

Cada banda va de −12 dB a +12 dB. Los decibelios miden una relación de
volumen: +6 dB hace las ondas de una banda unas dos veces más altas,
−6 dB la mitad, 0 dB la deja como está.

Bajar cosas es el uso más común. Cuando dos instrumentos tocan en el
mismo rango, se enmascaran: los graves de un piano y el bajo se
convierten en un sonido embarrado. Quitar los graves al piano deja
sitio al bajo, y el piano no suena más delgado una vez que el bajo toca.

## El compresor: igualar lo fuerte que suena una parte

Algunas partes salen desiguales: una línea de bajo donde unas notas
saltan y otras se pierden. Un **compresor** baja un sonido cuando se
hace fuerte:

- Por debajo de su **threshold** no hace nada.
- Por encima, baja el sonido en la cantidad que fija su **ratio**. La
  4:1, un sonido 8 dB por encima del threshold sale solo 2 dB por
  encima: 4 decibelios de entrada por cada decibelio de salida. A 1:1 no
  baja nada, así que el compresor está apagado. A 20:1 casi nada pasa
  del threshold.
- Su **gain** (el "make-up gain") sube toda la parte después, ya que las
  notas fuertes han bajado. La parte ahora suena tan fuerte como antes,
  pero más constante.

Un compresor no baja cada onda. Sigue lo fuerte que ha sido el sonido en
los últimos milisegundos. Baja rápido en cuanto un sonido pasa del
threshold (su **attack**, 10 ms) y vuelve a subir más despacio cuando el
sonido está de nuevo por debajo (su **release**, 150 ms). Si siguiera
cada onda, doblaría la forma de la onda y añadiría un zumbido.

## El delay: ecos a tiempo con la canción

Un **delay** reproduce el sonido otra vez un poco después, y otra vez,
cada eco más suave que el anterior. Su **time** es un valor de nota de
la canción: una semicorchea, una corchea, una corchea con puntillo, una
negra, una negra con puntillo o una blanca. Los ecos caen sobre el ritmo
de la canción, y siguen el tempo cuando cambia. La **corchea con
puntillo** es la favorita: sus ecos caen entre los pulsos, tres
semicorcheas después de cada nota. Una guitarra o una voz que toca
corcheas suena como si tocara semicorcheas.

- **Feedback** dice cuánto de cada eco vuelve como el siguiente. Al 50%,
  cada eco suena la mitad que el anterior. Al 0% hay un solo eco. Se
  detiene en el 90%, para que los ecos siempre se apaguen.
- **Mix** dice lo fuertes que suenan los ecos junto al propio sonido. 0%
  apaga el delay, 100% hace el primer eco tan fuerte como la nota.

Un eco llega como mucho 2 segundos después del sonido. Una blanca a 40
BPM serían 3 segundos, así que a un tempo tan lento los tiempos más
largos se detienen en 2.

## En el código

### Los ajustes

`lineup-core` guarda los ajustes, en `Channel::effects`:

```rust
pub struct Effects {
    #[serde(default, skip_serializing_if = "Equalizer::is_flat")]
    pub eq: Equalizer,
    #[serde(default, skip_serializing_if = "Compressor::is_default")]
    pub compressor: Compressor,
    #[serde(default, skip_serializing_if = "Delay::is_default")]
    pub delay: Delay,
}
```

`skip_serializing_if` deja un efecto fuera del archivo mientras esté
como en un canal nuevo, y `default` lo vuelve a rellenar cuando falta.
Una canción que no usa efectos guarda el mismo texto que antes, y un
archivo antiguo se abre con todos los efectos apagados. Una canción
donde cualquier canal usa uno se guarda en el formato 28.

La cuenta del compresor también está en el core, para poder probarla sin
sonido:

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

A 4:1, 8 dB por encima del threshold se bajan en 8 × (1 − ¼) = 6 dB, y
salen 2 dB por encima. Lo mismo el tiempo del delay: `Delay::seconds`
multiplica sus steps por la longitud de un step al tempo, y se detiene
en 2 segundos. Una corchea con puntillo (tres steps) a 120 BPM son
3 × 0.125 = 0.375 s.

### Cada sample pasa por los tres

En el hilo de audio, cada canal tiene un `ChannelEffects`. El motor hace
pasar por él cada sample que produce el instrumento, antes del volumen,
el pan y el envío de reverberación del canal:

```rust
let sample = channel.effects.process(channel.instrument.next_sample()) * volume;
```

`process` comprueba primero si hay algún efecto encendido. Si no hay
ninguno, devuelve el sample tal como llegó, así que un canal sin efectos
suena exactamente igual que antes y no cuesta nada.

### El ecualizador: tres biquads

Cada banda es un **biquad**, un filtro de segundo orden, el bloque
básico de la mayoría de los ecualizadores. Cada sample de salida es una
mezcla de los tres últimos samples de entrada y los dos últimos de
salida:

```rust
let output = b0 * input + b1 * self.inputs[0] + b2 * self.inputs[1]
    - a1 * self.outputs[0]
    - a2 * self.outputs[1];
```

Los cinco números `b0`, `b1`, `b2`, `a1` y `a2` deciden qué hace el
filtro. `Biquad::set` los calcula para una shelf o una campana, una
frecuencia y una ganancia, con las fórmulas del "Audio EQ Cookbook" de
Robert Bristow-Johnson, que usan casi todos los programas de audio. Los
tres filtros se ejecutan uno tras otro: low, mid, high. Solo se vuelven
a calcular cuando cambian los ajustes, nunca para cada sample.

Una línea más en `process` descarta cualquier salida menor que 10⁻²⁰.
El eco que un filtro deja de un sonido ya detenido se hace cada vez más
pequeño sin llegar nunca a cero, y unos números tan diminutos
(**denormals**) pueden hacer un procesador muchas veces más lento.

### El compresor: un nivel que sigue los picos

`Follower` guarda lo fuerte que ha sido el canal. Para cada sample, el
nivel avanza una parte del camino hacia el tamaño del sample: una parte
grande cuando el sample es más fuerte (el attack), una parte pequeña
cuando es más suave (el release):

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

`attack` y `release` se calculan una sola vez, a partir de los tiempos:
`1 − e^(−1 / (seconds × sample rate))`. Con ese número, el nivel recorre
unos dos tercios del camino hacia una nueva sonoridad en el tiempo dado.
`20 × log10` convierte el nivel en decibelios, `reduction` dice cuánto
bajar, y `10^(dB / 20)` convierte el resultado de nuevo en un número por
el que multiplicar el sample.

### El delay: un ring buffer

El delay necesita recordar el sonido que vuelve a reproducir: hasta 2
segundos de él, 96,000 samples a 48 kHz. Esa memoria es una `DelayLine`,
usada como **ring buffer**: un array fijo que se escribe una y otra vez
en círculo, donde el sample de `delay_frames` atrás siempre va detrás
del que se está escribiendo:

```rust
let read = (self.written + frames - self.delay_frames) % frames;
let echo = line.0[read];
line.0[self.written] = sample + echo * self.settings.delay.feedback;
self.written = (self.written + 1) % frames;
sample += echo * mix;
```

Lo que entra en la línea es el sonido más una parte del eco. Esa parte
vuelve una vez más `delay_frames` después: eso es el feedback. Lo que
sale es el sonido más el eco al nivel del mix. Sumar `frames` antes del
`%` evita que la resta baje de cero, algo que un `usize` no puede
contener.

### Crear la línea en el otro hilo

El hilo de audio nunca debe reservar memoria (ver [El hilo de audio,
crecido](../step-2/realtime.md)), y una línea de 96,000 samples es una
reserva grande. Así que el hilo de UI la crea, y la envía con los
ajustes:

```rust
SetEffects {
    channel: usize,
    effects: lineup_core::Effects,
    line: Option<DelayLine>,
},
```

La UI envía una línea cada vez que envía ajustes con el delay encendido,
porque solo el hilo de audio sabe si ese canal ya tiene una.
`ChannelEffects::set` conserva la primera línea que recibe y devuelve
cualquier otra. El motor pone esa en la cola de basura, y el hilo de UI
la libera:

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
apaga y se enciende otra vez, `set` rellena primero la línea con
silencio: si no, volverían los ecos de la última vez.

Cuando cambia el tempo, el motor llama a `set_tempo` en los efectos de
cada canal, para que los ecos se muevan con él.

## Cómo sabemos que está bien

- **Pruebas de DSP** en `lineup-audio/src/effects.rs` reproducen ondas
  seno a través de los efectos y miden lo que sale:
  - El EQ sube 50 Hz en 6 dB con Low a +6, baja 1 kHz en 6 dB con Mid a
    −6 y sube 15 kHz en 9 dB con High a +9, y cada uno deja el extremo
    lejano del rango como estaba.
  - Un compresor a −20 dB y 4:1 baja una onda a escala completa, 20 dB
    por encima, en 15 dB, y deja pasar una suave.
  - Un delay de una corchea a 120 BPM con 50% de feedback repite un
    clic a los 0.25 s con el mix al máximo, luego a la mitad, luego a un
    cuarto.
  - Una línea se conserva una vez, la siguiente se devuelve, y un delay
    apagado y encendido conserva su línea.
  - Un canal sin efectos devuelve cada sample tal como llegó.
- **Pruebas de ajustes** en `lineup-core/src/effects.rs`: la cuenta de
  la reducción, el tiempo del delay a 120 BPM y su tope de 2 segundos,
  ajustes fuera de rango y el resumen del botón ("EQ, comp").
- **Una prueba de render** reproduce un clic con un delay y encuentra su
  eco una corchea después, a la mitad de su nivel, en el centro como el
  canal.
- **Pruebas de la app** hacen clic en el botón Effects del mixer,
  comprueban que el menú tiene los tres efectos, eligen "dotted 1/4" y
  "No effects", y exportan una canción con todas las bandas bajadas para
  comprobar que sale más suave.
- **Una prueba de archivo**: una canción con efectos se guarda en el
  formato 28 y vuelve igual, un efecto apagado se queda fuera del texto,
  y un archivo editado a mano con un feedback del 150%, ecos que nunca
  se apagarían, se rechaza.

## Ejercicios

1. Pon un delay de corchea con puntillo con 30% de mix en el Lead de una
   demo, y reprodúcela a 60 BPM y a 140 BPM. ¿La cuántos milisegundos de
   distancia están los ecos en cada caso? Compruébalo con
   `Delay::seconds`.
2. Pon un compresor a −30 dB y 10:1 en la batería. Luego sube su Gain
   hasta que la batería suene tan fuerte como antes. ¿Qué le ha pasado a
   la diferencia entre los golpes fuertes y los suaves? ¿Por qué una
   batería con un compresor fuerte suena "aplastada"?
3. Un compresor que siguiera cada onda no necesitaría un `Follower`.
   Escribe una prueba que pase un seno de 50 Hz por un compresor con un
   attack de 0.0001 s, y mide cuánto se aleja de un seno la forma de la
   salida. Explica por qué el compresor real espera 10 ms.
4. La anchura de la campana está fijada en un Q de 1. Añade un `width` a
   `Equalizer` y un deslizador para él, y úsalo en `Biquad::set`. ¿Qué
   dice el Cookbook que es alpha para un Q? Recuerda el archivo: una
   canción que lo use necesita un formato nuevo, el 29.
5. La mayoría de las mesas dejan que un compresor escuche un canal
   mientras baja otro: un bajo que se atenúa cada vez que golpea el
   bombo ("compresión de cadena lateral"). Esboza qué necesitaría
   `ChannelEffects` para eso. ¿El sample de qué canal leería
   `Follower::gain`?
