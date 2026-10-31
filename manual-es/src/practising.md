# Practicar

Una banda que toca tus acordes sirve más cuando tocas con ella. La
**fila de práctica** de la hoja de acordes tiene lo que un músico busca
al practicar con una grabación: una cuenta de entrada, un metrónomo, una
velocidad más lenta, marcar el tempo con pulsaciones, repetir los
compases que estás trabajando, un entrenador de velocidad, y quitarle
una parte a la banda para tocarla tú. La ventana **Entrenamiento
auditivo** te entrena después para oír la armonía sin mirar.

![La hoja de acordes de la demostración Cadencia clásica con su fila de práctica: Cuenta de entrada y Metrónomo encendidos, Velocidad 70%, Entrenador de velocidad encendido con «desde 70% hasta 100% por 5% cada 1 pasada», y «Repetir los compases 3–4» encendido junto a «Dejar de repetir». Los compases 3 y 4 están sombreados, con una línea verde arriba. La barra de herramientas muestra TEMPO 70%, y la barra de estado dice «Entrenador de velocidad: 70%, después un 5% más rápido cada vez que el bucle vuelve a empezar…»](images/practising.png)

## Cuenta de entrada y metrónomo

- **Cuenta de entrada**: al pulsar Reproducir, primero suena un compás
  de clics, y la música empieza en el «uno» siguiente. Si empiezas en
  medio de un compás, la cuenta sigue hasta donde empiezas, así que el
  pulso nunca salta.
- **Metrónomo**: un clic en cada pulso mientras suena la
  música. El primer pulso de cada compás es más agudo, así que siempre
  sabes dónde está el «uno».

El metrónomo marca los pulsos que sientes, no cada nota de la indicación
de compás: dos veces por compás en 6/8, tres en 2+2+3/8 (ver [Ritmo y
métrica](rhythm-and-meter.md)). Los dos están también en la barra de
herramientas, así que funcionan con cualquier canción o patrón, y los
dos se recuerdan para la próxima vez.

## Velocidad y tempo con pulsaciones

**Velocidad** hace tocar a la banda más lento o más rápido que la
canción, del 25% al 200%. La canción conserva su propio tempo: el cuadro
**TEMPO** lo sigue mostrando, con la velocidad encima, como en «TEMPO
70%», y una canción guardada o un WAV exportado van a velocidad completa.
La velocidad vuelve al 100% cada vez que se inicia la aplicación, así
que una canción nunca suena lenta por sorpresa.

**Pulsar** lleva el tempo de la canción al tuyo: haz clic en él a
tiempo unas cuantas veces, junto con una grabación o con el tempo que
tienes en la cabeza.

## Repetir compases

Haz clic en un compás, **Mayús+clic** en otro (o mantén **Mayús** y
pulsa una flecha), y haz clic en **Repetir los compases 3–4**: esos
compases suenan una y otra vez. Quedan sombreados en la hoja, y la barra
de la lista de reproducción dice **Repitiendo los compases 3–4**.
**Dejar de repetir** vuelve a tocar toda la canción desde donde estás.

## El entrenador de velocidad

Un pasaje que no puedes tocar a velocidad completa se aprende igual en
todos los instrumentos: lento hasta que sale bien, luego un poco más
rápido, y otra vez. El **Entrenador de velocidad** lo hace por ti. Haz
clic en él, y aparecen cuatro cuadros a su lado:

- **desde**: la velocidad con la que empieza Reproducir;
- **hasta**: la velocidad a la que quieres llegar. 100% es el tempo de
  la canción; 110% lo pasa, para tener algo de margen;
- **por**: cuánto más rápida es cada velocidad nueva;
- **cada**: cuántas vueltas a cada velocidad antes de la siguiente. Con
  **cada 2 pasadas** tocas cada velocidad dos veces: una para lograrla y
  otra para comprobar que la tienes.

Pulsa Reproducir, y cada vez que el bucle vuelve a empezar la banda
toca un poco más rápido, hasta llegar a la velocidad que buscas, y se
queda ahí. El bucle es lo que repita la reproducción: los compases que
repites, toda la canción si no repites nada, o el patrón en el modo
patrón. **Detener** (⏹) devuelve la velocidad a la de partida, lista
para la próxima vuelta; **Pausa** (❚❚) la conserva, así que Reproducir
sigue a la velocidad a la que llegaste. Hacer clic otra vez en
**Entrenador de velocidad** lo apaga y deja la velocidad donde llegó.

Los números del entrenador se recuerdan, y **Edición > Ajustes… >
Reproducción** también los tiene, junto a la cuenta de entrada, el
metrónomo y la velocidad de práctica.

## Tocar una parte tú

**Tocas tú**, en la misma fila, le quita un músico a la banda para que
toques esa parte, con tu teclado pasado al sonido de ese músico. Ver
[Bloquear una pista y tocar tu parte](locks.md#tocas-tú).

## Entrenamiento auditivo (experimental)

Los músicos también necesitan *oír* la armonía: distinguir una tercera
menor de una mayor, o el V de una canción de su IV, sin mirar. **Ver >
Entrenamiento auditivo** abre una ventana que toca algo y te pide que lo
nombres, y te dice enseguida si acertaste, y por qué:

![La ventana Entrenamiento auditivo en su pestaña Intervalos: los intervalos 3m, 3M, 4J, 5J y 8J elegidos, tocados Arriba. La respuesta 5J está en rojo y 3M en verde, y el recuadro dice «No exactamente. Dijiste quinta justa. Mi3 subiendo a Sol#3: una tercera mayor, 4 semitonos. La base de un acorde mayor: de su fundamental a su tercera. Como las dos primeras notas de When the Saints Go Marching In.» La línea de puntos dice «Aciertos: 0 de 1 (0%) · 0 seguidos».](images/ear-training.png)

Tiene seis pestañas. Las tres primeras entrenan el oído para la música
en una tonalidad:

- **Intervalos**: suenan dos notas, y nombras el intervalo. Elige qué
  intervalos pueden salir y cómo se tocan, en **Tocados**: **Arriba**,
  **Abajo** o **Juntas**. Empieza con unos pocos que suenen muy
  distintos, como una tercera, una quinta y la octava, y suma los demás
  de a uno.
- **Acordes**: suena un acorde, y nombras su clase: mayor, menor,
  disminuido, aumentado, o uno de los acordes de séptima. Con
  **Inversiones**, su nota más grave no siempre es la fundamental;
  **Nota por nota** lo toca de a una nota, desde el bajo hacia arriba.
- **Progresiones**: la banda toca unos pocos acordes en una tonalidad,
  un compás cada uno, y los nombras por sus números en una fila de
  teclas de armonía con los nombres ocultos. El primer acorde es siempre
  I, así que sabes dónde empiezas, y **Escuchar la tonalidad** toca una
  cadencia que dice dónde está la casa. Elige las **Tonalidades**, los
  acordes que pueden salir, **Séptimas**, los **Músicos**, la
  **Duración** y el **Tempo**.

Las otras tres lo entrenan para la música sin tonalidad (ver [Más allá
de las tonalidades: conjuntos, series y
micromodos](sets-rows-micromodes.md)):

- **Micromodos**: suenan tres notas, y nombras su color, uno de los
  doce [micromodos](sets-rows-micromodes.md#micromodos). Empieza con
  **Respuesta: Su familia**: **Menor** (con un semitono), **Mayor** (un
  tono y ningún semitono) o **Acorde** (dos terceras o más). **El
  micromodo** pide el micromodo mismo. Elige qué micromodos pueden
  salir y cómo se tocan, en **Tocados**; **Abiertas** toca las notas en
  cualquier orden desde abajo, en hasta dos octavas.
- **Formas**: suena una figura, y después otra forma del mismo
  conjunto, y dices si la segunda es la primera **Transportada** (cada
  paso queda igual) o **Dada vuelta** (cada paso va hacia el otro
  lado). Elige 3, 4 o 5 **Notas**, tocadas **Como línea** o **Como
  acorde**. Como acordes, una tríada mayor dada vuelta es una menor.
- **Series**: suena una serie dodecafónica, y después una de sus
  formas, y dices cuál: **P** (transportada), **I** (dada vuelta), **R**
  (hacia atrás) o **RI** (dada vuelta y hacia atrás). Las series salen
  **De obras** (de Schoenberg, Berg, Webern y otros) o **Al azar**.
  Algunas series, como la de la Sinfonía de Webern, leídas hacia atrás
  son la misma serie transportada; entonces las dos respuestas son
  correctas.

Después de responder, la respuesta correcta se pone verde, una
equivocada roja, y un recuadro la explica: las notas, el nombre
completo, cuántos semitonos y, para un intervalo hacia arriba, una
melodía que empieza con él (en las tres últimas pestañas, los pasos, la
familia o la obra de la que sale la serie). Haz clic en cualquier botón de respuesta
para oírlo junto al correcto. **▶ Volver a tocar** repite la pregunta
todas las veces que quieras, y **Siguiente ▸** hace otra (antes de
responder es **Saltar ▸**, y una pregunta saltada no cuenta). La línea
de abajo lleva los puntos de cada pestaña; **Empezar de nuevo** los
vuelve a cero.

La ventana no cambia nada en la canción, y la canción se pausa mientras
suena una pregunta.
