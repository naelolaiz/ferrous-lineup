# El rack de canales

El **rack de canales** (**F6**, o **Ver > Rack de canales**) muestra un
patrón, con una fila para cada **canal**: cada tambor, y cada
instrumento que toca notas. Es donde escribes la batería, paso a paso,
como en una caja de ritmos.

![El rack de canales sobre el patrón Acordes mientras suena en modo patrón: la luz sobre el paso que suena encendida en naranja y su columna enmarcada en todas las filas, las filas de batería vacías con la línea «No hay pasos de batería en «Acordes»: haz clic en un paso para añadir uno. Hay batería en: Ritmo, Relleno», y la fila de Lineup Osc mostrando los acordes como un pequeño piano roll](images/channel-rack.png)

## Leerlo

El título de la ventana nombra el patrón que muestra, como en **Rack de
canales · Ritmo**. Cada fila tiene, de izquierda a derecha:

- una **luz**, verde mientras el canal se oye: haz clic en ella para
  silenciar el canal o devolverle el sonido;
- una **perilla** para su volumen: arrástrala hacia arriba o hacia
  abajo;
- un **candado**, para conservar las notas y el sonido del canal cuando
  pruebas estilos (ver [Bloquear una pista y tocar tu parte](locks.md));
- su **nombre**;
- sus **pasos**: dieciséis por compás, uno por cada semicorchea,
  sombreados oscuros y claros pulso a pulso para poder contarlos.

Un canal que toca melodías o acordes muestra una pequeña imagen de sus
notas en lugar de pasos. Haz clic en ella para abrir el piano roll.

Sobre los pasos hay una fila de lucecitas. Mientras suena el patrón, la
luz sobre el paso que suena se enciende en naranja y su columna queda
enmarcada, en todas las filas.

## Escribir la batería

**Haz clic en un paso** para encenderlo, y el tambor suena ahí; haz clic
otra vez, o clic derecho, para apagarlo. Pulsa **Espacio** con el rack
delante para oír el patrón una y otra vez (o pulsa **L** para el modo
patrón y luego **Espacio**), y cambia pasos mientras suena.

Un buen primer ritmo en 4/4, en un compás de dieciséis pasos:

| Canal | Pasos | Qué se oye |
|---|---|---|
| Bombo | 1, 9 | el bombo en los pulsos 1 y 3 |
| Caja o Palmas | 5, 13 | el contratiempo, en los pulsos 2 y 4 |
| Charles | 1, 3, 5, 7, 9, 11, 13, 15 | corcheas |

Ese es el ritmo de base del rock y el pop: el tambor grave en los pulsos
fuertes, la caja en los débiles. Pasa un golpe de bombo al paso 11, el
«y» del pulso 3, y el ritmo gana empuje.

Cuando un patrón no tiene pasos de batería pero otros sí, el rack lo
dice, con un botón para cada patrón que los tiene, como en *No hay pasos
de batería en «Acordes»: haz clic en un paso para añadir uno. Hay
batería en: Ritmo, Relleno*. Las canciones suelen tener la batería en
patrones propios.

## Arriba

- **Editar estilo…** y **Nuevo estilo…** abren el editor de estilos
  (ver [Tus propios estilos](style-editor.md)).
- **▶ Tocar patrón** toca todos los canales de este patrón; mientras
  suena, el botón es **Pausar patrón**, y **■ Detener patrón** lo
  detiene.
- Mientras suena la canción, **▶ Tocar sobre la canción** toca el patrón
  encima de ella, desde el compás siguiente y a tempo, hasta que vuelves
  a hacer clic: una forma de probar un patrón nuevo con el resto de la
  canción.
- La lista elige qué patrón muestra el rack, como la lista PATRÓN de la
  barra de herramientas.

## Los canales

- **Haz clic en un nombre** para seleccionar el canal y oírlo; haz clic
  otra vez, o en una parte vacía del rack, o pulsa **Esc**, para no
  seleccionar ninguno. El canal seleccionado es el que tocan un teclado
  MIDI o el teclado del equipo.
- **Doble clic en un nombre** abre el piano roll del canal.
- **Clic derecho en un nombre** abre su menú: **Piano roll**, su
  **Instrumento** (ver [Sonidos](sounds.md)), enviarlo a un sintetizador
  externo por MIDI (ver [Teclados y sintetizadores MIDI](midi.md)),
  **Bloquear** o **Desbloquear**, y **Quitar canal**. **Supr** también
  quita el canal seleccionado, y **Ctrl+Z** lo devuelve.
- **+ Añadir canal**, debajo de las filas, añade un tambor (Bombo, Caja,
  Palmas, Charles, Charles abierto, Aro, Tom grave, Tom agudo, Ride,
  Crash, Cencerro, Shaker, Conga), un **Lineup Osc (sintetizador)** para
  tocar notas, o una **Salida MIDI (sintetizador externo)**, cuyas notas
  suenan en un sintetizador fuera de la aplicación.

### Tus propios sonidos

Arrastra un archivo **WAV** desde tu gestor de archivos hasta el nombre
de un canal, y ese canal lo toca en cada paso que enciendas, como una
caja de ritmos toca sus muestras. Soltado en cualquier otro lugar del
rack, recibe un canal propio. La canción guarda dónde está el archivo,
no el sonido mismo, así que deja el WAV donde está: si se mueve o se
borra, el canal no tiene nada que tocar.

## Zoom

Sobre el rack, **Ctrl** con **+** y **−**, o con la rueda del ratón,
agranda o achica todo el rack, y **Ctrl+0** lo devuelve al 100%.
