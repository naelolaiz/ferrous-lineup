# Solución de problemas

## La aplicación no arranca

Ver [Instalarlo](installing.md): la AppImage tiene que tener permiso
para ejecutarse y puede necesitar la biblioteca FUSE 2; Windows pregunta
una vez antes de abrir un programa sin firmar; macOS te pide permitir la
aplicación una vez en **Ajustes del Sistema > Privacidad y seguridad**.

## No hay sonido

Mira la izquierda de la barra de estado, abajo de todo en la ventana:

- **Audio: Default Audio Device · 48000 Hz · 2 canales**, o el nombre
  de tu tarjeta de sonido: la aplicación tiene tarjeta de sonido, así
  que el silencio está en algún lugar de la canción. Revisa el control
  **VOL** de la barra de herramientas y el volumen **Principal** del
  mezclador (**F9**), un **Silenciar** o un **Solo** en el mezclador, la
  luz de un canal en el rack de canales y la de una pista en la lista
  de reproducción: una luz que no está verde está silenciada.
- **Audio no disponible**, y un motivo: la aplicación no pudo abrir una
  tarjeta de sonido. Conecta los auriculares o los altavoces, cierra
  cualquier programa que pueda estar ocupando la tarjeta de sonido, y
  pulsa **Reintentar** en **Edición > Ajustes… > Audio**.

La aplicación suena por la salida predeterminada del sistema. Si
conectas auriculares mientras funciona, o cambias la salida
predeterminada, pulsa **Reiniciar audio** en **Edición > Ajustes… >
Audio** para llevar el sonido allí.

## Los instrumentos suenan como sintetizadores simples

Es el Lineup Osc, el sintetizador propio de la aplicación, que toca
hasta que eliges un SoundFont. Si elegiste **Ahora no** cuando la
aplicación ofreció GeneralUser GS, o la descarga falló, abre **Edición >
Ajustes… > Audio** y marca **Instrumentos del SoundFont**: la oferta
vuelve. **Elegir SoundFont…**, en la misma página, elige un archivo
`.sf2` que ya tengas (ver [Sonidos](sounds.md)). Si la descarga falla,
la ventana dice por qué; puedes descargar `GeneralUser-GS.sf2` con un
navegador desde [su página](https://github.com/mrbumpy409/GeneralUser-GS)
y elegirlo.

## El sonido se entrecorta

Una computadora lenta u ocupada puede quedarse atrás al tocar un arreglo
grande, como una demostración orquestal con cada instrumento de la
partitura. Cierra otros programas, y las ventanas que no necesites
mientras suena. **Archivo > Exportar WAV…** nunca se ve afectado: arma
el archivo aparte de lo que oyes, nota por nota, así que el archivo
nunca se entrecorta (ver [Exportar](exporting.md#un-archivo-de-sonido)).

## MIDI

- **El menú MIDI dice MIDI no disponible.** El sistema no le dio nada
  de MIDI a la aplicación. En Linux, el MIDI necesita el secuenciador
  ALSA: el módulo del kernel `snd-seq`. Cárgalo con `sudo modprobe
  snd-seq` y vuelve a abrir la aplicación.
- **Mi teclado no está en la lista.** El menú MIDI busca teclados cada
  vez que se abre: conecta el teclado, enciéndelo y vuelve a abrir el
  menú. Un teclado que otro programa tomó para sí puede no aparecer.
- **Toco, pero no suena nada.** Lo que tocas suena en el canal
  seleccionado: haz clic en el nombre de un canal en el rack de
  canales. Revisa que ese canal no esté silenciado.
- Ver [Teclados y sintetizadores MIDI](midi.md).

## Tomas

- **«No se pudo grabar una toma: no se encontró ningún micrófono».** El
  sistema no tiene ninguna entrada desde la que grabar: enchufa un
  micrófono o unos auriculares con micrófono, o elige uno como entrada
  en los ajustes de sonido del sistema, y vuelve a hacer clic en el
  botón del micrófono.
- **En un Mac, la toma sale en silencio.** Ferrous Lineup necesita
  permiso para usar el micrófono: actívalo en Ajustes del Sistema >
  Privacidad y seguridad > Micrófono.
- **También se escriben las notas de la banda**, o notas que no
  cantaste: el micrófono oye los altavoces. Usa auriculares.
- **Las notas caen un poco tarde o temprano.** Ajusta
  **Ajustes > Audio > Micrófono > Retrasar las tomas en** (ver
  [Tomas](takes.md#si-las-notas-caen-tarde-o-temprano)).
- **Tu toma dice «No se oyeron notas: solo se puede conservar la
  grabación».** El micrófono te oyó demasiado bajo, o nada: revisa su
  nivel en los ajustes de sonido del sistema, y canta más cerca de él.

## Las teclas no funcionan

- **Espacio no reproduce**, y las otras teclas no hacen nada: un campo
  de texto tiene el teclado, como la búsqueda de la Biblioteca o un
  compás de la hoja de acordes en el que estás escribiendo. Haz clic
  primero en un lugar vacío.
- **Las letras tocan notas** en lugar de hacer lo que esperas: **MIDI >
  Teclado del equipo como piano** está activado.
- **En un Mac**, los atajos usan **⌘** donde este manual escribe Ctrl.
  Ver [Atajos de teclado](shortcuts.md).

## Guardar y archivos

- **Guardar pide un archivo nuevo cada vez.** Abriste una demostración,
  que es de solo lectura: guárdala una vez con un nombre tuyo, y
  **Guardar** escribe en ese archivo desde entonces (ver [Guardar, abrir
  y recuperar](saving.md#las-demostraciones-son-de-solo-lectura)).
- **La aplicación se cerró sin guardar.** Vuelve a abrirla: ofrece
  recuperar el trabajo sin guardar (ver [Recuperar el trabajo sin
  guardar](saving.md#recuperar-el-trabajo-sin-guardar)).
- **En un Mac, el doble clic en un archivo .lineup no lo abre.** Usa
  **Archivo > Abrir…** en la aplicación.

## La banda y los acordes

- **La banda toca algo distinto de lo que escribí.** Cuando cambias los
  acordes o el estilo, la banda vuelve a escribir sus partes. Para
  conservar una parte tal como está, bloquea su pista (ver [Bloquear una
  pista y tocar tu parte](locks.md)).
- **Los acordes de un archivo MIDI importado están mal.** La aplicación
  los adivina a partir de las notas, y puede equivocarse donde la música
  es escasa o va rápido. Corrígelos en la hoja de acordes (ver [Abrir
  archivos MIDI y hojas de acordes](importing.md#corregir-los-acordes)).
- **Los nombres de los acordes no son los que esperaba**, como C en
  lugar de Do, o al revés: **Edición > Ajustes… > Armonía >
  Nomenclatura de notas** elige letras o solfeo.

## La ventana

- **Todo se ve demasiado pequeño, o demasiado grande.** Deja el puntero
  sobre los menús o la barra de herramientas y pulsa **Ctrl++** o
  **Ctrl+−**, o cambia **Tamaño de la interfaz** en **Edición >
  Ajustes… > Apariencia**. **Ctrl+0** sobre cualquier cosa la devuelve
  al 100% (ver [Deshacer, zoom, ventanas y ajustes](settings.md#zoom)).
- **Desapareció una ventana.** Su tecla la trae de vuelta: de **F5** a
  **F9**, o **Ver**. Una ventana plegada hasta su título se vuelve a
  abrir con el **▼** de arriba a la izquierda.
- **Empezar de cero.** La aplicación guarda sus ajustes, y dónde estaban
  sus ventanas, en el archivo `app.ron` de su carpeta de datos (ver
  [Guardar, abrir y recuperar](saving.md#recuperar-el-trabajo-sin-guardar)
  para saber dónde está). Con la aplicación cerrada, cambia el nombre de
  ese archivo o bórralo, y el siguiente inicio es como el primero: en
  inglés, con los ajustes predeterminados, las ventanas donde se abren
  la primera vez y sin canciones recientes. Tus canciones, tus estilos y
  un SoundFont descargado no se tocan; si habías elegido otra carpeta
  para tus estilos, vuelve a elegirla con **Carpeta de estilos…** en
  **Edición > Ajustes… > Biblioteca**.

## Informar de un problema

**Ayuda > Acerca de Ferrous Lineup** tiene **Informar de un problema**,
que abre la lista de problemas conocidos del proyecto, donde puedes
buscar el tuyo o describir uno nuevo. Di qué versión usas (la ventana
Acerca de la muestra) y en qué sistema, y qué hiciste, paso a paso, para
que otra persona pueda ver lo mismo. Para un problema con lo que toca la
aplicación, activa **Edición > Ajustes… > Reproducción > Escribir un
registro mientras suena**, toca el pasaje y adjunta el archivo del
registro: tiene cada nota que tocó la aplicación, y el ajuste dice dónde
lo escribe.
