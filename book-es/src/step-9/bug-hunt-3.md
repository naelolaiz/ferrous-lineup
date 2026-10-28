# Una tercera caza de bugs: un archivo lento, un patrón que salta y nombres cortados

Archivos: [`crates/lineup-core/src/file.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/file.rs),
[`crates/lineup-core/src/diff.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/diff.rs),
[`crates/lineup-app/src/app/edit.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/edit.rs),
[`crates/lineup-app/src/widgets.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/widgets.rs),
[`crates/lineup-app/src/piano_roll.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/piano_roll.rs)
y [`crates/lineup-app/src/app/ux_tests.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/ux_tests.rs)

Tras la [segunda caza de bugs](bug-hunt.md), la app ganó piezas
orquestales enteras (Boléro, The Rite of Spring, Petrushka), una traza
de la reproducción, pistas bloqueadas y un estilo por sección. Cada
trabajo nuevo se probaba por su cuenta; esta caza preguntó otra vez qué
se rompe cuando se usan **juntos**, de la manera en que usas la app.
Salieron cuatro bugs, un commit y una prueba cada uno. Uno de ellos
hacía que un Boléro guardado tardara un minuto y medio en abrirse; su
arreglo es un simple reordenamiento de dos líneas, pero encontrar *por
qué* es una lección de cómo un programa puede ser lento sin hacer nada
mal.

## Cómo se hizo la caza

La build de release se ejecutaba en una pantalla virtual (`Xvfb`),
manejada con `xdotool`, como en las cazas anteriores. Esta vez las
preguntas eran sobre el flujo de trabajo completo, no sobre una parte:

- abrir una demo de una canción, estudiarla en el chord sheet, probar
  en ella un estilo de la Library, guardarla y abrirla otra vez;
- ¿sigue cada ventana mostrando lo que mostraba antes, después de cada
  paso?
- ¿se lee todo lo que la app escribe en pantalla con el zoom habitual?

Cada bug se **veía** primero, en la app en marcha o en una prueba sobre
el código tal cual estaba, y cada arreglo llegaba con una prueba que
falla sin él.

## Abrir una canción larga tardaba un minuto y medio

Guarda Boléro como archivo `.lineup`, ábrelo otra vez y la ventana se
congelaba: 85 segundos antes de responder. «Shrovetide Fair in the
Evening» de Petrushka tardaba 29 segundos, «Augurs of Spring» de The
Rite of Spring, 4.5. Guardar las mismas canciones tardaba una centésima
de segundo, así que el trabajo no estaba en el tamaño del archivo como
tal.

### Medir antes de adivinar

Lo primero que hay que hacer con un programa lento es averiguar
**adónde** va el tiempo, no adivinarlo. Un profiler (en Linux, `perf`)
detiene el programa miles de veces por segundo y anota en qué función
está. Casi todas las muestras estaban en una función de la biblioteca
RON, la que lee los archivos `.lineup`, y dentro de ella, en
`str::find`.

Luego, una segunda medición: leer el mismo texto directo a los tipos
del proyecto tardaba **0.02 segundos**. Así que RON era rápida leyendo
el archivo y lenta en otra cosa que la app le pedía.

### Qué pedía la app

Un archivo de proyecto empieza con su número de formato:

```text
(format: 24, tempo: 110.0, channels: [ ... ], patterns: [ ... ], ...)
```

Antes de leer un archivo, la app leía ese número a solas, para poder
decir «este archivo lo hizo una versión más nueva» en lugar de un error
confuso. Lo hacía con un struct que tiene solo ese campo:

```rust
#[derive(Deserialize)]
struct Header {
    format: u32,
}

let header: Header = ron::from_str(text)?;
```

serde, la biblioteca que convierte texto en valores de Rust, deja que
un struct omita campos: el lector **se salta** todo campo que el struct
no nombra. Así es como un struct con un campo puede leer el principio
de cualquier archivo de proyecto, incluso uno futuro. Pero para
saltarse un valor, el lector tiene que leerlo de todos modos, para
saber dónde termina, y tiene que hacerlo sin conocer su tipo (`Header`
no dice qué es `patterns`). Para un número, RON pregunta entonces: ¿es
un número entero o uno decimal? Aquí está la pregunta, del código
fuente de RON:

```rust
let raw_float_len = self.next_chars_while_from_len(skip, is_float_char);
// Trim at ".." to avoid treating range operators as float chars
let valid_float_len = self.src()[skip..]
    .find("..")
    .map(|i| i.min(raw_float_len))
    .map_or(raw_float_len, |i| i.min(raw_float_len));
```

`raw_float_len` es cuántos caracteres después de este punto podrían
pertenecer a un número decimal: unos pocos. La línea siguiente busca
`..` para detenerse antes de él, pero busca en `self.src()[skip..]`,
**todo el texto desde aquí hasta el final del archivo**, no solo esos
pocos caracteres. Un archivo de proyecto no tiene ningún `..`, así que
cada búsqueda lee hasta el final y no encuentra nada.

### Por qué eso crece tan rápido

Una canción larga es una lista de notas, y una nota es sobre todo
números: paso, duración, altura, velocidad. El archivo de Boléro tiene
más de dos megabytes de texto y bastante más de cien mil números. Por
cada número, la búsqueda lee el resto del archivo: el primer número lee
dos megabytes, uno de la mitad lee uno, el último no lee casi nada. De
media, la mitad del archivo por cada número.

Si un archivo tiene *n* números, el trabajo es más o menos *n* × *n* /
2. Eso se llama tiempo **cuadrático**, y es por lo que los tiempos de
arriba son tan desiguales: una canción el doble de larga tarda
**cuatro** veces más en abrirse, y una diez veces más larga, cien veces
más. Los archivos pequeños lo escondían (Carioca, 2.2 segundos, parecía
un disco lento), y solo las piezas más largas lo hicieron evidente.

Fíjate en que ninguna línea está mal por sí sola. `find` es la
herramienta correcta, solo mira más texto del que necesita, y el coste
viene de llamarlo una vez por cada número. Cuando un programa es lento
en proporción al *cuadrado* de su entrada, busca un loop que, por
dentro, haga algo proporcional a toda la entrada.

### El arreglo: leer el archivo entero primero

Cuando RON lee hacia los tipos reales del proyecto, sabe que un `tempo`
es un `f32` y un `step` un `u32`, así que nunca tiene que adivinar, y
nunca ejecuta esa búsqueda. Así que `from_text` lee ahora el proyecto
entero primero, y mira el número de formato después:

```rust
pub fn from_text(text: &str) -> Result<Project, FileError> {
    let file: ProjectFile = match ron::from_str(text) {
        Ok(file) => file,
        Err(err) => {
            return Err(match format_number(text) {
                Some(format) if format > FORMAT_VERSION => FileError::NewerFormat(format),
                _ => err.into(),
            });
        }
    };
    if file.format > FORMAT_VERSION {
        return Err(FileError::NewerFormat(file.format));
    }
    Project::try_from(file)
}
```

Hay dos maneras en las que puede aparecer un archivo de una versión más
nueva:

- **no se lee**, porque tiene un campo o un nombre que esta versión no
  conoce. Entonces el error a solas sería confuso («unexpected field
  `sparkles`»), así que el número de formato se lee a mano, y si es más
  nuevo, ese es el error que muestra la app;
- **se lee**, pero su número de formato es más nuevo, así que puede
  significar otra cosa. El `if` después del `match` lo atrapa.

El lector hecho a mano mira solo el principio del texto:

```rust
fn format_number(text: &str) -> Option<u32> {
    let rest = text.trim_start().strip_prefix('(')?.trim_start();
    let rest = rest.strip_prefix("format")?.trim_start();
    let rest = rest.strip_prefix(':')?.trim_start();
    let digits = rest
        .find(|c: char| !c.is_ascii_digit())
        .unwrap_or(rest.len());
    rest[..digits].parse().ok()
}
```

`strip_prefix` devuelve el texto después de un prefijo, o `None` si el
texto no empieza por él, y el `?` después de cada uno devuelve ese
`None` desde toda la función de golpe: «si falta cualquiera de estos,
esto no es el principio de un archivo de proyecto». Cada `rest` es un
slice del mismo texto, así que no se copia nada. Aquí `find` *sí* está
acotado: se detiene en el primer carácter que no es un dígito, dos o
tres caracteres más adentro.

El Boléro guardado se abre ahora en menos de un segundo. Una prueba lo
vuelve a leer y falla si eso tarda diez segundos, y otra lee el número
de formato de unos cuantos principios de archivo, buenos y malos. Un
efecto secundario: la prueba que guarda y vuelve a abrir cada demo
pasaba casi tres minutos en esa cabecera, y todas las pruebas de
`lineup-core` tardan ahora 14 segundos en una build optimizada,
así que las comprobaciones de cada pull request también son más
rápidas.

## Probar un estilo te llevaba a otro patrón

Abre El Choclo, selecciona su patrón «Pickup» en la caja de patrones y
aplica un estilo de la Library en modo canción (Tango nuevo, por
ejemplo). La banda toca la canción, y la caja de patrones dice ahora
**«Ending»**. Siempre.

### Cómo se entera la app de los cambios

La app guarda dos copias de la canción: la que editas, en el hilo de la
ventana, y la que suena, en el hilo de audio. Después de cada edición,
la app compara el proyecto antes y después de la edición y envía al
hilo de audio solo las diferencias (`diff.rs`): «insertar un patrón en
el 3», «quitar el canal 5», «esta nota cambió».

Para comparar dos listas de patrones, las alinea con un `Alignment`:
los patrones que son iguales al principio de ambas listas, los iguales
al final, y en medio lo que se quitó y lo que se insertó. Añadir un
patrón en medio da «igual, igual, *nuevo*, igual, igual».

Cuando ve un patrón insertado, `sync_audio` (en `app/edit.rs`) lo
selecciona, porque un patrón que acabas de añadir, clonar o traer de
vuelta con Undo es el que quieres ver:

```rust
Change::InsertPattern(index) => {
    // ...
    if bands {
        // A part the band wrote: the pattern you had stays selected.
        if index <= self.pattern {
            self.pattern += 1;
        }
    } else {
        self.pattern = index;
        self.rewind_for_new_pattern();
    }
```

### Por qué todo patrón parecía nuevo

La banda no solo añade patrones; añade **canales** (piano, bajo,
batería...). Un patrón guarda una fila de notas por cada canal, así que
añadir tres canales da a *todo* patrón tres filas nuevas. El
`Alignment` preguntaba «¿son iguales estos dos patrones?», con `==`, y
ningún patrón era ya igual a sí mismo. Así que el diff decía: todo
patrón se quitó, y todo patrón se insertó otra vez. `sync_audio`
seleccionaba cada patrón de la canción según lo «insertaban», y el
último, «Ending», ganaba. Al hilo de audio también se le enviaba la
canción entera otra vez, que sonaba bien pero hacía mucho más trabajo
del necesario.

### El arreglo: decir qué significa «el mismo patrón»

Igual y *el mismo* son dos preguntas distintas. Un patrón cuyas notas
cambiaron sigue siendo el mismo patrón, como una persona a la que le
cambió el pelo sigue siendo la misma persona. `Alignment::new` recibe
ahora la pregunta como argumento:

```rust
fn new<T>(before: &[T], after: &[T], same: impl Fn(&T, &T) -> bool) -> Self {
    // ...
    let start = before
        .iter()
        .zip(after)
        .take_while(|(old, new)| same(old, new))
        .count();
```

`impl Fn(&T, &T) -> bool` significa «cualquier función o closure que
recibe dos `&T` y responde sí o no». Antes, el tipo tenía que ser
`T: PartialEq` para poder usar `==`; ahora `T` no necesita nada, y
decide el que llama. Los patrones se alinean por lo que los hace *ese*
patrón:

```rust
fn same_pattern(old: &Pattern, new: &Pattern) -> bool {
    old.name == new.name && old.color == new.color && old.player == new.player
}

let patterns = Alignment::new(before.patterns(), after.patterns(), same_pattern);
let channels = Alignment::new(before.channels(), after.channels(), |old, new| old == new);
```

Observa los dos tipos de argumento. `same_pattern` es el **nombre de
una función**: en Rust una función es un valor, y se puede pasar donde
se espera un `impl Fn`. `|old, new| old == new` es un **closure**, una
función escrita en el sitio; Rust deduce los tipos de `old` y `new` de
la firma de `Alignment::new`. Los canales siguen usando `==`: un canal
son sus ajustes de sonido, y un canal cuyos ajustes cambiaron se envía
de nuevo de todos modos.

Un patrón alineado como «el mismo» pero con otras notas no se pierde:
el resto del diff compara las notas de cada patrón que alineó y envía
las que cambiaron. Así que esta elección no puede hacer que la canción
suene mal; solo decide qué cuenta como *nuevo*.

Dos pruebas lo comprueban: una en `diff.rs`, donde un canal nuevo y un
patrón nuevo dan exactamente un «insertar patrón», y otra en las pruebas
de la app, que aplica Tango nuevo a El Choclo con «Pickup» seleccionado
y comprueba que «Pickup» sigue seleccionado después.

## Seguir el playhead escondía el compás 1

Con **Follow playhead** activado, el playlist se desplaza para mantener
el playhead a la vista. Cuando una canción llega al final y vuelve a
empezar, o un loop vuelve a su principio, el playhead salta a la
izquierda y la vista lo sigue. Pero se detenía con el compás 1 medio
escondido, bajo los nombres de las pistas, y el playhead con él.

Los nombres de las pistas a la izquierda son **fijos** (sticky): se
dibujan sobre los compases, en el borde izquierdo de la vista, y se
quedan ahí mientras los compases se desplazan por debajo. Para el área
de scroll no están ahí en absoluto. Cuando el playhead se movía,
`follow_x` le pedía revelar un rectángulo pequeño alrededor del
playhead:

```rust
let margin = (view.width() * 0.1).min(60.0);
ui.scroll_to_rect(
    Rect::from_center_size(pos2(x, view.center().y), vec2(margin * 2.0, 1.0)),
    None,
);
```

`scroll_to_rect` se desplaza lo menos que puede para que el rectángulo
quede dentro de la vista. Yendo a la izquierda, eso deja el borde
izquierdo del rectángulo en el borde izquierdo de la vista: el playhead
a 60 puntos de él. Los nombres miden 116 puntos de ancho, así que el
playhead quedaba debajo. La tecla Home ya tenía en cuenta los nombres;
el seguimiento no.

El arreglo le dice a `follow_x` cuánto mide de ancho la parte fija, y
hace que el rectángulo llegue ese tanto más a la izquierda:

```rust
pub fn follow_x(ui: &Ui, step: Option<usize>, x: f32, view: Rect, covered: f32) {
    // ...
    let y = view.center().y;
    ui.scroll_to_rect(
        Rect::from_min_max(
            pos2(x - margin - covered, y - 0.5),
            pos2(x + margin, y + 0.5),
        ),
        None,
    );
```

El playlist pasa `HEADER_WIDTH` (sus nombres), y el piano roll
`KEY_WIDTH` (su teclado), que tenía el mismo problema. Yendo a la
derecha, nada cambia: el borde derecho del rectángulo está donde
estaba. Una prueba ejecuta un área de scroll de 800 puntos de ancho con
116 puntos de nombres, avanza muy a la derecha, salta de vuelta y
comprueba que el playhead acaba a la derecha de los nombres.

## El piano roll imprimía trozos de nombres

Abre el patrón «A» de El Choclo en el piano roll. Debajo de las notas,
el carril de acordes nombra el acorde de cada tramo de pasos. Con el
zoom habitual mostraba «n», «7/», «/La7»; la fila de la pista, que
nombra notas sueltas, mostraba «E♭E» y «G♯B», dos nombres pegados.

Un nombre se dibujaba centrado en su bloque y **recortado** a él: lo
que caía fuera del bloque se cortaba. «E7/B» en un bloque demasiado
estrecho mostraba solo su mitad, «7/», que no es ningún nombre, y
«/La7» parece otro acorde. Para alguien que aprende armonía mirando la
pantalla, un nombre equivocado es peor que ninguno.

### Medir, y luego dibujar

egui dibuja texto en dos pasos. Primero lo **compone**: elige el glifo
de cada carácter en la fuente, y calcula dónde va cada uno y cuánto
espacio ocupa el conjunto. El resultado es una *galera* (el nombre
viene de la imprenta: una bandeja de tipos compuestos en líneas, listos
para imprimir). Luego pinta la galera. `painter.text(...)` hace ambas
cosas a la vez; para decidir *antes* de dibujar, los dos pasos se
separan:

```rust
let name = painter.layout_no_wrap(label, FontId::proportional(11.0), LANE_TEXT);
if name.size().x <= chord_rect.width() - 2.0 {
    // A long chord's name stays in view while it scrolls.
    let shown = chord_rect.intersect(cells);
    painter.with_clip_rect(chord_rect).galley(
        shown.center() - name.size() / 2.0,
        name,
        LANE_TEXT,
    );
}
```

`layout_no_wrap` compone el texto en una sola línea (sin partirlo en
una segunda línea), y `name.size()` es cuánto mide de ancho y de alto.
Si cabe en el bloque, con un punto de sobra a cada lado, se pinta la
misma galera; nada se compone dos veces. `galley` recibe la esquina
superior izquierda del texto, así que el centro se mueve hacia atrás la
mitad del tamaño. Si no cabe, no se dibuja nada; señalar el bloque
sigue nombrando el acorde completo en la línea bajo la barra de
herramientas («Step 12: E7/B»).

La prueba abre «A» en el piano roll, encuentra cada trozo de texto
pintado en el color del carril y comprueba que ninguno es más ancho que
su bloque. Antes del arreglo fallaba en «Re♯»: 14.9 puntos de ancho, en
un bloque de 6.2.

## Qué se lanzó después de esta caza

La misma caza produjo una lista de ideas para el flujo de trabajo y la
pantalla: insertar y borrar compases, Open recent, una copia de
recuperación, una barra de herramientas de una fila, poder elegir los
números en tonalidades menores, y más. Las principales se han lanzado
desde entonces. Los compases se pueden [insertar y
eliminar](insert-delete-bars.md) moviéndose toda la canción con ellos;
**File > Open recent** guarda los últimos diez proyectos; un cierre
inesperado deja una [copia de recuperación](recovery.md) que se ofrece
en el siguiente arranque; [el capítulo del flujo de
trabajo](workflow.md) cubre la barra de herramientas de una fila y la
lista de atajos; y los [números en tonalidades
menores](minor-numerals.md) se escriben como los escribe tu libro.

La búsqueda de RON también se podría arreglar en su sitio, buscando
`..` solo entre los caracteres que podrían formar parte del número. Eso
es un cambio en la biblioteca de otra persona, así que la app no
depende de ello: simplemente ya no le pide a RON que se salte campos.

## Ejercicios

1. Guardada, «The Shrovetide Fair» son 780 KB de texto con unos 43,000
   números. Si la búsqueda de cada número lee, de media, la mitad del
   archivo, ¿cuántos bytes se leen en total? La unos 2 GB por segundo,
   ¿cuánto se tarda? Compáralo con los 10 segundos medidos.
2. Arregla la lectura anticipada de RON: reescribe la línea del
   `find("..")` para que busque solo los primeros `raw_float_len`
   caracteres. (Pista: corta `self.src()[skip..skip + raw_float_len]`,
   y piensa qué pasa si el slice termina en mitad de un carácter.)
3. `format_number` devuelve `None` para `(tempo: 120.0, format: 25)`.
   ¿Por qué no pasa nada por que solo lea el principio del archivo?
   (Mira el orden de los campos en `ProjectFile`, y `to_text`.)
4. Escribe `same_channel(old: &Channel, new: &Channel) -> bool` que
   alinee los canales solo por su nombre, y pásalo a `Alignment::new`.
   Luego encuentra un caso en el que dé la respuesta equivocada: ¿qué
   pasa con dos canales llamados «Piano»?
5. El piano roll omite el nombre de un acorde que no cabe. Haz que
   pruebe primero un nombre más corto: para un acorde con barra, el
   acorde sin su bajo («E7» para «E7/B»), y luego la fundamental sola
   («E»). Compón cada uno con `layout_no_wrap` y dibuja el primero que
   quepa.
