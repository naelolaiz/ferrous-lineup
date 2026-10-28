# Casillas de la Library: presets como la canción, y un ayudante para probarlos

Archivos: [`crates/lineup-app/src/library_browser.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/library_browser.rs),
[`crates/lineup-app/src/library_browser/catalogue.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/library_browser/catalogue.rs),
[`crates/lineup-app/src/app/library.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/library.rs)
y [`crates/lineup-app/src/app/ux_tests.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/ux_tests.rs)

La Library contiene unos cientos de presets: estilos de armonía, ritmos,
tus propios estilos, fills, estilos de filas dodecafónicas y estilos de
micromodos. El cuadro de búsqueda encuentra uno por nombre, género o
compás. Pero a menudo la pregunta no es «¿dónde está la bossa nova?».
Es «¿cuál de estos encajaría con **esta** canción?». Un vals no lo
hará, si la canción está en 4/4; el Drum and bass beat, hecho para 174
BPM, no encajará con una balada a 70.

Tres casillas bajo la búsqueda reducen ahora la lista a presets como la
canción:

- **In 4/4**: solo presets en el compás de la canción, y los que tocan
  cada compás en su propio metro (como Orchestra). La etiqueta muestra
  el compás de la canción, sea cual sea.
- **Near 112 BPM**: solo presets hechos para más o menos la velocidad
  de la canción, como mucho 1,15 veces más rápidos o más lentos. La
  etiqueta muestra el tempo de la canción.
- **With chords**: solo presets que traen una progresión de acordes. Se
  oculta donde todo preset trae una, como en el menú Progression de la
  hoja de acordes.

![La Library para El Choclo con In 4/4 y Near 112 BPM marcadas: quedan 72 presets.](../images/step-9-library-filters.png)

## Probarlo

1. Abre **File > Demos > El Choclo** (4/4, 112 BPM) y la pestaña
   **Library**. El encabezado dice **Presets** y cuántos hay.
2. Marca **In 4/4**. Los valses y los estilos en 6/8 se van, y el
   número baja.
3. Marca **Near 112 BPM**. Lo que queda está hecho para 97 a 129 BPM.
4. Pon el tempo a 20 y mira otra vez: nada está hecho para eso, así que
   la Library dice «Nada en la biblioteca encaja con las casillas
   marcadas» y «Desmarca una casilla para ver más».

Las marcas se guardan con el estado de la app, así que siguen ahí la
próxima vez que la arranques. Reducen también los fills, los estilos de
filas y los estilos de micromodos.

## Reducir, en un solo struct

Las marcas son tres `bool`s:

```rust
#[derive(Clone, Copy, Default, PartialEq, serde::Serialize, serde::Deserialize)]
struct Filters {
    /// In the song's time signature.
    time: bool,
    /// Made for about the song's speed.
    speed: bool,
    /// Bringing a chord progression.
    chords: bool,
}
```

`Default` da los tres `false`, nada marcado. `PartialEq` deja que el
browser compare las marcas antes y después de dibujar las casillas, y
las guarde solo cuando cambiaron. `Serialize` y `Deserialize` dejan que
egui las guarde con `insert_persisted`.

Para decidir, las marcas necesitan aquello contra lo que comparan: el
compás y el tempo de la canción. Un segundo struct guarda las marcas y
esos dos juntos, y responde una pregunta:

```rust
impl Narrowing {
    fn keeps(self, time: TimeSignature, tempo: f32, any_meter: bool, chords: bool) -> bool {
        let filters = self.filters;
        // 7/8 as 2+2+3 is still 7/8.
        let same_time = any_meter || time.ungrouped() == self.time.ungrouped();
        let ratio = tempo / self.tempo;
        let near = ratio.max(1.0 / ratio) <= NEAR_SPEED;
        (!filters.time || same_time) && (!filters.speed || near) && (!filters.chords || chords)
    }
}
```

Lee la última línea como tres reglas unidas por «y»: `!filters.time ||
same_time` dice «o la casilla no está marcada, o el compás es el
mismo». Una casilla sin marcar deja pasar todo.

Tres detalles:

- **Metros agrupados.** Un compás puede llevar su agrupación, como
  «7/8 (2+2+3)» (mira [Cualquier
  compás](../step-8/any-time-signature.md)). La agrupación dice dónde
  caen los acentos, no cuánto dura el compás, así que `ungrouped` la
  quita antes de comparar.
- **Más rápido o más lento, igual.** `ratio.max(1.0 / ratio)` convierte
  «la mitad de rápido» (0,5) y «el doble de rápido» (2,0) en el mismo
  número, 2. Así que un preset 1,15 veces más lento y uno 1,15 veces
  más rápido quedan ambos justo dentro. A 112 BPM eso va de
  112 / 1,15 ≈ 97 a 112 × 1,15 ≈ 129. Comparar cocientes en lugar de
  una diferencia de BPM trata igual a una canción lenta y a una rápida:
  ±15 BPM sería mucho a 60 y poco a 180.
- **`self`, no `&self`.** `Narrowing` es `Copy`: tres marcas, un compás
  y un `f32`, unos pocos bytes. Tomarlo por valor no cuesta más que una
  referencia, y llamar a `keeps` no lo agota: un valor `Copy` se copia,
  de modo que el siguiente preset también puede comprobarse con él.

El mismo `keeps` filtra toda lista. Los presets se filtran con
`Vec::retain`, que conserva en el sitio los elementos a los que un
closure dice que sí:

```rust
presets.retain(|preset| {
    narrowing.keeps(
        preset.time(),
        preset.tempo(),
        preset.follows_the_song(),
        preset.has_progression(),
    )
});
```

Los fills, los estilos de filas y los estilos de micromodos se filtran
a medida que se listan, con `Iterator::filter`. Cuando una búsqueda y
las marcas juntas no dejan nada, el browser dice cuál lo hizo: «Nada en
la biblioteca encaja» con la búsqueda, o «las casillas marcadas», con
«Desmarca una casilla para ver más».

## Un ayudante para probarlos: un pequeño refactoring

Hacer clic en un preset en modo patrón lo **prueba**: la Library lo
escribe en un patrón propio y lo reproduce allí, de modo que la canción
no se toca. Hay tres clases de preset con las que puede pasar esto
—estilos de armonía, estilos de filas dodecafónicas y estilos de
micromodos—, y cada una tenía su propia copia de los mismos cinco
pasos:

1. seleccionar el patrón recién escrito,
2. recordar lo que estaba escrito ahí, para que el siguiente clic
   reescriba el mismo patrón mientras no lo hayas cambiado (conservando
   solo los últimos 64),
3. cambiar a modo patrón,
4. rebobinar,
5. y reproducir, si no estaba reproduciendo ya.

Tres copias de los mismos pasos son tres sitios que arreglar la próxima
vez que uno cambie, y tarde o temprano uno se olvida. Los pasos son
ahora un solo método:

```rust
/// Selects `pattern`, which the library has just written, and plays
/// it on its own. `audition` remembers what was written there, so
/// that the next click rewrites the same pattern while it is
/// unchanged; only the last [`REMEMBERED`] are kept.
fn play_written_pattern(&mut self, pattern: usize, audition: Option<Audition>) {
    self.select_pattern(pattern);
    if let Some(audition) = audition {
        if self.auditions.len() == REMEMBERED {
            self.auditions.remove(0);
        }
        self.auditions.push(audition);
    }
    self.set_mode(PlayMode::Pattern);
    if !self.playing {
        self.rewind_for_new_pattern();
        self.set_playing(true);
    }
}
```

Los estilos de filas y los estilos de micromodos compartían más: toda
la escritura. Elegir el patrón (el último escrito si está sin cambios,
o uno nuevo), limpiarlo, escribir las notas, la batería y el tempo, y
decirlo cuando no hay sitio para otro patrón. Solo «escribir las notas»
es distinto entre los dos: un estilo de fila escribe las formas de una
fila dodecafónica, un estilo de micromodo escribe micromodos. Así que
ese único paso se volvió un parámetro, un **closure**:

```rust
fn write_pattern_style(
    &mut self,
    name: &'static str,
    tempo: f32,
    rhythm: Option<&'static Rhythm>,
    write: impl FnOnce(&mut Project, usize),
) -> Option<(usize, Pattern)> {
    // … pick the pattern, clear it …
    write(project, index);
    // … the drums and the tempo …
}
```

y los dos llamadores pasan lo suyo:

```rust
self.write_pattern_style(style.name, style.tempo, style.rhythm(), |project, index| {
    // write the micromodes of `style` into pattern `index`
});
```

`impl FnOnce(&mut Project, usize)` acepta cualquier closure que pueda
llamarse una vez con esos dos argumentos. El compilador hace una copia
separada de `write_pattern_style` por cada closure que se le pase, de
modo que la llamada no cuesta más que si el código estuviera escrito.
`FnOnce` es el menos exigente de los tres traits de closure (`Fn`,
`FnMut`, `FnOnce`): solo promete llamarse una vez, de modo que el
closure puede mover valores fuera de lo que capturó. Todo closure es al
menos `FnOnce`, así que pedirlo acepta cualquier closure.

Nada de lo que ve el usuario cambió, que es el objetivo de un
refactoring. Pero algo tenía que mostrar que nada cambió. El camino de
los estilos de armonía tenía pruebas; el de las filas y los micromodos
no tenía ninguna. Así que el cambio vino con
`trying_row_and_micromode_styles_rewrites_one_pattern_while_it_is_unchanged`.
Prueba dos estilos de fila y luego un estilo de micromodo, comprueba
que todos aterrizan en el mismo patrón nuevo mientras está como lo
escribió la Library, que reproduce en modo patrón, y que cada prueba es
un paso de deshacer.

## Las pruebas

`the_library_tick_boxes_keep_only_presets_like_the_song` pone la
canción a 4/4 a 120 BPM y cuenta los presets que lista la Library:
todos, luego los que están en 4/4 (un vals se va, Orchestra se queda),
luego los que están en 4/4 hechos para 104 a 138 BPM. Arranca la app de
nuevo a 10 BPM, comprueba que las casillas siguen marcadas, que no se
lista nada y que la Library dice por qué, y que desmarcar la velocidad
trae de vuelta los presets en 4/4.

## Ejercicios

1. Marca **Near … BPM** en una canción a 60 BPM. ¿Qué rango conserva?
   Luego a 180. ¿Por qué habría sido peor ±15 BPM?
2. Añade una cuarta casilla, **Same genre as the song's style**, que
   conserve los presets del género del estilo que toca la banda. ¿Qué
   necesitaría saber `Narrowing`, y de dónde lo sacarías?
3. `write_pattern_style` toma `impl FnOnce`. ¿Funcionaría `impl Fn`
   para los dos closures que se le pasan hoy? ¿Qué podría hacer un
   closure futuro que `Fn` rechazaría?
