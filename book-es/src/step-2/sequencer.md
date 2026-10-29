# El secuenciador: temporización a la muestra

Archivo:
[`crates/lineup-core/src/sequencer.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/sequencer.rs)

## La aritmética

A un tempo de *T* pulsos por minuto y una frecuencia de muestreo de *R*
frames por segundo, un pulso dura `R × 60 / T` frames y un paso (un
cuarto de pulso) dura un cuarto de eso:

```text
frames por paso = R × 60 / T / 4

120 BPM, 48 kHz:  48000 × 60 / 120 / 4 = 6000 frames       (exacto)
130 BPM, 44.1 kHz: 44100 × 60 / 130 / 4 = 5088.46... frames (no entero)
```

Si redondeáramos 5088.46 a 5088, cada paso se quedaría 0.46 frames
corto. Son unos 10 microsegundos, nada que fueras a oír una sola vez,
pero se acumula: al cabo de unos minutos la batería iría audiblemente
por delante de cualquier otra cosa que corra a 130 BPM. Por eso el
secuenciador conserva la fracción.

## El tick

El hilo de audio llama a `tick` una vez por cada frame que renderiza:

```rust
pub fn tick(&mut self) -> Option<usize> {
    let fired = if self.frames_until_next <= 0.0 {
        let step = self.next_step;
        self.next_step = (self.next_step + 1) % self.length;
        self.frames_until_next += self.frames_per_step;
        Some(step)
    } else {
        None
    };
    self.frames_until_next -= 1.0;
    fired
}
```

`frames_until_next` cuenta hacia atrás de uno en uno cada frame. Cuando
llega a cero, se dispara el siguiente paso y se *añade* al contador la
longitud de un paso entero, no se asigna. La fracción que sobrara se
queda en el contador, de modo que los límites de los pasos caen en 0,
5088.46, 10176.92... redondeados al frame más cercano cada vez, pero sin
derivar nunca. `%` es el operador de resto, que devuelve el número de
paso a 0 al final del patrón.

Como un paso se dispara en el frame exacto donde empieza, y no al
principio del bloque de audio que lo contiene, la temporización no
depende del tamaño del búfer de la tarjeta de sonido. Eso es lo que
significa «exacto a la muestra».

## Por qué vive en `lineup-core`

El secuenciador no tiene nada que ver con tarjetas de sonido. Solo
cuenta. Ponerlo en `lineup-core` significa que se prueba en milisegundos,
sin audio, y la lista de reproducción del paso 4 puede reutilizarlo.

## Probar con iteradores

```rust
fn run(sequencer: &mut Sequencer, frames: usize) -> Vec<(usize, usize)> {
    (0..frames)
        .filter_map(|frame| sequencer.tick().map(|step| (frame, step)))
        .collect()
}
```

Este pequeño ayudante avanza el secuenciador `frames` veces y enumera
cada paso que se disparó junto con el frame en el que se disparó:

- `0..frames` es un **rango**, que es un iterador sobre números.
- `filter_map` llama al closure para cada frame y se queda solo con los
  resultados `Some`, sin envolver.
- `Option::map` convierte `Some(step)` en `Some((frame, step))`, una
  tupla.
- `collect()` reúne los resultados en un `Vec`. Rust deduce qué
  colección construir a partir del tipo de retorno.

Las pruebas comprueban entonces números de frame exactos:

```rust
assert_eq!(
    fired,
    vec![(0, 0), (6_000, 1), (12_000, 2), (18_000, 3), (24_000, 0)]
);
```

y que un minuto a 130 BPM da exactamente 520 pasos (130 pulsos × 4).
Los guiones bajos en los literales numéricos como `6_000` los ignora el
compilador y solo hacen más legibles los números grandes.

La misma temporización también se comprobó de extremo a extremo: se
ejecutó la app con su salida de sonido grabada en un archivo, y cada
golpe empezaba en el frame esperado.
