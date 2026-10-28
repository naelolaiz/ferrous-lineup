# Ejercicios

Estos son para después del Step 9: la banda toca, las ventanas están en
su sitio y el código es tuyo para cambiarlo. Cada uno es un pequeño
trabajo con un final claro. Algunos son sobre todo de la app, otros
sobre todo de Rust, y unos pocos tocan ambos. Hazlos en cualquier orden.

Cada ejercicio dice lo que **ya existe**, para que lo amplíes en lugar
de construirlo otra vez. Los ejercicios 6 y 8 amplían ambos el formato
de proyecto, hasta el 29; si haces los dos, dale al segundo el siguiente
número libre, el 30. Ejecuta `cargo fmt --all --check` y
`cargo clippy --workspace --all-targets -- -D warnings` antes de dar uno
por terminado; CI ejecuta ambos.

## 1. Una copia de recuperación dañada (lineup-app, fácil)

**Objetivo.** Hacer fiable la recuperación tras un crash cuando la copia
en disco no lo es. `app/recovery.rs` ya escribe un archivo `.lineup` y
uno `.info` por cada ejecución en la carpeta `recovery` de la carpeta de
datos de la app, ofrece la copia al arrancar y elimina una copia que no
carga. La prueba existente `a_recovery_copy_comes_back_with_its_file`
cubre el camino feliz. Añade los infelices.

**Archivos.** `crates/lineup-app/src/app/recovery.rs`.

**Pasos.**

1. Míralo a mano: abre una demo, teclea un acorde en el primer compás,
   espera un momento y fuerza la salida de la app. No uses **File >
   Quit**. Arranca de nuevo y elige **Recover it**. Cierra la canción
   recuperada sin guardarla.
2. Lee `Files`, `load` y `write`, y las pruebas existentes al final del
   archivo.
3. Añade una prueba que tome `Files::of_run(&folder, "test")`, escriba
   basura en su archivo `project`, ponga un archivo `info` válido al
   lado, llame a `load(files, None)` y compruebe que devuelve `None`
   **y** que ambos archivos han desaparecido.
4. Añade una segunda prueba que cree la carpeta, escriba un proyecto
   válido en `files.project` con `file::save`, pero deje fuera el
   archivo info. Comprueba que `load` sigue devolviendo el proyecto, sin
   ruta y sin nombre, para que el diálogo pueda llamarlo "Untitled".
5. Limpia la carpeta temporal al final de cada prueba, como hace la
   existente.

**Ejecución.** `cargo test -p lineup-app recovery`.

**Terminado cuando.** Ambas pruebas pasan, y la primera falla si se
borra la llamada `files.remove()` de la rama de error de `load`.

## 2. Trazar un compás insertado y borrado (app, fácil)

**Objetivo.** Ver con tus propios ojos que insertar un compás mueve todo
lo que viene después, y que borrarlo lo devuelve todo. El chord sheet y
la playlist ya tienen **Insert a bar before**, **Insert a bar after** y
**Delete this bar**; `--trace` ya escribe cada nota que toca el player.

**Archivos.** `crates/lineup-core/src/bars.rs`,
`crates/lineup-core/src/project.rs`,
`crates/lineup-app/src/chord_sheet.rs`,
`crates/lineup-app/src/app/trace.rs`.

**Pasos.**

1. Escribe una traza de referencia de una demo con tempo constante:

   ```sh
   cargo run --release -- --trace "El Choclo" --out before.tsv
   ```

2. Abre la misma demo, elige **File > Save as…** y guárdala como
   `inserted.lineup`.
3. Haz clic derecho en el compás 3 del chord sheet y elige **Insert a
   bar after**. Guarda. Traza el archivo guardado:

   ```sh
   cargo run --release -- --trace inserted.lineup --out after.tsv
   ```

4. Compara las dos tablas, ignorando las líneas de cabecera con `#`. En
   `after.tsv`, cada nota que antes empezaba en el compás 4 o después
   debería estar ahora un compás más tarde; las notas de los compases 1
   a 3 no deberían cambiar.
5. Borra el compás que insertaste (es el compás 4), guarda, traza otra
   vez y compara con `before.tsv`.

**Ejecución.**

```sh
diff <(grep -v '^#' before.tsv) <(grep -v '^#' after.tsv) | head -40
```

**Terminado cuando.** La traza insertada muestra el desplazamiento de un
compás en `start`, `bar` y `step`, la traza borrada coincide con la de
referencia, y puedes señalar el código de `bars.rs` que mueve los clips,
marcadores y secciones.

## 3. Construir un estilo, guardarlo y encontrarlo en una canción (app y Rust, medio)

**Objetivo.** Usar el editor de estilos que ya está en el channel rack
([El editor de estilos](style-editor.md)), y luego leer los dos archivos
que produce: el archivo `.lineup-style` reutilizable y la canción que lo
incrusta.

**Archivos.** `crates/lineup-app/src/style_editor.rs`,
`crates/lineup-app/src/app/styles.rs`,
`crates/lineup-core/src/library/user_style.rs`,
`crates/lineup-core/src/band.rs`,
`crates/lineup-core/src/file.rs`.

**Pasos.**

1. Abre el channel rack (**F6**) y haz clic en **New style…**.
2. Dale un nombre, dos compases, un tempo y una progresión como
   `I | vi | ii7 | V7`. Añade un kick y una snare en el rack, y luego
   una nota de bajo en el piano roll. Pulsa **Space** para oírlo.
3. Haz clic en **Save as…** y déjalo en la carpeta de estilos en la que
   se abre el diálogo. Es la carpeta de datos de la app bajo `styles/`;
   en Linux es `~/.local/share/ferrouslineup/styles/`. Un estilo
   guardado fuera de esa carpeta no aparece en la Library.
4. Abre el archivo guardado en un editor de texto. Es RON. Encuentra el
   `format: 1` propio del estilo, sus `bars`, su `chart` y los `lanes`
   que guardan cada canal y sus notas.
5. Abre una demo, elige el estilo en la Library y guarda la canción.
   Abre el archivo de la canción y busca `custom_rhythm`: el estilo
   entero está dentro de la canción, porque la canción ya no puede
   depender de un archivo de tu carpeta de datos. Su formato ahora es al
   menos el 22, y mayor si la canción usa también una función posterior
   como un mixer guardado (formato 26).

**Ejecución.** `cargo test -p lineup-core user_style`, y luego
`cargo test -p lineup-app styles`.

**Terminado cuando.** La app lista el estilo tras un reinicio, la
canción lo reproduce, y el archivo de la canción contiene los lanes del
estilo y un formato de al menos 22.

## 4. Meter una canción nueva en su archivo de género (lineup-core, medio)

**Objetivo.** Añadir una melodía de dominio público a la biblioteca de
demos, en el archivo de su género, y hacer que el menú y las pruebas de
la biblioteca estén de acuerdo con ella. Las canciones ya son datos:
`Song`, `Part`, `SONGS` y los módulos por género existen; estás
añadiendo una entrada, no una función.

**Archivos.** `crates/lineup-core/src/library/song/<genre>.rs` (por
ejemplo `blues.rs` o `folk.rs`),
`crates/lineup-core/src/library/song.rs`.

**Pasos.**

1. Lee dos canciones del archivo de género que elegiste. Copia la forma
   de una: `name`, `origin`, `feel`, `genre`, `tonic`, `mode`, `time`,
   `tempo`, `comping`, `bass`, `rhythm`, `band`, `style`, `parts`,
   `form`, `drums_from`, `about`.
2. Escribe la melodía con `|` entre compases y un chart de acordes
   debajo. Usa un nombre de parte como "Verse" o "Tune". Mantén la
   melodía en dominio público.
3. Haz la constante `pub(super) const` y añádela a `SONGS` en `song.rs`
   dentro del bloque de su género. `SONGS` se mantiene en el orden de
   `Genre::ALL`, y la prueba lo comprueba.
4. Deja que las pruebas encuentren los errores: un compás que no cuadra,
   un acorde que no se analiza, un estilo en otra métrica.

**Ejecución.**

```sh
cargo test -p lineup-core library
cargo run --release -- --demos
```

**Terminado cuando.** Las pruebas pasan, la demo aparece bajo **File >
Demos > Songs >** su género, y se abre en el chord sheet y suena.

## 5. Un ritmo con cowbell y un fill (lineup-core, fácil)

**Objetivo.** Añadir un patrón de batería a la biblioteca, con un
cowbell y un fill con nombre. `RHYTHMS` y `FILLS` ya existen, las nueve
baterías extra incluida `Cowbell` ya existen, y las pruebas ya
comprueban la longitud y los caracteres de cada línea.

**Archivos.** `crates/lineup-core/src/library/rhythm.rs`.

**Pasos.**

1. Añade una entrada `rhythm(...)` a `RHYTHMS` en el bloque de género
   correcto. Dale un nombre, un género, un tempo, sus líneas y un fill
   que exista, como `"Tom fill"`.
2. Escribe una línea para `Kick`, una para `Snare` y una para `Cowbell`.
   Cada línea debe tener exactamente dieciséis caracteres en 4/4, con
   `X`, `x`, `o`, `h`, `l` y `.` como únicos caracteres. Los espacios
   son solo para leer.
3. Pon el cowbell en un contratiempo o una clave, no en cada step; la
   idea es oírlo como un color distinto.
4. Abre la Library, encuentra el ritmo en su género, haz clic en él y
   reprodúcelo. Luego haz un cambio de sección para que se use el fill.

**Ejecución.** `cargo test -p lineup-core rhythm`.

**Terminado cuando.** `every_line_is_whole_bars_of_known_characters` y
`names_are_unique_and_fills_exist` pasan, el ritmo aparece listado con
su tooltip, y el fill suena hacia la siguiente sección.

## 6. Un segundo envío: delay (lineup-core, lineup-audio y lineup-app, difícil)

**Objetivo.** El mixer ya tiene un envío **Reverb** por canal y una
reverberación maestra. Añade un envío **Delay** junto a él, para que un
canal pueda entrar en un eco sin perder su sonido seco. Cada canal ya
tiene su propio delay entre sus [efectos](channel-effects.md); este es
compartido, como la reverberación, para que varios canales hagan eco
juntos en un solo delay. Es un capstone: cruza el archivo de proyecto,
el hilo de audio y la ventana del mixer.

**Archivos.** `crates/lineup-core/src/project.rs`,
`crates/lineup-core/src/mixer.rs`, `crates/lineup-core/src/file.rs`,
`crates/lineup-audio/src/engine.rs`,
`crates/lineup-audio/src/reverb.rs` (como modelo para un `delay.rs`
nuevo), `crates/lineup-app/src/app/mixer.rs`.

**Pasos.**

1. En `Channel`, añade `delay_send: f32`, por defecto `0.0`, e
   inclúyelo en las comprobaciones de rango y en `has_plain_mix`. Da a
   todos los constructores el valor por defecto.
2. En `Mixer`, añade los ajustes propios del delay: tiempo en
   milisegundos, feedback y wet/dry. Valídalos en `Mixer::valid`.
3. Añade `Command::SetDelay` y `Command::SetDelaySend`, y un delay
   compartido que reserve su búfer cuando se construye el motor, nunca
   en `process`. El ring buffer del delay de canal en
   `crates/lineup-audio/src/effects.rs` muestra cómo leer el eco. Sigue
   el patrón de la reverberación: saca del dry mix la `amount` del
   sample como `send`, y vuelve a añadir la señal con delay.
4. Añade un deslizador **Delay** a cada línea del mixer, junto a
   **Reverb**, y una pequeña sección de delay junto a los controles de
   reverberación. Envía el comando por el mismo camino que
   `SetReverbSend`.
5. Sube `FORMAT_VERSION` a 29, enseña a `format_for` que un envío de
   delay distinto de cero o unos ajustes de delay no por defecto
   necesitan 29, y deja los campos fuera del texto cuando estén en sus
   valores por defecto.
6. Prueba el camino de audio: un envío de 0 no produce eco, un envío
   mayor que 0 produce uno, y el sample seco se reduce en la cantidad
   húmeda. Prueba el archivo: un archivo antiguo se abre con envío 0;
   una canción con envío de delay se escribe como formato 29 y hace ida
   y vuelta.

**Ejecución.**

```sh
cargo test -p lineup-core mixer
cargo test -p lineup-audio delay
cargo test -p lineup-app mixer
```

**Terminado cuando.** El deslizador se oye en la app y en un WAV
exportado, el hilo de audio no reserva memoria mientras se ejecuta, y
una canción sin delay se sigue escribiendo en el formato que tenía.

## 7. F10 para el metrónomo, y su línea del F1 (lineup-app, fácil)

**Objetivo.** El metrónomo ya es un interruptor en la barra de
herramientas y en **Edit > Settings… > Playback**. Dale una tecla y
añade la tecla a la lista del F1. F10 está libre; no uses la **M** sola,
que toca una nota cuando el teclado de escritura está activo.

**Archivos.** `crates/lineup-app/src/app/practice.rs`,
`crates/lineup-app/src/app.rs`,
`crates/lineup-app/src/app/shortcuts.rs`,
`crates/lineup-app/src/app/ux_tests.rs`.

**Pasos.**

1. En `practice.rs`, añade una constante `KeyboardShortcut` para F10.
2. En `LineupApp::keyboard_shortcuts`, léela en el mismo sitio protegido
   que Space: no mientras un campo de texto tiene el teclado, y no
   mientras un modal está abierto. Llama a `set_metronome(!metronome)`.
3. Añade una línea a la sección **Anywhere** de `shortcuts.rs`:
   "Metronome on or off". Importa la constante en lugar de escribir la
   tecla dos veces.
4. Añade una prueba headless en `ux_tests.rs`: pulsa F10, comprueba con
   assert `practice.view().metronome`; púlsala otra vez, comprueba que
   está apagado.

**Ejecución.**

```sh
cargo test -p lineup-app shortcuts
cargo test -p lineup-app metronome
```

**Terminado cuando.** F10 alterna el botón de la barra de herramientas,
F1 lo lista bajo "Anywhere", y la prueba existente
`no_two_global_shortcuts_share_a_key` sigue pasando.

## 8. Un campo de proyecto nuevo y su prueba de formato (lineup-core, medio)

**Objetivo.** Añadir un campo inofensivo a un proyecto, guardarlo sin
romper archivos antiguos, y demostrar que una canción que lo usa se
escribe como el siguiente formato. `ProjectFile`, sus valores por
defecto de serde y las pruebas de archivo ya están; este es el ejercicio
completo más pequeño de ampliar el formato.

**Archivos.** `crates/lineup-core/src/project.rs`,
`crates/lineup-core/src/file.rs`.

**Pasos.**

1. Añade `pub notes: String` a `Project`, inicializado a `String::new()`
   en `Project::empty`.
2. Añade `notes: String` a `ProjectFile`, con
   `#[serde(default, skip_serializing_if = "String::is_empty")]`, y
   cópialo en ambas direcciones.
3. Sube `FORMAT_VERSION` a 29. En `format_for`, devuelve 29 primero
   cuando `!project.notes.is_empty()`; deja todo lo demás como está.
4. Escribe tres pruebas en `file.rs`: un proyecto vacío se queda en su
   formato antiguo y no tiene línea `notes:`; un proyecto con notas es
   formato 29 y hace ida y vuelta; y un archivo escrito antes de que
   existiera el campo se vuelve a leer con notas vacías.

**Ejecución.** `cargo test -p lineup-core file`.

**Terminado cuando.** Los textos de proyecto antiguos siguen
abriéndose, el campo nuevo sobrevive a una ida y vuelta, y `format_for`
nombra el 29 solo para una canción que lo necesita.

## 9. Dos vistas en un vídeo, y una comprobación de sincronía (app, medio)

**Objetivo.** Exportar un vídeo con dos vistas y comprobar que la imagen
y el sonido coinciden. La exportación de vídeo ya existe, con chord
sheet, lead sheet, playlist, piano roll y harmony keys; estás probando
la sincronía, no construyendo una vista.

**Archivos.** `crates/lineup-app/src/app/video.rs`,
`crates/lineup-video/src/` como referencia.

**Pasos.**

1. Abre una demo con un cambio de métrica o de tonalidad, como **El
   Choclo**.
2. Elige **File > Export WAV…** y guarda `el-choclo.wav`.
3. Elige **File > Export video…**, marca **Chord sheet** y **Piano
   roll**, ponlas lado a lado en la vista previa, elige **1280×720** y
   exporta `el-choclo.mp4`.
4. Si `ffprobe` está instalado, compara las duraciones. La imagen va a
   30 fotogramas por segundo, y el sonido es MP3 dentro del MP4, así que
   permite una décima de segundo:

   ```sh
   ffprobe -v error -show_entries format=duration -of default=nw=1:nk=1 el-choclo.wav
   ffprobe -v error -show_entries format=duration -of default=nw=1:nk=1 el-choclo.mp4
   ```

5. Mira el compás donde cambia la tonalidad o la métrica. El compás
   enmarcado del chord sheet, el cabezal de reproducción del piano roll
   y el sonido deberían coincidir allí. Comprueba un segundo punto, como
   el primer compás después de una repetición.

**Terminado cuando.** Las duraciones difieren en no más de uno o dos
fotogramas más la cola de audio, las dos vistas muestran el mismo compás
en el mismo momento, y puedes decir qué código de `video.rs` dibuja la
imagen **antes** que el sonido en cada fotograma.

## 10. Una prueba de regresión de note-off MIDI (lineup-audio, medio)

**Objetivo.** Demostrar que **Stop** envía note-offs para las notas que
el secuenciador empezó en un canal MIDI Out. El motor ya guarda
note-offs cuando la cola MIDI está llena, y ya envía All Notes Off antes
de cerrar una salida; esta prueba fija el camino de Stop.

**Archivos.** `crates/lineup-audio/src/engine.rs` (el módulo
`#[cfg(test)]`).

**Pasos.**

1. Lee `midi_out_channels_send_their_notes` y
   `stopping_releases_held_notes`. Usa `add_midi_out`, `received` y
   `note_off` del módulo de pruebas.
2. Añade `stopping_releases_midi_out_notes`: una nota larga en el canal
   MIDI 3, **Play**, renderiza un step y comprueba con assert el
   NoteOn; envía **Stop**, renderiza unos cuantos fotogramas y
   comprueba con assert el NoteOff.
3. Haz la nota lo bastante larga como para que no pueda haber terminado
   por su propio gate.
4. Ejecuta la prueba una vez con su línea `send(&mut h, Command::Stop)`
   quitada para verla fallar, y luego devuelve la línea.

**Ejecución.**
`cargo test -p lineup-audio stopping_releases_midi_out_notes`.

**Terminado cuando.** La prueba falla si `stop` deja de llamar a
`release_all`, y pasa con el código actual.
