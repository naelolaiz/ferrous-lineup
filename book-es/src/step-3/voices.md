# Tocar notas: voces y envolventes

Archivos: [`crates/lineup-audio/src/instrument.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-audio/src/instrument.rs)
y [`crates/lineup-audio/src/engine.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-audio/src/engine.rs)

En el paso 2, un instrumento solo podía ser *disparado*: volver a empezar
el sonido, siempre en la misma altura. Para tocar acordes y melodías
necesita tocar varias notas a la vez, cada una en su propia altura, y
sostener cada una durante su propia duración.

## Enviar notas al hilo de audio

El hilo de audio guarda su propia copia de las notas de cada canal,
igual que antes guardaba una copia de los pasos. `Command::SetStep` ya
no está; en su lugar, la interfaz envía la lista completa de nuevo cada
vez que cambia algo en ella:

```rust
pub enum Command {
    // ...
    AddChannel { instrument: Instrument, notes: Vec<NoteEvent>, volume: f32, muted: bool },
    SetNotes { channel: usize, notes: Vec<NoteEvent> },
    PreviewNote { channel: usize, pitch: Note },
}
```

Enviar una lista completa por una sola nota cambiada suena a
desperdicio, pero una lista de unos cientos de notas son unos kilobytes,
copiados en el hilo de la interfaz. Mantiene el lado de audio trivial:
intercambia la lista vieja por la nueva.

La lista nueva la reservó la interfaz, y la vieja no debe liberarse en
el hilo de audio (liberar memoria está tan prohibido como reservarla).
Así que la cola de basura del paso 2 ahora lleva dos clases de basura, y
un enum dice cuál:

```rust
enum Garbage {
    Instrument(Instrument),
    Notes(Vec<NoteEvent>),
}

Command::SetNotes { channel, notes } => match self.channels.get_mut(channel) {
    Some(state) => {
        let old = std::mem::replace(&mut state.notes, notes);
        self.throw_away(Garbage::Notes(old));
    }
    None => self.throw_away(Garbage::Notes(notes)),
},
```

`Garbage` es privado para `engine.rs`. `AudioEngine` es público, pero
sus campos son privados, así que un tipo privado puede vivir dentro de
uno de ellos (`trash: rtrb::Consumer<Garbage>`) sin ser visible jamás
para otros crates. En el hilo de la interfaz, `collect_garbage` vacía la
cola y elimina cada elemento, que es cuando la memoria se libera de
verdad.

## Empezar las notas a tiempo

Cuando el secuenciador dispara un paso, cada nota que empieza ahí
comienza a sonar. Su **gate**, cuánto se sostiene, es su duración en
pasos convertida en frames:

```rust
fn start_notes(&mut self, step: usize) {
    let step_frames = self.sequencer.step_frames();
    let length = self.sequencer.length();
    for channel in &mut self.channels {
        if channel.muted {
            continue;
        }
        for note in channel.notes.iter().filter(|note| note.start == step) {
            // A note that runs past the end of the pattern is cut there.
            let steps_held = note.length.min(length.saturating_sub(step));
            let gate = (steps_held as f64 * step_frames) as u32;
            channel
                .instrument
                .note_on(note.pitch, note.velocity, gate, self.sample_rate);
        }
    }
}
```

El bucle interior lee `channel.notes` mientras cambia
`channel.instrument`. Un préstamo compartido y un préstamo mutable al
mismo tiempo normalmente se rechazarían, pero estos son préstamos de
**dos campos distintos** del mismo struct, y el compilador puede ver que
no se superponen. (Esto solo funciona cuando los campos se nombran
directamente. Si se llegara a `notes` a través de un método como
`channel.notes()`, el método tomaría prestado el `channel` completo y el
código no compilaría.)

`saturating_sub` resta pero se detiene en cero en lugar de desbordarse:
un `usize` no puede ser negativo, y `0 - 1` haría panic en una
compilación de debug.

Cada nota decide su propio momento de terminar cuando empieza, así que
el hilo de audio nunca tiene que acordarse de detenerla. Eso también
significa que editar o borrar una nota mientras suena nunca puede
dejarla sonando para siempre: sigue terminando cuando se le acaba el
gate.

## Voces

Una **voz** es todo lo que hace falta para producir una nota: su propio
oscilador, su propia posición en la envolvente. El Lineup Osc tiene 16
voces y un sampler tiene 8. Cada `note_on` toma una voz:

```rust
trait Voice {
    fn is_idle(&self) -> bool;
    fn started(&self) -> u64;
}

fn pick_voice<V: Voice>(voices: &mut [V]) -> &mut V {
    let index = voices
        .iter()
        .position(Voice::is_idle)
        .or_else(|| {
            voices
                .iter()
                .enumerate()
                .min_by_key(|(_, voice)| voice.started())
                .map(|(index, _)| index)
        })
        .unwrap_or(0);
    &mut voices[index]
}
```

El orden de búsqueda es: una voz en silencio si la hay; si no, la voz
cuya nota empezó hace más tiempo. Quitarle una voz ocupada a alguien se
llama **voice stealing**; todo sintetizador de hardware lo hace cuando
sostienes más notas que voces tiene.

Este es el primer **trait** que Ferrous Lineup define por sí mismo.
`SamplerVoice` y `SynthVoice` son structs distintos, pero ambos
implementan `Voice`, así que una sola función genérica sirve para los
dos. `V: Voice` es el límite del trait: la función solo puede llamar a
lo que el trait promete. El trait es privado (sin `pub`), porque es un
detalle de este archivo.

¿Por qué buscar un índice primero y luego devolver `&mut voices[index]`,
en lugar de devolver la voz que encontró la búsqueda? Las búsquedas usan
`iter()`, que presta el slice para leer. Devolver una referencia mutable
desde una búsqueda de solo lectura no es posible, así que el código
encuentra la posición y luego presta de forma mutable una sola vez, al
final.

`started` viene de un contador que sube con cada nota, así que «la que
empezó hace más tiempo» es «el contador más pequeño». Un `u64` no se
desbordará: a mil notas por segundo tardaría 584 millones de años.

### Un número fijo de voces, en un Box

```rust
pub struct Synth {
    sample_rate: f32,
    voices: Box<[SynthVoice; SYNTH_VOICES]>,
    envelope: EnvelopeRates,
    notes_started: u64,
}

voices: Box::new(std::array::from_fn(|_| SynthVoice::new(sample_rate))),
```

Las voces son un array de tamaño fijo: el hilo de audio nunca lo agranda
ni lo encoge, así que nunca reserva memoria. `std::array::from_fn`
construye un array llamando a un closure una vez por elemento (el
closure recibe el índice, que aquí no hace falta, de ahí el `|_|`).

El array se mete en un `Box`, en el montículo, por culpa de la cola de
comandos. Cada ranura de la cola es tan grande como el `Command` más
grande, y `AddChannel` lleva un `Instrument`. Dieciséis voces son unos
640 bytes; un `Box` es un puntero de 8 bytes. Una prueba fija el tamaño:

```rust
#[test]
fn commands_stay_small() {
    let size = std::mem::size_of::<Command>();
    assert!(size <= 64, "a command takes {size} bytes");
}
```

El paso 2 advirtió que enviar un `Box<Instrument>` haría que el hilo de
audio liberara memoria cuando terminara con él. Eso sigue siendo cierto,
y por eso los instrumentos con los que el hilo de audio termina van
siempre a la cola de basura, voces en Box incluidas. El Box se crea y se
libera en el hilo de la interfaz; el hilo de audio solo mueve el
puntero.

## La envolvente: una máquina de estados

Una nota sostenida no debería limitarse a encenderse y apagarse: eso
hace clic, y no suena nada parecido a un instrumento. Una **envolvente**
moldea el volumen a lo largo del tiempo. La clásica tiene cuatro partes,
**ADSR**:

```text
 volumen
   1 ┤  /\
     │ /  \___________          A: ataque, subiendo al volumen completo
 0.45┤/    D    S      \        D: caída, bajando hasta...
     │                  \       S: sustain, mantenido mientras la nota se sostiene
   0 ┼───────────────────\___   R: release, desvaneciéndose cuando la nota termina
       A    fin del gate ^
```

Cada voz está en una **etapa**, un enum, y cada muestra puede moverla a
la siguiente etapa. El código con esta forma se llama **máquina de
estados**:

```rust
enum Stage { Idle, Attack, Decay, Release }

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
        self.level = SYNTH_SUSTAIN + (self.level - SYNTH_SUSTAIN) * envelope.decay_factor;
    }
    Stage::Release => {
        self.level *= envelope.release_factor;
        if self.level < SILENT {
            self.stage = Stage::Idle;
            return 0.0;
        }
    }
}

// Count down the gate while the note is held, then let go.
if matches!(self.stage, Stage::Attack | Stage::Decay) {
    match self.gate.checked_sub(1) {
        Some(left) => self.gate = left,
        None => self.stage = Stage::Release,
    }
}
```

- La caída y el release son **exponenciales**: cada frame conserva una
  fracción fija de la distancia que queda, así que el sonido se desvanece
  como lo hacen las cuerdas y campanas reales. `decay_factor` es
  `e^(-1 / (seconds × sample_rate))`, calculado una vez por instrumento
  en `EnvelopeRates::new`, no por muestra.
- `checked_sub` devuelve `None` en lugar de bajar de cero: «si no queda
  gate, release».
- No hay una variante `Sustain`. La caída se acerca al nivel de sustain
  y se queda ahí, así que una etapa aparte no haría nada.

`Instrument::release_all` mueve a `Release` todas las voces sostenidas.
El motor lo llama al detener la reproducción, así que las notas largas se desvanecen en
lugar de seguir sonando después de que la música se detuvo.

## Los samplers también tocan alturas

Un sampler toca una nota reproduciendo su muestra más rápido o más
lento. Una octava arriba es el doble de rápido, y cada semitono
multiplica la velocidad por la raíz duodécima de dos, la misma regla que
usa `Note::frequency`:

```rust
let semitones = pitch.midi() as f64 - STEP_NOTE.midi() as f64;
let pitch_ratio = 2.0_f64.powf(semitones / 12.0);
let rate_ratio = self.sample.sample_rate() as f64 / output_rate as f64;
// ...
increment: rate_ratio * pitch_ratio,
```

En `STEP_NOTE` (Do4), la proporción es 1, así que los pasos suenan
exactamente igual que antes. Abre el piano roll en el Bombo y dibuja una
melodía para escuchar una batería con altura. Los samplers ignoran el
gate: como una caja de ritmos, reproducen el sonido completo cada vez.
Como ahora tienen voces, un golpe nuevo ya no corta el anterior; los dos
se superponen.

## Comprobarlo de oído, sin oídos

El contenedor que compila Ferrous Lineup no tiene altavoces, así que
este paso también se comprobó grabando la salida de la aplicación a un
archivo y mirando su espectro. Durante el primer acorde, las frecuencias
más fuertes eran 131, 261, 329 y 392 Hz: Do3, Do4, Mi4 y Sol4, las notas del
acorde de Do mayor de la demo. Las pruebas unitarias hacen una versión
más simple de lo mismo: `synth_plays_the_right_pitch` cuenta cuántas
veces cruza el cero la salida mientras reproduce La4, y espera unos 880
cruces por segundo (dos por ciclo de 440 Hz).
