# Ejercicios

Varios de estos se construyeron más adelante en el libro. Donde eso pasó,
el ejercicio empieza ahora señalando el código que lo hace, y después pide
algo que todavía falta. Eso también es buena práctica: leer una
implementación que funciona suele ser la forma más rápida de aprender la
forma de un cambio.

## 1. Borrar todas las pistas vacías (lineup-core y lineup-app, fácil)

Borrar una pista ya está implementado: lee `Project::remove_track` y el
elemento «Borrar pista» del menú de clic derecho de la pista. Quita la
pista y sus clips, y sube uno los clips de las pistas de debajo, igual que
hace `remove_pattern` con los patrones.

Lo que falta es una forma de quitar *todas* las pistas vacías a la vez.
`remove_unused_tracks` solo descarta las pistas vacías del fondo de la lista de reproducción. Añade `remove_empty_tracks`, que quita una pista esté donde esté
si no tiene clips, y un comando en el menú de la pista. ¿Qué índices debes
visitar, y en qué orden, para que los clips sigan apuntando a las pistas
correctas? ¿Qué debería pasar si eso no dejara ninguna pista? Escribe
primero una prueba con la canción de demostración, y después el método.

*Prácticas:* `retain`, mantener reglas que abarcan dos listas, probar el
orden de los cambios de índice.

## 2. Alt-arrastrar para copiar un clip (lineup-app, fácil)

En los estudios basados en patrones, arrastrar un clip con un modificador
pulsado lo copia en lugar de moverlo. `Ctrl+D` ya duplica los clips
seleccionados, pero arrastrar una copia es más rápido cuando la quieres en
otro sitio.

Mayús está ocupado: Mayús-clic añade un clip a la selección. Usa Alt para
la copia. En `PlaylistView::handle_input`, lee las teclas modificadoras
con `ui.input(|input| input.modifiers.alt)`. El plan obvio es añadir una
copia al pulsar y arrastrar la copia. Pero `add_clip` rechaza un duplicado
exacto, así que no se puede añadir una copia encima del original. En su
lugar, recuerda en `ClipDrag` que este arrastre copia, y añade la copia
solo cuando el puntero llegue a otra celda. ¿Qué índice recibe el clip
nuevo en `Project::playlist().clips()`, y cómo se entera el arrastre?

*Prácticas:* leer la entrada de egui, reutilizar acciones que ya tienes.

## 3. Un selector de color de patrón (lineup-app, medio)

Los colores de los patrones ya se guardan. `Pattern` tiene un `color`
público (un índice en la paleta de `playlist.rs`), `PatternFile` lo
escribe, y los archivos guardados antes de que existiera cargan con el
lugar del patrón como su color. Lee `PatternFile` y `pattern_color`.

Lo que falta es el selector. Añade uno al menú ⋯ de patrones, con un
cuadrado pequeño para cada color de la paleta, y una `PlaylistAction` que
ponga el color del patrón seleccionado a través de `edit`. Después añade
«Restablecer colores», que le devuelve a cada patrón su lugar en la lista.
¿Cambiar un color necesita un `FORMAT_VERSION` nuevo? ¿Qué hace un Ferrous
Lineup más viejo con un archivo que lo tiene?

*Prácticas:* evolucionar un formato de archivo, `#[serde(default)]`,
encaminar un cambio de UI a través de `edit`.

## 4. Olvidar un proyecto reciente (lineup-app, medio)

Archivo > Abrir recientes ya recuerda los últimos diez proyectos, en los
datos persistidos de egui, y se mantiene correcto cuando un archivo se ha
movido o borrado (la apertura falla y la ruta se retira). Lee
`app/recent.rs`.

Amplíalo: un elemento «Borrar recientes» que olvida la lista entera, y una
✕ pequeña junto a cada entrada que olvida una. La ✕ no debe abrir también
el archivo, así que vigila el orden en el que compruebas los dos clics.
¿Qué debería pasarle al menú mientras corre una exportación? ¿Por qué se
guarda la lista como `Vec<String>` en lugar de `Vec<PathBuf>`?

*Prácticas:* menús de egui, persistencia, pensar en rutas que no son
UTF-8.

## 5. Buscar sin el ratón (lineup-app, medio)

Hacer clic en la regla de la lista de reproducción ya mueve el cursor de reproducción: envía
`PlaylistAction::Seek`, la app lo convierte en `Command::Seek(step)`, y el
motor mueve allí el step sequencer. Lee `Command::Seek` y
`LineupApp::seek`.

Añade las teclas que suele tener un transporte: Inicio va al principio, Fin al final de la canción, y RePág / AvPág mueven un compás. Después
añade una pequeña caja de «compás» junto al indicador de posición, donde
escribir un número de compás y pulsar Intro busca allí. ¿Qué ventana
debería ser dueña de estas teclas, y cómo evitas que se disparen mientras
un campo de texto tiene el teclado? ¿Qué debería hacer ▶ tras una
búsqueda: empezar desde la posición buscada, o desde el principio?

*Prácticas:* atajos de teclado, convertir compases en pasos, decidir el
comportamiento antes de escribir código.

## 6. Exportar la canción como archivo WAV (lineup-audio, más difícil)

*El paso 5 construye esto; inténtalo tú mismo antes de leer [su
capítulo](../step-5/export.md). Si estás leyendo el libro terminado, el
código ya está ahí: lee `render.rs` y `app/export.rs`, y después compara
tu diseño con él.*

Las pruebas del motor ya reproducen canciones sin tarjeta de sonido: crean
un `AudioThread`, le envían comandos y llaman a `process` sobre un búfer.
Usa la misma idea para Archivo > Exportar WAV. Construye un `AudioThread`
separado en el hilo de UI (nunca se acerca al real), envíale el proyecto
entero como hace `send_whole_project`, después `SetMode(Song)` y `Play`, y
llama a `process` bloque a bloque durante `song_length` pasos, escribiendo
las muestras con `hound::WavWriter`. Añade un segundo o dos al final para
que las últimas notas puedan resonar.

Cosas que resolver: ¿cómo envías comandos si `AudioThread::new` quiere el
extremo consumidor de un ring buffer? ¿Qué partes de `engine.rs` deben
volverse públicas, y puedes mantenerlas `pub(crate)` poniendo la función
de exportación en `lineup-audio`? ¿Cuánto tarda exportar una canción de
tres minutos, y debería la ventana seguir dibujándose mientras corre?

*Prácticas:* reutilizar código de tiempo real en offline, diseño de API
entre crates, `hound`.

## 7. Piénsalo

- Los clips se refieren a los patrones por su *posición* en la lista, por
  eso `remove_pattern` tiene que renumerarlos. Imagina en su lugar que cada
  patrón tiene un id permanente (`struct PatternId(u32)`) que nunca
  cambia. ¿Qué métodos de `Project` se vuelven más simples, y cuáles más
  difíciles? ¿Qué cambiaría en el archivo, y en el `ChannelNotes` del hilo
  de audio? ¿Una función de deshacer sería más fácil con ids o con
  posiciones?
- `has_unsaved_changes` compara el proyecto entero con la copia guardada,
  pero solo después de que una edición o un deshacer ponga `check_dirty`,
  no en cada frame. ¿Cuántas notas necesitaría aproximadamente un proyecto
  para que una comparación ocupara una parte apreciable de un frame (unos
  16 milisegundos)? ¿Por qué `project_edited` pone `check_dirty` solo
  cuando `dirty` ya es falso? ¿Qué añadiría a este diseño un contador que
  cada edición incrementa?
- El archivo de proyecto tiene sus propios tipos (`ProjectFile`,
  `PatternFile`) en lugar de derivar `Serialize` sobre el propio
  `Project`. ¿Qué iría mal el día que alguien renombre un campo de
  `Project`, si el archivo lo usara directamente?
- Un Ferrous Lineup más viejo se niega a abrir un archivo con un número de
  formato más nuevo. ¿Podría en su lugar cargar lo que entiende e ignorar
  el resto? ¿Cuándo sería útil eso, y cuándo perdería silenciosamente el
  trabajo de alguien?
- La UI envía la duración de la canción junto con sus piezas, en lugar de
  dejar que el hilo de audio la calcule. ¿Qué haría falta para mantener
  dos copias de ese cálculo de acuerdo, y cómo te darías cuenta de que han
  divergido? Desde el paso 8, las piezas son `Play`s hechos por
  `Project::playback_arrangement`; ¿por qué desenrollar las repeticiones en
  el lado de la UI es más simple que enseñarle al hilo de audio lo que son
  las secciones?
