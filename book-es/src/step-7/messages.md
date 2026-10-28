# Mensajes MIDI y archivos MIDI

Archivos: [`crates/lineup-core/src/midi.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/midi.rs)
y [`crates/lineup-core/src/midi_file.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/midi_file.rs)

## Tres bytes por tecla

Un teclado MIDI dice casi todo con mensajes de tres bytes. El primero,
el *byte de estado*, dice qué clase de mensaje es y en cuál de los 16
*canales*. Los otros dos son *bytes de datos*. Para una tecla que baja:

```text
  1001 0000   0011 1100   0110 0100
  │    │      │           │
  │    │      │           └ velocidad 100: con qué fuerza se pulsó la tecla
  │    │      └ nota 60: do central (C4)
  │    └ canal 0 (la gente lo llama canal 1)
  └ 1001 = note-on
```

Un byte de estado siempre tiene el bit superior a 1 (de 128 en
adelante), y un byte de datos nunca (de 0 a 127). Por eso un número de
nota MIDI y una velocidad nunca pasan de 127, y por eso `Note` en el
paso 1 usaba los mismos números: Do4 es 60 en Ferrous Lineup porque es
60 en MIDI.

Hay muchas clases de mensaje: potenciómetros y pedales («control
change»), pitch bends, tics de reloj para mantener las máquinas en
tiempo. En el paso 7 Ferrous Lineup solo necesitaba dos, y `MidiMessage`
solo tenía dos variantes:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MidiMessage {
    NoteOn { channel: u8, pitch: Note, velocity: u8 },
    NoteOff { channel: u8, pitch: Note },
}

/// The kinds of message, as the top four bits of the status byte.
const NOTE_OFF: u8 = 0x80;
const NOTE_ON: u8 = 0x90;
```

> **Adelante: el paso 8 añade un tercero.** Un program change le dice a
> un sintetizador qué instrumento toca un canal. Son dos bytes:
> `0xC0 | channel` y un número de programa de 0 a 127, sin velocidad.
> `MidiMessage` ahora tiene también una variante
> `ProgramChange { channel, program }`
> ([Cada voz en su propio canal](../step-8/voices-and-instruments.md)).
> Los mensajes de nota y los patrones de slice de abajo siguen igual;
> `parse` solo lee notas, mientras que `bytes` escribe las tres clases.

El byte de estado guarda dos cosas a la vez, así que el código lo
divide con operaciones de bits (el paso 3 usó las mismas para los
conjuntos de alturas): `status & 0xF0` se queda con los cuatro bits
superiores, la clase, y `status & 0x0F` se queda con los cuatro
inferiores, el canal.

## Leer un mensaje: patrones de slice

En el paso 7, leer un mensaje de nota se veía así:

```rust
pub fn parse(bytes: &[u8]) -> Option<MidiMessage> {
    let [status, pitch, velocity, ..] = *bytes else {
        return None;
    };
    // Data bytes have the top bit clear; anything else is malformed.
    if pitch > 127 || velocity > 127 {
        return None;
    }
    let channel = status & 0x0F;
    let pitch = Note::new(pitch);
    match status & 0xF0 {
        NOTE_ON if velocity > 0 => Some(MidiMessage::NoteOn { channel, pitch, velocity }),
        NOTE_ON | NOTE_OFF => Some(MidiMessage::NoteOff { channel, pitch }),
        _ => None,
    }
}
```

`let [status, pitch, velocity, ..] = *bytes else` es un **patrón de
slice**. Un slice puede tener cualquier longitud, así que un patrón
para él describe una *forma*: «al menos tres elementos; llama a los
tres primeros `status`, `pitch` y `velocity`, y `..` para lo que
siga». Si el slice es más corto, el patrón no encaja y el bloque `else`
retorna. Sin patrones de slice esto sería una comprobación de longitud
y tres indexados, cada uno capaz de entrar en pánico si la comprobación
estuviera mal. El `*` encaja con el slice en sí, no con la referencia a
él; está permitido porque las partes que se extraen son `u8`, que son
`Copy`.

El `match` usa dos cosas de pasos anteriores. `NOTE_ON` y `NOTE_OFF`
son constantes, y una constante en un patrón significa «igual a este
valor», no «una variable nueva». Y `NOTE_ON if velocity > 0` tiene una
**match guard**: el brazo solo encaja cuando la condición también se
cumple. Un note-on con velocidad 0 cae al siguiente brazo y se
convierte en un note-off. Muchos teclados de verdad envían eso en lugar
de un note-off: una ráfaga de mensajes del mismo tipo puede omitir el
byte de estado repetido («running status»), lo que ahorraba tiempo en
los cables lentos de 1983.

En el paso 7, convertir un mensaje de vuelta en bytes era lo mismo al
revés:

```rust
pub fn bytes(self) -> [u8; 3] {
    match self {
        MidiMessage::NoteOn { channel, pitch, velocity } => {
            [NOTE_ON | (channel & 0x0F), pitch.midi(), velocity.clamp(1, 127)]
        }
        // Velocity 64 is the usual "no particular speed" for a release.
        MidiMessage::NoteOff { channel, pitch } => [NOTE_OFF | (channel & 0x0F), pitch.midi(), 64],
    }
}
```

Ese `[u8; 3]` es la versión del paso 7. Hoy `bytes` devuelve un
`MidiBytes` pequeño que se desreferencia a un slice de la longitud
correcta, porque un program change solo tiene dos bytes. Los brazos de
nota no han cambiado.

El `clamp(1, 127)` importa. Un note-on con velocidad 0 lo leería el
sintetizador como un note-off, así que una nota tocada «lo más suave
posible» no sonaría en absoluto.

El paso 7 también escribe un mensaje que nunca lee: control change 123,
«All Notes Off». Le dice a un sintetizador que detenga todas las notas
de un canal. Ferrous Lineup lo envía en los 16 canales antes de soltar
un puerto de salida, por si un note-off se perdió por el camino:

```rust
pub fn all_notes_off(channel: u8) -> [u8; 3] {
    [CONTROL_CHANGE | (channel & 0x0F), 123, 0]
}
```

## Un archivo MIDI

Un Standard MIDI File (`.mid`) guarda los mismos mensajes, con tiempos.
Guarda notas, no sonido: «Do4 empieza aquí, termina allí, en el canal
1». El programa que lo abre elige los instrumentos. Eso lo convierte en
el idioma común de los programas de música, y en la manera de llevar un
patrón de Ferrous Lineup a otro estudio.

El archivo es una lista de *chunks*. Cada uno empieza con cuatro letras
y su longitud:

```text
  MThd  length 6  format 1  3 pistas  96 ticks por pulso     la cabecera
  MTrk  length …  name "Ferrous Lineup", tempo, compás, fin
  MTrk  length …  name "Lineup Osc", las notas del sintetizador, fin
  MTrk  length …  name "Kick", las notas del bombo, fin
```

Esto es el **formato 1**: varias pistas que suenan juntas. La primera
pista guarda solo el tempo y el compás, y cada canal del proyecto
recibe una pista propia, con su nombre. Un canal sin notas, o
silenciado, no recibe pista.

Los números de un archivo MIDI son big-endian: primero el byte más
significativo. Los enteros de Rust tienen un método para eso, que
devuelve un array de bytes:

```rust
let mut file = Vec::new();
file.extend_from_slice(b"MThd");
file.extend_from_slice(&6u32.to_be_bytes()); // the header's length
file.extend_from_slice(&1u16.to_be_bytes()); // format 1
file.extend_from_slice(&(tracks.len() as u16).to_be_bytes());
file.extend_from_slice(&TICKS_PER_BEAT.to_be_bytes());
```

`b"MThd"` es un **byte string**: un `&[u8; 4]` con los códigos ASCII de
las letras, en lugar de un `&str`. `6u32` es el número 6 como `u32` (el
tipo escrito después del número), así que `to_be_bytes` da cuatro
bytes: `[0, 0, 0, 6]`. Un `u16` da dos.

## Ticks y delta time

El tiempo en un archivo MIDI se cuenta en *ticks*, y la cabecera dice
cuántos ticks forman un pulso. Ferrous Lineup escribe 96, que es
divisible por 2, 3, 4, 8, 16 y 32, así que también cabrían tresillos y
fusas. Un paso es una semicorchea, un cuarto de pulso, así que son 24
ticks:

```rust
pub const TICKS_PER_BEAT: u16 = 96;
pub const TICKS_PER_STEP: u32 = TICKS_PER_BEAT as u32 / STEPS_PER_BEAT;
```

Dentro de una pista, cada evento se escribe después de un *delta time*:
el número de ticks desde el evento anterior, no desde el principio. La
mayoría de los deltas son pequeños (0 para las notas de un acorde, 24
para el siguiente paso), así que se escriben en un formato donde los
números pequeños ocupan un byte, la **cantidad de longitud variable**:
siete bits del número por byte, el más significativo primero, con el
bit superior a 1 en todos los bytes salvo el último.

```text
  200  = 1 1001000            ->  1000 0001  0100 1000   (0x81 0x48)
         │ └ los últimos 7 bits         │         └ bit superior a 0: el último byte
         └ el resto: 1                  └ bit superior a 1: hay más
  384  = 11 0000000           ->  0x83 0x00
```

```rust
fn write_variable_length(out: &mut Vec<u8>, value: u32) {
    let mut groups = [0u8; 5];
    let mut count = 0;
    let mut rest = value;
    loop {
        groups[count] = (rest & 0x7F) as u8;
        count += 1;
        rest >>= 7;
        if rest == 0 {
            break;
        }
    }
    for index in (0..count).rev() {
        let more = if index > 0 { 0x80 } else { 0 };
        out.push(groups[index] | more);
    }
}
```

El bucle parte el número en grupos de siete bits por el extremo bajo,
porque ese es el extremo al que llegan `& 0x7F` y `>>= 7`. El archivo
los quiere por el extremo alto, así que el segundo bucle recorre los
grupos al revés con `(0..count).rev()`. Cinco grupos bastan para
cualquier `u32`, así que caben en un array y no se reserva memoria. La
prueba le da los ejemplos impresos en la especificación del archivo
MIDI.

Un `TrackWriter` guarda el tick del último evento, así que el resto del
código puede pensar en ticks absolutos:

```rust
#[derive(Default)]
struct TrackWriter {
    data: Vec<u8>,
    last_tick: u32,
}

impl TrackWriter {
    fn delta(&mut self, tick: u32) {
        let delta = tick.saturating_sub(self.last_tick);
        write_variable_length(&mut self.data, delta);
        self.last_tick = self.last_tick.max(tick);
    }
    ...
    fn finish(mut self, end: u32) -> Vec<u8> {
        self.meta(end.max(self.last_tick), END_OF_TRACK, &[]);
        let mut chunk = Vec::with_capacity(8 + self.data.len());
        chunk.extend_from_slice(b"MTrk");
        chunk.extend_from_slice(&(self.data.len() as u32).to_be_bytes());
        chunk.extend(self.data);
        chunk
    }
}
```

`#[derive(Default)]` da `TrackWriter::default()`, un `Vec` vacío y tick
0. `finish` recibe `mut self`, no `&mut self`: consume el writer, y sus
datos se mueven al chunk sin copia. Un writer terminado no puede
escribirse por error, porque ya no existe.

## Meta eventos: nombres, tempo, compás

Los eventos que no son notas empiezan con `0xFF`, una clase y una
longitud:

| Bytes | Significa |
|-------|-----------|
| `FF 03 len text` | el nombre de la pista |
| `FF 51 03 tt tt tt` | el tempo, en microsegundos por pulso |
| `FF 58 04 4 2 24 8` | compás de 4/4 (4 pulsos, un pulso es 2² = una negra) |
| `FF 2F 00` | el final de la pista |

El tempo es el raro: no pulsos por minuto, sino cuánto dura un pulso,
en microsegundos, en tres bytes. A 120 BPM un pulso dura medio segundo,
500 000 µs, que es `07 A1 20`. El código toma los tres últimos de los
cuatro bytes que da `to_be_bytes`:

```rust
let micros = (60_000_000.0 / f64::from(project.tempo.bpm())).round() as u32;
let micros = micros.min(0xFF_FFFF).to_be_bytes();
track.meta(0, TEMPO, &micros[1..]);
```

`&micros[1..]` es un slice del array desde el índice 1 hasta el final:
los tres bytes bajos.

## Notas, batería y canales

Cada nota se convierte en dos eventos, su note-on y su note-off. Las
notas de un patrón colocado en un paso de la canción se desplazan ese
número de pasos, y una nota que se pasa del final de su patrón se corta
allí, exactamente igual que la toca el hilo de audio:

```rust
let held = note.length.min(length - note.start);
let on = (first_step + note.start) as u32 * TICKS_PER_STEP;
let off = on + held as u32 * TICKS_PER_STEP;
```

Luego los eventos se ordenan por tiempo. En el mismo tick, los note-off
deben ir antes que los note-on: si una nota termina exactamente donde
la misma nota vuelve a empezar, «on, off» terminaría la nota nueva al
momento, y solo «off, on» se oye dos veces.

```rust
events.sort_by_key(|&(tick, message)| (tick, matches!(message, MidiMessage::NoteOn { .. })));
```

La clave es una tupla, `(tick, is_note_on)`, y las tuplas se comparan
elemento por elemento (el [paso 6](../step-6/tools.md#agrupar-notas-en-acordes-chords)
ordenaba los acordes de la misma manera). En el mismo tick,
`false < true`, así que ganan los note-off. `sort_by_key` es un
ordenamiento *estable*: los eventos con claves iguales mantienen su
orden, así que las notas de un acorde quedan en el orden en que se
escribieron.

¿Qué canal MIDI usa cada pista? Un programa que lee el archivo usa el
canal para elegir el instrumento, y General MIDI, el estándar que
siguen la mayoría de los sintetizadores, reserva el **canal 10** (9,
contando desde 0) para la batería, con una nota fija para cada tambor:
36 es un bombo, 38 una caja, 39 una palmada, 42 un hi-hat cerrado. Así
que:

```rust
fn midi_channels(channels: &[Channel]) -> Vec<u8> {
    let mut melodic = (0..16u8).filter(|&channel| channel != DRUM_CHANNEL).cycle();
    channels
        .iter()
        .map(|channel| match channel.kind {
            ChannelKind::Sampler(SampleSource::Builtin(_)) => DRUM_CHANNEL,
            ChannelKind::MidiOut { channel } => channel,
            _ => melodic.next().unwrap_or(0),
        })
        .collect()
}
```

La batería integrada comparte el canal 10, y sus notas se sustituyen
por las de General MIDI (`general_midi_drum`). Un canal MIDI Out
conserva el canal que se le dio. Los demás canales toman el siguiente
de `melodic`: los números 0 a 15 sin el 9, repetidos sin fin con
`cycle` (que el [paso 6](../step-6/tools.md) usó para los arpegios). El
iterador se crea una vez, fuera del closure, y el closure lo toma
prestado mutablemente: cada `next()` lo hace avanzar, así que los
canales reciben 0, 1, 2… por turno. Con más de 15 canales melódicos,
empiezan a compartirse.

## Probar un formato de archivo

Las pruebas leen cada archivo de vuelta con un lector pequeño, `read`,
escrito en el módulo de pruebas, igual que lo haría cualquier programa
MIDI. Luego comprueban lo que volvió:

```rust
#[test]
fn drums_use_channel_ten_and_general_midi_notes() {
    let tracks = read(&pattern_to_midi(&project(), 0));
    let kick = &tracks[2];
    assert!(kick.contains(&(0, vec![0x99, 36, DEFAULT_VELOCITY])));
    assert!(kick.contains(&(192, vec![0x99, 36, DEFAULT_VELOCITY])));
}
```

`0x99` es note-on (`0x90`) en el canal 10 (`0x09`), y el tick 192 es el
paso 8. Eso sí, un lector escrito por la misma persona que escribió el
escritor puede compartir sus errores. Por eso los archivos exportados
también se abrieron con un programa que no sabe nada de Ferrous Lineup,
la librería `mido` de Python, que leyó la canción de demostración como
seis pistas de 14,769 segundos: 128 pasos a 130 BPM, como debe ser.
