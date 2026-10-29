# Primeros pasos

## 1. Instalar Rust

Rust se instala con **rustup**, una pequeña herramienta que descarga el
compilador (`rustc`), la herramienta de compilación (`cargo`) y los
mantiene al día:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Reinicia la terminal después y comprueba que funcionó:

```sh
cargo --version
rustc --version
```

Ferrous Lineup necesita Rust 1.95 o más reciente. Usa la edición 2024
del lenguaje (más sobre las ediciones en
[Cargo, workspaces y crates](step-1/workspace.md)), las cadenas `if let`,
que llegaron en la 1.88, y los bloqueos de archivo, que llegaron en la
1.89; egui y eframe, que dibujan la ventana, necesitan la 1.95. Si tu
Rust es más antiguo, `rustup update` lo arregla.

## 2. Instalar las herramientas de compilación del sistema

Rust descarga sus propias librerías (llamadas *crates*)
automáticamente, pero necesita un linker de tu sistema, y en Linux
algunos crates hablan con librerías que vienen con el propio sistema:
ALSA para el sonido y X11 o Wayland para las ventanas.

### Linux

En Debian o Ubuntu:

```sh
sudo apt install build-essential pkg-config libasound2-dev \
    libxkbcommon-dev libxkbcommon-x11-0 libwayland-dev
```

En Fedora:

```sh
sudo dnf install gcc pkgconf-pkg-config alsa-lib-devel \
    libxkbcommon-devel libxkbcommon-x11 wayland-devel
```

Los diálogos de archivos usan el portal de tu escritorio, que GNOME y
KDE ya traen. En un window manager a secas, instala
`xdg-desktop-portal-gtk` o `zenity` para tenerlos.

El MIDI pasa por el *sequencer* de ALSA. Los sistemas de escritorio ya
lo cargan; si el menú MIDI de Ferrous Lineup dice **MIDI no disponible**,
cárgalo con `sudo modprobe snd-seq`.

### macOS

Instala las herramientas de línea de comandos de Apple, que traen el
linker y las librerías del sistema:

```sh
xcode-select --install
```

### Windows

Descarga y ejecuta `rustup-init.exe` desde <https://rustup.rs> en lugar
del comando `curl` anterior. Cuando pregunte, deja que instale las
Visual Studio Build Tools: Rust usa su linker. Luego usa los comandos de
abajo en PowerShell o en el símbolo del sistema.

## 3. Obtener el código y compilarlo

Clona el repositorio (Git forma parte de las herramientas de compilación
en macOS y Linux; en Windows, instálalo desde <https://git-scm.com>) y
entra en su carpeta:

```sh
git clone https://github.com/naelolaiz/ferrous-lineup
cd ferrous-lineup
```

Si en su lugar descargaste un archivo con el código fuente,
descomprímelo y entra en esa carpeta. Luego compila y ejecuta:

```sh
cargo run --release
```

La primera compilación tarda unos minutos porque Cargo compila cada
dependencia. Las compilaciones posteriores solo recompilan lo que ha
cambiado y tardan segundos. `--release` activa las optimizaciones, algo
que importa en el código de audio. Durante el desarrollo puedes quitarlo
para compilar más rápido.

Deberías ver esta ventana:

![Ferrous Lineup al abrirse: la Library a la izquierda, la canción de demostración en el playlist, el channel rack encima con Kick, Clap, Hat, Snare y el Lineup Osc, y las harmony keys de do mayor en la parte de abajo.](images/getting-started-window.png)

Pulsa **Espacio** o haz clic en **▶** para oír la canción de
demostración: dos compases de acordes, luego entra un pulso, y un
relleno de batería la termina antes de que se repita en bucle. La
primera vez, una ventana se ofrece a descargar un **SoundFont** gratuito,
grabaciones de instrumentos reales: el sintetizador propio de la app
solo se parece de lejos a un piano o a una batería. **Descargar** lo
trae (unos 30 MB, una sola vez) y toca los instrumentos con él; **Ahora
no** se queda con el sintetizador ([Sonidos de instrumentos reales: un
SoundFont](step-9/soundfont.md) cuenta más). La lista de reproducción llena
la ventana; **F6** muestra y oculta el rack de canales, **F7** el piano
roll, y **L** alterna entre reproducir la canción y repetir en bucle el
patrón seleccionado. **Ctrl+S** (⌘S en un Mac) guarda el proyecto,
**Ctrl+Z** deshace el último cambio y **Ctrl+Mayús+Z** lo rehace, y
**Archivo > Exportar WAV…** guarda la canción como un archivo de sonido. En
el piano roll, **Ctrl+clic** selecciona notas, las **teclas de flecha**
las mueven, y el menú **Herramientas** enumera todo lo demás que puede hacer
con ellas, con sus teclas. El menú **MIDI** conecta un teclado MIDI o un
sintetizador, y convierte las teclas de letras en un piano; **⏺**
graba lo que tocas. La barra de estado, abajo, muestra qué tarjeta de
sonido está en uso, o por qué el audio no pudo arrancar, y pone nombre
al acorde que mantienes pulsado.

Para tocar una canción tuya con la banda, elige **Archivo > Nuevo**. Se abre
una **hoja de acordes** vacía: haz clic en un compás y escribe sus acordes
(`C`, `Am7`, `G7/B`), y un piano, un bajo y una batería los tocan, cada
uno en su propia pista. **🎷 Banda** elige a los músicos, y la
**Biblioteca**, el estilo en el que tocan. **F5**, o **📝 Acordes** en la
barra de herramientas, abre la hoja de acordes de cualquier canción, también
de las demos. El [Paso 9](step-9/README.md) cuenta cómo funciona la
banda.

Encima de la barra de estado están las **teclas de armonía**: la tonalidad y
el compás, y una tecla grande para cada acorde de la tonalidad. Mantén
una pulsada, o pulsa de **1** a **7**, para tocar su acorde. La
**Biblioteca**, a la izquierda, contiene los estilos y los ritmos: haz
clic en uno y luego en **Aplicar** para oír la canción tocada de esa
manera; la pestaña **Patrones**, a su lado, enumera los patrones de la
canción. **Archivo > Demostraciones** abre canciones enteras: prueba *Para Elisa* o *Frère Jacques*, no pulses nada y mira cómo las teclas de armonía
siguen a los acordes.

[Un recorrido por la aplicación](tour.md) pasa por cada ventana, menú y
tecla.

Para abrir un proyecto directamente, pásale su archivo al programa:

```sh
cargo run --release -- my-song.lineup
```

El `--` le dice a Cargo que lo que sigue es para el programa, no para
Cargo.

### Sin compilar: descargas ya listas

Cada ejecución de CI en GitHub compila el programa para Linux, macOS y
Windows. En la página de la ejecución, bajo **Artifacts**, descarga
`ferrous-lineup-linux`, `ferrous-lineup-macos` o
`ferrous-lineup-windows` y descomprímelo. En Linux y macOS, hazlo
ejecutable primero con `chmod +x ferrous-lineup`. Son programas pelados,
no instaladores, y no están firmados, así que macOS y Windows te
avisarán antes del primer arranque;
[Linux, macOS y Windows](step-4/platforms.md#lo-que-todavía-no-está)
explica cómo pasar del aviso.

Para algo listo para instalar, usa la
**[página de Releases](https://github.com/naelolaiz/ferrous-lineup/releases)**
del repositorio. Cada etiqueta `v` publica los paquetes que compila el
workflow **Release**: AppImage de Linux, `.deb` y `.tar.gz`; Setup
`.exe` y `.zip` portable para Windows; y un `.dmg` de macOS con
`Ferrous Lineup.app`, para Apple Silicon y para Intel. La ejecución de
Release también los guarda bajo **Artifacts**.
[Compilaciones de lanzamiento: una descarga para cada sistema](step-9/releases.md)
explica cada uno, y lo que dicen macOS y Windows en el primer arranque.

## 4. Ejecutar las pruebas

```sh
cargo test
```

Esto ejecuta todas las pruebas de todos los crates del workspace. El
capítulo [Pruebas e integración continua](step-1/ci.md) explica cómo
están escritas.

## 5. Compilar este libro

El libro está escrito con
[mdBook](https://rust-lang.github.io/mdBook/), una herramienta de Rust.
Sus fuentes están en la carpeta `book/`, junto al código de la
aplicación. Instala mdBook una vez y luego compila:

```sh
cargo install mdbook --version 0.5.4 --locked
cd book
mdbook build
```

Abre `book/book/index.html` (el sitio web que mdBook escribió dentro de
`book/`) en tu navegador. Funciona sin internet. Mientras escribes,
`mdbook serve --open` reconstruye las páginas cada vez que guardas un
archivo. El mismo libro está en línea, y como un único PDF
([El libro en línea y como PDF](step-9/book-online.md)).

## Comandos útiles

| Comando | Qué hace |
|---------|--------------|
| `cargo build` | Compila todo (modo debug) |
| `cargo run` | Compila y arranca la aplicación |
| `cargo test` | Ejecuta todas las pruebas |
| `cargo fmt` | Da formato a todo el código con el estilo estándar de Rust |
| `cargo clippy` | Ejecuta el linter, que sugiere mejores maneras de escribir las cosas |
| `cargo doc --open` | Genera la documentación de la API a partir de los comentarios `///` y la abre |
