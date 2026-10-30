# Herramientas de práctica: metrónomo, entrenador de velocidad, entrenamiento auditivo

Archivos: [`crates/lineup-audio/src/metronome.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-audio/src/metronome.rs),
[`crates/lineup-audio/src/engine.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-audio/src/engine.rs),
[`crates/lineup-audio/src/phrase.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-audio/src/phrase.rs),
[`crates/lineup-core/src/meter.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/meter.rs),
[`crates/lineup-core/src/sequencer.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/sequencer.rs),
[`crates/lineup-core/src/band.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/band.rs),
[`crates/lineup-core/src/history.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/history.rs),
[`crates/lineup-core/src/ear_training.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/ear_training.rs),
[`crates/lineup-app/src/app/practice.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/practice.rs),
[`crates/lineup-app/src/chord_sheet/practice.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/chord_sheet/practice.rs),
[`crates/lineup-app/src/ear_training.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/ear_training.rs)
y [`crates/lineup-app/src/app/ear.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/ear.rs)

Una banda que toca tu hoja de acordes es más útil cuando tocas con
ella. Este capítulo trata de las herramientas que busca un músico que
practica con una grabación: una **cuenta de entrada**, un
**metrónomo**, una **velocidad** más lenta y un **entrenador de
velocidad** que la vuelve a subir, **marcar el tempo** pulsando,
quitarle una parte a la banda para tocarla **tú**, y **repetir** los
compases en los que estás trabajando. Luego, la otra mitad de
practicar: el **entrenamiento auditivo**, aprender a nombrar lo que oyes
sin mirar.

Nada de esto forma parte de la canción. Una canción guardada y una
exportación WAV conservan el tempo propio de la canción y no llevan
clics.

## Practicar con la banda

![La hoja de acordes con su fila de práctica: Count-in y Metronome encendidos, Speed 80%, Tap, «You play» con Bass encendido, «Loop bars 2–3» encendido y «✖ Stop looping»; los compases 2 y 3 sombreados, con una línea verde arriba, y el compás 3 enmarcado en blanco mientras suena](../images/step-9-practice-sheet.png)

### Probarlo

Abre la hoja de acordes (**F5**). Debajo de los botones Banda y
Progresión está la fila de práctica:

- **Cuenta de entrada**: cuando pulsas Reproducir, primero suena un
  compás de clics, y la música empieza en el siguiente «uno». Empieza en
  medio de un compás y la cuenta sigue hasta donde está el cursor de
  reproducción: desde el pulso 3 de un compás de 4/4 oyes «1 2 3 4 1 2»,
  y la música entra en el 3, así que el pulso nunca salta.
- **Metrónomo**: un clic en cada pulso mientras suena la música. El
  primer pulso de cada compás es más agudo (La6, 1760 Hz) que los demás
  (Mi6, una cuarta más abajo, 1318,5 Hz), para que siempre sepas dónde
  está el «uno».
- **Velocidad**: a qué velocidad toca la banda, como porcentaje del
  tempo de la canción, de 25% a 200%. La canción conserva su propio
  tempo: la caja TEMPO sigue diciendo 110, con «TEMPO 80%» encima. El
  **Entrenador de velocidad** de al lado sube la velocidad un poco cada
  vez que el bucle da la vuelta (ver [El entrenador de
  velocidad](#el-entrenador-de-velocidad) más abajo).
- **Pulsar**: haz clic en él al pulso unas cuantas veces y el tempo de
  la canción pasa a ser el tuyo. Las pulsaciones seguidas son un solo
  paso de deshacer.
- **Tocas tú**: un botón por cada músico del estilo de la banda. La
  banda deja fuera esa parte y la tocas tú. Para todos los músicos salvo
  la batería, tu teclado (MIDI, o el teclado del ordenador) cambia al
  sonido de esa parte.
- **Repetir los compases**: haz clic en un compás, Mayús+clic en otro
  (o mantén Mayús y pulsa una flecha), y **🔁 Repetir los compases 2–3**
  los toca una y otra vez. **✖ Dejar de repetir** toca toda la canción
  otra vez desde donde estás.

Cuenta de entrada y Metrónomo también son dos botones con icono al lado
de Reproducir en la barra de herramientas, así que funcionan sin la
hoja de acordes, sobre cualquier canción o patrón, y **Edición >
Ajustes… > Reproducción > Práctica** los tiene junto con la velocidad
de práctica y el entrenador de velocidad. La cuenta de entrada, el
metrónomo y los números del entrenador se recuerdan para la próxima
vez; la velocidad empieza en 100% cada vez que arranca la app, con el
entrenador apagado, para que una canción nunca esté lenta por sorpresa.

### De dónde salen los clics

Los clics no son un canal de la canción. Si lo fueran, se guardarían
con ella, se exportarían a cada archivo WAV, los silenciaría el
mezclador y la banda los movería cuando reescribe sus partes. Los hace
el propio hilo de audio, que ya sabe cuándo empieza cada paso.

Lo que necesita saber es *qué clic tocar en cada paso*. Eso depende del
compás, que puede cambiar en medio de la canción, así que el hilo de la
interfaz lo calcula y envía la respuesta como una lista, una entrada por
paso:

```rust
pub enum Click {
    #[default]
    None,
    Beat,
    Bar,
}
```

```rust
pub fn click(self, step: usize) -> Click {
    if self.is_bar_start(step) {
        Click::Bar
    } else if self.is_beat_start(step) {
        Click::Beat
    } else {
        Click::None
    }
}
```

`Beat` quiere decir el pulso *que se siente*, el que marcas con el pie
(ver [Pulsos de distinta
longitud](../sound/tempo.md#pulsos-de-distinta-longitud)):

| Compás | Clics en un compás | Por qué |
|--------|--------------------|---------|
| 4/4 | 1 2 3 4 | cuatro negras |
| 3/4 | 1 2 3 | un vals |
| 6/8 | 1 2 | dos negras con puntillo: seis corcheas sentidas como dos grupos de tres |
| 2/2 | 1 2 | dos blancas, el «compás partido» |
| 2+2+3/8 | 1 2 3 | tres pulsos, el último más largo |

Así que un metrónomo en 6/8 no hace seis clics por compás. Una giga, o
Noche de paz, se siente en dos, y marcar cada corchea lo escondería.

La lista de la canción sale de `Project::played_clicks`, que cuenta los
compases tal como *suena* la canción: una sección en 3/4 que se repite
está en 3/4 las dos veces (ver [La canción tal como
suena](../sound/tempo.md#la-canción-tal-como-suena)). La del patrón
seleccionado sale de `Project::pattern_clicks`. Van al hilo de audio
como `Command::SetSongClicks` y `Command::SetPatternClicks`. Cuando llega
una lista nueva, el hilo de audio la pone en su sitio con
`std::mem::replace` y devuelve el `Vec` viejo al hilo de la interfaz
como `Garbage::Clicks`, para liberarlo allí (ver [Liberar memoria en
otro
sitio](../program/threads.md#liberar-memoria-en-otro-sitio-la-cola-de-basura)).

### El clic en sí

`ClickVoice`, en `metronome.rs`, toca una onda senoidal corta que se
apaga:

```rust
const RING: f32 = 0.03;
```

```rust
decay: (-6.9 / (RING * sample_rate)).exp(),
```

En cada frame el nivel se multiplica por `decay`. Después de `RING`
segundos (30 ms, 1440 frames a 48 kHz) se ha multiplicado por `decay`
1440 veces, que es e<sup>−6,9</sup> ≈ 0,001, una milésima de donde
empezó: −60 dB, demasiado bajo para oírse. Lo bastante largo para oír
una altura, lo bastante corto para terminar mucho antes del pulso
siguiente incluso a 300 BPM.

### La cuenta de entrada: un segundo reloj

El hilo de audio tiene un `Sequencer`, el reloj que dice cuándo empieza
cada paso (ver [El secuenciador](../sound/sequencer.md)). También
mueve el cursor de reproducción, que la ventana lee para dibujar por
dónde va la canción. Durante una cuenta de entrada la canción no ha
empezado, así que el cursor no debe moverse. La cuenta tiene un reloj
propio, `counter`, y un pequeño registro de cuánto ha avanzado:

```rust
/// A count-in under way: the bar it counts (where it starts in the
/// loop's clicks, and how many steps it has), how many steps it lasts in
/// all, and how many have gone by.
#[derive(Debug, Clone, Copy)]
struct CountIn {
    bar: usize,
    length: usize,
    steps: usize,
    counted: usize,
}
```

Reproducir la pone en marcha:

```rust
let (bar, length) = count_in_bar(self.loop_clicks(), self.start);
let length = length.max(1);
let into_the_bar = self.start.saturating_sub(bar) % length;
self.counter.set_tempo(self.tempo_of_step(self.start));
self.counter.reset();
self.counting = Some(CountIn {
    bar,
    length,
    steps: length + into_the_bar,
    counted: 0,
});
```

`count_in_bar` busca hacia atrás desde el cursor el último
`Click::Bar`, y hacia delante el siguiente, para encontrar el compás en
el que está el cursor. La cuenta dura ese compás entero *más* los pasos
desde su comienzo hasta el cursor: desde el paso 6 de un compás de 16
pasos, 16 + 6 = 22 pasos, «1 2 3 4 1 2», y la música empieza en el paso
6, medio pulso dentro del pulso 2. Cuenta al tempo en que empezará la
música, así que una canción que empieza lenta se cuenta lenta.

Después, en cada frame:

```rust
if let Some(count) = self.counting {
    self.count_in_frame(count);
}
```

```rust
fn count_in_frame(&mut self, mut count: CountIn) {
    if self.counter.tick().is_none() {
        return;
    }
    if count.counted == count.steps {
        self.counting = None;
        self.sequencer.start_at(self.start);
        self.playing = true;
        self.trace_play(self.start);
        return;
    }
    let step = count.bar + count.counted % count.length;
    self.click.strike(click_at(self.loop_clicks(), step));
    count.counted += 1;
    self.counting = Some(count);
}
```

Cuando la cuenta termina, el secuenciador de la canción arranca *en ese
mismo frame*: el bucle de frames atiende la cuenta antes de pedirle un
paso al secuenciador de la canción, así que la primera nota cae justo un
compás después del primer clic. La prueba
`the_count_in_comes_before_the_music` lo comprueba al frame: a 120 BPM
un paso son 6.000 frames a 48 kHz, los clics llegan en los frames 0,
24.000, 48.000 y 72.000 (cada uno se oye un frame después, porque una
senoide empieza en cero), el cursor espera, y la primera nota suena en
el 96.000.

`CountIn` deriva `Copy`, así que `if let Some(count) = self.counting`
hace una copia, y `count_in_frame` recibe la copia. Si tomara una
referencia dentro de `self.counting`, ese préstamo de `self` seguiría
vivo mientras la función llama a `self.counter.tick()` y a
`self.click.strike(…)`, que necesitan `&mut self`: el borrow checker lo
rechazaría. Trabajar sobre una copia y escribirla de vuelta al final
(`self.counting = Some(count)`) es la salida habitual para un estado
pequeño como este.

Detener, durante la cuenta, la termina. Mover el cursor o cambiar entre
modo patrón y modo canción durante la cuenta la vuelve a empezar,
contando el compás en el que empezará ahora la reproducción. Las teclas
que tocas durante la cuenta no se graban: la canción no tiene posición
hasta que empieza la música, y la grabación solo guarda las notas que
la tienen.

### La velocidad

La velocidad de práctica nunca toca el proyecto. `Project::tempo` y las
marcas de tempo se quedan como están; solo cambia lo que se le dice al
hilo de audio:

```rust
pub(super) fn at_speed(tempo: Tempo, speed: f32) -> Tempo {
    if speed == 1.0 {
        tempo
    } else {
        Tempo::new(tempo.bpm() * speed)
    }
}
```

Todo comando que lleva un tempo pasa por ahí: `SetTempo` después de una
edición, y el tempo de cada paso de la canción, así que una canción que
se frena al final se sigue frenando, desde el 80% de su tempo. Una
exportación WAV construye sus comandos a velocidad `1.0`, y por eso
nunca sale más lenta.

### Marcar el tempo

Marcar el tempo es aritmética con los momentos de las pulsaciones.
Pulsaciones a los 10,0, 10,5, 11,0 y 11,5 segundos son tres pulsos en
1,5 segundos:

> BPM = 60 × pulsos ÷ segundos = 60 × 3 ÷ 1,5 = 120

Usar la primera y la última pulsación de la serie, en vez de solo las
dos últimas, compensa la irregularidad de una mano humana. Solo cuentan
las últimas ocho, así que se sigue un tempo que se asienta mientras
pulsas, y una pausa de más de dos segundos empieza una serie nueva. El
resultado se redondea a un BPM entero.

Pulsas los pulsos que marca el metrónomo, y no siempre son negras,
mientras que un tempo en BPM cuenta negras. Así que la app calcula
primero cuántas negras dura un pulso:

```rust
let time = self.project.time_signature;
let beats = time.beat_starts().len().max(1);
let quarters = time.steps_per_bar() as f64 / beats as f64 / f64::from(STEPS_PER_BEAT);
```

```rust
let bpm = (60.0 * quarters * beats as f64 / (last - first)).round() as f32;
```

En 6/8 un compás tiene 12 pasos y dos pulsos, así que un pulso son 6
pasos, una negra con puntillo, 1,5 negras. Pulsar 80 pulsos por minuto
fija 120 BPM. En 2/2 un pulso es una blanca, dos negras. En un 7/8
simple cada corchea es un pulso; en 2+2+3/8 los tres pulsos son
desiguales, así que un pulso marcado cuenta como su promedio, 7/6 de
negra.

Cada pulsación es un clic, y cada clic empieza normalmente un paso de
deshacer nuevo (ver [Varios clics, un
paso](../program/undo.md#varios-clics-un-paso)). Deshacer un tempo
pulsación a pulsación sería pesado, así que `History` tiene un método
para eso:

```rust
pub fn reopen(&mut self, label: &'static str) -> bool {
    if self.redo.is_empty() && self.undo_label() == Some(label) {
        self.open = true;
    }
    self.open
}
```

A partir de la tercera pulsación de una serie, la app vuelve a abrir el
paso «marcar el tempo» antes de cambiar el tempo, así que toda la serie
es un paso. Solo lo hace cuando el último paso *es* «marcar el tempo» y
no se deshizo nada desde entonces: nunca une una pulsación a otro
cambio.

### Tocas tú

Quitarle una parte a la banda silencia la pista de ese músico en la
lista de reproducción:

```rust
pub fn set_you_play(&mut self, player: Player, you: bool) -> bool {
    let Some(track) = self.player_track(player) else {
        return false;
    };
    self.playlist.tracks[track].muted = you;
    true
}
```

Silenciar, y no quitar, importa. La banda sigue escribiendo la parte
cuando cambias un acorde, así que cuando vuelves a hacer clic en el
botón toca lo que la hoja dice *ahora*. Y como la huella de una parte
deja fuera la pista en la que están sus clips (ver [Una parte que
editas sigue siendo
tuya](band.md#una-parte-que-editas-sigue-siendo-tuya)),
silenciarla no cuenta como edición: la parte no se congela.

Para todos los músicos salvo la batería, la app también selecciona el
canal de ese músico, el que toca tu teclado: te encargas del bajo *con*
el sonido del bajo. La batería toca en varios canales (bombo, caja,
charles…), así que no hay un solo canal que seleccionar.

### Repetir los compases que elijas

La hoja recuerda, además del compás seleccionado, un `anchor`: el otro
extremo de una selección hecha con Mayús.

```rust
fn select_to(&mut self, bar: usize, shift: bool) {
    self.anchor = match (shift, self.anchor, self.selected) {
        (false, ..) => None,
        (true, Some(anchor), _) => Some(anchor),
        (true, None, selected) => selected,
    };
    self.selected = Some(bar);
    self.clipboard = None;
}
```

El primer Mayús+clic guarda como ancla el compás que estaba
seleccionado; los siguientes mueven solo el otro extremo, como en un
editor de texto. Suprimir borra todos los compases seleccionados, en un
paso de deshacer.

**Repetir los compases** convierte la selección en la región de bucle
de la lista de reproducción, que el modo canción ya sabe tocar (ver
[Secciones, repeticiones y
bucles](../program/project.md#secciones-repeticiones-y-bucles)), pasa
a modo canción y va al comienzo del bucle. Los compases después del
final de la canción no se pueden repetir: no hay nada que tocar ahí, y
la ayuda del botón lo dice. En la hoja, los compases que se repiten
llevan una línea verde arriba, el color del bucle en la lista de
reproducción.

## El entrenador de velocidad

Un pasaje que no puedes tocar a velocidad completa se aprende igual en
todos los instrumentos: despacio, hasta que sale bien, luego un poco
más rápido, y otra vez, hasta llegar al tempo. Con un metrónomo eso
obliga a parar cada pocas vueltas para girar una perilla. El
**entrenador de velocidad** la gira por ti: cada vez que el bucle da la
vuelta, la banda toca unos puntos más rápido, hasta llegar a la
velocidad que buscas.

![La fila de práctica de la hoja de acordes mientras se repiten los compases 3–4: Speed 80%, Speed trainer encendido, «from 70% to 100% by 5% every 1 pass» al lado, y la caja TEMPO diciendo 80%. El compás 3 está enmarcado en blanco mientras suena, y la barra de estado dice «Speed trainer: 80%, on the way to 100%»](../images/step-9-speed-trainer-sheet.png)

### Probarlo

1. Pulsa **Ctrl+N** para una canción nueva con banda, a 110 BPM, y
   escribe cuatro compases en la hoja de acordes: `| C | Am | F | G |`.
   Haz clic en el compás 3, Mayús+clic en el compás 4 y haz clic en
   **🔁 Repetir los compases 3–4**.
2. Haz clic en **Entrenador de velocidad**. La velocidad baja a
   **70%**, y aparecen cuatro casillas junto al botón: **desde 70% hasta
   100% por 5% cada 1 pasada**.
3. Pulsa Reproducir. La banda empieza al 70% de 110 BPM, 77 BPM. Cada
   vez que termina el compás 4 y vuelve el compás 3, la velocidad sube un
   5% del tempo de la canción: 75%, 80%, 85%… En la séptima vuelta toca
   al 100%, el tempo propio de la canción, y se queda ahí. La barra de
   estado dice por dónde va: «Entrenador de velocidad: 80%, camino al
   100%».
4. Pulsa **Detener** (■). La velocidad vuelve al 70%, lista para empezar
   lenta la próxima tanda. **Pausa** (❚❚) conserva la velocidad, así que
   Reproducir sigue desde donde estabas, a la velocidad a la que habías
   llegado.

Las cuatro casillas se arrastran, o se escriben con doble clic:

- **desde**: la velocidad a la que empieza Reproducir.
- **hasta**: la velocidad a la que llegar. 100% es el tempo de la
  canción; 110% lo pasa, para tener margen cuando lo toques de verdad.
  Un objetivo por debajo del comienzo frena en vez de acelerar.
- **por**: cuánto más rápida es cada velocidad nueva, del 1% al 25% del
  tempo de la canción. El último paso se detiene en el objetivo, así que
  96% por 8% da 100%.
- **cada**: cuántas veces suena el bucle a cada velocidad antes de la
  siguiente, de 1 a 16. Con **cada 2 pasadas** tocas cada velocidad dos
  veces: una para conseguirla, otra para comprobar que la tienes.

Una velocidad que pones a mano en la casilla **Velocidad**, durante una
tanda, es desde donde sigue el entrenador. Hacer clic otra vez en
**Entrenador de velocidad** lo apaga y deja la velocidad donde llegó.

«El bucle» es lo que la reproducción repite: los compases que
repites, la canción entera cuando no se repite nada (vuelve a empezar
desde el compás 1 al terminar), o el patrón, en modo patrón. El
entrenador funciona con los tres. Los mismos controles están en
**Edición > Ajustes… > Reproducción**, debajo de la velocidad de
práctica:

![Ajustes, Playback, Practice: Count-in, Metronome, Practice speed 100% con Reset, y luego Speed trainer con «from 70% to 100% by 5% every 1 pass»](../images/step-9-speed-trainer-settings.png)

### Contar las vueltas

Para ir más rápido «cada vez que el bucle da la vuelta», la app tiene
que saber cuándo pasa. La ventana podría mirar el cursor y contar una
vuelta cada vez que el número de paso baja, del último paso al primero.
Pero el número de paso también baja cuando haces clic en un compás
anterior mientras suena la música, y eso no es una vuelta: el
entrenador aceleraría cada vez que saltas atrás para volver a probar un
compás.

Quien lo sabe es el `Sequencer` del hilo de audio, que cuenta sus
**vueltas** (`laps`): las veces que siguió tocando más allá del último
paso y volvió al paso 0 (ver [El tick](../sound/sequencer.md#el-tick)).
Dentro de `tick`:

```rust
if self.coming_round {
    self.laps += 1;
}
self.next_step = (self.next_step + 1) % self.length;
self.coming_round = self.next_step == 0;
```

`coming_round` se activa cuando suena el último paso, y la vuelta se
cuenta cuando suena el paso 0 después. Un salto (`start_at`, que envía
un clic en un compás), un reinicio (`reset`) y un bucle acortado por
debajo de donde iba la reproducción (`set_length`) desactivan
`coming_round`, así que ninguno cuenta.

El hilo de audio guarda la cuenta después de cada paso en un entero
atómico, como hace con el cursor, y `AudioEngine::laps()` la lee. La
ventana la mira una vez por fotograma:

```rust
fn came_round(&mut self, laps: usize) -> Option<u16> {
    let seen = self.laps_seen.replace(laps);
    if !self.training {
        return None;
    }
    // A new audio thread, after a restart, counts from 0 again.
    let new = laps.checked_sub(seen?)?;
    if new == 0 {
        return None;
    }
    // ...
}
```

Comparar con la cuenta que vio la última vez, en vez de buscar «hubo
una vuelta», hace que nunca se pierda una: si la ventana estuvo oculta
unos segundos y no se dibujó, encuentra varias vueltas nuevas a la vez
y avanza la velocidad una vez por cada una.

En una función que devuelve un `Option`, `valor?` quiere decir «el valor
de dentro, o devuelve `None` desde aquí». `seen?` devuelve `None` la
primera vez, cuando todavía no se vio nada. `checked_sub` es una resta
que da `None` en vez de bajar de cero: después de **Reiniciar audio**
un hilo de audio nuevo vuelve a contar desde 0, `laps` es menor que
`seen`, y el entrenador espera a la siguiente vuelta en vez de calcular
un número enorme de ellas. Un simple `laps - seen` entraría en pánico
ahí en una compilación de depuración, y daría la vuelta hasta un número
cercano a 2⁶⁴ en una de release.

### En el pulso, no un paso tarde

La ventana se dibuja unas 60 veces por segundo, así que se entera de
una vuelta hasta un fotograma después, y el hilo de audio necesita
luego un bloque más para recibir un comando. Para entonces el paso 0 de
la vuelta nueva ya empezó, al tempo viejo, y la velocidad nueva
empezaría en el paso 1: la primera semicorchea de cada pasada un poco
lenta. Nadie notaría una semicorchea un 5% más larga, pero el clic del
primer pulso llegaría unos milisegundos tarde, en cada vuelta.

Así que la ventana no espera a la vuelta. En cuanto sabe la velocidad
siguiente (justo después de la última vuelta, o cuando enciendes el
entrenador), le envía al hilo de audio los tempos de esa velocidad,
para usarlos **a partir de la vuelta siguiente**:

```rust
pub struct LapTempos {
    /// In place of the [`Command::SetTempo`] tempo.
    pub tempo: Tempo,
    /// In place of the [`Command::SetSongTempos`] list.
    pub song: Vec<Tempo>,
}
```

```rust
SetLapTempos(Option<LapTempos>),
```

El hilo de audio los guarda hasta que el bucle da la vuelta. Justo
antes de que suene un paso ya pregunta a qué tempo va ese paso; antes,
pregunta si es el paso 0 que vuelve, y si lo es, toma los tempos que
esperan:

```rust
if let Some(step) = self.sequencer.due() {
    if self.sequencer.comes_round()
        && let Some(tempos) = self.lap_tempos.take()
    {
        self.tempo = tempos.tempo;
        let old = std::mem::replace(&mut self.song_tempos, tempos.song);
        self.throw_away(Garbage::Tempos(old));
    }
    let tempo = self.tempo_of_step(step);
    if tempo != self.tempo_now {
        self.set_tempo(tempo);
    }
}
```

`take()` vacía la ranura, así que los tempos se usan una sola vez. La
lista vieja vuelve al hilo de la interfaz para liberarse, como toda
lista que suelta el hilo de audio.

Cuando la ventana ve después la vuelta, envía otra vez los mismos
tempos con los habituales `SetTempo` y `SetSongTempos`. Si llegaron a
tiempo, eso no cambia nada: son exactamente los mismos números. Importa
cuando no llegaron: si el pedido alcanzó al hilo de audio después de que
el bucle ya diera la vuelta, el reenvío pone la velocidad nueva un paso
tarde, y el pedido siguiente reemplaza al viejo antes de que pueda
sonar una vuelta tarde.

### Mantener el pedido al día

Un pedido espera una vuelta entera, y la canción puede cambiar mientras
tanto: escribes un acorde, y la parte nueva de la banda va al hilo de
audio con la lista de tempos de la canción. Una lista de tempos
calculada antes podría tener la longitud equivocada, o faltarle un
cambio de tempo que acabas de hacer. Así que cada vez que la ventana
envía `SetTempo` o `SetSongTempos`, lo cuenta (`tempos_sent`), y guarda,
junto al cambio de velocidad que pidió, la cuenta de ese momento. Una
vez por fotograma los compara con lo que pediría ahora:

```rust
let change = self.practice.next_change();
let sent = change.map_or(0, |_| self.practice.tempos_sent);
if self.practice.lap_change != Some((change, sent)) {
    let tempos = change.map(|(_, next)| {
        let (tempo, song) = self.tempos_at(next);
        LapTempos { tempo, song }
    });
    self.send(Command::SetLapTempos(tempos));
    self.practice.lap_change = Some((change, sent));
}
```

`next_change` es `None` mientras el entrenador está apagado, en su
objetivo, o antes de la última pasada a una velocidad cuando toca cada
velocidad más de una vez; entonces la ventana envía
`SetLapTempos(None)`, que retira los tempos que aún esperan. Detener,
una velocidad escrita a mano o números nuevos en el entrenador cambian
`next_change`, así que el pedido los sigue también.

### Cuánto dura una tanda

Dos compases de 4/4 son 8 pulsos. Con los valores por defecto sobre los
compases 3–4 de la canción de arriba, a 110 BPM:

| Vuelta | Velocidad | BPM | Empieza a los |
|---|---|---|---|
| 1 | 70% | 77 | 0 s |
| 2 | 75% | 82,5 | 6,23 s |
| 3 | 80% | 88 | 12,05 s |
| 4 | 85% | 93,5 | 17,51 s |
| 5 | 90% | 99 | 22,64 s |
| 6 | 95% | 104,5 | 27,49 s |
| 7 | 100% | 110 | 32,08 s |

Ocho pulsos a 77 BPM duran 8 × 60 ÷ 77 = 6,23 segundos; a 82,5 BPM los
ocho siguientes duran 5,82, y 6,23 + 5,82 = 12,05. Cada cambio llega en
el primer paso del bucle, que el [registro de
reproducción](../sound/trace.md) muestra como una línea `# tempo:`
en el paso 0.

## Entrenamiento auditivo

Todo lo demás en la app te ayuda a *ver* la armonía: nombres de
acordes en la hoja, números romanos y colores en las teclas de
armonía, nombres de conjuntos en la ventana dodecafónica. Los músicos
también necesitan *oírla*: distinguir una tercera menor de una mayor,
un acorde disminuido de uno menor, y el V de una canción de su IV, sin
mirar. Es una habilidad propia, y se aprende como la enseñan los juegos
de entrenamiento auditivo: algo suena, lo nombras, y te dicen al
momento si acertaste.

Es **experimental** y no estorba: vive en su propia ventana, que está
cerrada hasta que la abres, y no cambia nada de la canción ni del
archivo.

### Probarlo

**Ver > Entrenamiento auditivo** abre la ventana. Arriba hay tres
pestañas: **Intervalos**, **Acordes** y **Progresiones**. Debajo, un
recuadro dice qué preguntar; luego **▶ Empezar** hace la primera
pregunta y la toca. Si la canción estaba sonando, se pausa, para que la
pregunta se oiga sola.

![La pestaña Intervals: m3, M3, P4, P5 y P8 elegidos, tocados Up. La respuesta P5 está en verde, y el texto explica: «Bb3 up to F4: a perfect fifth, 7 semitones. Open and stable: the root and the fifth of every major and minor chord. Like Twinkle, Twinkle, Little Star, at its first leap.»](../images/step-9-ear-intervals.png)

**Intervalos.** Suenan dos notas, y los botones grandes son las
respuestas. El recuadro de arriba elige qué intervalos se pueden
preguntar (para empezar, una tercera menor, una tercera mayor, una
cuarta, una quinta y la octava) y cómo se tocan: **Arriba** (primero la
nota grave), **Abajo** (primero la aguda) o **Juntas**. Menos opciones
lo hacen más fácil; empieza con unas pocas que suenen muy distintas, y
añade las demás de una en una. En la interfaz en español los botones se
llaman 3m, 3M, 4J, 5J, 8J y así: el número y la clase, M mayor, m
menor, J justa.

Después de responder, tu botón se pone verde o rojo, el correcto se
pone verde, y un recuadro explica la respuesta: las dos notas con sus
octavas, el nombre completo del intervalo, cuántos semitonos abarca, qué
le da su sonido y, hacia arriba, una melodía que empieza con él. Luego
cada botón de respuesta toca *su propio* intervalo desde la misma nota,
para que puedas oír el que dijiste al lado del correcto. **▶ Volver a
tocar** repite la pregunta todas las veces que quieras; **Siguiente ▸**
hace otra (antes de responder es **Saltar ▸**, y una pregunta saltada
no cuenta).

![La pestaña Chords con las nueve clases e Inversions marcados. La respuesta «maj» está en rojo, «dim7» en verde, y el texto: «Ddim7: D diminished seventh, root position. D F Ab Cb: a minor third, a minor third and a minor third stacked up from D. Cb is the B key, written Cb because it is the chord's seventh...»](../images/step-9-ear-chords.png)

**Acordes.** Suena un acorde, y nombras su clase: mayor, menor,
disminuido, aumentado, o uno de cinco acordes de séptima. La
fundamental cambia de una pregunta a otra; solo cuenta la clase. **Nota
por nota** toca el acorde una nota cada vez desde el bajo hacia arriba,
cada una sostenida, así que termina como el acorde entero. Con
**Inversiones** marcado, la nota más grave no siempre es la
fundamental.

![La pestaña Progressions mientras toca la banda: cuatro casillas, la primera con I, la segunda enmarcada en blanco porque suena su acorde, y debajo siete teclas de armonía que muestran solo I ii iii IV V vi vii° y home/away/tension, con iii y vii° apagadas.](../images/step-9-ear-progression-playing.png)

**Progresiones.** La banda toca unos acordes en una tonalidad, un
compás cada uno, y los nombras por sus números romanos. La pregunta
dice la tonalidad («En Fa mayor: ¿cuáles son los acordes después de
I?») y el primer acorde es siempre I, la casa, para que sepas desde
dónde empiezas. Las respuestas son las teclas de armonía, con sus
colores y su función (casa, fuera, tensión) pero con los nombres de los
acordes ocultos. Las teclas de los acordes que no se pueden preguntar
aparecen apagadas. Mientras toca la banda, un marco blanco recorre las
casillas, como en las teclas de armonía, para que sepas qué compás
estás oyendo.

Haz clic en las teclas en orden, una por cada acorde después de I.
Luego cada casilla se pone verde o roja, una roja dice cuál era el
acorde («era IV»), y el recuadro escribe los acordes, sus números y por
qué cada uno sigue al anterior, con las palabras de las ayudas de las
teclas de armonía:

![La progresión respondida: casillas I, vi (was IV), IV (was V), V (was I) y el texto «F – Bb – C – F: I IV V I in F major. I to IV: moving away from home, toward the dominant. IV to V: a step up to the dominant, ready to come home. V to I: the strongest ending. The leading tone rises to the root.»](../images/step-9-ear-progression.png)

Después de responder, cada casilla y cada tecla tocan su acorde en el
piano, y **Escuchar la tonalidad** toca I, IV, V y I: una cadencia que
dice dónde está la casa, lo que ayuda cuando te pierdes.

El recuadro de arriba elige las **Tonalidades** (Mayor, Menor o Ambas),
los **Acordes** que pueden salir (I IV V; I ii IV V vi; o Los siete),
si son **Séptimas** (V7, ii7, Imaj7...), los **Músicos** (Piano, Piano y
bajo, o la Banda con batería), la **Duración** (4 o 8 acordes) y el
**Tempo** (de 60 a 160 BPM).

**La puntuación.** La línea de abajo cuenta los aciertos, el porcentaje
y cuántos seguidos. Cada pestaña lleva la suya, y en una progresión cada
acorde cuenta como una respuesta. **Empezar de nuevo** la pone a cero.
La puntuación se olvida al cerrar la app; las opciones de los recuadros
se recuerdan.

### La teoría en las respuestas

La app enseña con sus respuestas, así que tienen que ser exactamente
correctas. Esto es lo que dicen, y por qué.

#### Intervalos

El nombre de un intervalo tiene dos partes. El **número** cuenta
letras (nombres de nota), incluidos los dos extremos: de Do a Mi son
Do, Re, Mi, tres notas, una *tercera*. De Do a Mi♭ también es una
tercera: los mismos nombres. La **clase** sale luego de los semitonos:
una tercera de 4 semitonos es *mayor*, de 3 *menor*.

Segundas, terceras, sextas y séptimas son mayores o menores. Unísonos,
cuartas, quintas y octavas se llaman **justas**: invertidas (la nota
grave subida una octava), un intervalo justo da otro justo, mientras
que uno mayor se vuelve menor y uno menor mayor. Los números de un
intervalo y de su inversión suman nueve, sus semitonos doce: una
tercera mayor (4) invertida es una sexta menor (8), una cuarta justa (5)
una quinta justa (7). Un semitono más que mayor o justo es *aumentado*,
uno menos que menor o justo *disminuido*.

| Botón | Nombre | Semitonos | Desde Do | Una melodía que empieza con él |
|-------|--------|-----------|----------|--------------------------------|
| 2m | segunda menor | 1 | Do–Re♭ | Tiburón, las dos notas del tiburón |
| 2M | segunda mayor | 2 | Do–Re | Cumpleaños feliz, en su primer paso hacia arriba |
| 3m | tercera menor | 3 | Do–Mi♭ | Greensleeves |
| 3M | tercera mayor | 4 | Do–Mi | When the Saints Go Marching In |
| 4J | cuarta justa | 5 | Do–Fa | la Marcha nupcial de Wagner |
| TT | tritono (cuarta aumentada) | 6 | Do–Fa♯ | Maria, de West Side Story |
| 5J | quinta justa | 7 | Do–Sol | Estrellita, ¿dónde estás?, en su primer salto |
| 6m | sexta menor | 8 | Do–La♭ | The Entertainer, en su primer salto hacia arriba |
| 6M | sexta mayor | 9 | Do–La | My Bonnie Lies over the Ocean |
| 7m | séptima menor | 10 | Do–Si♭ | Somewhere, de West Side Story |
| 7M | séptima mayor | 11 | Do–Si | |
| 8J | octava | 12 | Do–Do | Over the Rainbow |

Las melodías empiezan con el intervalo hacia arriba, así que la app
nombra una solo cuando la pregunta fue hacia arriba. La séptima mayor
no tiene ninguna: pocas melodías conocidas empiezan con ella.

El **tritono** es el único intervalo con dos números. Seis semitonos
por encima de Do está la tecla de Fa♯ o la de Sol♭: de Do a Fa♯ son
cuatro nombres (Do Re Mi Fa), una *cuarta aumentada*; de Do a Sol♭ son
cinco (Do Re Mi Fa Sol), una *quinta disminuida*. Las mismas teclas,
el mismo sonido, dos escrituras. Su nombre dice lo que es: tres tonos
enteros, exactamente media octava.

**Escritura.** La explicación escribe las notas como lo haría un
músico. La nota de abajo es una tecla blanca, o una negra escrita con
sostenido o con bemol, la que escriba el *par* con menos alteraciones:
de Do♯ a Mi es una tercera menor (escribirla de Re♭ a Fa♭ costaría dos
bemoles), pero de Re♭ a Fa es una tercera mayor (de Do♯ a Mi♯ costaría
dos sostenidos). Cuando las dos son igual de simples, una tecla negra
conserva su nombre habitual: de Si♭ a Re♭, no de La♯ a Do♯. El tritono
es una cuarta aumentada o una quinta disminuida por la misma regla: de
Fa a Si (sin alteración) en vez de Fa a Do♭, y una cuarta aumentada
cuando las dos son igual de simples: de Do a Fa♯ en vez de Do a Sol♭,
de Si♭ a Mi en vez de Si♭ a Fa♭ o La♯ a Mi.

#### Acordes

Todo acorde que toca el ejercicio está hecho de terceras, apiladas
desde la fundamental (ver [Acordes](../theory/chords.md)). Las nueve
clases, sobre Do:

| Botón | Nombre | Símbolo | Notas | Terceras, desde abajo |
|-------|--------|---------|-------|-----------------------|
| maj | mayor | C | Do Mi Sol | mayor, menor |
| m | menor | Cm | Do Mi♭ Sol | menor, mayor |
| dim | disminuido | Cdim | Do Mi♭ Sol♭ | menor, menor |
| aug | aumentado | Caug | Do Mi Sol♯ | mayor, mayor |
| 7 | séptima de dominante | C7 | Do Mi Sol Si♭ | mayor, menor, menor |
| maj7 | séptima mayor | Cmaj7 | Do Mi Sol Si | mayor, menor, mayor |
| m7 | séptima menor | Cm7 | Do Mi♭ Sol Si♭ | menor, mayor, menor |
| m7b5 | séptima semidisminuida | Cm7b5 (Cø7) | Do Mi♭ Sol♭ Si♭ | menor, menor, mayor |
| dim7 | séptima disminuida | Cdim7 (C°7) | Do Mi♭ Sol♭ Si♭♭ | menor, menor, menor |

Dos acordes que a menudo se enseñan pronto quedan **fuera a propósito**:
los acordes de sexta y los suspendidos. C6 es Do Mi Sol La, exactamente
las mismas teclas que Am7 (La Do Mi Sol); Csus2 (Do Re Sol) tiene las
teclas de Gsus4 (Sol Do Re). Tocados solos, nada los distingue: lo que
hace que uno sea C6 y el otro Am7 es el bajo y la música de alrededor.
Un examen que los preguntara marcaría mal respuestas correctas. Una
prueba comprueba que ningún par de las nueve clases comparte notas
sobre dos fundamentales cualesquiera.

Algunas escrituras sorprenden al principio: **Si♭♭** en Cdim7, **Do♭**
en Fdim. No son errores. La séptima de un acorde de C es alguna clase
de Si (Do Re Mi Fa Sol La Si, siete nombres), y una séptima disminuida
está un semitono por debajo de la séptima menor Si♭, así que se escribe
Si doble bemol, aunque sea la tecla de La. Del mismo modo, la quinta de
F es alguna clase de Do, y una quinta disminuida está un semitono por
debajo de Do: Do♭, la tecla de Si. La explicación lo dice cada vez que
escribe una nota con un nombre que no es el habitual de su tecla, porque
el nombre dice qué función cumple la nota en el acorde.

**Inversiones.** Un acorde está *invertido* cuando la nota más grave no
es su fundamental: la tercera (primera inversión), la quinta (segunda
inversión) o la séptima (tercera inversión). El símbolo nombra el bajo
después de una barra: C/E es Do mayor con Mi en el bajo (ver
[Inversiones](../theory/chords.md#inversiones)). Un acorde invertido
conserva su clase, así que la respuesta no cambia, pero suena menos
asentado y cuesta más nombrarlo; para eso está la casilla
**Inversiones**.

Dos clases son especiales. Las notas del acorde aumentado están a la
misma distancia (una tercera mayor entre cada una, y una tercera mayor
desde la de arriba de vuelta a la fundamental), y lo mismo las de la
séptima disminuida (una tercera menor). Pon cualquiera de sus notas en
el bajo y obtienes exactamente las mismas teclas que el acorde en
estado fundamental sobre *esa* nota: Caug/E es también Eaug (Mi Sol♯
Si♯, donde Si♯ es la tecla de Do). El oído no puede saber qué nota es
la fundamental, así que la explicación de un acorde aumentado o de
séptima disminuida invertido dice qué otro acorde es también. La clase
sigue siendo segura, así que la pregunta sigue siendo justa.

#### Progresiones

Los números y sus colores vienen de las teclas de armonía (ver [Las
teclas de armonía](../theory/numerals.md#las-teclas-de-armonía)):
mayúsculas para acordes mayores, minúsculas para menores, ° para
disminuidos, y tres funciones, casa (azul), fuera (verde) y tensión
(naranja). En Do mayor:

```text
  I    ii    iii   IV    V     vi    vii°
  C    Dm    Em    F     G     Am    Bdim
```

Una tonalidad menor se lee `i ii° bIII iv V bVI bVII`. Los números
cuentan a lo largo de la escala *mayor* desde la misma tónica, así que
los acordes sobre la tercera, la sexta y la séptima rebajadas llevan un
bemol: en La menor, C es bIII. Y V es mayor, como en casi toda la
música en tonalidad menor: E mayor en La menor, con el Sol♯ que tira
hacia el La. Con **Séptimas** las teclas pasan a ser `Imaj7 ii7 iii7
IVmaj7 V7 vi7 viiø7` en mayor e `i7 iiø7 bIIImaj7 iv7 V7 bVImaj7 bVII7`
en menor.

**Cómo se hace una progresión.** Empieza en I. Cada acorde siguiente
suele ser uno de los movimientos que las teclas de armonía sugieren
después del acorde anterior (la misma lista detrás de sus tenues pistas
amarillas, ver [Qué viene
después](../theory/numerals.md#qué-viene-después-el-conocimiento-como-datos)),
el movimiento más habitual más a menudo. Una vez de cada cuatro es en
cambio cualquier acorde elegido, para que también se oigan los acordes
a los que rara vez llevan los movimientos habituales, como iii. Nunca
toca el mismo acorde dos veces seguidas, vuelve a I solo al final, y
termina en I o en V: un punto final, o una semicadencia, una pregunta
que espera su respuesta.

**Qué toca la banda.** Cada acorde dura un compás de cuatro pulsos. El
piano lo toca en los pulsos 1 y 3, cada acorde dispuesto cerca del
anterior, como se mueven las manos de un pianista (ver [Tocar una
tecla](../theory/numerals.md#tocar-una-tecla)). El bajo toca la
fundamental en los mismos pulsos, en su octava grave. La batería lleva
el tiempo: bombo en 1 y 3, caja en 2 y 4, charles en cada corchea. El
último acorde se sostiene todo el compás sobre un platillo. Con el
**Piano** solo, el piano también toca la fundamental debajo del acorde:
sin bajo, una disposición cerrada podría oírse como una inversión.

**Pistas para escuchar.**

- **Sigue el bajo.** Toca las fundamentales, así que su línea *es* la
  progresión. Una fundamental que baja una quinta (de V a I, de ii a V)
  suena a llegada.
- **I y V son la pareja más fácil.** V es tensión, I es reposo; V casi
  pide I.
- **IV y ii llevan los dos fuera y hacia V.** Comparten dos notas (Fa y
  La en Do mayor: Fa La Do y Re Fa La), así que suenan parecidos; ii es
  el más oscuro.
- **vi es una casa triste.** Comparte dos notas con I (Do Mi Sol y La Do
  Mi), y lo mismo iii (Mi Sol Si). Si un acorde suena a casa pero menor,
  probablemente es vi.
- **Escuchar la tonalidad** cada vez que pierdas de vista dónde está la
  casa.

### Cómo está construido

El trabajo se reparte según las líneas que la app ya tiene.
`lineup-core` hace las preguntas y sus explicaciones y no sabe nada de
ventanas ni de sonido. `lineup-audio` toca unas notas aparte de la
canción. `lineup-app` dibuja la ventana y une las dos cosas.

#### Una pregunta a partir de un número

Cada pregunta sale de una **semilla** de 64 bits:

```rust
pub fn new(settings: &IntervalSettings, seed: u64) -> Self {
    let mut random = Random(seed);
    let intervals = settings.intervals.places(Interval::ALL.len());
    let directions = settings.directions.places(Direction::ALL.len());
    let interval = Interval::ALL[intervals[random.below(intervals.len())]];
    let direction = Direction::ALL[directions[random.below(directions.len())]];
    let low = Note::new(INTERVAL_LOWEST + random.below(INTERVAL_SPAN) as u8);
    IntervalQuestion {
        low,
        interval,
        direction,
    }
}
```

`Random` es el pequeño generador SplitMix64 con el que barajan las
series dodecafónicas (ver [Qué hace interesante a una
serie](../theory/twelve-tone.md#qué-hace-interesante-a-una-serie)):
la misma semilla da siempre los mismos números, así que la misma
pregunta. Eso simplifica las pruebas, porque pueden hacer miles de
preguntas y comprobar cada una, y una semilla que falla se puede
repetir exactamente.

La ventana toma su primera semilla del reloj, y cada una de las
siguientes de `next_seed`:

```rust
pub fn next_seed(seed: u64) -> u64 {
    Random(seed).next()
}
```

¿Por qué no sumar simplemente la constante del propio generador,
`0x9E37_79B9_7F4A_7C15`? Porque eso es exactamente lo que el generador
se hace a sí mismo para cada número: la pregunta siguiente empezaría un
número más adelante en la serie de la anterior, así que su primer
número (la clase de un acorde) sería el segundo de la pregunta anterior
(su fundamental). Después de un Do, un Mi o un La♭, el acorde siguiente
habría sido siempre mayor. Una semilla tomada de la *salida* del
generador empieza números nuevos; la prueba
`one_question_does_not_give_the_next_away` hace 2.000 acordes seguidos
y comprueba que el acorde después de un Do puede ser de cualquiera de
las cuatro clases. Si la pregunta es igual a la anterior, la ventana lo
intenta otra vez, hasta ocho veces, así que rara vez sale la misma
pregunta dos veces seguidas.

#### Un conjunto de respuestas en dieciséis bits

Qué intervalos o acordes se preguntan es un conjunto de posiciones en
una lista. La lista es corta (doce intervalos como mucho), así que el
conjunto es un `u16`, un bit por posición, como un conjunto de clases
de altura son doce bits (ver [La teoría de conjuntos en doce
bits](../theory/sets.md#la-teoría-de-conjuntos-en-doce-bits)):

```rust
pub struct Choices(u16);
```

```rust
pub fn has(self, place: usize) -> bool {
    self.0 & (1 << place) != 0
}

/// Adds the answer at `place`, or takes it away. The last one left
/// stays: there is always something to ask.
pub fn toggle(&mut self, place: usize) {
    let toggled = self.0 ^ (1 << place);
    if toggled != 0 {
        self.0 = toggled;
    }
}
```

`^` (o exclusivo) invierte un bit. Negarse a apagar el último hace que
siempre se pueda hacer una pregunta, sin un caso especial en ningún
otro sitio.

#### Cues: qué tocar, en segundos

Una pregunta no se toca a sí misma: devuelve **cues**, valores simples
que dicen quién toca qué, cuándo y con qué fuerza.

```rust
/// One note of a question: who plays it, when, for how long and how loud.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cue {
    pub part: Part,
    /// When it starts, in seconds from the start of the question.
    pub start: f64,
    /// How long it is held, in seconds.
    pub length: f64,
    pub note: Note,
    pub velocity: u8,
}
```

Dejar el sonido fuera de `lineup-core` permite a las pruebas comprobar
*qué* se tocaría sin tocarlo: que cada compás de una progresión toca su
acorde sobre su fundamental, que el piano solo añade la fundamental,
que la nota más grave de una pregunta de acorde es el bajo que nombra
su símbolo.

Las respuestas con las que comparar usan la misma idea.
`cues_for(interval)` toca otro intervalo desde la misma nota de abajo
(o, hacia abajo, desde la misma nota de arriba), y `cues_as(quality)`
otra clase de acorde sobre la misma fundamental con la misma nota del
acorde en el bajo. Así, cuando haces clic en una respuesta equivocada
después de responder, oyes exactamente lo que dijiste al lado de lo que
sonó.

#### Una frase en el hilo de audio

La canción está hecha de patrones y clips sobre una línea de tiempo.
Una pregunta no: son unas pocas notas que empiezan *ahora*, en
instrumentos propios, mientras los canales de la canción se quedan como
están. Así que `lineup-audio` tiene una `Phrase`: sus instrumentos y
sus notas, con sus tiempos en frames.

El hilo de audio no debe reservar ni liberar memoria (ver [La regla de
oro del hilo de
audio](../program/threads.md#la-regla-de-oro-del-hilo-de-audio)).
Así que el hilo de la interfaz construye la frase entera, instrumentos
incluidos, y la envía en un solo comando:

```rust
PlayPhrase(Box<Phrase>),
```

El `Box` importa. Un `Command` es tan grande como su variante más
grande, y la prueba `commands_stay_small` los mantiene en 80 bytes o
menos para que la cola siga siendo pequeña. Una `Phrase` tiene dos
`Vec` y unos contadores, unos 80 bytes propios; dentro de un `Box`, el
comando lleva solo un puntero.

En el hilo de audio, la frase solo empieza cada nota cuando le llega su
momento y suma sus instrumentos a la mezcla:

```rust
pub fn next_sample(&mut self) -> f32 {
    while let Some(note) = self.notes.get(self.next)
        && note.start <= self.clock
    {
        if let Some(instrument) = self.instruments.get_mut(note.instrument) {
            instrument.note_on(note.pitch, note.velocity, note.gate, self.sample_rate);
        }
        self.next += 1;
    }
    self.clock += 1;
    let mix: f32 = self
        .instruments
        .iter_mut()
        .map(Instrument::next_sample)
        .sum();
    mix * VOLUME
}
```

Las notas están ordenadas por comienzo, así que `next` solo avanza: una
comparación por muestra cuando no empieza nada. `while let ... && ...`
es una cadena de `let` en un bucle: sigue mientras *haya* una nota
siguiente *y* le toque. Cada nota recibe su duración (`gate`) al
empezar, y el instrumento la suelta solo, así que la frase no necesita
una lista de notas que parar.

Cuando llega una frase nueva, la vieja vuelve al hilo de la interfaz
por la cola de basura para liberarse. Una frase que ya tocó su última
nota espera dos segundos, para que el sonido se apague, y luego se va
por el mismo camino. `StopPhrase` (que se envía al cerrar la ventana o
cambiar de pestaña) suelta todas las notas y descarta las que aún no
empezaron.

Los instrumentos salen de `build_instrument`, la misma función que usan
los canales: el piano es un canal con el programa General MIDI 0 (Piano
de cola acústico), el bajo el programa 33 (Bajo eléctrico (dedos)),
la batería el kit integrado. Así que con **Instrumentos del SoundFont**
encendido (**Edición > Ajustes… > Audio**), las preguntas suenan con el
piano, el bajo y la batería del SoundFont, como la banda (ver
[SoundFonts y General MIDI](../sound/soundfonts.md)).

#### La ventana: acciones hacia fuera

La ventana no tiene el motor de audio. Devuelve lo que quiere oír como
una lista de acciones, y la app las toca:

```rust
pub enum EarAction {
    /// Play these notes now, in place of anything it played before.
    Play(Vec<Cue>),
    /// Stop what it is playing.
    Stop,
}
```

Dentro, la pregunta y las respuestas dadas hasta ahora son un solo
enum, `Asked`. Dibujar las respuestas necesita cambiarlas *y* llamar a
métodos de la ventana (para sumar a la puntuación, para tocar un
sonido), algo que Rust no permite mientras las respuestas están
prestadas desde `self`. La ventana saca la pregunta de `self` mientras
la dibuja, y la vuelve a poner:

```rust
match self.asked.take() {
    Some(Asked::Interval {
        question,
        mut answer,
    }) => {
        self.interval_answers(ui, &question, &mut answer, actions);
        self.asked = Some(Asked::Interval { question, answer });
    }
    // ... chords and progressions the same way
}
```

`Option::take` deja `None` en su lugar, así que durante esas pocas
líneas `self` y la pregunta son dos valores separados, y los dos se
pueden tomar prestados libremente.

Las opciones de la ventana se recuerdan como los demás ajustes: un
struct con `#[serde(default)]` guardado en la memoria de egui, que
eframe guarda con el tamaño de la ventana.

## Cómo se prueba

El metrónomo y la cuenta de entrada se prueban en el motor de audio,
frame a frame: `the_count_in_comes_before_the_music`,
`the_count_in_runs_on_into_a_bar_started_in_the_middle` y
`stopping_ends_the_count_in`. Las vueltas se prueban dos veces:
`laps_count_the_loops_played_to_the_end`, en el secuenciador, toca un
bucle de cuatro pasos con saltos, reinicios y un bucle acortado, y
`laps_count_the_times_the_loop_came_round` y
`lap_tempos_start_on_the_loops_first_step`, en el motor, comprueban la
cuenta y que una velocidad nueva empieza justo con el bucle.

En la app, `the_trainer_moves_on_as_the_loop_comes_round`,
`the_next_speed_is_asked_for_ahead_of_the_lap_it_starts_on` y
`the_trainer_can_play_each_speed_more_than_once` manejan el entrenador
con cuentas de vueltas inventadas, incluida una ventana que se perdió
unos fotogramas y un hilo de audio que se reinició;
`taps_set_the_tempo_of_their_beats` y `only_the_last_taps_count`
comprueban el marcado del tempo, en 4/4 y en 6/8; y
`the_trainer_moves_towards_its_target_and_stays_there` las velocidades
70, 75, … 100, un paso que no divide el camino y un objetivo por debajo
del comienzo.

El núcleo del entrenamiento auditivo tiene 22 pruebas. Las que vigilan
la teoría:

- **Cada intervalo desde cada nota**
  (`every_interval_is_spelled_as_its_name_says_from_every_note`): para
  las 12 notas de abajo y los 12 intervalos, las dos notas escritas son
  las teclas correctas, ninguna necesita doble sostenido ni doble
  bemol, y el nombre calculado a partir de las dos notas escritas es el
  del intervalo.
- **Escrituras a mano** (`intervals_are_written_the_simple_way`): Do♯ Mi,
  Re♭ Fa, Fa Si, Mi La♯, Do Fa♯, Mi Do, Mi♭ Re, Fa♯ Fa♯, Si♭ Re♭, Si♭ Mi,
  La♭ Mi♭.
- **Acordes contra el analizador**
  (`the_app_names_a_chord_in_root_position_as_the_question_does`): 500
  preguntas de acordes en estado fundamental pasan por el analizador
  que usa la app para nombrar lo que tocas en un teclado MIDI, que debe
  nombrar la misma clase. No invertidos: el analizador oye con razón
  Am7/C como C6.
- **Cada pregunta de acorde toca su acorde con el bajo correcto**, en
  2.000 semillas con las inversiones encendidas.
- **Explicaciones escritas** (`chords_are_spelled_from_their_root`):
  Bdim7, E♭maj7/G, Fdim/C♭, el Si♭♭ de Cdim7, Caug/E (también Eaug),
  Ddim7/C♭ (también Bdim7).
- **Los números** (`numerals_are_those_of_the_harmony_keys`) coinciden
  con las teclas de armonía en mayor y en menor, tríadas y séptimas.
- **Las progresiones** empiezan en I, nunca repiten un acorde, vuelven
  a I solo al final, terminan en I o V y usan todos los acordes
  elegidos; cada compás toca su propio acorde sobre su fundamental.
- **Salen todas las tonalidades, fundamentales y notas de abajo**: 24
  tonalidades, 12 fundamentales, 16 notas de abajo.

La frase tiene cuatro pruebas propias en `phrase.rs` (las notas empiezan
a tiempo en cualquier orden en que lleguen, una frase termina cuando se
ha apagado, detener descarta las notas que faltan, las notas de un
instrumento que no existe se dejan fuera), y tres en el motor: una frase
suena sin canción cargada y se descarta después de apagarse, una frase
nueva ocupa el lugar de la anterior, y una frase detenida no toca nada
más.
