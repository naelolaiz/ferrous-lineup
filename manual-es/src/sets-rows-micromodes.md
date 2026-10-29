# Más allá de las tonalidades: conjuntos, series y micromodos

Todo en los capítulos anteriores suponía una tonalidad: una nota de
casa, y acordes que tiran hacia ella. Mucha de la música del siglo XX no
tiene tonalidad, y los nombres de los acordes no sirven para sus
armonías. Compositores y teóricos encontraron otras maneras de pensarla,
y Ferrous Lineup tiene tres:

- los **conjuntos de clases de alturas**, un nombre para cualquier grupo
  de notas, sea acorde o no;
- las **series dodecafónicas**, las doce notas en un orden del que está
  hecha toda una obra;
- los **micromodos**, los doce colores de tres notas que enseñaba en
  Buenos Aires el compositor Francisco Kröpfl.

Cada uno tiene una ventana en la barra del piano roll (**Serie…**,
**Conjuntos…**, **Micromodos…**), y cada uno puede ser la tonalidad de
una canción.

## Conjuntos de clases de alturas

**Clases de alturas.** Numera las doce notas desde Do: Do es 0, Do♯ es
1, y así hasta Si, 11. Las octavas y la escritura no cuentan: todo Do es
0, y Do♯ y Re♭ son los dos 1. Un **conjunto** es una elección de algunas
de las doce.

**Mover un conjunto.** Subir cada nota *n* semitonos es **T*n***: Do Mi
Sol subido 3 es Mi♭ Sol Si♭. Dar vuelta cada intervalo es **invertir**:
Do Mi Sol, cuatro semitonos hacia arriba y luego tres, pasa a Do La♭
Fa, cuatro hacia abajo y luego tres. Eso es Fa menor: dada vuelta, una
tríada mayor es una tríada menor.

**Clases de conjuntos.** Un conjunto con todas sus transposiciones e
inversiones es una **clase de conjuntos**. Las 24 tríadas mayores y
menores son una sola clase. Los 4096 conjuntos posibles caen en 224
clases, y cada una tiene un **número de Forte**, de la lista que hizo el
teórico Allen Forte en 1973: **3-11** es la clase de las tríadas mayores
y menores (3 notas, la 11.ª clase de 3 notas). Una clase también se
escribe por su **forma prima**, su forma más compacta llevada a empezar
en 0: **[037]** para 3-11, Do Mi♭ Sol.

| Conjunto | Número de Forte | Forma prima |
|---|---|---|
| una tríada mayor o menor | 3-11 | [037] |
| una tríada disminuida | 3-10 | [036] |
| una tríada aumentada | 3-12 | [048] |
| un acorde de séptima de dominante o semidisminuido | 4-27 | [0258] |
| un acorde de séptima disminuida | 4-28 | [0369] |
| la escala pentatónica | 5-35 | [02479] |
| la escala de tonos enteros | 6-35 | [02468T] |
| la escala mayor, y sus modos | 7-35 | [013568T] |
| la escala octatónica | 8-28 | [0134679T] |

(T vale 10, para que cada número sea un solo carácter.)

El **vector interválico** cuenta los intervalos entre cada par de notas
de un conjunto: cuántas segundas menores, segundas mayores, terceras
menores, terceras mayores, cuartas y tritonos. Una quinta cuenta como
cuarta, porque es una cuarta dada vuelta. Para la tríada 3-11 es
**<001110>**: una tercera menor, una tercera mayor, una cuarta. Es un
buen resumen de cómo suena un conjunto.

### En la aplicación

El piano roll nombra el conjunto de las notas bajo el puntero, o de las
notas que seleccionas, en la línea debajo de los botones de acordes:

> Conjunto: 3-11 · forma normal Do Mi Sol · forma prima [037] ·
> intervalos <001110> · tríada mayor o menor · micromodo Acorde
> menor-mayor

**Conjuntos…** abre una lista de todas las clases del tamaño que elijas,
con su número, su forma prima, su vector interválico y, para las
famosas, un nombre. Búscalas por número o por nombre («4-Z15»,
«octatónica»), o haz clic en **Mostrarlo** para la clase de las notas
seleccionadas. Elige una, y **Transportarlo** e **Invertido (I)** eligen
cuál de sus formas: **T3** es el conjunto tres semitonos más arriba,
**T3I** su inversión subida tres. **▶ Escucharlo** lo toca, **Dibujar
con él** pone el conjunto entero con cada clic en el piano roll, y
**Usar como tonalidad** lo hace la tonalidad de la canción. En la hoja
de acordes, un conjunto se escribe por su nombre o por sus notas:
`3-11 T2`, `{Re Fa La}` o `{0,1,4}`.

## Una armonía, cuatro nombres

Un conjunto es otra manera de **nombrar** notas, no otras notas. El
acorde de Do de Do mayor es Do, Mi y Sol se lo nombre como se lo
nombre. El menú **Mostrar** de la hoja de acordes escribe la misma
armonía de cuatro maneras, sin cambiar una nota: **Como está escrito**,
como **Acordes**, como **Conjuntos de clases de alturas** o como
**Micromodos**.

![La hoja de acordes de la demostración Cadencia clásica con Mostrar en Conjuntos de clases de alturas: Do, Fa, Sol7, Do, Lam, Rem, Sol7, Do escritos como 3-11 T7I (Do Mi Sol), 3-11 T0I (Fa La Do), 4-27 T7I (Si Re Fa Sol), 3-11 T7I, 3-11 T9 (La Do Mi), 3-11 T2 (Re Fa La), 4-27 T7I y 3-11 T7I, cada uno con sus notas debajo](images/sets-view.png)

| Como está escrito | Acordes | Conjuntos de clases de alturas | Micromodos |
|---|---|---|---|
| Do | Do | 3-11 T7I | Acorde menor-mayor T7I |
| Rem | Rem | 3-11 T2 | Acorde menor-mayor T2 |
| Sol7 | Sol7 | 4-27 T7I | 4-27 T7I |
| 3-11 T2 | Rem | 3-11 T2 | Acorde menor-mayor T2 |
| Menor 3 | Menor 3 | 3-3 | Menor 3 |

Cómo leerlos:

- **T*n*** es la forma prima subida *n* semitonos: la forma prima de
  3-11 es Do Mi♭ Sol, una tríada de Do menor, así que **3-11 T2** es Re
  Fa La, Re menor.
- **T*n*I** es la forma prima dada vuelta primero y subida después: Do
  Mi♭ Sol dado vuelta es Do La Fa, y subido 7 es Sol Mi Do. Así que
  toda tríada mayor tiene una I en su nombre, y ninguna tríada menor la
  tiene.
- Un micromodo tiene siempre tres notas, así que Sol7, con cuatro,
  conserva su nombre de conjunto en la vista Micromodos.
- **Acordes** nombra un conjunto como acorde cuando sus notas forman
  uno, y conserva el nombre del conjunto cuando no.

Para conjuntos y micromodos, un segundo menú elige qué notas forman cada
grupo: **Las armonías de la hoja**, **Notas a la vez (vertical)** o
**Notas en línea (serial)**, con cuántas notas tiene cada grupo y qué
pista leer. Eso lee las notas de la canción, así que puedes mirar una
melodía como una cadena de grupos de tres notas.

La tonalidad funciona igual. Elige **conjunto de clases de alturas** en
el menú **Tonalidad** de una canción en Do mayor, y nada se mueve: la
tonalidad se llama ahora 7-35, y las teclas de armonía muestran los
mismos siete acordes con sus clases de conjuntos, seis de ellos 3-11 y
Si Re Fa 3-10. Las notas cambian solo cuando eliges otro conjunto.

## Series dodecafónicas

Hacia 1921, Arnold Schoenberg buscó una manera de escribir música en la
que ninguna nota fuera la casa. Su respuesta fue usar las doce notas en
un orden fijo, elegido para cada obra: la **serie**. Ninguna nota vuelve
hasta que hayan sonado las otras once, así que ninguna se vuelve el
centro. Esta es la serie de su Suite para piano op. 25 (1925):

Mi Fa Sol Do♯ Fa♯ Re♯ Sol♯ Re Si Do La La♯

Una serie viene en cuatro formas:

| Forma | Cómo | La serie de Schoenberg |
|---|---|---|
| **P**, original | como está escrita | Mi Fa Sol Do♯ Fa♯ Re♯ Sol♯ Re Si Do La La♯ |
| **I**, inversión | cada intervalo dado vuelta | Mi Re♯ Do♯ Sol Re Fa Do Fa♯ La Sol♯ Si La♯ |
| **R**, retrogradación | de atrás hacia adelante | La♯ La Do Si Re Sol♯ Re♯ Fa♯ Do♯ Sol Fa Mi |
| **RI**, retrogradación de la inversión | la inversión de atrás hacia adelante | La♯ Si Sol♯ La Fa♯ Do Fa Re Sol Do♯ Re♯ Mi |

Cada una puede empezar en cualquiera de las doce notas, lo que da 48
formas. Se nombran con una letra y el número de la nota en que empiezan,
con Do como 0: la serie de Schoenberg es **P4**, porque empieza en Mi.
Una forma R toma el número de la forma que da vuelta, así que **R4**
termina en Mi. (Algunos libros numeran las formas desde la primera nota
de la serie, para que la serie como está escrita sea siempre P0. Las
formas son las mismas; solo cambian sus números.)

### La ventana Serie dodecafónica

**Serie…**, en el piano roll, la abre. Escribe una serie en el cuadro de
arriba, con nombres de notas o con números, y haz clic en **Usar**; o
dibuja doce notas en el piano roll y haz clic en **Desde las notas**; o
toma una de **🎲 Al azar**, **🎲 Todos los intervalos** o **Series
famosas y de ejemplo**: la Suite de Schoenberg, la Suite lírica de Berg,
la Sinfonía y el Concierto de Webern, y cuatro series escritas para
Ferrous Lineup.

![La ventana Serie dodecafónica con la serie del Concierto op. 24 de Webern sobre Do: Do Si Mib Mi Lab Sol La Fa Fa# Do# Re Sib, con su matriz de 48 formas, P0 elegida, y debajo «Pasos hacia arriba, en semitonos: 11 4 1 4 11 2 8 1 7 1 8», qué formas son combinatorias con ella, y sus tricordios, los cuatro 3-3 (Menor 3)](images/row-window.png)

La **matriz** tiene las 48 formas en un cuadrado. Cada fila, leída de
izquierda a derecha, es una forma P, y de derecha a izquierda una forma
R; cada columna, leída hacia abajo, es una forma I, y hacia arriba una
forma RI. Las etiquetas alrededor del cuadrado dicen cuál: haz clic en
una para elegir esa forma. La diagonal desde arriba a la izquierda es la
misma nota de punta a punta, una manera rápida de ver que el cuadrado
está bien.

Debajo de la matriz, la ventana dice de qué está hecha la serie: sus
pasos en semitonos, sus grupos de tres notas, y qué formas son
**combinatorias** con ella, cuyas seis primeras notas son las seis que
la serie deja fuera, así que las dos tocadas juntas dan las doce notas
en cada mitad.

Luego escribe una forma en el patrón: **Juntas** dice cuántas de sus
notas suenan a la vez, como acorde; **Después**, qué viene después de la
forma, **Una vez**, **Repetir** o **P, R, I, RI**; el registro,
**Cerrado** (todo en una octava), **Suave** (cada nota cerca de la
anterior) o **Amplio** (saltando por tres octavas, como en Webern).
**Serialismo** va más allá, como Messiaen, Boulez y Babbitt hacia 1950:
deja que la serie elija también las **Duraciones**, las **Dinámicas** y
las **Octavas**. **Escribir P0 en el patrón actual** reemplaza las notas
del canal, y **Guardar en un patrón nuevo** las pone en un patrón
propio.

### Una serie como tonalidad

Elige **serie dodecafónica** en el menú **Tonalidad**, o haz clic en
**Usar como tonalidad** en la ventana Serie dodecafónica para la forma
que elegiste, y la serie pasa a ser la tonalidad. El piano roll numera
sus notas del 1 al 12 en el teclado, en el orden de la serie, así que
puedes dibujar la serie siguiendo los números. Las teclas de armonía
tocan los grupos de la serie en lugar de acordes: **3 notas** da cuatro
teclas, 1–3, 4–6, 7–9 y 10–12; **4 notas**, tres, y **6 notas**, dos.

Con **Conducción de voces** marcada, el piano roll revisa las reglas de
la escritura dodecafónica tal como las aprendían los alumnos de
Schoenberg: las notas siguen formas de la serie, cada una en su orden;
notas vecinas en la serie pueden sonar juntas como acorde, y una nota
puede repetirse enseguida, pero vuelve solo después de las otras once;
una forma se toca entera; ninguna nota se duplica en octavas; y ningún
acorde suena como armonía tonal, como una tríada mayor o menor o una
séptima de dominante. Una línea **Serie** debajo de los nombres de los
acordes muestra las formas que lee en las notas.

El grupo **Series dodecafónicas** de la Biblioteca tiene siete estilos,
y **Archivo > Demostraciones > Dodecafónico** toca cada uno como una
pieza corta.

## Micromodos

**Francisco Kröpfl** (1931–2021) fue un compositor argentino, pionero de
la música electrónica que fundó el primer estudio universitario para
ella en América Latina, en Buenos Aires en 1958, y profesor de
composición durante décadas. Enseñaba a sus alumnos a oír los grupos de
tres notas por su color. Tres notas cualesquiera tienen tres intervalos
entre sí, y esos intervalos le dan al grupo su sonido, en cualquier
orden, octava o escritura, transportado o dado vuelta. Con esas reglas,
que son las de los conjuntos de clases de alturas, hay exactamente doce
grupos de tres. Kröpfl los llamó **micromodos**:

| Micromodo | Forma prima | Forte |
|---|---|---|
| **Menor 1** | do - do♯ - re | 3-1 |
| **Menor 2** | do - do♯ - re♯ | 3-2 |
| **Menor 3** | do - do♯ - mi | 3-3 |
| **Menor 4** | do - do♯ - fa | 3-4 |
| **Menor 5** | do - do♯ - fa♯ | 3-5 |
| **Mayor 1** | do - re - mi | 3-6 |
| **Mayor 2** | do - re - fa | 3-7 |
| **Mayor 3** | do - re - fa♯ | 3-8 |
| **Mayor 4** | do - re - sol | 3-9 |
| **Acorde disminuido** | do - mi♭ - sol♭ | 3-10 |
| **Acorde menor-mayor** | do - mi♭ - sol | 3-11 |
| **Acorde aumentado** | do - mi - sol♯ | 3-12 |

Vienen en tres familias. Los **menores** tienen todos una segunda
menor, un semitono; los **mayores** tienen una segunda mayor y ningún
semitono; y el resto son las tres tríadas, que llevan su nombre. La
tríada menor y la mayor son un mismo micromodo, **Acorde menor-mayor**,
porque cada una es la otra dada vuelta: todos los acordes de un I, IV,
V, I tienen el mismo color, y la música tonal trabaja con *cuáles* de
sus formas usa.

**Micromodos…**, en el piano roll, muestra las tres familias, cada
micromodo dibujado en un reloj de las doce notas, con su forma prima,
sus intervalos, su conjunto y dónde se oye:

![La ventana Micromodos: las tres familias, Menores (Menor 1 a 5), Mayores (Mayor 1 a 4) y Acordes (Acorde disminuido, Acorde menor-mayor, Acorde aumentado), y Menor 3 dibujado en un reloj como un triángulo Do, Reb, Mi, con su forma prima do - do# - mi, sus intervalos, el conjunto 3-3 [014] con un botón a la ventana Conjuntos, y dónde se oye](images/micromodes.png)

Selecciona algunas notas en el piano roll, y la ventana lista los
micromodos que hay dentro: las siete notas de Do mayor contienen 35
grupos de tres. **Transportarlo** e **Invertido (I)** eligen una forma;
**▶ Escucharlo**, **Dibujar con él** y **Usar como tonalidad** funcionan
como en la ventana Conjuntos, y el botón del conjunto abre la ventana
Conjuntos en la misma clase. En la hoja de acordes, escribe un micromodo
por su nombre: `Menor 3`, o `Menor 3 T4I` para una de sus formas.

Los cuatro grupos de tres de la serie del Concierto op. 24 de Webern, en
la imagen de la ventana Serie dodecafónica, son todos Menor 3. El grupo
**Micromodos (Kröpfl)** de la Biblioteca tiene siete estilos, y
**Archivo > Demostraciones > Micromodos** toca cada uno.
