# Encontrar una demo: orden alfabético y búsqueda

Archivos: [`crates/lineup-app/src/app/demo_menu.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/demo_menu.rs)
y [`crates/lineup-app/src/app/library.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/library.rs)

Ahora hay más de doscientas demos, y **File > Demos** las listaba en el
orden en que se escribieron en la biblioteca: London Bridge antes que
Deck the Halls, Tango antes que Blues. Para encontrar una abrías
submenús hasta que aparecía. Ahora cada lista del menú está en orden
alfabético, y una caja de búsqueda arriba encuentra cualquier demo a
partir de unas pocas letras de su título, su compositor, su género, su
tonalidad o su compás.

![File > Demos > Songs > Classical and film: los géneros de Blues a Tango en orden alfabético, y las canciones clásicas de «Air on the G String» a «Wedding March», con «Für Elise» entre «Funeral March» y «Hallelujah Chorus».](../images/step-9-demos-sorted.png)

## Probarlo

Abre **File > Demos**. Los géneros bajo **Songs** y **Styles** van de
Blues a Tango, y las canciones y los estilos de cada género están en
orden alfabético, igual que las piezas **Twelve-tone** y **Micromodes**.

Sin hacer clic en nada, escribe **tango**. La caja encima de los
submenús tiene el teclado en cuanto el menú se abre, así que las letras
entran directamente, y los submenús dan paso a una sola lista: primero
cada demo con «tango» en su título, luego las canciones tocadas como
tango o milonga, luego los estilos de tango, y al final, más apagadas,
las demos cuyas notas solo mencionan el tango, como Candombe y *St.
Louis Blues*, que abre con una introducción de tango. La la derecha, cada
línea dice dónde vive la demo en el menú, como **Songs ▸ Tango**.

![El menú Demos con «tango» escrito en la caja de búsqueda. Tango, Tango de salón, Tango del telar, Tango nuevo, Tango síncopa, Tango vals y Tangos flamencos van primero; luego El Choclo, Gallo ciego, Habanera, La Cumparsita, La Paloma, Milonga, Milonga del río, Vals del puerto y Vals in minor; al final, más apagadas, Candombe y St. Louis Blues. La la derecha de cada una, «Songs ▸ Tango» o «Styles ▸ Tango».](../images/step-9-demo-search.png)

Pulsa **Enter** para abrir la primera, o la **flecha abajo** para
recorrer la lista y **Enter** en la que quieras. Un clic también
funciona. Como antes, la demo se abre y suena desde el principio, con su
estilo y su ritmo seleccionados en la Library.

Algunas búsquedas para probar:

| Escribe | Encuentra |
|---|---|
| `choclo` | El Choclo: vale cualquier palabra del título |
| `fur elise` | Für Elise: no hace falta escribir los acentos ni las mayúsculas |
| `bach` | las piezas de Bach, por compositor, después de Bachata, cuyo título empieza con «bach»; no el Can-can de Offenbach |
| `d minor` | primero las canciones en Re menor, luego, más apagadas, las demás que tienen una palabra que empieza con «d» y un «minor» |
| `3/4` | cada demo en compás de tres por cuatro |
| `menor 3` | las piezas de micromodos construidas sobre Menor 3 |
| `waltz` | canciones tocadas como vals, y los estilos de vals |

Cada palabra que escribes tiene que encontrarse al principio de una
palabra, en cualquier orden: `minor prelude` pone primero los dos
preludios menores, y `bach` encuentra a Johann Sebastian Bach pero no a
Jacques Offenbach. Cuando las palabras se encuentran separadas en lugar
de juntas, o solo en las notas sobre una demo, esta aparece al final,
más apagada, para que sobresalgan las coincidencias que probablemente
querías:

![El menú Demos con «d minor» escrito. Primero, a todo color, Toccata in D minor y las otras ocho canciones en Re menor: Alma llanera, Dies Irae, Dies irae (Mozart), Gallo ciego, Habanera, House of the Rising Sun, Marinera del callejón y When Johnny Comes Marching Home. Debajo, más apagadas, Anitra's Dance, Asturias, Bambuco del páramo y más, que solo tienen una palabra que empieza con «d» y un «minor» en alguna parte.](../images/step-9-demo-search-key.png)

Pulsa **Esc**, o pasa a otro menú, para cerrarlo; la caja vuelve a estar
vacía la próxima vez.

## ¿Cuán alfabético es alfabético?

Las computadoras comparan el texto letra por letra, por el número que
cada letra tiene en Unicode. Eso da algunos órdenes raros: cada
mayúscula va antes que cada minúscula («Zamba» antes que «alma
llanera»), y una letra acentuada va después de todas las normales
(«Ábalos» después de «Zamba»). La puntuación también estorba: que «Oh!
Susanna» vaya antes o después de «Oh My Darling» dependería de si «!»
tiene un número menor que un espacio, algo que nadie debería tener que
saber.

Por eso el menú ordena por una copia **plegada** de cada título: solo
minúsculas, acentos quitados, puntuación descartada, un guion convertido
en espacio. «Für Elise» se ordena como «fur elise», justo después de
«Funeral March»; «St. James Infirmary» como «st james infirmary»;
«Tico-Tico» como «tico tico». El menú sigue mostrando el título real.

Los catálogos de bibliotecas van un paso más allá y se saltan un «The»,
«El» o «La» inicial, archivando *El Choclo* bajo la Do. El menú no:
ordena por el título exactamente como lo lees, así que las primeras
letras van de la La a la Z y *El Choclo* se sienta bajo la Mi. La
búsqueda lo compensa: `choclo` lo encuentra enseguida.

## El código

### Ordenar en el menú, no en la biblioteca

Las demos viven en el array `SONGS` en
`crates/lineup-core/src/library/song.rs`, agrupadas por género en el
orden en que se añadieron. Ordenar ese array a mano habría sido posible,
pero cada demo nueva tendría que insertarse en el lugar correcto, y
varios cambios que añaden demos a la vez editarían las mismas líneas.
En su lugar, la aplicación hace una copia ordenada cuando el menú se
muestra por primera vez, y el array conserva su orden.

La copia es una lista de valores `Entry`, uno por demo:

```rust
struct Entry {
    demo: Demo,
    /// What its button says, such as "El Choclo · Tango".
    label: String,
    /// Where the menu keeps it, such as "Songs ▸ Tango".
    place: String,
    hover: String,
    /// The title, folded (see [`fold`]): what the menu sorts by and a
    /// search looks at first.
    title: String,
    /// Who wrote it, its feel, genre, key and time signature, folded.
    details: String,
    /// The notes about it, folded.
    about: String,
}
```

Se construye una vez y se conserva mientras la aplicación se ejecuta, en
un **`LazyLock`**: un `static` cuyo valor se calcula la primera vez que
alguien lo lee, no cuando el programa arranca. Un `static` normal debe
conocerse en tiempo de compilación, y ordenar necesita código que solo
corre en tiempo de ejecución.

```rust
static DEMOS: LazyLock<Demos> = LazyLock::new(|| {
    let mut genres = Genre::ALL.to_vec();
    genres.sort_by(|a, b| by_title(a.name(), b.name()));
    let mut entries = all_entries();
    entries.sort_by(|a, b| {
        a.title
            .cmp(&b.title)
            .then_with(|| a.label.cmp(&b.label))
            .then_with(|| a.place.cmp(&b.place))
    });
    Demos { entries, genres }
});
```

`sort_by` toma una función que dice si `a` va antes, después o al nivel
de `b`, como un `Ordering`. `then_with` rompe un empate: dos demos con
el mismo título plegado («Dies Irae» y «Dies irae (Mozart)» no lo
tienen, pero una canción y un estilo pueden compartir nombre) se ordenan
por su label, y luego por su place, para que el orden nunca dependa de
la suerte.

El submenú de cada género recorre entonces la única lista ordenada y se
queda con las canciones de ese género, así que cada submenú queda
ordenado sin volver a ordenarlo:

```rust
let songs: Vec<&Entry> = demos
    .entries
    .iter()
    .filter(|e| matches!(e.demo, Demo::Song(song) if song.genre == genre))
    .collect();
```

`matches!` es una macro pequeña que convierte un patrón de `match` en un
`bool`: «¿es esto un `Demo::Song`, y si lo es, es su género este?»

### Plegar texto

```rust
fn fold(text: &str) -> String {
    let mut folded = String::with_capacity(text.len());
    for c in text.chars().flat_map(char::to_lowercase) {
        match c {
            'à' | 'á' | 'â' | 'ã' | 'ä' | 'å' | 'ā' | 'ă' | 'ą' => folded.push('a'),
            // ... one line per letter ...
            c if c.is_alphanumeric() || matches!(c, '#' | '/' | '+') => folded.push(c),
            c if (c.is_whitespace() || matches!(c, '-' | '–' | '—' | '(' | ')' | '·' | '▸' | ':' | ';'))
                && !folded.is_empty()
                && !folded.ends_with(' ') =>
            {
                folded.push(' ');
            }
            _ => {}
        }
    }
    // ...
}
```

`char::to_lowercase` devuelve un iterador, no un único `char`, porque
algunas mayúsculas se convierten en más de una minúscula. `flat_map` une
esos pequeños iteradores en un solo flujo de caracteres.

`#`, `/` y `+` se conservan para poder buscar tonalidades («Fa♯»),
compases («3/4») y pulsos agrupados («2+2+3/8»). Un espacio solo se
añade cuando el último carácter no era ya uno, para que «Row, Row, Row»
se convierta en «row row row» con espacios simples, lo que importa para
la parte siguiente: una búsqueda busca el principio de una palabra, y
una palabra empieza después de un espacio.

Un trabajo completo usaría las reglas de Unicode para descomponer los
acentos (el crate `unicode-normalization` lo hace). Una tabla de las
letras latinas acentuadas es suficiente para los títulos de la
biblioteca, y no añade ninguna dependencia.

### Buscar

Una búsqueda pliega lo que escribiste de la misma manera y lo divide en
palabras. Cada palabra tiene que empezar una palabra del texto de la
demo:

```rust
fn starts_a_word(text: &str, words: &str) -> bool {
    text.match_indices(words)
        .any(|(at, _)| at == 0 || text[..at].ends_with(' '))
}
```

`match_indices` encuentra cada lugar donde `words` aparece en `text`,
con el byte donde empieza. La coincidencia cuenta si está al principio
mismo, o justo después de un espacio: «bach» se encuentra en «johann
sebastian bach» pero no en «jacques offenbach». (`text[..at]` es un
slice del texto hasta ese byte; cortar un `&str` en medio de una letra
haría panic, pero `match_indices` solo da lugares donde empieza una
coincidencia, que siempre están entre letras.)

Luego cada demo recibe un **rango**, o ninguno si una de las palabras no
aparece en ningún sitio:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Rank {
    TitleStart,
    TitleWords,
    Details,
    Words,
    About,
}
```

`TitleStart`: el título empieza con lo que escribiste. `TitleWords`:
cada palabra que escribiste empieza una palabra del título. `Details`:
lo que escribiste, tal como lo escribiste, está en quién lo escribió, el
feel, el género, la tonalidad o el compás, como «d minor» en una canción
cuya tonalidad es «Re minor». `Words`: todas las palabras se encontraron
en esos, pero separadas. `About`: alguna palabra solo se encontró en las
notas sobre la demo.

La tonalidad necesitó un poco de cuidado musical. Muchas canciones
menores están escritas en la escala **menor armónica**, cuya séptima
nota se eleva para dar un tirón fuerte de vuelta a la tónica. Una
canción en Re menor armónica sigue estando en la tonalidad de Re menor,
así que sus detalles buscables dicen ambas cosas, «Re harmonic minor» y
«Re minor», y `d minor` encuentra el Dies irae del Réquiem de Mozart
además de la Toccata in D minor.

Derivar `Ord` en un enum ordena sus variantes tal como están escritas,
así que `Rank::TitleStart < Rank::About`, y ordenar las coincidencias
por rango pone las mejores primero:

```rust
// A stable sort, so equal ranks stay in alphabetical order.
found.sort_by_key(|&(_, rank)| rank);
```

Las demos ya estaban alfabéticas, y el `sort_by_key` de Rust es
**estable**: las cosas que comparan igual conservan el orden que tenían.
Así, dentro de cada rango las coincidencias se quedan en orden
alfabético sin decirlo. (`sort_unstable_by_key` es un poco más rápido
pero no promete eso.)

El rango decide el color también: desde `Rank::Words`, el label se
dibuja con `RichText::weak()`, más apagado que el resto. La comparación
`rank >= Rank::Words` funciona por la misma razón que la ordenación.

### Una caja de texto en un menú

Tres detalles de egui, la biblioteca de GUI, necesitaron cuidado:

- **Un clic dentro no debe cerrar el menú.** Los menús de egui se
  cierran con cualquier clic, dentro o fuera, para que hacer clic en un
  elemento los cierre. El submenú Demos se hace con `SubMenuButton` y un
  `MenuConfig` cuyo `close_behavior` es
  `PopupCloseBehavior::CloseOnClickOutside`. Hacer clic en una demo
  sigue cerrando todo el menú File, porque el código llama a `ui.close()`
  cuando se elige una.
- **La caja toma el teclado cuando el menú se abre.** egui llama al
  código del menú cada frame en que se muestra, y le da a cada pasada un
  número (`ctx.cumulative_pass_nr()`). El menú guarda el número de la
  última pasada en la que se mostró; si esa no fue la pasada justo
  anterior a esta, el menú acaba de abrirse, así que la caja se vacía y
  se llama a `request_focus()` sobre ella.
- **La flecha abajo sale de la caja.** Los menús de egui ya se mueven
  entre sus elementos con las flechas, pero una caja de texto se queda
  las flechas para sí. Así que antes de que la caja se dibuje, mientras
  tiene el foco, el menú quita la flecha abajo de la entrada con
  `consume_key`, y le da el foco a la primera coincidencia. La partir de
  ahí, las flechas propias del menú toman el control.

Lo que escribiste se conserva entre frames en la memoria propia de egui
(`ctx.data_mut(|data| data.insert_temp(id, query))`), con un `Id` como
clave, en lugar de un campo nuevo en `LineupApp`: solo importa mientras
el menú está abierto.

## Pruebas

`demo_menu.rs` comprueba que el plegado quita acentos y puntuación, que
las canciones de cada género y los géneros mismos salen en orden
alfabético, que una búsqueda encuentra títulos antes que compositores y
necesita cada palabra, que encuentra canciones por compositor (Bach pero
no Offenbach), género, tonalidad (cada canción en Re menor, por delante
de Hungarian Dance No. 5, que solo tiene un «Dance» y está en Sol menor) y
compás, y que las coincidencias más débiles van al final. Ejecútalas
con:

```sh
cargo test -p lineup-app demo_menu
```

## Pruébalo tú mismo

1. Busca a tu compositor favorito. ¿Qué demos salen más apagadas, y por
   qué?
2. En `fold`, quita `'/'` de los caracteres que se conservan. ¿Qué
   prueba falla, y qué encuentra ahora una búsqueda de `3/4`?
3. Los catálogos de bibliotecas se saltan un artículo inicial al
   ordenar. Escribe una función `sort_key(title: &str) -> String` que
   pliegue el título y luego suelte una primera palabra «the», «a»,
   «an», «el», «la», «los» o «las». Dale a `Entry` un campo `sort_key`
   rellenado con ella, y ordena `DEMOS.entries` por eso en lugar de
   `title` (conserva `title` para la búsqueda, para que `el ch` siga
   encontrando *El Choclo*). ¿Adónde va *El Choclo* en el submenú Tango
   ahora? ¿Te gusta más?
