# Paso 7: MIDI de entrada y salida

MIDI es cómo se hablan entre sí los teclados, los sintetizadores y los
programas de música. Es el mismo desde 1983: unos pocos bytes que dicen
«esta tecla bajó, con esta fuerza» o «esta tecla subió». El paso 7
enseña a Ferrous Lineup a hablarlo, en ambas direcciones:

- **tocar desde un teclado MIDI**: elígelo en **MIDI > Entrada**, y sus
  teclas tocan el *canal seleccionado*. Es el canal que muestra el piano
  roll, resaltado en el channel rack; haz clic en el nombre de un canal
  para seleccionarlo,
- **el teclado de escritura**: ¿no tienes un teclado MIDI a mano?
  Activa **MIDI > Teclado de escritura como piano**, y las teclas de
  letras tocan notas, dispuestas como un piano (más abajo),
- **acordes nombrados mientras tocas**: mantén pulsadas algunas teclas,
  y la barra de estado nombra el acorde («Am: La menor, acorde vi de Do
  mayor»). En el piano roll, las teclas se iluminan en el teclado, y los
  botones de acordes se iluminan con los acordes que suelen venir
  *después del que estás manteniendo*,
- **grabación**: haz clic en **● Rec**, pulsa Play y toca. Cada tecla
  se convierte en una nota en el paso más cercano del patrón
  seleccionado, y una toma entera es un solo paso de deshacer,
- **canales MIDI Out**: **Añadir canal > MIDI Out (sintetizador
  externo)** crea un canal que no suena por sí mismo. Sus notas van a
  otro sintetizador, uno de hardware u otro programa, elegido en **MIDI
  > Salida**. Haz clic derecho para elegir su canal MIDI, del 1 al 16,
- **archivos MIDI**: **File > Export song as MIDI…** y
  **Export pattern as MIDI…** escriben un archivo `.mid` con las
  notas, que cualquier otro estudio puede abrir. La batería integrada va
  en el canal 10 con las notas que esperan las baterías de General MIDI,
  así que allí también salen como batería.

![Manteniendo La menor en el teclado de escritura. Las tres teclas se iluminan en naranja, la línea sobre la cuadrícula nombra el acorde, y los acordes que suelen seguirlo están resaltados.](../images/step-7-held.png)

![Una toma de Do, La menor, Fa y Sol, un acorde por pulso, grabada con el teclado de escritura mientras un patrón de un compás se repetía. Los acordes entraron un poco tarde, y el Sol se pasó a la siguiente vuelta, así que aparece en los dos extremos: la grabación mueve cada nota al paso más cercano, no a donde la querías.](../images/step-7-take.png)

![Un canal MIDI Out en el channel rack, y su menú de canal MIDI](../images/step-7-midi-out.png)

## Cómo usarlo

| Haz esto | Para |
|----------|------|
| MIDI > Entrada > un puerto | tocar el canal seleccionado desde un teclado MIDI |
| MIDI > Salida > un puerto | enviar los canales MIDI Out a un sintetizador |
| MIDI > Teclado de escritura como piano | tocar notas con las teclas de letras |
| clic en el nombre de un canal | oírlo, y seleccionarlo para los teclados y el piano roll |
| ● Rec, luego Espacio | grabar en el patrón seleccionado mientras se repite |
| Espacio de nuevo | detener; las teclas que siguen pulsadas terminan ahí |
| Ctrl+Z | deshacer toda la toma |
| Añadir canal > MIDI Out | añadir un canal para un sintetizador externo |
| clic derecho en un canal MIDI Out > Canal MIDI | elegir su canal, del 1 al 16 |
| File > Export song as MIDI… | guardar las notas de la canción en un archivo `.mid` |
| File > Export pattern as MIDI… | guardar las notas del patrón seleccionado |

El teclado de escritura usa dos filas de letras para las teclas
blancas, y la fila de encima de cada una para las teclas negras, como
las teclas negras de un piano se sientan entre las blancas y por encima
de ellas:

```text
teclas negras:   S  D     G  H  J      2  3     5  6  7     9  0
teclas blancas: Z  X  C  V  B  N  M   Q  W  E  R  T  Y  U  I  O  P
notas:          C4 D4 E4 F4 G4 A4 B4  C5 D5 E5 F5 G5 A5 B5 C6 D6 E6
```

Las teclas se encuentran por su posición en el teclado, no por la letra
impresa en ellas, así que el piano tiene la misma forma en una
distribución francesa o alemana. Ninguna de estas teclas hacía nada
antes, así que nada se pierde mientras el teclado de escritura está
activo: Espacio, L, F6, F7, las teclas de flecha y todos los atajos con
Ctrl siguen funcionando como antes.

**Rec** cambia a modo patrón, porque una toma va al patrón que se
repite. Grabar en modo canción es el [ejercicio 5](exercises.md).

## Cómo se probó

Este paso se construyó en una máquina virtual sin sistema MIDI: en
Linux, el MIDI pasa por el *secuenciador* de ALSA, que necesita el
dispositivo `/dev/snd/seq`, y esa máquina no lo tenía. Así que allí, el
menú MIDI dice **MIDI no disponible**:

![El menú MIDI en un equipo sin sistema MIDI](../images/step-7-midi-menu.png)

Qué se probó en la aplicación en marcha:

- el teclado de escritura: las notas suenan mientras se mantienen y se
  detienen al soltarlas (comprobado grabando el sonido de la
  aplicación), y el acorde se nombra,
- grabar tomas, una tecla aún pulsada cuando se detiene la
  reproducción, y deshacer una toma,
- canales MIDI Out: añadir uno, cambiar su canal MIDI, y los mensajes
  que el hilo de audio genera para él (en las pruebas),
- archivos MIDI: los archivos exportados se leyeron de vuelta con un
  programa aparte, la librería `mido` de Python, que encontró las
  pistas, notas, canal de batería, tempo y longitud correctos.

Para ser exactos: se probaron los viajes de ida y vuelta de los
mensajes y de los archivos, pero no un teclado o sintetizador MIDI de
verdad. La causa son el `/dev/snd/seq` que faltaba en la máquina de
construcción, y el menú **MIDI no disponible** que resultó de ello. La
ruta del hardware es pequeña, y todo lo que hay después del primer byte
se comparte con el teclado de escritura o está cubierto por pruebas. Se
compila para Linux, macOS y Windows, pero no ha conocido hardware real.
Si pruebas un teclado o sintetizador MIDI, será bienvenido un informe
de lo que funcionó —el puerto, el sistema operativo y lo que pasó—;
por favor [abre un issue](https://github.com/naelolaiz/ferrous-lineup/issues).

## Qué cambió en la arquitectura

```text
  lineup-core
    midi.rs (nuevo):      MidiMessage, note-on y note-off, a bytes y de vuelta
    midi_file.rs (nuevo): song_to_midi y pattern_to_midi: Standard MIDI Files
    record.rs (nuevo):    recorded_note: una tecla pulsada se vuelve una nota de la cuadrícula
    project.rs:           ChannelKind::MidiOut { channel }
    sequencer.rs:         position(): dónde está la reproducción, con fracción de paso
    file.rs:              formato 2, escrito solo por proyectos que lo necesitan

  lineup-audio
    instrument.rs:  notas sostenidas y note_off; el instrumento MidiOut
    engine.rs:      las colas LiveInput y MidiOutput, Command::Live,
                    la posición de reproducción compartida

  lineup-app
    midi.rs (nuevo):      midir: puertos, el callback de entrada, el hilo de envío
    app/live.rs (nuevo):  el menú MIDI, el teclado de escritura, teclas pulsadas, grabación
    app/export.rs:        exportar MIDI
    channel_rack.rs:      el canal seleccionado, los canales MIDI Out
    piano_roll.rs:        teclas pulsadas, sugerencias a partir de las teclas
```

Cuatro hilos se reparten ahora el trabajo. Una tecla de un teclado MIDI
recorre este camino:

```text
  teclado MIDI
     │ 3 bytes
     ▼
  hilo de midir: on_message ──▶ LiveInput (ring buffer) ──▶ hilo de audio ──▶ tarjeta de sonido
     │                                                          │
     │ una copia, con la posición de reproducción (canal mpsc)  │ mensajes de los canales MIDI Out
     ▼                                                          ▼
  hilo de la UI: teclas pulsadas, nombre del acorde, grabación  MidiOutput (ring buffer)
     │                                                          │
     │ teclado de escritura: Command::Live ──▶ hilo de audio    ▼
                                                    hilo de envío ──▶ puerto MIDI ──▶ sintetizador
```

Cuatro ideas sostienen este paso:

1. **Las teclas se saltan la UI.** La UI dibuja una vez por frame, y
   una tecla podría esperar allí un frame entero antes de que pase
   nada. Así que una tecla va del hilo de MIDI directa al hilo de
   audio, y suena en un bloque de audio, unos pocos milisegundos. La UI
   recibe una copia, para mostrarla y grabarla.
2. **El hilo de audio sigue sin esperar nunca.** Enviar un mensaje MIDI
   es una llamada al sistema, que puede tardar lo que sea. Así que el
   hilo de audio deja los mensajes en una cola, y un hilo propio los
   envía.
3. **Un solo reloj para todo.** El hilo de audio publica dónde está en
   el patrón, con fracciones de paso. Cada tecla se marca con ello al
   llegar, y la grabación solo tiene que redondear.
4. **El formato del archivo solo se mueve cuando hace falta.** Un
   proyecto con un canal MIDI Out se escribe como formato 2, que las
   versiones antiguas de la aplicación rechazan. Cualquier otro
   proyecto se sigue escribiendo como formato 1, para que puedan
   abrirlo.

## Archivos de este paso

```text
crates/lineup-core/src/midi.rs           MidiMessage, DRUM_CHANNEL, all_notes_off (nuevo)
crates/lineup-core/src/midi_file.rs      exportar Standard MIDI Files (nuevo)
crates/lineup-core/src/record.rs         recorded_note (nuevo)
crates/lineup-core/src/project.rs        ChannelKind::MidiOut, Channel::midi_out
crates/lineup-core/src/sequencer.rs      Sequencer::position
crates/lineup-core/src/file.rs           FORMAT_VERSION 2 y format_for
crates/lineup-audio/src/instrument.rs    HELD, note_off, MidiOut
crates/lineup-audio/src/engine.rs        LiveInput, MidiOutput, Command::Live, live_keys
crates/lineup-audio/src/render.rs        el hilo de audio de la exportación no tiene MIDI
crates/lineup-app/src/midi.rs            el envoltorio de midir (nuevo)
crates/lineup-app/src/app/live.rs        teclado de escritura, teclas pulsadas, grabación (nuevo)
crates/lineup-app/src/app/export.rs      exportar MIDI
crates/lineup-app/src/app/files.rs       los elementos MIDI del menú File
crates/lineup-app/src/app/edit.rs        una toma es un paso de deshacer
crates/lineup-app/src/app.rs             el menú MIDI, Rec, el canal seleccionado
crates/lineup-app/src/channel_rack.rs    selección, canales MIDI Out
crates/lineup-app/src/piano_roll.rs      teclas pulsadas, sugerencias a partir de las teclas
```

Los capítulos siguen a una tecla en su recorrido. Primero los bytes en
sí, y el archivo MIDI que se hace con ellos. Luego el viaje por los
hilos, de entrada y de salida. Por último, la grabación, donde una
tecla se convierte en una nota.
