# Teclados y sintetizadores MIDI

**MIDI** es la forma en que los instrumentos electrónicos y las
computadoras se pasan notas: un teclado dice qué tecla bajó y con qué
fuerza, y un sintetizador que lo recibe toca la nota. Ferrous Lineup
puede escuchar un teclado MIDI, para que toques junto con la banda o
grabes en un patrón, y puede enviar sus notas a un sintetizador fuera de
la aplicación.

No hace falta un teclado MIDI para tocar: el propio teclado del equipo
puede hacer de uno.

## Conectar un teclado

Conecta el teclado (la mayoría se conecta por USB) y abre el menú
**MIDI**. Bajo **Entrada: un teclado para tocar** aparecen los teclados
que encuentra: haz clic en el tuyo. El menú vuelve a buscar cada vez que
se abre, así que también aparece un teclado conectado después de abrir
la aplicación. **Ninguna** lo desconecta. La elección dura hasta que se
cierra Ferrous Lineup.

Lo que tocas suena en el **canal seleccionado**, el que muestra el
piano roll: haz clic en el nombre de un canal en el rack de canales para
elegirlo. Cada nota conserva la fuerza con que la tocaste. Mientras
mantienes teclas, la barra de estado dice qué forman, como el piano
roll: el acorde, y su número en la tonalidad.

Si el menú dice **MIDI no disponible**, el sistema no le dio a la
aplicación ningún MIDI. En Linux, MIDI necesita el secuenciador ALSA;
ver [Solución de problemas](troubleshooting.md).

## El teclado del equipo

Marca **MIDI > Teclado del equipo como piano**, y las teclas de letras
tocan el canal seleccionado, dispuestas como un piano:

| Teclas | Tocan |
|---|---|
| **Z X C V B N M** | las teclas blancas desde el Do central (Do4) hasta el Si |
| **S D G H J** | las teclas negras entre ellas, desde Do♯4 |
| **Q W E R T Y U I O P** | las teclas blancas desde el Do de arriba (Do5) |
| **2 3 5 6 7 9 0** | las teclas negras entre esas, desde Do♯5 |

Las teclas cuentan por dónde están en el teclado, no por la letra
impresa en ellas, así que el piano conserva su forma con cualquier
distribución de teclado. Mientras está activado, las teclas de números
tocan notas en lugar de las teclas de armonía. Las teclas no tocan
mientras escribes en una casilla de texto o hay un menú abierto, y los
atajos con **Ctrl** y **Alt**, como **Ctrl+Z**, siguen funcionando.

## Grabar

El botón **●**, junto a ▶, graba en un patrón lo que tocas:

1. Selecciona el canal donde grabar, y el patrón donde grabar.
2. Haz clic en **●**. Se pone rojo, y la aplicación pasa al modo
   patrón.
3. Pulsa **Espacio** para tocar el patrón, y toca junto con él. Cada
   nota entra en el patrón cuando sueltas su tecla.
4. Haz clic otra vez en **●** para dejar de grabar.

El patrón da vueltas mientras grabas, y cada vuelta se suma a lo que ya
tiene: toca las notas del bajo en una vuelta y los acordes en la
siguiente. Cada nota se lleva al paso más cercano de la cuadrícula, así
que una nota un poco adelantada o atrasada cae en el pulso, y un toque
rápido igual da una nota de un paso. Se conserva la fuerza con que
tocaste. Una nota que sigue sonando cuando termina el patrón se corta
ahí.

Una toma dura hasta que se detiene la música o haces clic en **●**, y
**Ctrl+Z** deshace la toma entera. Con **Ajustar las notas que tocas**
activado (clic derecho en **Ajustar a la escala** en el piano roll),
cada tecla se graba como la nota más cercana de la tonalidad. Para
grabar con más comodidad, baja la velocidad de la canción mientras
tocas (ver [Practicar](practising.md)).

## Tocar acordes para la banda

**MIDI > El acompañamiento sigue los acordes** hace que la banda siga
tu mano izquierda, como un teclado arreglista: mientras suena la
música, el acorde que mantienes se vuelve el acorde que toca el
acompañamiento. Los acordes y la línea de bajo conservan su ritmo y
pasan a tu acorde desde su nota siguiente.

No hace falta tocar el acorde entero:

| Mantienes | La banda toca |
|---|---|
| una tecla | su acorde mayor: Do toca Do |
| dos teclas a una tercera menor | un acorde menor sobre la más grave: La y Do tocan Lam |
| dos teclas a una tercera mayor o a una quinta | un acorde mayor sobre la más grave |
| dos teclas a una séptima menor | un acorde de séptima sobre la más grave: Sol y Fa tocan Sol7 |
| tres teclas o más | el acorde que forman |

El último acorde se queda hasta que tocas otro, y la barra de estado
dice en qué acorde está el acompañamiento. Cuando se detiene la
reproducción vuelven las notas escritas: la canción misma no cambia.
[Las teclas de armonía](harmony-keys.md) también pueden tocar acordes
para la banda.

## Enviar a un sintetizador

Ferrous Lineup puede tocar sus canales en un sintetizador fuera de la
aplicación, un sintetizador físico o un programa, en lugar de sus
propios sonidos.

**Un canal a la vez.** Haz clic derecho en el nombre de un canal en el
rack de canales, o haz clic en su instrumento en el mezclador, y abre
**Enviar por MIDI**:

- **Canal MIDI** elige en cuál de los dieciséis canales de MIDI suena.
  Un sintetizador General MIDI toca la batería en el canal 10.
- **Salida, para todos los canales enviados por MIDI** elige el
  sintetizador. Es la misma salida que la del menú MIDI.

El menú dice entonces, por ejemplo, **● Enviado por MIDI, canal 2**, y
**Tocarlo aquí en su lugar** le devuelve al canal su propio sonido.
**+ Añadir canal > Salida MIDI (sintetizador externo)**, en el rack de
canales, añade un canal que solo suena en el sintetizador.

**Toda la canción.** En el menú MIDI, elige el sintetizador bajo
**Salida: un sintetizador externo para tocar**, y marca **Enviar todos
los canales**. Todos los canales suenan entonces en el sintetizador en
lugar de en los altavoces, en los canales que usa un archivo MIDI
exportado: la batería en el canal 10 como un set de batería General
MIDI, los demás en 1, 2, 3, etc. El instrumento General MIDI de cada
canal va con él. Desmárcalo para volver a oír los sonidos de la
aplicación.

No se envía nada mientras la aplicación no tiene tarjeta de sonido: el
menú MIDI lo dice.
