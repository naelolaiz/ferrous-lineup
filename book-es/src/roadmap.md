# Hoja de ruta

| Paso | Estado | Contenido |
|------|--------|----------|
| 1. Fundamentos | Hecho | Workspace de Cargo, motor de audio `cpal`, ventana `egui`, tono de prueba, pruebas, CI de Linux, este libro |
| 2. Step sequencer | Hecho | Channel rack, step sequencer con precisión de muestra, batería sintetizada, sampler de WAV con arrastrar y soltar, sintetizador Lineup Osc |
| 3. Piano roll y armonía | Hecho | Editor de piano roll, notas con altura y duración, resaltado de escala, estampado de acordes, nombres de acordes y números romanos en vivo, sintetizador polifónico, samplers con altura |
| 4. Playlist y guardado | Hecho | Muchos patrones, el playlist con pistas y clips, modo patrón y modo canción, archivos de proyecto `.lineup`, compilaciones de macOS y Windows en CI |
| 5. Deshacer y exportar | Hecho | Deshacer y rehacer para cada cambio, un único camino basado en diffs hacia el hilo de audio, una caché de muestras, exportación WAV en segundo plano |
| 6. Herramientas de armonía | Hecho | Seleccionar y mover grupos de notas, un menú Tools (transponer, mover en la tonalidad, ajustar a la tonalidad, invertir, arpegiar), inversiones estampadas, siete tipos de acordes más, sugerencias del siguiente acorde con dominantes secundarias, notas fantasma de los demás canales, un carril de velocidad |
| 7. MIDI | Hecho | Teclados MIDI de entrada, sintetizadores externos de salida (canales MIDI Out), un teclado de escritura, acordes con nombre mientras tocas, grabación de tomas, exportación `.mid` de canciones y patrones, formato de archivo 2 |
| 8. Harmony keys y biblioteca | Hecho | Las harmony keys (los siete acordes de la tonalidad, coloreados por función, con sugerencias del siguiente acorde y el acorde que suena), compases, 78 estilos de armonía y 49 ritmos en 13 géneros incluido el tango, 16 canciones de dominio público, demos, nueve baterías nuevas, patrones de hasta dieciséis compases, formato de archivo 3 |
| 9. La banda | Hecho | Nombres de acordes y acordes con barra, un chord sheet que la banda toca, una barra de práctica, sonidos SoundFont, inversiones de acordes, entrenamiento auditivo, la forma de la canción, variaciones, swing, chord sheets como texto, un lead sheet, una banda más grande, shots, holds, silencios y pushes, y un solista experimental; después paquetes de release (AppImage de Linux, `.deb` y `.tar.gz`; Setup de Windows y `.zip`; `.dmg` de macOS para Apple Silicon e Intel), métricas irregulares, un mixer en orden de partitura, un mixer con paneo, un envío de reverb y medidores, exportación de vídeo, una traza de la reproducción, pistas bloqueadas, un estilo para cada sección, el editor de estilos, muchas más canciones, una biblioteca de 177 estilos de armonía y 112 ritmos, recuperación tras un cierre inesperado, proyectos recientes y reiniciar el audio, acordes de fuera de la tonalidad explicados (acordes prestados, dominantes secundarias, acordes pivote), cómo se mueve cada parte de un acorde a otro, una sección de vientos para cualquier estilo, EQ, compresión y delay en cada canal, y este libro en línea y como PDF con índice, un índice de palabras y marcadores; mira el [paso 9](step-9/README.md) |

Ferrous Lineup es una banda de acompañamiento con la que puedes
aprender armonía: escribe los acordes de una canción, elige un estilo, y
la banda la toca, mientras la aplicación pone nombre a cada acorde que
oyes. Funciona en Linux, macOS y Windows, usando el mismo código en los
tres.

Los pasos 1 a 8 construyeron aquello sobre lo que se apoya la banda. Los
pasos 1 a 4 hicieron el sonido, los patrones y las pistas de una
canción, y el paso 5 añadió deshacer y la exportación WAV. El paso 6 dio
al piano roll su ayuda de armonía. El paso 7 nació de una pregunta:
«¿tiene MIDI, de entrada y de salida?». Ahora lo tiene, en ambos
sentidos, y pone nombre al acorde que mantienes pulsado. Se construyó en
una computadora sin sistema MIDI, así que todavía no se ha probado con
hardware MIDI real; el
[paso 7](step-7/README.md#cómo-se-probó) dice qué se probó. El paso
8 añadió las harmony keys, una para cada acorde de la tonalidad, y una
biblioteca de estilos, ritmos y canciones: el conocimiento con el que
toca la banda. En el paso 8 el acompañamiento ya podía seguir los
acordes que tocas, como hace un teclado arreglista. El paso 9 puso a la
banda al frente, con un chord sheet donde escribir la canción.

Algunas ideas de la hoja de ruta inicial se cumplieron por el camino: un
mixer, la importación de archivos MIDI, notas que empiezan entre pasos
(swing, pushes, guitarras rasgueadas, MIDI importado), cambios de compás
a mitad de una canción, y descargas instalables para Linux, Windows y
macOS, con un AppImage y un `.deb` para Linux.

## Ideas para lo que viene después

Nada de esto está planificado todavía. Son ideas, más o menos de menor a
mayor:

- **Más banda**: más estilos, y más instrumentos que pueda traer a
  cualquiera de ellos, como ahora trae los vientos.
- **Un solista que ya no sea un experimento.**
- **Automatización**: movimientos de mandos grabados o dibujados a lo
  largo del tiempo, sobre las pistas.
- **Clips de audio** en las pistas, junto a los patrones, para tocar a
  la vez que una grabación.
- **Grabar en modo canción** (ejercicio 5 del paso 7).
