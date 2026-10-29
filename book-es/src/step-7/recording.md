# Grabación: de una tecla a una nota

Archivos: [`crates/lineup-core/src/record.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/record.rs),
[`crates/lineup-app/src/app/live.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/live.rs)
y [`crates/lineup-core/src/file.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/file.rs)

El [último capítulo](live.md) llevó cada tecla al hilo de la UI con la
posición en la que se tocó. La grabación es lo que la UI hace con eso:
una tecla bajó en una posición y subió en otra, y entre ambas hay una
nota.

## Cuándo se graban las teclas

El botón **⏺** (grabar) solo *arma* la grabación. Las teclas se graban
mientras tres cosas son ciertas a la vez:

```rust
pub(super) fn is_recording(&self) -> bool {
    self.live.recording && self.playing && self.mode == PlayMode::Pattern
}
```

Armada, reproduciendo y en modo patrón, porque una toma va al patrón
que se repite. Armar cambia a modo patrón por ti (`toggle_recording`),
así que basta con pulsar Rec y luego Espacio.

## Recordar las teclas que están pulsadas

Cada tecla que está pulsada, de cualquiera de los dos teclados, se
guarda en una lista de `Held`:

```rust
struct Held {
    pitch: Note,
    velocity: u8,
    channel: usize,
    recorded_from: Option<f64>,
}
```

Cuando una tecla baja, `key_played` añade una:

```rust
self.live.held.retain(|held| held.pitch != pitch);
let recorded_from = played.position.filter(|_| self.is_recording());
self.live.held.push(Held {
    pitch,
    velocity,
    channel: self.piano_roll.channel(),
    recorded_from,
});
```

La misma lista alimenta el nombre del acorde en la barra de estado y
las teclas iluminadas en el piano roll; la grabación es solo uno de sus
usos. `retain` primero quita la misma altura si ya estaba pulsada (en
el otro teclado, digamos), así que cada altura está una sola vez en la
lista.

`Option::filter` conserva un `Some` solo si el closure dice que sí, y
si no lo convierte en `None`. Así que `recorded_from` es la posición
cuando la tecla se está grabando, y `None` cuando solo se está tocando.
El closure ignora su argumento (`|_|`): la cuestión no es la posición,
sino si la grabación está activa.

El `channel` se recuerda por la misma razón que `live_keys` en el hilo
de audio: la nota va al canal en el que tocó la tecla, aunque se
seleccione otro antes de que la tecla suba.

Cuando la tecla sube, su `Held` se saca de la lista, y si tiene una
posición de inicio y hay una posición final, se convierte en una nota:

```rust
let held = self.live.held.remove(index);
if let (Some(from), Some(until)) = (held.recorded_from, played.position) {
    self.record(held, from, until);
}
```

Encajar una tupla de dos `Option` comprueba ambos a la vez: el bloque
solo se ejecuta si ambos son `Some`.

## Cuantizar: `recorded_note`

Nadie toca exactamente en el paso. Una tecla que bajó en 3,9 y subió en
6,2 quería ser una nota del paso 4 al paso 6. Mover los extremos de una
nota a la cuadrícula se llama **cuantizar**, y es una función normal de
`lineup-core`, sin teclado ni UI a la vista:

```rust
pub fn recorded_note(pitch: Note, velocity: u8, from: f64, until: f64, length: usize) -> NoteEvent {
    let length = length.max(1);
    let until = if until < from {
        until + length as f64
    } else {
        until
    };
    let start = from.round();
    let end = until.round().max(start + 1.0);
    let start_step = start as usize % length;
    let steps = ((end - start) as usize).min(length - start_step);
    NoteEvent { pitch, start: start_step, length: steps, velocity }
}
```

Línea por línea:

1. `length.max(1)` protege el `%` de abajo: un resto entre cero entra
   en pánico. Un patrón nunca está vacío, pero una función no debería
   depender de quien la llama para eso.
2. El patrón se repite. Si la tecla subió *antes* de donde bajó (baja
   en 14,5, sube en 1,5), el bucle volvió a empezar por el medio, y el
   final es en realidad 17,5, un bucle más allá.
3. `round()` mueve cada extremo al paso más cercano. `f64::round`
   redondea las mitades alejándose de cero, así que 2,5 se convierte en
   3.
4. `.max(start + 1.0)`: una pulsación rápida redondearía ambos extremos
   al mismo paso, una nota de longitud 0. Una nota mide al menos un
   paso.
5. Una tecla pulsada justo antes de que el bucle vuelva a empezar (en
   15,7 en un patrón de 16 pasos) redondea a 16, que es el paso 0 de la
   siguiente vuelta: `% length` la dobla de vuelta al 0. Ahí es donde
   la quería quien toca.
6. Una nota no puede pasar del final del patrón, así que se corta allí
   con `.min(length - start_step)`.

Las pruebas se leen como esas reglas:

```rust
#[test]
fn ends_go_to_the_nearest_step() {
    assert_eq!(recorded(2.0, 4.0), (2, 2));
    assert_eq!(recorded(1.8, 4.3), (2, 2), "a little early, a little late");
    assert_eq!(recorded(2.4, 4.6), (2, 3));
}

#[test]
fn just_before_the_loop_is_step_zero() {
    assert_eq!(recorded(15.7, 0.4), (0, 1));
    assert_eq!(recorded(15.6, 1.9), (0, 2));
}

#[test]
fn a_note_across_the_loop_end_is_cut_there() {
    assert_eq!(recorded(14.0, 2.0), (14, 2));
}
```

`recorded(from, until)` es un helper del módulo de pruebas que graba un
Do4 en un patrón de 16 pasos y devuelve `(start, length)`. Poder
escribir `15.7` en lugar de tener que tocar una tecla exactamente en
ese momento es la razón de que la función viva en `lineup-core` y
reciba números normales.

### De floats a enteros: `as`

`start as usize` convierte un `f64` en un `usize`. Con floats, `as`
descarta la fracción (no queda ninguna después de `round`) y
**satura**: un número negativo se convierte en 0, un número demasiado
grande en `usize::MAX`, y NaN en 0. Nunca entra en pánico ni da la
vuelta. Eso es distinto del `as` entre enteros, que conserva solo los
bits bajos: `300_i32 as u8` es 44. De float a entero, saturar es lo que
casi siempre quieres, y Rust lo hace así desde la versión 1.45.

### Lo que no puede saber

Las posiciones dicen *dónde en el bucle* estuvo una tecla, no *en qué
vuelta*. Una tecla mantenida más de un bucle entero se ve igual que una
corta: baja en 2, sube en 3 un bucle más tarde, y se lee como un paso de
largo. Y una nota que cruza el final del bucle se corta allí, en lugar
de continuar en el paso 0. Ambas quedan como
[ejercicio 4](exercises.md).

## Añadir la nota: un paso de deshacer por toma

`record` añade la nota al patrón seleccionado mediante `edit_pattern`,
como cualquier otro cambio desde el paso 5, así que se puede deshacer y
el hilo de audio se entera:

```rust
fn record(&mut self, held: Held, from: f64, until: f64) {
    let Some(length) = self.project.pattern(self.pattern).map(Pattern::length) else {
        return;
    };
    let note = recorded_note(held.pitch, held.velocity, from, until, length);
    self.edit_pattern("record notes", |pattern| {
        pattern.add_note(held.channel, note);
    });
}
```

`.map(Pattern::length)` le pasa a `map` un método como función:
`Pattern::length` es una función que recibe un `&Pattern`, exactamente
lo que `map` entrega. `add_note` omite una nota con la misma altura e
inicio que una ya existente, así que tocar lo mismo en la siguiente
vuelta no apila notas unas sobre otras.

Una toma pueden ser docenas de notas, y Ctrl+Z debería deshacerlas
todas, no una a una. El [paso 5](../step-5/history.md) ya mantiene un
paso abierto mientras se mantiene el ratón, así que un arrastre es un
solo paso. La grabación se une a esa regla:

```rust
pub(super) fn end_frame_for_undo(&mut self, ctx: &egui::Context) {
    let holding = ctx.input(|input| input.pointer.any_down());
    if !holding && !ctx.text_edit_focused() && !self.is_recording() {
        self.history.close();
    }
}
```

Mientras se graba, el paso sigue abierto, y cada nota se une a él. La
toma termina, y con ella el paso, cuando se detiene la reproducción, se
desarma Rec o cambia el modo. Cualquier otra cosa que cambies durante
la toma, como un paso pulsado en el rack, se une al mismo paso.

## Terminar una toma: antes de detener

Cuando se detiene la reproducción, puede haber teclas aún pulsadas. Se
graban como si hubieran subido justo entonces, mediante
`finish_recording`:

```rust
pub(super) fn finish_recording(&mut self) {
    let until = self.position();
    for index in 0..self.live.held.len() {
        let held = self.live.held[index];
        if let (Some(from), Some(until)) = (held.recorded_from, until) {
            self.record(held, from, until);
        }
        self.live.held[index].recorded_from = None;
    }
}
```

Las teclas se quedan en la lista (siguen pulsadas y sonando), pero su
`recorded_from` se borra, así que su key-up posterior no graba nada. El
bucle va por índice, no `for held in &self.live.held`, porque `record`
necesita `&mut self` mientras la lista está prestada. `Held` es `Copy`,
así que `let held = self.live.held[index];` lo copia fuera, y el
préstamo termina en esa línea.

El orden en `set_playing` importa:

```rust
fn set_playing(&mut self, playing: bool) {
    if !playing {
        // Before the audio thread forgets where it was.
        self.finish_recording();
    }
    self.playing = playing;
    self.send(if playing { Command::Play } else { Command::Stop });
}
```

`finish_recording` necesita la posición, y `position()` da `None` en
cuanto `self.playing` es false, o en cuanto el hilo de audio se ha
detenido y ha guardado `NO_POSITION`. Así que la toma se termina
primero, mientras ambos dicen aún «reproduciendo», y solo entonces se envía la orden de detenerse.

## Latencia: lo que oyes está en el pasado

Cuando pulsas una tecla «en el pulso», la pulsas en el pulso que
*oíste*. Pero el sonido tarda en llegarte. El hilo de audio prepara
cada bloque con antelación y se lo entrega a la tarjeta de sonido, que
lo reproduce un poco más tarde. En el equipo en el que se construyó
este paso, los bloques eran de 512 frames, y un sonido tardaba unos
21 ms del hilo de audio a la salida de la tarjeta de sonido. Así que
cuando oyes empezar el paso 4, el hilo de audio ya va unos 21 ms dentro
de él, y una tecla pulsada exactamente entonces se marca 21 ms tarde.

A 120 BPM un paso dura 125 ms, así que 21 ms es un sexto de paso, y
redondear al paso más cercano lo esconde, salvo que ya llegaras un poco
tarde. Un teclado MIDI casi no añade nada encima: sus teclas se marcan
en el hilo de midir según llegan.

El teclado de escritura añade más. egui lee el teclado una vez por
frame, así que una tecla escrita puede esperar hasta un frame antes de
marcarse: 17 ms a 60 frames por segundo. La máquina virtual en la que
se probó este paso dibujaba sus frames despacio, sin tarjeta gráfica, y
las tomas escritas allí caían unos 110 a 130 ms tarde. A 90 BPM eso es
casi un paso entero, y por eso el La menor de la [toma que se muestra en el resumen](README.md)
empieza un paso tarde. En un equipo normal es mucho menos.

La solución es restar la latencia de la posición de cada tecla antes de
redondear. cpal puede decir cuánto tarda: cada llamada del callback de
audio recibe un `OutputCallbackInfo` con dos marcas de tiempo, cuándo
se ejecutó el callback y cuándo se reproducirá su sonido. Eso es el
[ejercicio 7](exercises.md).

## Formato 2: solo cuando hace falta (paso 7)

Un canal de salida MIDI es una clase nueva de canal, y un Ferrous Lineup
antiguo que lea `MidiOut(channel: 4)` en un archivo de proyecto no
sabría qué es. El paso 4 se preparó para esto: cada archivo empieza con
su número de formato, y una versión que ve un número mayor que el suyo
rechaza el archivo con un mensaje claro, en lugar de leerlo mal. Así
que este paso sube el formato a 2.

Pero la mayoría de los proyectos no tienen ningún canal de salida MIDI, y se
escriben exactamente con el mismo texto que antes. Marcarlos como
formato 2 haría que las versiones antiguas los rechazaran sin motivo.
Así que el archivo dice el formato más bajo capaz de contener el
proyecto:

```rust
pub fn format_for(project: &Project) -> u32 {
    let uses_midi_out = project
        .channels()
        .iter()
        .any(|channel| matches!(channel.kind, ChannelKind::MidiOut { .. }));
    if uses_midi_out { 2 } else { 1 }
}
```

`matches!` con `MidiOut { .. }` pregunta solo «¿es esta variante?», y
`..` ignora los campos. `any` se detiene en el primer acierto. La
prueba comprueba ambos lados, y que un archivo de formato 2 se lee de
vuelta como el mismo proyecto:

```rust
#[test]
fn midi_out_channels_need_format_two() {
    let mut project = Project::template();
    assert!(to_text(&project).unwrap().contains("format: 1,"));

    project.add_channel(Channel::midi_out(4)).unwrap();
    let text = to_text(&project).unwrap();
    assert!(text.contains("format: 2,"), "{text}");
    ...
    assert_eq!(from_text(&text).unwrap(), project);
}
```

En el paso 7, `FORMAT_VERSION` pasó a ser 2, y eso era lo que aceptaba
el lector: leía los formatos 1 y 2, y rechazaba el 3. La prueba de los
archivos del futuro se movió del 2 al 3 por esa razón.

**Hoy.** `FORMAT_VERSION` es 27, y `format_for` comprueba muchas más
características que un canal de salida MIDI: del formato 3 del paso 8 (nueve
tambores nuevos, patrones de más de cuatro compases y compases) hasta
el formato 27 (enviar una batería integrada por MIDI). La regla no ha
cambiado: escribe el formato más bajo capaz de contener el proyecto. Un
proyecto cuya única característica nueva es un canal de salida MIDI se sigue
escribiendo como formato 2.
