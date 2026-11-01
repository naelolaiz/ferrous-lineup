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

### Un conjunto como tonalidad: sus formas

Cuando la tonalidad de la canción es un conjunto de clases de alturas
(después de **Usar como tonalidad**, o de elegir **conjunto de clases de
alturas** en el menú Tonalidad), las teclas de armonía pueden tocar dos
cosas, que se eligen a la izquierda:

- **Acordes**: acordes apilados con las notas del conjunto, de tres o
  cuatro notas, cada uno llamado por el conjunto que forma, como lo
  escribe la hoja de acordes: **3-11 T7I** es Do Mi Sol.
- **Formas**: las **formas** del propio conjunto, el conjunto entero
  transportado o dado vuelta, cada una nombrada por su **grado** (T0,
  T2, T2I…), como en un micromodo (ver [Un micromodo como
  tonalidad](#un-micromodo-como-tonalidad)). Un conjunto de tres notas
  muestra siempre sus formas.

Las formas empiezan por casa y siguen según cuántas notas conservan de
casa, más brillantes cuantas más conservan. Se cuentan notas, no
intervalos: cada forma es el mismo conjunto transportado o dado vuelta,
así que todas tienen los mismos intervalos, y lo que las distingue son
las notas que comparten con casa, en cualquier octava. Las formas que
conservan la misma cantidad van por grado, primero las derechas, en
orden ascendente, y después las dadas vuelta. En una tonalidad de Do
que es el conjunto Sol La♭ La Si Do Mi♭ Mi (7-21), T4, T8, T3I, T7I y
T11I conservan seis notas; T0I, T4I y T8I, cinco; T1, T3, T5, T7, T9,
T11 y T6I, cuatro. Cada tecla muestra las notas de la forma en su orden
más compacto y, debajo, el conjunto, 4-27 o 6-20, el mismo en todas
las teclas. Las teclas se ensanchan cuando
las notas lo necesitan, así que seis notas en solfeo se escriben
enteras. Apunta a una tecla para leer el nombre de la forma como lo
escribe la hoja de acordes, «4-27 T2»: ese nombre cuenta desde la forma
prima del conjunto sobre Do, y el grado de la tecla cuenta desde la
tonalidad, así que los dos números difieren salvo que la tonalidad sea
la forma prima sobre Do. Un conjunto es música sin tonalidad, así
que ni las teclas ni los acordes apilados con el conjunto llevan los
nombres de los acordes y escalas de la música tonal a los que pueden
sonar. En una tonalidad que es el conjunto Do Mi Sol Si♭ (4-27), T2I es
Re Mi Sol Si♭, la única forma que conserva tres de sus notas (para un
oído tonal, Mi Sol Si♭ Re es una séptima semidisminuida, pero la tecla
no la llama así). En Do mayor como conjunto (7-35), T7 conserva seis
notas, y sus notas se siguen escribiendo como las escribe Sol mayor,
con Fa♯. Solo los tres micromodos acorde de Kröpfl (Acorde disminuido,
menor-mayor y aumentado), que son tríadas por su nombre, muestran la
tríada que es cada forma (ver [Un micromodo como
tonalidad](#un-micromodo-como-tonalidad)). **Cercanas** muestra las
formas que conservan al menos la mitad de las notas de casa, redondeada
hacia abajo: dos de cuatro para 4-27, tres de siete para 7-21. Si menos
de cuatro formas están así de cerca, las muestra todas. **Todas las
formas** las muestra todas, las más lejanas al final.

Mientras suena una forma, mantenida o sonando en la canción, las
formas que conservan todas sus notas menos una aparecen con borde: son
las maneras más suaves de seguir, con una sola nota que se mueve. La
línea debajo de las teclas nombra la forma desde la que se cuentan. En
esa tonalidad de 7-21, mientras suena T4 (Si Do Do♯ Mi♭ Mi Sol La♭),
las teclas con borde son T0, T8, T3I, T7I y T11I, que conservan seis de
sus siete notas. Suelta la tecla, o detén la canción, y el borde se va:
las teclas muestran lo que está sonando, como las de los acordes.

Algunos conjuntos no tienen ninguna forma a una nota de distancia, y
entonces aparecen con borde las formas que más notas conservan, y la
línea dice cuántas. 6-20, la primera mitad de la serie de Webern (Mi♭
Mi Sol La♭ Si Do como tonalidad), tiene solo cuatro formas: mientras
suena T0, T1 y T3 aparecen con borde, porque conservan tres de sus seis
notas, y la línea dice «Con borde: conservan 3 notas de T0; ninguna
forma conserva más». Donde ninguna otra forma comparte una sola nota,
como en la séptima disminuida o la escala de tonos enteros, ninguna
tiene borde y la línea dice «Ninguna otra forma comparte una nota con
T0». Una forma con borde lejana de casa se suma al final de la fila con
**Cercanas**, así que todo borde está en una tecla que se ve; la tecla
se queda al soltar, para que puedas tocarla después.

Mientras suena la canción, se enciende la forma que se está tocando,
aunque sus notas lleguen de a pocas. Las teclas leen la canción como lo
hace **Formas de la tonalidad (mixtas)** en la hoja de acordes (ver
[Una armonía, cuatro nombres](#una-armonía-cuatro-nombres)). En
*Los tricordios de Webern* con 6-20 como tonalidad, cada mitad de la serie
llega como dos acordes de tres notas, y la mitad se enciende como una
sola forma: T0 y T2 mientras suenan la serie y su retrógrado, T1 y T3
mientras suena dada vuelta, con borde en las formas más cercanas a cada
una. Un patrón tocado solo se lee igual.

![Las teclas de armonía en una tonalidad que es el conjunto Do Mi Sol Si♭, conjunto 4-27 en el menú Tonalidad, con Formas activado, mientras se mantiene T2I: T0 (Mi Sol Si♭ Do), T2I (Re Mi Sol Si♭), encendida, después T3, T6, T9 y las formas dadas vuelta que conservan dos notas, cada una con 4-27 debajo; T0 aparece con borde porque conserva tres notas de T2I](images/set-forms-keys.png)

**Una progresión de formas.** En una tonalidad que es un conjunto, el
botón **Progresión** de la hoja de acordes ofrece maneras de encadenar
las formas del conjunto, en lugar de las progresiones de la Biblioteca,
y escribe ocho compases de ellas desde el compás elegido:

- **Conservar todas las notas menos una**: cada forma conserva todas las
  notas de la anterior menos una, la manera más suave de seguir. Do
  mayor como conjunto recorre el círculo de quintas: Fa mayor, Si♭
  mayor, Mi♭ mayor… En un micromodo se llama **Conservar dos notas,
  mover una**.
- **Conservar todas las notas que se pueda**, para un conjunto cuyas
  formas conservan todas las notas menos una solo de a pares, como la
  séptima de dominante y la séptima semidisminuida: Do7, Mim7b5, La7,
  Do♯m7b5… Nunca vuelve enseguida al acorde anterior.
- **Conservar una nota, mover dos** (micromodos): un pivote, un
  movimiento más amplio.
- **Las doce notas, cada *N* formas**: formas que no comparten ninguna
  nota, de modo que unas pocas juntas tienen las doce notas: cuatro
  formas de un micromodo, los tres acordes de séptima disminuida, o un
  hexacordo y las seis notas que deja fuera. Después la vuelta siguiente
  empieza desde una forma que conserva notas de la última. Solo aparece
  para los conjuntos que pueden hacerlo.
- **Secuencia**: todas las notas suben un semitono, un tono, una tercera
  menor, una cuarta o una quinta en cada compás.

Cada compás muestra su grado y cuántas notas conserva del compás
anterior («T2I · 3 comunes»), y puedes escribir un grado en cualquier
compás: `T2`, `T5I`. La banda toca cada forma como un acorde cuyas notas
comunes se quedan en las mismas teclas, sobre su fundamental cuando la
forma es un acorde (Do bajo Do7) y si no sobre la primera nota de su
orden más compacto.

Pasar a otro conjunto, con **Usar como tonalidad** en la ventana
Conjuntos, conserva el grado de cada forma, igual que pasar de un
micromodo a otro (ver [Un micromodo como
tonalidad](#un-micromodo-como-tonalidad)): de Do7 como conjunto a Domaj7
como conjunto, Do7 Mim7b5 La7 pasa a ser Domaj7 Mi♭maj7 Lamaj7.

Con **Conducción de voces** marcada, el piano roll revisa que las notas
guarden el color del conjunto: ver [Revisar el color](#revisar-el-color).

**Un estilo de micromodo sobre tus formas.** En una canción, un estilo
del grupo **Micromodos (Kröpfl)** de la Biblioteca toca las formas
escritas en la hoja de acordes, en tu orden, como un estilo toca los
acordes de una canción tonal: cambias un compás y el estilo toca la
forma nueva. Sirve en cualquier tonalidad de conjunto, no solo en un
micromodo: **Puntos vieneses** puede tocar Do7, Mim7b5, La7… en una
tonalidad que es Do Mi Sol Si♭ como conjunto. El teclado, el bajo y la
línea del estilo tocan en lugar del piano, el bajo y el solista de la
banda, y la guitarra, las cuerdas y el órgano descansan. Cuando la hoja
no tiene formas de la tonalidad (una canción en una tonalidad mayor o
menor, o una hoja vacía), o cuando la casilla **Progresión de acordes**
de la Biblioteca está marcada, el estilo escribe en cambio su propio
recorrido de formas en la hoja, y la tonalidad se vuelve su micromodo,
empezando en la tónica. Un estilo tuyo, tocado sobre un conjunto, toca
solo las notas del conjunto.

## Una armonía, cuatro nombres

Un conjunto es otra manera de **nombrar** notas, no otras notas. El
acorde de Do de Do mayor es Do, Mi y Sol se lo nombre como se lo
nombre. El menú **Mostrar** de la hoja de acordes escribe la misma
armonía de cuatro maneras, sin cambiar una nota: **Como está escrito**,
como **Acordes**, como **Conjuntos de clases de alturas** o como
**Micromodos**.

Toda canción se abre **Según la tonalidad**: cada armonía con los
nombres de la tonalidad en que está. Una canción en una tonalidad mayor
o menor o en un modo muestra acordes, una canción cuya tonalidad es un
conjunto o una serie muestra conjuntos, y una canción en un micromodo
muestra micromodos. Elige otro nombre para ver la canción de esa
manera; dura hasta que abres otra canción, que se abre otra vez según
su propia tonalidad.

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

La música construida sobre un conjunto mezcla las dos: una forma puede
ser dos notas tocadas juntas y una tercera después. Donde la tonalidad
es un conjunto o un micromodo, **Formas de la tonalidad (mixtas)**
escribe las formas del conjunto de la tonalidad como las lee la
[revisión del color](#revisar-el-color), vengan como vengan sus notas.
Donde una melodía y los acordes de debajo están cada uno en una forma
propia, escribe las dos, la de arriba primero, como dos acordes a la
vez: **3-3|3-3 T7** como conjuntos. En **Color Menor 3** encuentra las formas escritas
en la hoja, compás por compás. En una canción cuya tonalidad no es un
conjunto no tiene nada que leer, y lo dice.

Para encontrar el micromodo o el conjunto que una canción usa más, mira
la línea debajo de estos menús. **Más usados** ordena cada clase que
muestra la hoja, primero la que suena más tiempo, con cuánto de la
canción ocupa: **Menor 3 72% · 2-3 6% · Menor 2 6% …** para la melodía
de **Color Menor 3** leída en línea, de a tres. Cuenta los grupos como
los arma el segundo menú, así que si cambias el agrupamiento o la
pista, el orden cambia con ellos. Cada armonía cuenta desde donde
empieza hasta la siguiente. Señala una clase para ver cuántas veces
aparece y cuáles de sus formas aparecen más; **más** al final muestra
las demás. La línea aparece solo cuando **Mostrar** nombra la armonía
como conjuntos o micromodos: los acordes, las formas de la serie y N.C.
no cuentan.

El **piano roll** también tiene un menú **Mostrar**, arriba junto a
**Conducción de voces**, para los nombres de los acordes sobre su
cuadrícula: **Según la tonalidad**, con la que empieza, **Acordes**,
**Conjuntos de clases de alturas** o **Micromodos**. Nombra las notas
que suenan: Do, Re y Mi juntos se oyen
como Doadd9, pero como conjunto son 3-6, el micromodo Mayor 1, porque
ningún Sol suena. Un acorde y su inversión tienen las mismas notas, así
que Do y Do/Mi, dos bloques como acordes, son un solo bloque como 3-11
T7I. Donde un bloque es demasiado estrecho para el nombre entero,
muestra la clase sola, **3-11**; apúntalo para leer el nombre completo
en la línea bajo la barra de herramientas.

![El piano roll de la demostración Bruma de tonos enteros con Mostrar en Conjuntos de clases de alturas: la fila Todo sobre la cuadrícula nombra 2-2, 3-6, 2-2, 2-2 T2, 3-6 T2 y así, un bloque por pulso](images/piano-roll-sets.png)

La tonalidad funciona igual. Elige **conjunto de clases de alturas** en
el menú **Tonalidad** de una canción en Do mayor, y nada se mueve: la
tonalidad se llama ahora 7-35, y las teclas de armonía muestran los
mismos siete acordes con sus clases de conjuntos, seis de ellos 3-11 y
Si Re Fa 3-10. Las notas cambian solo cuando eliges otro conjunto.

Tres notas son a la vez un conjunto y un micromodo: Do Do♯ Mi es el
conjunto 3-3 y el micromodo Menor 3. El menú Tonalidad guarda el nombre
que eliges: desde una serie, **conjunto de clases de alturas** toma las
tres primeras notas de la serie y las nombra como conjunto, «conjunto
3-3 sobre Do» para la serie del Concierto de Webern sobre Do; **3** en
la fila **Menor** de los micromodos, al final del menú, nombra las
mismas notas como micromodo, «Menor 3 sobre Do». Pasar de uno a otro no
mueve nada, y **serie dodecafónica** trae de vuelta la serie de la
canción.

**Cualquier conjunto como tonalidad.** Debajo de **conjunto de clases
de alturas**, el menú Tonalidad lista los conjuntos como lista los
micromodos. **Notas** elige cuántas notas, de 2 a 10, e **Invertido
(I)** pone los conjuntos al revés; el menú sigue abierto mientras
cambias cualquiera de los dos. Haz clic en un conjunto para que sea la
tonalidad sobre la tónica: **3-11** sobre Do es Do Mi♭ Sol, y con
**Invertido (I)** Do Mi Sol. Un conjunto que es igual al revés, como
**4-28**, la séptima disminuida, sale igual de las dos maneras. El menú
se abre en el número de notas de la tonalidad, con su conjunto
rellenado: Do mayor se abre en los conjuntos de siete notas.

El conjunto que la propia canción usa más, entre los grupos de ese
número de notas, aparece recuadrado. En *Los tricordios de Webern* es 3-3
entre los conjuntos de tres y 6-20 entre los de seis, las dos mitades
de la serie. Apunta a él para leer cuánto de la canción ocupa y dónde
se encontró: en las notas que suenan juntas, o en la línea de una
pista. Haz clic en él para tomar las propias notas de la canción como
tonalidad, así nada tiene que moverse. Cualquier otro conjunto mueve
las notas como lo hace un micromodo nuevo: desde otro conjunto cada
forma conserva su grado, y desde una tonalidad mayor o menor las notas
se quedan como están.

Una vez que un conjunto es la tonalidad, el menú Tonalidad lo nombra
(**conjunto 6-20**) y las teclas de armonía pasan a sus formas, para
que puedas tocarlas enseguida (ver [Un conjunto como tonalidad: sus
formas](#un-conjunto-como-tonalidad-sus-formas)); **Acordes** vuelve a
los acordes apilados con sus notas.

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
arriba, con nombres de notas (en letras o en solfeo) o con números, y
haz clic en **Usar**; o
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
forma, **Una vez**, **Repetir** o **P, R, I, RI**, o **Un plan de
formas**: las formas que escribes en la casilla **Plan**, como P0 RI5
I7 R0, una tras otra, como una progresión de acordes es el plan de una
pieza tonal. Un clic en una etiqueta alrededor de la matriz agrega esa
forma al final del plan. Luego el registro,
**Cerrado** (todo en una octava), **Suave** (cada nota cerca de la
anterior) o **Amplio** (saltando por tres octavas, como en Webern).
**Serialismo** va más allá, como Messiaen, Boulez y Babbitt hacia 1950:
deja que la serie elija también las **Duraciones**, las **Dinámicas** y
las **Octavas**. Cuando la serie es combinatoria, **Con su compañera,
I3** (o la forma que sea) escribe con la forma su compañera, una octava
más abajo y nota por nota con el mismo ritmo, así que las seis primeras
notas de las dos forman las doce, y también las seis últimas: la manera
preferida de Schoenberg de mantener las doce notas sonando en dos voces
a la vez. Una serie sin compañera deja la casilla en gris. **Escribir
P0 en el patrón actual** (o **Escribir P0 RI5 I7…**, con un plan)
reemplaza las notas del canal, y **Guardar en un patrón nuevo** las
pone en un patrón propio.

### Una serie como tonalidad

Elige **serie dodecafónica** en el menú **Tonalidad**, o haz clic en
**Usar como tonalidad** en la ventana Serie dodecafónica para la forma
que elegiste, y la serie pasa a ser la tonalidad. El piano roll numera
sus notas del 1 al 12 en el teclado, en el orden de la serie, así que
puedes dibujar la serie siguiendo los números. Las teclas de armonía
tocan los grupos de la serie en lugar de acordes: **3 notas** da cuatro
teclas, 1–3, 4–6, 7–9 y 10–12; **4 notas**, tres, y **6 notas**, dos.

Toma otra serie en la ventana Serie dodecafónica y haz clic en **Usar
como tonalidad**, y la pieza se toca sobre la serie nueva: cada nota
conserva su lugar en su forma, así que la quinta nota de RI5 pasa a ser
la quinta nota de RI5 de la serie nueva. Ferrous Lineup encuentra de
qué forma es parte cada nota como lo hace la revisión de conducción de
voces, así que el plan de formas, el ritmo y los acordes se quedan, y
solo cambia la serie. Algunas series, como la de Berg, les dan dos
nombres a cada forma, y volver a la primera serie puede elegir el otro
nombre: **Ctrl+Z** es la forma segura de volver.

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

### Un estilo de series toca tu serie y tu plan

Cuando la tonalidad es una serie, los estilos de series de la
Biblioteca tocan esa serie, la tuya, cada uno a su manera: el Coral de
tríadas como acordes sobre un bajo lento, el Lo-fi dodecafónico sobre
un ritmo, y así. En una tonalidad que no es una serie, o con
**Progresión de acordes** marcada, un estilo trae su propia serie,
empezando en la tónica.

En una canción, la hoja de acordes guarda el **plan** de la pieza: qué
forma de la serie suena dónde. Haz clic en un compás y escribe una
forma, como **P0**, **RI5**, **I7** o **R0**, y la banda toca esa forma
ahí, mientras dure en la hoja, con las texturas del estilo: cada voz
del estilo lee su propia forma de la misma manera, así que donde la
hoja dice I5 un bajo que toca la serie al revés toca RI5. Cuando la
hoja todavía no tiene formas de la serie, el estilo escribe ahí su
propio plan, una parte cada una: la serie como está escrita, al revés,
dada vuelta y las dos cosas. Cambia cualquier compás y la banda toca la
forma nueva.

Los acordes, el bajo y la melodía del estilo ocupan el lugar del piano,
el bajo y el solista; los acordes y el bajo propios de la canción se
callan, y su melodía sigue sonando. El estilo Pads de hexacordios toca
su melodía desde la compañera de cada forma (ver [La ventana Serie
dodecafónica](#la-ventana-serie-dodecafónica)), así que cada compás
tiene las doce notas con cualquier serie combinatoria.

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
sus formas usa. Para aprender a oír las familias y después cada
micromodo, usa la pestaña **Micromodos** de la ventana [Entrenamiento
auditivo](practising.md#entrenamiento-auditivo-experimental).

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
**Archivo > Demostraciones > Micromodos** toca cada uno. En una
canción, la banda los toca desde la hoja de acordes (ver [Un estilo de
micromodo sobre tus formas](#un-conjunto-como-tonalidad-sus-formas)).

### Un micromodo como tonalidad

Cuando la tonalidad de la canción es un micromodo (después de **Usar
como tonalidad**, o de un estilo de micromodo de la Biblioteca), sus
acordes son las **formas** del micromodo: las mismas tres notas movidas,
o dadas vuelta y movidas. Se nombran por su **grado**, contado desde la
tonalidad como un número romano:

- **T0** es la forma propia de la tonalidad, casa. En Menor 3 sobre Do
  es Do Do♯ Mi.
- **T2** es casa subida 2 semitonos: Re Re♯ Fa♯.
- **T0I** es casa dada vuelta alrededor de la nota de la tonalidad, cada
  nota tan por debajo de Do como estaba por encima: Do Si La♭.
- **T1I** es eso subido un semitono: La Do Do♯.

Lleva toda la canción a Re y cada forma conserva su grado, como ii sigue
siendo ii en cualquier tonalidad mayor. Casa es siempre T0. Con las
tríadas, el acorde paralelo es T7I: Do menor en Do mayor, y Do mayor en
Do menor.

![Las teclas de armonía en una tonalidad de Menor 3 sobre Do mientras se mantiene T1I: T0, T1I (encendida), T4I y T5I, después las formas que conservan una nota de casa, cada una con sus notas y Menor 3 debajo; T0, T8 y T9 aparecen con borde porque conservan dos notas de La Do Do♯, y la ayuda de abajo lo dice](images/micromode-keys.png)

Las **teclas de armonía** muestran las formas de la tonalidad: casa
primero, después las formas que conservan dos de sus notas, después las
que conservan una; cuanto más conserva una forma de casa, más brillante
es su tecla. Las formas que conservan la misma cantidad van por grado,
primero las derechas. **Cercanas** deja fuera las formas que no conservan ninguna
nota de casa, y **Todas las formas** las muestra todas (24 para Menor
3). Mientras suena una forma, mantenida o sonando en la canción, las
formas que conservan dos de *sus* notas aparecen con borde: son las maneras más suaves de seguir, con una
sola nota que se mueve. La línea debajo de las teclas nombra la forma
desde la que se cuentan, y una forma con borde lejana de casa se suma
al final de la fila. Las cuatro formas de la tríada aumentada (Acorde
aumentado) no comparten ninguna nota, así que ninguna tiene borde, y la
línea dice «Ninguna otra forma comparte una nota con T0». Pasa el
puntero sobre una tecla para leer qué se
queda y qué se mueve, como «conserva La y Do; Do♯ pasa a La♭»; para las
tríadas mayores y menores también nombra el movimiento (P, L o R).

En la **hoja de acordes**, cada compás muestra su grado y, desde el
segundo compás, cuántas notas conserva del compás anterior: «T1I · 2
comunes». Escribe un grado para escribir esa forma: `T2`, `T5I`. Las
demostraciones de micromodos escriben sus formas en la hoja, así que
puedes leer el orden de cada una.

**Otro color, los mismos grados.** El menú **Tonalidad** lista los doce
micromodos debajo de las escalas, en tres filas cortas: **Menor** 1 a
5, **Mayor** 1 a 4, y las tríadas como **°**, **m/M** y **+** (apunta a
uno para ver su nombre completo). Se abre en el micromodo de la
tonalidad. Elige otro, y la pieza conserva su plan en el color nuevo,
como una canción en Do mayor conserva los grados de sus acordes cuando
pasa a Do menor: cada forma de la hoja de acordes conserva su grado (T2
de Menor 3 pasa a ser T2 de Menor 5) y cada nota escrita va al mismo
lugar en su forma nueva. Un estilo de micromodo que toca la banda sigue
las formas nuevas. Los enlaces suaves cambian con el color: la
demostración de Menor 3 conserva dos notas de compás a compás, y en
Menor 5 los mismos grados no conservan ninguna, como muestra la hoja
(«T4I · 0 comunes»).

Algunos micromodos son iguales dados vuelta (Menor 1, Mayor 1, y las
tríadas disminuida y aumentada), así que en ellos una forma dada vuelta
es solo una forma movida; al pasar a uno de ellos y volver no se puede
saber qué formas estaban dadas vuelta. Desde una tonalidad mayor o
menor a un micromodo, las notas se quedan como están, igual que con
**conjunto de clases de alturas**.

### Revisar el color

La música hecha con un solo conjunto toma su sonido, su **color**, del
conjunto, como la música tonal toma su sonido de su tonalidad: sus notas
salen de formas del conjunto, una forma tras otra. Así que cuando la
tonalidad es un micromodo o cualquier otro conjunto de clases de
alturas, la casilla **Conducción de voces** del piano roll revisa el
color en lugar de las reglas clásicas (ver [Conducción de voces y
movimiento de las partes](voice-leading.md)).

1. Abre una canción cuya tonalidad sea un conjunto o un micromodo, como
   **Archivo > Demostraciones > Micromodos > Color Menor 3**.
2. Abre el piano roll en un patrón con notas y marca **Conducción de
   voces** arriba.
3. Aparece una línea **Formas** debajo de los nombres de los acordes.
   Nombra cada forma que la revisión lee en las notas por su grado,
   **T0**, **T4I**, encima de las notas que la tocan. Apunta a una forma
   para leer cómo está hecha y sus notas: «T4I: la forma de la
   tonalidad dada vuelta alrededor de Do, transportada 4 semitonos hacia
   arriba: Do Mib Mi.»
4. Donde dos pistas, o la línea de arriba de un piano y sus acordes,
   están cada una en una forma propia a la vez, la línea tiene una
   franja para cada una, la más aguda arriba, y apuntar a una forma dice
   de quién es. La revisión lee así las partes solo donde deja menos
   notas fuera de lugar: una forma repartida entre los instrumentos, una
   nota cada uno, sigue siendo una forma.

La revisión lee sola qué forma suena dónde: no hace falta escribir
antes las formas en la hoja de acordes. Lo que les pide a las notas
depende del tamaño del conjunto:

- **Un micromodo se oye entero.** Sus tres notas forman un solo sonido,
  juntas como un acorde, una tras otra en una línea o repartidas entre
  las partes. Una nota fuera de la forma que se oye a su alrededor se
  marca **fuera**. Una forma que se detiene antes de que hayan sonado
  sus tres notas se marca **hueco**, sobre su última nota, porque dos
  notas solas forman un intervalo que comparten muchos micromodos: Do y
  Mi están en Menor 3, en Do mayor y en la tríada aumentada. Cuando
  falta una sola nota, un contorno punteado muestra dónde podría ir. La
  primera y la última forma de un patrón no se marcan si quedan sin
  terminar, porque pueden venir del patrón anterior o seguir en el
  siguiente.
- **Un conjunto más grande se usa como una escala.** Una melodía toma
  unas pocas de sus notas por vez, así que una forma no necesita sonar
  entera; solo se marcan **fuera** las notas fuera de la forma que se
  oye a su alrededor. Un pasaje que pasa a otra forma, como una melodía
  en Do mayor que pasa a Sol mayor con varios Fa♯ y ningún Fa, se lee
  como la forma nueva (T7). Un solo Fa♯ entre varios Fa queda como una
  nota fuera de T0.

La línea encima de las filas de acordes lo resume: «Color de Menor 3,
T0, T5I: todas las notas en el color.» Apunta a un paso con una marca y
la línea dice qué pasa allí, por ejemplo «Fuera del color: Melodía toca
Sol4, que no está en T0 (Do Do# Mi), la forma que se oye ahí.»; pasa el
puntero por la línea para leer por qué la música hecha con un conjunto
lo evita. Las demostraciones de micromodos guardan todas sus notas en su
color, así que la revisión no marca nada en ellas hasta que agregues
una nota suelta o saques una de una forma.

### Encontrar el color

**🔍 Buscar la tonalidad**, debajo de las teclas de armonía (ver
[Buscar la
tonalidad](keys-and-modes.md#buscar-la-tonalidad-experimental)), dice
en qué tonalidad mayor o menor parece estar la música. La música hecha
con un micromodo u otro conjunto no tiene una tonalidad así, de modo
que donde la tonalidad de la canción es un conjunto, un micromodo, una
serie o la escala cromática, una segunda línea debajo dice de qué
**color** está hecho el pasaje, cuando encuentra uno:

- **El micromodo** cuyas formas contienen las notas, el más fuerte, con
  los que encajan casi igual de bien después, más tenues: «Color en los
  compases 1 a 8: Mayor 1 sobre Do». Apúntalo para leer cuántas de las
  notas están en sus formas, y cuál de las formas es su centro: la que
  más se oye.
- **La colección** de notas de la que está hecho el pasaje, como una
  escala es las notas de las que está hecha una melodía: «hecho de 6-35
  (escala de tonos enteros)». Una colección que es justo la tonalidad
  mayor o menor que se muestra no se vuelve a nombrar.

![Buscar la tonalidad en la demostración Bruma de tonos enteros: «Suena en las notas de aquí: Re mayor, o La menor, o Do mayor», y debajo «Color en los compases 1 a 8: Mayor 1 sobre Do (la tonalidad), hecho de 6-35 (escala de tonos enteros)» con un botón Usar como tonalidad](images/find-colour.png)

En una canción lee de ocho en ocho compases: los ocho alrededor del
cursor de reproducción, contados desde el comienzo de la canción o
desde la marca de tonalidad anterior. En el modo patrón lee el patrón
entero.

**Usar como tonalidad** hace de ese micromodo o conjunto la tonalidad
de esos compases, con marcas de tonalidad en la lista de reproducción,
así que las teclas de armonía, la hoja de acordes y la revisión del
color lo siguen. Ninguna nota se mueve, y Deshacer lo revierte. Donde
ya es la tonalidad, la línea dice **(la tonalidad)**.

Una canción en una tonalidad mayor o menor o en un modo no tiene línea
de color: se muestra en acordes, como en la hoja de acordes. Para
buscar el color de una canción así, por ejemplo una pieza importada que
no suena tonal, elige primero **cromática** en el menú Tonalidad; nada
se mueve.

Pruébalo en **Archivo > Demostraciones > Micromodos > Bruma de tonos
enteros**. Sus notas son la escala de tonos enteros, que no encaja en
ninguna tonalidad mayor o menor, así que la suposición de la primera
línea, Re mayor, es pobre. La línea del color nombra de qué está hecha
la música: Mayor 1 (Do Re Mi), transportado por tonos enteros, y la
escala de tonos enteros sobre Do.
