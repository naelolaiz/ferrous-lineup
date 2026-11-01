# Compilarlo desde el código fuente

Para usar Ferrous Lineup alcanzan las descargas ya preparadas (ver
[Instalarlo](https://naelolaiz.github.io/ferrous-lineup/es/manual/installing.html),
en el manual). Para leer su código con este libro, cambiarlo y ejecutar
sus tests, compílalo tú. Son unos pocos comandos en Linux, macOS o
Windows.

## 1. Instalar Rust

Rust se instala con **rustup**, una pequeña herramienta que descarga el
compilador (`rustc`) y la herramienta de compilación (`cargo`) y los
mantiene al día:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

En Windows, descarga y ejecuta en cambio `rustup-init.exe` desde
<https://rustup.rs>. Después reinicia la terminal y comprueba que
funcionó:

```sh
cargo --version
rustc --version
```

Ferrous Lineup necesita Rust 1.95 o más nuevo, como dice `rust-version`
en el `Cargo.toml` raíz: usa la edición 2024 del lenguaje (ver
[Ediciones](program/crates.md#ediciones)), las cadenas de `if let`,
que llegaron en Rust 1.88, y los bloqueos de archivos, que llegaron en
1.89, y egui y eframe, que dibujan la ventana, necesitan 1.95. Si tu
Rust es más viejo, `rustup update` lo arregla.

## 2. Instalar las herramientas de compilación del sistema

Rust descarga sus propias bibliotecas, los *crates*, por su cuenta, pero
necesita un enlazador del sistema, y en Linux algunos crates hablan con
bibliotecas que trae el sistema: ALSA para el sonido, y X11 o Wayland
para las ventanas.

**Debian o Ubuntu:**

```sh
sudo apt install build-essential pkg-config libasound2-dev \
    libxkbcommon-dev libxkbcommon-x11-0 libwayland-dev
```

**Fedora:**

```sh
sudo dnf install gcc pkgconf-pkg-config alsa-lib-devel \
    libxkbcommon-devel libxkbcommon-x11 wayland-devel
```

Los diálogos de archivos usan el portal del escritorio, que GNOME y KDE
ya tienen; en un gestor de ventanas sin escritorio, instala
`xdg-desktop-portal-gtk`. El MIDI pasa por el *secuenciador* de ALSA,
que los sistemas de escritorio ya cargan; si el menú MIDI de la
aplicación dice que MIDI no está disponible, cárgalo con
`sudo modprobe snd-seq`.

**macOS:** las herramientas de línea de comandos de Apple traen el
enlazador y las bibliotecas del sistema:

```sh
xcode-select --install
```

**Windows:** cuando `rustup-init.exe` lo pregunte, deja que instale las
Visual Studio Build Tools; Rust usa su enlazador. Después usa los
comandos de abajo en PowerShell o en el Símbolo del sistema.

## 3. Conseguir el código y ejecutarlo

Clona el repositorio (Git viene con las herramientas de compilación en
macOS y Linux; en Windows, instálalo desde <https://git-scm.com>).
Instala también [Git LFS](https://git-lfs.com/): descarga las imágenes
y las fuentes que necesitan el programa y los libros. Luego compila:

```sh
git lfs install
git clone https://github.com/naelolaiz/ferrous-lineup
cd ferrous-lineup
git lfs pull
cargo run --release
```

Si ya tienes un clon, instala Git LFS y ejecuta `git lfs pull` antes
de compilar. Usa un clon en vez de un archivo del código fuente, que
puede contener solo punteros de LFS en lugar de los archivos.

La primera compilación tarda unos minutos, porque Cargo compila cada
dependencia; las siguientes compilan solo lo que cambió. `--release`
activa las optimizaciones, que el código de audio necesita para llegar
a tiempo. Mientras trabajas en el código, `cargo run` solo compila más
rápido, y conserva la consola donde aparecen los mensajes y los
pánicos.

![Ferrous Lineup al abrirse, en inglés: la Library a la izquierda, la canción de demostración en la lista de reproducción, y las teclas de armonía de do mayor abajo.](images/getting-started-window.png)

La primera vez, una ventana encima ofrece sonidos de instrumentos
reales; [SoundFonts y General MIDI](sound/soundfonts.md#la-oferta-y-la-descarga)
dice qué hace.

[La ventana
principal](https://naelolaiz.github.io/ferrous-lineup/es/manual/main-window.html),
en el manual, dice para qué sirve cada parte.

## 4. Desde la línea de comandos

El programa acepta algunos argumentos propios. Después de `cargo run`,
`--` dice que lo que sigue es para el programa, no para Cargo:

```sh
cargo run --release -- my-song.lineup                       # open a project
cargo run --release -- --demos                              # list the demos
cargo run --release -- --trace "Frère Jacques" --out round.tsv   # play a demo headless, note by note
cargo run --release -- --help
```

`--trace` es la [traza de reproducción](sound/trace.md): toca un
proyecto o una demostración como lo hace **Archivo > Exportar WAV…**,
sin ventana ni tarjeta de sonido, y escribe en una tabla cada nota que
empieza.

## 5. Ejecutar los tests

```sh
cargo test
```

ejecuta cada test de cada crate del workspace. [Cómo se prueba el
código](shipping/testing.md) dice qué tipos hay y cómo ejecutar
algunos de ellos.

## 6. Construir los libros

Los libros se escriben con [mdBook](https://rust-lang.github.io/mdBook/),
una herramienta de Rust. Instálalo una vez, en la versión que usa el
flujo de trabajo, y construye:

```sh
cargo install mdbook --version 0.5.4 --locked
cd book-es
mdbook serve --open
```

`mdbook serve` construye el sitio web, lo abre, y lo vuelve a construir
cada vez que guardas una página. `book/`, `manual/` y `manual-es/`
funcionan igual. [Cómo se construyen los libros](shipping/books.md)
también imprime el PDF.

## Comandos útiles

| Comando | Qué hace |
|---------|--------------|
| `cargo build` | Compila todo, en modo de depuración |
| `cargo run` | Compila y arranca la aplicación |
| `cargo test` | Ejecuta todos los tests |
| `cargo fmt` | Da formato a todo el código en el estilo estándar de Rust |
| `cargo clippy` | Ejecuta el linter, que sugiere mejores maneras de escribir las cosas |
| `cargo doc --open` | Construye la documentación de la API a partir de los comentarios `///` y la abre |
