# El chord sheet llena la ventana, con líneas que se ajustan a los nombres

Archivos: [`crates/lineup-app/src/chord_sheet/layout.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/chord_sheet/layout.rs),
[`crates/lineup-app/src/chord_sheet.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/chord_sheet.rs),
[`crates/lineup-app/src/app.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app.rs)
y [`crates/lineup-app/src/app/ux_tests.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/ux_tests.rs)

[El chord sheet](chord-sheet.md) ponía cuatro compases en cada línea,
en una ventana sobre la playlist. Cuatro compases es lo que suele tener
un chart impreso, pero el sheet ahora escribe nombres mucho más largos
que "C" y "G7": poliacordes como "Eb7|Fb", conjuntos como "3-11 T7I",
micromodos como "Acorde menor-mayor T7I", cada uno con un numeral o sus
notas debajo. Con cuatro de esos en un compás y cuatro compases por
línea, los nombres no cabían, y el sheet los hacía más pequeños hasta
que cupieran. En una pantalla ancha era al revés: una canción corta en
cajas vacías grandes, y una ventana que tapaba la playlist.

Dos cambios, ambos arriba del sheet, junto a **Show**:

- **Bars per line.** Por defecto, **Fit the names**: tantos compases
  como quepan en una línea con cada nombre y cada palabra debajo a su
  tamaño completo, nunca encogidos. El menú dice cuántos son ahora, como
  en **Fit (4)**. O elige un número de 1 a 16; entonces la línea
  contiene ese número, y los nombres que no caben se escriben más
  pequeños.
- **⛶ Fill the window.** El sheet ocupa el lugar de la playlist en la
  ventana principal, junto a los patrones y la Library. Con todo el
  ancho, una línea contiene más compases. **Back to the playlist** lo
  devuelve a su ventana. El mismo interruptor está en el menú **View**
  como **Chord sheet in the main window**.

Ambos se recuerdan de una ejecución de la app a la siguiente. Y **F5**
sigue abriendo y cerrando el sheet: cerrado y abierto de nuevo, vuelve a
donde estaba, en la ventana principal o en la suya.

![Embraceable You en el chord sheet, llenando la ventana principal junto a los patrones: cuatro compases por línea, donde su ventana tenía sitio para dos, y cada nombre y numeral a su tamaño completo.](../images/step-9-chord-sheet-fills-window.png)

## Pruébalo

1. Abre una canción desde **File > Demos**. Las canciones se abren en el
   chord sheet.
2. Haz la ventana del sheet más estrecha y más ancha por su esquina. Con
   **Fit the names**, las líneas contienen 1, 2, 4, 8, 12 o 16 compases,
   y el **Fit (N)** del menú lo sigue. Los nombres conservan su tamaño.
3. Haz clic en **⛶ Fill the window**. La playlist deja sitio al sheet, y
   las líneas se alargan. El channel rack, el piano roll y el mixer
   siguen abriéndose encima con F6, F7 y F9.
4. Elige **Bars per line > 16**. Los nombres que no caben en un
   dieciseisavo del ancho se hacen más pequeños, todos los nombres de un
   compás al mismo tamaño.
5. Elige otra vez **Fit the names**, y **Back to the playlist**.

![El menú Bars per line: Fit the names, o un número de compases de 1 a 16 (la lista se desplaza hasta 15 y 16).](../images/step-9-chord-sheet-bars-per-line.png)

## Por qué 1, 2, 4, 8, 12 o 16

El ajuste podría haber puesto cualquier número de compases en una línea:
cinco, siete, once. Un chart impreso nunca lo hace, y por una razón. La
mayoría de las canciones están hechas de **frases de cuatro compases**,
y un chart pone el primer compás de una frase al principio de una línea,
para que el ojo encuentre los compases 1, 5, 9 y 13 en la misma columna.
Con cinco compases por línea, la segunda frase empezaría en la segunda
columna de la segunda línea, la tercera en la tercera: la forma de la
canción se perdería.

Así que los números que ajusta son los que mantienen juntas las frases:
1 y 2 caben enteros en una frase, y 4, 8, 12 y 16 están hechos de frases
completas. Doce es un blues de doce compases en una línea.

```rust
/// The counts of bars a line holds when they fit, from the most: each
/// divides a four-bar phrase or is made of them.
const FITS: [usize; 6] = [16, 12, 8, 4, 2, 1];

/// No bar is made narrower than this to fit more on a line, however
/// short its names.
const NARROWEST_BAR: f32 = 110.0;
```

`NARROWEST_BAR` está para las canciones de nombres cortos. Un sheet de
"C" y "G" si no cabría dieciséis compases en una ventana que muestra
cuatro con holgura, y un compás demasiado estrecho para hacer clic o
teclear en él.

## La elección, como un tipo

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum Columns {
    /// As many as fit with every name at its full size.
    #[default]
    Fit,
    /// This many, from 1 to [`MOST_BARS`]: names that don't fit shrink.
    Bars(usize),
}
```

`#[default]` en una variante dice a `#[derive(Default)]` qué variante es
`Columns::default()`. Funciona en enums desde Rust 1.62; antes, el
`impl Default` había que escribirlo a mano.

El número sale de un método:

```rust
impl Columns {
    /// How many bars a line holds when it is `width` wide and the widest
    /// bar needs `needed` to show its names in full.
    pub fn count(self, width: f32, needed: f32) -> usize {
        match self {
            Columns::Bars(bars) => bars.clamp(1, MOST_BARS),
            Columns::Fit => {
                let bar = needed.max(NARROWEST_BAR);
                FITS.into_iter()
                    .find(|&bars| width / bars as f32 >= bar)
                    .unwrap_or(1)
            }
        }
    }
}
```

`FITS` va del mayor número de compases al menor, así que `find` se
detiene en el número más grande cuyos compases siguen siendo
suficientemente anchos. Si ni siquiera un compás cabe (un nombre muy
largo en una ventana muy estrecha), sigue habiendo un compás por línea,
y los nombres de ese compás se escriben más pequeños.

`FITS.into_iter()` itera sobre el array **por valor**, así que sus
elementos son `usize`. `find` entrega cada elemento a su closure por
referencia, como `&usize`, para poder probarlo sin quedárselo, y
devolverlo si es el bueno. El patrón `&bars` del parámetro del closure
saca el `usize` de vuelta de la referencia.

`count` es una función pura de dos números, así que su prueba no
necesita ninguna ventana:

```rust
// Short names: the narrowest bar decides.
assert_eq!(Columns::Fit.count(760.0, 60.0), 4);
assert_eq!(Columns::Fit.count(1300.0, 60.0), 8);
// Long names take more room, and whole phrases still start lines:
// room for three bars makes lines of two.
assert_eq!(Columns::Fit.count(760.0, 240.0), 2);
```

A 1300 píxeles, doce compases serían 108 píxeles cada uno, por debajo
del compás más estrecho; ocho son 162. A 760 píxeles con nombres que
necesitan 240, cabrían tres compases (253 cada uno), pero tres no es un
número que mantenga juntas las frases, así que son dos.

## Cuánto de ancho debe ser un compás

Antes de dibujar, el sheet pregunta a cada compás de la canción cuánto
debe medir de ancho, y se queda con el más ancho. Los nombres de un
compás no comparten su anchura a partes iguales: **cada nombre tiene la
parte del compás que va de su pulso al del siguiente nombre**, porque
ahí es donde lo dibuja el sheet. En un compás de 4/4 de "C La7", C tiene
los pulsos 1 y 2, la mitad del compás, y La7 lo mismo. En "C / / La7", C
tiene tres cuartos y La7 uno.

Así que un nombre de 30 píxeles de ancho que ocupa un cuarto del compás
necesita un compás de 120 píxeles:

```rust
let share = (end - starts[index]) as f32 / length;
let text = width(&item.text, NAME_SIZE);
let caption = item
    .caption
    .as_deref()
    .map_or(0.0, |caption| width(caption, CAPTION_SIZE));
(text.max(caption) + gap) / share
```

`width` mide el texto con las fuentes del painter (`layout_no_wrap`),
igual que se dibujará. La **caption** es la línea bajo el nombre: el
numeral romano de un acorde, o las notas de un conjunto ("C E G"), que a
menudo es la más ancha de las dos. `as_deref` convierte el
`Option<String>` en un `Option<&str>`, y `map_or` da 0 cuando no hay
caption. Un nombre seguido de otro guarda con él un hueco de 4 píxeles,
y el relleno a ambos lados del compás y cualquier signo de repetición se
añaden al final.

El más ancho de todos se saca con un **fold**:

```rust
.map(|bar| self.needed_width(ui.painter(), project, &meter, &keys, &bar, marks, &heard))
.fold(0.0, f32::max)
```

¿Por qué no `.max()`? Porque `Iterator::max` necesita `Ord`, un orden
total, y `f32` no tiene uno: `NaN` no es ni menor ni mayor que ningún
número. `f32::max` es una función normal que recibe dos `f32` y devuelve
el mayor (e ignora un `NaN`), así que hacer fold con ella desde `0.0` da
el compás más ancho, o 0 para una canción sin compases. Pasar `f32::max`
por nombre funciona porque una función es un valor que se puede llamar,
igual que lo sería un closure `|a, b| a.max(b)`.

Medir todos los compases cuesta un poco en cada fotograma: una canción
de 200 compases son 200 compases de texto medido. egui guarda en una
caché las galeras que ha compuesto hace poco, así que volver a medir el
mismo nombre es una búsqueda.

## Nombres que no caben, cuando eliges el número

Con un número elegido, un compás puede quedar demasiado estrecho para
sus nombres. Entonces se escriben más pequeños, con `draw_names`, según
tres reglas:

- **Todos los nombres de un compás a un mismo tamaño**, el que permite
  el más apretado. Un compás que mezcla nombres grandes y pequeños se
  lee como si los pequeños importaran menos.
- Un nombre que viene **de un compás anterior**, dibujado tenue al
  principio del compás, solo se encoge a sí mismo.
- Un conjunto o micromodo cuyo nombre no cabe ni al tamaño más pequeño
  se escribe solo por su **clase** ("Menor 3" en lugar de "Menor 3
  T8I"), y donde ni eso cabe, "…": un nombre cortado se leería como otro
  nombre.

## Ocupar el lugar de la playlist

La ventana principal tiene un panel central, donde se dibuja la
playlist. Cuando el sheet llena la ventana, ese panel dibuja el sheet en
su lugar, siempre junto a la lista de Patterns y la Library:

```rust
// The chord sheet can take the playlist's place, beside the
// pattern picker and the library.
self.chord_sheet.remember(&ctx);
let sheet_fills = self.chord_sheet.fills_window();
// …
let playlist = egui::CentralPanel::default_margins().show(ui, |ui| {
    if sheet_fills {
        let actions = self.playlist.browser(ui, &self.project, self.pattern, tried);
        let sheet = self.chord_sheet.main(ui, &self.project, playing, typing, /* … */);
        (actions, sheet)
    } else {
        let actions = self.playlist.show(ui, /* … */);
        (actions, Vec::new())
    }
});
```

Ambas ramas devuelven un par de listas de acciones: lo que pide la
playlist (o el browser junto al sheet) y lo que pide el sheet. El `if`
es una expresión, así que las dos ramas deben tener el mismo tipo; la
rama de la playlist devuelve un `Vec` vacío para las acciones del sheet.
La app aplica luego ambas listas de la misma manera, se haya dibujado lo
que se haya dibujado.

`ChordSheet::main` dibuja el mismo contenido que la ventana del sheet:
las mismas filas de botones, los mismos compases, las mismas
tonalidades. Solo cambia el marco que los rodea. Así que todo lo que el
sheet hace en su ventana lo hace en la ventana principal, sin una
segunda copia del código.

`remember` se llama **antes** que `fills_window`. Lee las elecciones
guardadas por la última ejecución, la primera vez que se llama. Si se
llamara después, el primer fotograma dibujaría la playlist y el segundo
el sheet: un destello de la vista equivocada cada vez que arranca la
app.

## Recordar las elecciones

Las dos elecciones son un struct pequeño, guardado con el estado de la
app:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
struct Choices {
    columns: Columns,
    /// Whether the sheet takes the playlist's place in the main window.
    fills: bool,
}
```

`#[serde(default)]` en el struct dice: un campo que falte en lo guardado
toma su valor de `Choices::default()`. Si una versión posterior añade
una tercera elección, el estado guardado por esta sigue cargando, con la
tercera elección en su valor por defecto, en lugar de fallar y perder
las dos primeras.

`LayoutView::remember` guarda una copia de lo último que guardó, y
escribe solo cuando las elecciones difieren de ella:

```rust
match self.saved {
    None => {
        self.choices = ctx
            .data_mut(|data| data.get_persisted(choices_id()))
            .unwrap_or_default();
        self.saved = Some(self.choices);
    }
    Some(saved) if saved != self.choices => {
        let choices = self.choices;
        ctx.data_mut(|data| data.insert_persisted(choices_id(), choices));
        self.saved = Some(choices);
    }
    Some(_) => {}
}
```

La rama `None` se ejecuta una vez, en el primer fotograma. La segunda
rama tiene un **match guard**, `if saved != self.choices`: la rama
coincide solo cuando el patrón coincide y la condición se cumple. Todo
lo demás cae en `Some(_) => {}`, nada que hacer, que es casi todos los
fotogramas.

## Las flechas siguen cayendo en la columna correcta

El primer capítulo del chord sheet terminaba con un ejercicio: hacer del
número de compases por línea un ajuste, y comprobar que las flechas
arriba y abajo siguen cayendo en la columna correcta. Lo hacen, porque
nunca contaron compases. El sheet guarda cada compás dibujado como una
celda con su **línea** y su **columna**, y la flecha abajo busca la
celda de la línea siguiente cuya columna esté más cerca de la
seleccionada:

```rust
let in_line = |line: usize, column: usize| {
    cells
        .iter()
        .filter(|cell| cell.line == line)
        .min_by_key(|cell| cell.column.abs_diff(column))
        .map(|cell| cell.bar.number)
};
```

Las celdas salen de lo dibujado, así que son correctas con cuatro
compases por línea, con dieciséis, y en una línea corta al final de una
sección. `min_by_key` funciona aquí, a diferencia del `max` de arriba,
porque la clave es un `usize`, que es `Ord`.

## Las pruebas

`lines_fit_as_many_whole_phrases_as_there_is_room_for`, en `layout.rs`,
comprueba `count` con los números de arriba.

En la app, `lines_hold_as_many_bars_as_show_every_name_in_full` abre una
canción cuyo primer compás es "Dm7/C F/C G7/D C/E", cuatro armonías en
un compás. Comprueba que el sheet contiene menos de cuatro compases por
línea y que cada nombre se dibuja a su tamaño completo, 20 puntos. Luego
elige 8: el menú ya no dice "Fit (8)", y "Dm7/C" es más pequeño. Elegir
otra vez **Fit the names** devuelve el primer número y el tamaño
completo.

`the_sheet_fills_the_main_window_and_goes_back` hace clic en **⛶ Fill
the window** y comprueba que la palabra "Playlist" ha desaparecido, que
"Patterns" sigue ahí, que las líneas contienen más compases que en la
ventana y que los nombres están enteros. Cierra y abre el sheet con F5,
reinicia la app y encuentra el sheet en la ventana principal cada vez.
**Back to the playlist** y el elemento del menú View lo rematan.

## Ejercicios

1. Una canción en 3/4 a menudo se escribe en chart con ocho compases por
   línea, un vals incluso dieciséis. ¿Debería `FITS` depender del
   compás? ¿Qué le pasarías a `count`?
2. `needed_width` hace que el compás más ancho decida por todo el sheet.
   Haz que decida por **sección**, para que una estrofa de acordes
   cortos pueda tener más compases por línea que un puente de
   poliacordes. ¿Qué les pasa a las flechas arriba y abajo entre
   secciones?
3. Escribe `widest(values: impl Iterator<Item = f32>) -> Option<f32>`
   que devuelva `None` sin valores, con `reduce` en lugar de `fold`.
   ¿Por qué encaja mejor `reduce` cuando no hay un valor inicial
   natural?
