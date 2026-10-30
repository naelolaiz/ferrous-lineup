# Ejercicios: la forma del programa

Estos ejercicios practican el Rust de esta parte sobre el código real.
Cada uno dice lo que **ya existe**, para que lo amplíes en lugar de
construirlo otra vez. Algunos son pequeños trabajos con un final claro;
los que se llaman «Piénsalo» son preguntas, sin código que escribir.

Hazlos en una rama tuya. Ejecuta `cargo test` después de cada cambio, y
`cargo fmt --all --check` y
`cargo clippy --workspace --all-targets -- -D warnings` antes de dar uno
por terminado: CI ejecuta los tres.

## Crates e hilos

### 1. Rómpelo a propósito (cualquier crate)

Aprender a leer los errores del compilador es la mitad de aprender Rust.
Prueba cada una de estas cosas, lee el error y luego deshazla:

- En `AudioEngine::new` (`engine.rs`), llama dos veces seguidas a
  `build_stream::<f32>(&device, config.clone(), audio_thread, Arc::clone(&problems))`,
  de modo que la segunda llamada falle porque `audio_thread` se movió a
  la primera.
- Quita `move` del closure del callback de audio en `build_stream`.
- Añade una variante `SetName(String)` a `Command`. ¿Por qué `Command`
  no puede ser `Copy`? ¿Qué variantes ya lo hacen imposible, y qué dice
  el compilador cuando pides `Clone`?
- Haz `pub` el campo de `Note` (en `lineup-core/src/pitch.rs`) y escribe
  `Note(200)` en `app.rs`. ¿Por qué compila eso, y por qué es una mala
  idea?

*Practica:* propiedad, closures `move`, tipos `Copy` y no `Copy`,
privacidad.

### 2. Piénsalo: el hilo de audio

- ¿Por qué sería peor
  `Command::InsertChannel { instrument: Box<Instrument>, ... }` que
  enviar el instrumento por valor? (`Instrument` ya mete en una caja sus
  arrays de voces; ¿qué ganaría una segunda caja?)
- ¿Qué pasaría si la ventana olvidara llamar a `collect_garbage`?
- `Garbage` es un único enum con una variante por cada clase de cosa que
  se tira. ¿Qué costaría una cola de basura por cada clase, y qué
  ganaría?
- `AudioEngine::playhead` usa `Ordering::Relaxed`. ¿Podría la ventana
  ver alguna vez un número de paso que nunca sonó?

## El proyecto

### 3. Eliminar todas las pistas vacías (lineup-core y lineup-app, fácil)

Eliminar una pista ya existe: lee `Project::remove_track` y el elemento
«Eliminar la pista» del menú contextual de la pista. Quita la pista y
sus clips, y sube una posición los clips de las pistas de debajo, igual
que hace `remove_pattern` con los patrones.

Lo que falta es una forma de quitar *todas* las pistas vacías a la vez.
`remove_unused_tracks` solo quita las pistas vacías del final de la
lista de reproducción (llama a `trim_tracks`). Añade
`remove_empty_tracks`, que quite una pista esté donde esté si no tiene
clips, y un elemento para ella en el menú de la pista. ¿Qué índices
tienes que visitar, y en qué orden, para que los clips sigan apuntando a
las pistas correctas? ¿Qué debería pasar si eso dejara sin pistas? Escribe
primero un test con la canción de ejemplo, y luego el método.

*Practica:* `retain`, mantener reglas que abarcan dos listas, probar el
orden de los cambios de índice.

### 4. Piénsalo: posiciones o identificadores

Los clips se refieren a los patrones por su *posición* en la lista, y
por eso `remove_pattern` tiene que renumerarlos. Imagina en cambio que
cada patrón tiene un identificador permanente (`struct PatternId(u32)`)
que nunca cambia.

- ¿Qué métodos de `Project` se vuelven más sencillos, y cuáles más
  difíciles?
- ¿Qué cambiaría en el archivo, y en los `ChannelNotes` del hilo de
  audio?
- ¿Sería más fácil deshacer con identificadores o con posiciones?

## Avisar al hilo de audio de lo que cambió

### 5. Ediciones al azar para el diff (lineup-core, media)

Los tests del diff comprueban casos elegidos a mano. Escribe uno que
compruebe mil al azar: empieza con la demo, haz una edición al azar
(activar un paso, añadir o quitar un patrón o un canal, cambiar un
volumen, mover un clip...), llama a `check_both_ways(&before, &after)` y
repite desde el proyecto nuevo.

Para tener azar sin ningún crate, un pequeño generador «xorshift» basta
para los tests:

```rust
struct Random(u64);

impl Random {
    fn next(&mut self, below: usize) -> usize {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        (self.0 % below as u64) as usize
    }
}
```

Arráncalo con un número fijo (no 0, que se queda en 0 para siempre),
para que un fallo vuelva a ocurrir cada vez que ejecutes el test. Si
quieres ir más lejos, el crate `proptest` hace esto como es debido, y
cuando encuentra un caso que falla lo *reduce* a la lista de ediciones
más corta que sigue fallando.

Prueba también varias ediciones al azar entre dos comprobaciones, como
las que puede contener un paso de deshacer. ¿Produce `diff` alguna vez
una lista larga donde bastaría una corta? ¿Es eso un problema?

*Practica:* tests basados en propiedades, azar reproducible.

### 6. Canales emparejados por nombre (lineup-core, media)

`diff` empareja los canales de dos proyectos con
`|old, new| old == new`, y los patrones con `same_pattern`. Escribe
`same_channel(old: &Channel, new: &Channel) -> bool`, que diga que dos
canales son el mismo cuando tienen el mismo nombre, y pásala a
`Alignment::new` en su lugar. Ejecuta los tests del diff. Luego busca un
caso en que dé la respuesta equivocada: ¿qué pasa con dos canales
llamados «Piano»?

*Practica:* funciones como argumentos, qué significa «el mismo».

### 7. Piénsalo: qué enviar

- `SetNotes` envía la lista entera de notas de un canal con cada
  cambio. ¿A partir de qué tamaño sería eso un problema, y qué enviarías
  en su lugar?
- La ventana envía la duración de la canción junto con sus piezas, en
  lugar de dejar que la calcule el hilo de audio. ¿Qué haría falta para
  que dos copias de ese cálculo coincidieran siempre, y cómo notarías que
  se separan? Las piezas son los `Play` que hace
  `Project::playback_arrangement`. ¿Por qué es más sencillo desplegar las
  repeticiones del lado de la ventana que enseñarle al hilo de audio qué
  son las secciones?
- Deshacer mientras suena la canción. Cuando deshacer quita un canal,
  sus notas que suenan se cortan en el acto, porque el canal va a la
  basura. En el caso de [tres canales Lineup Osc idénticos](sync.md#cuando-la-historia-es-ambigua),
  el hilo de audio quita el *último* canal y da sus notas a otro. ¿La
  nota de qué canal sigue sonando, y podrías llegar a oír la diferencia?

## Deshacer y rehacer

### 8. Una ventana con el historial (lineup-app, fácil)

Añade Edición > Historial…, una ventanita que liste cada paso que se
puede deshacer o rehacer, el más nuevo arriba, con la posición actual
marcada. Hacer clic en un paso salta hasta él.

Necesitas dos métodos en `History`: uno que dé las etiquetas para
deshacer y otro las de rehacer, cada uno como iterador
(`self.undo.iter().map(|step| step.label)`). Volver cinco pasos atrás
son cinco llamadas a `history.undo`. ¿Debería ejecutarse `sync_audio`
después de cada una, o una vez al final, comparando con el proyecto de
antes de la primera? ¿Cuál es más barato, y acaba el hilo de audio igual
en los dos casos? (El ejercicio 5 responde a la última pregunta.)

*Practica:* iteradores sobre campos privados, `rev()`, reutilizar
`diff`.

### 9. Copias más baratas (lineup-core, difícil)

Cada paso de deshacer copia el proyecto entero, con todos sus patrones,
aunque un clic en un paso cambie un solo patrón. (Un arrastre copia una
vez, cuando abre su paso, no en cada fotograma, pero la copia sigue
siendo el proyecto entero.) Abarata las copias compartiendo lo que no
cambió: guarda los patrones como `Vec<Arc<Pattern>>`.

Clonar un `Vec<Arc<Pattern>>` copia solo los punteros y suma uno a cada
contador; los patrones mismos se comparten entre el proyecto actual y
cada copia del historial. Para cambiar uno, usa **`Arc::make_mut`**, que
da un `&mut Pattern` y antes hace una copia privada *solo si* el patrón
está compartido («clonar al escribir»). Así,
`Project::pattern_mut` pasa a ser
`self.patterns.get_mut(index).map(Arc::make_mut)`.

Después `diff` puede saltarse rápido los patrones sin cambios: si
`Arc::ptr_eq` dice que dos patrones son la misma memoria, seguro que son
iguales. Haz un proyecto de prueba con 999 patrones llenos de notas, y
mide el tiempo de una edición antes y después con `std::time::Instant`.
¿Sigue funcionando sin cambios el `PartialEq` del proyecto? ¿Tiene que
cambiar el formato del archivo?

*Practica:* `Arc` para compartir, clonar al escribir, medir antes de
optimizar.

### 10. Piénsalo: etiquetas y lo que abarca deshacer

- Las etiquetas de los pasos son `&'static str`, así que «rename
  pattern» no puede decir qué patrón. Una etiqueta `String` permitiría
  «rename pattern Beat», pero la mayoría de las etiquetas son texto fijo,
  y un `String` reservaría memoria para cada una. Busca
  `std::borrow::Cow<'static, str>`: ¿cómo permite que una etiqueta sea
  de los dos tipos, y que reserve memoria solo cuando haga falta?
- El volumen master forma parte del mezclador del proyecto, así que
  deshacer lo devuelve a su valor anterior, y se guarda con la canción.
  El patrón seleccionado, el zoom y las ventanas abiertas no. ¿Esperarías
  que deshacer cambiara alguno de ellos? ¿Qué tendría que pasar a
  `Project` para eso, y qué significaría para el archivo guardado?

## Guardar

### 11. Un campo nuevo del proyecto y su test de formato (lineup-core, media)

**Objetivo.** Añadir un campo inofensivo a un proyecto, guardarlo sin
romper los archivos antiguos, y demostrar que una canción que lo usa se
escribe con el formato siguiente. `ProjectFile`, sus valores por defecto
de serde y los tests de archivo ya existen; este es el ejercicio
completo más pequeño para hacer crecer el formato.

**Archivos.** `crates/lineup-core/src/project.rs`,
`crates/lineup-core/src/file.rs`.

**Pasos.**

1. Añade `pub notes: String` a `Project`, inicializado a
   `String::new()` en `Project::empty`.
2. Añade `notes: String` a `ProjectFile`, con
   `#[serde(default, skip_serializing_if = "String::is_empty")]`, y
   cópialo en los dos sentidos.
3. Sube `FORMAT_VERSION` en uno, a 29 (si otro ejercicio ya tomó el 29,
   usa el siguiente número libre). En `format_for`, devuelve el número
   nuevo en primer lugar cuando `!project.notes.is_empty()`; deja todo lo
   demás como está.
4. Escribe tres tests en `file.rs`: un proyecto vacío se queda en su
   formato anterior y no tiene línea `notes:`; un proyecto con notas es
   del formato nuevo y sobrevive a guardar y abrir; y un archivo escrito
   antes de que existiera el campo se lee con las notas vacías.

**Ejecuta.** `cargo test -p lineup-core file`.

**Terminado cuando.** Los textos de proyecto antiguos siguen abriéndose,
el campo nuevo sobrevive a guardar y abrir, y `format_for` da el número
nuevo solo para una canción que lo necesita.
`files_from_the_future_are_refused` construye su archivo con
`FORMAT_VERSION + 1`, así que sigue pasando.

### 12. Una copia de recuperación dañada (lineup-app, fácil)

**Objetivo.** Hacer fiable la recuperación tras un cierre inesperado
cuando la copia en disco no lo es. `app/recovery.rs` ya escribe una
copia del proyecto, un archivo `.info` y un archivo de bloqueo por cada
ventana en la carpeta `recovery` de la carpeta de datos de la app,
ofrece la copia en el siguiente arranque y elimina una copia que no se
puede cargar. El test `a_recovery_copy_comes_back_with_its_file` cubre el
camino feliz. Añade los otros.

**Archivos.** `crates/lineup-app/src/app/recovery.rs`.

**Pasos.**

1. Míralo a mano: abre una demo, escribe un acorde en el primer compás,
   espera un momento y fuerza el cierre de la app. No uses
   **Archivo > Salir**. Arráncala otra vez y elige **Recuperarlo**.
   Cierra la canción recuperada sin guardarla.
2. Lee `Files`, `load` y `write`, y los tests al final del archivo.
3. Añade un test que tome `Files::of_run(&folder, "test")`, cree la
   carpeta, escriba basura en su archivo `project`, ponga al lado un
   archivo `info` válido, llame a `load(files.clone(), None)` y compruebe
   que devuelve `None` **y** que los dos archivos han desaparecido.
4. Añade un segundo test que escriba un proyecto válido en
   `files.project` con `file::save`, pero sin el archivo info. Comprueba
   que `load` sigue devolviendo el proyecto, sin ruta y sin nombre, para
   que el diálogo lo llame «Sin título».
5. Borra la carpeta temporal al final de cada test, como hacen los que
   ya existen.

**Ejecuta.** `cargo test -p lineup-app recovery`.

**Terminado cuando.** Los dos tests pasan, y el primero falla si se
borra la llamada a `files.remove()` en la rama de error de `load`.

### 13. Olvidar un proyecto reciente (lineup-app, media)

Archivo > Abrir recientes recuerda los últimos diez proyectos en los
datos persistentes de egui, y sigue siendo correcto cuando un archivo se
movió o se borró (la apertura falla y la ruta se quita). Lee
`app/recent.rs`.

Amplíalo: un elemento «Borrar recientes» que olvide la lista entera, y
una pequeña ✕ junto a cada entrada que olvide una. La ✕ no debe abrir
además el archivo, así que fíjate en el orden en que compruebas los dos
clics. ¿Por qué se guarda la lista como `Vec<String>` y no como
`Vec<PathBuf>`, y qué le pasa a una ruta que no es UTF-8?

*Practica:* menús de egui, persistencia, rutas que no son UTF-8.

### 14. Leer números despacio (lineup-core y RON, media)

[El número de versión](saving.md#el-número-de-versión)
explica por qué la app ya no lee el formato de un archivo con un struct
de un solo campo: saltarse el resto del archivo hacía que RON mirara
hacia delante, número tras número, hasta el final del texto.

1. Supón que una canción guardada tiene 2 MB de texto con 120 000
   números. Si saltarse cada número lee, de media, medio archivo,
   ¿cuántos bytes se leen en total? A unos 2 GB por segundo, ¿cuánto
   tarda? ¿Y una canción el doble de larga?
2. Busca la búsqueda hacia delante en el código de RON: en la versión
   que nombra `Cargo.lock`, `src/parse.rs` busca `".."` en
   `self.src()[skip..]`. Reescribe esa línea para que busque solo en los
   primeros `raw_float_len` caracteres. (Pista: corta
   `self.src()[skip..][..raw_float_len]`, y piensa qué pasaría si el
   corte terminara en medio de un carácter.) Después compáralo con cómo
   lo hace RON 0.12.3, y lee su comentario sobre por qué un `..` no
   puede quedar a caballo del final.
3. `format_number` devuelve `None` para `(tempo: 120.0, format: 25)`.
   ¿Por qué no importa que solo lea el principio del archivo? (Mira el
   orden de los campos en `ProjectFile`, y `to_text`.)

*Practica:* pensar en ritmos de crecimiento, leer el código de una
biblioteca, porciones de cadenas.

### 15. Piénsalo: archivos

- `has_unsaved_changes` compara el proyecto entero con la copia
  guardada, pero solo después de que una edición o un deshacer active
  `check_dirty`, no en cada fotograma. ¿Cuántas notas, más o menos,
  necesitaría un proyecto para que una comparación ocupe una parte
  notable de un fotograma (unos 16 milisegundos)? ¿Por qué
  `project_edited` activa `check_dirty` solo cuando `dirty` ya es
  falso? ¿Qué añadiría a este diseño un contador que cada edición
  incrementara?
- El archivo de proyecto tiene sus propios tipos (`ProjectFile`,
  `PatternFile`) en lugar de derivar `Serialize` en el propio `Project`.
  ¿Qué fallaría el día que alguien renombrara un campo de `Project`, si
  el archivo lo usara directamente?
- Un Ferrous Lineup más antiguo se niega a abrir un archivo con un número
  de formato más nuevo. ¿Podría cargar lo que entiende e ignorar el
  resto? ¿Cuándo ayudaría eso, y cuándo perdería sin avisar el trabajo
  de alguien?
- El test de archivos dañados escribe `NaN` en lugar de números. ¿Qué
  pasa cuando un archivo de proyecto dice `tempo: NaN`? ¿Y
  `master: NaN`? Busca el código que decide cada caso, y di por qué son
  distintos.

## Dibujar las ventanas

### 16. Un doble clic restablece el volumen (lineup-app, fácil)

Haz que un doble clic en el deslizador VOL de la barra de herramientas
devuelva el volumen master a `lineup_core::Mixer::default().master`
(0.8). Un botón de restablecer no cabría: la barra de herramientas tiene
que caber en una fila en una ventana de 1280 puntos de ancho, también en
español, y `the_toolbar_fits_on_one_row_at_1280_points` lo comprueba.

Llama a `LineupApp::set_master_volume` (en `app/mixer.rs`) en lugar de
asignar el campo, para que el cambio sea un paso que se pueda deshacer y
el diff envíe al hilo de audio `Command::SetMasterVolume`. Ejecuta la
app y comprueba que el deslizador se mueve.

*Practica:* entrada en modo inmediato, `Response`, encaminar un cambio a
través de `edit`.

### 17. Recordar el nivel de las pistas (lineup-app y lineup-core, media)

El nivel de las pistas de las teclas de armonía vuelve a Breve cada vez
que arranca la app. Otras elecciones se recuerdan:
[Recordar entre ejecuciones](egui.md#recordar-entre-ejecuciones)
muestra cómo, y `settings.rs` guarda la elección de los grados en menor
con `get_persisted` e `insert_persisted` bajo un `Id` propio. Haz lo
mismo con `HintLevel`: dale `Serialize` y `Deserialize` de serde en
`lineup-core` (que ya depende de serde), y léelo y escríbelo donde
`HarmonyKeys` guarda `hints`. ¿Qué otras elecciones recordarías de la
misma manera?

*Practica:* derives de serde entre crates, los datos persistentes de
egui.

### 18. Pistas en todas partes (lineup-app, pequeño)

`theme::hint` da a un widget una línea en la barra de pistas y el mismo
texto como tooltip. La barra de herramientas, las teclas de armonía y
algunos botones de la lista de reproducción lo usan; el rack de canales y
el piano roll todavía usan `on_hover_text` a secas. Cámbialos a
`theme::hint`, y haz que la barra de pistas muestre el nombre del patrón
cuando no hay nada bajo el ratón. ¿Por qué `theme::hint` toma el
`Response` por valor y lo devuelve, en lugar de tomar `&Response`?

*Practica:* propiedad a través de una función, el `Response` de egui,
leer el módulo de otro antes de cambiarlo.

### 19. F10 para el metrónomo, y su línea en F1 (lineup-app, fácil)

**Objetivo.** El metrónomo es un interruptor en la barra de herramientas
y en **Edición > Ajustes… > Reproducción**. Dale una tecla y añade la
tecla a la lista de F1. F10 está libre; no uses **M** a secas, que toca
una nota cuando el teclado del equipo hace de piano.

**Archivos.** `crates/lineup-app/src/app/practice.rs`,
`crates/lineup-app/src/app.rs`,
`crates/lineup-app/src/app/shortcuts.rs`,
`crates/lineup-app/src/app/ux_tests.rs`.

**Pasos.**

1. En `practice.rs`, añade una constante `KeyboardShortcut` para F10,
   como `view.rs` guarda F5 a F8.
2. En `LineupApp::keyboard_shortcuts`, léela en el mismo sitio protegido
   que Inicio y Fin: no mientras un campo de texto tiene el teclado, y no
   mientras se usa un control. Llama a `set_metronome` con lo contrario
   de `practice.view().metronome`.
3. Añade una línea a la sección **En cualquier lugar** de
   `shortcuts.rs`: «Metronome on or off». Usa la constante en lugar de
   escribir la tecla dos veces.
4. Añade un test sin pantalla en `ux_tests.rs`: pulsa F10 y comprueba
   `practice.view().metronome`; púlsala otra vez y comprueba que está
   apagado.

**Ejecuta.**

```sh
cargo test -p lineup-app shortcuts
cargo test -p lineup-app metronome
```

**Terminado cuando.** F10 activa y desactiva el botón de la barra de
herramientas, F1 lo lista en «En cualquier lugar», y
`no_two_global_shortcuts_share_a_key` sigue pasando. Una línea nueva de
la lista de F1 es texto en inglés: el test
`every_window_writes_in_spanish` te avisa de que también necesita una
en español (mira [Dos idiomas](languages.md)).

### 20. Un dibujo y una versión (lineup-app, fácil)

- Dibuja otro icono para **Seguir el cursor de reproducción** en
  `icons.rs`: un cursor con dos flechas. ¿Qué test te avisa cuando una
  línea se sale de su cuadrado?
- Añade un botón **Copiar versión** a la ventana Acerca de Ferrous
  Lineup que ponga «Ferrous Lineup» y la versión real en el
  portapapeles con `ctx.copy_text`. ¿Puede el texto ser una `const`,
  construida con `concat!` y `env!`?

*Practica:* dibujar con egui, cadenas en tiempo de compilación.

### 21. Piénsalo: listas y ventanas

- En `shortcuts.rs`, la sección de las teclas de armonía usa
  `Written("1 to 7")`. ¿Por qué no siete `KeyboardShortcut` en un
  `Any`? ¿Qué mostraría entonces la lista?
- La partitura guía se abre con `Opening::FirstTime`. ¿Qué cambiaría
  para ti si usara `EveryTime`? Pruébalo, y di cuál prefieres.

## Los editores

### 22. Vaciar una fila (lineup-app, fácil)

Hacer clic derecho en un paso ya lo vacía (`RackAction::ClearStep`).
Añade «Vaciar los pasos» al menú del nombre de un canal en
`channel_menu`, con una nueva `RackAction::ClearRow { channel }`. En
`LineupApp::apply`, llama a `edit_pattern` para apagar todos los pasos
de ese canal, sin enviar a mano un comando al hilo de audio: el diff de
la edición envía las notas nuevas. ¿Por qué es mejor una edición de toda
la fila que un bucle de ediciones de un paso?

*Practica:* el patrón de acciones, mantener proyecto y audio a la par.

### 23. Pintar pasos arrastrando (lineup-app, media)

En el rack haces clic en un paso cada vez. Dale a
`widgets::step_button` un `Sense::click_and_drag()` y haz que
`Rows::steps` pinte cada paso por el que pasa el puntero. `Rows` se
reconstruye en cada fotograma y solo devuelve acciones, así que guarda el
primer paso, y si se estaba encendiendo o apagando, en `LineupApp` (o en
la memoria de egui) y pásalo.

Añade `RackAction::SetStep { channel, step, on }` en lugar de reutilizar
`ToggleStep`, para que un paso que el puntero cruza dos veces no vuelva
atrás. Comprueba `response.hovered()` junto con
`ui.input(|i| i.pointer.primary_down())`, y deja de pintar cuando el
puntero sale de la fila. ¿Debería ser todo el arrastre un solo paso de
deshacer? Mira [¿Cuándo termina un paso?](undo.md#cuándo-termina-un-paso)

*Practica:* estado en modo inmediato, entrada de egui, el patrón de
acciones.

### 24. Mostrar el grado de la escala en las teclas (lineup-app, fácil)

`paint_keys` en `piano_roll.rs` dibuja un punto en cada tecla de la
escala. Dibuja en su lugar el número del grado (1 para la tónica, 2,
3...), con `Scale::degree_of`. Una serie dodecafónica ya numera sus
notas; deja ese camino como está. Los músicos usan estos números todo el
tiempo («el acorde de quinto grado», «una séptima menor»).

*Practica:* `Option` en el código de dibujo, `painter.text`.

### 25. Las luces del rack (lineup-app, fácil)

1. Haz que la luz encendida sea verde en vez de naranja. ¿Dónde se
   define el color, y qué más lo usa?
2. Da a las luces del primer paso de cada compás un color propio, en vez
   de solo uno más brillante.
3. Mientras la canción toca un patrón distinto del que muestra el rack,
   las luces se quedan apagadas. Añade una nota tenue en el rack, como
   «No suena ahora», para ese caso. ¿Qué valores de
   `channel_rack::show` te dicen que suena la canción?

*Practica:* leer código de dibujo, colores de egui.

### 26. Nombres de acorde más cortos (lineup-app, media)

La franja de acordes del piano roll omite un nombre que no cabe en su
bloque. Haz que pruebe antes un nombre más corto: para un acorde con
bajo, el acorde sin su bajo («E7» para «E7/B»), y luego la fundamental
sola («E»). Mide cada uno con `layout_no_wrap` y dibuja el primero que
quepa. ¿Sigue siendo un acorde sin su bajo el nombre correcto de lo que
suena?

*Practica:* medir el texto antes de dibujarlo, iteradores con `find`.

### 27. Copiar un clip arrastrando con Alt (lineup-app, fácil)

**Ctrl+D** duplica los clips seleccionados, pero arrastrar una copia es
más rápido cuando la quieres en otro sitio. Mayús ya está ocupada
(Mayús+clic añade un clip a la selección), así que usa Alt.

En `PlaylistView::handle_input`, lee las teclas modificadoras con
`ui.input(|input| input.modifiers.alt)`. El plan obvio es añadir una
copia al pulsar y arrastrar la copia. Pero `add_clip` rechaza un clip
del mismo patrón que ya empieza en el mismo sitio, así que no se puede
añadir una copia encima del original. En su lugar, recuerda en
`ClipDrag` que este arrastre copia, y añade la copia solo cuando el
puntero llega a otra celda. ¿Qué índice recibe el clip nuevo en
`Project::playlist().clips()`, y cómo se entera el arrastre?

*Practica:* leer la entrada de egui, reutilizar acciones que ya tienes.

### 28. Un selector de color para los patrones (lineup-app, media)

Los colores de los patrones ya se guardan. `Pattern` tiene un `color`
público (un índice en `PALETTE`, en `playlist.rs`), `PatternFile` lo
escribe cuando no es el lugar del patrón en la lista, y los archivos que
no lo tienen cargan con el lugar del patrón como color. Lee
`PatternFile` y `pattern_color`.

Lo que falta es el selector. Añade uno al menú ⋯ del patrón en la barra
de herramientas, con un cuadradito por cada color de la paleta, que
cambie el color del patrón con `edit_pattern`. Después añade «Restablecer
colores», que devuelva a cada patrón su lugar en la lista. ¿Necesita un
cambio de color un `FORMAT_VERSION` nuevo? ¿Qué hace un Ferrous Lineup
más antiguo con un archivo que lo tiene?

*Practica:* hacer evolucionar un formato de archivo,
`#[serde(default)]`, encaminar un cambio a través de `edit`.

### 29. Ir a un compás (lineup-app, media)

Inicio, Fin, RePág y AvPág ya mueven el cursor de reproducción: lee las
teclas de transporte en `LineupApp::keyboard_shortcuts`, `seek` y
`skip_bars`. Añade una pequeña casilla «compás» junto a la posición,
donde escribir un número de compás y pulsar Intro lleve allí. El primer
paso de un compás depende de cada cambio de compás anterior: ¿qué
función convierte ya un paso en un compás, y puede ir en el otro
sentido? Mientras escribes en la casilla, Inicio y Fin tienen que mover
el cursor del texto, no el de reproducción: ¿qué impide que hagan las
dos cosas?

*Practica:* el foco del teclado, convertir compases en pasos.

### 30. Piénsalo: lo que recuerda un arrastre

El arrastre de una sola nota en el piano roll recuerda la nota por su
índice en la lista (`Drag::Move { index, .. }`), mientras que una
selección recuerda las notas mismas. ¿Qué podría salir mal con el índice
si la lista se mantuviera ordenada por inicio, y una nota se moviera
mientras se arrastra?

## Dos idiomas

### 31. Letras y solfeo (lineup-core y la app)

1. Con el solfeo elegido, escribe `Do` en un compás vacío de la hoja de
   acordes. Luego elige letras y escribe `Do` en el compás siguiente.
   ¿Qué contiene cada compás, y por qué son distintos?
2. Escribe en solfeo los siete acordes de Si♭ mayor, y compruébalos en
   las teclas de armonía. ¿Qué acorde es disminuido?
3. Un proyecto se guarda con letras más de una vez: `file::to_text` hace
   su trabajo dentro de `NoteNaming::Letters.with`, y lo mismo hace
   `From<HarmonyName> for String`. Quita todos los
   `NoteNaming::Letters.with` de los dos, dejando lo que hay dentro, y
   ejecuta `cargo test -p lineup-core solfege`. ¿Qué comprobación falla,
   y qué encontraría en tu archivo un amigo que usa letras? Luego vuelve
   a poner solo el de `From<HarmonyName>`. Los tests vuelven a pasar:
   ¿por qué, y por qué conservar también los otros?
4. `with_notes` deja `{0,1,4}` como está. Busca la línea de `with_notes`
   que lo decide, y di qué haría con `{Do}`.
5. `note_naming` podría guardar la nomenclatura del idioma la primera
   vez que no encuentra ninguna, con `insert_persisted`, para que la
   siguiente lectura la encuentre. Quedaría más ordenado. ¿Qué fallaría
   entonces para alguien que arranca la app en inglés y la cambia a
   español una semana después?

*Practica:* ajustes locales a cada hilo, porciones de cadenas, leer
tests.
