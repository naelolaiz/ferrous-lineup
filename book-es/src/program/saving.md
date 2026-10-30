# Guardar: el archivo .lineup, la recuperación y los archivos dañados

Archivos: [`crates/lineup-core/src/file.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/file.rs),
[`crates/lineup-app/src/app/files.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/files.rs),
[`crates/lineup-app/src/app/recovery.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/recovery.rs),
[`crates/lineup-app/src/app/recent.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/recent.rs)
y [`crates/lineup-core/tests/damaged_files.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/tests/damaged_files.rs)

Una canción se guarda como un archivo `.lineup`. Este capítulo trata de
qué hay en ese archivo y por qué tiene esa forma, de cómo se lee un
archivo de una versión más vieja o más nueva, de cómo guardar evita
dejar nunca un archivo escrito a medias, y de cómo la aplicación guarda
una copia del trabajo sin guardar por si se cae. La lista numerada de
versiones del formato está en la página de referencia [Formato del
archivo de proyecto](../reference/project-format.md).

## Cómo es un archivo de proyecto

Un archivo `.lineup` es texto plano. Para ver uno, abre una demostración
en Archivo > Demostraciones, usa Archivo > Guardar como… y abre el
archivo guardado en un editor de texto. Archivo > Nuevo ritmo empieza una
canción con un patrón vacío y una batería; este es el archivo entero que
guarda:

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
                [],
                [],
                [],
                [],
            ],
        ),
    ],
    tracks: [
        (
            name: "Track 1",
            muted: false,
        ),
    ],
    clips: [],
)
```

Otras canciones llevan más. El canal de sintetizador de la canción de
inicio toca un piano General MIDI, así que lleva `program: Some(0)` y su
archivo dice `format: 9`. Sus acordes se leen como notas, una por línea:

```text
                    (pitch: 48, start: 0, length: 8, velocity: 100),
                    (pitch: 60, start: 0, length: 8, velocity: 100),
                    (pitch: 64, start: 0, length: 8, velocity: 100),
                    (pitch: 67, start: 0, length: 8, velocity: 100),
```

Archivo > Nuevo, una hoja de acordes vacía con una banda, guarda
`format: 17` y termina con la banda: `band: Some((style: "Pop beat", ...))`.

El formato es **RON**, «Rusty Object Notation». Se parece a Rust: un
struct es `( field: value, ... )`, una lista es `[ ... ]`, y las
variantes de enum se escriben por su nombre, `Sampler(Builtin(Kick))`,
igual que en el código. JSON también serviría, pero JSON no tiene enums,
así que el mismo canal se leería como `{"Sampler": {"Builtin": "Kick"}}`.
RON es más fácil de leer y de arreglar a mano, algo que importa en un
programa con el que se aprende: puedes abrir una canción en un editor de
texto y ver tus propias estructuras de datos.

## serde: describir los datos una sola vez

La forma estándar de Rust de convertir valores en texto (o bytes) y de
vuelta es el crate **serde**, abreviatura de *ser*ialize y
*de*serialize. serde no conoce ningún formato por sí mismo. Define dos
traits, `Serialize` y `Deserialize`, y los crates de formato como `ron`
o `serde_json` los usan.

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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub program: Option<u8>,
    #[serde(default, skip_serializing_if = "is_centred")]
    pub pan: f32,
    // ...the reverb send, the MIDI channel a drum is sent on, and the
    // channel's effects, each left out at its default.
}
```

La macro lee la definición del struct en tiempo de compilación y genera
código que escribe cada campo por su nombre y los lee de vuelta. Para
que esto funcione, el tipo de cada campo debe ser también `Serialize`,
así que `ChannelKind`, `SampleSource`, `Drum`, `NoteEvent`, `Scale`,
`PitchClass`, `Track`, `Clip` y los demás también lo derivan. `String`,
los números, `bool`, `Vec`, `Option` y `PathBuf` vienen con serde.

El derive se activa pidiendo la feature `derive` de serde en
`Cargo.toml`. Las **features** son partes opcionales de un crate que
enciendes cuando las necesitas:

```toml
serde = { version = "1", features = ["derive"] }
```

Las líneas `#[serde(default, skip_serializing_if = ...)]` son la forma
silenciosa de crecer del formato de archivo. Un campo que está en su
valor por defecto no se escribe en absoluto, y un campo que falta en un
archivo más viejo se rellena con su valor por defecto. Por eso la
batería de arriba, que no usa nada más allá del primer formato, se
escribe exactamente como lo escribía la primera versión del formato: un
canal en el centro y con todo en la reverberación no dice nada del
paneo ni del envío a la reverberación.

### Mantener las reglas al leer: `from` e `into`

Un `Tempo` no se puede crear fuera de 10 a 522 BPM, porque su campo es
privado y `Tempo::new` lo limita. Un `Deserialize` derivado pondría el
campo directamente desde el archivo, así que un archivo que diga
`tempo: 9000.0` crearía un tempo que el resto del programa promete que
no puede existir.

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
`values_out_of_range_are_clamped` edita un archivo guardado para que
diga 9000 BPM y comprueba que carga como 522. `into = "f32"` hace lo
contrario al guardar, lo que además hace que el archivo diga
`tempo: 130.0` en lugar de `tempo: (bpm: 130.0)`. `Note` hace lo mismo
con su número MIDI.

### Siempre en letras y en inglés

La aplicación puede mostrar los nombres de las notas en letras (C, D, E)
o en solfeo (Do, Re, Mi), y sus textos en inglés o en español (ver [Dos
idiomas](languages.md)). Un archivo no debe depender de
ninguna de las dos cosas: una canción guardada por alguien que lee
solfeo debe abrirse para alguien que lee letras. Así que `to_text` y
`from_text` fijan ambas mientras se ejecutan:

```rust
let file = Language::English.with(|| NoteNaming::Letters.with(|| ProjectFile::from(project)));
let mut text = NoteNaming::Letters.with(|| ron::ser::to_string_pretty(&file, style))?;
```

`with` fija la forma de nombrar para el hilo, ejecuta el closure y
vuelve a poner la anterior, así que la ventana vuelve a dibujar en
solfeo en su siguiente fotograma.

## Un tipo separado para el archivo

`Project` en sí no deriva serde. Sus campos son privados porque deben
estar de acuerdo entre ellos (una fila por canal, clips que apuntan a
patrones reales), y un `Deserialize` derivado los rellenaría
directamente desde el archivo, estén de acuerdo o no. Así que el
archivo tiene su propio struct:

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
    // ...sections, the loop region, time, tempo and key markers, loose
    // timing, swing, the chord sheet, the band, where each part came
    // from, and the locked channels: each left out when it is empty.
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

Un `ProjectFile` acepta cualquier cosa con la forma correcta.
Convertirlo en un `Project` es donde ocurre la comprobación, y se
escribe como una implementación de **`TryFrom`**, el trait estándar
para «una conversión que puede fallar»:

```rust
impl TryFrom<ProjectFile> for Project {
    type Error = FileError;

    fn try_from(file: ProjectFile) -> Result<Self, Self::Error> {
        let mut project = Project::empty();
        // ...checks on the mixer, the time signature and each channel's
        // mix, effects and MIDI channel...
        for channel in file.channels {
            project.add_channel(channel);
        }
        for (index, saved) in file.patterns.into_iter().enumerate() {
            // `Project::empty` already has one pattern; add the others.
            if index > 0 {
                project.add_pattern();
            }
            let Some(pattern) = project.pattern_mut(index) else { ... };
            pattern.set_length(saved.length);
            for (channel, notes) in saved.notes.into_iter().enumerate() {
                for note in notes {
                    if pattern.add_note(channel, note).is_none() {
                        return invalid(/* where, and what is wrong */);
                    }
                }
            }
        }
        // ...tracks, clips, sections, markers, the chord sheet and the band...
        Ok(project)
    }
}
```

El proyecto se reconstruye **por la puerta principal**: `add_channel`,
`add_pattern`, `add_note`, `add_clip`, los mismos métodos que llama la
aplicación cuando haces clic. Esos métodos ya mantienen todas las
reglas (ver [El proyecto](project.md)), así que un archivo
nunca puede crear un proyecto que la ventana no habría podido crear.
Cuando un método se niega (una nota que no cabe, un clip que reproduce
un patrón que no está), el error dice dónde, para que alguien que edite
el archivo a mano pueda arreglarlo: «el archivo del proyecto está
dañado: el clip 1 (patrón 99, pista 1) no entra en la lista de
reproducción».

`type Error = FileError;` es un **tipo asociado**: parte del trait que
elige cada implementación. Con `TryFrom` en su sitio, la biblioteca
estándar también proporciona el `try_into` correspondiente, así que
`let project: Project = file.try_into()?;` también funcionaría.

Guardar va en la otra dirección con `From<&Project> for ProjectFile`,
que copia los datos hacia fuera con `to_vec()` y `map`. No puede
fallar, así que es un `From`, no un `TryFrom`.

## El número de versión

```rust
pub const FORMAT_VERSION: u32 = 28;
```

`FORMAT_VERSION` es la disposición más nueva que este programa puede
leer. Un archivo no siempre la usa: `format_for` escribe el formato más
**bajo** que puede contener el proyecto. Un proyecto que no usa nada
nuevo se queda en formato 1, para que un Ferrous Lineup más viejo pueda
seguir abriéndolo. La canción de inicio necesita el formato 9, porque
su canal de sintetizador toca un programa General MIDI; una canción con
hoja de acordes necesita al menos el formato 12; una con sección de
vientos o con un efecto en un canal necesita el 28. El largo comentario
encima de `FORMAT_VERSION` dice, para cada número, qué añadió y qué
haría mal un programa más viejo sin él; [Formato del archivo de
proyecto](../reference/project-format.md) tiene la misma lista como
tabla.

Un archivo de una versión **más nueva** debe rechazarse con un mensaje
claro, no leerse mal. La forma obvia es leer primero un struct diminuto:

```rust
#[derive(Deserialize)]
struct Header {
    format: u32,
}
```

Por defecto, serde se salta los campos que un struct no enumera, así que
un `Header` se puede leer del archivo entero. Pero para saltarse el
resto del archivo, RON tiene que mirar hacia delante a través de todo
él, número a número. Con una canción larga como el Boléro, eso tardaba
más de un minuto, incluso en una compilación release. Así que primero
se lee el archivo entero, y solo si eso falla `format_number` lee el
número de los primeros bytes:

```rust
pub fn from_text(text: &str) -> Result<Project, FileError> {
    // A text editor may start the file with a byte-order mark, which is
    // not part of the RON text or of the format number.
    let text = text.strip_prefix('\u{feff}').unwrap_or(text);
    let file: ProjectFile = match NoteNaming::Letters.with(|| ron::from_str(text)) {
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
    NoteNaming::Letters.with(|| Project::try_from(file))
}

/// The format number at the start of a project file, `(format: 25, ...`,
/// read without the RON reader.
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

`to_text` siempre escribe `format` primero, así que esto mira solo
donde debe estar el número. Un archivo más nuevo a menudo no se puede
leer en absoluto, por campos o nombres que esta versión no conoce;
entonces el número de formato por sí solo explica por qué. Si el
archivo está tan roto que ni siquiera el número se puede leer, el error
de RON original es el que se informa. Y si el archivo se lee pero aun
así dice un formato más nuevo, también se rechaza, porque el mismo
texto puede significar otra cosa en ese formato. Una marca de orden de
bytes al principio (algunos editores de texto añaden una) se elimina
antes, para que no pueda esconder el `(`.

La prueba `files_from_the_future_are_refused` lee un archivo un formato
más nuevo que esta versión, hoy `(format: 29, sparkles: true)`, y espera
`NewerFormat(29)`, con el mensaje «este archivo usa el formato 29, hecho
por una versión más nueva de Ferrous Lineup (esta lee hasta el formato
28)».

## Errores como enum

```rust
#[derive(Debug)]
pub enum FileError {
    /// The file could not be read or written: missing, no permission, disk full...
    Io(io::Error),
    /// The text is not a valid project file. The error says where.
    Parse(ron::error::SpannedError),
    /// The project could not be turned into text (should never happen).
    Write(ron::Error),
    /// The file was saved by a newer version of Ferrous Lineup.
    NewerFormat(u32),
    /// The file is shaped right, but what it says does not make sense.
    Invalid(String),
}
```

Es el mismo patrón que `EngineError` (ver [Un tipo de error
propio](threads.md#un-tipo-de-error-propio)) y
`SampleError`: una variante por cada tipo de problema, una
implementación de `Display` con un mensaje para las personas (en el
idioma de la aplicación), y `std::error::Error` para que funcione con
cualquier tratamiento de errores de Rust. Dos cosas más lo hacen cómodo
de usar.

**`From` para el `?`.** Dentro de una función que devuelve
`Result<_, FileError>`, el operador `?` solo puede pasar un error si
sabe cómo convertirlo en un `FileError`. Estas implementaciones se lo
enseñan:

```rust
impl From<io::Error> for FileError {
    fn from(err: io::Error) -> Self {
        FileError::Io(err)
    }
}
```

Con una para `io::Error`, una para el error de lectura de `ron` y una
para su error de escritura, una función puede mezclar llamadas de ambas
bibliotecas y usar `?` en todas ellas:

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
        FileError::Parse(err) => Some(err),
        FileError::Write(err) => Some(err),
        FileError::NewerFormat(_) | FileError::Invalid(_) => None,
    }
}
```

`&(dyn Error + 'static)` es un **objeto trait**: una referencia a
*algún* valor que implementa `Error`, sin decir de qué tipo. Eso permite
a un método devolver un `io::Error` en un caso y un error de `ron` en
otro.

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
mitad viejo, mitad nuevo, y ninguna de las dos versiones se puede
cargar. Escribir primero en `song.lineup.tmp` y luego renombrarlo sobre
el original lo evita, porque el sistema operativo reemplaza un archivo
al renombrar en un único paso. O el renombrado ocurrió y el archivo
nuevo está completo, o no ocurrió y el archivo viejo queda intacto. En
Windows, el `fs::rename` de Rust también reemplaza un archivo
existente, así que el mismo código funciona en todas partes. Las
exportaciones WAV y de video y la copia de recuperación se escriben de
la misma manera.

`to_text` termina siempre las líneas con `\n` (RON usaría `\r\n` en
Windows), así que una canción es el mismo archivo en todos los
sistemas, y un proyecto guardado en un ordenador no muestra diferencias
en el control de versiones al guardarlo en otro.

Las pruebas guardan en `std::env::temp_dir()`, la carpeta del sistema
para archivos temporales (`/tmp` en Linux, algo dentro de tu carpeta de
usuario en Windows), con el id del proceso en el nombre para que dos
ejecuciones de pruebas a la vez no choquen, y borran el archivo
después.

## Los archivos dañados nunca hacen caer la aplicación

Un archivo de proyecto puede quedar cortado por un disco lleno,
editarse a mano, o ser otra cosa con un nombre `.lineup`. Los archivos
MIDI vienen de todos los programas que existen. Leer un archivo debe
terminar en un proyecto o en un mensaje de error, nunca en una caída. El
archivo de pruebas `damaged_files.rs` lo comprueba como lo hace un
**fuzzer**: daña archivos buenos de muchas formas pequeñas y lee cada
resultado.

El daño viene de un pequeño generador de números aleatorios, xorshift,
escrito en la prueba:

```rust
struct Damage(u64);

impl Damage {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
}
```

Empieza desde una semilla fija, así que cada ejecución daña los archivos
de la misma manera, y un fallo vuelve a aparecer en la siguiente
ejecución. No necesita ninguna dependencia nueva. En los archivos de
proyecto y las hojas de acordes, sustituye números por valores extremos
(`-1`, `65535`, `18446744073709551615`, `NaN`), recorta trozos y repite
trozos. En los archivos MIDI, cambia, recorta y repite bytes. Cada
prueba lee 600 copias dañadas. Un proyecto que se lee también debe
guardarse y volver a leerse.

La prueba captura un panic con `std::panic::catch_unwind`, así que un
fallo imprime el archivo dañado que lo causó, hasta sus primeros 4000
caracteres:

```rust
fn must_not_panic<T: std::fmt::Debug>(input: T, read: impl FnOnce(&T)) {
    if panic::catch_unwind(AssertUnwindSafe(|| read(&input))).is_err() {
        let shown = format!("{input:?}");
        let shown: String = shown.chars().take(4000).collect();
        panic!("reading this damaged file panicked:\n{shown}");
    }
}
```

El desbordamiento es el tipo de error que atrapa. Un archivo con un
cambio de tonalidad en el paso 18446744073709551615, el `usize` más
grande, no pasa la validación, como debe ser, pero escribir el mensaje
de error calculaba `marker.start + 1` para decir «paso …», y eso
desbordaba. Rust trata el desbordamiento de forma distinta en los dos
perfiles de compilación. En una compilación **debug**, `+` entra en pánico al
desbordar («attempt to add with overflow»). En una compilación
**release** da la vuelta hasta 0, en silencio. Así que una ejecución en
modo release sobre 15.000 archivos dañados no vio nada, y los mismos
archivos en modo debug se caían. Los mensajes usan `saturating_add(1)`,
que se queda en el número más grande en lugar de desbordar, y la
integración continua ejecuta las pruebas en modo debug (ver [Cómo se
prueba el código](../shipping/testing.md)).

## En la aplicación: el menú Archivo

El lado de la aplicación está en `crates/lineup-app/src/app/files.rs`.
Sus primeras líneas explican una disposición poco habitual:

```rust
//! This is a child module of `app` (its file is `app/files.rs`), so it can
//! use the private fields of [`LineupApp`]: in Rust, private means "visible
//! in this module and the modules inside it".
```

`app.rs` declara `mod files;`, y Rust busca ese módulo en
`app/files.rs`. El archivo contiene otro bloque `impl LineupApp`. Un
tipo puede tener cualquier número de bloques `impl`, en cualquier
módulo de su crate, así que un tipo grande se puede dividir por temas.
Al ser hijo de `app`, `files` ve los campos privados `self.project`,
`self.audio` y demás, cosa que un módulo hermano como `playlist` no
podría.

### Elegir un archivo

Los diálogos de archivos vienen del crate **rfd** («Rusty File
Dialog»), que muestra el diálogo propio del sistema: GTK o KDE en Linux
(a través del portal del escritorio, o del programa `zenity`), el panel
del Finder en macOS, el diálogo del Explorador en Windows.

```rust
fn project_dialog() -> rfd::FileDialog {
    rfd::FileDialog::new().add_filter(tr("Ferrous Lineup project"), &[file::EXTENSION])
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

Guardar como… le da al archivo la extensión `.lineup` aunque escribas
otra, así que `song.txt` se guarda como `song.lineup`, no como algo que
otro importador intentaría abrir.

### «¿Guardar cambios?»

¿Cómo sabe la aplicación que hay cambios sin guardar? Mantiene una copia
del proyecto tal como estaba la última vez que se abrió o guardó, y
compara. Pero `Project` deriva `PartialEq`, así que `!=` recorre cada
canal, nota y clip. Eso es barato una vez, pero un arrastre edita el
proyecto en cada fotograma, y comparar la canción entera en cada
fotograma es trabajo desperdiciado. `FileState` por eso guarda la
respuesta:

```rust
/// Whether the project differs from `saved`, as last worked out.
dirty: bool,
/// Set when the project may differ from `saved`; `dirty` is worked
/// out again the next time it is asked for.
check_dirty: bool,
```

Una edición pone `check_dirty`, pero solo si el proyecto no tenía ya
cambios. Mientras los tiene, los siguientes fotogramas de un arrastre
no vuelven a pedir la comparación entera:

```rust
pub(super) fn project_edited(&mut self) {
    if !self.dirty {
        self.check_dirty = true;
    }
}
```

Un cambio hecho de una vez (un clic), deshacer, rehacer y el final de
un arrastre pueden hacer que el proyecto vuelva a coincidir con la copia
guardada, así que siempre piden una comparación nueva con
`project_reverted` (ver [Decirle al hilo de audio lo que ha
cambiado](sync.md)). La respuesta guardada se lee en cada
fotograma para el título, pero la comparación en sí solo se hace cuando
la marca está puesta:

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

Así, un paso que se activa y se vuelve a desactivar quita el asterisco,
y también una nota arrastrada lejos y de vuelta, en cuanto la sueltas.

Cualquier cosa que fuera a tirar el proyecto espera antes una
respuesta:

```rust
pub(super) enum Pending {
    /// A chord sheet with a band.
    New,
    /// An empty pattern and a drum kit.
    NewBeat,
    Open,
    OpenPath(PathBuf),
    Demo(Demo),
    Import,
    ImportMidi,
    ImportChordSheet,
    Quit,
}

pub(super) fn request(&mut self, ctx: &egui::Context, action: Pending) {
    // ...a quit during an export asks about the export first...
    if self.has_unsaved_changes() || ... {
        self.file.pending = Some(action);
    } else {
        self.perform(ctx, action);
    }
}
```

Mientras `pending` guarda algo, `unsaved_changes_dialog` muestra un
`egui::Modal`, una ventana que oscurece todo lo que hay detrás y se
queda con todos los clics:

![Preguntando antes de que se pierdan los cambios sin guardar](../images/step-4-save-changes.png)

Guardar guarda y después lleva a cabo la acción que esperaba, a menos
que guardar fallara o el usuario cancelara el diálogo de Guardar como;
No guardar la lleva a cabo inmediatamente; Cancelar, Esc o un clic
fuera la olvida. El valor `pending` es toda la máquina de estados:
`None` significa «nada esperando», `Some(Pending::Open)` significa «el
usuario quiere abrir un archivo, en cuanto decida sobre los cambios».

`pub(super)` hace el enum visible para el módulo padre (`app`), que lo
necesita para `FileState`, pero no más allá.

El editor de estilos tiene su propio borrador sin guardar. Salir con un
borrador editado pregunta también **¿Guardar los cambios del estilo?**,
así que guardar la canción nunca tira el estilo en silencio.

### Cerrar la ventana

Hacer clic en la × de la ventana le pide al sistema operativo que la
cierre. egui informa de la petición, y la aplicación puede decir que
no:

```rust
let close_requested = ctx.input(|input| input.viewport().close_requested());
if close_requested
    && !self.file.quitting
    && (self.has_unsaved_changes() || self.style_editor.has_unsaved_changes())
{
    ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
    self.file.pending = Some(Pending::Quit);
}
```

Cuando el usuario elige entonces Guardar o No guardar,
`perform(Pending::Quit)` pone `quitting` y envía
`ViewportCommand::Close`, que esta vez se deja pasar.

Salir mientras una exportación WAV o de video aún se está ejecutando
pregunta **Una exportación aún se está ejecutando**, con **Detener y
salir** y **Continuar exportando**. Detener espera a que la exportación
se haya detenido y haya borrado su archivo sin terminar, y luego cierra,
así que nunca queda un archivo escrito a medias que parezca terminado.

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
  muestra también el correcto: «Ctrl+S» o «⌘S».
- `consume_shortcut` comprueba las teclas y las retira de la entrada,
  para que un campo de texto no las vea también.
- Ctrl+S también encajaría con Ctrl+Mayús+S (el Mayús de más se
  ignora), así que el atajo más largo se comprueba primero.

Estos valores `const` llaman a `KeyboardShortcut::new` mientras se
compila. Eso está permitido porque egui lo marcó como `const fn`, una
función lo bastante simple para ejecutarse en tiempo de compilación.

### La barra de título

El título de la ventana muestra el nombre de la canción y un asterisco
para los cambios sin guardar, «song.lineup* · Ferrous Lineup». Una
demostración muestra su propio nombre y dice que es de solo lectura,
«Für Elise (demostración, solo lectura) · Ferrous Lineup»: las
demostraciones vienen dentro de la aplicación, así que Guardar pide un
archivo nuevo, como Guardar como…. La aplicación envía un
`ViewportCommand::Title` solo cuando cambia el texto, no en cada
fotograma.

### Abrir un archivo desde fuera

```rust
let mut args = std::env::args_os().skip(1);
let first = args.next();
```

`ferrous-lineup song.lineup` empieza con la canción abierta, que es
también lo que hace un gestor de archivos cuando haces doble clic en un
archivo asociado a la aplicación. (El mismo primer argumento puede ser
en cambio `--trace`, `--demos` o `--help`, que hacen su trabajo sin
abrir ninguna ventana; ver [La traza de
reproducción](../sound/trace.md).) `args_os` devuelve los argumentos
como `OsString`s, exactamente como los dio el sistema. `std::env::args`
haría panic con un nombre de archivo que no es UTF-8 válido, cosa que
Linux permite. `skip(1)` se salta el primer argumento, que es el nombre
del propio programa.

Un archivo `.lineup` soltado sobre la ventana también se abre, tras la
misma pregunta «¿Guardar cambios?», y un archivo MIDI, una hoja de
acordes o una grabación se importan. Los demás archivos WAV soltados
van al rack de canales (en [Importar
grabaciones](../theory/listen.md#desde-el-menú-la-línea-de-comandos-o-soltada)
está cuál WAV es cuál).

## Archivo > Abrir recientes

**Archivo > Abrir recientes** lista los últimos diez proyectos abiertos
o guardados, el más reciente primero. La lista se guarda en los datos
que egui conserva, así que sobrevive a un reinicio. Cada elemento
muestra el nombre del archivo, con la ruta completa en su descripción
emergente. Abrir un archivo que ya no carga lo quita de la lista en
lugar de dejar una entrada muerta.

La lista guarda rutas, no copias de proyectos. Si un proyecto se mueve
o se renombra fuera de la aplicación, la ruta vieja se queda hasta que
falla un intento de abrirlo.

## Una copia del trabajo sin guardar

La aplicación escribe una copia de recuperación del proyecto mientras
tiene cambios sin guardar. El primer cambio escribe una enseguida, para
que la primera edición de una sesión no sea la que pierde una caída;
después escribe cada treinta segundos:

```rust
/// How often unsaved work is written while it keeps changing.
const INTERVAL: Duration = Duration::from_secs(30);
```

Una ventana que nadie toca no dibuja fotogramas, así que nunca llegaría
a escribir. Por eso, cuando toca una copia, `autosave` le pide a egui un
fotograma en ese momento con `request_repaint_after`: la copia se pone
al día en más o menos medio minuto después de un cambio, aunque nadie
toque la aplicación.

Los archivos están en una carpeta `recovery` dentro de la carpeta de
datos de la aplicación, la que le da `eframe::storage_dir(APP_NAME)`. En
Linux suele ser `~/.local/share/ferrouslineup/recovery`. Cada ejecución
de la aplicación nombra sus archivos según ella misma, como
`4211-1793047912612-0.lineup` y `4211-1793047912612-0.info`: el número
de proceso, el momento en que empezó en milisegundos, y un contador. Por
qué cada ejecución necesita los suyos está en [Dos ventanas a la
vez](#dos-ventanas-a-la-vez) más abajo. La copia **no** se escribe junto
al proyecto, porque un proyecto puede estar en un medio de solo lectura,
y un archivo de autoguardado en una carpeta de canciones sorprendería a
cualquiera que la mirase. El pequeño archivo `.info` lleva la ruta y el
nombre del proyecto, así que una canción recuperada vuelve como ella
misma, no como «Sin título».

La copia se escribe con `file::save`, que escribe un archivo temporal y
lo renombra. Una caída a mitad deja entera la copia anterior; el
archivo de información se escribe después del proyecto, así que una
caída entre los dos pierde el nombre en lugar de darle al proyecto uno
equivocado.

Al arrancar, si hay una copia, un diálogo pregunta **¿Recuperar el
trabajo sin guardar?** y nombra la canción:

- **Recuperarlo** vuelve a poner el proyecto, con su ruta y su nombre, y
  borra la copia. También carga el archivo del disco, si aún existe,
  como la versión «guardada», así que el trabajo recuperado muestra
  correctamente el asterisco de cambios sin guardar; deshacer hasta el
  archivo lo quita. Si el archivo ya no está, el proyecto recuperado
  simplemente no está guardado.
- **Descartarlo** borra la copia y deja el proyecto que la aplicación
  habría abierto de todos modos.

Una salida limpia borra la copia, porque no hay nada que recuperar. La
copia que queda es la señal de una caída, de un cierre forzado o de un
corte de luz. Una copia cuyo proyecto no carga se borra al arrancar en
lugar de ofrecerse. Un único `recovery.lineup` directamente en la
carpeta de datos, como lo escribían versiones anteriores, también se
ofrece.

### Dos ventanas a la vez

Nada te impide arrancar Ferrous Lineup dos veces. Si cada ejecución
escribiera la misma copia, la segunda ventana encontraría al arrancar la
copia de la primera y la ofrecería como si la primera se hubiera caído,
mientras seguía abierta; y el autoguardado de cada ventana sustituiría
la copia de la otra.

Así que cada ejecución escribe su propia copia, y junto a ella un tercer
archivo, `<run>.lock`, que **bloquea** antes de escribir la primera
copia y mantiene mientras se ejecuta. Un bloqueo de archivo es una
promesa que cumple el sistema operativo: solo un archivo abierto puede
tenerlo, y cuando el programa que lo tiene termina, el sistema lo
suelta, incluso en una caída o un corte de luz. Eso lo convierte en la
pregunta correcta al arrancar: «¿sigue en marcha la ejecución que
escribió esta copia?». Si su bloqueo se puede tomar, no sigue, y la
copia se ofrece; si no, la copia pertenece a una ventana que sigue
abierta y se deja en paz.

La biblioteca estándar tiene bloqueos de archivo desde Rust 1.89
(`File::lock`, `File::try_lock` y compañía). `try_lock` no espera: dice
enseguida si consiguió el bloqueo, y su error distingue «otro lo tiene»
de «este sistema no puede bloquear archivos»:

```rust
fn take_lock(path: &Path) -> Lock {
    // A copy whose lock file is gone was left by a run that closed with
    // work unsaved: the file is made again, to be taken.
    let Ok(file) = OpenOptions::new()
        .create(true)
        .truncate(false)
        .write(true)
        .open(path)
    else {
        return Lock::Unknown;
    };
    match file.try_lock() {
        Ok(()) => Lock::Taken(file),
        Err(TryLockError::WouldBlock) => Lock::Held,
        Err(TryLockError::Error(_)) => Lock::Unknown,
    }
}
```

El bloqueo dura lo que dura el valor `File`: soltarlo lo libera. Así
que una ventana que encuentra una copia guarda el `File` en el valor
`Recovered` hasta que respondes a la pregunta; una tercera ventana
abierta mientras tanto no puede tomar el bloqueo y no ofrece la misma
copia dos veces. Cuando la aplicación se cierra, el `Drop` de `Recovery`
borra su archivo de bloqueo; una copia de trabajo sin guardar se queda,
sin su bloqueo, para ofrecerse la próxima vez. Un sistema que no puede
bloquear archivos (`Lock::Unknown`) ofrece la copia de todos modos:
ofrecerla dos veces no pierde nada, no ofrecerla nunca sí.

Si se cayeron dos ejecuciones, hay dos copias. La más nueva se ofrece
primero, y la siguiente espera hasta que el trabajo recuperado antes
se guarde o se descarte, así que recuperar una nunca sustituye a otra.
