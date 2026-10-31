# Índice

[Introducción](introduction.md)

[Compilarlo desde el código fuente](building.md)

# Parte I: La forma del programa

- [Cinco crates, y qué puede saber cada uno](program/crates.md)
- [Dos hilos: la ventana y el audio](program/threads.md)
- [El proyecto: patrones, canales, pistas, clips y marcas](program/project.md)
- [Decirle al hilo de audio lo que ha cambiado](program/sync.md)
- [Deshacer y rehacer](program/undo.md)
- [Guardar: el archivo .lineup, la recuperación y los archivos dañados](program/saving.md)
- [Dibujar las ventanas con egui: distribución, zoom, temas, iconos](program/egui.md)
- [Los editores: rack de canales, piano roll y lista de reproducción](program/editors.md)
- [Dos idiomas: traducciones, y letras o solfeo](program/languages.md)
- [Ejercicios: la forma del programa](program/exercises.md)

# Parte II: El sonido

- [El secuenciador: temporización a la muestra](sound/sequencer.md)
- [Tempo, compás, swing y sensación](sound/tempo.md)
- [Instrumentos: batería, sampler y Lineup Osc](sound/instruments.md)
- [SoundFonts y General MIDI](sound/soundfonts.md)
- [El mezclador: niveles, balance, reverberación, efectos y orden de partitura](sound/mixer.md)
- [MIDI: mensajes, dispositivos, grabación y archivos](sound/midi.md)
- [Exportar: WAV, MIDI y video](sound/export.md)
- [La traza de reproducción](sound/trace.md)
- [Ejercicios: el sonido](sound/exercises.md)

# Parte III: La teoría musical en código

- [Clases de altura, notas y su escritura](theory/pitch.md)
- [Escalas, modos y tonalidades](theory/scales.md)
- [Acordes: construirlos, nombrarlos e invertirlos](theory/chords.md)
- [Números romanos, las teclas de armonía y las funciones de los acordes](theory/numerals.md)
- [Herramientas de edición como funciones simples](theory/tools.md)
- [Oír la armonía: acordes y tonalidades a partir de las notas](theory/hearing.md)
- [Conjuntos de clases de altura, y una armonía con cuatro nombres](theory/sets.md)
- [Series dodecafónicas y sus reglas](theory/twelve-tone.md)
- [Micromodos](theory/micromodes.md)
- [Conducción de voces y movimiento de las partes](theory/voice-leading.md)
- [Tomas: oír una melodía cantada](theory/listen.md)
- [Ejercicios: la teoría musical en código](theory/exercises.md)

# Parte IV: La banda

- [La biblioteca: estilos y ritmos como datos](band/library.md)
- [La hoja de acordes: modelo, distribución y texto](band/chord-sheet.md)
- [Cómo toca la banda una hoja de acordes](band/band.md)
- [Una orquesta para música que cambia de compás](band/orchestra.md)
- [Forma, secciones, variaciones y bloqueos](band/song-form.md)
- [Golpes, sostenidos, silencios y anticipaciones](band/stops.md)
- [El solista](band/soloist.md)
- [Tus propios estilos: el editor de estilos](band/style-editor.md)
- [Herramientas de práctica: metrónomo, entrenador de velocidad, entrenamiento auditivo](band/practice.md)
- [La partitura guía: de las notas a la notación](band/lead-sheet.md)
- [Ejercicios: la banda](band/exercises.md)

# Parte V: Las canciones como datos

- [Cómo se escribe una demostración](songs/demos.md)
- [De una interpretación MIDI o una partitura a una demostración](songs/from-midi-and-scores.md)
- [Ejercicios: las canciones como datos](songs/exercises.md)

# Parte VI: Compilar, probar y distribuir

- [Cómo se prueba el código: tests unitarios, tests de ventana, trazas](shipping/testing.md)
- [Tres sistemas e integración continua](shipping/ci.md)
- [Paquetes de cada versión](shipping/releases.md)
- [Cómo se construyen los libros](shipping/books.md)

# Referencia

- [Formato del archivo de proyecto](reference/project-format.md)
- [Glosario de Rust](reference/rust-glossary.md)
- [Glosario de música](reference/music-glossary.md)
- [Ideas para lo que viene](reference/ideas.md)
