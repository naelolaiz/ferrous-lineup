# Hojas de acordes como texto: para pegar y compartir

Archivos: [`crates/lineup-core/src/chord_sheet/text.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/chord_sheet/text.rs),
y en la app
[`crates/lineup-app/src/app/chord_text.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/chord_text.rs)
(menú Archivo) y
[`crates/lineup-app/src/chord_sheet.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/chord_sheet.rs)
(copiar y pegar)

Los músicos se envían charts de acordes como texto plano todo el
tiempo: en un correo antes de un ensayo, en un mensaje de un foro
sobre una canción, en una app de notas del teléfono. Nadie se puso de
acuerdo en un formato, pero la mayoría de los charts se parecen
bastante, porque copian cómo se imprime un chart de acordes: líneas de
compás entre los compases, los acordes de cada compás entre ellas, y
el nombre de cada sección sobre sus compases.

```text
Title: Autumn Song
Key: C major
Time: 4/4
Tempo: 120
Style: Bossa nova

[Verse ×2]
| C      | Am7 | Dm7 G7 | C  |
| F      | %   | G7     | C  |

[Chorus]
| F      | G   | Em7 A7 | Dm |
```

Este es el punto 10 del plan. Ferrous Lineup lee un chart así y hace
una canción con banda, escribe la hoja de acordes de cualquier canción de
la misma manera, y copia y pega compases de la hoja de acordes como texto.
Nada nuevo se guarda en la canción, así que el formato de archivo
sigue como estaba.

## Lo que ves

![El menú File con Import chord sheet… bajo Import MIDI file… y Export chord sheet… bajo Export pattern as MIDI…](../images/step-9-text-file-menu.png)

**Archivo > Importar hoja de acordes…** abre un archivo `.txt` (o suelta uno
sobre la ventana) y hace una canción nueva con él: la banda toca los
acordes en la tonalidad, el compás, el tempo y el estilo del chart,
las secciones están en la lista de reproducción, y la hoja de acordes se abre para que
veas lo que se leyó. Si la canción abierta tiene cambios sin guardar,
primero se te pregunta, como con **Abrir**. La canción nueva se llama
por el título del chart (o el nombre del archivo) hasta que la
guardas, y **Guardar como…** sugiere ese nombre. Una canción importada
desde un archivo MIDI ahora conserva igual el nombre de su archivo.

![Autumn Leaves importado desde texto: el chord sheet muestra el estilo de la banda (Swing), la tonalidad Sol mayor, 132 BPM, la sección A ×2 sobre los compases 1 a 8 y la sección B desde el compás 9, cada acorde con su numeral; la barra de estado dice «Importado autumn.txt: 24 compases»](../images/step-9-text-chart-imported.png)

Si algo del chart no se puede leer, no se importa nada, y la barra de
estado dice dónde:

> No se pudo importar bad.txt: línea 4: compás 3: «Qx7» no es un acorde
> (Am7, G/B), un conjunto (3-11 T2, {0,4,7}), un micromodo (Menor 3),
> una forma de serie (P0, RI7) ni N.C.

**Archivo > Exportar hoja de acordes…** escribe la hoja de acordes de la canción en
un archivo de texto, en el formato de arriba, con los compases
alineados en columnas para que el texto se lea como la ventana.

En la propia ventana de la hoja de acordes:

- **Ctrl+C** copia los compases seleccionados (haz clic en uno,
  Mayús+clic en otro para seleccionar varios) como texto, «| Am7 | D7 |
  Gmaj7 | Cmaj7 |». Con todos los compases seleccionados, o ninguno,
  copia toda la hoja de acordes con sus ajustes y secciones, como lo escribe **Exportar hoja de acordes…**.
- **Ctrl+A** selecciona todos los compases.
- **Ctrl+V** escribe el texto del portapapeles desde el compás
  seleccionado en adelante, y selecciona los compases que escribió.
  Una línea sin ningún `|` se lee como un compás, como si lo hubieras
  tecleado ahí, así que «Dm7 G7» copiado de un mensaje va directo a un
  compás.

Un pegado es un solo cambio de la hoja de acordes, así que **Ctrl+Z** lo
deshace de una vez. El mensaje sobre los compases dice qué se copió o
pegó, y desaparece cuando haces clic en otro compás.

![Veinticuatro compases pegados en una canción nueva en Do mayor: los mismos nombres de acorde, ahora con sus numerales en Do (Am7 vi7, D7 V7/V, F#m7b5 #ivø7, B7 V7/iii). [Dominantes secundarias](chord-functions.md#dominantes-secundarias-v-de-v) como V7/V se nombran por el acorde al que llevan](../images/step-9-text-pasted.png)

Pegar solo lleva los acordes. La canción conserva su propia tonalidad,
compás, tempo, banda y secciones; el Autumn Leaves pegado de arriba
está en Sol mayor, pero la canción a la que entró está en Do mayor, así
que sus numerales se leen en Do. Para traer un chart entero con sus
ajustes, impórtalo en su lugar.

## El formato

Todo es opcional salvo los compases.

| Línea | Significa |
|------|-------|
| `Title: Autumn Leaves` | El nombre de la canción. Una primera línea que no es otra cosa también es el título, como arriba de un chart impreso. |
| `Key: G major` | La tonalidad. `G`, `Em`, `E minor`, `A Dorian`, `C harmonic minor` y las otras escalas del menú de tonalidad (no series ni conjuntos, que se eligen en la app). Una tonalidad con dos nombres conserva el que se escribió: `Gb major` sigue como Sol♭ mayor, `F# major` sigue como Fa♯ mayor. |
| `Time: 3/4` | El compás, cualquiera que la app acepte: `6/8`, `5/4`, `2+2+3/8`. |
| `Tempo: 132` | Negras por minuto; `132 BPM` también se lee. |
| `Style: Bossa nova` | El estilo de la banda, por su nombre en la biblioteca. |
| `[Verse]` | Una sección empieza con el siguiente compás. `[Chorus ×2]`, `[Chorus x2]` o `[Chorus] x2` la toca dos veces. |
| `\| C \| Am7 \| Dm7 G7 \|` | Compases. Una línea puede tener cualquier número de ellos. |
| `# Ask Ana about the bridge` | Una nota para las personas, que se salta. |

Dentro de un compás va exactamente lo que tecleas en un compás de la hoja de acordes ([Escribir en un compás](chord-sheet.md#escribir-en-un-compás)):
acordes, numerales romanos, conjuntos, micromodos, formas de serie o
`N.C.`, repartidos por los pulsos con la misma regla, y `/` para un
pulso que sostiene la armonía anterior. Dos signos más vienen de los
charts impresos:

- **`%`** es el signo de **simile**, impreso como una barra entre dos
  puntos (𝄎). Significa «toca otra vez el compás anterior». Un compás
  vacío, `| |`, significa lo mismo.
- **`||`**, **`|:`** y **`:|`** son una doble barra de compás y signos
  de repetición. Se leen como líneas de compás normales; las
  repeticiones se escriben con `×2` en una sección, que es como las
  guarda la lista de reproducción.

Cuando no hay una línea `Style:`, la banda toca un estilo del compás
del chart, así que un chart en 3/4 recibe el jazz waltz, y sin compás
tampoco, el estilo con el que empieza una canción nueva. Cuando no hay
ningún estilo del compás del chart (5/4, por ejemplo), la banda toca
el estilo con el que empieza una canción nueva, en el compás del
chart, y la barra de estado lo dice: «La banda toca Ritmo pop, un
estilo en 4/4, en 5/4». Una línea `Key:` o `Time:` entre compases
cambia la tonalidad o el compás desde el siguiente compás en adelante,
como lo hace un marcador de tonalidad o de compás en la lista de reproducción.

Los numerales romanos se leen en la tonalidad del chart. Este chart

```text
Key: F major
| I | vi | ii7 V7 | I |
```

se convierte en F, Dm, Gm7 C7, F, el turnaround más común que hay, en
Fa. El mismo texto con `Key: D major` daría D, Bm, Em7 A7, D: una sola
progresión en cualquier tonalidad, que es por lo que a los músicos
que tocan de oído les gustan los numerales.

## Leer el texto

`text::read` convierte el texto en un `TextSheet`, y solo comprueba la
disposición. Aún no puede leer los compases: cuántos pulsos tiene un
compás depende del compás vigente en ese punto, que solo la canción
conoce una vez que las líneas `Time:` anteriores están en su lugar.

```rust
pub struct TextSheet {
    pub title: Option<String>,
    pub key: Option<Scale>,
    pub time: Option<TimeSignature>,
    pub tempo: Option<f32>,
    pub style: Option<(String, usize)>,
    pub items: Vec<Item>,
}

pub enum Item {
    Bar { text: String, line: usize },
    Section { name: String, repeats: usize, line: usize },
    Key(Scale),
    Time(TimeSignature),
}
```

`Item` es un enum cuyas variantes guardan cosas distintas: dos tienen
campos con nombre, como un struct, y dos guardan un valor, como una
tupla. Un `match` sobre un `Item` tiene que decir qué hace con cada
una de las cuatro, así que cuando se añade un quinto tipo de línea, el
compilador enumera cada lugar que debe enterarse de él.

Los compases y las secciones conservan el número de su línea, para que
un problema encontrado más tarde, cuando se lee el compás, pueda aún
nombrarla. Las líneas se numeran con `enumerate`, que cuenta desde 0,
así que el número es `index + 1`:

```rust
for (index, line) in text.lines().enumerate() {
    let number = index + 1;
    let line = line.trim();
    if line.is_empty() || line.starts_with('#') {
        continue;
    }
    ...
}
```

El error es un struct pequeño con la línea y el mensaje, y
`impl fmt::Display` dice cómo se imprime, «línea 4: …». Toda función
que puede fallar devuelve `Result<_, TextError>`, y `?` sube el error
de inmediato. `map_err` convierte otro error en este por el camino:

```rust
let key = read_key(value).map_err(|message| error(number, message))?;
```

### Ajustes, secciones y compases

Una línea es un ajuste cuando tiene dos puntos y la palabra anterior
es uno de los cinco ajustes, sea cual sea su mayúscula.
`split_once(':')` parte «Key: G major» en el primer dos puntos en
`("Key", " G major")`, y devuelve `None` cuando no hay dos puntos:

```rust
fn setting(line: &str) -> Option<(String, &str)> {
    let (name, value) = line.split_once(':')?;
    let name = name.trim().to_lowercase();
    ["title", "key", "time", "tempo", "style"]
        .contains(&name.as_str())
        .then_some((name, value))
}
```

`bool::then_some(value)` es `Some(value)` cuando el bool es true y
`None` si no, una manera corta de terminar una función que conserva o
descarta su respuesta.

Un encabezado de sección puede poner su cuenta dentro de los corchetes
o después. `read_times` lee «×2», «x2» o «X 2»:

```rust
fn read_times(text: &str) -> Option<Option<usize>> {
    let number = text
        .strip_prefix(['×', 'x', 'X'])?
        .trim()
        .parse::<usize>()
        .ok()?;
    Some((1..=MAX_REPEATS).contains(&number).then_some(number))
}
```

`strip_prefix` toma un array de caracteres y quita aquel con el que
empiece el texto. La respuesta es una option dentro de una option,
porque hay tres casos que distinguir: `None` es «esto no es una
cuenta» (el encabezado sigue con otras palabras), `Some(None)` es «una
cuenta, pero demasiadas veces» (un error), y `Some(Some(2))` es dos
veces. Donde se lee el encabezado, dos condiciones `let` se encadenan
con `&&`, algo que Rust permite desde su edición 2024:

```rust
if repeats.is_none()
    && let Some((first, last)) = name.rsplit_once(char::is_whitespace)
    && let Some(times) = read_times(last)
{
    repeats = Some(times);
    name = first.trim().to_string();
}
```

El bloque corre solo si todas las condiciones se cumplen, y los
nombres ligados por el camino (`first`, `last`, `times`) pueden usarse
en las condiciones posteriores y en el bloque. Sin la cadena serían
tres `if` anidados.

Las líneas de compás se allanan primero, luego la línea se parte en
cada `|`. Los trozos antes del primer signo y después del último
quedan vacíos cuando la línea empieza y termina con una línea de
compás, así que se descartan; cuando no están vacíos, son compases,
así que «C | G» son dos compases.

## Hacer la canción

`text::import` parte de `Project::with_band()`, la canción que hace
**Archivo > Nuevo**, y fija lo que dice el chart, en el orden en que la
banda lo necesita: primero el estilo (que puede cambiar la métrica de
la canción vacía a su propio compás), luego encima el compás, la
tonalidad y el tempo del chart. Luego recorre los items una vez,
llevando el paso donde empieza el siguiente compás:

```rust
Item::Bar { text, line } => {
    bars.push((step, text, *line));
    step += time.steps_per_bar();
}
Item::Time(new) => {
    time = *new;
    project.set_time_marker(TimeMarker::new(step, time));
}
Item::Key(key) => project.set_key_marker(KeyMarker::new(step, *key)),
Item::Section { name, repeats, .. } => sections.push((name.clone(), *repeats, step)),
```

Una sección termina donde empieza la siguiente, o al final de la
canción. Una vez que cada marcador está en su lugar, cada compás se
lee con `read_bar`, que recibe el compás (para conocer sus pulsos) y
la tonalidad de su inicio (para leer los numerales). Todos los acordes
entran en la canción en un solo `set_chords`, y `follow_chord_sheet`
le dice a la banda que escriba sus partes.

`text::paste` hace la misma lectura en una canción existente, desde un
compás en adelante, y devuelve los marcadores en lugar de escribirlos.
La ventana los convierte en un solo `SheetAction::SetChords`, la misma
acción que teclear en un compás, así que el undo, la banda y la lista de reproducción no necesitan nada nuevo.

## Escribir el texto

`text::write` recorre los compases de la hoja de acordes con `write_bar` (la
función que llena el campo de texto de un compás cuando haces clic en
él) y junta líneas: palabras («[Verse ×2]», «Key: D minor») o una fila
de hasta cuatro compases. Una sección, un cambio de tonalidad o de
compás empieza una fila nueva, y un compás vacío se escribe `%`.

Luego `layout` alinea los compases. Primero encuentra el compás más
ancho de cada una de las cuatro columnas, y después rellena cada
compás hasta el ancho de su columna:

```rust
for (width, bar) in widths.iter_mut().zip(bars) {
    *width = (*width).max(bar.chars().count());
}
...
.map(|(bar, width)| format!("{bar:<width$}"))
```

`{bar:<width$}` escribe `bar` alineado a la izquierda (`<`) en un
campo tan ancho como la variable `width` (el `$` dice que el ancho
viene de una variable). Los anchos se cuentan con `chars().count()`,
no con `len()`: `len()` cuenta los bytes del texto UTF-8, y una letra
fuera del ASCII simple (ø, ×, ♭) ocupa dos o tres bytes pero un lugar
en la pantalla.

Una tonalidad que el texto no puede decir, una serie dodecafónica o un
conjunto de clases de alturas, se escribe como una nota para las
personas, «# Key: … (chosen in the app)», para que el chart siga
leyéndose y nadie se engañe.

## Copiar y pegar en la ventana

egui convierte Ctrl+C y Ctrl+V en eventos propios,
`egui::Event::Copy` y `egui::Event::Paste(text)`, con el texto ya
tomado del portapapeles del sistema. Es igual en Linux, macOS (donde
las teclas son Cmd+Do y Cmd+V) y Windows. La hoja de acordes los busca
entre los eventos del frame mientras tiene el teclado y no se está
tecleando en un compás; mientras se teclea en un compás, van al campo
de texto como siempre.

```rust
for event in &input.events {
    match event {
        egui::Event::Copy => copy = true,
        egui::Event::Paste(text) => paste = Some(text.clone()),
        _ => {}
    }
}
```

Para copiar, `ctx.copy_text(text)` pone el texto en el portapapeles al
final del frame.

## Dos errores viejos, encontrados por un viaje de ida y vuelta

La mejor prueba de un par de funciones como `write` y `read` es un
**viaje de ida y vuelta**: escribir algo, volver a leerlo y comprobar
que obtuviste lo mismo. La prueba `every_song_demo_comes_back_from_its_text`
hace eso con cada demo de canción: la hoja de acordes de cada una se
escribe como texto, se importa, y sus acordes, tonalidades y compases
se comparan con los de la demo.

Encontró dos errores que ya estaban en la ventana de la hoja de acordes
([La hoja de acordes](chord-sheet.md)), ambos en compases raros:

1. **«C G7 C /»**, tres acordes en los tres primeros pulsos y el
   último pulso sosteniendo el tercero. `write_bar` descartaba las
   barras del final de un compás, ya que «C / G7 /» dice lo mismo que
   «C / G7». Pero «C G7 C» sin ninguna barra significa tres acordes
   repartidos por el compás, en los pulsos 1, 3 y 4, así que hacer
   clic en un compás así y salir de él movía dos acordes. Una barra
   final ahora se conserva cuando es la última del compás.
2. **"A E / B"**, A, luego E sostenido dos pulsos, luego B. Al buscar
   nombres de varias palabras ([Nombres de varias
   palabras](chord-sheet.md#nombres-de-varias-palabras)), `read_bar`
   probaba «E / B» y encontraba el acorde con barra E/B. Un `/` que va
   solo es un pulso, así que un nombre ahora nunca cruza por uno;
   «E/B» escrito junto sigue siendo el acorde con barra.

Unas pocas demos tienen un acorde entre dos pulsos (Happy Birthday,
por ejemplo), algo que una hoja de acordes no puede escribir, así que la
prueba compara los acordes colocados en los pulsos, y comprueba que
menos de una demo de cada diez tiene un acorde así.

## Ejercicios

1. **Acordes a contratiempo.** Algunos charts de texto escriben una
   anticipación, un acorde que llega medio pulso antes, con un signo como `>`
   o `^`. ¿Qué función lo leería, y qué tendría que guardar
   `ChordMarker::start`?
2. **El título al exportar.** Una canción guardada como
   `leaves.lineup` se exporta con «Title: leaves», el nombre del
   archivo. Guarda el título leído de un chart en la propia canción,
   para que vuelva al exportar. ¿Por qué hace falta un número de
   formato de archivo nuevo, y qué leería un archivo viejo?
3. **La tonalidad al pegar.** Cuando el texto pegado tiene una línea
   `Key:` distinta de la tonalidad de la canción, los acordes
   conservan sus nombres. Ofrece transponerlos a la tonalidad de la
   canción en su lugar. ¿Qué función de `lineup_core` ya mueve acordes
   de una tonalidad a otra?
4. **Más columnas.** Los charts en un teléfono se leen mejor con dos
   compases por línea. Haz del número de compases por línea un
   parámetro de `write`. ¿Qué hay en `layout` que da por hecho cuatro?
5. **Otros formatos.** ChordPro escribe los acordes dentro de la
   letra, como en `[C]Autumn [Am7]leaves`. Esboza un lector para sus
   líneas `{title: …}` y `{key: …}`. ¿Podría devolver un `TextSheet`?
