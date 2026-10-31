# Micromodos

Archivos: [`crates/lineup-core/src/harmony/micromode.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/harmony/micromode.rs),
[`crates/lineup-core/src/micromodal.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/micromodal.rs),
[`crates/lineup-core/src/library/micromodes.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/micromodes.rs)
y [`crates/lineup-app/src/micromodes.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/micromodes.rs)

Los [conjuntos de clases de altura](sets.md) le dan un
nombre a cada grupo de notas. Este capítulo mira de cerca los grupos
más pequeños que todavía tienen un carácter propio, los grupos de
**tres** notas, tal como el compositor argentino **Francisco Kröpfl**
enseñaba a oírlos a sus alumnos. Los llamó **micromodos**: doce
pequeños «modos» de tres notas, cada uno con un color que se aprende a
reconocer.

En la aplicación, **Micromodos…**, en el piano roll, abre una ventana
con los doce micromodos de Kröpfl en sus tres familias. Cada uno se
dibuja en un reloj, con su forma prima en solfeo (*do - do# - mi*), sus
intervalos, su conjunto de clases de altura y su hora en el Tone Clock
de Peter Schat; puedes escucharlo, dibujar con él o usarlo como
tonalidad. Selecciona algunas notas y la ventana lista los micromodos
que contienen. Las ventanas Conjuntos de clases de alturas y Micromodos
se señalan una a la otra. La Biblioteca tiene un grupo **Micromodos
(Kröpfl)** con siete estilos, y **Archivo > Demostraciones >
Micromodos** toca cada uno como una pieza corta. La hoja de acordes
también puede mostrar cualquier armonía como micromodos (ver [Una
armonía, cuatro nombres](sets.md#una-armonía-cuatro-nombres)).

## La música, en pocas palabras

Francisco Kröpfl (1931–2021) nació en Timișoara, Rumania, y creció en
la Argentina. Fue un pionero de la música electrónica en América
Latina: en 1958 fundó el Estudio de Fonología Musical de la Universidad
de Buenos Aires, el primer estudio de música electrónica de una
universidad en el continente. También enseñó composición durante
décadas, y su obra para piano *Música '66* se escribió para poner a
prueba el método que enseñaba.

Ese método parte de una observación sencilla. Elige tres notas
distintas cualesquiera. Como sea que las escribas, en el orden y en las
octavas que sea que las toques, las tres notas tienen tres intervalos
entre sí, y esos intervalos le dan al grupo un color: Do, Re♭ y Mi
tienen el mismo sonido tenso, agridulce, ya los toques como acorde,
como melodía o repartidos en cuatro octavas. Sube el grupo (Re, Mi♭,
Fa♯) y conserva su color. Dalo vuelta (Do, Si, La♭, los mismos
intervalos hacia abajo) y el color sigue siendo la misma familia de
sonido.

Así que Kröpfl contaba tres notas como **el mismo micromodo** cuando
son:

- las mismas notas en otro orden u otra octava, o escritas de otra
  manera (Do♯ y Re♭ son aquí una sola nota),
- movidas hacia arriba o hacia abajo (transportadas), o
- dadas vuelta (invertidas).

Son exactamente las reglas de los conjuntos de clases de altura. De las
220 maneras de elegir tres notas entre doce, las reglas dejan solo
**doce** micromodos distintos. Repite uno de ellos a lo largo de un
pasaje y el pasaje toma su color; Pablo Cetta, compositor y teórico que
estudió la música de Kröpfl, lo describe como «un 'color' o timbre
global particular y reconocible».

### La tabla

Cada micromodo se escribe en su **forma prima**: su forma más compacta,
empezando en Do, la misma forma prima que en la ventana Conjuntos de
clases de alturas. Esta es la tabla de Pablo Cetta, con los números que
Allen Forte les dio a los mismos doce grupos en 1973:

| | Micromodo | Forma prima | Forte | Intervalos | Formas |
|---|-----------|-------------|-------|------------|--------|
| 1 | **Menor 1** | do - do# - re | 3-1 [012] | dos segundas menores, una segunda mayor | 12 |
| 2 | **Menor 2** | do - do# - re# | 3-2 [013] | segunda menor, segunda mayor, tercera menor | 24 |
| 3 | **Menor 3** | do - do# - mi | 3-3 [014] | segunda menor, tercera menor, tercera mayor | 24 |
| 4 | **Menor 4** | do - do# - fa | 3-4 [015] | segunda menor, tercera mayor, cuarta | 24 |
| 5 | **Menor 5** | do - do# - fa# | 3-5 [016] | segunda menor, cuarta, tritono | 24 |
| 6 | **Mayor 1** | do - re - mi | 3-6 [024] | dos segundas mayores, una tercera mayor | 12 |
| 7 | **Mayor 2** | do - re - fa | 3-7 [025] | segunda mayor, tercera menor, cuarta | 24 |
| 8 | **Mayor 3** | do - re - fa# | 3-8 [026] | segunda mayor, tercera mayor, tritono | 24 |
| 9 | **Mayor 4** | do - re - sol | 3-9 [027] | segunda mayor, dos cuartas | 12 |
| 10 | **Acorde disminuido** | do - mib - solb | 3-10 [036] | dos terceras menores, un tritono | 12 |
| 11 | **Acorde menor-mayor** | do - mib - sol | 3-11 [037] | tercera menor, tercera mayor, cuarta | 24 |
| 12 | **Acorde aumentado** | do - mi - sol# | 3-12 [048] | tres terceras mayores | 4 |

«Cuarta» quiere decir aquí una cuarta justa *o* una quinta justa: en un
conjunto, un intervalo y su inversión cuentan como uno (de Do a Sol
hacia arriba hay una quinta, de Sol a Do una cuarta; el par Do y Sol es
el mismo de las dos maneras). «Formas» cuenta cuántos grupos distintos
de tres notas son ese micromodo: la forma prima y su inversión sobre
cada una de las doce notas. La mayoría tiene 24; los que son su propia
inversión tienen 12; el acorde aumentado solo 4, porque subirlo una
tercera mayor da otra vez las mismas tres notas. Las formas suman 220,
todos los grupos de tres que hay.

### Tres familias, y sus nombres

Kröpfl ordenó los doce en tres familias. Pablo Jaureguiberry, que
analizó la música de Diana Rud, alumna de Kröpfl, da las reglas:

- «Los micromodos **menores** son todos aquellos que están compuestos
  por al menos un intervalo de segunda menor». Son cinco, de Menor 1 a
  Menor 5.
- «Los **mayores** tienen todos al menos una segunda mayor y a su vez
  carecen de semitonos». Son cuatro, de Mayor 1 a Mayor 4.
- «Los restantes contienen al menos dos terceras cada uno». Son las
  tres tríadas, y llevan su nombre: el acorde **disminuido**, el
  **menor-mayor** y el **aumentado**.

Los nombres salen de la forma prima, como explica Cetta: los nueve
primeros empiezan con una segunda, menor (*do - do#*) o mayor
(*do - re*), y el número sube a medida que sube la tercera nota. Así,
Menor 3 es «el tercer grupo que empieza con una segunda menor»,
*do - do# - mi*.

Un micromodo merece una segunda mirada: el **Acorde menor-mayor**. Su
forma prima, Do Mi♭ Sol, es una tríada menor. Dada vuelta desde Do es
Do, La y Fa: una tríada de Fa mayor. Las tríadas menores y mayores son
imágenes en espejo una de la otra, así que son un solo micromodo, y su
nombre dice las dos cosas. Todos los acordes de una progresión I–IV–V–I
son el mismo micromodo; lo que la música tonal hace con ellos es
cuestión de *qué* formas, no de qué color.

### Encontrar a mano el micromodo de tres notas

Cetta da dos ejemplos de Menor 3: Mi, Fa y La♭, y Fa, Re y Sol♭.

1. **Mi Fa La♭.** Ordena las notas dentro de una octava, tan juntas
   como se pueda: Mi Fa La♭ abarca cuatro semitonos, el menor ámbito.
   Mueve el grupo para que empiece en Do: Do Do♯ Mi. Eso es
   *do - do# - mi*, la forma prima de Menor 3.
2. **Fa Re Sol♭.** Ordenadas y lo más juntas posible: Re Fa Sol♭, un
   ámbito de cuatro semitonos. Desde Do: Do Mi♭ Mi, que es
   *do - mib - mi*, y no está en la tabla. Dalo vuelta: los intervalos 3
   y luego 1 pasan a ser 1 y luego 3, Do Do♯ Mi. Otra vez Menor 3.

Ese es todo el algoritmo: la **forma normal** (el orden más compacto),
movida a Do, comparada con su inversión, y gana la más compacta de las
dos. La ventana Conjuntos de clases de alturas ya lo hace, así que
Ferrous Lineup encuentra el micromodo de tres notas pidiendo su forma
prima y buscándola en la tabla.

### Los mismos doce, tres veces

Kröpfl no fue el único que encontró estos doce. En *The Structure of
Atonal Music* (1973), el teórico estadounidense **Allen Forte** hizo la
lista de todas las clases de conjuntos, y sus doce tricordios, de 3-1 a
3-12, son los micromodos en el mismo orden. El compositor neerlandés
**Peter Schat** los volvió a encontrar y los ordenó como las doce horas
de su **Tone Clock** («reloj tonal», *The Tone Clock*, 1993): la hora I
es Menor 1, la hora III es Menor 3, la hora XII el acorde aumentado.

Pero los tres no querían decir lo mismo con ellos. Marcela Pavia, que
compara a Forte con Kröpfl, describe la clasificación de Forte como
matemática, y la de Kröpfl como construida sobre cómo *percibimos* los
grupos de notas, a partir de la psicología de la Gestalt; y encuentra
que las dos dan resultados muy distintos cuando se usan para analizar
música. Forte cuenta cada subconjunto de cada acorde; Kröpfl escucha los
grupos que la música hace oír como una unidad. Ferrous Lineup muestra
las dos cosas: el número de Forte junto a cada micromodo, y los
micromodos junto a cada conjunto de Forte.

## Los conjuntos más grandes están hechos de micromodos

Un conjunto de *n* notas tiene *n*(*n*−1)(*n*−2)/6 grupos de tres: 4
para un acorde de cuatro notas, 10 para una escala pentatónica, 35 para
una escala de siete notas. Contar qué micromodos son da el **contenido
de micromodos** del conjunto, una huella de sus colores. Algunos
ejemplos, todos comprobados por las pruebas o calculados con las mismas
funciones:

- **La séptima de dominante**, Do Mi Sol Si♭: Do Mi Sol es una tríada
  mayor (Acorde menor-mayor), Mi Sol Si♭ una tríada disminuida, Do Mi
  Si♭ es Mayor 3 (el acorde sin su quinta) y Do Sol Si♭ es Mayor 2. Uno
  de cada uno.
- **La escala pentatónica**, Do Re Mi Sol La (5-35): Mayor 2 cuatro
  veces, Mayor 4 tres veces, Acorde menor-mayor dos veces (Do Mi Sol y
  La Do Mi) y Mayor 1 una vez (Do Re Mi). Ningún semitono, así que
  ningún micromodo menor, y por eso la escala pentatónica suena tan
  suave.
- **La escala diatónica**, las siete notas de Do mayor (7-35): Mayor 2
  ×8, Acorde menor-mayor ×6 (las tres tríadas mayores y las tres
  menores), Mayor 4 ×5, Menor 2 ×4, Menor 4 ×4, Mayor 1 ×3, Menor 5 ×2,
  Mayor 3 ×2 y una tríada disminuida (Si Re Fa). Ningún Menor 1, Menor 3
  ni acorde aumentado: no se pueden tocar solo con las teclas blancas.
- **La escala de tonos enteros** (6-35): Mayor 3 ×12, Mayor 1 ×6 y el
  acorde aumentado dos veces. Nada más, porque todos sus intervalos son
  pares.
- **Do Do♯ Re♯ Mi** (4-3), del análisis de Jaureguiberry de *Tres
  amores* (1986) de Diana Rud: dos Menor 2 y dos Menor 3. Rud arma sus
  acordes de cuatro notas de modo que cada compás conserva tres notas
  del anterior y añade una nueva, y los micromodos que contienen
  cambian de a poco.
- **Do Do♯ Fa♯ Sol** (4-9), del análisis de Cetta de *Música '66*:
  cuatro grupos de tres, todos Menor 5. Kröpfl los pone junto a Menor 3
  en la obra.

Así se enlazan las dos ventanas. Un micromodo *es* un conjunto de
clases de altura, uno de los doce de tres notas, así que la ventana
Conjuntos de clases de alturas marca esos doce con el nombre de su
micromodo; y un conjunto más grande *está hecho* de micromodos, así que
esa ventana los lista bajo sus detalles («Micromodos que contiene, en
sus 35 grupos de tres»). En la ventana Micromodos, las notas que
seleccionas se cuentan de la misma manera.

## Una serie hecha de un solo micromodo: el Concierto de Webern

Compositores anteriores a Kröpfl ya habían oído lo mismo. La *Pieza
para piano* op. 11 n.º 1 (1909) de Arnold Schoenberg empieza con Si,
Sol♯ y Sol: Menor 3. Anton Webern fue más lejos. La serie de su
*Concierto para nueve instrumentos* op. 24 (1934) está hecha de cuatro
grupos de tres (ver [Series dodecafónicas y sus
reglas](twelve-tone.md) sobre las series):

```text
Si Si♭ Re | Mi♭ Sol Fa♯ | La♭ Mi Fa | Do Do♯ La
```

Cada grupo es Menor 3, y cada uno es una forma del primero:

| grupo | notas | intervalos | forma del primer grupo |
|-------|-------|------------|------------------------|
| 1 | Si Si♭ Re | baja 1, sube 4 | él mismo |
| 2 | Mi♭ Sol Fa♯ | sube 4, baja 1 | al revés y dado vuelta (RI) |
| 3 | La♭ Mi Fa | baja 4, sube 1 | al revés (R) |
| 4 | Do Do♯ La | sube 1, baja 4 | dado vuelta (I) |

Al comienzo del Concierto, los cuatro grupos los tocan el oboe, la
flauta, la trompeta y el clarinete, uno tras otro, así que se oye el
mismo color cuatro veces en cuatro instrumentos. Schat armaba armonías
de la misma manera, con formas de una sola hora que juntas usan las
doce notas; en sus términos, la serie de Webern está hecha de la hora
III.

Webern no pensaba en micromodos: la palabra llegó décadas después. El
Concierto es una obra dodecafónica, y Webern construyó su serie con
cuatro formas de un solo grupo; «Menor 3» y «3-3» son nombres
posteriores para ese grupo, como lo describiría un teórico. Cetta señala
que Kröpfl, como los compositores del círculo de Schoenberg, prefería
los grupos de tres notas, y por eso los dos se encuentran aquí.

La serie es una de las series famosas de la ventana Serie dodecafónica,
**Concierto de Webern**, y el estilo **Los tricordios de Webern** la
toca: la serie como melodía que salta por tres octavas, con cada grupo
de tres como acorde debajo. Está en el grupo **Series dodecafónicas**
de la Biblioteca y al final de **Archivo > Demostraciones >
Micromodos**. El ritmo es de Ferrous Lineup, no de Webern.

Solo se copia en Ferrous Lineup música de dominio público. Webern murió
en 1945, así que su música es de dominio público donde los derechos de
autor duran 70 años después de la muerte del autor, y solo se usan las
doce notas de la serie, nunca su partitura. Kröpfl murió en 2021 y sus
alumnos viven, así que su música todavía tiene derechos de autor, y
nada de ella está en la aplicación: los siete estilos de micromodos se
escribieron para Ferrous Lineup.

## Música con un solo micromodo

¿Cómo se escribe música con un solo micromodo? Alejandro Martínez
describe el micromodo como «elemento mínimo de construcción
melódico/armónica». Los acordes son formas del micromodo, y también los
grupos de la melodía, así que todo el pasaje tiene un solo color. La
pregunta es qué forma viene después. Ferrous Lineup ofrece dos
respuestas, en `micromodal.rs`.

### Mover todas las notas la misma distancia

Lo más simple es una **secuencia**: subir las tres notas el mismo
intervalo cada vez. *Bruma de tonos enteros* sube Mayor 1 un tono en
cada compás: Do Re Mi, Re Mi Fa♯, Mi Fa♯ Sol♯… Nunca sale de una escala
de tonos enteros, el sonido de *Voiles* de Debussy. *Deriva aumentada*
baja el acorde aumentado un semitono por vez; después de cuatro
acordes, el quinto tiene las notas del primero, porque solo hay cuatro
acordes aumentados.

### Conservar dos notas, mover una

La manera más suave es conservar **dos** notas de cada grupo y mover
solo la tercera, para que cada grupo crezca a partir del anterior. Para
las tríadas, esto da un conjunto famoso de movimientos. Desde Do mayor
(Do Mi Sol):

| movimiento | conserva | mueve | da |
|------------|----------|-------|----|
| **P** (paralelo) | Do y Sol | Mi baja a Mi♭ | Do menor |
| **L** (intercambio de sensible) | Mi y Sol | Do baja a Si | Mi menor |
| **R** (relativo) | Do y Mi | Sol sube a La | La menor |

Son los tres movimientos de la **teoría neorriemanniana**, que los
teóricos usan para explicar los cambios de acorde cromáticos de Wagner
y Liszt. El estilo *Tríadas, dos notas fijas* camina así: Do menor, Do
mayor, Mi menor, Mi mayor, Do♯ menor, Re♭ mayor, Si♭ menor, Si♭
mayor…

El mismo recorrido sirve para cada micromodo, con una sorpresa. Tres de
ellos no pueden salir de un pequeño rincón de las doce notas:

- **Mayor 1** (Do Re Mi) y **Mayor 3** (Do Re Fa♯) solo tienen
  intervalos pares: 2, 4 y 6 semitonos. Una forma que conserva dos de
  sus notas tiene que añadir una tercera a una distancia par de ellas,
  así que se queda en la misma **escala de tonos enteros**. El recorrido
  llega a 6 notas, nunca a 12.
- El **acorde disminuido** solo tiene intervalos de 3 y 6 semitonos, así
  que se queda en un mismo **acorde de séptima disminuida**: 4 notas.
- El **acorde aumentado** solo tiene intervalos de 4 semitonos. Dos de
  sus notas fijan la tercera, así que ninguna otra forma comparte dos
  notas con él. Su recorrido lo sube un semitono en lugar de eso.

Todos los demás micromodos tienen algún intervalo impar, y su recorrido
da la vuelta por las doce notas. Las pruebas comprueban cada uno de
estos números.

### Qué forma conservar

Varias formas comparten dos notas con la actual (para Menor 3 sobre Do
Do♯ Mi hay tres: La Do Do♯, Do Re♯ Mi y Do♯ Mi Fa), así que el
recorrido necesita una regla para elegir una. En orden:

1. la forma cuya **nota nueva** lleva más tiempo sin oírse, para que la
   música recorra todas las notas a las que puede llegar,
2. después, la forma misma usada hace más tiempo, para que un recorrido
   encerrado en pocas notas (Mayor 1, el acorde disminuido) igual pase
   por todas sus formas en vez de ir y venir entre dos,
3. después, el menor movimiento de las notas que cambian,
4. y por último los bits del conjunto, para que la elección sea siempre
   la misma.

### Texturas

Un recorrido es una lista de formas. Una **textura** dice cómo las toca
un instrumento: como **acordes** (las notas compartidas se quedan donde
están y la nota que cambia se mueve al lugar más cercano: conducción de
voces suave), como acordes golpeados una y otra vez (**pulso**), como
una **línea** (primero la nota nueva, después las otras), como un
**arpegio** (subiendo y bajando), o como un **bajo** (la fundamental de
una tríada, si no, la primera nota de la forma normal). Cada
instrumento de un estilo toca el mismo recorrido, cada uno en su
textura, así que el bajo, los acordes y la melodía comparten siempre
sus tres notas.

Una línea toca primero la nota que acaba de llegar, para que se oiga
qué cambió, y después las dos que se quedaron. `write_forms` parte las
notas de la forma en dos con `partition`, que manda cada elemento a una
de dos colecciones según un closure de sí o no, en una sola pasada:

```rust
let (new, kept): (Vec<PitchClass>, Vec<PitchClass>) =
    normal.into_iter().partition(|p| !old.contains(*p));
new.into_iter().chain(kept).collect()
```

`old` es la forma anterior; las notas que no están en ella son nuevas.
Después `chain` vuelve a unir las dos listas, las notas nuevas primero.

Los siete estilos:

| estilo | micromodo | recorrido | sonido |
|--------|-----------|-----------|--------|
| Color Menor 3 | Menor 3 | conservar dos notas | acordes, melodía y bajo en un solo color |
| Puntos vieneses | Menor 5 | conservar dos notas | notas sueltas que saltan por tres octavas sobre acordes suaves |
| Pulso de clusters | Menor 1 | conservar dos notas | clusters que suben de a un semitono, sobre un ritmo techno |
| Bruma de tonos enteros | Mayor 1 | sube un tono | arpegios en 3/4 |
| Groove de cuartas | Mayor 4 | conservar dos notas | acordes sus sobre un ritmo neo soul |
| Tríadas, dos notas fijas | menor-mayor | conservar dos notas | movimientos P, L y R |
| Deriva aumentada | aumentado | baja un semitono | cuatro acordes aumentados, dando vueltas |

Una demostración toca una pasada desde el micromodo sobre Do, después
dado vuelta, después desde un tritono de distancia (Fa♯), después dado
vuelta desde allí, y otra vez la primera pasada. Haz clic en un estilo
de la Biblioteca mientras suena la canción y toda la canción pasa a
él: sus acordes y su bajo se reemplazan por el recorrido, los patrones
empiezan por turno desde esos cuatro lugares, y la tonalidad pasa a ser
el propio micromodo, un conjunto de clases de altura, así que las
teclas de armonía tocan sus formas, contadas desde casa (ver [Un
micromodo como tonalidad: grados](#un-micromodo-como-tonalidad-grados)).

## Los micromodos en Rust: una tabla en un `static`

Un micromodo son datos: un nombre, una familia, una forma prima, una
línea de texto. Los doce son un array `static` de un struct simple:

```rust
/// One of the twelve micromodes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Micromode {
    /// From 1 to 12: its place in the table, which is also its Forte
    /// number (3-`number`) and its hour on Peter Schat's Tone Clock.
    pub number: u8,
    pub family: MicromodeFamily,
    /// Kröpfl's name, in Spanish, as his students use it: "Menor 3".
    pub name: &'static str,
    /// The name in English.
    pub english: &'static str,
    /// The prime form, from C.
    pub prime: PitchSet,
    /// The prime form in solfège, as in Cetta's table: "do - do# - mi".
    pub solfege: &'static str,
    /// Where you hear it: a chord or scale it belongs to, or music that
    /// is built from it.
    pub about: &'static str,
}
```

```rust
static MICROMODES: [Micromode; 12] = [
    Micromode {
        number: 1,
        family: MicromodeFamily::Minor,
        name: "Menor 1",
        english: "Minor 1",
        prime: prime(0b0000_0000_0111),
        solfege: "do - do# - re",
        about: "Three notes a semitone apart: the smallest cluster, a chromatic scale in \
                miniature.",
    },
    // ...
];
```

`prime(0b0000_0000_0111)` es el conjunto {Do, Do♯, Re} escrito como los
bits de un `u16`, un bit por nota, con Do como el bit más bajo (ver [Un
conjunto de clases de altura en doce
bits](pitch.md#un-conjunto-de-clases-de-altura-en-doce-bits)).
`prime` es una `const fn`, así que toda la tabla se construye al
compilar y no cuesta nada al arrancar el programa. Cada campo es `Copy`
(números, enums, `&'static str` y `PitchSet`, que es un `u16`), así que
`Micromode` deriva `Copy` también, pero el resto del código pasa
`&'static Micromode`: una referencia a la tabla, que nunca puede quedar
colgando porque la tabla vive tanto como el programa. Los nombres de
Kröpfl están en castellano en los dos idiomas de la aplicación; en
inglés la ventana añade el nombre inglés entre paréntesis, «Menor 3
(minor 3)».

Encontrar el micromodo de tres notas reutiliza la teoría de conjuntos:

```rust
pub fn of(set: PitchSet) -> Option<&'static Micromode> {
    if set.len() != 3 {
        return None;
    }
    let prime = set.prime_form();
    MICROMODES.iter().find(|m| m.prime == prime)
}
```

### Comprobar una tabla con pruebas

Una tabla escrita a mano puede tener errores. Las pruebas la comprueban
desde tres direcciones, ninguna de las cuales confía en la tabla:

- **Contra Forte.** Las clases de conjuntos, filtradas a tres notas,
  tienen que ser las doce formas primas en el mismo orden, llamadas de
  3-1 a 3-12.
- **Contra la cuenta.** Un bucle sobre los 4.096 conjuntos (cada `u16`
  de 0 a 4.095) comprueba que exactamente los conjuntos de tres notas
  tienen un micromodo, que hay 220, y que las formas de cada micromodo
  coinciden con las veces que se lo encontró.
- **Contra las reglas.** Las familias se vuelven a deducir de los
  intervalos, con las reglas de Jaureguiberry, con una función que solo
  ven las pruebas:

```rust
#[cfg(test)]
fn of_intervals(vector: [u8; 6]) -> MicromodeFamily {
    if vector[0] > 0 {
        MicromodeFamily::Minor
    } else if vector[1] > 0 {
        MicromodeFamily::Major
    } else {
        MicromodeFamily::Chord
    }
}
```

`#[cfg(test)]` sobre una sola función la compila solo para
`cargo test`. Sin él, `cargo clippy` avisa que la función nunca se usa,
porque solo la llaman las pruebas. El solfeo también se comprueba: cada
sílaba se vuelve a convertir en un número (`"do#"` es 1, `"mib"` es 3)
y las notas tienen que ser la forma prima.

### Los intervalos en palabras

`vector[0]` cuenta segundas menores y `vector[1]` segundas mayores: es
el vector interválico de los [Conjuntos de clases de
altura](sets.md). El mismo vector da los intervalos en
palabras, para la ventana:

```rust
pub fn intervals(&self) -> String {
    const NAMES: [(&str, &str); 6] = [
        ("minor second", "minor seconds"),
        ("major second", "major seconds"),
        ("minor third", "minor thirds"),
        ("major third", "major thirds"),
        ("perfect fourth or fifth", "perfect fourths or fifths"),
        ("tritone", "tritones"),
    ];
    let vector = self.prime.interval_vector();
    let mut parts = Vec::new();
    for (class, &count) in vector.iter().enumerate() {
        let (one, many) = NAMES[class];
        // Whole phrases, as a Spanish number goes with its noun.
        match count {
            0 => {}
            1 => parts.push(crate::i18n::tr_string(format!("a {one}"))),
            2 => parts.push(crate::i18n::tr_string(format!("two {many}"))),
            _ => parts.push(crate::i18n::tr_string(format!("three {many}"))),
        }
    }
    crate::i18n::list(&parts)
}
```

Los plurales son un par de textos en vez de `format!("{name}s")`,
porque en inglés el plural no siempre es añadir una «s»: «two perfect
fourths or fifths», no «two perfect fourth or fifths». Y cada frase se
traduce entera («a minor second», «two minor seconds»), porque en
castellano el número cambia con el sustantivo: *una segunda menor*,
*dos segundas menores* (ver [Traducir: la clave es el texto en
inglés](../program/languages.md#traducir-la-clave-es-el-texto-en-inglés)).
Después `list` une las frases:

```rust
pub fn list(items: &[impl AsRef<str>]) -> String {
    match items.split_last() {
        None => String::new(),
        Some((last, [])) => last.as_ref().to_string(),
        Some((last, rest)) => {
            let rest: Vec<&str> = rest.iter().map(AsRef::as_ref).collect();
            trf("{} and {}", &[&rest.join(", "), &last.as_ref()])
        }
    }
}
```

`split_last` da el último elemento y la porción de antes, o `None` para
una lista vacía. `Some((last, []))` es un **patrón de slice**: `[]` solo
coincide con una porción vacía, así que esta rama es una lista de uno.
Así, `["una segunda menor", "una tercera menor", "una tercera mayor"]`
se vuelve «una segunda menor, una tercera menor y una tercera mayor»,
con la «y» (o la inglesa «and») en su lugar y sin aritmética de índices
que pueda fallar. `items: &[impl AsRef<str>]` acepta igual una porción
de `String` que de `&str`.

### Contar grupos de tres

`trichords` lista cada grupo de tres notas de un conjunto, con tres
bucles anidados cuyos índices siempre suben, así que ningún grupo se
lista dos veces:

```rust
pub fn trichords(set: PitchSet) -> Vec<PitchSet> {
    let notes: Vec<PitchClass> = set.iter().collect();
    let mut trios = Vec::new();
    for a in 0..notes.len() {
        for b in a + 1..notes.len() {
            for c in b + 1..notes.len() {
                trios.push([notes[a], notes[b], notes[c]].into_iter().collect());
            }
        }
    }
    trios
}
```

`[notes[a], notes[b], notes[c]].into_iter().collect()` convierte un
array de tres clases de altura en un `PitchSet`: `PitchSet` implementa
`FromIterator<PitchClass>`, así que `collect` sabe construir uno, y el
tipo de retorno `Vec<PitchSet>` le dice qué colección construir.

`micromode_content` los cuenta en un `[u16; 12]`, un contador por
micromodo. Para mostrarlos, `micromodes_in` se queda con los
encontrados y los ordena, los más frecuentes primero:

```rust
pub fn micromodes_in(set: PitchSet) -> Vec<(&'static Micromode, u16)> {
    let counts = micromode_content(set);
    let mut found: Vec<(&'static Micromode, u16)> = MICROMODES
        .iter()
        .zip(counts)
        .filter(|(_, count)| *count > 0)
        .collect();
    found.sort_by_key(|(m, count)| (std::cmp::Reverse(*count), m.number));
    found
}
```

`sort_by_key` ordena desde la clave más pequeña, y aquí la cuenta *más
grande* va primero. `std::cmp::Reverse` envuelve un valor y lo compara
al revés, así que `Reverse(8)` va antes que `Reverse(6)`. La clave es
una **tupla**, y las tuplas se comparan elemento por elemento: primero
las cuentas (al revés), y solo cuando dos cuentas son iguales, el número
del micromodo. Así, las cuentas iguales conservan el orden de la tabla,
y la lista sale igual cada vez que se dibuja.

## El recorrido: elegir con una tupla

La regla del recorrido, «la nota nueva más fresca, después la forma
usada hace más tiempo, después el menor movimiento», es un solo
`min_by_key` con una tupla de cuatro:

```rust
members
    .iter()
    .copied()
    .filter(|&set| set != current && shared(set) == most)
    .min_by_key(|&set| {
        let new = PitchSet::from_bits(set.bits() & !current.bits());
        // The new note heard most recently decides; never heard is best.
        let freshest = new
            .iter()
            .map(|p| heard[usize::from(p.semitone())].map_or(0, |i| i + 1))
            .max()
            .unwrap_or(0);
        // How far the moving notes go, each to its nearest new note.
        // ...
        // Then the form itself heard least recently, so a walk shut in
        // a few notes still goes round all its forms.
        let used = before.iter().rposition(|&f| f == set).map_or(0, |i| i + 1);
        (freshest, used, moves, set.bits())
    })
    .unwrap_or(current)
```

Algunas cosas para notar:

- `members` son todas las formas del micromodo (24, 12 o 4).
  `shared(set)` cuenta las notas que una forma tiene en común con la
  actual, con los bits: `set.bits() & current.bits()` se queda con las
  notas que están en las dos. `set.bits() & !current.bits()` se queda
  con las notas de `set` que *no* están en `current`: la nota nueva.
- `heard` es un array de doce `Option<usize>`, uno por clase de altura:
  `None` si la nota todavía no sonó, o `Some(i)` si sonó por última vez
  en la forma número `i`. `map_or(0, |i| i + 1)` convierte «nunca» en
  0, la clave más pequeña, así que una nota nunca oída siempre gana, y
  una nota oída hace mucho le gana a una oída recién.
- `rposition` busca en una porción **desde el final** y da el índice
  del último elemento que coincide: cuándo se usó la forma por última
  vez. Otra vez, «nunca» se vuelve 0.
- Como la tupla se compara elemento por elemento, el orden de sus
  cuatro elementos *es* el orden de las reglas. Cambiar las reglas es
  reordenar una tupla, no reescribir una cadena de `if`.

¿Cuántas formas necesita una pasada? Un estilo dice cuántos pasos dura
cada forma, y el patrón tiene una duración en pasos; la cuenta redondea
hacia arriba, así que una última forma que solo cabe a medias igual se
escribe:

```rust
let count = length.div_ceil(self.steps.max(1)).max(1);
```

`div_ceil` es la división que redondea hacia arriba: `16.div_ceil(6)`
es 3, donde `16 / 6` es 2.

## Los estilos se adueñan de la armonía

Un estilo de micromodo, como un estilo dodecafónico, no tiene acordes
que seguir. Cuando haces clic en él mientras suena la canción,
*reemplaza* la armonía de la canción: calla los acordes y el bajo que
no toca, y escribe sus propias voces en los canales correspondientes.
Los estilos de series hacen lo mismo, así que los dos tipos de estilo
llaman a una sola función, de [Series dodecafónicas y sus
reglas](twelve-tone.md#tocar-una-canción-entera-como-una-serie):

```rust
pub(super) fn take_over_harmony(
    project: &mut Project,
    uses: &dyn Fn(&str) -> bool,
    rhythm: Option<&Rhythm>,
    mut write: impl FnMut(&mut Project, usize, usize, Option<usize>),
) -> RhythmChange {
```

Los dos closures se pasan de dos maneras distintas, a propósito:

- `uses` solo responde una pregunta («¿el estilo toca este canal?»),
  así que es un `&dyn Fn`: un **trait object**, una referencia a algún
  closure cuyo tipo se decide al ejecutar. Una sola copia compilada de
  `take_over_harmony` sirve para todos los closures.
- `write` se llama una vez por patrón con `&mut Project`, y hace la
  escritura. Es un `impl FnMut`: el compilador hace una copia de
  `take_over_harmony` para cada tipo de closure que se le pasa (una para
  los estilos de series, otra para los de micromodos), lo que es un poco
  más rápido y deja que el closure cambie lo que capturó. `FnMut` es la
  cota más laxa que todavía se puede llamar muchas veces.

Los estilos de micromodos la llaman así:

```rust
pub fn use_micromode_style(project: &mut Project, style: &MicromodeStyle) -> RhythmChange {
    let key = project.scale.root;
    let uses = |name: &str| style.voices.iter().any(|voice| voice.channel == name);
    let change = take_over_harmony(
        project,
        &uses,
        style.rhythm(),
        |project, pattern, n, keys| {
            let start = DEMO_SECTIONS[n % DEMO_SECTIONS.len()].0;
            write_voices(project, pattern, style, key, start, keys);
        },
    );
    if change.harmony > 0 {
        project.scale = style.scale(key);
    }
    change
}
```

`n` cuenta los patrones escritos hasta ahora, así que el primer patrón
de la canción empieza desde el micromodo, el siguiente dado vuelta, el
siguiente a un tritono, y así: la canción no se repite. El closure
recibe `project` como argumento en vez de capturarlo:
`take_over_harmony` ya tiene `&mut Project`, y Rust solo permite un
préstamo mutable a la vez, así que el proyecto se le presta al closure
en cada llamada.

Las demostraciones se arman como las dodecafónicas: `section_patterns`
hace un patrón por sección y lo llena con un closure, y
`arrange_sections` los pone en la lista de reproducción, con la batería
entrando después de la primera sección. `micromode_demo` y `row_demo`
usan las dos, cada una pasando su propio closure.

## Un micromodo como tonalidad: grados

En Do mayor el acorde sobre Re se llama ii, esté donde esté la canción:
«ii» dice dónde está el acorde en la tonalidad, no qué notas tiene, así
que una progresión escrita en números romanos suena en cualquier
tonalidad. Una canción cuya tonalidad es un micromodo necesita la misma
clase de nombre para sus formas. Pero sus formas no se construyen sobre
las notas de una escala: cada una es el grupo de tres notas de la
tonalidad movido, o dado vuelta y movido. Así que el nombre dice
justamente eso, cuánto y si está dado vuelta, medido desde la
tonalidad.

El **grado** de una forma dice cómo se hace a partir de la forma propia
de la tonalidad: **T*n*** es casa subida *n* semitonos, y **T*n*I** es
casa dada vuelta alrededor de la nota de la tonalidad y después subida
*n*. Dada vuelta quiere decir reflejada: cada nota queda tan por debajo
de la nota de la tonalidad como estaba por encima. En una canción en
Menor 3 sobre Re (Re Re♯ Fa♯), Mi Fa Sol♯ es **T2**; dada vuelta
alrededor de Re, Re Re♯ Fa♯ se convierte en Re Do♯ La♯, y un semitono
más arriba, Si Re Re♯, es **T1I**:

| Grado | Quiere decir | En Menor 3 sobre Do | En Menor 3 sobre Re |
|---|---|---|---|
| T0 | la forma propia de la tonalidad, casa | Do Do♯ Mi | Re Re♯ Fa♯ |
| T2 | casa subida 2 semitonos | Re Re♯ Fa♯ | Mi Fa Sol♯ |
| T0I | casa dada vuelta alrededor de la nota de la tonalidad | La♭ Si Do | La♯ Do♯ Re |
| T1I | eso, subido 1 | La Do Do♯ | Si Re Re♯ |
| T4 | casa subida 4 | Mi Fa Sol♯ | Fa♯ Sol La♯ |

Como ii, un grado sigue igual cuando toda la pieza se mueve: la última
columna es la anterior subida un tono, con los mismos grados. En Menor
3 sobre Do los grados son justamente los nombres que la hoja de acordes
escribe después de un micromodo («Menor 3 T4», «Menor 3 T1I»), que
cuentan desde la forma de la tabla sobre Do; en cualquier otra
tonalidad se cuentan desde la tonalidad.

Dónde está el espejo importa. El T*n*I de la teoría de conjuntos de
clases de altura da vuelta las notas alrededor de Do, y con ese espejo
un nombre cambiaría cuando la música se mueve: en Menor 3 sobre Do, La
Do Do♯ es Do Do♯ Mi dada vuelta alrededor de Do y subida 1, pero un tono
más arriba las mismas figuras, Si Re Re♯ desde Re Re♯ Fa♯, serían «dada
vuelta alrededor de Do y subida 5». La nota propia de la tonalidad se
mueve con la pieza, así que la app refleja alrededor de ella, y Si Re
Re♯ en Menor 3 sobre Re es T1I, como La Do Do♯ en Menor 3 sobre Do.

Casa es siempre T0, esté como esté la forma propia de la tonalidad.
Para las tríadas, el acorde paralelo es T7I desde los dos lados: en Do
mayor como tonalidad, Do Mi Sol dada vuelta alrededor de Do es Do La♭
Fa, Fa menor, y una quinta más arriba eso es Do menor; en Do menor como
tonalidad, Do mayor también es T7I. El relativo menor de Do mayor, La
menor, es T4I, y el relativo mayor de Do menor, Mi♭ mayor, es T10I.

### Qué tan cerca está una forma: las notas que conserva

Una tonalidad tonal ordena sus acordes por lo que hacen (tónica,
subdominante, dominante). Un micromodo no tiene esos papeles, porque
todas sus formas tienen los mismos intervalos; lo que distingue dos
formas, al oído, es cuántas notas comparten. Para tres notas:

- conservar **dos** notas y mover una es el enlace más suave, el
  movimiento del recorrido de [Conservar dos notas, mover
  una](#conservar-dos-notas-mover-una);
- conservar **una** nota es un pivote: una nota se queda mientras dos
  cambian;
- no conservar **ninguna** es un contraste: las tres notas son nuevas.

Menor 3 tiene 24 formas. Desde Do Do♯ Mi, tres conservan dos notas (La
Do Do♯, Do Mi♭ Mi y Do♯ Mi Fa), nueve conservan una y once no conservan
ninguna. Así que las teclas de armonía de una tonalidad que es un
micromodo muestran sus formas en ese orden, casa primero, cada una más
brillante cuanto más conserva de casa, y dejan fuera las que no
conservan nada hasta que **Todas las formas** las pide. Después de que
suena una forma, las formas que conservan dos de *sus* notas aparecen
con borde, como las teclas tonales marcan con borde los acordes que suelen
venir después, y la ayuda dice qué se queda y qué se mueve: «conserva La
y Do; Do♯ pasa a La♭». Para las tríadas también nombra el movimiento, P,
L o R.

En la hoja de acordes, cada compás de una tonalidad que es un micromodo
muestra su grado y, desde el segundo compás, cuántas notas conserva del
compás anterior: «T1I · 2 comunes». Los grados también se pueden
escribir: `T2` y `T5I` son esas formas de la tonalidad.

### Los grados en Rust: un struct `Copy` pequeño

Un grado son dos datos, así que es un struct de dos campos, en
`harmony/degree.rs`:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Degree {
    /// How many semitones it is moved, from 0 to 11.
    pub up: u8,
    /// Whether it is turned upside down first, around the key's note.
    pub inverted: bool,
}
```

Es `Copy` porque ocupa dos bytes: pasarlo por valor no cuesta nada, y el
código puede seguir usándolo después de pasarlo sin pensar en la
propiedad.

Encontrar el grado es una búsqueda: casa subida de 0 a 11 semitonos, y
después, solo si ninguna de esas es la forma, casa dada vuelta y
movida:

```rust
pub fn of(key: Scale, form: PitchSet) -> Option<Degree> {
    let own = key.pitch_set();
    if own.is_empty() || own.len() != form.len() {
        return None;
    }
    let mirrored = mirror(own, key.root);
    let moved = |shape: PitchSet| (0..12u8).find(|&up| shape.transpose(up) == form);
    moved(own)
        .map(|up| Degree {
            up,
            inverted: false,
        })
        .or_else(|| moved(mirrored).map(|up| Degree { up, inverted: true }))
}
```

El closure `moved` prueba los doce movimientos de una figura; `find` se
detiene en el primero que coincide y lo da como `Some(up)`, o `None`.
`map` convierte `Some(up)` en `Some(Degree { .. })` y deja `None` como
está, y `or_else` ejecuta su closure solo con `None`, así que la figura
reflejada solo se prueba cuando casa movida no puede dar la forma. Ese
orden importa para un conjunto que es igual dado vuelta, como el acorde
aumentado o Do mayor como conjunto: cada forma suya es casa movida, así
que recibe el nombre más simple.

El espejo es una sola línea:

```rust
pub fn mirror(set: PitchSet, note: PitchClass) -> PitchSet {
    set.invert().transpose((2 * note.semitone()) % 12)
}
```

`invert` da vuelta un conjunto alrededor de Do: cada nota *n* semitonos
por encima de Do pasa a *n* por debajo. Subir el resultado el doble de
la distancia de Do a la nota de la tonalidad pone el espejo en esa nota:
una nota *n* por encima de Re cae *n* por debajo de Re. El `% 12` deja
el movimiento por debajo de una octava; un `u8` por dos puede llegar a
22.

`Display` escribe un grado como «T2» o «T5I», y `parse` lo vuelve a leer.
`strip_prefix` y `strip_suffix` reciben un patrón, y un array de
caracteres es un patrón que coincide con cualquiera de ellos, así que las
letras pueden ir en mayúscula o minúscula:

```rust
pub fn parse(text: &str) -> Option<Degree> {
    let number = text.trim().strip_prefix(['T', 't'])?;
    let (number, inverted) = match number.strip_suffix(['I', 'i']) {
        Some(number) => (number, true),
        None => (number, false),
    };
    if number.is_empty() || !number.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }
    let up = number.parse::<u8>().ok().filter(|&up| up < 12)?;
    Some(Degree { up, inverted })
}
```

La comprobación de los dígitos está porque `"+3".parse::<u8>()` funciona;
`filter` convierte T12 en `None`.

El orden de las teclas de armonía es un solo ordenamiento, otra vez con
una tupla cuyos elementos son las reglas en orden:

```rust
forms.sort_by_key(|&(form, degree)| {
    (
        degree != Degree::HOME,
        std::cmp::Reverse(kept(own, form)),
        degree.inverted,
        degree.up,
    )
});
```

`false` va antes que `true`, y `degree != Degree::HOME` es `false` solo
para casa, así que casa va primero. `Reverse`
invierte el orden de lo que envuelve, así que las formas que conservan
*más* notas van primero, sin cambiarle el signo a una cuenta. Después
las formas derechas van antes que las dadas vuelta, y por último van por
grado. `kept` cuenta las notas compartidas con los bits, como el
recorrido: `PitchSet::from_bits(a.bits() & b.bits()).len()`.

## La ventana: un reloj dibujado con el painter de egui

Las doce notas se sientan en un círculo como las horas de un reloj, Do
arriba, y las tres notas del micromodo se unen en un triángulo. La
forma *es* el micromodo: mover el grupo hace girar el triángulo por el
reloj, y darlo vuelta lo refleja como en un espejo. Por eso Schat llamó
horas a los tricordios.

```rust
/// The point for pitch class `n` on a clock face: C at the top, going
/// round clockwise a semitone at a time.
fn on_clock(center: Pos2, radius: f32, n: u8) -> Pos2 {
    let angle = f32::from(n) / 12.0 * TAU;
    center + Vec2::new(angle.sin(), -angle.cos()) * radius
}
```

`TAU` es 2π, una vuelta entera, así que la clase de altura `n` está a
`n` doceavos de vuelta. En matemáticas, el ángulo 0 apunta a la derecha
y los ángulos giran en sentido antihorario; aquí el 0 está arriba, y se
gira en sentido horario. Usar `sin` para x y `-cos` para y logra las
dos cosas: en 0, x es 0 e y es −1, que en una pantalla es *arriba*,
porque la y de la pantalla crece hacia abajo.

`clock` pide entonces un cuadrado de la ventana con
`ui.allocate_exact_size`, toma un `painter_at` en ese rectángulo (lo
que se dibuje fuera se recorta), y dibuja: el círculo con
`circle_stroke`, el triángulo con `Shape::convex_polygon` (relleno con
el color de acento a un cuarto de fuerza, `gamma_multiply(0.25)`), un
punto para cada nota, más grande para las tres del grupo, y los nombres
alrededor. Los botones de la lista, cada uno un reloj diminuto y un
nombre, se dibujan igual: `allocate_exact_size` con `Sense::click()` da
una `Response` que sabe de pasar el puntero y de clics, como un botón
de verdad.

### Dos ventanas que se señalan una a la otra

Ninguna de las dos ventanas llama a la otra. Cada una devuelve una
lista de **acciones**, como las demás ventanas del piano roll:

```rust
/// Something done in the micromode window.
pub enum MicroAction {
    /// Make this group the key.
    UseAsKey(Scale),
    /// Play these notes now.
    Hear(Vec<Note>),
    /// Show this set in the set browser.
    ShowSet(PitchSet),
}
```

La ventana de conjuntos tiene dos acciones propias para esto,
`ShowMicromode` y `OpenMicromodes`. El piano roll es dueño de las dos
ventanas, así que maneja las acciones: `ShowSet` llama al `pick` de la
ventana de conjuntos y la abre; `ShowMicromode` llama al `pick` de la
ventana Micromodos. Cada ventana solo sabe mostrar un conjunto que le
dan, y el piano roll las conecta. (**Dibujar con él** no necesita
ninguna acción: las dos ventanas reciben el `ChordStamp` del piano roll
como `&mut`, y lo cambian directamente.)

`pick` muestra no solo el micromodo sino la forma exacta en la que se
hizo clic. Haz clic en Menor 3 bajo la serie de Webern en la ventana de
conjuntos, y la ventana Micromodos se abre en Si Si♭ Re, no en Do Do♯
Mi:

```rust
pub fn pick(&mut self, form: PitchSet) {
    if let Some(micromode) = Micromode::of(form) {
        self.micromode = micromode;
        let (n, inverted) = micromode.prime.moved_to(form).unwrap_or((0, false));
        self.transposition = n;
        self.inverted = inverted;
        self.raise = true;
    }
}
```

`moved_to`, de los [Conjuntos de clases de
altura](sets.md), encuentra cómo mover la forma prima sobre
la forma: subir `n` semitonos, dada vuelta o no.

Una ventana que se muestra desde otra tiene que pasar al frente, o puede
quedar *detrás* de la ventana en la que hiciste clic. A una ventana que
se abre la mantiene delante `Front` (ver [Una ventana que se abre
delante](twelve-tone.md#una-ventana-que-se-abre-delante));
a una que ya estaba abierta, pero detrás, la sube la bandera `raise`:
`pick` la pone, y el fotograma siguiente la lee y la borra de una vez
con `std::mem::take`, que devuelve el valor y deja `false` en su lugar:

```rust
if std::mem::take(&mut self.raise) {
    ctx.move_to_top(egui::LayerId::new(egui::Order::Middle, window_id()));
}
```

La ventana de conjuntos hace lo mismo con una bandera `reveal`, y
además desplaza su larga lista hasta la clase que se le pidió mostrar:
`response.scroll_to_me(Some(egui::Align::Center))` sobre la línea de
esa clase, durante un solo fotograma.

### Escribir las tríadas como tríadas

Do, Mi♭ y Fa♯ y Do, Mi♭ y Sol♭ son las mismas tres teclas, pero solo
una es una tríada disminuida en el papel: Do Mi♭ Sol♭, dos terceras
menores apiladas. Los tres micromodos de acordes se escriben como los
acordes que son. El trait `NoteNames`, que escribe notas con la
escritura de la tonalidad o con sostenidos o bemoles sin más, tiene un
segundo método para esto:

```rust
pub trait NoteNames: Copy {
    fn note_name(self, pitch_class: PitchClass) -> NoteName;
    /// The notes of a chord, from its root up.
    fn chord_names(self, chord: Chord) -> Vec<NoteName>;
}
```

`Micromode::chord` encuentra la tríada de una forma (la fundamental es
la nota de la forma normal desde la cual las otras dos forman un acorde
conocido), y la ventana escribe sus notas con `chord_names`. El acorde
aumentado no tiene una fundamental de verdad (cada una de sus notas
está a una tercera mayor de la siguiente), así que se nombra desde la
primera nota de su forma normal.

## Fuentes

Kröpfl nunca publicó su teoría; como escribe Jaureguiberry, «esta
propuesta nunca fue publicada por su autor». Se conoce por sus alumnos
y por análisis de su música. Estas son las fuentes de este capítulo y
de los textos de la aplicación:

- Pablo Cetta, «*La tercera es la vencida* de Francisco Kröpfl y Oscar
  Steimberg», *Cuadernos de Análisis y Debate sobre Músicas
  Latinoamericanas Contemporáneas* 1 (2018), pp. 8–36. La tabla de los
  doce micromodos (Ejemplo 1) con los números de Forte, las formas
  primas en solfeo, los ejemplos Mi Fa La♭ y Fa Re Sol♭, el análisis de
  *Música '66* (Menor 3 y Menor 5, el conjunto 4-9) y de la canción de
  la bruja en la ópera de cámara de Kröpfl (2011), construida sobre
  todo con Menor 3. En línea, en el repositorio de la Universidad
  Católica Argentina.
- Pablo Jaureguiberry, «*Tres amores* de Diana Rud: otras
  particularidades de los micromodos de Francisco Kröpfl» (Universidad
  Nacional de Rosario, 2016), pp. 499–510. La definición, las tres
  familias y sus reglas, Menor 3 como uno de los preferidos de los
  alumnos de Kröpfl, y conjuntos de cuatro notas hechos de dos Menor 2
  y dos Menor 3.
- Alejandro Martínez, «Algunos aspectos de los micromodos de Francisco
  Kröpfl y su contextualización histórica», en *Actas de la XXI
  Conferencia de la Asociación Argentina de Musicología* (2015),
  pp. 399–415. El micromodo como la unidad mínima de la melodía y la
  armonía, su uso por compositores argentinos de varias generaciones, y
  su lugar junto a los conjuntos de Forte, el análisis tricordal de
  Gilbert y el Tone Clock de Schat.
- Marcela Pavia, «La estructura de la música atonal: dos enfoques»,
  *CodeXXI* 1 (1998), pp. 199–214. La clasificación de Kröpfl,
  construida sobre la percepción, comparada con la de Forte, construida
  sobre la matemática.
- Allen Forte, *The Structure of Atonal Music* (Yale University Press,
  1973). Los tricordios de 3-1 a 3-12.
- Peter Schat, *The Tone Clock* (Harwood Academic Publishers, 1993).
  Las doce horas.

Los ejemplos musicales de otros compositores solo se nombran, nunca se
copian, salvo las doce notas de la serie de Webern.

## Cómo se prueba

`micromode.rs` comprueba la tabla y las fuentes: los doce coinciden con
los tricordios de Forte en orden; los 4.096 conjuntos dan 220 grupos de
tres, cada uno en exactamente un micromodo, con 12, 24 o 4 formas; las
familias se deducen de los intervalos con las reglas de Jaureguiberry;
el solfeo escribe cada forma prima; los nombres se encuentran en
castellano y en inglés, en mayúsculas o minúsculas; Mi Fa La♭ y Fa Re
Sol♭ de Cetta, Si Sol♯ Sol de Schoenberg y los cuatro grupos de la serie
de Webern son Menor 3; Do Do♯ Re♯ Mi tiene dos Menor 2 y dos Menor 3;
4-9 tiene cuatro Menor 5; la séptima de dominante tiene uno de cada uno
de Mayor 2, Mayor 3, el acorde disminuido y menor-mayor; y la escala de
tonos enteros tiene solo Mayor 1, Mayor 3 y el acorde aumentado.

`micromodal.rs` comprueba el recorrido y la escritura: cada paso
conserva dos notas; el recorrido llega a las doce notas, o a 6 para
Mayor 1 y Mayor 3, o a 4 para el acorde disminuido, sin repetir una
forma en sus cuatro primeros pasos; los movimientos de las tríadas son
P, L o R; una secuencia de Mayor 1 se queda en una escala de tonos
enteros; los acordes conservan sus notas compartidas en las mismas
teclas; cada textura escribe notas de las formas correctas y nunca
después del final del patrón; el bajo toca fundamentales.

```rust
#[test]
fn the_bass_plays_the_root_of_a_triad() {
    assert_eq!(bass_note(set(&[9, 0, 4])), PitchClass::A);
    assert_eq!(bass_note(set(&[11, 2, 5])), PitchClass::B);
    assert_eq!(bass_note(set(&[0, 1, 4])), PitchClass::C);
    assert_eq!(bass_note(set(&[11, 0, 3])), PitchClass::B);
}
```

La Do Mi es La menor, así que su bajo es La; Si Re Fa es Si disminuido;
Do Do♯ Mi no es una tríada, así que su bajo es la primera nota de su
forma normal, Do; y Si Do Mi♭ tampoco es una tríada, y empieza en Si.

`harmony/degree.rs` comprueba los grados: contados desde la forma
propia de la tonalidad y reflejados alrededor de su nota, iguales
cuando toda la pieza se mueve, casa T0 esté como esté la tonalidad (Do
mayor y Do menor, cada uno T7I del otro), leídos como se escriben («T2», «t7i») y nunca
T12; las 24 formas de Menor 3 en orden de cercanía (3, 9 y 11 formas que
conservan dos, una y ninguna nota), las cuatro formas del acorde
aumentado sin enlace suave; y los nombres de los movimientos P, L y R.

`library/micromodes.rs` comprueba que cada estilo toca solo su
micromodo, en cualquier tonalidad y desde el comienzo de cada sección,
con la tonalidad convertida en el micromodo; que la primera forma
empieza en la tónica; y que una canción toma un estilo de micromodo. Las
pruebas de la aplicación comprueban que los micromodos de acordes se
escriben como acordes (Do Mi♭ Sol♭), que **▶ Escucharlo** toca las tres
notas desde el Do central, y que elegir una forma muestra cómo se
movió.

Los ejercicios sobre micromodos están en [Ejercicios: la teoría musical
en código](exercises.md).
