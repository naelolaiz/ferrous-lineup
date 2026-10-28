# Los estilos conservan el tempo de la canción

Archivos: [`crates/lineup-core/src/library/apply.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/apply.rs),
[`crates/lineup-core/src/remeter.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/remeter.rs),
[`crates/lineup-app/src/library_browser.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/library_browser.rs),
[`crates/lineup-app/src/app/library.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/library.rs)
y [`crates/lineup-app/src/app/ux_tests.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/ux_tests.rs)

Abre El Choclo (112 BPM), aplica **Tango de salón** desde la Library y
luego **Tango nuevo**. Ambos son tangos en 4/4, y nada del compás de la
canción cambia, sin embargo la casilla del tempo pasó de 112 a 132.
Cada nota de la canción se acortó: la melodía que conoces se aceleró,
solo porque querías oír otro acompañamiento.

Una banda de acompañamiento no se comporta así. A una banda a la que le
piden que lo toque «como tango nuevo» conserva el tempo del cantante.
Ahora Ferrous Lineup también: **un estilo conserva el tempo de la
canción**, y las propias notas de la canción duran lo mismo que antes,
en segundos, pruebes el estilo que pruebes.

![Las opciones de Tango nuevo: Tempo no está marcado, y el tempo sigue viniendo de El Choclo (demo).](../images/step-9-style-tempo-popup.png)

## Probarlo

1. Abre **File > Demos > El Choclo**. La casilla del tempo dice
   **112.0**.
2. En la **Library**, haz clic en **Tango de salón** y luego en
   **Apply**. Haz clic en **Tango nuevo** y luego en **Apply**. El
   tempo se queda en 112.
3. Haz clic en **Tango vals (3/4)** y en **Apply**. La canción está
   ahora en 3/4 y la casilla del tempo dice **84.0** (más abajo, por
   qué es la misma velocidad).
4. Aplica **Tango nuevo** otra vez: 4/4 y **112.0**, exactamente donde
   empezaste.

Si *quieres* el tempo propio de un estilo, marca **Tempo** en sus
opciones antes de **Apply**. Se usa cuando el compás no cambia.

## De dónde venían los saltos

Cada elección de la Library tiene una lista de partes que puede traer:
los patrones que toca, los instrumentos, el compás, los acordes, el
tempo y el swing. Son los campos de `StyleParts` en `apply.rs`, y las
casillas de las opciones de cada estilo. `tempo` empezaba **marcado**,
así que cada estilo traía su «tempo habitual»: 112 para Tango de salón,
132 para Tango nuevo, 180 para Tango vals.

Las marcas también se recuerdan entre ejecuciones, así que el Tempo
marcado volvía cada vez que la app arrancaba. Y con cambios de compás
por medio, los saltos se acumulaban. Una canción que cambia de métrica
conserva cada compás tan largo como era (más abajo), y el estilo
siguiente, en la métrica nueva, tomaba su tempo propio a partir de ahí.
En una ejecución, El Choclo pasó por unos cuantos estilos y volvió a
4/4 a **240** BPM: más del doble de su velocidad.

## Un cambio de métrica conserva la velocidad

El tempo cuenta negras. Un compás de 4/4 tiene cuatro; uno de 3/4,
tres. Cuando un estilo en 3/4 pone la canción en 3/4 (el compás n de la
canción pasa a ser el compás n del vals, mira [cambiar la
métrica](../step-8/re-metering.md)), el tempo baja en la misma
proporción, para que cada compás dure exactamente lo mismo que antes:

```text
4/4 a  112:  4 pulsos × 60/112 s = 2.14 s por compás
3/4 a   84:  3 pulsos × 60/84  s = 2.14 s por compás
```

Así que 84 en 3/4 no es una canción más lenta: es la misma canción,
contada en tres. Cada nota conserva su lugar en su compás, así que
empieza en el mismo momento y dura lo mismo. La [traza de la
reproducción](playback-trace.md) lo muestra. Estas son las primeras
notas de la melodía de El Choclo, inicio + duración en segundos:

```text
4/4 a  112:  1.74+0.13  1.88+0.13  2.01+0.13  2.14+0.13 …   canción 98.9 s
3/4 a   84:  1.74+0.13  1.88+0.13  2.01+0.13  2.14+0.13 …   canción 98.9 s
```

Lo que cambia es cómo se escriben las notas, no cómo suenan: un compás
de 3/4 tiene doce pasos en lugar de dieciséis, así que una nota cubre
menos pasos en el piano roll, y cada paso dura más. La batería y el
acompañamiento son lo que lo hace sonar como un vals.

![El Choclo después de Tango vals: 3/4, tempo 84.](../images/step-9-style-tempo-waltz.png)

## Qué cambió en el código

### El valor por defecto

```rust
impl Default for StyleParts {
    fn default() -> Self {
        Self {
            rhythm: true,
            instruments: true,
            time_signature: true,
            progression: false,
            tempo: false, // was true
            swing: true,
            matching_style: true,
        }
    }
}
```

El struct también tiene `#[serde(default)]`. Cuando se vuelve a leer
desde los ajustes guardados, un campo que falte ahí toma su valor de
`Default::default()`. Así se puede añadir un campo nuevo sin romper los
ajustes guardados por una versión anterior.

### Ajustes guardados con Tempo marcado

Cambiar el valor por defecto no basta: tu archivo de ajustes sigue
diciendo `tempo: true`, guardado porque estaba marcado para todo el
mundo. La app guarda sus preferencias con egui, bajo una clave (un
`egui::Id`). La corrección las mueve a una clave nueva,
`"library-options"`. Cuando esa clave está vacía, la app lee la
antigua, `"preset-copy-options"`, una vez, y desactiva Tempo:

```rust
self.parts = ctx
    .data_mut(|data| {
        data.get_persisted(egui::Id::unique(OPTIONS)).or_else(|| {
            data.get_persisted(egui::Id::unique(OPTIONS_TEMPO_ON))
                .map(|parts| StyleParts {
                    tempo: false,
                    ..parts
                })
        })
    })
    .unwrap_or_default();
```

Aquí trabajan dos piezas de Rust. `Option::or_else` ejecuta su closure
solo cuando la primera búsqueda no encontró nada. `StyleParts { tempo:
false, ..parts }` es *sintaxis de actualización de structs*: una copia
de `parts` con un campo cambiado. Cualquier otra elección que hicieras
(Plain accents, los acordes, el swing) se conserva. Si vuelves a marcar
Tempo, se guarda bajo la clave nueva y se queda marcado.

### El modo patrón sigue la misma regla

La casilla de Tempo dice: «Use the preset's usual tempo when the meter
stays the same». En modo canción eso ya era cierto. En modo patrón,
un estilo probado en un patrón de prueba tomaba su tempo incluso cuando
el patrón pasaba a otra métrica. Ahora el código compara el compás del
patrón antes y después de escribirlo, y toma el tempo solo si es el
mismo:

```rust
let time = project.pattern_time(index);
// … the style is written into the pattern …
if parts.tempo && project.pattern_time(index) == time {
    project.scale_tempo(Tempo::new(tempo));
}
```

### 124, no 124,00001

Un tempo es un `f32`, un número de coma flotante con unos siete dígitos
significativos. Cambiar la métrica solía multiplicar el tempo por la
razón entre las longitudes de los compases, `16.0 / 14.0` de 7/8 a
4/4. Esa razón no tiene un valor `f32` exacto (es 1,142857…, para
siempre), así que 108,5 BPM en 7/8 volvía a 4/4 como 124,00001 en lugar
de 124. Nadie oye la diferencia, pero la canción ya no está exactamente
donde empezó. La corrección multiplica primero y divide al final, en
`f64`:

```rust
let scaled = |tempo: Tempo, step: usize| {
    Tempo::new((f64::from(tempo.bpm()) * bar as f64 / old_bar(step) as f64) as f32)
};
```

`108.5 × 16 = 1736` es exacto, y `1736 / 14 = 124` también lo es. Una
canción puesta en 7/8, 3/4 o 13/16 y devuelta vuelve ahora exactamente
a su tempo. Una prueba lo comprueba para varios tempos y métricas.

## Las pruebas

`styles_tried_one_after_another_keep_the_songs_tempo` lleva a El Choclo
por siete cambios: Four chords, Emotional pop, Tango vals, Ranchera,
Six-eight ballad, el selector Time (7/8) y Four chords otra vez. Lo
hace tres veces: en modo patrón, en modo canción detenida y en modo
canción en reproducción. Después de cada clic, el BPM debe quedar igual
si la métrica no cambia, y ser el BPM propio de la canción siempre que
vuelva a 4/4. Luego la prueba reproduce el comienzo de la canción con
la traza activada y comprueba que cada nota de la melodía empieza y
dura igual que antes, con un margen de 2 ms.

Reproducir una canción entera para una traza lleva unos segundos, y
muchos más en una compilación debug, que es como CI ejecuta las
pruebas. Así que la prueba reproduce solo los primeros doce segundos, a
8000 muestras por segundo en lugar de 48000. Eso basta de sobra para
medir las notas, y la prueba tarda unos seis segundos en una
compilación debug.

Una cosa sigue moviendo a propósito las propias notas de la canción: el
**swing**. Un estilo con swing toca tarde la segunda corchea de cada
pulso, también en la melodía (mira [Swing](swing.md)). Si el swing
debería aplicarse solo a la banda es otra cuestión.
