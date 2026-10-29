# Una demo conserva su nombre, y se guarda como una copia

Archivos: [`crates/lineup-app/src/app/files.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/files.rs)
y [`crates/lineup-app/src/app/library.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/library.rs)

Abre "Para Elisa" desde **Archivo > Demostraciones** y, hasta ahora, el título de
la ventana decía "Sin título". La app solo conocía dos clases de
proyecto: uno que venía de un archivo (mostrado por su nombre de
archivo) y uno que no ("Sin título"). Una demo no viene de ningún
archivo, así que caía en el segundo grupo, igual que un proyecto vacío
recién creado.

Ahora el título dice lo que está abierto:

![Für Elise recién abierta: su nombre, y que es una demo que no puedes cambiar.](../images/step-8-demo-title.png)

Cambia cualquier cosa (un paso, una nota, un ritmo de la biblioteca) y
aparece una estrella tras el nombre, como marcan la mayoría de los
programas los cambios sin guardar:

![Tras un cambio: "Für Elise*". Pasar el ratón sobre Guardar explica lo que hará.](../images/step-8-demo-changed.png)

La estrella reemplaza al punto que solía ir delante del nombre
(["En la app: menú Archivo, atajos de teclado y barra de título"](../step-4/saving.md#en-la-app-menú-archivo-atajos-de-teclado-y-barra-de-título)),
también para los archivos: "song.lineup*".

## Por qué una demo es de solo lectura

Una demo no es un archivo en ningún sitio de tu disco. Es código Rust,
compilado dentro del programa: `song_demo(song)` convierte las notas
escritas en `crates/lineup-core/src/library/song/` en un `Project` cada
vez que lo abres. No hay nada donde escribir tus cambios, y si lo
hubiera, la siguiente versión de la app traería su propia copia de
todas formas.

Así que **Guardar** en una demo hace lo que hace **Guardar como…**:
pide un archivo nuevo. El diálogo ya lleva el nombre de la demo: en Para Elisa sugiere "Für Elise.lineup", con el título original de la demo.

Una vez guardado, el proyecto es ese archivo: el título muestra "Für
Elise.lineup", lo de "(demostración, solo lectura)" desaparece, y el siguiente
Guardar escribe en el archivo sin preguntar. La demo del menú sigue
como estaba. Las exportaciones (WAV, MIDI) sugieren también el nombre
de la demo: "Für Elise.wav".

La app empieza con "La canción inicial", que también es una demo (es la
primera entrada de **Archivo > Demostraciones**), así que muestra ese nombre
desde el principio. **Archivo > Nuevo** y una grabación importada
siguen empezando como "Sin título"; un archivo MIDI importado o una hoja de acordes recibe el nombre de su archivo (`name_imported`), hasta que se
guarda.

## El código

`FileState`, las notas de la app sobre el archivo del proyecto, recibe
un campo más:

```rust
pub(super) struct FileState {
    /// Where the project was last opened from or saved to.
    path: Option<PathBuf>,
    /// The demo's name, while the project is a demo that has not been
    /// saved to a file yet.
    demo: Option<&'static str>,
    // ...
}
```

### `&'static str`

El nombre de la demo es un `&'static str`: un trozo de texto prestado
que vive durante toda la ejecución del programa. El nombre de cada
canción está escrito en el código fuente (`name: "Für Elise"`), así que
se guarda en el propio archivo del programa y nunca desaparece. Eso es
lo que promete `'static`, y por qué `FileState` puede guardar la
referencia sin poseer una copia (`String`) y sin ningún parámetro de
lifetime en el struct.

`Demo::name` lo entrega:

```rust
pub(super) fn name(self) -> &'static str {
    match self {
        Demo::Start => "The starting song",
        Demo::Song(song) => song.name,
        Demo::Style(style) => style.name,
        Demo::Row(style) => style.name,
        Demo::Micromode(style) => style.name,
    }
}
```

`open_demo` primero reemplaza el proyecto (lo que limpia `demo`, como
con cualquier otro proyecto) y luego escribe el nombre:

```rust
self.replace_project(demo.project(), None);
self.file.open_demo(demo);
```

Guardar en un archivo pone `path` y limpia `demo`, así que una copia
guardada es un archivo normal a partir de entonces.

### Hacer match sobre dos cosas a la vez

Qué mostrar depende de dónde vino el proyecto: un archivo, una demo, o
un nombre dado a una canción importada. `song_name` hace match sobre
las dos primeras en una tupla, y cae en `self.name`:

```rust
pub(super) fn song_name(&self) -> Option<String> {
    match (&self.path, self.demo) {
        (Some(path), _) => Some(file_name(path, Path::file_stem)),
        (None, Some(demo)) => Some(demo.to_string()),
        (None, None) => self.name.clone(),
    }
}
```

Los brazos se prueban de arriba abajo. `_` significa "cualquier cosa",
así que un archivo gana a un nombre de demo; el compilador comprueba
que los tres brazos juntos cubren todos los casos, y se negaría a
compilar si faltara uno. El último brazo es el nombre dado a un archivo
MIDI importado o una hoja de acordes (`name_imported`), que es un `String` y
se clona.

`&self.path` presta la ruta: hacer match sobre `self.path` en sí
intentaría mover el `PathBuf` fuera de `self`, cosa que un método que
solo tiene `&self` no puede hacer. `self.demo` está bien tal cual,
porque un `Option<&str>` es `Copy`: hacer match sobre él lo copia.

`display_name` mantiene entonces la extensión para un archivo, y dice
"Sin título" cuando no hay nombre en absoluto:

```rust
fn display_name(&self) -> String {
    match &self.path {
        Some(path) => file_name(path, Path::file_name),
        None => self.song_name().unwrap_or_else(|| "Untitled".to_string()),
    }
}
```

### Una función como valor

El nombre mostrado quiere "Für Elise.lineup" a partir de una ruta, el
nombre de exportación quiere "Für Elise" (el stem). Ambos vienen de un
método de `Path`, así que el helper toma el método como argumento:

```rust
fn file_name(path: &Path, part: fn(&Path) -> Option<&std::ffi::OsStr>) -> String {
    part(path).map_or_else(
        || "Untitled".to_string(),
        |name| name.to_string_lossy().into_owned(),
    )
}
```

`fn(&Path) -> Option<&OsStr>` es el tipo de una función normal (no una
closure que capture nada). `Path::file_name` y `Path::file_stem` tienen
ambos ese tipo, así que cualquiera de los dos se puede pasar:
`file_name(path, Path::file_stem)`.

### Una barra en un nombre

El nombre de una canción se vuelve un nombre de archivo sugerido. Un
nombre con `/` se leería como una carpeta en el diálogo, así que `stem`
lo reemplaza antes:

```rust
self.song_name().map_or_else(
    || "Untitled".to_string(),
    |name| name.replace(['/', '\\'], "-"),
)
```

`str::replace` toma un *patrón*, y un array de `char`s es un patrón que
coincide con cualquiera de ellos: las dos clases de barra se vuelven un
guion en una sola llamada.

### El título

```rust
fn title(&self, changed: bool) -> String {
    let star = if changed { "*" } else { "" };
    let demo = if self.path.is_none() && self.demo.is_some() {
        " (demo, read-only)"
    } else {
        ""
    };
    format!("{}{star}{demo} · Ferrous Lineup", self.display_name())
}
```

`title` toma `changed` como argumento en lugar de comparar proyectos él
mismo, para que `FileState` siga siendo comprobable sin una app entera:
las pruebas abren "Für Elise" en un `FileState`, piden el título con y
sin cambios, y comprueban el nombre de archivo sugerido.

## Cómo se comprobó

En la app en marcha: el título al arrancar ("La canción inicial (demo,
read-only)"), un paso cambiado (estrella), **Archivo > Demostraciones > Canciones > Clásica y de cine > Für Elise** (el diálogo "¿Guardar los
cambios?" nombra "La canción inicial"), No guardar, el título "Für Elise
(demo, read-only)" mientras suena, un paso cambiado (estrella),
**Archivo > Guardar** abriendo el diálogo de guardar del sistema con
"Für Elise.lineup", guardar, el título volviéndose el nombre del
archivo sin estrella, y un segundo Guardar escribiendo el archivo sin
diálogo.
