# Micromodos: los doce colores de tres notas de Kröpfl

Archivos: [`crates/lineup-core/src/harmony/micromode.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/harmony/micromode.rs),
[`crates/lineup-core/src/micromodal.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/micromodal.rs),
[`crates/lineup-core/src/library/micromodes.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/micromodes.rs),
[`crates/lineup-core/src/library/rows.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/rows.rs),
[`crates/lineup-app/src/micromodes.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/micromodes.rs)
y [`crates/lineup-app/src/twelve_tone.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/twelve_tone.rs)

El capítulo anterior dio un nombre a cada grupo de notas: su conjunto de
clases de alturas. Este mira de cerca los grupos más pequeños que aún
tienen carácter propio, los grupos de **tres** notas, de la manera en
que el compositor argentino **Francisco Kröpfl** enseñó a sus alumnos a
oírlos. Él los llamó **micromodos** (*micromodes*): doce «modos»
pequeños de tres notas, cada uno con un color que aprendes a reconocer.

Qué gana Ferrous Lineup:

- **Micromodes…** en el piano roll abre una ventana con los doce
  micromodos de Kröpfl en sus tres familias. Cada uno se dibuja en un
  reloj, con su forma prima en solfeo (*do - do# - mi*), sus intervalos,
  su conjunto de clases de alturas y su hora en el Tone Clock de Peter
  Schat. Puedes oírlo, dibujar con él o usarlo como tonalidad.
- Selecciona algunas notas y la ventana lista los micromodos que hay
  dentro: las siete notas de Do mayor contienen 35 grupos de tres, y ves
  cuántos hay de cada color.
- La ventana **Sets** y la ventana **Micromodes** se señalan la una a la
  otra. Un conjunto de tres notas dice qué micromodo es, un conjunto
  más grande lista los micromodos que contiene, y cada nombre es un
  botón que abre la otra ventana en la clase correcta.
- La Library tiene un grupo **Micromodes (Kröpfl)** con siete estilos, y
  **File > Demos > Micromodes** toca cada uno como una pieza corta en
  cinco secciones.
- La serie del *Concierto*, Op. 24, de Webern, hecha de cuatro grupos de
  tres que son todos el mismo micromodo, se une a las series famosas,
  con un estilo y un demo propios.

![La ventana Micromodes sobre el piano roll. Los acordes de Do mayor de la canción inicial están seleccionados: sus siete notas contienen Mayor 2 ocho veces, Acorde menor-mayor seis veces, y así. Abajo, las tres familias, y Menor 3 dibujado en un reloj: Do, Re♭ y Mi unidos en un triángulo, con su forma prima do - do# - mi y dónde se oye.](../images/step-8-micromodes.png)

![La ventana Sets en 7-35, la escala diatónica, abierta desde la ventana Micromodes. Bajo los detalles, «Micromodes inside, en sus 35 grupos de tres», cada uno un botón de vuelta a la ventana Micromodes.](../images/step-8-micromodes-in-sets.png)

## Qué hacen otros programas de música, y qué hacemos nosotros

Los programas de música no tienen nada parecido; como el capítulo
anterior, este no es una copia de nada. Es la herramienta que un
estudiante del método de Kröpfl querría junto a un piano roll: algo que
nombra los grupos de tres de lo que tocas, y te los toca de vuelta, para
que tu oído aprenda sus colores.

## Tres notas, doce colores

Francisco Kröpfl (1931–2021) nació en Timișoara, Rumania, y creció en
Argentina. Fue pionero de la música electrónica en América Latina: en
1958 fundó el Estudio de Fonología Musical en la Universidad de Buenos
Aires, el primer estudio de música electrónica de una universidad del
continente. Fue también profesor de composición durante décadas, y su
pieza para piano *Música '66* se escribió para probar el método que
enseñaba.

Ese método parte de una observación simple. Elige tres notas distintas
cualesquiera. Las deletrees como las deletrees, en el orden que sea y en
las octavas que sean, las tres notas tienen tres intervalos entre ellas,
y esos intervalos dan al grupo un color: Do, Re♭ y Mi tienen el mismo
sonido tenso y agridulce tanto si las tocas como acorde, como melodía o
repartidas en cuatro octavas. Mueve el grupo hacia arriba (Re, Mi♭, Fa♯) y
conserva su color. Ponlo al revés (Do, Si, La♭, los mismos intervalos
bajando) y el color sigue siendo la misma familia de sonido.

Así que Kröpfl contaba tres notas como **el mismo micromodo** cuando
son:

- las mismas notas en otro orden u octava, o deletreadas de otra manera
  (Do♯ y Re♭ son una sola nota aquí),
- movidas arriba o abajo (transpuestas), o
- puestas al revés (invertidas).

Estas son exactamente las reglas de los conjuntos de clases de alturas
del capítulo anterior. De las 220 maneras de elegir tres notas entre
doce, las reglas dejan solo **doce** micromodos distintos. Repite uno de
ellos a lo largo de un pasaje y el pasaje toma su color; Pablo Cetta,
compositor y teórico que estudió la música de Kröpfl, lo describe como
«un "color" o timbre global particular y reconocible», un color o timbre
global particular y reconocible.

### La tabla

Cada micromodo se escribe en su **forma prima**: su forma más compacta,
empezando en Do (*do*), la misma forma prima que en el navegador de
conjuntos. Esta es la tabla de Pablo Cetta, con los números que Allen
Forte dio a los mismos doce grupos en 1973:

| | Micromodo | Forma prima | Forte | Intervalos (vector) | Formas |
|---|-----------|------------|-------|--------------------|-------|
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

«Cuarta» aquí significa cuarta justa *o* quinta justa: en un conjunto,
un intervalo y su inversión cuentan como uno (de Do a Sol es una quinta, de
Sol a Do una cuarta; el par Do y Sol es el mismo de las dos maneras). «Formas»
cuenta cuántos grupos distintos de tres notas son ese micromodo: la
forma prima y su inversión en cada una de las doce notas. La mayoría
tienen 24; los que son su propia inversión tienen 12; la tríada
aumentada solo 4, porque moverla una tercera mayor hacia arriba da las
mismas tres notas otra vez. Las formas suman 220, todos los grupos de
tres que hay.

### Tres familias, y sus nombres

Kröpfl ordenó los doce en tres familias. Pablo Jaureguiberry, que
analizó la música de Diana Rud, alumna de Kröpfl, da las reglas:

- «Los micromodos **menores** son todos aquellos que están compuestos
  por al menos un intervalo de segunda menor»: los micromodos
  **menores** son todos los que tienen al menos una segunda menor (un
  semitono). Hay cinco, Menor 1 a 5.
- «Los **mayores** tienen todos al menos una segunda mayor y a su vez
  carecen de semitonos»: los **mayores** tienen todos al menos una
  segunda mayor y ningún semitono. Hay cuatro, Mayor 1 a 4.
- «Los restantes contienen al menos dos terceras cada uno»: los
  restantes tienen al menos dos terceras cada uno. Son las tres
  tríadas, y se nombran por ellas: el acorde **disminuido**, el
  **menor-mayor** (*menor-mayor*) y el **aumentado**.

Los nombres vienen de la forma prima, como explica Cetta: los nueve
primeros empiezan con una segunda, menor (*do - do#*) o mayor
(*do - re*), y el número sube a medida que sube la tercera nota. Así que
Menor 3 es «el tercer grupo que empieza con una segunda menor»,
*do - do# - mi*.

Un micromodo merece una segunda mirada: **Acorde menor-mayor**. Su forma
prima, Do Mi♭ Sol, es una tríada menor. Puesta al revés desde Do es Do, La y Fa:
una tríada de Fa mayor. Las tríadas menor y mayor son imágenes
especulares la una de la otra, así que son un solo micromodo, y su
nombre dice ambas cosas. Cada acorde de una progresión I–IV–V–I es el
mismo micromodo; lo que hace la música tonal con ellos es cuestión de
*qué* formas, no de qué color.

### Encontrar el micromodo de tres notas a mano

Cetta da dos ejemplos de Menor 3: Mi, Fa y La♭, y Fa, Re y Sol♭.

1. **Mi Fa La♭.** Pon las notas en orden dentro de una octava, tan juntas
   como quepan: Mi Fa La♭ abarca cuatro semitonos, la amplitud más pequeña.
   Mueve el grupo para que empiece en Do: Do Do♯ Mi. Eso es *do - do# - mi*,
   la forma prima de Menor 3.
2. **Fa Re Sol♭.** En orden y lo más juntas posible: Re Fa Sol♭, una amplitud de
   cuatro semitonos. Desde Do: Do Mi♭ Mi, que es *do - mib - mi*, y no está
   en la tabla. Ponlo al revés: los intervalos 3 y luego 1 se convierten
   en 1 y luego 3, Do Do♯ Mi. Menor 3 otra vez.

Ese es todo el algoritmo: la **forma normal** (el orden más cerrado),
movida a Do, comparada con su inversión, y gana la más compacta de las
dos. El navegador de conjuntos del capítulo anterior ya lo hace, así que
Ferrous Lineup encuentra el micromodo de tres notas pidiendo su forma
prima y buscándola en la tabla.

### Los mismos doce, tres veces

Kröpfl no fue el único en encontrar estos doce. En *The Structure of
Atonal Music* (1973), el teórico estadounidense **Allen Forte** listó
todas las clases de conjuntos, y sus doce tricordos, 3-1 a 3-12, son los
micromodos en el mismo orden. El compositor neerlandés **Peter Schat**
los encontró de nuevo y los ordenó como las doce horas de su **Tone
Clock** (*The Tone Clock*, 1993): la hora I es Menor 1, la hora III es
Menor 3, la hora XII la tríada aumentada.

Los tres no querían decir lo mismo con ellos, sin embargo. Marcela
Pavia, al comparar a Forte y Kröpfl, describe la clasificación de Forte
como matemática, y la de Kröpfl como construida sobre cómo *percibimos*
los grupos de notas, a la manera de los psicólogos de la Gestalt; ella
encuentra que las dos dan resultados muy distintos cuando las usas para
analizar música. Forte cuenta cada subconjunto de cada acorde; Kröpfl
escucha los grupos que la música te hace oír como una unidad. Ferrous
Lineup muestra ambas: el número de Forte junto a cada micromodo, y los
micromodos junto a cada conjunto de Forte.

## Los conjuntos más grandes están hechos de micromodos

Un conjunto de *n* notas contiene *n*(*n*−1)(*n*−2)/6 grupos de tres: 4
para un acorde de cuatro notas, 10 para una escala pentatónica, 35 para
una escala de siete notas. Contar qué micromodos son da el **contenido
de micromodos** del conjunto, una huella dactilar de sus colores.
Algunos ejemplos, todos comprobados por las pruebas:

- **La séptima de dominante**, Do Mi Sol Si♭: Do Mi Sol es una tríada mayor
  (Acorde menor-mayor), Mi Sol Si♭ una tríada disminuida, Do Mi Si♭ es Mayor 3
  (el acorde sin su quinta) y Do Sol Si♭ es Mayor 2. Una de cada.
- **La escala pentatónica**, Do Re Mi Sol La (5-35): Mayor 2 cuatro veces,
  Mayor 4 tres veces, Acorde menor-mayor dos veces (Do Mi Sol y La Do Mi) y
  Mayor 1 una vez (Do Re Mi). Ningún semitono, así que ningún micromodo
  menor, y por eso la escala pentatónica suena tan suave.
- **La escala diatónica**, las siete notas de Do mayor (7-35): Mayor 2
  ×8, Acorde menor-mayor ×6 (las tres tríadas mayores y las tres
  menores), Mayor 4 ×5, Menor 2 ×4, Menor 4 ×4, Mayor 1 ×3, Menor 5 ×2,
  Mayor 3 ×2 y una tríada disminuida (Si Re Fa). Ningún Menor 1, Menor 3 ni
  tríada aumentada: no puedes tocar esos solo con las teclas blancas.
- **La escala de tonos enteros** (6-35): Mayor 1 ×6, Mayor 3 ×12 y la
  tríada aumentada dos veces. Nada más, porque todo intervalo en ella es
  par.
- **Do Do♯ Re♯ Mi** (4-3), del análisis de Jaureguiberry de *Tres amores* de
  Diana Rud (1986): dos Menor 2 y dos Menor 3. Rud construye sus acordes
  de cuatro notas de modo que cada compás conserva tres notas del compás
  anterior y añade una nueva, y los micromodos que hay dentro cambian
  poco a poco.
- **Do Do♯ Fa♯ Sol** (4-9), del análisis de Cetta de *Música '66*: cuatro
  grupos de tres, todos Menor 5. Kröpfl pone estos junto a Menor 3 en la
  pieza.

Así se enlazan las dos ventanas. Un micromodo *es* un conjunto de clases
de alturas, uno de los doce con tres notas, así que la ventana Sets
marca esos doce con el nombre de su micromodo, y un conjunto más grande
está *hecho de* micromodos, así que la ventana Sets los lista bajo sus
detalles. En la ventana Micromodes, las notas que seleccionas se
analizan de la misma manera.

## Una serie hecha de un solo micromodo: el Concierto de Webern

Compositores anteriores a Kröpfl habían oído lo mismo. La *Pieza para
piano*, Op. 11 n.º 1, de Arnold Schoenberg (1909), empieza con Si, Sol♯ y
Sol: Menor 3. Anton Webern fue más lejos. La serie de su *Concierto para
nueve instrumentos*, Op. 24 (1934), está hecha de cuatro grupos de tres:

```text
B Bb D | Eb G F# | Ab E F | C C# A
```

Cada grupo es Menor 3, y cada uno es una forma del primero:

| grupo | notas | intervalos | forma del primer grupo |
|-------|-------|-----------|-------------------------|
| 1 | Si Si♭ Re | baja 1, sube 4 | ella misma |
| 2 | Mi♭ Sol Fa♯ | sube 4, baja 1 | hacia atrás y de cabeza (RI) |
| 3 | La♭ Mi Fa | baja 4, sube 1 | hacia atrás (R) |
| 4 | Do Do♯ La | sube 1, baja 4 | de cabeza (I) |

Al principio del Concierto los cuatro grupos los tocan oboe, flauta,
trompeta y clarinete, uno tras otro, así que oyes el mismo color cuatro
veces en cuatro instrumentos. Schat construía armonías de la misma
manera, a partir de formas de una hora que juntas usan las doce notas;
en sus términos, la serie de Webern está hecha de la hora III.

Webern no pensaba en micromodos: la palabra llegó décadas después. El
Concierto es una pieza dodecafónica, y Webern construyó su serie a
partir de cuatro formas de un solo grupo. «Menor 3» y «3-3» son nombres
posteriores de ese grupo, a la manera en que lo describiría un teórico.
Cetta señala que Kröpfl, como los compositores del círculo de
Schoenberg, prefería los grupos de tres notas, y por eso los dos se
encuentran aquí.

La serie es ahora una de las series famosas de la ventana Row, como
**Webern's Concerto**, y la Library y **File > Demos > Micromodes**
tienen un estilo construido sobre ella, **Webern's trichords**: la serie
como melodía saltando por tres octavas, con cada grupo de tres como
acorde debajo. El ritmo es de Ferrous Lineup, no de Webern.

**Por qué este sí está permitido.** Solo se copia en Ferrous Lineup
música de dominio público. Webern murió en 1945, así que su música es de
dominio público donde el copyright dura 70 años tras la muerte del
autor, y solo se usan las doce notas de la serie, nunca su partitura.
Kröpfl murió en 2021 y sus alumnos están vivos, así que su música sigue
bajo copyright, y nada de ella está en la app: los siete estilos de
micromodos se escribieron para Ferrous Lineup.

## Música desde un solo micromodo

¿Cómo escribes música con un solo micromodo? Alejandro Martínez describe
el micromodo como «elemento mínimo de construcción melódico/armónica»,
el bloque de construcción más pequeño de la melodía y la armonía por
igual. Los acordes son formas del micromodo, y también lo son los grupos
de la melodía, así que todo el pasaje tiene un solo color. La pregunta
es qué forma viene después. Ferrous Lineup ofrece dos respuestas, en
`micromodal.rs`.

### Mover cada nota la misma distancia

La más simple es una **secuencia**: mueve las tres notas hacia arriba el
mismo intervalo cada vez. *Whole-tone haze* sube Mayor 1 un tono en cada
compás: Do Re Mi, Re Mi Fa♯, Mi Fa♯ Sol♯... No sale nunca de una escala de tonos
enteros, el sonido de *Voiles* de Debussy. *Augmented drift* desliza la
tríada aumentada un semitono hacia abajo cada vez; tras cuatro acordes,
el quinto tiene las notas del primero, porque solo hay cuatro tríadas
aumentadas.

### Mantener dos notas, mover una

La manera más suave es conservar **dos** notas de cada grupo y mover
solo la tercera, de modo que cada grupo crezca del anterior. Para las
tríadas, esto da un conjunto famoso de movimientos. Desde Do mayor
(Do Mi Sol):

| movimiento | conserva | mueve | da |
|------|-------|-------|-------|
| **P** (paralelo) | Do y Sol | Mi baja a Mi♭ | Do menor |
| **L** (intercambio de sensible) | Mi y Sol | Do baja a Si | Mi menor |
| **R** (relativo) | Do y Mi | Sol sube a La | La menor |

Estos son los tres movimientos de la **teoría neoriemanniana**, que los
teóricos usan para explicar los cambios de acordes cromáticos de Wagner
y Liszt. El demo *Triads, two notes kept* recorre este camino: Do menor,
Do mayor, Mi menor, Mi mayor, Do♯ menor, Re♭ mayor, Si♭ menor, Si♭ mayor...

El mismo recorrido funciona para todos los micromodos, con una sorpresa.
Tres de ellos no pueden salir de un rincón pequeño de las doce notas:

- **Mayor 1** (Do Re Mi) y **Mayor 3** (Do Re Fa♯) solo tienen intervalos
  pares: 2, 4 y 6 semitonos. Una forma que conserva dos de sus notas
  debe añadir una tercera a una distancia par de ellas, así que se queda
  en la misma **escala de tonos enteros**. El recorrido alcanza 6 notas,
  nunca 12.
- La **tríada disminuida** solo tiene intervalos de 3 y 6 semitonos, así
  que se queda en un **acorde de séptima disminuida**: 4 notas.
- La **tríada aumentada** solo tiene intervalos de 4 semitonos. Dos de
  sus notas fijan la tercera, así que ninguna otra forma comparte dos
  notas con ella. Su recorrido la mueve un semitono hacia arriba en su
  lugar.

Todos los demás micromodos tienen un intervalo impar en algún sitio, y
su recorrido pasa por las doce notas. Las pruebas comprueban cada uno de
estos números.

### Qué forma conservar

Varias formas comparten dos notas con la actual (para Menor 3 en Do Do♯ Mi
hay tres: La Do Do♯, Do Re♯ Mi y Do♯ Mi Fa), así que el recorrido necesita una
regla para elegir una. La regla de Ferrous Lineup, en orden:

1. la forma cuya **nota nueva** lleva más tiempo sin oírse, así que la
   música pasa por todas las notas que puede alcanzar,
2. luego la propia forma usada menos recientemente, así que un recorrido
   encerrado en unas pocas notas (Mayor 1, la tríada disminuida) pasa
   igualmente por todas sus formas en lugar de balancearse entre dos,
3. luego el movimiento más pequeño de las notas que cambian,
4. y por último los bits del conjunto, para que la elección sea siempre
   la misma.

### Texturas

Un recorrido es una lista de formas. Una **textura** dice cómo toca las
formas un instrumento: como **acordes** (las notas compartidas se quedan
donde están y la nota que cambia se mueve al lugar más cercano,
conducción de voces suave), como acordes golpeados una y otra vez
(**pulso**), como una **línea** (la nota nueva primero, luego las
otras), como un **arpegio** (arriba y abajo), o como un **bajo** (la
raíz de una tríada, o si no la primera nota de la forma normal). Cada
instrumento de un estilo toca el mismo recorrido, cada uno en su propia
textura, así que el bajo, los acordes y la melodía comparten siempre sus
tres notas.

Una línea toca primero la nota que acaba de llegar, así que oyes lo que
cambió, y luego las dos que se quedaron. `write_forms` divide las notas
de la forma en dos con `partition`, que envía cada elemento a una de dos
colecciones según una closure de sí-o-no, en una sola pasada:

```rust
let (new, kept): (Vec<PitchClass>, Vec<PitchClass>) =
    normal.into_iter().partition(|p| !old.contains(*p));
new.into_iter().chain(kept).collect()
```

`old` es la forma anterior; las notas que no están en ella son nuevas.
`chain` vuelve a unir las dos listas, las notas nuevas primero.

Los siete estilos:

| estilo | micromodo | recorrido | sonido |
|-------|-----------|------|-------|
| Menor 3 colour | Menor 3 | mantiene dos notas | acordes, melodía y bajo en un solo color |
| Viennese points | Menor 5 | mantiene dos notas | notas sueltas saltando por tres octavas sobre acordes suaves |
| Cluster pulse | Menor 1 | mantiene dos notas | clusters que suben un semitono cada vez, sobre un pulso techno |
| Whole-tone haze | Mayor 1 | sube un tono | arpegios en 3/4 |
| Fourths groove | Mayor 4 | mantiene dos notas | acordes sus sobre un pulso neo-soul |
| Triads, two notes kept | menor-mayor | mantiene dos notas | movimientos P, L y R |
| Augmented drift | aumentado | baja un semitono | cuatro tríadas aumentadas, una y otra vez |

Un demo toca una pasada desde el micromodo en Do, luego al revés, luego
desde un tritono de distancia (Fa♯), luego al revés desde ahí, y la
primera pasada otra vez. Haz clic en un estilo de la Library mientras la
canción suena y toda la canción se pasa a él: sus acordes y su bajo se
reemplazan por el recorrido, los patrones empiezan desde esos cuatro
lugares por turno, y la tonalidad se convierte en el propio micromodo,
un conjunto de clases de alturas, así que las harmony keys tocan sus
formas.

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
    // ... eleven more
];
```

`prime(0b0000_0000_0111)` es el conjunto {Do, Do♯, Re} escrito como los
bits de un `u16`, un bit por nota, como en el capítulo anterior (Do es el
bit más bajo). `prime` es un `const fn`, así que toda la tabla se
construye en tiempo de compilación y no cuesta nada cuando arranca el
programa. Como cada campo es `Copy` (números, enums, `&'static str` y
`PitchSet`, que es un `u16`), `Micromode` deriva `Copy` también, pero el
resto del código pasa `&'static Micromode` de un lado a otro: una
referencia dentro de la tabla, que nunca puede colgar porque la tabla
vive tanto como el programa.

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

Una tabla escrita a mano puede tener erratas. Las pruebas la comprueban
desde tres direcciones, ninguna de las cuales se fía de la tabla:

- **Contra Forte.** Las clases de conjuntos del capítulo anterior,
  filtradas a tres notas, deben ser las doce formas primas en el mismo
  orden, nombradas 3-1 a 3-12.
- **Contra el conteo.** Un loop sobre los 4096 conjuntos (cada `u16` de
  0 a 4095) comprueba que exactamente los conjuntos de tres notas tienen
  un micromodo, que hay 220 de ellos, y que las formas de cada micromodo
  coinciden con la frecuencia con la que se encontró.
- **Contra las reglas.** Las familias se vuelven a deducir de los
  intervalos, con las reglas de Jaureguiberry, mediante una función que
  solo las pruebas pueden ver:

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

`#[cfg(test)]` en una sola función la compila solo para `cargo test`.
Sin él, `cargo clippy` avisa de que la función nunca se usa, porque solo
las pruebas la llaman. El solfeo también se comprueba: cada sílaba se
convierte de nuevo en un número (`"do#"` es 1, `"mib"` es 3) y las notas
deben ser la forma prima.

El vector de intervalos se lee en esa función como `vector[0]` (segundas
menores), `vector[1]` (segundas mayores) y así; el mismo vector también
da los intervalos en palabras, para la ventana:

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
        match count {
            0 => {}
            1 => parts.push(format!("a {one}")),
            2 => parts.push(format!("two {many}")),
            _ => parts.push(format!("three {many}")),
        }
    }
    match parts.as_slice() {
        [one] => one.clone(),
        [first @ .., last] => format!("{} and {last}", first.join(", ")),
        [] => String::new(),
    }
}
```

El último `match` es un **slice pattern** con un binding. `[one]`
coincide con un slice de exactamente un elemento. `[first @ .., last]`
coincide con cualquier slice de dos o más: `..` representa «el resto», y
`first @` da un nombre a ese resto, así que `first` es un slice de todo
salvo el último elemento. Entonces `["a minor second", "a minor third",
"a major third"]` se convierte en «a minor second, a minor third and a
major third», con el «and» en el lugar correcto y sin aritmética de
índices que pueda salir mal.

Los plurales son un par de strings en lugar de `format!("{name}s")`,
porque los plurales en inglés no siempre son una «s» añadida: «two
perfect fourths or fifths», no «two perfect fourth or fifths».

### Contar grupos de tres

`trichords` lista cada grupo de tres notas de un conjunto, con tres
loops anidados cuyos índices siempre suben, así que ningún grupo se
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
array de tres clases de alturas en un `PitchSet`: `PitchSet` implementa
`FromIterator<PitchClass>`, así que `collect` sabe cómo construir uno, y
el tipo de retorno `Vec<PitchSet>` le dice qué colección construir.

`micromode_content` los cuenta en un `[u16; 12]`, un contador por
micromodo. Para mostrarlos, `micromodes_in` conserva los encontrados y
los ordena, primero el más común:

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

`sort_by_key` ordena de la clave más pequeña hacia arriba, y aquí
queremos la cuenta *más grande* primero. `std::cmp::Reverse` envuelve un
valor y lo compara al revés, así que `Reverse(8)` va antes que
`Reverse(6)`. La clave es una **tupla**, y las tuplas se comparan
elemento a elemento: primero las cuentas (invertidas), y solo cuando dos
cuentas son iguales, el número del micromodo. Así que las cuentas
iguales conservan el orden de la tabla, y la lista es la misma cada vez
que se dibuja.

## El recorrido: elegir con una tupla

La regla del recorrido, «la nota nueva más fresca, luego la forma usada
menos recientemente, luego el movimiento más pequeño», es un solo
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
        let moves: u8 = /* ... */;
        // Then the form itself heard least recently, so a walk shut in
        // a few notes still goes round all its forms.
        let used = before.iter().rposition(|&f| f == set).map_or(0, |i| i + 1);
        (freshest, used, moves, set.bits())
    })
    .unwrap_or(current)
```

Unas cuantas cosas que notar:

- `members` es cada forma del micromodo (24, 12 o 4 de ellas).
  `shared(set)` cuenta las notas que una forma tiene en común con la
  actual, con los bits: `set.bits() & current.bits()` conserva las notas
  que están en ambas. `set.bits() & !current.bits()` conserva las notas
  de `set` que *no* están en `current`: la nota nueva.
- `heard` es un array de doce `Option<usize>`, uno por clase de alturas:
  `None` si la nota aún no se ha tocado, o `Some(i)` si se tocó por
  última vez en la forma número `i`. `map_or(0, |i| i + 1)` convierte
  «nunca» en 0, la clave más pequeña, así que una nota nunca oída
  siempre gana, y una nota oída hace mucho vence a una oída hace un
  momento.
- `rposition` busca en un slice **desde el final** y da el índice del
  último elemento que coincide: cuándo se usó la forma por última vez.
  Otra vez «nunca» se convierte en 0.
- Como la tupla compara elemento a elemento, el orden de los cuatro
  elementos *es* el orden de las reglas. Cambiar las reglas significa
  reordenar una tupla, no reescribir una cadena de `if`s.

¿Cuántas formas necesita una pasada? Un estilo dice cuántos pasos dura
cada forma, y el patrón tiene una longitud en pasos; la cuenta redondea
hacia arriba, así que una última forma que solo cabe a medias se escribe
igualmente:

```rust
let count = length.div_ceil(self.steps.max(1)).max(1);
```

`div_ceil` es una división que redondea hacia arriba: `16.div_ceil(6)`
es 3, mientras que `16 / 6` es 2.

## Los estilos se apoderan de la armonía

Un estilo de micromodo, como un estilo dodecafónico, no tiene acordes
que seguir. Cuando haces clic en él mientras la canción suena, tiene que
*reemplazar* la armonía de la canción: silenciar los acordes y el bajo
que no toca, y escribir sus propias voces en los canales correctos. Los
estilos de serie ya hacían esto, así que el código salió de
`use_row_style` a una función que llaman los dos tipos de estilo:

```rust
pub(super) fn take_over_harmony(
    project: &mut Project,
    uses: &dyn Fn(&str) -> bool,
    rhythm: Option<&Rhythm>,
    mut write: impl FnMut(&mut Project, usize, usize, Option<usize>),
) -> RhythmChange {
```

Las dos closures se pasan de dos maneras distintas, a propósito:

- `uses` solo responde una pregunta («¿toca el estilo este canal?»), así
  que es un `&dyn Fn`: un **trait object**, una referencia a alguna
  closure cuyo tipo se decide en tiempo de ejecución. Una sola copia
  compilada de `take_over_harmony` funciona para cada closure.
- `write` se llama una vez por patrón con `&mut Project`, y hace la
  escritura. Es un `impl FnMut`: el compilador hace una copia de
  `take_over_harmony` para cada tipo de closure que se pasa (para los
  estilos de serie y para los estilos de micromodos), lo que es un poco
  más rápido y permite a la closure cambiar lo que capturó. `FnMut` es
  el límite más laxo que aún se puede llamar muchas veces.

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
de la canción empieza desde el micromodo, el siguiente al revés, el
siguiente a un tritono de distancia, y así: la canción no se repite.
Nota que la closure recibe `project` como argumento en lugar de
capturarlo: `take_over_harmony` ya tiene `&mut Project`, y Rust solo
permite un préstamo mutable a la vez, así que el proyecto se presta a la
closure en cada llamada.

Los demos reutilizan el arreglo de los demos dodecafónicos. El código
que hacía las cinco secciones de un demo de serie se dividió en
`section_patterns` (un patrón por sección, rellenado por una closure) y
`arrange_sections` (el playlist, con la batería entrando tras la primera
sección), y `micromode_demo` usa ambos con su propia closure.

## La ventana: un reloj dibujado con el painter de egui

Las doce notas se sientan alrededor de un círculo como las horas de un
reloj, Do arriba, y las tres notas del micromodo se unen en un triángulo.
La forma *es* el micromodo: mover el grupo gira el triángulo alrededor
del reloj, y ponerlo al revés lo refleja. Por eso Schat llamaba horas a
los tricordos.

```rust
/// The point for pitch class `n` on a clock face: C at the top, going
/// round clockwise a semitone at a time.
fn on_clock(center: Pos2, radius: f32, n: u8) -> Pos2 {
    let angle = f32::from(n) / 12.0 * TAU;
    center + Vec2::new(angle.sin(), -angle.cos()) * radius
}
```

`TAU` es 2π, una vuelta completa, así que la clase de alturas `n` es `n`
doceavos de vuelta. En matemáticas, el ángulo 0 apunta a la derecha y
los ángulos van en sentido antihorario; aquí queremos 0 arriba, en
sentido horario. Usar `sin` para x y `-cos` para y hace ambas cosas: en
0, x es 0 e y es −1, que es *arriba* en una pantalla, porque la y de
pantalla crece hacia abajo.

`clock` pide entonces un cuadrado de la ventana con
`ui.allocate_exact_size`, toma un `painter_at` en ese rect (todo lo
dibujado fuera se recorta), y dibuja: el círculo con `circle_stroke`, el
triángulo con `Shape::convex_polygon` (relleno con el color de acento a
un cuarto de fuerza, `gamma_multiply(0.25)`), un punto para cada nota,
más grande para las tres del grupo, y los nombres alrededor por fuera.
Los botones de la lista, con un reloj diminuto y el nombre, se dibujan
de la misma manera: `allocate_exact_size` con `Sense::click()` da un
`Response` que sabe de hover y de clics, como un botón de verdad.

### Dos ventanas que se señalan la una a la otra

Ninguna ventana llama a la otra. Cada una devuelve una lista de
**acciones**, como hacen las otras ventanas del piano roll:

```rust
/// Something done in the micromode window.
pub enum MicroAction {
    /// Click the grid to place this group above the note clicked.
    Stamp(PitchSet),
    /// Make this group the key.
    UseAsKey(Scale),
    /// Play these notes now.
    Hear(Vec<Note>),
    /// Show this set in the set browser.
    ShowSet(PitchSet),
}
```

La ventana Sets recibió dos acciones nuevas propias, `ShowMicromode` y
`OpenMicromodes`. El piano roll es dueño de ambas ventanas, así que
maneja las acciones: `ShowSet` llama al `pick` de la ventana Sets y la
abre; `ShowMicromode` llama al `pick` de la ventana Micromodes. Cada
ventana solo sabe mostrar un conjunto que se le da, y el piano roll las
conecta.

`pick` muestra no solo el micromodo, sino la forma exacta en la que se
hizo clic. Si haces clic en Menor 3 bajo la serie de Webern en la
ventana Sets, la ventana Micromodes se abre en Si Si♭ Re, no en Do Do♯ Mi:

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

`moved_to`, del capítulo anterior, encuentra cómo mover la forma prima
hasta la forma: `n` semitonos arriba, al revés o no.

Una ventana abierta desde otra debe salir delante, o puede abrirse
*detrás* de la ventana en la que hiciste clic. `raise` es una bandera
para eso: `pick` la pone, y el siguiente frame la lee y la limpia de una
vez con `std::mem::take`, que devuelve el valor y deja un `false` atrás:

```rust
if std::mem::take(&mut self.raise) {
    ctx.move_to_top(egui::LayerId::new(egui::Order::Middle, window_id()));
}
```

La ventana Sets hace lo mismo con una bandera `reveal`, y además
desplaza su lista larga hasta la clase que le pidieron mostrar:
`response.scroll_to_me(Some(egui::Align::Center))` en la línea de esa
clase, solo durante un frame.

### Deletrear las tríadas como tríadas

Do, Mi♭ y Fa♯ y Do, Mi♭ y Sol♭ son las mismas tres teclas, pero solo una es una
tríada disminuida sobre el papel: Do Mi♭ Sol♭, una pila de dos terceras
menores. Los tres micromodos de acordes se deletrean como los acordes
que son. El trait `NoteNames`, que escribe notas ya sea con la grafía de
la tonalidad o con sostenidos o bemoles simples, recibió un segundo
método para esto:

```rust
pub trait NoteNames: Copy {
    fn note_name(self, pitch_class: PitchClass) -> NoteName;
    /// The notes of a chord, from its root up.
    fn chord_names(self, chord: Chord) -> Vec<NoteName>;
}
```

`Micromode::chord` encuentra la tríada de una forma (la raíz es la nota
de la forma normal desde la cual las otras dos forman un acorde
conocido), y la ventana escribe sus notas con `chord_names`. La tríada
aumentada no tiene raíz real (cada una de sus notas está a una tercera
mayor de la siguiente), así que se nombra desde la primera nota de su
forma normal.

## Fuentes

Kröpfl nunca publicó su teoría; como escribe Jaureguiberry, «esta
propuesta nunca fue publicada por su autor». Se conoce por sus alumnos y
por los análisis de su música. Estas son las fuentes usadas para este
capítulo y para los textos de la app:

- Pablo Cetta, «*La tercera es la vencida* de Francisco Kröpfl y Oscar
  Steimberg», *Cuadernos de Análisis y Debate sobre Músicas
  Latinoamericanas Contemporáneas* 1 (2018), pp. 8–36. La tabla de los
  doce micromodos (Ejemplo 1) con los números de Forte, las formas
  primas en solfeo, los ejemplos Mi Fa La♭ y Fa Re Sol♭, el análisis de *Música
  '66* (Menor 3 y Menor 5, el conjunto 4-9) y de la canción de la bruja
  en la ópera de cámara de Kröpfl (2011), construida principalmente
  sobre Menor 3. En línea en el repositorio de la Universidad Católica
  Argentina.
- Pablo Jaureguiberry, «*Tres amores* de Diana Rud: otras
  particularidades de los micromodos de Francisco Kröpfl» (Universidad
  Nacional de Rosario, 2016), pp. 499–510. La definición, las tres
  familias y sus reglas, Menor 3 como favorito de los alumnos de Kröpfl,
  y conjuntos de cuatro notas hechos de dos Menor 2 y dos Menor 3.
- Alejandro Martínez, «Algunos aspectos de los micromodos de Francisco
  Kröpfl y su contextualización histórica», en *Actas de la XXI
  Conferencia de la Asociación Argentina de Musicología* (2015),
  pp. 399–415. El micromodo como la unidad más pequeña de melodía y
  armonía, su uso por compositores argentinos de varias generaciones, y
  su lugar junto a los conjuntos de Forte, el análisis tricordal de
  Gilbert y el Tone Clock de Schat.
- Marcela Pavia, «La estructura de la música atonal: dos enfoques»,
  *CodeXXI* 1 (1998), pp. 199–214. La clasificación de Kröpfl,
  construida sobre la percepción, comparada con la de Forte, construida
  sobre las matemáticas.
- Allen Forte, *The Structure of Atonal Music* (Yale University Press,
  1973). Los tricordos 3-1 a 3-12.
- Peter Schat, *The Tone Clock* (Harwood Academic Publishers, 1993).
  Las doce horas.

Los ejemplos musicales de otros compositores solo se nombran, nunca se
copian, salvo las doce notas de la serie de Webern.

## Cómo se probó

- **La tabla**: los doce coinciden con los tricordos de Forte en orden;
  los 4096 conjuntos dan 220 grupos de tres, cada uno en exactamente un
  micromodo, con 12, 24 o 4 formas; las familias se siguen de los
  intervalos por las reglas de Jaureguiberry; el solfeo deletrea cada
  forma prima; los nombres se encuentran en español y en inglés, con o
  sin mayúsculas.
- **Los ejemplos de las fuentes**: Mi Fa La♭ y Fa Re Sol♭ de Cetta, Si Sol♯ Sol de
  Schoenberg y los cuatro grupos de la serie de Webern son Menor 3;
  Do Do♯ Re♯ Mi contiene dos Menor 2 y dos Menor 3; 4-9 contiene cuatro
  Menor 5; la séptima de dominante contiene uno de cada de Mayor 2,
  Mayor 3, la tríada disminuida y menor-mayor; la escala de tonos
  enteros contiene solo Mayor 1, Mayor 3 y la tríada aumentada.
- **El recorrido**: cada paso conserva dos notas; alcanza las doce
  notas, o 6 para Mayor 1 y Mayor 3, o 4 para la tríada disminuida; los
  movimientos de las tríadas son P, L o R; una secuencia de Mayor 1 se
  queda en una escala de tonos enteros.
- **La escritura**: los acordes mantienen en su sitio las notas
  compartidas; cada textura escribe notas de las formas correctas y
  nunca más allá del final del patrón; el bajo toca raíces.
- **Estilos y demos**: cada estilo nombra un micromodo y un ritmo que
  existen, escribe en cualquier tonalidad, se apodera de la armonía de
  una canción y hace un demo con cinco secciones.
- **En la app en marcha**: la ventana y sus tres familias, el reloj, la
  grafía de los acordes (Do Mi♭ Sol♭), *Hear it* (comprobado grabando el
  sonido y encontrando sus alturas), los micromodos de la selección, los
  enlaces entre las dos ventanas en ambas direcciones y por delante, los
  estilos de la Library tanto parados como con la canción sonando, y los
  demos, de nuevo con sus alturas comprobadas desde el sonido.

## Pruébalo

1. Abre el piano roll (**F7**) en el patrón **Chords** de la canción
   inicial, haz clic en la barra de título del piano roll, pulsa
   **Ctrl+A** y haz clic en **Micromodes…**. Lee los colores de Do mayor:
   ocho Mayor 2, seis tríadas, una disminuida.
2. Haz clic en **Menor 3** y luego en **▶ Hear it**. Prueba
   **Inverted (I)** y **Move it up**, y mira cómo el triángulo gira y se
   voltea en el reloj.
3. Haz clic en **3-3 in Sets →**. La ventana Sets se abre en 3-3, al que
   llama Menor 3 en su lista. Elige **7-35** y haz clic en uno de los
   micromodos que hay debajo para volver.
4. Abre **File > Demos > Micromodes > Triads, two notes kept** y pulsa
   Space. Escucha la única nota que se mueve en cada acorde.
5. Haz clic en **Webern's trichords** bajo **Twelve-tone rows** en la
   Library. La tonalidad se convierte en la serie de Webern, y con
   **3 notes** elegido bajo la tonalidad, las harmony keys tocan sus
   cuatro grupos de tres: cada uno está etiquetado 3-3. Ahora cambia el
   menú de tonalidad de **twelve-tone row** a **pitch-class set**: la
   tonalidad se convierte en el primer grupo de la serie, Do Si Mi♭, y las
   harmony keys tocan sus formas, todas 3-3. (Antes de arreglarse esto,
   el cambio daba el acorde de séptima disminuida, el conjunto por
   defecto del menú, y las teclas mostraban 3-10: las harmony keys
   siempre tocan acordes de la *tonalidad*, no de las notas del patrón.
   La función `set_key` de `twelve_tone.rs` ahora toma en su lugar el
   primer grupo de tres de una serie.)
6. De vuelta en la canción inicial, mientras suena, haz clic en
   **Fourths groove** en la Library: la canción sigue en acordes sus.

## Ejercicios

1. **Los micromodos de una melodía (lineup-core, pequeño).** Escribe
   `melody_micromodes(notes: &[PitchClass]) -> Vec<Option<&'static
   Micromode>>`, que nombra el micromodo de cada tres notas seguidas:
   notas 1–3, 2–4, 3–5 y así (busca `windows` en el glosario). ¿Por qué
   la respuesta es `None` para algunas ventanas? Pruébalo con la serie
   de Webern: ¿qué ventanas, además de los cuatro grupos, son Menor 3?
2. **Todos los colores a la vez (lineup-core, pequeño).** Haz un loop
   sobre `SetClass::all()` y encuentra los conjuntos más pequeños cuyo
   contenido de micromodos no tiene ceros: conjuntos que contienen los
   doce micromodos. ¿Cuántas notas necesitan, y cuáles son? (Una pista
   para el tamaño: la tríada aumentada y Menor 1 deben caber ambas.)
3. **Las tonalidades de Schat (lineup-core, mediano).** La serie de
   Webern divide las doce notas en cuatro formas de Menor 3. Escribe una
   búsqueda con backtracking, como `random_all_interval` del capítulo
   anterior, que encuentre todas las maneras de dividir las doce notas
   en cuatro formas de un micromodo, y cuéntalas para cada uno de los
   doce. Un micromodo no puede hacerlo en absoluto: ¿cuál, y por qué?
   (Piensa en qué notas pueden tomar las formas de cada micromodo de
   acordes.)
4. **Un recorrido que recuerda más (lineup-core, mediano).** El
   recorrido de notas comunes solo recuerda cuándo se usó por última vez
   cada nota y cada forma. Haz un recorrido que no use una forma otra
   vez hasta haber usado todas las formas que puede alcanzar, y prueba
   que Menor 3 visita las 24 formas en 24 pasos. ¿Es eso siempre
   posible? (¿Para qué micromodos puede un recorrido de «mantener dos
   notas» visitar cada forma exactamente una vez? Es una pregunta sobre
   caminos en un grafo.)
5. **Oír la familia (lineup-app, pequeño).** Añade un botón **▶** junto
   al nombre de cada familia en la ventana Micromodes que toque sus
   micromodos uno tras otro, desde el Do central. `MicroAction::Hear`
   toca notas juntas; ¿qué añadirías para tocarlas por turno? Mira cómo
   se detiene la vista previa de un acorde del piano roll.
