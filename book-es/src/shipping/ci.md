# Tres sistemas e integración continua

Archivos: [`.github/workflows/ci.yml`](https://github.com/naelolaiz/ferrous-lineup/blob/main/.github/workflows/ci.yml),
[`crates/lineup-app/src/main.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/main.rs),
[`crates/lineup-app/build.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/build.rs)
y [`Cargo.toml`](https://github.com/naelolaiz/ferrous-lineup/blob/main/Cargo.toml)

Ferrous Lineup funciona en Linux, macOS y Windows con un solo código.
Casi nada de ese código sabe en qué sistema está, y tres flujos de
trabajo en GitHub comprueban cada cambio en los tres. Este capítulo
trata de cómo un programa en Rust se mantiene portable, y de cómo se
ejecutan esas comprobaciones.

## Quién habla con el sistema operativo

Ferrous Lineup nunca llama por su cuenta al sistema operativo para el
sonido, las ventanas o los diálogos. Lo hacen cuatro crates:

| Tarea | Crate | Linux | macOS | Windows |
|-----|-------|-------|-------|---------|
| Sonido | `cpal` | ALSA (también a través de PipeWire o PulseAudio) | Core Audio | WASAPI |
| Ventana, teclas y ratón | `winit`, dentro de `eframe` | X11 o Wayland | AppKit | Win32 |
| Dibujo | `wgpu`, dentro de `eframe` | Vulkan u OpenGL | Metal | DirectX 12 o Vulkan |
| Diálogos de archivo | `rfd` | el portal del escritorio | los paneles de abrir y guardar del Finder | el diálogo del Explorador |

Cada uno de estos crates tiene el código de todos los sistemas y se
queda solo con el que corresponde al compilar. La aplicación llama a
`cpal::default_host()`, dibuja con egui y abre un `rfd::FileDialog`;
nada de eso nombra un sistema. Cada biblioteca se eligió por eso: algo
que funcionara en un solo sistema habría que reemplazarlo para llegar a
los demás.

El código propio del workspace nombra un sistema en cuatro lugares:

- `main.rs` esconde la ventana de consola en Windows (más abajo);
- `build.rs` pone el icono y la versión dentro del `.exe` de Windows;
- la partitura guía elige `cmd`, `open` o `xdg-open` para mostrar un PDF
  en el visor del sistema;
- la ventana de sonidos muestra una carpeta dentro de la carpeta
  personal como `~/...` solo en Linux y macOS, donde la gente escribe
  así las rutas.

## Cómo un crate guarda el código de un solo sistema: `cfg`

Dentro de `cpal`, el host por defecto se elige así (abreviado):

```rust,ignore
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

Dos módulos con el mismo nombre serían normalmente un error. Pero
`#[cfg(...)]` es un atributo de **compilación condicional**: cuando su
condición es falsa, el elemento que tiene debajo se quita antes de que
el compilador siquiera lo mire. En Linux solo existe el primer módulo;
en un Mac, solo el segundo. Las condiciones pueden preguntar por:

- el sistema para el que compilas: `target_os = "windows"`, `target_os =
  "macos"`, `target_family = "unix"` (o solo `unix`), `target_vendor =
  "apple"`;
- el tipo de compilación: `debug_assertions` está activo en las
  compilaciones de depuración, `test` mientras se ejecuta `cargo test`
  (cada `#[cfg(test)] mod tests` lo usa);
- las **features** de un crate: `feature = "wayland"`;
- y combinaciones, con `any(..)`, `all(..)` y `not(..)`.

Cargo puede hacer lo mismo con las dependencias. El `Cargo.toml` de
`cpal` pide la biblioteca de ALSA solo en los sistemas que la tienen:

```toml
[target.'cfg(any(target_os = "linux", target_os = "freebsd"))'.dependencies]
alsa = "0.11"
```

También hay una macro `cfg!(...)`, que da `true` o `false` como un valor
corriente. La partitura guía la usa:

```rust
let mut command = if cfg!(target_os = "windows") {
```

La diferencia importa. Con `cfg!`, todas las ramas se compilan igual,
así que todas tienen que ser válidas en todos los sistemas; con
`#[cfg]`, el código de los otros sistemas no se compila nunca, y por eso
puede llamar a funciones que solo existen allí. Ese es también su
peligro: un error de tipeo en una función `#[cfg(windows)]` pasa
inadvertido en Linux, porque el compilador nunca la lee. El código que
no puedes compilar es código en el que no puedes confiar, y por eso
conviene tenerlo dentro de unos pocos crates bien probados, y por eso la
integración continua de más abajo compila en todos los sistemas.

## Features: elegir las partes de un crate

La línea de `rfd` en el `Cargo.toml` del workspace apaga parte del
crate:

```toml
rfd = { version = "0.17", default-features = false, features = ["xdg-portal"] }
```

Una **feature** es una parte opcional de un crate, nombrada en su
`Cargo.toml`. Algunas están activas salvo que digas lo contrario: las
features `default`. `rfd` normalmente activa dos: `xdg-portal`, que le
pide al escritorio de Linux que muestre su propio diálogo de archivos
(por D-Bus, el sistema de mensajes del escritorio, así no hay que
instalar GTK), y `wayland`, que le dice al portal a qué ventana
pertenece el diálogo. Ferrous Lineup no se lo dice, así que
`default-features = false` quita `wayland`, y `features =
["xdg-portal"]` deja la que hace falta. Menos features significa menos
crates que compilar. En macOS y Windows la feature de Linux no cambia
nada: los `#[cfg]` dentro de `rfd` dejan su código afuera.

Las features solo pueden *agregar*. Si dos crates de un programa usan
`rfd`, uno con `wayland` y otro sin, Cargo lo compila una vez, con
`wayland`. Así que una feature nunca debe quitar nada: un crate con ella
tiene que seguir haciendo todo lo que hacía sin ella.

## Sin ventana de consola en Windows: `cfg_attr`

En Windows, cada programa es o un programa de *consola*, que abre una
ventana de terminal al arrancar, o un programa de *ventanas*, que no la
abre. Rust hace programas de consola salvo que se le diga otra cosa, así
que hacer doble clic en `ferrous-lineup.exe` abriría una ventana negra
vacía junto a la aplicación. Una línea al principio de `main.rs` dice
otra cosa:

```rust
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
```

- `windows_subsystem = "windows"` pide el segundo tipo. En los otros
  sistemas el atributo no hace nada.
- `#![...]`, con `!`, se aplica a aquello dentro de lo que está, aquí el
  crate entero. `#[...]` sin `!` se aplica al elemento siguiente.
- `cfg_attr(condition, attribute)` aplica el atributo solo cuando la
  condición es verdadera. `not(debug_assertions)` es verdadera en las
  compilaciones release.

Así `cargo run`, una compilación de depuración, conserva la consola,
donde aparecen los `println!` y los mensajes de pánico mientras
trabajas, y `cargo build --release` hace el programa silencioso que le
das a otras personas. Esos mensajes entonces no van a ninguna parte en
Windows, que es una razón más para que la aplicación muestre los errores
de audio y los mensajes «No se pudo abrir» y «No se pudo guardar» dentro
de su propia ventana.

## Un script de compilación para el `.exe`

`crates/lineup-app/build.rs` es un **script de compilación**: Cargo lo
compila y lo ejecuta antes de compilar el crate. En Windows pone el
icono, la versión y el copyright dentro del `.exe`, para que el
Explorador y la barra de tareas muestren el icono de la aplicación y la
página de Propiedades del archivo su nombre:

```rust
#[cfg(windows)]
{
    let version = env!("CARGO_PKG_VERSION");
```

Aquí `#[cfg(windows)]` describe la computadora que compila, porque un
script de compilación se ejecuta allí: el compilador de recursos al que
llama es el de Microsoft, que solo tiene Windows. El flujo de trabajo de
versiones compila el programa de Windows en Windows, así que el icono
siempre está. macOS y Linux toman sus iconos de los paquetes y los
archivos de escritorio de `packaging/` (ver [Paquetes de cada
versión](releases.md)).

## Pequeñas diferencias que el código ya resuelve

- **Atajos:** `Modifiers::COMMAND` es Ctrl en Linux y Windows y ⌘ en
  macOS.
- **Rutas:** `PathBuf` y `Path` manejan `/` y `\`, las letras de unidad
  y los nombres de archivo que no son texto válido.
  `std::env::args_os` pasa el archivo que hay que abrir tal como lo dio
  el sistema.
- **Reemplazar un archivo:** `std::fs::rename` reemplaza un archivo que
  ya existe en todos los sistemas, y de eso depende [guardar sin
  riesgo](../program/saving.md#guardar-sin-perder-el-archivo-viejo).
- **Sonido:** cpal le da a la función de audio de cada sistema el mismo
  `&mut [f32]`, así el hilo de audio nunca sabe dónde se ejecuta.

## Tres flujos de trabajo

Un **flujo de trabajo** (workflow) es un archivo en
`.github/workflows/` que le dice a GitHub qué ejecutar, en qué máquinas
y cuándo. Ferrous Lineup tiene tres:

| Flujo de trabajo | Se ejecuta en | Hace |
|---|---|---|
| **CI** (`ci.yml`) | cada pull request y cada push a main | revisa el formato y los lints, prueba y compila en Linux, macOS y Windows |
| **Release** (`release.yml`) | cada pull request, push a main y etiqueta `v` | compila las descargas para cada sistema, y las publica para una etiqueta (ver [Paquetes de cada versión](releases.md)) |
| **Book** (`book.yml`) | los cambios en los libros y los manuales | compila los sitios web y los PDF, y los publica desde main (ver [Cómo se construyen los libros](books.md)) |

Un pull request muestra una marca para cada trabajo, verde si pasó y
roja si falló, con un enlace a su registro.

### Un trabajo, tres máquinas: la matriz

El flujo de trabajo CI tiene un solo trabajo, `check`, que se ejecuta
tres veces:

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
  `include` es una copia del trabajo, con sus propios valores para
  `name`, `os`, `artifact` y `binary`.
- `${{ matrix.os }}` se reemplaza por el valor de la entrada, así
  `runs-on` elige una máquina Linux, Mac o Windows, y el nombre del
  trabajo dice cuál («Build and test (macOS)»).
- `fail-fast: false` deja que las otras copias terminen cuando una
  falla. Sin eso, un fallo en Windows cancelaría el trabajo del Mac, y
  un segundo problema aparecería solo después de arreglar el primero.

Cada copia ejecuta después los mismos pasos:

1. **Instalar las bibliotecas del sistema**, solo en Linux: los
   archivos de desarrollo de ALSA, xkbcommon y Wayland. macOS y Windows
   traen los suyos.
2. **Rust**, el estable más nuevo, con `rustfmt` y `clippy`.
3. **Una caché** de las descargas de Cargo y de las dependencias
   compiladas (`Swatinem/rust-cache`), así una ejecución compila solo lo
   que cambió.
4. **Revisar el formato**, solo en Linux: el formato es igual en todas
   partes.
5. **Lint**: `cargo clippy --workspace --all-targets -- -D warnings`.
6. **Tests**: `cargo test --workspace`, en una compilación de depuración
   (ver [Ejecutarlos](testing.md#ejecutarlos)).
7. **Compilar en release**, y subir el programa como artefacto de la
   ejecución.

Los pasos que solo tienen sentido en un sistema lo dicen:

```yaml
- name: Install system libraries
  if: runner.os == 'Linux'
```

Clippy y los tests se ejecutan en los tres: clippy porque cada sistema
compila un código un poco distinto, y los tests porque un test que pasa
en un sistema puede fallar en otro (una ruta escrita con `/`, un archivo
que no se puede renombrar mientras está abierto).

En la página de una ejecución, bajo **Artifacts**, están los tres
programas. GitHub comprime cada uno en un zip, y el zip olvida que el
programa era ejecutable, así que en Linux y macOS hace falta
`chmod +x ferrous-lineup` después de descomprimirlo. Las descargas de
verdad, con instaladores, salen del flujo de trabajo Release.

## Comprobar el código de Windows desde Linux

Esperar a que el CI encuentre un error de Windows es lento. Rust puede
comprobar el código para otro sistema sin salir de Linux:

```sh
rustup target add x86_64-pc-windows-gnu
cargo clippy --workspace --all-targets --target x86_64-pc-windows-gnu -- -D warnings
```

Un **target** es el sistema *para* el que compilas, a diferencia de
aquel *en* el que compilas. `rustup target add` descarga la biblioteca
estándar de Rust compilada para Windows. `cargo clippy` y `cargo check`
solo ejecutan las comprobaciones del compilador y nunca enlazan un
programa, así que no necesitan ni el enlazador de Windows ni sus
bibliotecas del sistema. Eso atrapa la mayoría de los errores, un
`#[cfg]` solo de Windows que no compila o un crate que no admite Windows
en absoluto, aunque no lo que solo pasa cuando el programa se ejecuta.
Lo mismo funciona para los Mac con `aarch64-apple-darwin` (los
procesadores de Apple) o `x86_64-apple-darwin` (los Mac con Intel),
siempre que ninguna dependencia necesite las herramientas propias de
Apple para compilar.

## Lo que las comprobaciones no pueden demostrar

Las máquinas del CI no tienen tarjeta de sonido ni pantalla. Los
trabajos demuestran que el programa compila y que la lógica musical es
correcta en cada sistema, no que se oiga algo: los tests de ventana
dibujan en memoria (ver [Tests de
ventana](testing.md#tests-de-ventana)), y los tests de
audio renderizan en un búfer. Ejecutarlo en un Mac o una computadora
con Windows de verdad sigue siendo la prueba real del sonido y del
dibujo.
