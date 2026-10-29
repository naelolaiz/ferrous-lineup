# Cambiar el compás a mitad de camino

Archivos: [`crates/lineup-core/src/meter.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/meter.rs),
[`crates/lineup-core/src/project.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/project.rs),
[`crates/lineup-core/src/midi_file.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/midi_file.rs)
y [`crates/lineup-app/src/playlist/sections.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/playlist/sections.rs)

[Los compases](time-signatures.md) dieron a un proyecto un solo compás.
La música real no siempre mantiene uno: la última línea de un himno
puede estirarse a 3/4, una danza folclórica puede alternar entre 6/8 y
3/4, una canción de rock progresivo cambia cada pocos compases. Una
hoja de acordes escribe el compás nuevo donde cambia, y los programas
de música lo marcan en la línea de tiempo; ahora Ferrous Lineup
también.

Haz clic derecho en los números de compás y elige un compás. Desde ese
compás en adelante, los compases se cuentan en él. La regla escribe el
compás en el compás 1 y dondequiera que cambie, y el mismo menú tiene
**Quitar este cambio**.

![La demo con 3/4 desde el compás 5 y 6/8 desde el compás 7. Los compases 5 y 6 miden 12 pasos de ancho, así que sus líneas de compás están más juntas.](../images/step-8-time-markers.png)

![El clic derecho en el compás 10 ofrece todo compás. El compás 1 pone aquel en el que empieza la canción.](../images/step-8-time-menu.png)

Como antes, un compás nunca mueve una nota. Los pasos siguen siendo
semicorcheas, y un patrón de 16 pasos sigue durando 16 pasos cuando lo
suelta en un tramo en 3/4. Lo que cambia es dónde van las líneas de
compás.

## Un marcador, y un metro

Un cambio es un struct diminuto:

```rust
pub struct TimeMarker {
    pub start: usize,        // the step it starts on
    pub time: TimeSignature, // the bars from there on
}
```

El proyecto guarda su propio `time_signature` para el comienzo de la
canción, y la lista de reproducción guarda una lista ordenada de marcadores después
de él, como guarda sus secciones. Un marcador siempre empieza un compás
nuevo. Pon uno en mitad de un compás y ese compás se queda corto, que
es como una partitura muestra un compás al que le falta un pulso.

Antes de los marcadores, toda pregunta sobre compases era aritmética
simple: "¿en qué compás está el paso 50?" es `50 / 16`. Con marcadores
depende de todo lo anterior al paso 50, así que las respuestas se
mudaron a un solo tipo, `Meter`:

```rust
pub struct Meter {
    first: TimeSignature,
    markers: Vec<TimeMarker>,
}
```

Sus métodos son las preguntas que hace la app: `bar_at(step)` (qué
compás, dónde empieza, cuánto dura y en qué compás), `bar(number)`,
`is_bar_start`, `floor_to_beat`, `round_up_to_bar`, y `position(step)`
para la pantalla BAR:BEAT:STEP. Una canción sin marcadores es
`Meter::steady(time)`, y da exactamente las mismas respuestas que el
`TimeSignature` solo; una prueba lo comprueba, paso a paso.

`bar_at` recorre la canción en *tramos*, uno por compás, contando los
compases que pasa:

```text
4/4 desde 0, 3/4 desde 32:
tramo 1: pasos 0..32,  16 por compás -> compases 0 y 1
tramo 2: pasos 32..,   12 por compás -> compases 2, 3, 4, ...
```

```rust
for (start, end, time) in self.stretches() {
    let bar = time.steps_per_bar();
    if step < end {
        let index = (step - start) / bar;
        // ... bar number `bars_before + index`, starting at start + index * bar
    }
    bars_before += (end - start).div_ceil(bar);
}
```

`div_ceil` redondea hacia arriba, así que un compás corto antes de un
marcador sigue contando como un compás. `stretches()` es un buen
ejemplo de construir un iterador a partir de otros: los comienzos (el
primer compás, luego cada marcador) se `zip`ean con los finales (cada
marcador, luego `usize::MAX` para "nunca").

`bars()` da todos los compases sin fin, hecho con `std::iter::from_fn`,
que convierte una closure en un iterador. La lista de reproducción dibuja su
cuadrícula con `meter.bars().take_while(|bar| bar.start <= steps)`: el
iterador nunca se detiene solo, `take_while` decide cuánto de él usar.

## Quién tuvo que cambiar

Todo lo que antes preguntaba `project.time_signature` sobre compases
pregunta ahora `project.meter()`:

- la cuadrícula de la lista de reproducción, los números de compás y el ajuste, a
  través de `Snap::floor` y `Snap::next` (el final del compás o pulso
  en el que está el puntero, que es lo que necesita arrastrar una
  sección o un bucle);
- los conteos de compases de las secciones y la etiqueta "Repitiendo compases";
- la longitud de la canción, que se redondea hacia arriba a la
  siguiente línea de compás.

El rack de canales y el piano roll muestran un patrón, no la canción.
Preguntan `project.pattern_time(pattern)`: el compás donde empieza el
primer clip del patrón. Un patrón usado en el tramo en 3/4 se dibuja en
compases de tres. Los ritmos, los fills y la escucha de acordes de la
biblioteca usan lo mismo, así que aplicar un ritmo a un patrón en 3/4
lo coloca en compases de 3/4.

## La canción tal como suena

Las secciones pueden repetirse ([la estructura de la
canción](song-structure.md)). Si una sección en 3/4 suena dos veces, la
canción oída tiene dos tramos en 3/4, pero la lista de reproducción dibuja uno. Las
*pasadas* del arreglo ya dicen qué parte de la línea de tiempo suena
cuándo, así que `Project::played_meter` construye un segundo `Meter` en
pasos de canción: al comienzo de cada pasada, el compás en el que está
esa parte de la línea de tiempo, más los marcadores dentro de ella.
Luego `tidy()` descarta los marcadores que no cambian nada, como una
pasada en 4/4 que sigue a otra pasada en 4/4 sobre una línea de compás.

El transporte usa ese metro para la pantalla de posición y para RePág/AvPág, así que "compás siguiente" significa el siguiente compás
que oyes. La exportación MIDI lo usa para escribir un meta evento de
compás en cada cambio, los mismos cuatro bytes de antes, ahora en el
tick de cada cambio en lugar de solo en el tick 0.

Duplicar una sección copia también sus marcadores, y pone uno al
comienzo de cada copia, para que la copia se cuente como el original.
Luego los marcadores se limpian, así que una sección en 4/4 normal
copiada en una canción en 4/4 no añade ninguno.

## En los archivos

Los marcadores se guardan como `time_markers` tras la región de bucle.
Un proyecto con marcadores es **formato 5**: un Ferrous Lineup viejo se
saltaría el campo que no conoce y dibujaría todo en el primer compás,
así que debe rechazar el archivo en su lugar. Un proyecto sin
marcadores omite el campo y conserva su formato viejo, así que las
versiones viejas pueden seguir abriéndolo. Al cargar, todo marcador
pasa por `set_time_marker`, que rechaza uno demasiado tarde o cuyos
compases no cabrían en un patrón, como si lo hubieras añadido a mano.

## Canciones demo en dos compases

Una parte de una canción demo puede tener su propio compás:

```rust
Part::new("Coda", "...", "I | IV | V").in_time(THREE_FOUR)
```

`song_demo` escribe el patrón de esa parte en compases de tres, elige
batería en ese compás (el ritmo propio de la canción si encaja, otro
ritmo del mismo género si no), y pone un marcador donde empieza la
parte y otro donde vuelve el compás de la canción. El nombre `in_time`
coincide con el que ya usan los ritmos.
