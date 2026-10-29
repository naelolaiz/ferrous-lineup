# Paso 8: Teclas de armonía, una biblioteca de estilos y demos

Los estudios basados en patrones te dan una fila de teclas de acordes
grandes: una por cada acorde de la tonalidad en la que estás, para que
puedas tocar una progresión con un dedo y *ver* cómo funciona la
armonía. El paso 8 las añade a Ferrous Lineup, y una biblioteca que las
acompañe:

- **las harmony keys**, a lo largo de la parte inferior de la ventana
  principal. A la izquierda, la **tonalidad** y el **compás** del
  proyecto, siempre a la vista. En el medio, siete teclas grandes, de
  **I** a **vii°**, una por cada acorde de la tonalidad. Se colorean
  según lo que *hace* el acorde: azul para casa (tónica), verde para
  salir de casa (subdominante), naranja para tensión (dominante).
  Mantén una pulsada, con el ratón o las teclas de número **1** a
  **7**, y toca su acorde en el canal seleccionado; con **⏺** (grabar)
  activado, se graba. Los acordes que suelen venir después se iluminan
  con un contorno amarillo, y la línea de abajo dice por qué. A la
  derecha, un teclado muestra las notas de la escala y las notas que
  suenan,
- **los acordes seguidos mientras suena la música**: mientras un patrón
  o la canción suena, la tecla de armonía del acorde que suena se ilumina, y
  el teclado muestra sus notas en azul. Pulsa ▶ en cualquier demo y
  mira la progresión moverse entre casa, salida y tensión,
- **los compases**: diez habituales — 2/4, 3/4, 4/4, 5/4, 3/8, 5/8,
  6/8, 7/8, 9/8 y 12/8 — más cualquier otro número de arriba y de abajo
  que escribas, incluidos agrupamientos irregulares como 2+2+3/8
  ([Cualquier compás, y métricas irregulares](any-time-signature.md)).
  Los compases de la lista de reproducción, el snap, los colores de pulso del rack de canales, las líneas del piano roll y la longitud de los patrones lo
  siguen todos, y va a los archivos de proyecto y a los archivos MIDI,
- **una biblioteca** en una pestaña **Biblioteca** nueva junto a la lista
  de patrones: **78 estilos de armonía** y **49 ritmos** de 13 géneros,
  del pop y el blues a la bossa nova, el flamenco, el reggae y, por
  supuesto, el **tango** (marcato, síncopa, milonga, habanera y el tango
  vals). Haz clic en un estilo para oírlo en tu tonalidad, clic en otro
  para cambiarlo, y **Ctrl+Z** lo deshace. Pasa el ratón por encima de
  un estilo para ver sus acordes en tu tonalidad,
- **demostraciones** bajo **Archivo > Demostraciones**: **62 canciones**, ordenadas por
  género, de *Oda a la alegría* y *Para Elisa* a *Korobeiniki*, la *Habanera*
  y *When the Saints Go Marching In*, cada una con su melodía, acordes,
  bajo y batería. La mayoría son de dominio público; los géneros con
  pocos clásicos libres, como el tango y el flamenco, reciben melodías
  nuevas escritas para ellos ([Más demos](more-demos.md)). Y cada estilo
  de armonía como una canción corta,
- **una grabación, oída y seguida (experimental)**: **Archivo > Importar grabación…** abre un archivo MP3, WAV, FLAC u Ogg y le pone una pista
  de armonía, a partir de los pulsos, tonalidades, acordes y métrica que
  oye ([Importar una grabación](../spike-listen/README.md)). Los
  resultados son aproximados,
- **un ritmo nuevo para una canción**: elige un ritmo después de un
  estilo, o haz clic derecho en un ritmo y toca la canción entera con
  él, y los acordes y el bajo siguen el ritmo nuevo, conservando los
  mismos acordes, como haría una banda. Ferrous Lineup deduce antes los
  acordes a partir de las notas ([Un ritmo nuevo para una
  canción](new-rhythm.md)),
- **el acompañamiento sigue a tus acordes**: actívalo en el menú MIDI, y
  mientras suena la música, el acorde que mantienes en un teclado MIDI o
  de escritura se vuelve el acorde del acompañamiento, como en un
  teclado arreglador ([El acompañamiento sigue a tus
  acordes](chord-follow.md)),
- **Salida MIDI para cada canal**: elige un sintetizador en MIDI > Salida y
  la canción entera suena en él, la batería en el canal 10, sin añadir
  antes canales de salida MIDI ([Salida MIDI para cada
  canal](midi-out-every-channel.md)),
- **música sin tonalidad**: una serie dodecafónica o un conjunto de
  clases de alturas como tonalidad, una ventana que muestra la matriz de
  una serie con las 48 formas y escribe cualquiera de ellas en el
  patrón, serialismo de duraciones, dinámicas y octavas, un navegador de
  las 224 clases de conjuntos, el conjunto de las notas que
  selecciones, y estilos y demos dodecafónicos ([Series dodecafónicas y
  conjuntos de clases de alturas](twelve-tone.md)),
- **micromodos**: los doce grupos de tres notas de Francisco Kröpfl en
  sus tres familias, dibujados en un reloj, oídos, dibujados con ellos o
  usados como tonalidad; los micromodos dentro de las notas que
  selecciones; las ventanas Conjuntos y Micromodos enlazadas en ambos
  sentidos; y estilos y demos de micromodos, con la serie del Concerto
  de Webern, hecha de un solo micromodo
  ([Micromodos](micromodes.md)),
- **nueve tambores nuevos**, hechos por el sintetizador como los cuatro
  primeros: hi-hat abierto, rim, toms grave y agudo, ride, crash,
  cencerro, shaker y conga,
- **patrones de hasta dieciséis compases de largo**, para que un blues
  entero de doce compases quepa en un patrón.
- **una cacería de errores**: la lista de reproducción y el piano roll hacen zoom
  horizontal y vertical por su cuenta; las notas, escalas y acordes se
  escriben con los nombres correctos (Mi♯ en Fa♯ mayor, E#dim); los
  hints y las teclas de armonía aplican bien su teoría en tonalidades
  menores, modos y conjuntos; las notas mantenidas sobreviven a los
  bordes de sección, al silenciado y a los cambios de tempo; y muchos
  pequeños fallos han desaparecido ([Una cacería de
  errores](bug-hunt.md)).

![Las harmony keys mientras suena la canción de demo. Fa (IV) está pulsada, en naranja en el teclado; la canción toca La menor, en azul. Las teclas con contorno amarillo son los acordes que suelen venir después de IV.](../images/step-8-harmony-keys.png)

![La pestaña Library. Acaba de probarse la bossa nova: está en su propio patrón, y su tooltip muestra sus acordes en Do mayor, su tempo y su ritmo.](../images/step-8-library.png)

![File > Demos: la canción de inicio, las canciones y todos los estilos por género](../images/step-8-demos.png)

## Cómo usarlo

| Haz esto | Para |
|----------|------|
| mantén una tecla de armonía, o **1** a **7** | tocar su acorde en el canal seleccionado |
| Tríadas, Séptimas, Novenas (bajo la tonalidad) | tocar acordes de tres, cuatro o cinco notas |
| Tonalidad, Compás (a la izquierda de las teclas de armonía) | cambiar la tonalidad del proyecto (que mueve la canción a ella) y el compás |
| **F8**, o el botón 🎼 Armonía | mostrar u ocultar las teclas de armonía |
| ● Rec, Espacio, luego mantén teclas de armonía | grabar acordes en el patrón |
| pestaña Biblioteca, clic en un estilo | tocarlo en un patrón nuevo, en tu tonalidad |
| clic en otro estilo o ritmo | cambiarlo en el mismo patrón |
| Ctrl+Z | deshacerlo |
| Tempo (pestaña Biblioteca) | poner también el tempo habitual del estilo |
| clic en un ritmo después de un estilo | tocar los acordes y el bajo del estilo a la manera del ritmo |
| clic derecho en un ritmo, Tocar toda la canción con este ritmo | dar batería nueva a la canción entera, y volver a tocar sus acordes y su bajo a juego |
| MIDI > El acompañamiento sigue los acordes, luego mantén un acorde mientras tocas | tocar los acordes y el bajo en el acorde que mantienes |
| MIDI > Salida, elige un sintetizador (Enviar todos los canales está activado) | tocar también cada canal en el sintetizador externo, la batería en el canal 10 |
| Archivo > Demostraciones | abrir una canción o un estilo como proyecto entero, y tocarlo |
| menú Tonalidad > Serie dodecafónica…, o Serie… en el piano roll | hacer de una serie la tonalidad, ver su matriz, escribir sus formas |
| Conjuntos… en el piano roll | buscar cualquier conjunto, dibujar con él o usarlo como tonalidad |
| Micromodos… en el piano roll | ver los doce micromodos de Kröpfl y los que hay en las notas seleccionadas; oír uno, dibujar con él o usarlo como tonalidad |
| Ctrl+rueda, Alt+rueda (o Ctrl y Alt con + y −), los zooms abajo a la derecha | hacer compases y pasos más anchos, o pistas y teclas más altas; Ctrl+0 para volver al 100% ([Una cacería de errores](bug-hunt.md)) |
| Ver > Tamaño de la interfaz | hacer textos, botones y menús más grandes o más pequeños en todas partes |
| menú Ver, o **Alt+F8** | mostrar u ocultar cada ventana, y el panel Patrones y Biblioteca |

Las teclas de número tocan las teclas de armonía solo mientras el teclado de
escritura está apagado, porque el teclado de escritura las usa para las
teclas negras.

En cuanto cambias un patrón que escribió la biblioteca (añadir una nota,
renombrarlo), es tuyo: el siguiente clic en la biblioteca empieza un
patrón nuevo en lugar de reemplazarlo.

## Cómo se probó

- La biblioteca es datos, y sus pruebas leen cada entrada: cada tabla y
  melodía se parsea, cada compás suma lo que marca su compás, cada
  estilo y ritmo se puede escribir en un proyecto, cada demo sobrevive a
  guardar y cargar, y al menos el 80% de las notas de cada melodía
  están en su tonalidad (el resto son notas de paso).
- Cada una de las 140 demos de este paso (62 canciones y 78 estilos) se
  renderizó en offline, con el mismo código de audio que la exportación
  WAV, y se midió: ninguna recorta, y los picos más fuertes están en
  0,96 de la escala completa.
- En la app en marcha: las teclas de armonía con el ratón y las teclas de
  número, las sugerencias, la pestaña Biblioteca (probar, cambiar,
  deshacer), el menú Demostraciones con su pregunta "¿Guardar cambios?", y los
  compases en la lista de reproducción, el rack y el piano roll.
- El compás en los archivos MIDI exportados lo comprueba una prueba.

## Qué cambió en la arquitectura

```text
  lineup-core
    meter.rs (nuevo):          TimeSignature: pasos por compás y por pulso
    harmony/keys.rs (nuevo):   los siete acordes de una tonalidad, sus funciones, tocarlos
    library/ (nuevo):
      numeral.rs     "I V vi IV": acordes contados desde la tónica
      chart.rs       tablas de acordes por compás, melodías nota a nota
      voicing.rs     voice leading: cada acorde cerca del anterior
      comping.rs     ritmos para acordes y líneas de bajo
      style.rs       78 estilos de armonía
      rhythm.rs      49 ritmos y 6 fills, una línea de texto por tambor
      song.rs        el tipo Song y SONGS, 62 canciones
      song/          un archivo de canciones por género
      apply.rs       escribir estilos y ritmos en patrones
      demo.rs        proyectos de demo enteros
    project.rs:     el compás, nueve tambores nuevos, el acorde en un paso
    pattern.rs:     MAX_STEPS 256, set_notes
    file.rs:        formato 3
    midi_file.rs:   el compás y los tambores nuevos en archivos .mid

  lineup-audio
    drums.rs:       nueve sonidos de tambor nuevos

  lineup-app
    harmony_keys.rs (nuevo):      la barra de harmony keys
    library_browser.rs (nuevo):   la pestaña Library
    app/library.rs (nuevo):       probar estilos y ritmos, las demos
    playlist.rs, channel_rack.rs, piano_roll.rs:  compases y pulsos de cualquier compás
```

Tres ideas sostienen este paso:

1. **El conocimiento como datos.** Un músico lleva mucho en la cabeza:
   qué acordes hacen una bossa nova, dónde va la snare en un ritmo de
   reggae, cómo va *Greensleeves*. La biblioteca lo escribe todo como
   strings cortos y tablas que una persona puede leer y comprobar, y
   unas pocas funciones pequeñas los convierten en notas. Añadir un
   estilo es añadir unas pocas líneas, no escribir código.
2. **Relativo, no absoluto.** Los estilos se escriben en numerales
   romanos y los ritmos en pasos de un compás, así que la misma tabla
   toca en cualquier tonalidad, y un ritmo conserva su sitio en
   cualquier compás.
3. **La biblioteca solo escribe patrones.** Probar un estilo es una
   edición normal a través de `LineupApp::edit`, así que deshacer,
   guardar, el hilo de audio y todo lo demás funcionan sin saber que la
   biblioteca existe.

## Archivos de este paso

```text
crates/lineup-core/src/meter.rs               TimeSignature (nuevo)
crates/lineup-core/src/harmony/keys.rs        chord_keys, Function, ChordSize, play (nuevo)
crates/lineup-core/src/harmony/scale.rs       Scale::ninth
crates/lineup-core/src/library/*.rs           la biblioteca (nuevo)
crates/lineup-core/src/project.rs             time_signature, tambores nuevos, chord_notes_in_song
crates/lineup-core/src/pattern.rs             MAX_STEPS, set_notes
crates/lineup-core/src/file.rs                formato 3
crates/lineup-core/src/midi_file.rs           evento meta de compás, notas de tambor nuevas
crates/lineup-audio/src/drums.rs              hat abierto, rim, toms, ride, crash, cencerro, shaker, conga
crates/lineup-app/src/harmony_keys.rs         la barra de harmony keys (nuevo)
crates/lineup-app/src/library_browser.rs      la pestaña Library (nuevo)
crates/lineup-app/src/app/library.rs          audiciones y demos (nuevo)
crates/lineup-app/src/app.rs                  la barra, el acorde que suena, F8
crates/lineup-app/src/app/files.rs            Archivo > Demostraciones
crates/lineup-app/src/app/live.rs             play_typed se comparte con las harmony keys
crates/lineup-app/src/playlist.rs             la pestaña Library, compases de cualquier longitud
crates/lineup-core/src/midi_file/read.rs      leer archivos MIDI (nuevo)
crates/lineup-core/src/midi_file/import.rs    un archivo MIDI se vuelve un proyecto (nuevo)
crates/lineup-core/src/harmony/key_finding.rs la tonalidad de una pieza a partir de sus notas (nuevo)
crates/lineup-app/src/channel_rack.rs         colores de pulso por compás
crates/lineup-app/src/piano_roll.rs           líneas de compás y de pulso por compás
crates/lineup-app/src/app/view.rs             el menú View y el control de zoom (nuevo)
```

Los números de formato de las listas de arriba son los del paso 8: el
paso 8 introdujo el formato 3 para sus tambores nuevos, los patrones
largos y los compases. Hoy `FORMAT_VERSION` es 27, y `format_for` sigue
devolviendo 3 para un proyecto que solo necesita esas cosas, como un
patrón largo o un 3/4 simple; una característica posterior, o un compás
agrupado, necesita un número más alto.

Hacia dónde va este paso, en el orden de los capítulos: las teclas de armonía; los compases; la biblioteca y las demos que construye; adaptarse a
la ventana y encontrar tus notas; la estructura de la canción, el
transporte y más demos; oír acordes y usar solo las pistas que una
canción necesita; los marcadores de tiempo y los cambios de tonalidad;
el acompañamiento y la salida MIDI en cada canal; el folclore sudamericano;
las series dodecafónicas y los micromodos; la cacería de errores, el
tempo y el feel, y la Marcha turca; la importación MIDI; las voces y los
instrumentos; encontrar la tonalidad; cualquier compás y cambiar la
métrica; los nombres de las demos; el zoom que sigue al puntero; y, al
final, una tonalidad como conjunto de clases de alturas. Siguen los
[ejercicios](exercises.md).
