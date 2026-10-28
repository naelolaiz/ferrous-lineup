# Resolución de problemas

La aplicación suele decir qué ha ido mal en la barra de estado, abajo de
la ventana. Empieza por ahí, y luego busca el síntoma de abajo.

| Síntoma | Qué hacer |
|---|---|
| **Sin sonido**, o la barra de estado dice **Audio unavailable** | Abre **Edit > Settings…**, elige **Audio**, y haz clic en el botón bajo **Output**: **Restart audio**, o **Try again** después de un fallo. La aplicación usa el dispositivo de salida por defecto del sistema, así que comprueba también ese dispositivo y su volumen. |
| **Una nota se queda colgada en un sintetizador MIDI externo** | Pulsa **Space** para detener. Si sigue sonando, abre el menú **MIDI**, elige **None** bajo **Output: an external synth to play**, y luego vuelve a elegir el puerto. Cerrar la conexión antigua envía All Notes Off en los 16 canales. |
| **"MIDI unavailable"** en el menú **MIDI** | En Linux la aplicación necesita el sequencer de ALSA: el módulo del kernel `snd-seq` y `/dev/snd/seq`. Prueba `sudo modprobe snd-seq`, y luego vuelve a abrir el menú MIDI. En Windows y macOS, comprueba que el teclado o el sintetizador está conectado y su driver instalado. |
| **"Recover unsaved work?"** al arrancar | La aplicación encontró trabajo sin guardar de una ejecución que no se cerró limpiamente. **Recover it** abre esa copia; **Discard it** la borra. Guarda la canción recuperada con un nombre nuevo antes de cerrar. El primer cambio escribe una copia al momento, y luego la aplicación escribe cada 30 segundos mientras haya cambios sin guardar. |
| **Sin aviso de recuperación** después de un cierre inesperado, aunque había cambios sin guardar | Puede que la aplicación no pueda escribir en su carpeta de datos. Ahí viven las copias de recuperación y las preferencias, por defecto en `~/.local/share/ferrouslineup/` en Linux, `~/Library/Application Support/Ferrous-Lineup/` en macOS, y `%APPDATA%\Ferrous Lineup\data\` en Windows. Hazla escribible. Guardar un proyecto en otro sitio sigue funcionando. |
| **"The recording is longer than 10 minutes"** | **File > Import recording…** rechaza el audio de más de diez minutos, porque el análisis mantiene la grabación en memoria. Recórtala en un editor de audio e importa el archivo más corto. |
| **Una exportación WAV o de vídeo fallida** | La aplicación escribe `song.wav.tmp` o `song.mp4.tmp` y solo lo renombra cuando la exportación está completa, de modo que el archivo antiguo no se reemplaza. Libera espacio en disco y vuelve a intentarlo. Una exportación fallida puede dejar su archivo `.tmp`; una cancelada lo elimina. |
| **macOS no abre la aplicación** | La aplicación está firmada ad-hoc pero sin notarizar. Intenta abrirla una vez, y luego permítela en **System Settings > Privacy & Security**. En versiones antiguas de macOS, haz clic derecho en la aplicación y elige **Open**. |
| **Windows SmartScreen bloquea el instalador** | El instalador no está firmado con certificado de código. Elige **More info**, y luego **Run anyway**. |
| **El AppImage no arranca (error de FUSE)** | Instala el runtime de FUSE 2 (`libfuse2`, o `libfuse2t64` en Ubuntu más reciente), o ejecuta el AppImage con `--appimage-extract-and-run`. |
| **Un archivo `.lineup` no se abre con doble clic en macOS** | El bundle de macOS no declara un tipo de documento `.lineup`, así que Finder no tiene ninguna asociación para él. Abre el archivo desde **File > Open…** en la aplicación en su lugar. El instalador de Windows y el paquete de Linux sí declaran el tipo. |
| **No hay paquete Flatpak ni AUR** | Los manifiestos están en `packaging/flathub` y `packaging/aur`, pero todavía no se publican en Flathub ni en el AUR. Usa el AppImage, el `.deb` o el `.tar.gz` de la release, o compila desde el código fuente. |

## Si la propia aplicación se detiene

Si la ventana se cierra inesperadamente, vuelve a arrancarla: ofrece la
copia de recuperación descrita arriba. **File > Open recent** enumera
los últimos diez proyectos. Si la tarjeta de sonido se detuvo pero la
ventana sigue abierta, **Edit > Settings… > Audio > Restart audio** la
arranca de nuevo sin reabrir la aplicación.
[Recuperación, proyectos recientes y reiniciar el audio](step-9/recovery.md)
tiene los detalles, y
[Una traza de la reproducción](step-9/playback-trace.md#notas-que-no-deben-perderse)
describe la garantía de note-off desde el lado del código.

Si nada de esto ayuda, el mensaje de la barra de estado, la versión de
la aplicación y tu sistema operativo son lo que necesita un informe de
incidencia. Las incidencias del proyecto están en
<https://github.com/naelolaiz/ferrous-lineup/issues>.
