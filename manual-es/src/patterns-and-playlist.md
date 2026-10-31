# Los patrones y la lista de reproducción

Una canción en Ferrous Lineup se arma con **patrones**, bloques cortos
de música de uno o pocos compases, colocados en el tiempo en la **lista
de reproducción**. Un patrón es «lo que toca cada canal durante estos
compases»: un ritmo de batería, cuatro compases de acordes, un relleno.
La lista de reproducción dice dónde suena cada patrón, y cuántas veces.
El mismo patrón puede sonar muchas veces: lo cambias una vez, y cambia
en todos los lugares donde suena.

La canción con la que se abre la aplicación muestra la idea. Tiene tres
patrones: **Acordes** (dos compases de Do, Lam, Fa y Sol), **Ritmo** (un
compás de batería) y **Relleno** (un redoble de batería). En la lista de
reproducción, Acordes suena cuatro veces en la primera pista, y Ritmo
cinco veces en la segunda desde el compás 3, con Relleno al final.

![La lista de reproducción de la canción inicial con la pestaña Patrones abierta a la izquierda (Ritmo, Acordes y Relleno, cada uno con su color), los compases 3 y 4 en bucle sobre los números de compás, y el menú de clic derecho de la pista Batería: Alto de la pista, Restablecer el alto de la pista, Insertar una pista arriba, Insertar una pista abajo, Subir, Bajar y Eliminar la pista](images/playlist.png)

## Los patrones

La pestaña **Patrones**, junto a la Biblioteca a la izquierda de la
ventana, lista los patrones de la canción, cada uno con su color. Haz
clic en uno para seleccionarlo. El **patrón seleccionado** es el que
muestran el rack de canales y el piano roll, y el que coloca un clic en
la lista de reproducción.

Los controles de **PATRÓN** de la barra de herramientas trabajan sobre
el patrón seleccionado:

- la lista elige otro patrón;
- **+** añade un patrón nuevo, vacío;
- **⋯** tiene el **Nombre** del patrón, **Duplicar** (una copia para
  cambiarla por separado) y **Eliminar** (que también quita sus clips de
  la lista de reproducción);
- la última lista fija su duración: 1, 2, 3, 4, 6, 8, 12 o 16 compases.

Una canción con banda tiene un patrón para cada músico, escrito a partir
de la hoja de acordes: no hace falta tocarlos para oír a la banda, y la
banda los vuelve a escribir cuando cambian los acordes (ver [Bloquear
una pista y tocar tu parte](locks.md)).

## Modo patrón y modo canción

**PAT** y **CANC**, a la izquierda de la barra de herramientas, eligen
qué toca ▶:

- el **modo patrón** repite una y otra vez el patrón seleccionado.
  Úsalo mientras escribes un ritmo o una parte;
- el **modo canción** toca la lista de reproducción desde el cursor de
  reproducción hasta el final.

**L** pasa de uno a otro.

## Colocar y mover clips

Cada bloque de la lista de reproducción es un **clip**: un patrón, en un
lugar de una pista. La línea de la barra de la lista de reproducción lo
dice en breve: *Haz clic para colocar «Ritmo». Doble clic en un clip
para editarlo, arrástralo para moverlo, arrastra sus bordes para
recortarlo o repetirlo, clic derecho para ver su menú.*

- **Haz clic** en un lugar vacío de una pista para poner ahí un clip del
  patrón seleccionado.
- **Arrastra** un clip para moverlo, a otro compás o a otra pista.
- **Arrastra hacia adentro su borde izquierdo o derecho** para
  recortarlo, como en la mayoría de los programas de audio: el clip
  oculta el comienzo o el final de su patrón, y esas notas no suenan.
  Siguen en el patrón, así que al arrastrar el borde de nuevo hacia
  afuera vuelven, y cada nota que sigue sonando queda donde estaba en
  la canción.
- **Arrastra su borde derecho más allá del final del patrón** para
  repetirlo: el patrón vuelve a sonar desde su comienzo mientras dure
  el clip, y el nombre del clip dice cuántas veces empieza, como «×2».
  Puede terminar a mitad de camino, en cualquier compás, pulso o paso.
  Arrastrado hacia afuera más allá del comienzo, el borde izquierdo
  toca una vez más el final del patrón antes de él.
- **Doble clic** en un clip para editarlo: las melodías y los acordes se
  abren en el piano roll, la batería en el rack de canales.
- **Clic derecho** en un clip para ver su menú: **Editar su patrón**,
  como un doble clic, y **Eliminar**, que elimina todos los clips
  seleccionados cuando el clip es uno de ellos. En una grabación,
  **Transcribir otra vez…** ocupa el lugar de Editar (ver
  [Tomas](takes.md#transcribir-otra-vez-una-toma)).

**Ajuste**, en la barra de la lista de reproducción, dice dónde caen los
clips: en un **Compás**, un **Pulso** o un **Paso** (una semicorchea).
Un borde arrastrado se ajusta igual, y también a los lugares donde el
patrón vuelve a empezar, para llegar fácil a un número entero de
repeticiones. **Ctrl+Z** deshace cada recorte. Una canción con un clip
recortado necesita esta versión de Ferrous Lineup o una más nueva para
abrirse.

Para trabajar con varios clips a la vez, haz **Ctrl+clic** en ellos uno
por uno, o **arrastra con Ctrl** un recuadro a su alrededor sobre una
parte vacía de la lista; arrastra uno de ellos y se mueven todos juntos.
**Ctrl+clic** en una sección de la franja Secciones, o **Seleccionar los
clips de esta sección** en su menú, selecciona los clips que tiene
debajo. Luego:

| Tecla | Qué hace |
|---|---|
| **Ctrl+C**, **Ctrl+X**, **Ctrl+V** | copia, corta y pega los clips |
| **Ctrl+D** | los duplica, justo después de sí mismos |
| **Supr** | los elimina |
| **Ctrl+A** | selecciona todos los clips |
| **Esc** | no selecciona ninguno |

**Ctrl+clic** en un lugar vacío elige dónde pega **Ctrl+V**, sin colocar
ningún clip.

## Las pistas

Las pistas son filas donde ordenar los clips; cualquier patrón puede ir
en cualquier pista. Una canción tiene tantas pistas como usa, y debajo
de la última hay una fila más tenue: coloca un clip en ella, o haz clic
en su **+ Nueva pista**, y se convierte en pista. **+ Pista**, en la
barra de la lista de reproducción, también añade una.

- La **luz** a la izquierda de una pista la silencia y le devuelve el
  sonido.
- **S**, a su derecha, deja sonar solo esa pista: todas las demás se
  silencian. Haz clic otra vez para oírlas todas.
- **Haz clic en su nombre** para cambiarlo.
- **Arrastra su borde inferior** para hacerla más alta o más baja;
  **doble clic** en el borde le devuelve su alto habitual.
- **Clic derecho en su nombre** para **Alto de la pista**,
  **Restablecer el alto de la pista**, **Insertar una pista arriba**,
  **Insertar una pista abajo**, **Añadir una grabación…** (ver
  [Tomas](takes.md#añadir-una-grabación-hecha-antes)), **Subir**,
  **Bajar** y **Eliminar la pista**, que también elimina sus clips.

## Las franjas sobre las pistas

- **Secciones**: arrastra por ella para marcar una parte de la canción,
  como una estrofa o un estribillo. Ver [Forma de la canción y
  secciones](song-form.md).
- **Tempo**: el tempo como una línea, con cada cambio marcado. Ver
  [Tempo, compases y cambios de tonalidad](tempo-meter-key.md).
- **Compases**: los números de compás, con el compás y la tonalidad en
  el compás 1 y donde cambian. **Haz clic** en un número para llevar ahí
  el cursor de reproducción. **Arrastra** por ellos para repetir esa
  parte de la canción: el modo canción la toca una y otra vez, y la
  barra de la lista de reproducción muestra **🔁 Repitiendo los compases
  3–4 ✖**; haz clic en él para volver a tocar toda la canción. **Clic
  derecho** en un número para insertar o eliminar compases, o para
  cambiar el compás o la tonalidad desde ahí.

## Moverse

| Tecla | Qué hace |
|---|---|
| **Espacio** | reproduce o detiene |
| **Inicio**, **Fin** | va al principio, o al final |
| **RePág**, **AvPág** | un compás atrás o adelante |
| **Ctrl+rueda**, **Ctrl++**, **Ctrl+−** | compases más anchos o más estrechos, alrededor del puntero |
| **Alt+rueda**, **Alt++**, **Alt+−** | pistas más altas o más bajas |
| **Ctrl+0** | vuelve al 100% |

Los dos zooms están también a la derecha de la barra de estado: **↔**
para los compases y **↕** para las pistas. **Seguir el cursor de
reproducción**, en la barra de herramientas, mantiene a la vista el
compás que suena; apágalo para desplazarte libremente mientras suena la
canción.
