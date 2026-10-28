# Un nombre nuevo

Archivos: [`Cargo.toml`](https://github.com/naelolaiz/ferrous-lineup/blob/main/Cargo.toml),
[`crates/lineup-app/Cargo.toml`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/Cargo.toml),
[`crates/lineup-app/src/main.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/main.rs),
[`crates/lineup-app/src/app.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app.rs),
[`crates/lineup-app/src/midi.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/midi.rs),
[`crates/lineup-core/src/file.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/file.rs),
[`crates/lineup-core/src/midi_file.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/midi_file.rs)
y [`.github/workflows/ci.yml`](https://github.com/naelolaiz/ferrous-lineup/blob/main/.github/workflows/ci.yml)

Ferrous Loom es ahora **Ferrous Lineup**. Renombrar un programa suena a
buscar y reemplazar, y en su mayoría lo es. Las partes interesantes son
los lugares donde un nombre es una promesa a algo que ya existe: los
archivos guardados por la versión vieja, y los programas que los leen.

## Qué cambió

Todo lo que una persona ve o escribe, y los nombres de los archivos:

| Dónde | Antes | Después |
|-------|--------|-------|
| El título de la ventana | `song.loom · Ferrous Loom` | `song.lineup · Ferrous Lineup` |
| Archivos de proyecto | `song.loom` | `song.lineup` (y `song.loom` sigue abriéndose) |
| El nombre de archivo del programa | `ferrous-loom` | `ferrous-lineup` |
| Los crates (carpetas bajo `crates/`) | `loom-core`, `loom-audio`, `loom-listen`, `loom-app` | `lineup-core`, `lineup-audio`, `lineup-listen`, `lineup-app` |
| En el código | `use loom_core::Project;`, `LoomApp` | `use lineup_core::Project;`, `LineupApp` |
| Las descargas del CI | `ferrous-loom-linux`, … | `ferrous-lineup-linux`, … |
| El nombre que ven otros programas MIDI | `Ferrous Loom` | `Ferrous Lineup` |
| El nombre de pista en los archivos `.mid` exportados | `Ferrous Loom` | `Ferrous Lineup` |
| Los diálogos de abrir y guardar | «Ferrous Loom project» | «Ferrous Lineup project» |
| Una grabación decodificada junto a la original | `song.ferrous-loom.wav` | `song.ferrous-lineup.wav` |
| El sintetizador integrado | Loom Osc | Lineup Osc |
| Una fila en la ventana Twelve-tone | Loom row | Lineup row |
| La tonalidad oculta en los archivos `.mid` exportados | `Ferrous Loom key ` | `Ferrous Lineup key ` |
| Mensajes, el README, los comentarios, este libro | Ferrous Loom | Ferrous Lineup |

## Renombrar un crate

El nombre de un crate vive en tres lugares, y los tres tuvieron que
cambiar a la vez.

1. **Su carpeta**: `crates/loom-core` pasó a ser `crates/lineup-core`.
   Git lo registra como un renombrado (`git mv`), de modo que
   `git log --follow` sigue encontrando el historial anterior de un
   archivo.
2. **Su `[package] name`**, en su propio `Cargo.toml`, y cada lugar que
   lo nombra: la lista de `members` del workspace y sus
   `[workspace.dependencies]` en el
   [`Cargo.toml`](https://github.com/naelolaiz/ferrous-lineup/blob/main/Cargo.toml)
   de arriba, y los crates que dependen de él:

   ```toml
   [workspace]
   members = ["crates/lineup-core", "crates/lineup-audio", "crates/lineup-listen", "crates/lineup-app"]

   [workspace.dependencies]
   lineup-core = { path = "crates/lineup-core" }
   ```

   `Cargo.lock` también lista los paquetes por nombre; Cargo lo
   reescribe en la siguiente compilación.
3. **Su nombre en el código Rust.** Un crate llamado `lineup-core` se
   escribe `lineup_core` en las líneas `use`, porque un guion es un
   signo menos en Rust y no puede formar parte de un nombre. Cargo
   convierte los guiones en guiones bajos por ti, de modo que
   `use loom_core::Project;` se volvió `use lineup_core::Project;` en
   todas partes.

El propio tipo de la app, `LoomApp`, se volvió `LineupApp` con ellos,
de modo que el código que lees dice el mismo nombre que la ventana.

## Los archivos de proyecto viejos se siguen abriendo

Toda canción guardada hasta ahora termina en `.loom`. Un programa que
solo conociera `.lineup` ya no las vería en su diálogo Open, y soltar
una sobre la ventana no haría nada. Así que
[`file.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/file.rs)
conoce ambas:

```rust
pub const EXTENSION: &str = "lineup";

/// The extension projects had while the app was called Ferrous Loom.
/// Such files open as before, and saving one keeps its name.
pub const OLD_EXTENSION: &str = "loom";

pub fn is_project(path: &Path) -> bool {
    path.extension().is_some_and(|extension| {
        [EXTENSION, OLD_EXTENSION]
            .iter()
            .any(|known| extension.eq_ignore_ascii_case(known))
    })
}
```

`path.extension()` es la parte tras el último punto, sin el punto, o
`None` cuando no hay. Es un `OsStr`, la clase de texto del sistema
operativo, no un `String` de Rust, porque los nombres de archivo en
Linux no tienen por qué ser UTF-8 válido. `eq_ignore_ascii_case` lo
compara con nuestro texto, de modo que `SONG.LOOM` también cuenta.

El diálogo Open lista ambas clases, la ventana abre ambas cuando se
sueltan sobre ella, y la línea de comandos acepta ambas. Save As
sugiere siempre `.lineup`. Ctrl+S sobre un archivo `.loom` que abriste
escribe ese mismo archivo, con su nombre viejo: guardar en un
`song.lineup` nuevo a su lado dejaría dos copias de la canción, y la
próxima vez que abrieras la vieja no tendría tus cambios. El contenido
es el mismo en cualquier caso; solo difiere el nombre.

## Nombres guardados dentro de archivos

La primera versión del renombrado dejó tres cosas como estaban, porque
sus nombres se guardan dentro de archivos: el sintetizador **Loom Osc**
(todo proyecto tiene un canal llamado «Loom Osc»), la **Loom row**, y
la tonalidad oculta en los archivos MIDI exportados. Eso dejaba «Loom»
en el channel rack de todo proyecto nuevo, así que también se
renombraron, cada una con una manera de leer lo que se guardó antes.

**El sintetizador.** El nombre de un canal se guarda en el proyecto,
así que un proyecto viejo todavía tiene un canal llamado «Loom Osc».
Leer un archivo de proyecto lo renombra, en
[`file.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/file.rs):

```rust
const OLD_SYNTH_NAME: &str = "Loom Osc";

for mut channel in file.channels {
    if channel.kind == ChannelKind::Synth && channel.name == OLD_SYNTH_NAME {
        channel.name = Channel::synth().name;
    }
    project.add_channel(channel);
}
```

`for mut channel in` saca cada canal del archivo *por valor* y deja que
el loop lo cambie antes de entregárselo al proyecto: el `mut` pertenece
a la variable del loop, como en `let mut`. Solo cambia un sintetizador
con exactamente el nombre viejo. Un sampler al que tú mismo llamaste
«Loom Osc» conserva su nombre, y también un sintetizador que
renombraste.

**La fila** solo es un nombre en la lista de filas de la app. Un
proyecto guarda las doce notas de la fila, no su nombre, así que no hay
nada viejo que leer.

**La tonalidad oculta en los archivos MIDI.** Cuando la app exporta un
archivo `.mid`, escribe la tonalidad exacta, como Mi dórico, en un
evento propio que empieza con los bytes `\x7DFerrous Lineup key `
([Abrir archivos MIDI](../step-8/midi-import.md) lo lee de vuelta). Los
archivos exportados antes del renombrado empiezan con
`\x7DFerrous Loom key `, de modo que
[`midi_file.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/midi_file.rs)
acepta cualquiera de los dos:

```rust
let key = data
    .strip_prefix(SCALE_EVENT_START)
    .or_else(|| data.strip_prefix(OLD_SCALE_EVENT_START))?;
```

`strip_prefix` devuelve lo que sigue al prefijo, o `None` si los datos
no empiezan con él. `or_else` prueba el prefijo viejo solo cuando falta
el nuevo, y el `?` se rinde cuando no está ninguno. Una prueba lee un
evento viejo y recupera Mi dórico.

**El repositorio** también se renombró en GitHub, de
`naelolaiz/ferrous-loom` a `naelolaiz/ferrous-lineup`. GitHub reenvía
la dirección vieja a la nueva, así que los enlaces viejos siguen
funcionando.

## Nombre de paquete y nombre de programa

El programa se compila a partir del paquete `lineup-app`, pero el
archivo que escribe Cargo se llama `ferrous-lineup`. Los dos nombres
viven en lugares distintos de
[`crates/lineup-app/Cargo.toml`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/Cargo.toml):

```toml
[package]
name = "lineup-app"

[[bin]]
name = "ferrous-lineup"
path = "src/main.rs"
```

`[package] name` es como Cargo y los demás crates se refieren al
paquete (`cargo test -p lineup-app`). `[[bin]]` describe un programa
que compila el paquete; los corchetes dobles son la manera de TOML de
decir «una entrada de una lista», porque un paquete puede compilar
varios programas. Sin una entrada `[[bin]]`, Cargo llamaría al
programa como el paquete, `lineup-app`.

## Dónde se recuerda el tamaño de la ventana

`main` le entrega el nombre de la app a eframe. El nombre es la
constante `app::APP_NAME`, en
[`app.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app.rs),
de modo que la barra de título y la carpeta de ajustes usan un solo
string:

```rust
eframe::run_native(
    app::APP_NAME,
    options,
    Box::new(|cc| Ok(Box::new(app::LineupApp::new(cc, path)))),
)
```

eframe usa ese nombre para dos cosas: el título de la ventana cuando no
se da ninguno, y la carpeta donde guarda lo que recuerda entre
ejecuciones (el tamaño y el lugar de la ventana, y los zooms). En Linux
es `~/.local/share/ferrouslineup`, en macOS
`~/Library/Application Support/Ferrous-Lineup` y en Windows
`%APPDATA%\Ferrous Lineup\data`. La carpeta cambió con el nombre, de
modo que el primer arranque tras el renombrado abre la ventana en su
tamaño inicial y los zooms al 100%, una vez. Copiar encima la carpeta
vieja los habría conservado; por un tamaño de ventana no valía la pena
el código.

## Las pruebas también fijan nombres

Algunas pruebas cambiaron con el nombre. `FileState::title` se prueba
con títulos enteros como `"song.lineup* · Ferrous Lineup"`,
`is_project` con ambas extensiones, y el `is_own` del menú MIDI con
nombres de puerto:

```rust
fn is_own(name: &str) -> bool {
    name.starts_with(&format!("{CLIENT_NAME}:"))
}

assert!(is_own("Ferrous Lineup:output 130:0"));
assert!(!is_own("Ferrous Lineupish Synth:in 20:0"));
```

En Linux, los propios puertos de la app aparecen en sus listas de
puertos, y elegir uno enviaría cada nota en círculo, en un loop, así
que el menú los deja fuera. El dos puntos es por qué se cumple la
segunda línea: un sintetizador cuyo nombre simplemente empieza con
«Ferrous Lineup» es de otra persona.

## Ejercicios

1. El nombre de la app ya es una constante: `pub const APP_NAME` en
   `app.rs`, que usa `main.rs`. Termina el trabajo en los lugares que
   todavía lo escriben letra a letra: los títulos de ventana en
   `files.rs`, el nombre del cliente MIDI en `midi.rs`, el productor de
   PDF en `lead_sheet/pdf.rs`, las cabeceras del trace y la etiqueta
   del portapapeles. ¿Qué lugares pueden usar `APP_NAME`, y cuáles
   necesitan una constante propia? `lineup-core` (el nombre de pista y
   la tonalidad oculta en los archivos `.mid`) no puede depender del
   crate de la app, y el nombre viejo tiene que quedarse en las
   constantes `OLD_`. ¿Dónde tendría que vivir una constante
   compartida para servir a ambos crates?
2. Haz que Ctrl+S sobre un archivo `.loom` lo guarde como `.lineup` en
   su lugar, y borra el archivo viejo solo una vez que el nuevo esté
   escrito con seguridad. ¿Qué debería pasar con el título, y con el
   archivo que estaba abierto?
3. Haz que `ferrous-lineup --version` imprima la versión y salga, con
   `env!("CARGO_PKG_VERSION")`, una macro que Cargo rellena en tiempo
   de compilación a partir de `Cargo.toml`.
