# Variaciones a y b: la banda se pone a fondo

Archivos: [`crates/lineup-core/src/band/variation.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/band/variation.rs),
[`crates/lineup-core/src/band.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/band.rs),
[`crates/lineup-core/src/project.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/project.rs),
[`crates/lineup-app/src/chord_sheet.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/chord_sheet.rs)
y [`crates/lineup-app/src/playlist.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/playlist.rs)

Una banda no toca igual todas las partes de una canción. La estrofa es
más ligera, para que se entiendan las palabras; el estribillo es más
lleno, y el baterista, el bajista y el pianista levantan la canción a
la vez. Un grupo de jazz toca el primer estribillo de una balada con
suavidad y luego empieza a swinguear más fuerte. Hasta ahora la banda
de Ferrous Lineup tocaba toda la canción con una sola energía. Ahora
todo estilo tiene dos variaciones: **a** es el estilo tal como era, y
**b** es el mismo estilo tocado más lleno.

![La hoja de acordes de una balada pop de ocho compases. El compás 5 tiene una «b» enmarcada. Su menú está abierto, con Variation a y Variation b, b elegida, y la pista: «Desde el compás 5, la banda toca el estilo más lleno: la batería en el ride, un bajo o piano más ocupado donde el estilo lo tiene, un poco más fuerte. La batería hace un fill hacia él».](../images/step-9-variations-sheet.png)

## Probarlo

Haz clic derecho en un compás de la hoja de acordes (**F5**) y elige
**Variation a** o **Variation b**. La banda toca esa variación desde
ese compás hasta el siguiente cambio, o hasta el final de la canción.
Para cambiar solo algunos compases, selecciónalos primero: haz clic en
el primero, Shift+clic en el último, y luego clic derecho en uno de
ellos. La selección se queda, y los compases de después conservan lo
que tocaban antes. Así es como tocas más lleno solo el puente de una
canción, o solo el último estribillo.

Una letra enmarcada marca cada cambio, en la hoja tras el número de
compás y en la regla de compases del playlist, como las letras de
ensayo impresas sobre el pentagrama en las partituras de banda. Elegir
la variación que ya está sonando no pone ningún marcador; devolver un
compás a lo de antes se lleva el marcador.

La batería anuncia cada cambio: el compás anterior es un **fill**, y el
primer pulso de la variación nueva recibe un platillo **crash**. Es la
misma señal que dan los bateristas reales, y por qué un oyente sabe que
«algo nuevo empieza aquí» incluso sin oír las palabras.

![El playlist de la misma balada: los compases 1 a 4 y 5 a 8 son patrones separados para cada músico, y la regla muestra una «b» enmarcada en el compás 5. Bass 2 tiene una nota en cada pulso, y Drums 2 está más ocupado.](../images/step-9-variations-playlist.png)

## Qué significa «más lleno»

La variación b no inventa un estilo nuevo. Abre el mismo, como lo hacen
los músicos:

| Quién | Variación a | Variación b |
|---|---|---|
| Batería | tiempo en el hi-hat cerrado | tiempo en el **ride**, salvo que el estilo ya lleve el tiempo en el ride |
| Batería, en una balada pop o rock | **cross-stick** en el aro de la caja | la caja completa |
| Bajo, un estilo de jazz tocado en dos, en 4/4 | **en dos**: fundamental y quinta, una nota cada dos pulsos | **walking**: una nota en cada pulso |
| Bajo, shuffle de blues | negras | la línea de **boogie**, arriba y abajo del acorde |
| Bajo, pop, rock, soul o folk en 4/4 | la fundamental mantenida | la fundamental en cada pulso |
| Piano, balada pop o rock | el acorde, y luego sus notas una a una | corcheas constantes |
| Todos | | un poco más fuerte: la velocity de cada nota una décima más alta |

Unas palabras de esa tabla:

- El **hi-hat** son dos platillos que el baterista cierra con un pedal.
  Cerrado, da un «tick» corto y seco. El **ride** es un platillo
  grande que vibra; las corcheas en él hacen una marea brillante que
  llena más el sonido. Pasar de uno al otro es la manera más común en
  que un baterista levanta un estribillo.
- Un **cross-stick** es la baqueta apoyada sobre la caja, con la punta
  golpeando el aro: un sonido suave, de madera. Golpear en su lugar el
  parche de la caja es mucho más fuerte. Las baladas están llenas de
  estrofas en cross-stick y estribillos con la caja completa.
- Un bajista de jazz **en dos** toca blancas, normalmente la
  fundamental y la quinta, lo que deja aire alrededor de una melodía
  tranquila. **Walking** llena cada pulso con una línea que lleva al
  siguiente acorde. Pasar de «dos» a «cuatro» es como una sección
  rítmica de jazz sube la temperatura sin tocar más fuerte.

Donde un estilo no tiene una manera más ocupada conocida de tocarse (un
tango, una bossa nova, un coral), la variación b conserva sus patrones
y solo cambia la batería y el volumen. Las reglas son pocas a
propósito, para que b suene siempre como la misma banda, no como una
canción distinta.

## Cómo funciona

### Marcadores que solo marcan cambios

```rust
pub struct VariationMarker {
    pub start: usize,
    pub variation: Variation,
}
```

Los marcadores viven en la `Band`, junto a su `Form`, porque solo una
banda toca variaciones. Un marcador que no cambia nada (variación a al
principio de la canción, o b justo después de otra b) solo ensuciaría
la hoja, así que tras cada cambio `tidy` conserva solo los reales:

```rust
let mut playing = Variation::A;
markers.retain(|marker| {
    let changes = marker.variation != playing;
    playing = marker.variation;
    changes
});
```

El closure que se le pasa a `retain` cambia `playing`, una variable
fuera de él. Un closure que cambia lo que captura es un `FnMut`: puede
llamarse muchas veces, pero no desde dos sitios a la vez, porque cada
llamada necesita la variable para sí. `retain` pide exactamente eso.

Qué variación suena en un paso es el último marcador en él o antes:

```rust
markers
    .iter()
    .rev()
    .find(|marker| marker.start <= step)
    .map_or(Variation::A, |marker| marker.variation)
```

`rev()` recorre la lista hacia atrás, de modo que `find` se detiene en
el primer marcador que no está después del paso.

### Tocar por tramos

El piano y el bajo solían tocar la canción entera en una llamada:
`style.comping.play_in_bars(&bars, &harmonies)`. Ahora los compases se
agrupan en **runs**, tramos en una misma variación, y cada run se toca
con la manera de tocar de esa variación:

```rust
fn played_in_runs(
    runs: &[Run],
    harmonies: &[Played],
    play: impl Fn(Variation, &[Bar], &[Played]) -> Vec<NoteEvent>,
) -> Vec<NoteEvent>
```

El piano pasa `|variation, bars, played|
style.comping_in(variation).play_in_bars(bars, played)`. Cada run recibe
solo sus propias armonías, cortadas en sus bordes con `cut`, la misma
función que usa el final. Una canción sin marcadores es un solo run,
tocado exactamente como antes, así que ninguna canción cambió su sonido
con esta función.

`comping_in` y `bass_in` son pequeños `match` sobre la variación, el
género del estilo y su manera de tocar:

```rust
match (variation, self.genre, self.bass) {
    (Variation::B, Genre::Jazz, Bass::RootFifth) if common => Bass::Walking,
    (Variation::B, Genre::Blues, Bass::Quarters) if common => Bass::Boogie,
    // ...
    (_, _, bass) => bass,
}
```

Hacer match sobre una tupla deja que una línea nombre una situación
entera, y la última rama conserva todo lo demás como estaba.

### La batería en otros tambores

La batería conserva los patrones del ritmo y cambia qué tambor los
toca: `drum_in(Drum::HiHat, Variation::B)` es `Drum::Ride` cuando el
ritmo no tiene ride propio. La banda escribe cada patrón en el canal
del tambor que debe sonar, de modo que las corcheas del hi-hat
aterrizan en el canal del ride. Un canal que la canción todavía no
tiene (un ride, un crash) se añade la primera vez que una variación lo
necesita, y nunca si no: las canciones sin marcadores conservan
exactamente los canales que tenían.

Cada cambio de variación empieza también un **chunk** nuevo, el tramo
que cubre un patrón, de modo que el playlist muestra las partes a y b
como clips separados que puedes ver, mutear o editar.

### Los marcadores se mueven con la canción

Todo lo que mueve la canción mueve también los marcadores: una intro
puesta delante (`make_room`), una intro quitada (`cut_start`), una
sección copiada (`copy_section`, que copia las variaciones dentro de
ella), un compás nuevo (`remeter`), y «End the song here», que descarta
los marcadores después del final. Una canción con marcadores se guarda
como formato 15.

## Ejercicios

1. Elige el estilo **Jazz ballad**, escribe | Cmaj7 | Am7 | Dm7 | G7 |
   dos veces, y pon los compases 5 a 8 en la variación b. Abre los
   patrones del bajo en el piano roll. ¿Cuántas notas tiene un compás
   en a, y en b? ¿Qué notas toca el bajo en un compás de Dm7 en la
   variación a?
2. En un **Pop beat** de ocho compases, pon el compás 5 en la variación
   b y el compás 6 de vuelta en a. ¿Qué compases reciben un fill de
   batería, y dónde caen los crash? ¿Por qué podría sonar inquieto eso
   en una canción real?
3. Elige **Swing** y toca unos compases en cada variación. ¿Qué cambia
   y qué se queda? ¿Por qué el hi-hat sigue siendo un hi-hat aquí?
4. (Rust) `tidy` ordena primero los marcadores, luego usa `dedup_by` y
   luego `retain`. ¿Qué saldría mal si llamara a `retain` antes de
   ordenar?
5. (Rust) `played_in_runs` toma `play: impl Fn(...)`. ¿Podría tomar un
   `FnMut` en su lugar? ¿Qué cambiaría en los sitios que lo llaman?
