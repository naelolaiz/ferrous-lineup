# Practicar con la banda

Archivos: [`crates/lineup-audio/src/metronome.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-audio/src/metronome.rs),
[`crates/lineup-audio/src/engine.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-audio/src/engine.rs),
[`crates/lineup-core/src/meter.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/meter.rs),
[`crates/lineup-core/src/band.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/band.rs),
[`crates/lineup-core/src/history.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/history.rs),
[`crates/lineup-app/src/app/practice.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/practice.rs)
y [`crates/lineup-app/src/chord_sheet/practice.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/chord_sheet/practice.rs)

Una banda que toca tu chord sheet es más útil cuando tocas con ella. Este
cambio añade lo que un músico que practica con una grabación busca a
mano: un **count-in**, un **metrónomo**, una **velocidad** menor, **tap
tempo**, una manera de quitarle una parte a la banda y tocarla **tú
mismo**, y hacer **loop** en los compases en los que estás trabajando.

![El chord sheet con su fila de práctica: Count-in y Metronome encendidos, Speed 80%, Tap, «You play» con Bass encendido, «Loop bars 2–3» encendido y «✖ Stop looping»; los compases 2 y 3 sombreados, con una línea verde a lo largo de su parte superior, y el compás 3 enmarcado en blanco mientras suena](../images/step-9-practice-sheet.png)

## Probarlo

Abre el chord sheet (**F5**). Debajo de los botones Band y Progression
hay ahora una segunda fila:

- **Count-in**: cuando pulsas Play, primero suena un compás de clics, y
  la música empieza en el siguiente «uno». Empieza en medio de un compás
  y la cuenta sigue hasta donde está el playhead: desde el pulso 3 de un
  compás de 4/4 oyes «1 2 3 4 1 2», y la música entra en el 3, así que
  el pulso nunca salta.
- **Metronome**: un clic en cada pulso mientras suena la música. El
  primer pulso de cada compás es más alto (un La, 1760 Hz) que los demás
  (el Mi una quinta más abajo), para que siempre sepas dónde está el
  «uno».
- **Speed**: a qué velocidad toca la banda, como porcentaje del tempo de
  la canción, de 25% a 200%. La canción conserva su propio tempo: la
  caja TEMPO sigue diciendo 110, con «TEMPO 80%» encima, y una canción
  guardada o una exportación WAV va a velocidad completa. El **Speed
  trainer** de al lado sube la velocidad un poco cada vez que el loop da
  la vuelta (mira [El entrenador de velocidad](speed-trainer.md)).
- **Tap**: haz clic en él al pulso unas cuantas veces y el tempo de la
  canción se vuelve el tuyo. Los taps seguidos son un solo paso de
  deshacer, «Undo tap tempo».
- **You play: Piano, Bass, Drums**: la banda deja fuera esa parte y la
  tocas tú. Para el piano y el bajo, tu teclado (MIDI, o el teclado de
  escritura) cambia al sonido de esa parte.
- **Loop bars**: haz clic en un compás, Shift+clic en otro (o mantén
  Shift y pulsa una flecha), y **🔁 Loop bars 2–3** los toca una y otra
  vez. **✖ Stop looping** toca toda la canción otra vez desde donde
  estás.

Count-in, Metronome y Speed también están en el menú nuevo **Practice**,
así que funcionan sin el chord sheet, sobre cualquier canción o patrón.
El count-in y el metrónomo se recuerdan para la próxima vez; la
velocidad empieza en 100% cada vez que la aplicación arranca, para que
una canción nunca esté lenta por sorpresa.

![El menú Practice: Count-in y Metronome marcados, Speed (80%) con su lista de porcentajes](../images/step-9-practice-menu.png)

## De dónde salen los clics

Los clics no son un canal de la canción. Si lo fueran, se guardarían con
ella, se exportarían a cada archivo WAV, los silenciaría el mixer y la
banda los movería cuando reescribe sus partes. Los hace el propio hilo
de audio, que ya sabe cuándo empieza cada paso.

Lo que necesita saber es *qué clic tocar en cada paso*. Eso depende del
compás, y el compás puede cambiar en medio de la canción ([Cualquier
compás, y métricas irregulares](../step-8/any-time-signature.md)), así
que el hilo de la interfaz lo calcula y envía la respuesta como una
lista, una entrada por paso:

```rust
pub enum Click {
    None,
    Beat,
    Bar,
}

impl TimeSignature {
    pub fn click(self, step: usize) -> Click {
        if self.is_bar_start(step) {
            Click::Bar
        } else if self.is_beat_start(step) {
            Click::Beat
        } else {
            Click::None
        }
    }
}
```

«Beat» significa el pulso *sentido*, el que marcas con el pie, que
`beat_starts` ya conocía del paso 8:

| Compás | Clics en un compás | Por qué |
|------|-----------------|-----|
| 4/4 | 1 2 3 4 | cuatro negras |
| 3/4 | 1 2 3 | un vals |
| 6/8 | 1 2 | dos negras con puntillo: seis corcheas sentidas como dos grupos de tres |
| 2+2+3/8 | 1 2 3 | tres pulsos, el último más largo |

Así que un metrónomo en 6/8 no hace clic seis veces por compás. Una
giga, o Silent Night en 6/8, se siente en dos, y hacer clic en cada
corchea lo escondería.

La lista de la canción viene de `Project::played_clicks`, que cuenta los
compases como la canción se *toca*: una sección en 3/4 que se repite
está en 3/4 en las dos vueltas, y un loop empieza su cuenta en el primer
compás del loop. La lista del patrón seleccionado viene de
`Project::pattern_clicks`. Se envían como dos comandos nuevos,
`SetSongClicks` con la canción y `SetPatternClicks` cuando se selecciona
otro patrón o cambian su longitud o su compás.

> **Rust: la lista vieja vuelve.** Cuando llega una lista nueva, el hilo
> de audio la intercambia con `std::mem::replace` y devuelve el `Vec`
> viejo al hilo de la interfaz como `Garbage::Clicks`, para liberarlo
> allí. Liberar memoria puede tomar un lock dentro del allocator, y el
> hilo de audio nunca debe esperar ([El hilo de audio, ya
> adulto](../step-2/realtime.md)).

### El clic en sí

`ClickVoice` en `metronome.rs` toca una onda senoidal corta que se
apaga:

```rust
const RING: f32 = 0.03;

decay: (-6.9 / (RING * sample_rate)).exp(),
```

Cada frame el nivel se multiplica por `decay`. Después de `RING`
segundos (30 ms, 1440 frames a 48 kHz) se ha multiplicado por `decay`
1440 veces, lo que es e<sup>−6.9</sup> ≈ 0.001, una milésima de donde
empezó: −60 dB, demasiado bajo para oírse. Lo bastante largo para oír
una altura, lo bastante corto para acabarse bastante antes del siguiente
pulso incluso a 300 BPM.

## El count-in: un segundo reloj

El hilo de audio tiene un `Sequencer`, el reloj que dice cuándo empieza
cada paso ([El secuenciador: temporización a la
muestra](../step-2/sequencer.md)). También mueve el playhead, que la
interfaz lee para dibujar dónde está la canción. Durante un count-in la
canción no ha empezado, así que el playhead no debe moverse. El
count-in recibe un reloj propio, `counter`, y un pequeño registro de
cuánto ha avanzado:

```rust
struct CountIn {
    bar: usize,     // where the bar it counts starts, in the clicks
    length: usize,  // how many steps that bar has
    steps: usize,   // how many steps the whole count lasts
    counted: usize, // how many have gone by
}
```

Play lo pone en marcha:

```rust
fn begin_play(&mut self) {
    if !self.count_in {
        self.sequencer.start_at(self.start);
        self.playing = true;
        return;
    }
    // …
    let (bar, length) = count_in_bar(self.loop_clicks(), self.start);
    let into_the_bar = self.start.saturating_sub(bar) % length;
    self.counter.set_tempo(self.tempo_of_step(self.start));
    self.counter.reset();
    self.counting = Some(CountIn {
        bar,
        length,
        steps: length + into_the_bar,
        counted: 0,
    });
}
```

`count_in_bar` mira hacia atrás desde el playhead hasta el último
`Click::Bar`, y hacia delante hasta el siguiente, para encontrar el
compás en el que está el playhead. La cuenta dura ese compás entero
*más* los pasos desde su principio hasta el playhead: desde el paso 6 de
un compás de 16 pasos, 16 + 6 = 22 pasos, «1 2 3 4 1 2», y la música
empieza en el paso 6, medio pulso dentro del pulso 2. Cuenta al tempo en
el que empezará la música, así que una canción que empieza despacio se
cuenta despacio.

Luego, en cada frame:

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
        return;
    }
    let step = count.bar + count.counted % count.length;
    self.click.strike(click_at(self.loop_clicks(), step));
    count.counted += 1;
    self.counting = Some(count);
}
```

Cuando la cuenta termina, el secuenciador de la canción arranca *en ese
mismo frame*: el bucle de frames atiende el count-in antes de pedirle un
paso al secuenciador de la canción, así que la primera nota cae
exactamente un compás después del primer clic. La prueba
`the_count_in_comes_before_the_music` lo comprueba al frame: clics en
los frames 0, 24000, 48000 y 72000 a 120 BPM, la primera nota en el
96000.

> **Rust: por qué `count` se pasa por valor.** `CountIn` deriva `Copy`,
> así que `if let Some(count) = self.counting` hace una copia, y
> `count_in_frame` recibe la copia. Si tomara una referencia a
> `self.counting`, ese préstamo de `self` seguiría vivo mientras la
> función llama a `self.counter.tick()` y a `self.click.strike(…)`, que
> necesitan `&mut self`: el borrow checker lo rechazaría. Trabajar sobre
> una copia y escribirla de vuelta al final (`self.counting =
> Some(count)`) es la salida habitual para un estado pequeño como este.

Stop, durante el count-in, lo termina (`stop` pone `counting` en
`None`). Mover el playhead o cambiar entre el modo patrón y el modo
canción durante la cuenta la vuelve a empezar, contando el compás en el
que ahora empezará la reproducción.

Las teclas tocadas durante el count-in no se graban: la posición de la
canción es «none» hasta que la música empieza, y la grabación solo
conserva las notas que tienen posición.

## La velocidad

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

Todo comando que lleva un tempo pasa por ella: `SetTempo` después de una
edición, y el tempo de cada paso de la canción, así que una canción que
se ralentiza al final sigue ralentizándose, desde el 80% de su tempo.
Una exportación WAV construye sus comandos a velocidad `1.0`, por eso
nunca se ralentiza.

## Tap tempo

El tap tempo es aritmética sobre los tiempos de los clics. Taps a los
10.0, 10.5, 11.0 y 11.5 segundos son tres pulsos en 1.5 segundos:

> BPM = 60 × pulsos ÷ segundos = 60 × 3 ÷ 1.5 = 120

Usar el primer y el último tap de la racha, en lugar de solo los dos
últimos, promedia la irregularidad de una mano humana. Solo cuentan los
últimos ocho taps, así que se sigue un tempo que se asienta mientras
haces tap, y una pausa de más de dos segundos empieza una racha nueva.
El resultado se redondea a un BPM entero.

Cada tap es un clic, y cada clic normalmente empieza un paso de deshacer
nuevo (mira [Deshacer y rehacer: un historial de
copias](../step-5/history.md)). Deshacer un tap de tempo de uno en uno
sería cansador, así que `History` ganó un método:

```rust
pub fn reopen(&mut self, label: &'static str) -> bool {
    if self.redo.is_empty() && self.undo_label() == Some(label) {
        self.open = true;
    }
    self.open
}
```

Desde el tercer tap de una racha, la aplicación reabre el paso «tap
tempo» antes de cambiar el tempo, así que toda la racha es un paso. Solo
lo hace cuando el último paso *es* «tap tempo» y nada se ha deshecho
desde entonces: nunca une un tap a otro cambio.

## Tú tocas

Quitar una parte a la banda silencia la pista de ese músico en el
playlist:

```rust
pub fn set_you_play(&mut self, player: Player, you: bool) -> bool {
    let Some(track) = self.player_track(player) else {
        return false;
    };
    self.playlist.tracks[track].muted = you;
    true
}
```

Silenciar, no quitar, importa. La banda sigue escribiendo la parte
cuando cambias un acorde, así que cuando vuelves a hacer clic en el
botón toca lo que el sheet dice *ahora*. Y como la huella de una parte
deja fuera la pista en la que están sus clips ([Una parte que editas
sigue siendo tuya](band.md#una-parte-que-editas-sigue-siendo-tuya)), silenciar no
cuenta como una edición: la parte no queda congelada.

Para el piano y el bajo, la aplicación también selecciona el canal de
ese músico, el que toca tu teclado: te haces cargo del bajo *con* el
sonido del bajo. La batería toca en varios canales (kick, snare,
hat…), así que no hay un único canal que seleccionar para ella.

## Hacer loop en los compases que eliges

El sheet recuerda, además del compás seleccionado, un `anchor`: el otro
extremo de una selección hecha con Shift.

```rust
fn select_to(&mut self, bar: usize, shift: bool) {
    self.anchor = match (shift, self.anchor, self.selected) {
        (false, ..) => None,
        (true, Some(anchor), _) => Some(anchor),
        (true, None, selected) => selected,
    };
    self.selected = Some(bar);
}
```

El primer Shift+clic conserva como anchor el compás que estaba
seleccionado antes; los siguientes mueven solo el otro extremo, como en
un editor de texto. Delete vacía todos los compases seleccionados, en un
solo paso de deshacer.

**Loop bars** convierte la selección en la región de loop del playlist,
que el modo canción ya sabía tocar ([La estructura de la canción:
secciones, repeticiones y loops](../step-8/song-structure.md)), cambia
al modo canción y se mueve al principio del loop. Los compases más allá
del final de la canción no pueden entrar en loop: no hay nada allí para
tocar. En el sheet, los compases que hacen loop reciben una línea verde
a lo largo de su parte superior, el color del loop en el playlist.

## Ejercicios

1. Enciende el metrónomo sobre Silent Night, una demo en 6/8. ¿Cuántos
   clics hay en un compás, y dónde cae el alto? Ahora Greensleeves, en
   3/4: las mismas seis corcheas por compás, contadas distinto.
2. Con el count-in encendido, toca una canción y pausa (el botón ❚❚) a
   mitad de un compás, y vuelve a pulsar Play. Escribe lo que oyes antes
   de la música. ¿Por qué es mejor que contar un compás entero y luego
   empezar a mitad del siguiente?
3. Pon la velocidad al 50% y exporta un archivo WAV. ¿Cuánto dura,
   comparado con la canción a velocidad completa? ¿Por qué?
4. Haz tap 4 veces a un ritmo parejo, y luego 4 veces más rápido sin
   parar. ¿Qué tempo obtienes, y por qué no el promedio de las 8?
5. Hazte cargo del bajo, y cambia un acorde. Devuelve el bajo: ¿está el
   acorde nuevo en su parte?
6. (Rust) `ClickVoice::strike(Click::None)` vuelve enseguida. ¿Qué
   oirías si pusiera `level` en 0?
7. (Rust) En `begin_play`, ¿qué sale mal si no se llama a `self.stop()`
   cuando llega Play mientras la canción ya está sonando?
