# Sonidos

Ferrous Lineup puede tocar sus instrumentos de dos maneras:

- con su propio sintetizador, el **Lineup Osc**, que construye cada
  sonido a partir de unas pocas ondas simples. No necesita nada más, y
  es lo que suena hasta que elijas otra cosa;
- con un **SoundFont**, un archivo de grabaciones de instrumentos
  reales: un piano, un contrabajo, un bandoneón, un coro, una batería.
  Así es como debería sonar la banda.

## Al abrir la aplicación

La primera vez que abres Ferrous Lineup, antes de que nada suene, una
ventana llamada **Sonidos de instrumentos reales** ofrece descargar
**GeneralUser GS**, un SoundFont libre de S. Christian Collins, de unos
30 MB, y pregunta «¿Tocar los instrumentos con él?». Si la aplicación
ofrece antes recuperar trabajo sin guardar, los sonidos vienen justo
después.

![La aplicación al abrirse por primera vez: la ventana Sonidos de instrumentos reales, sobre la canción inicial, ofrece GeneralUser GS y pregunta «¿Tocar los instrumentos con él?», con Descargar, Elegir un archivo… y Ahora no](images/soundfont-offer.png)

- **Descargar** lo baja una vez y lo guarda en tu computadora. Una barra
  muestra cuánto falta, y **Detener** abandona la descarga. Al terminar,
  la barra de estado dice «Los instrumentos suenan desde GeneralUser
  GS», y cada instrumento toca su grabación desde entonces, también la
  próxima vez.
- **Elegir un archivo…** usa otro SoundFont que ya tengas, cualquier
  archivo `.sf2` hecho para General MIDI.
- **Ahora no**, o cerrar la ventana, se queda con el Lineup Osc. La
  aplicación no vuelve a preguntar; el ajuste de abajo está ahí para
  cuando cambies de idea.

Si GeneralUser GS ya está en tu computadora, descargado por una versión
anterior por ejemplo, la ventana lo dice y ofrece **Usarlo** en lugar de
**Descargar**: un momento después los instrumentos suenan con él.
Hasta que respondas, vuelve a preguntar cada vez que se abre la
aplicación, y al pulsar ▶ o abrir una demostración. Tu respuesta y los
ajustes de sonido de abajo se conservan cuando actualizas Ferrous
Lineup.

Si la descarga falla (sin red, un cortafuegos), la ventana dice por qué.
Puedes descargar `GeneralUser-GS.sf2` desde [su
página](https://github.com/mrbumpy409/GeneralUser-GS) con un navegador y
elegirlo con **Elegir un archivo…**.

## Los ajustes de sonido

**Edición > Ajustes…**, en **Audio**, tiene las mismas opciones:

| Ajuste | Qué hace |
|---|---|
| **Instrumentos del SoundFont** | Los canales con un instrumento tocan la grabación del SoundFont. Desmárcalo para volver al Lineup Osc. |
| **Batería del SoundFont** | La batería también toca la del SoundFont: un bombo, una caja y un charles grabados. |
| **Elegir SoundFont…** | Toca otro archivo `.sf2` en su lugar. |
| **Usar GeneralUser GS** | Vuelve a GeneralUser GS después de haber elegido otro archivo. |

La página Audio también dice a qué salida va el sonido, y tiene
**Reiniciar audio** para cuando la tarjeta de sonido se detiene (ver
[Solución de problemas](troubleshooting.md)).

Las exportaciones a WAV y a video, el teclado del equipo y un teclado
MIDI también tocan con el SoundFont.

## Un instrumento para cada canal

Cada canal que toca notas tiene un instrumento, uno de los 128 de
**General MIDI**, la lista estándar de instrumentos que todos los
sintetizadores y SoundFonts numeran igual, desde el piano de cola hasta
el disparo. Los músicos de la banda, el piano y el bajo de la
Biblioteca, y cada voz de cada demostración vienen con el que les
corresponde.

Para cambiar el instrumento de un canal:

- en el **rack de canales**, haz clic derecho en el nombre del canal y
  abre **Instrumento**, o
- en el **mezclador**, haz clic en el nombre del instrumento en la línea
  del canal.

Los instrumentos están agrupados como los agrupa General MIDI: pianos,
órganos, guitarras, bajos, cuerdas, metales, lengüetas, etc. **Ninguno
(el sonido propio del Lineup Osc)** devuelve el canal al Lineup Osc,
diga lo que diga el ajuste del SoundFont.

El mismo menú puede enviar el canal a un sintetizador externo por MIDI:
ver [Teclados y sintetizadores MIDI](midi.md).

## Tus propios sonidos

El rack de canales también toca sonidos tuyos. Arrastra un archivo WAV
desde tu gestor de archivos a un canal del rack, y ese canal lo toca en
cada paso que enciendas, como una caja de ritmos toca sus muestras. Ver
[El rack de canales](channel-rack.md).
