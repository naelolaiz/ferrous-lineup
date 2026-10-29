# Guardar y cargar proyectos

Archivos: [`crates/lineup-core/src/file.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/file.rs)
y [`crates/lineup-app/src/app/files.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/files.rs)

## Cómo es un archivo de proyecto

Un archivo `.lineup` es texto plano. Para ver uno, arranca la app, elige
una demo en Archivo > Demostraciones, luego usa Archivo > Guardar As… y abre el
archivo guardado en un editor de texto:

```sh
cargo run --release
# Archivo > Demostraciones > La canción inicial, luego Archivo > Guardar como… song.lineup
head -n 30 song.lineup
```

Un proyecto nuevo (Archivo > Nuevo) todavía no tiene notas, así que su
archivo es corto. Este es el comienzo real del archivo que escribe
`Project::template()`:

```text
(
    format: 1,
    tempo: 130.0,
    scale: (
        root: C,
        kind: Major,
    ),
    channels: [
        (
            name: "Kick",
            kind: Sampler(Builtin(Kick)),
            volume: 0.8,
            muted: false,
        ),
        (
            name: "Clap",
            kind: Sampler(Builtin(Clap)),
            volume: 0.8,
            muted: false,
        ),
        ...
        (
            name: "Lineup Osc",
            kind: Synth,
            volume: 0.6,
            muted: false,
        ),
    ],
    patterns: [
        (
            name: "Pattern 1",
            length: 16,
            notes: [
                [],
                ...
```

La canción de inicio es un poco distinta: su canal de sintetizador toca un
piano General MIDI, así que lleva `program: Some(0)` y su archivo dice
`format: 9`. Sus acordes se leen como notas, una por línea:

```text
                    (pitch: 48, start: 0, length: 8, velocity: 100),
                    (pitch: 60, start: 0, length: 8, velocity: 100),
                    (pitch: 64, start: 0, length: 8, velocity: 100),
                    (pitch: 67, start: 0, length: 8, velocity: 100),
```

El formato es **RON**, «Rusty Object Notation». Se parece a Rust: un
struct es `( field: value, ... )`, una lista es `[ ... ]`, y las variantes
de enum se escriben por su nombre, `Sampler(Builtin(Kick))`, igual que en
el código. JSON también serviría, pero JSON no tiene enums, así que el
mismo canal se leería como `{"Sampler": {"Builtin": "Kick"}}`. RON es más
fácil de leer y de arreglar a mano, algo que importa en un proyecto para
aprender: puedes abrir una canción en un editor de texto y ver tus propias
estructuras de datos.

## serde: describe los datos una sola vez

La forma estándar de Rust de convertir valores en texto (o bytes) y de
vuelta es el crate **serde**, abreviatura de *ser*ialize y *de*serialize.
serde no conoce ningún formato por sí mismo. Define dos traits,
`Serialize` y `Deserialize`, y los crates de formato como `ron` o
`serde_json` los usan.

Casi nunca escribes esos traits a mano. Una **macro derive** los escribe
por ti:

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Channel {
    pub name: String,
    pub kind: ChannelKind,
    pub volume: f32,
    pub muted: bool,
}
```

La macro lee la definición del struct en tiempo de compilación y genera
código que escribe cada campo por su nombre y los lee de vuelta. Para que
esto funcione, el tipo de cada campo debe ser también `Serialize`, así que
el derive se añadió a `ChannelKind`, `SampleSource`, `Drum`, `NoteEvent`,
`Scale`, `ScaleKind`, `PitchClass`, `Track` y `Clip`. `String`, los
números, `bool`, `Vec` y `PathBuf` vienen con serde.

El derive se activa pidiendo la feature `derive` de serde en `Cargo.toml`.
Las **features** son partes opcionales de un crate que enciendes cuando
las necesitas:

```toml
serde = { version = "1", features = ["derive"] }
```

### Mantener las reglas al leer: `from` e `into`

Desde el paso 1, un `Tempo` es imposible de crear fuera de 10 a 522 BPM,
porque su campo es privado y `Tempo::new` lo limita. Un `Deserialize`
derivado pondría el campo directamente desde el archivo, así que un
archivo que diga `tempo: 9000.0` crearía un tempo que el resto del
programa promete que no puede existir.

El arreglo es decirle a serde que pase por un tipo que controlamos:

```rust
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(from = "f32", into = "f32")]
pub struct Tempo {
    bpm: f32,
}

impl From<f32> for Tempo {
    fn from(bpm: f32) -> Self {
        Tempo::new(bpm)
    }
}

impl From<Tempo> for f32 {
    fn from(tempo: Tempo) -> Self {
        tempo.bpm
    }
}
```

`#[serde(from = "f32")]` significa «lee un `f32` y luego conviértelo con
`From<f32>`». Esa conversión es `Tempo::new`, que limita. La prueba
`values_out_of_range_are_clamped` edita un archivo guardado para que diga
9000 BPM y comprueba que carga como 522. `into = "f32"` hace lo contrario
al guardar, lo que además hace que el archivo diga `tempo: 130.0` en lugar
de `tempo: (bpm: 130.0)`. `Note` hace lo mismo con su número MIDI.

## Un tipo separado para el archivo

`Project` en sí no deriva serde. Sus campos son privados porque deben
estar de acuerdo entre ellos (una fila por canal, clips que apuntan a
patrones reales), y un `Deserialize` derivado los rellenaría directamente
desde el archivo, estén de acuerdo o no. Así que el archivo tiene su
propio struct:

```rust
#[derive(Debug, Serialize, Deserialize)]
struct ProjectFile {
    format: u32,
    #[serde(default, skip_serializing_if = "crate::Mixer::is_default")]
    mixer: crate::Mixer,
    tempo: Tempo,
    scale: Scale,
    #[serde(default, skip_serializing_if = "TimeSignature::is_common")]
    time_signature: TimeSignature,
    channels: Vec<Channel>,
    patterns: Vec<PatternFile>,
    tracks: Vec<Track>,
    clips: Vec<Clip>,
    // ...sections, a loop region, time, tempo, key and chord markers,
    // the band, and where each part came from. Later chapters add them.
}

#[derive(Debug, Serialize, Deserialize)]
struct PatternFile {
    name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    color: Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    time: Option<TimeSignature>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    player: Option<Player>,
    length: usize,
    /// One list of notes per channel, in the order of `channels`.
    notes: Vec<Vec<NoteEvent>>,
}
```

Las líneas `#[serde(default, skip_serializing_if = ...)]` son la forma
silenciosa de crecer del formato de archivo: un campo que está en su valor
por defecto no se escribe en absoluto, y un campo que falta en un archivo
más viejo se rellena con el valor por defecto del tipo. Por eso un
proyecto sin nada especial se sigue escribiendo como el mismo texto corto
de formato 1 de antes.

Un `ProjectFile` acepta cualquier cosa con la forma correcta. Convertirlo
en un `Project` es donde ocurre la comprobación, y se escribe como una
implementación de **`TryFrom`**, el trait estándar para «una conversión
que puede fallar»:

```rust
impl TryFrom<ProjectFile> for Project {
    type Error = FileError;

    fn try_from(file: ProjectFile) -> Result<Self, Self::Error> {
        let mut project = Project::empty();
        project.mixer = file.mixer;
        project.tempo = file.tempo;
        project.scale = file.scale;
        // ...checks on the mixer, the time signature and the channel mix...
        for channel in file.channels {
            project.add_channel(channel);
        }
        for (index, saved) in file.patterns.into_iter().enumerate() {
            if index > 0 {
                project.add_pattern();
            }
            let pattern = project.pattern_mut(index).unwrap();
            pattern.set_length(saved.length);
            for (channel, notes) in saved.notes.into_iter().enumerate() {
                for note in notes {
                    if pattern.add_note(channel, note).is_none() {
                        return invalid(format!(/* where, and what is wrong */));
                    }
                }
            }
        }
        // ...tracks, clips, sections, markers and the band...
        Ok(project)
    }
}
```

El proyecto se reconstruye **por la puerta principal**: `add_channel`,
`add_pattern`, `add_note`, `add_clip`, los mismos métodos que llama la app
cuando haces clic. Esos métodos ya mantienen todas las reglas, así que un
archivo nunca puede crear un proyecto que la UI no habría podido crear.
Cuando un método se niega (una nota que no cabe, un clip que reproduce un
patrón que no está), el error dice dónde, para que alguien que edite el
archivo a mano pueda arreglarlo.

`type Error = FileError;` es un **tipo asociado**: parte del trait que
elige cada implementación. Con `TryFrom` en su sitio, la biblioteca
estándar también proporciona el `try_into` correspondiente, así que
`let project: Project = file.try_into()?;` también funcionaría.

Guardar va en la otra dirección con `From<&Project> for ProjectFile`, que
copia los datos hacia fuera con `to_vec()` y `map`. No puede fallar, así
que es un `From`, no un `TryFrom`.

## El número de versión

```rust
pub const FORMAT_VERSION: u32 = 27;
```

`FORMAT_VERSION` es el layout más nuevo que este programa puede leer. Un
archivo no siempre lo usa: `format_for` escribe el formato más **bajo** que
puede contener el proyecto. Un proyecto que no usa nada nuevo se queda en
formato 1, para que un Ferrous Lineup más viejo pueda seguir abriéndolo.
La canción de inicio necesita el formato 9, porque su canal de
sintetizador toca un programa General MIDI; una canción con hoja de acordes
necesita al menos el formato 12; y así sucesivamente. La lista actual
llega hasta 27, para una batería enviada por MIDI. Cada entrada del largo
comentario encima de `FORMAT_VERSION` dice qué añadió esa versión y qué
haría mal un programa más viejo sin ella.

Rechazar un archivo más nuevo es más difícil de lo que parece. La forma
obvia es leer primero un struct diminuto:

```rust
#[derive(Deserialize)]
struct Header {
    format: u32,
}
```

Por defecto, serde se salta los campos que un struct no enumera, así que
un `Header` se puede leer del archivo entero. Pero para saltarse el resto
del archivo, RON tiene que mirar hacia delante a través de todo él. Con
una canción larga como Boléro, eso tardaba 85 segundos, incluso en una
compilación release. Ahora el archivo entero se abre en una fracción de
segundo, porque `format_number` lee solo los primeros bytes:

```rust
pub fn from_text(text: &str) -> Result<Project, FileError> {
    let text = text.strip_prefix('\u{feff}').unwrap_or(text);
    let file: ProjectFile = match ron::from_str(text) {
        Ok(file) => file,
        Err(err) => {
            return Err(match format_number(text) {
                Some(format) if format > FORMAT_VERSION => FileError::NewerFormat(format),
                _ => err.into(),
            });
        }
    };
    if file.format > FORMAT_VERSION {
        return Err(FileError::NewerFormat(file.format));
    }
    Project::try_from(file)
}

/// Reads `(format: 25, ...` without the RON reader.
fn format_number(text: &str) -> Option<u32> {
    let rest = text.trim_start().strip_prefix('(')?.trim_start();
    let rest = rest.strip_prefix("format")?.trim_start();
    let rest = rest.strip_prefix(':')?.trim_start();
    let digits = rest
        .find(|c: char| !c.is_ascii_digit())
        .unwrap_or(rest.len());
    rest[..digits].parse().ok()
}
```

`to_text` siempre escribe `format` primero, así que esto mira solo donde
debe estar el número. Si el archivo está tan roto que ni siquiera el
número se puede leer, el error de RON original es el que se informa. Y si
el archivo se parsea pero aun así dice un formato más nuevo, el número se
comprueba una vez más. Una marca de orden de bytes al principio (algunos
editores de texto añaden una) se elimina antes, para que no pueda esconder
el `(`.

Ese `#[serde(default)]` de `ProjectFile` es también como una versión
futura puede añadir un campo sin romper los archivos viejos: los archivos
que no lo tienen reciben el valor por defecto del tipo. La prueba
`files_from_the_future_are_refused` lee un archivo un formato más nuevo
que esta versión, hoy `(format: 29, sparkles: true)`, y espera
`NewerFormat(29)`.

## Errores como enum

```rust
#[derive(Debug)]
pub enum FileError {
    Io(io::Error),
    Parse(ron::error::SpannedError),
    Write(ron::Error),
    NewerFormat(u32),
    Invalid(String),
}
```

El mismo patrón que `EngineError` en el paso 1 y `SampleError` en el paso
2: una variante por cada tipo de problema, una implementación de `Display`
con un mensaje para las personas, y `std::error::Error` para que funcione
con cualquier tratamiento de errores de Rust. Aquí hay dos cosas nuevas.

**`From` para el `?`.** Dentro de una función que devuelve
`Result<_, FileError>`, el operador `?` solo puede pasar un error si sabe
cómo convertirlo en un `FileError`. Estas implementaciones se lo enseñan:

```rust
impl From<io::Error> for FileError {
    fn from(err: io::Error) -> Self {
        FileError::Io(err)
    }
}
```

Con una para `io::Error`, una para el error de parseo de `ron` y una para
su error de escritura, una función puede mezclar llamadas de ambas
librerías y usar `?` en todas ellas:

```rust
pub fn load(path: &Path) -> Result<Project, FileError> {
    let text = fs::read_to_string(path)?; // io::Error -> FileError::Io
    from_text(&text)
}
```

**`source`.** El trait `Error` tiene un método opcional que devuelve el
error de nivel inferior que envuelve un valor. Las herramientas que
informan de errores lo siguen para imprimir una cadena como «no se pudo
cargar: no existe el archivo». `FileError::Io` devuelve su `io::Error`;
`Invalid` no tiene nada debajo y devuelve `None`.

```rust
fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
    match self {
        FileError::Io(err) => Some(err),
        // ...
        FileError::NewerFormat(_) | FileError::Invalid(_) => None,
    }
}
```

`&(dyn Error + 'static)` es un **objeto trait**: una referencia a *algún*
valor que implementa `Error`, sin decir de qué tipo. Eso permite a un
método devolver un `io::Error` en un caso y un error de `ron` en otro.

## Guardar sin perder el archivo viejo

```rust
pub fn save(project: &Project, path: &Path) -> Result<(), FileError> {
    let text = to_text(project)?;
    let temporary = path.with_extension(format!("{EXTENSION}.tmp"));
    if let Err(err) = fs::write(&temporary, text) {
        let _ = fs::remove_file(&temporary);
        return Err(err.into());
    }
    if let Err(err) = fs::rename(&temporary, path) {
        let _ = fs::remove_file(&temporary);
        return Err(err.into());
    }
    Ok(())
}
```

Escribir directamente sobre `song.lineup` tiene un fallo feo: si el
programa se cae o el disco se llena a mitad de camino, el archivo queda
mitad viejo, mitad nuevo, y ninguna de las dos versiones se puede cargar.
Escribir primero en `song.lineup.tmp` y luego renombrarlo sobre el original
lo evita, porque el sistema operativo reemplaza un archivo al renombrar en
un único paso. O el rename ocurrió y el archivo nuevo está completo, o no
ocurrió y el archivo viejo queda intacto. En Windows, el `fs::rename` de
Rust también reemplaza un archivo existente, así que el mismo código
funciona en todas partes.

Las pruebas guardan en `std::env::temp_dir()`, la carpeta del sistema para
archivos temporales (`/tmp` en Linux, algo dentro de tu carpeta de usuario
en Windows), con el id del proceso en el nombre para que dos ejecuciones
de pruebas a la vez no colisionen, y borran el archivo después.

## En la app: menú Archivo, atajos de teclado y barra de título

El lado de la app está en `crates/lineup-app/src/app/files.rs`. Sus
primeras líneas explican un layout poco habitual:

```rust
//! This is a child module of `app` (its file is `app/files.rs`), so it can
//! use the private fields of [`LineupApp`]: in Rust, private means "visible
//! in this module and the modules inside it".
```

`app.rs` declara `mod files;`, y Rust busca ese módulo en `app/files.rs`.
El archivo contiene otro bloque `impl LineupApp`. Un tipo puede tener
cualquier número de bloques `impl`, en cualquier módulo de su crate, así
que un tipo grande se puede dividir por temas. Al ser hijo de `app`,
`files` ve los campos privados `self.project`, `self.audio` y demás, cosa
que un módulo hermano como `playlist` no podría.

### Elegir un archivo

Los diálogos de archivos vienen del crate **rfd** («Rusty File Dialog»),
que muestra el diálogo propio del sistema: GTK o KDE en Linux (a través
del portal del escritorio, o del programa `zenity`), el panel del Finder en
macOS, el diálogo del Explorer en Windows.

```rust
fn project_dialog() -> rfd::FileDialog {
    let extensions = [file::EXTENSION, file::OLD_EXTENSION];
    rfd::FileDialog::new().add_filter("Ferrous Lineup project", &extensions)
}

if let Some(path) = project_dialog().pick_file() {
    self.open_now(path);
}
```

`pick_file` espera hasta que el usuario elige un archivo o cancela, y
devuelve un `Option<PathBuf>`. La ventana no se redibuja mientras el
diálogo está abierto, pero la música sigue sonando, porque corre en el
hilo de audio. En un sistema Linux sin portal ni `zenity`, `pick_file`
devuelve `None` inmediatamente; el diálogo simplemente no aparece.

### «¿Guardar los cambios?»

¿Cómo sabe la app que hay cambios sin guardar? Mantiene una copia del
proyecto tal como estaba la última vez que se abrió o guardó, y compara.
Pero `Project` deriva `PartialEq`, así que `!=` recorre cada canal, nota y
clip. Eso es barato una vez, pero un arrastre edita el proyecto en cada
frame, y comparar la canción entera en cada frame es trabajo
desperdiciado. `FileState` por eso guarda la respuesta en caché:

```rust
/// Whether the project differs from `saved`, as last worked out.
dirty: bool,
/// Set when the project may differ from `saved`.
check_dirty: bool,
```

Una edición pone `check_dirty`, pero solo si el proyecto no estaba ya
sucio. Mientras está sucio, más ediciones no vuelven a pedir la
comparación entera; deshacer, rehacer y guardar la pedirán cuando la
necesiten:

```rust
pub(super) fn project_edited(&mut self) {
    if !self.dirty {
        self.check_dirty = true;
    }
}
```

Deshacer y rehacer, en cambio, pueden hacer que el proyecto vuelva a
coincidir con la copia guardada, así que siempre piden una comparación
nueva (`project_reverted`). La respuesta guardada en caché se lee cada
frame para el título, pero la comparación en sí solo corre cuando una
edición o un deshacer pone la marca:

```rust
pub(super) fn has_unsaved_changes(&mut self) -> bool {
    if self.file.check_dirty {
        self.file.dirty = match &self.file.saved {
            Some(saved) => self.project != *saved,
            None => true, // recovered work has never been saved
        };
        self.file.check_dirty = false;
    }
    self.file.dirty
}
```

Esto acierta en algo que una simple marca de sucio no lograría: deshacer
pone `check_dirty` incluso cuando el proyecto ya está sucio, de modo que
deshacer de vuelta a la copia guardada hace que la comparación los
encuentre iguales otra vez y quita la estrella. El precio es que las
ediciones normales no vuelven a recorrer el proyecto una vez que está
sucio; cambiar un paso a mano de vuelta puede dejar la estrella hasta el
siguiente deshacer, guardar o reabrir. Es un intercambio justo por no
comparar una canción entera en cada frame de un arrastre.

Cualquier cosa que fuera a tirar el proyecto espera antes una respuesta:

```rust
pub(super) enum Pending {
    New,
    Open,
    OpenPath(PathBuf),
    Quit,
}

fn request(&mut self, ctx: &egui::Context, action: Pending) {
    if self.has_unsaved_changes() {
        self.file.pending = Some(action);
    } else {
        self.perform(ctx, action);
    }
}
```

Mientras `pending` guarda algo, `unsaved_changes_dialog` muestra un
`egui::Modal`, una ventana que oscurece todo lo que hay detrás y se queda
con todos los clics:

![Preguntando antes de que se pierdan los cambios sin guardar](../images/step-4-save-changes.png)

Guardar guarda y después lleva a cabo la acción que esperaba, a menos que
guardar fallara o el usuario cancelara el diálogo de Guardar como; No
guardar la lleva a cabo inmediatamente; Cancelar, Esc o un clic fuera
la olvida. El valor `pending` es toda la máquina de estados: `None`
significa «nada esperando», `Some(Pending::Open)` significa «el usuario
quiere abrir un archivo, en cuanto decida sobre los cambios».

`pub(super)` hace el enum visible para el módulo padre (`app`), que lo
necesita para `FileState`, pero no más allá.

### Cerrar la ventana

Hacer clic en la × de la ventana le pide al sistema operativo que la
cierre. egui informa de la petición, y la app puede decir que no:

```rust
let close_requested = ctx.input(|input| input.viewport().close_requested());
if close_requested && !self.file.quitting && self.has_unsaved_changes() {
    ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
    self.file.pending = Some(Pending::Quit);
}
```

Cuando el usuario elige entonces Guardar o No guardar,
`perform(Pending::Quit)` pone `quitting` y envía `ViewportCommand::Close`,
que esta vez se deja pasar.

### Atajos de teclado

```rust
const SAVE: KeyboardShortcut = KeyboardShortcut::new(Modifiers::COMMAND, Key::S);
const SAVE_AS: KeyboardShortcut =
    KeyboardShortcut::new(Modifiers::COMMAND.plus(Modifiers::SHIFT), Key::S);

let pressed = |shortcut| ctx.input_mut(|input| input.consume_shortcut(&shortcut));
if pressed(SAVE_AS) {
    self.save_as();
} else if pressed(SAVE) {
    self.save();
}
```

- `Modifiers::COMMAND` es Ctrl en Linux y Windows y ⌘ en macOS, así que
  cada sistema tiene el atajo que esperan sus usuarios. El menú Archivo
  muestra también el correcto: `ctx.format_shortcut(&SAVE)` da «Ctrl+S» o
  «⌘S».
- `consume_shortcut` comprueba las teclas y las retira de la entrada, para
  que un campo de texto no las vea también.
- Ctrl+S también encajaría con Ctrl+Mayús+S (el Mayús extra se ignora),
  así que el atajo más largo se comprueba primero.

Estos valores `const` llaman a `KeyboardShortcut::new` mientras se
compila. Eso está permitido porque egui lo marcó como `const fn`, una
función lo bastante simple para ejecutarse en tiempo de compilación.

El título de la ventana muestra el nombre del archivo y una estrella para
los cambios sin guardar («song.lineup* · Ferrous Lineup»). La app envía un
`ViewportCommand::Title` solo cuando cambia el texto, no en cada frame.
(Las versiones anteriores usaban un punto antes del nombre; una demo ahora
muestra su propio nombre y dice que es de solo lectura: ver [Una demo
conserva su nombre](../step-8/demo-names.md).)

### Abrir un archivo desde fuera

```rust
let path = std::env::args_os().nth(1).map(PathBuf::from);
```

`ferrous-lineup song.lineup` empieza con la canción abierta, que es
también lo que hace un gestor de archivos cuando haces doble clic en un
archivo asociado a la app. `args_os` devuelve los argumentos como
`OsString`s, exactamente como los dio el sistema. `std::env::args`
entraría en pánico con un nombre de archivo que no es UTF-8 válido, cosa
que Linux permite. `nth(1)` se salta el primer argumento, que es el nombre
del propio programa.

Un archivo `.lineup` soltado sobre la ventana también se abre, tras la
misma pregunta «¿Guardar los cambios?». Los archivos WAV soltados siguen
yendo al rack de canales.
