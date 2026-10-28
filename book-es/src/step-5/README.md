# Paso 5: Deshacer, rehacer y exportar la canción

El paso 5 añade dos cosas que todo estudio necesita y que echas de menos
en cuanto has escrito algo que te gusta:

- **deshacer y rehacer** para cada cambio del proyecto: pasos, notas,
  acordes, clips, pistas, canales, muestras, patrones, el tempo, los
  nombres y la tonalidad. El menú Edit dice qué se va a deshacer ("Undo
  move clip"). Un arrastre es un solo paso, dure los fotogramas que dure,
  y escribir un nombre también es un solo paso. El historial guarda entre
  10 y 100 pasos, según la canción: las canciones pequeñas guardan 100, y
  una grande guarda menos para mantenerse cerca de un presupuesto de
  256 MiB, pero nunca menos de 10,
- **Export WAV**: File > Export WAV… guarda la canción como un archivo de
  sonido que cualquier reproductor de música, editor de vídeo u otro
  estudio puede abrir: estéreo de 16 bits a 44,100 fotogramas por
  segundo, como un CD. La exportación se ejecuta en segundo plano, con
  una barra de progreso en la barra de estado y un botón Stop. Puedes
  seguir reproduciendo y editando mientras se ejecuta, y nada de lo que
  hagas cambia el archivo que se está escribiendo.

![El menú Edit después de hacer clic en dos pasos: "Undo change steps"](../images/step-5-edit-menu.png)

![La barra de estado mientras se exporta una canción larga](../images/step-5-exporting.png)

![La barra de estado cuando termina la exportación](../images/step-5-exported.png)

## Cómo usarlo

| Haz esto | Para |
|---------|----|
| Ctrl+Z, o Edit > Undo | deshacer el último cambio |
| Ctrl+Shift+Z o Ctrl+Y, o Edit > Redo | rehacer lo deshecho |
| File > Export WAV… | guardar la canción como archivo `.wav` |

En un Mac, usa ⌘ en lugar de Ctrl. Mientras escribes en un campo de
texto (un nombre de patrón o de pista), Ctrl+Z deshace lo que escribes
dentro del campo, como en cualquier otro programa; en cuanto sales del
campo, vuelve a deshacer cambios completos.

Algunas cosas no forman parte del proyecto, así que deshacer no las
toca: si la música se está reproduciendo, el volumen maestro, qué
ventanas están abiertas, el ajuste de snap y qué patrón está seleccionado
(salvo que un patrón recuperado por deshacer queda seleccionado, para que
puedas verlo). Abrir un archivo o empezar uno nuevo olvida el historial.

La exportación reproduce la canción una vez, desde el primer compás
hasta el final del último clip, y luego una cola para que las últimas
notas puedan resonar: dos segundos, más la caída de la reverberación
cuando esta está activada. El archivo toma el nombre del proyecto
("song.lineup" sugiere "song.wav"). Export WAV aparece atenuado cuando la
canción no tiene longitud (ni clips ni secciones).

## Qué cambió en la arquitectura

```text
  lineup-core
    history.rs (nuevo): History<T>, copias del proyecto para deshacer y rehacer
    diff.rs (nuevo):    diff(before, after) -> Vec<Change>

  lineup-app
    app/edit.rs (nuevo):   edit(), undo(), redo(), sync_audio(), el menú Edit
    app/export.rs (nuevo): File > Export WAV, en un hilo propio

  lineup-audio
    render.rs (nuevo): Renderer y export_wav: el hilo de audio, sin tarjeta de sonido
    engine.rs:        InsertPattern e InsertChannel, en cualquier posición
    sample.rs:        SampleCache, para que los archivos de muestra se lean una vez
```

Cada cambio del proyecto sigue ahora el mismo camino:

```text
  clic en un paso
    └▶ edit("change steps", |project| ...)
         el paso aún no está abierto:
           before = project.clone()
           cambia el proyecto
           diff(&before, &project)  ──▶  [Notes { pattern: 0, channel: 3 }]
                                               └▶ Command::SetNotes ──▶ audio
           history.record("change steps", before)   (el paso sigue abierto)

  el siguiente fotograma del arrastre
    └▶ edit("move notes", |project| ...)
         el paso sigue abierto:
           cambia el proyecto, sin copia
           sincroniza como mucho cada 50 ms

  se suelta el botón del ratón
    └▶ cierra el paso y envía sus últimos cambios

  Ctrl+Z
    └▶ undo()
         before = project.clone()
         history coloca el proyecto anterior en su lugar
         diff(&before, &project)  ──▶  el mismo tipo de comandos, al revés
```

Tres ideas sostienen este paso:

1. **Guardar copias, no instrucciones.** El historial guarda copias
   completas del proyecto, no una lista de "cómo revertir lo que hice".
   Una copia no se puede revertir mal. Un arrastre copia una vez, cuando
   abre su paso, no en cada fotograma; el presupuesto de bytes del
   historial decide cuántas copias puede guardar una canción.
2. **Una sola forma de avisar al hilo de audio.** Antes de este paso,
   cada tipo de edición enviaba sus propios comandos al hilo de audio.
   Ahora cada edición, y cada deshacer y rehacer, compara el proyecto
   antes y después y envía las diferencias. Hay un único fragmento de
   código que acertar, y se prueba por separado.
3. **El mismo código, más rápido que el tiempo real.** La exportación no
   tiene su propia forma de producir sonido. Ejecuta exactamente el mismo
   código del hilo de audio que usa la tarjeta de sonido, en un hilo
   propio, tan rápido como pueda ir la computadora, y escribe el resultado
   en un archivo.

## Archivos de este paso

```text
crates/lineup-core/src/history.rs        History<T>: deshacer y rehacer (nuevo)
crates/lineup-core/src/diff.rs           Change y diff(): qué cambió entre dos proyectos (nuevo)
crates/lineup-audio/src/render.rs        Renderer, export_wav y WavExport (nuevo)
crates/lineup-audio/src/engine.rs        InsertPattern, InsertChannel, settle_volumes
crates/lineup-audio/src/sample.rs        SampleCache
crates/lineup-audio/src/instrument.rs    Instrument::for_channel recibe la caché
crates/lineup-audio/src/dsp.rs           Smoothed::settle
crates/lineup-app/src/app/edit.rs        edit, undo, redo, el menú Edit (nuevo)
crates/lineup-app/src/app/export.rs      File > Export WAV (nuevo)
crates/lineup-app/src/app.rs             cada cambio pasa por edit()
crates/lineup-app/src/app/files.rs       el elemento de menú Export WAV; abrir un archivo borra el historial
crates/lineup-app/src/piano_roll.rs      sigue a su canal cuando se inserta uno por encima
```

Los capítulos siguen el orden en que viaja un cambio: primero el
historial que lo recuerda, luego cómo se entera el hilo de audio, y por
último la exportación, que reproduce el resultado y lo escribe en un
archivo.
