# El hilo de audio, ya adulto

Archivo:
[`crates/lineup-audio/src/engine.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-audio/src/engine.rs)

La regla de oro del paso 1 sigue en pie: el hilo de audio nunca debe
esperar, bloquear, reservar ni liberar memoria. El paso 2 hace más
difícil mantener esa regla, porque los comandos ahora transportan
instrumentos enteros, los canales van y vienen, y la interfaz necesita
saber dónde está el playhead.

## Comandos que transportan datos

```rust
pub enum Command {
    Play,
    Stop,
    SetTempo(Tempo),
    AddChannel { instrument: Instrument, steps: StepRow, volume: f32, muted: bool },
    RemoveChannel(usize),
    SetInstrument { channel: usize, instrument: Instrument },
    SetStep { channel: usize, step: usize, on: bool },
    ...
}
```

El comando ya no es `Copy`, porque `Instrument` puede contener un
`Arc<SampleData>`. Moverlo por el ring buffer sigue estando bien: el
valor se mueve al hueco preasignado de la cola y luego fuera, sin
reservar memoria. Observa que se envía por valor, no como
`Box<Instrument>`: un `Box` lo liberaría el hilo de audio al terminar
con él.

## Espacio reservado de antemano

```rust
channels: Vec::with_capacity(MAX_CHANNELS),
```

Un `Vec` tiene una **longitud** (cuántos elementos contiene) y una
**capacidad** (cuántos caben en la memoria que tiene). `push` reserva
memoria solo cuando la longitud superaría la capacidad. Reservar sitio
para `MAX_CHANNELS` al arrancar, en el hilo de la interfaz, significa
que `push` en el hilo de audio nunca reserva memoria. La interfaz se
niega a añadir más canales que eso, así que el límite nunca se alcanza.

`Vec::remove(index)` desplaza hacia abajo los elementos siguientes, lo
que tampoco reserva memoria nunca. Así mantiene `RemoveChannel` los
números de canal del hilo de audio iguales a la lista de la interfaz.

## Liberar memoria en otro sitio: la cola de basura

Cuando se elimina un canal o se sustituye su instrumento, el instrumento
viejo puede tener el último `Arc` de una muestra. Liberarlo soltaría esa
memoria, y liberar puede tomar un cerrojo dentro del asignador de
memoria. Así que el hilo de audio no lo libera. Lo devuelve:

```rust
Command::SetInstrument { channel, instrument } => match self.channels.get_mut(channel) {
    Some(state) => {
        let old = std::mem::replace(&mut state.instrument, instrument);
        self.throw_away(old);
    }
    None => self.throw_away(instrument),
},
```

`std::mem::replace(&mut place, new)` pone `new` en `place` y devuelve lo
que había. Es la forma de sacar un valor de un campo de un struct del
que solo tienes un `&mut`, ya que Rust nunca deja un campo vacío.

`throw_away` empuja el instrumento a un segundo ring buffer, que va en
la dirección contraria. El hilo de la interfaz lo vacía una vez por
frame:

```rust
pub fn collect_garbage(&mut self) {
    while let Ok(old_instrument) = self.trash.pop() {
        drop(old_instrument);
    }
}
```

`drop(value)` es una función normal del preludio que toma la propiedad y
deja que el valor salga de ámbito. Escribirla explícitamente documenta
la intención: aquí es donde se libera la memoria.

## El playhead: un entero atómico

La interfaz quiere resaltar el paso que se está reproduciendo. El hilo
de audio lo sabe. Compartirlo con un `Mutex` significaría que el hilo de
audio podría esperar por el cerrojo, así que usamos un **atómico**:

```rust
let playhead = Arc::new(AtomicUsize::new(NO_STEP));
// audio thread:
self.playhead.store(step, Ordering::Relaxed);
// UI thread:
match self.playhead.load(Ordering::Relaxed) { NO_STEP => None, step => Some(step) }
```

Un `AtomicUsize` es un número que el procesador puede leer y escribir en
una sola operación indivisible, de modo que dos hilos pueden usarlo sin
cerrojo y sin ver nunca un valor a medio escribir. Ambos hilos tienen un
`Arc` al mismo (`Arc::clone(&playhead)` crea el segundo puntero).

`Ordering::Relaxed` dice que solo nos importa este valor, no el orden de
otras escrituras de memoria a su alrededor. Es el orden más barato y
basta para mostrarlo. Los órdenes más estrictos (`Acquire`, `Release`,
`SeqCst`) son para cuando un valor protege el acceso a otros.

Rust no tiene un paso «nulo», así que `usize::MAX` representa «parado»,
y `playhead()` lo convierte enseguida en un `Option<usize>` honesto para
que ningún otro código vea nunca el número mágico.

## Renderizar un bloque

```rust
for frame in output.chunks_mut(self.output_channels) {
    if self.playing
        && let Some(step) = self.sequencer.tick()
    {
        self.playhead.store(step, Ordering::Relaxed);
        for channel in &mut self.channels {
            if channel.steps[step % MAX_STEPS] && !channel.muted {
                channel.instrument.trigger(self.sample_rate);
            }
        }
    }

    let mut mix = 0.0;
    for channel in &mut self.channels {
        let volume = channel.volume.next_value();
        mix += channel.instrument.next_sample() * volume;
    }
    let sample = (mix * self.master_volume.next_value()).tanh();
    frame.fill(T::from_sample(sample));
}
```

- `if a && let Some(x) = b { ... }` es una **let chain** (Rust 1.88):
  una condición y un patrón combinados en un solo `if`.
- `for channel in &mut self.channels` itera sobre referencias mutables,
  para que cada instrumento pueda avanzar su propio estado.
- Mezclar es simplemente sumar: los sonidos en el aire se suman, y las
  muestras también.
- Varios tambores fuertes a la vez pueden sumar más de 1.0, y una
  tarjeta de sonido lo recortaría con dureza. `tanh` (la tangente
  hiperbólica) deja pasar los valores pequeños casi sin cambios y dobla
  los grandes suavemente hacia ±1.0, un **soft clipper** (recortador
  suave).
- La interfaz nunca pone un patrón de más de 64 pasos, pero
  `% MAX_STEPS` garantiza que el índice esté en rango aunque lo hiciera,
  de modo que esta línea nunca puede entrar en pánico en el hilo de
  audio.

## Probar sin tarjeta de sonido

Las pruebas al final de `engine.rs` construyen un `AudioThread` con ring
buffers, empujan comandos, renderizan en un `Vec<f32>` y comprueban el
resultado: un golpe en el frame 0 y en el frame 6000 a 120 BPM, silencio
al silenciar, ediciones que surten efecto durante la reproducción e
instrumentos viejos que llegan a la basura. Una pequeña struct `Harness`
guarda las piezas para que cada prueba sea corta.
