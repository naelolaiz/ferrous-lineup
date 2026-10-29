# El piano roll

El **piano roll** muestra las notas de un canal en un patrón: el tiempo
de izquierda a derecha, la altura de abajo hacia arriba, con un teclado
de piano a lo largo del lado izquierdo. Cada nota es una barra tan larga
como lo que dura la nota. Es donde se escriben y se cambian melodías,
acordes y líneas de bajo, nota por nota, y donde la aplicación te dice
qué forman las notas.

![El piano roll sobre el patrón Acordes de la canción inicial: la barra con la tonalidad, Ajustar a la escala, Serie…, Conjuntos…, Micromodos…, Dibujar, Duración y Herramientas; los botones de los acordes de Do mayor con ii, IV y V encendidos después de Lam; la línea «Paso 13: Lam: La menor, acorde vi de Do mayor.» y los nombres de acorde Do, Lam, Fa y Sol sobre la cuadrícula, y el carril de velocidad debajo](images/piano-roll-window.png)

## Abrirlo

- Pulsa **F7**, o haz clic en **Piano roll** en la barra de
  herramientas: se abre sobre el patrón seleccionado y el canal
  seleccionado.
- **Doble clic** en un clip de melodía o de acordes en la lista de
  reproducción, o en el nombre de un canal en el rack de canales.
- Clic derecho en el nombre de un canal en el rack y elige **Piano
  roll**.

El título de la ventana nombra el patrón y el canal, como en **Piano
roll · Acordes · Lineup Osc**. Cuando el canal no toca nada en ese
patrón, el piano roll lo dice y ofrece un botón para cada patrón donde
sí toca; para la batería ofrece el rack de canales, donde se escribe la
batería.

## Leer la cuadrícula

- Las filas de las notas de la tonalidad son más claras que las demás,
  y las de la nota de la tonalidad son marrones. En el teclado, un punto
  azul marca cada nota de la tonalidad y uno naranja la nota de la
  tonalidad. **Haz clic en una tecla** para oírla.
- Las barras de compás son más marcadas que las líneas de pulso.
- Sobre la cuadrícula, dos filas nombran los acordes: la primera, el
  acorde que toca este canal; la segunda, **Todo**, el acorde que forman
  todos los canales juntos.
- Encima de ellas, una línea dice qué hay bajo el puntero: el acorde en
  ese paso, su nombre y su número en la tonalidad («Lam: La menor,
  acorde vi de Do mayor»). Otra nombra el **conjunto** de las notas,
  para la teoría de conjuntos de [Más allá de las tonalidades:
  conjuntos, series y micromodos](sets-rows-micromodes.md).
- Mientras suena la música, el paso que suena aparece sombreado, y la
  línea sobre la cuadrícula nombra el acorde que suena ahí.

## Escribir notas

| Haz esto | Y |
|---|---|
| **Clic** en un lugar vacío | ahí va una nota (mantén pulsado el botón y arrastra para moverla enseguida) |
| **Arrastra** una nota | se mueve, en el tiempo y en altura |
| **Arrastra el extremo derecho** de una nota | se alarga o se acorta |
| **Clic derecho** en una nota, o arrastrar con el botón derecho sobre varias | se eliminan |
| **Clic** en una nota | queda seleccionada |
| **Ctrl+clic** en una nota | se añade a la selección, o se quita |
| **Arrastrar con Ctrl** desde un lugar vacío | se seleccionan las notas del recuadro |
| **Arrastra** una nota seleccionada | todas las notas seleccionadas se mueven juntas |

**Dibujar**, en la barra de herramientas, dice qué pone un clic:

- **Nota sola**;
- **Acorde de la tonalidad (tríada)** o **Acorde de la tonalidad
  (séptima)**: el acorde de la tonalidad sobre la nota en la que haces
  clic, con las notas que le da la tonalidad, así que en Do mayor un
  clic en Re dibuja Rem y en Sol dibuja Sol (o Rem7 y Sol7);
- un tipo de acorde, el mismo sobre cualquier nota: **Mayor**,
  **Menor**, **Séptima de dominante**, **Séptima mayor**, **Séptima
  semidisminuida**, **Sexta y novena** y los demás, treinta en total;
- **Un conjunto del explorador de conjuntos…**.

Para un acorde, la lista de al lado elige su **inversión**: cuál de sus
notas queda abajo (ver [Los acordes y sus nombres](chords.md)). La nota
en la que haces clic sigue siendo la fundamental. **Duración** es lo que
dura una nota nueva: **1 paso**, **2 pasos**, **1 pulso**, **2 pulsos**
o **1 compás**.

**Ajustar a la escala** hace que las notas que dibujas o mueves caigan
en las notas de la tonalidad. Haz clic derecho en él para **Ajustar las
notas que tocas** también, que convierte las teclas tocadas en un
teclado MIDI o en el del equipo en la nota más cercana de la tonalidad,
tanto al oírlas como al grabarlas.

## Los acordes de la tonalidad

Debajo de la barra de herramientas, un botón para cada acorde de la
tonalidad, como en **Acordes en Do mayor: I · Do, ii · Rem, iii ·
Mim…**. Haz clic en uno para oírlo. Señala un acorde en la cuadrícula, o
mantén uno en un teclado, y se encienden los acordes que suelen venir
después, con una línea que lo dice: «Después de Lam (vi), prueba: Rem
(ii), Fa (IV), Sol (V)». [Qué hacen los acordes](chord-functions.md)
explica por qué esos.

## Herramientas

El menú **Herramientas** cambia las notas seleccionadas, o todas las
notas cuando no hay ninguna seleccionada:

| Herramienta | Tecla |
|---|---|
| **Subir un semitono**, **Bajar un semitono** | **↑**, **↓** |
| **Subir una octava**, **Bajar una octava** | **Mayús+↑**, **Mayús+↓** |
| **Subir una nota de la tonalidad**, **Bajar una nota de la tonalidad** | **Alt+↑**, **Alt+↓** |
| **Un paso antes**, **Un paso después** | **←**, **→** |
| **Un pulso antes**, **Un pulso después** | **Mayús+←**, **Mayús+→** |
| **Cuantizar a los pasos**: las notas tocadas un poco fuera de la cuadrícula pasan a ella | |
| **Ajustar a la tonalidad**: las notas fuera de la tonalidad pasan a la nota más cercana de la tonalidad | |
| **Invertir los acordes hacia arriba**, **Invertir los acordes hacia abajo**: la nota más grave de cada acorde sube una octava, o la más aguda baja | |
| **Arpegiar los acordes**: cada acorde tocado nota tras nota, **arriba**, **abajo** o **arriba y abajo**, a la velocidad elegida en **Cada nota** | |
| **Seleccionar todo** | **Ctrl+A** |
| **Copiar**, **Cortar**, **Pegar**, **Duplicar** | **Ctrl+C**, **Ctrl+X**, **Ctrl+V**, **Ctrl+D** |
| **Eliminar las notas seleccionadas** | **Supr** |

**Subir una nota de la tonalidad** es la forma de mover una melodía que
usan los músicos: Do Mi Sol en Do mayor se vuelve Re Fa La, un acorde
menor, no Re Fa♯ La, porque se queda en la tonalidad. **Subir un
semitono** mueve todas las notas la misma distancia, fuera de la
tonalidad.

Las notas pegadas o duplicadas alargan el patrón cuando necesitan
lugar. Las notas copiadas en la canción se pueden pegar en el editor de
estilos, y al revés.

## Velocidad

Debajo de la cuadrícula, el carril **Velocidad** muestra con qué fuerza
se toca cada nota, como una barra de suave a fuerte. Haz clic o arrastra
en él para fijar la velocidad de las notas de ese paso, o de las
seleccionadas ahí.

## Ver las otras partes

- **Notas fantasma** muestra tenues las notas de los otros canales
  detrás de las de este, para que puedas escribir una línea de bajo que
  encaje con los acordes, y nombra los acordes que forman todos juntos.
- **Conducción de voces** dibuja cómo se mueve cada voz de un acorde al
  siguiente (ver [La conducción de voces y cómo se mueven las
  partes](voice-leading.md)).

## La tonalidad y las ventanas de conjuntos

Los menús de **Tonalidad**, arriba a la izquierda, son la tonalidad de
la canción: elegir otra mueve la canción, como se explica en
[Tonalidades y modos](keys-and-modes.md). Al lado se escriben las notas
de la tonalidad. **Serie…**, **Conjuntos…** y **Micromodos…** abren las
ventanas de las series dodecafónicas, los conjuntos de clases de altura
y los micromodos de Kröpfl (ver [Más allá de las tonalidades:
conjuntos, series y micromodos](sets-rows-micromodes.md)).

## Zoom

Sobre el piano roll, **Ctrl** con la rueda del ratón, **+** o **−**
ensancha o estrecha los pasos, y **Alt** hace las filas más altas o más
bajas, alrededor del puntero. **Ctrl+0** lo devuelve al 100%.
