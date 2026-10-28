# Linux, macOS y Windows

Archivos: [`.github/workflows/ci.yml`](https://github.com/naelolaiz/ferrous-lineup/blob/main/.github/workflows/ci.yml),
[`crates/lineup-app/src/main.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/main.rs)
y [`Cargo.toml`](https://github.com/naelolaiz/ferrous-lineup/blob/main/Cargo.toml)

El plan desde el principio fue «Linux primero, los demás al final, sin
reescribir nada». Aquí está todo el cambio al código del programa para
macOS y Windows: una línea en `main.rs`. Todo lo demás de este capítulo
trata de *por qué* eso es suficiente, y de cómo la CI lo demuestra en cada
push.

## Quién habla con el sistema operativo

Ferrous Lineup nunca llama al sistema operativo por sí mismo para el
sonido, las ventanas o los diálogos. Lo hacen cuatro crates:

| Tarea | Crate | Linux | macOS | Windows |
|-----|-------|-------|-------|---------|
| Sonido | `cpal` | ALSA (también a través de PipeWire o PulseAudio) | Core Audio | WASAPI |
| Ventana, teclas y ratón | `winit`, dentro de `eframe` | X11 o Wayland | AppKit | Win32 |
| Dibujo | `wgpu`, dentro de `eframe` | Vulkan u OpenGL | Metal | DirectX 12 o Vulkan |
| Diálogos de archivos | `rfd` | el portal del escritorio, o `zenity` | los paneles de abrir y guardar del Finder | el diálogo del Explorer |

Cada uno de estos crates contiene el código para todos los sistemas, y se
queda solo con el correcto al compilar. La app llama a
`cpal::default_host()`, dibuja con egui y abre un `rfd::FileDialog`; nada
de eso nombra un sistema operativo. El código propio del workspace tiene
solo dos comprobaciones de `target_os`, en el comando de previsualización
del lead sheet, que elige `cmd`, `open` o `xdg-open` para mostrar un PDF.
El código de audio, ventanas y diálogos nunca nombra un sistema.

Fue una elección hecha en el paso 1, cuando se eligió cada librería:
cualquier cosa que solo funcionara en Linux habría tenido que reemplazarse
ahora.

## Cómo guarda un crate el código de un sistema: `cfg`

Dentro de `cpal`, el host por defecto se elige así (abreviado):

```rust
#[cfg(any(target_os = "linux", target_os = "freebsd"))]
mod platform_impl {
    pub fn default_host() -> Host {
        AlsaHost::new().expect("the default host should always be available").into()
    }
}

#[cfg(target_vendor = "apple")]
mod platform_impl {
    pub fn default_host() -> Host {
        CoreAudioHost::new().expect("the default host should always be available").into()
    }
}
```

Dos módulos con el mismo nombre normalmente serían un error. Pero
`#[cfg(...)]` es un atributo de **compilación condicional**: cuando su
condición es falsa, el elemento que hay debajo se elimina antes de que el
compilador siquiera lo mire. En Linux solo existe el primer módulo; en un
Mac solo el segundo. Las condiciones pueden preguntar por:

- el sistema para el que compilas: `target_os = "windows"`, `target_os =
  "macos"`, `target_family = "unix"`, `target_vendor = "apple"`;
- el tipo de compilación: `debug_assertions` está activo en las
  compilaciones de debug, `test` mientras corre `cargo test` (los bloques
  `#[cfg(test)] mod tests` de cada archivo lo usan);
- las **features** de un crate: `feature = "wayland"`;
- y combinaciones, con `any(..)`, `all(..)` y `not(..)`.

Cargo puede hacer lo mismo con las dependencias. El propio `Cargo.toml` de
`cpal` solo pide la librería ALSA en los sistemas que la tienen:

```toml
[target.'cfg(any(target_os = "linux", target_os = "freebsd"))'.dependencies]
alsa = "0.11"
```

También hay una macro `cfg!(...)`, que da `true` o `false` como un valor
normal (`if cfg!(target_os = "macos") { ... }`). La diferencia importa:
con `cfg!`, ambas ramas se siguen compilando, así que ambas deben ser
válidas en todos los sistemas. Con `#[cfg]`, el código de otros sistemas
nunca se compila, y por eso puede llamar a funciones que solo existen
allí.

Ese es también su peligro. Una errata en una función `#[cfg(windows)]`
pasa desapercibida en Linux, porque el compilador nunca la lee. El código
que no puedes compilar es código en el que no puedes confiar; por eso es
mejor mantenerlo dentro de unos pocos crates bien probados, y por eso la
CI de abajo compila en todos los sistemas.

## Features: elegir las partes de un crate

La línea de `rfd` en el `Cargo.toml` del workspace apaga parte del crate:

```toml
rfd = { version = "0.17", default-features = false, features = ["xdg-portal"] }
```

Una **feature** es una parte opcional de un crate, nombrada en su
`Cargo.toml`. Algunas están encendidas salvo que digas lo contrario: son
las features `default`. La feature `derive` de `serde`, del capítulo de
guardar, es una que enciendes tú.

`rfd` normalmente enciende dos: `xdg-portal`, que le pide al escritorio de
Linux que muestre su diálogo de archivos (a través de Re-Bus, el sistema de
mensajes del escritorio, así que no hay GTK que instalar), y `wayland`,
que le dice al portal a qué ventana pertenece el diálogo, si el programa
lo indica. Ferrous Lineup no lo indica, así que `default-features = false`
deja caer `wayland`, y `features = ["xdg-portal"]` conserva la que hace
falta. Menos features significa menos crates que compilar. En macOS y
Windows la feature de Linux no cambia nada: los `#[cfg]`s dentro de `rfd`
dejan fuera su código.

Las features solo pueden *añadir*. Si dos crates de tu programa usan
`rfd`, uno con `wayland` y otro sin ella, Cargo lo compila una vez, con
`wayland`. Así que las features de un crate nunca deben quitar nada: un
crate con `wayland` debe seguir haciendo todo lo que hacía sin ella.

## Sin ventana de consola en Windows: `cfg_attr`

La única línea de código nuevo para Windows es el primer elemento de
`main.rs`:

```rust
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
```

En Windows, cada programa es o un programa de *consola*, que abre una
ventana de terminal al arrancar, o un programa de *ventanas*, que no lo
hace. Rust hace programas de consola salvo que se le diga otra cosa, así
que hacer doble clic en `ferrous-lineup.exe` abriría una ventana negra
vacía junto a la app.

- `windows_subsystem = "windows"` pide la segunda clase. En otros sistemas
  el atributo no hace nada.
- `#![...]`, con `!`, se aplica a aquello dentro de lo cual está, aquí
  todo el crate. `#[...]` sin `!` se aplica al siguiente elemento.
- `cfg_attr(condition, attribute)` aplica el atributo solo cuando la
  condición es cierta. `not(debug_assertions)` es cierto en las
  compilaciones release.

Así que `cargo run` (una compilación de debug) conserva la consola, donde
aparecen `println!` y los mensajes de pánico mientras desarrollas, y
`cargo build --release` hace el programa silencioso que das a otras
personas. En una compilación release en Windows, esos mensajes no van a
ninguna parte, lo que es una razón más para que la barra de estado muestre
los errores de audio y los mensajes «No se pudo abrir» y «No se pudo
guardar» dentro de la ventana.

## Pequeñas diferencias que el código ya maneja

- **Atajos de teclado:** `Modifiers::COMMAND` es Ctrl en Linux y Windows
  y ⌘ en macOS (ver [el capítulo de guardar](saving.md#atajos-de-teclado)).
- **Rutas:** `PathBuf` y `Path` manejan `/` y `\`, letras de unidad y
  nombres de archivo que no son texto válido. `std::env::args_os` pasa el
  archivo que abrir exactamente como lo dio el sistema.
- **Reemplazar un archivo:** `std::fs::rename` reemplaza un archivo
  existente en todos los sistemas, algo de lo que depende el [guardado
  seguro](saving.md#guardar-sin-perder-el-archivo-viejo).
- **Sonido en coma flotante:** cpal entrega al callback de audio de cada
  sistema el mismo `&mut [f32]`, así que el hilo de audio no sabe dónde
  corre.

## Demostrarlo: la matriz de CI

Hasta este paso, el trabajo principal de la CI corría en una máquina
Linux. Ahora el mismo trabajo corre tres veces:

```yaml
check:
  name: Build and test (${{ matrix.name }})
  strategy:
    fail-fast: false
    matrix:
      include:
        - name: Linux
          os: ubuntu-latest
          artifact: ferrous-lineup-linux
          binary: target/release/ferrous-lineup
        - name: macOS
          os: macos-latest
          artifact: ferrous-lineup-macos
          binary: target/release/ferrous-lineup
        - name: Windows
          os: windows-latest
          artifact: ferrous-lineup-windows
          binary: target/release/ferrous-lineup.exe
  runs-on: ${{ matrix.os }}
```

- Una **matriz** convierte un trabajo en varios. Cada entrada bajo
  `include` es una copia del trabajo, con sus propios valores para `name`,
  `os`, `artifact` y `binary`.
- `${{ matrix.os }}` se reemplaza por el valor de la entrada, así que
  `runs-on` elige una máquina Linux, Mac o Windows, y el nombre del
  trabajo muestra cuál es («Build and test (macOS)»).
- `fail-fast: false` deja que los otros trabajos terminen cuando uno
  falla. Sin él, un fallo de Windows cancelaría el trabajo del Mac, y solo
  descubrirías un segundo problema después de arreglar el primero.

Algunos pasos solo tienen sentido en un sistema:

```yaml
- name: Install system libraries
  if: runner.os == 'Linux'
  run: |
    sudo apt-get update
    sudo apt-get install -y libasound2-dev libxkbcommon-dev libwayland-dev

- name: Check formatting
  if: runner.os == 'Linux'
  run: cargo fmt --all --check
```

macOS y Windows traen sus librerías de sonido y ventanas, así que solo
Linux instala algo. El formato es el mismo en todas partes, así que se
comprueba una vez. Clippy, las pruebas y la compilación release corren en
los tres: clippy porque, como explicó la sección de `cfg`, cada sistema
compila un código ligeramente distinto, y las pruebas porque una prueba
que pasa en un sistema puede fallar en otro (una ruta con `/` dentro, un
archivo que no se puede renombrar estando abierto).

Cada trabajo termina subiendo su programa. En la página de una ejecución
de CI, bajo **Artifacts**, hay ahora tres descargas:
`ferrous-lineup-linux`, `ferrous-lineup-macos` y `ferrous-lineup-windows`.
GitHub mete cada uno en un archivo zip, y los zips hechos así olvidan que
el programa era ejecutable, así que en Linux y macOS ejecuta
`chmod +x ferrous-lineup` después de descomprimir. (El libro tiene su
propio workflow, `book.yml`, y su PDF es un artifact de ese workflow.)

## Comprobar el código de Windows desde Linux

Esperar a que la CI encuentre un error de Windows es lento. Rust puede
comprobar los tipos de tu código para otro sistema sin salir de Linux:

```sh
rustup target add x86_64-pc-windows-gnu
cargo clippy --workspace --all-targets --target x86_64-pc-windows-gnu -- -D warnings
```

Un **target** es el sistema *para el que* compilas, en contraposición al
sistema *en el que* compilas. `rustup target add` descarga la biblioteca
estándar de Rust compilada para Windows. `cargo clippy` (y `cargo check`)
solo ejecutan las comprobaciones del compilador y nunca enlazan un
programa, así que no necesitan el enlazador de Windows ni sus librerías de
sistema. Esto caza la mayoría de los errores: un `#[cfg]` solo de Windows
que no compila, o un crate que no soporta Windows en absoluto. No puede
cazar lo que solo ocurre cuando el programa corre.

Lo mismo funciona para Macs con `aarch64-apple-darwin` (los procesadores
de Apple) o `x86_64-apple-darwin` (los Intel Macs más viejos), siempre que
ninguna dependencia necesite las herramientas propias de Apple para
compilar. Ambas comprobaciones pasaron para este paso antes de que se
hiciera push de nada, así que la primera ejecución de CI en macOS y
Windows no fue un salto a oscuras.

## Compilarlo tú mismo

La página de [Primeros pasos](../getting-started.md) tiene los pasos para
cada sistema. En resumen:

- **macOS:** instala las herramientas de línea de comandos de Apple
  (`xcode-select --install`), que traen el enlazador y las librerías del
  sistema, después rustup, después `cargo run --release`.
- **Windows:** instala Rust con `rustup-init.exe` de rustup.rs. Ofrece
  instalar las Visual Studio Build Tools, que Rust usa para enlazar
  programas (la parte «Desktop development with C++»). Después
  `cargo run --release` en un terminal.

## Lo que todavía no está

Siendo honestos con los límites:

- Las máquinas de CI no tienen tarjeta de sonido ni pantalla. Los trabajos
  demuestran que el programa compila y que la lógica musical es correcta
  en cada sistema, no que se oiga nada. Una ejecución en un Mac o un
  Windows reales sigue siendo la prueba real.
- Los programas no están del todo **firmados**. La app de macOS está
  firmada ad-hoc, algo que Apple Silicon exige, pero no notarizada; el
  Setup de Windows y el `.exe` no están firmados. macOS se negará a abrir
  una app descargada al principio («no se puede verificar»), y Windows
  SmartScreen también avisará. [Compilaciones release: una descarga para
  cada sistema](../step-9/releases.md#abrir-lo-que-descargaste) muestra
  cómo pasar cada aviso. Notarizar la app de Mac y firmar los programas de
  Windows necesitan certificados de pago de Apple y Windows.
- La app de macOS tiene un icono y es un `.app` real dentro del `.dmg`,
  pero su `Info.plist` todavía no declara el tipo de documento `.lineup`,
  así que hacer doble clic en un proyecto en macOS puede no abrirlo en
  Ferrous Lineup. El instalador de Windows registra `.lineup`, y la
  entrada de escritorio y el tipo MIME de Linux también.
  Añadir el tipo de documento a `packaging/macos/Info.plist` es la pieza
  que falta.
