# El mixer: paneo, un envío de reverb, medidores de nivel y el instrumento en cada línea

Archivos: [`crates/lineup-app/src/app/mixer.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/mixer.rs),
[`crates/lineup-app/src/meters.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/meters.rs),
[`crates/lineup-audio/src/levels.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-audio/src/levels.rs),
[`crates/lineup-audio/src/engine.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-audio/src/engine.rs),
[`crates/lineup-audio/src/reverb.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-audio/src/reverb.rs),
[`crates/lineup-core/src/project.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/project.rs)
y [`crates/lineup-core/src/file.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/file.rs)

El mixer tenía un volumen, un silencio y un solo para cada canal, y una
reverberación para todo. Cada canal sonaba en el medio, desde ambos
altavoces por igual, y cada canal iba a la misma sala. Con una banda de
cinco o seis músicos, y orquestas enteras en las demos, no es mucho con
lo que dar forma a un sonido.

Ahora cada línea del mixer también tiene:

- **Instrument**: qué toca el canal. Haz clic en él para elegir otro,
  como en el channel rack.
- **Level**: un medidor de cuán fuerte está sonando.
- **Pan**: dónde se sienta entre los altavoces, de L100 a R100.
- **Reverb**: cuánto de él entra en la reverberación.

El master tiene un medidor para cada altavoz.

![El mixer tocando El Choclo con la reverberación encendida: las Keys con pan a L30, el Lead (un acordeón) a R30, y la batería enviando un 40% a la reverberación.](../images/step-9-mixer.png)

## Probarlo

1. Abre **File > Demos > El Choclo**, pulsa **F9** para el mixer y
   **Space** para tocar. Los medidores se mueven con cada canal.
2. Arrastra el **Pan** de un canal a la izquierda: sale solo del altavoz
   izquierdo en L100. **Doble clic** en el slider para devolverlo al
   medio (**Do**). También puedes escribir en él: `L30`, `R100`, `C`.
3. Marca **Reverb** arriba y pon **Wet / dry** al 50%. Luego arrastra el
   **Reverb** del Bass al 0%: suena dry, cerca, mientras el resto de la
   banda se queda en la sala.
4. Haz clic en el **Instrument** del canal Keys, por ejemplo «Acoustic
   Grand Piano», y elige otro de su lista de familias. Es un paso de
   deshacer, como cualquier cambio en el mixer.

## Pan: dos altavoces, un canal

El pan es un número de −1 (todo a la izquierda) a 1 (todo a la
derecha), con 0 en el medio. El mixer lo convierte en una ganancia para
cada altavoz:

```rust
/// How loud it is in the left and the right speaker for its pan: both
/// full in the middle, and the far side fading out as it moves away.
pub fn pan_gains(pan: f32) -> [f32; 2] {
    [(1.0 - pan).min(1.0), (1.0 + pan).min(1.0)]
}
```

```text
pan          −1.0  −0.5   0.0   0.5   1.0
izquierda    1.0   1.0   1.0   0.5   0.0
derecha      0.0   0.5   1.0   1.0   1.0
```

En el medio ambos altavoces tocan el canal a todo volumen. Es
exactamente como sonaba cada canal antes de que hubiera pan, así que
una canción que guardaste antes suena igual que antes. Mover un canal a
un lado baja el **otro** altavoz, como hace el control de balance de un
estéreo.

Esta es una de varias **leyes de paneo**. Las mesas de mezcla usan a
menudo una ley de *potencia constante*, donde el medio toca cada
altavoz a unos 0.71 (−3 dB), para que el canal suene igual de fuerte
donde sea que se coloque. Con la ley que usa la aplicación, un canal
movido a fondo a la izquierda suena un poco más bajo que en el medio, ya
que solo un altavoz lo toca. La aplicación conserva el medio como
siempre fue; un ejercicio de abajo prueba la otra ley.

## Un envío de reverb: cuánto de cada canal está en la sala

Una reverberación imita una sala: el sonido vuelve de sus paredes, más
tarde y más suave, durante un segundo o más. Antes, toda la canción
entraba en la reverberación, y su control **Wet / dry** mezclaba la sala
(wet) con la canción tal cual era (dry). Ahora cada canal tiene un
**envío**, de 0 a 1, que dice cuánto de él entra en la sala.

Cada muestra de la mezcla se calcula ahora así, en el hilo de audio:

```rust
let wet = self.reverb.wet();
let mut dry = [0.0; 2];
let mut send = 0.0;
for channel in &mut self.channels {
    let volume = channel.volume.next_value();
    let sample = channel.instrument.next_sample() * volume;
    channel.peak = channel.peak.max(sample.abs());
    let amount = channel.reverb_send.next_value();
    let gains = lineup_core::Channel::pan_gains(channel.pan.next_value());
    let kept = sample * (1.0 - wet * amount);
    dry[0] += kept * gains[0];
    dry[1] += kept * gains[1];
    send += sample * amount;
}
// …
let stereo = self.reverb.next(dry, send, wet);
```

La suma `send` entra en la reverberación, que ahora devuelve solo la
sala, añadida al sonido dry. La línea que hay que mirar es `kept`: lo
que entra en la sala se quita del sonido dry, en la misma proporción que
antes. Comprueba los dos extremos:

- **Envío 1** (todo él, como cada canal nuevo): `kept = sample × (1 −
  wet)`, y la muestra entera va a la sala. Es exactamente la mezcla wet /
  dry de antes, así que la canción suena como antes.
- **Envío 0**: `kept = sample`, y nada va a la sala. El canal suena dry,
  como con la reverberación apagada, incluso mientras los demás están en
  la sala.

La reverberación, el volumen, el pan y el envío son todos `Smoothed`:
cuando mueves un slider, el valor se desliza al nuevo a lo largo de unos
pocos milisegundos en lugar de saltar, ya que un salto en medio de una
onda se oye como un clic.

## Medidores de nivel: del hilo de audio a la pantalla

Un medidor muestra cuán fuerte suena un canal **ahora**. El hilo de
audio lo sabe, ya que hace el sonido; el hilo de la ventana dibuja el
medidor. Los dos hilos corren a velocidades distintas: el hilo de audio
llena un bloque de unos cientos de muestras muchas veces por segundo, la
ventana dibuja unos 60 frames por segundo. Y el hilo de audio nunca debe
esperar a la ventana (mira [El hilo de audio, ya
adulto](../step-2/realtime.md)), así que un `Mutex` queda descartado.

### Picos en atómicos

El hilo de audio guarda la muestra más fuerte de cada canal a lo largo
de un bloque (`channel.peak` arriba). Al final del bloque sube un pico
compartido hasta ella, si es más fuerte. Una vez por frame, el medidor
toma el pico compartido y lo devuelve al silencio. Así no se pierde
ningún pico, por cómo se alineen bloques y frames, y ningún lado espera:

```rust
/// The peaks since the UI last took them, as the bits of `f32`s.
///
/// For numbers that are not negative, comparing their bits compares the
/// numbers, so `fetch_max` on the bits keeps the louder peak.
pub struct Levels {
    channels: [AtomicU32; MAX_CHANNELS],
    master: [AtomicU32; 2],
}

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

No hay un `AtomicF32` en la biblioteca estándar de Rust. Pero un `f32`
son 32 bits, y `to_bits` y `from_bits` lo convierten en un `u32` y de
vuelta sin cambiar un bit. Mejor aún, para los números que **no son
negativos**, el orden de los bits como enteros sin signo es el orden de
los números: el exponente va en los bits altos, la mantisa debajo. Así
que `fetch_max` sobre los bits, un «conserva el mayor» atómico,
conserva el pico más fuerte. Los picos son valores absolutos, nunca
negativos, y el `if` deja fuera a NaN (sus bits compararían como mayores
que cualquier número).

`swap(0, …)` lee el valor y pone 0 (los bits de 0.0) en su lugar en un
solo paso, así que un pico subido entre una lectura y un reinicio no
puede perderse. `Ordering::Relaxed` basta porque cada pico se sostiene
solo: nada más en memoria tiene que verse en cierto orden junto con él.

`std::array::from_fn(|_| AtomicU32::new(0))` construye el array, ya que
un `AtomicU32` no es `Copy` y `[AtomicU32::new(0); N]` no compila.

### Decibelios, y un medidor que cae despacio

El oído oye la sonoridad por cocientes, no por diferencias. El medidor
se dibuja por eso en **decibelios**, una escala de cocientes:

```text
decibelios = 20 × log10(level)      1.0 → 0 dB (escala completa)
                                    0.5 → −6 dB
                                   0.25 → −12 dB
                                  0.004 → −48 dB (el suelo del medidor)
```

Cada 6 dB hacia abajo parten el nivel a la mitad, donde sea que esté en
la escala. Un medidor se ilumina desde −48 dB a la izquierda hasta 0 dB
a la derecha: verde, ámbar desde −6 dB, rojo en la escala completa y por
encima. Los medidores del master se miden después del volumen del
master y antes del último paso de la mezcla, que redondea los picos por
encima de 1.0 con `tanh` en lugar de recortarlos. Rojo en el master
significa que ese redondeo está trabajando.

Un medidor que siguiera cada pico exactamente parpadearía demasiado
rápido para leerse. Así que **sube de golpe** a un pico nuevo y **cae**
24 dB por segundo:

```rust
pub fn update(&mut self, levels: &Levels, channels: usize, seconds: f32) {
    let fall = gain(-FALL_DB_PER_SECOND * seconds);
    self.channels.resize(channels, 0.0);
    for (index, shown) in self.channels.iter_mut().enumerate() {
        *shown = levels.take_channel(index).max(*shown * fall);
    }
    // … the same for the master
}
```

`seconds` es el tiempo desde el último frame. Caer 24 dB por segundo
significa multiplicar por `10^(−24 × seconds / 20)` cada frame, sea cual
sea la tasa de frames. Medio segundo después de un pico, el medidor lo
muestra 12 dB más abajo, a un cuarto aproximado.

egui solo dibuja un frame cuando pasa algo. Mientras un medidor sigue
cayendo, el mixer pide el siguiente frame enseguida (`request_repaint`);
cuando todos los medidores están oscuros, mira otra vez cada décima de
segundo (`request_repaint_after`), lo que basta para cazar las notas de
un teclado MIDI tocado con la canción detenida.

## La columna del instrumento

La columna Instrument solía ser un label. Para un canal de sintetizador
o de MIDI Out ahora es un menú con las mismas opciones que el menú
**Instrument** del rack: «None» para el sonido propio del Lineup Osc, y
luego los 128 instrumentos de General MIDI en sus dieciséis familias. El
rack y el mixer llaman a la misma función,
`channel_rack::instrument_choices`. Una batería integrada no tiene
instrumento que elegir, pero puede enviarse por MIDI desde su menú (mira
el [próximo capítulo](midi-out-menus.md)).

El mixer dibuja sus líneas dentro de closures que egui llama, y esas
closures ya toman prestadas partes de la aplicación. Así que el menú no
puede cambiar el proyecto mientras se dibuja. En su lugar empuja lo que
elegiste a una lista, y la aplicación lo aplica cuando la ventana
termina:

```rust
let mut picked = Vec::new();
// … inside the window, for each channel:
ui.menu_button(instrument(channel), |ui| {
    channel_rack::instrument_choices(ui, index, channel, &mut picked);
});
// … after the window:
for action in picked {
    self.apply(action);
}
```

Es el mismo patrón que usan el channel rack y el chord sheet: el código
de interfaz dice **qué** hiciste, como un valor (`RackAction::SetProgram
{ channel, program }`), y un lugar de la aplicación lo hace, como un
paso de deshacer.

## Guardado con la canción: al menos el formato 26

Un canal tiene ahora dos campos más:

```rust
#[serde(default, skip_serializing_if = "is_centred")]
pub pan: f32,
#[serde(default = "full_send", skip_serializing_if = "is_full_send")]
pub reverb_send: f32,
```

`skip_serializing_if` deja el campo fuera del archivo cuando es el valor
simple (0.0 y 1.0), y `default` lo rellena cuando falta. Así, una
canción cuyos canales están todos en el medio con un envío completo se
escribe exactamente como antes, en el formato que tenía, y las versiones
más viejas de la aplicación pueden abrirla igual. Solo una canción con
otra mezcla necesita al menos el formato 26, que `file::format_for`
calcula a partir de los canales ([el formato del archivo de
proyecto](../appendix-project-format.md)). Un archivo editado a mano con
un pan de −3 se rechaza con un mensaje, en lugar de tocar un canal más
fuerte que el completo.

## Las pruebas

En `render.rs`, `a_panned_channel_plays_more_on_its_side` renderiza un
clic en cinco panes y comprueba las muestras izquierda y derecha contra
la tabla de arriba. `a_channel_sent_nowhere_plays_dry_in_a_room`
enciende la reverberación al 50% y comprueba que un envío de 0 suena tan
fuerte como con la reverberación apagada y no deja sala después del
clic, que un envío de 1 pone la mitad en la sala, y que un envío de 0.5
pone la mitad de sala. `the_meters_see_each_channel_and_the_master` toca
un clic a volumen 0.5, con pan a la izquierda, con el master a 0.8: el
medidor del canal lee 0.25 y el izquierdo del master 0.2, el derecho 0.

En la aplicación, `the_mixer_pans_a_channel_and_sets_its_reverb_send`
hace clic en el extremo izquierdo de los sliders de Pan y Reverb de un
canal y comprueba que ahora está todo a la izquierda sin nada enviado.
`an_exported_song_keeps_each_channel_where_the_mixer_put_it` renderiza
los primeros dos segundos de la canción como hace Export WAV: ambos
altavoces igual de fuertes al principio, y solo el izquierdo después de
que cada canal se panea a la izquierda.
`a_click_on_an_instrument_in_the_mixer_changes_it` hace clic en
«Acoustic Grand Piano», luego **Strings** y **41 Violin**, comprueba que
el canal ahora toca un violín, que el mixer se hizo más estrecho por el
nombre más corto, y que un Undo devuelve el piano.

## Ejercicios

1. Cambia `pan_gains` a una ley de potencia constante: con `angle = (pan
   + 1) × π / 4`, las ganancias son `[angle.cos(), angle.sin()]`. ¿Qué
   pruebas fallan, y por qué? Escucha un canal movido de L100 a R100 con
   cada ley.
2. Un nivel de 0.1, ¿cuántos decibelios son? ¿Hasta dónde se ilumina en
   el medidor? Calcúlalo, y luego compruébalo con `lit` en `meters.rs`.
3. ¿Por qué `raise` compara `peak > 0.0` en lugar de `peak >= 0.0`? ¿Qué
   haría `fetch_max` con los bits de −0.0?
