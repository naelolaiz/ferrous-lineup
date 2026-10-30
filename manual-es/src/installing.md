# Instalarlo

Hay programas listos para Linux, macOS y Windows en la [página de
versiones](https://github.com/naelolaiz/ferrous-lineup/releases) del
proyecto. Abre la versión más reciente y descarga el archivo para tu
computadora. No hace falta instalar nada más: los sonidos, las fuentes
y el codificador de video vienen con la aplicación.

| Descarga | Para |
|---|---|
| `ferrous-lineup-…-linux-x86_64.AppImage` | Linux en una PC: un solo archivo, nada que desempaquetar |
| `ferrous-lineup_…_amd64.deb` | Debian, Ubuntu, Linux Mint y sus parientes |
| `ferrous-lineup-…-linux-x86_64.tar.gz` | Linux en una PC, como una carpeta corriente |
| `ferrous-lineup-…-windows-x86_64-setup.exe` | Windows: lo instala, con desinstalador |
| `ferrous-lineup-…-windows-x86_64.zip` | Windows, portátil: descomprimir y ejecutar |
| `ferrous-lineup-…-macos-arm64.dmg` | Mac con Apple Silicon (M1 y posteriores) |
| `ferrous-lineup-…-macos-x86_64.dmg` | Mac con procesador Intel |

Los `…` son la versión, por ejemplo `v0.4.1`. **Ayuda > Acerca de
Ferrous Lineup** muestra en la aplicación la versión que estás usando.

## Linux

**El AppImage.** Un archivo descargado no puede ejecutarse hasta que lo
permitas. En un gestor de archivos, abre las **Propiedades** del
archivo y, en **Permisos**, marca **Permitir ejecutar el archivo como
un programa**; después haz doble clic en él. En una terminal:

```sh
chmod +x ferrous-lineup-v0.4.1-linux-x86_64.AppImage
./ferrous-lineup-v0.4.1-linux-x86_64.AppImage
```

Si no arranca y menciona FUSE, instala la biblioteca FUSE 2 (`libfuse2`,
o `libfuse2t64` en los Ubuntu más nuevos), o arráncalo con
`--appimage-extract-and-run` después de su nombre.

**El `.deb`.** Instálalo con:

```sh
sudo apt install ./ferrous-lineup_0.4.0-1_amd64.deb
```

Añade Ferrous Lineup al menú de aplicaciones, y los gestores de archivos
abren con él los proyectos `.lineup`. `sudo apt remove ferrous-lineup`
lo quita; tus canciones se quedan donde las guardaste.

**El `.tar.gz`.** Desempaquétalo y arranca `ferrous-lineup` dentro de la
carpeta:

```sh
tar -xzf ferrous-lineup-v0.4.1-linux-x86_64.tar.gz
./ferrous-lineup-v0.4.1-linux-x86_64/ferrous-lineup
```

Todo Linux de escritorio tiene las bibliotecas de sonido y de ventanas
que necesita la aplicación. La carpeta corriente necesita además la
biblioteca de teclado `libxkbcommon-x11-0`, que los otros dos formatos
traen consigo o instalan.

## Windows

**El programa de instalación** instala Ferrous Lineup con una entrada en
el menú Inicio y un desinstalador (en **Configuración > Aplicaciones**),
y un doble clic en un proyecto `.lineup` lo abre. El programa no está
firmado, así que la primera vez Windows puede mostrar una ventana azul,
«Windows protegió su PC»: elige **Más información** y después
**Ejecutar de todas formas**.

**El `.zip`** es el mismo programa sin instalador: descomprímelo en
cualquier carpeta y haz doble clic en `ferrous-lineup.exe`.

## macOS

Abre el `.dmg` y arrastra **Ferrous Lineup** a la carpeta
**Aplicaciones** que aparece a su lado; después arráncalo desde
Aplicaciones.

Apple no ha notarizado la aplicación, así que la primera vez macOS
puede decir que no puede comprobar si contiene software malicioso.
Intenta abrirla una vez, luego abre **Ajustes del Sistema > Privacidad
y seguridad** y permítela allí. En versiones más antiguas de macOS, haz
clic derecho en la aplicación y elige **Abrir**.

Con la aplicación en Aplicaciones, un doble clic en un archivo
`.lineup` en el Finder lo abre en Ferrous Lineup.

## El primer arranque

Ferrous Lineup se abre con una canción corta y su hoja de acordes, lista
para sonar: pulsa **Espacio**. La primera vez, apenas se abre, ofrece
descargar un juego de sonidos de instrumentos reales, de unos 30 MB;
[Sonidos](sounds.md) explica qué es. Después sigue con [La ventana
principal](main-window.md) y [Tu primera canción](first-song.md).

La primera vez, la aplicación arranca en inglés. Para pasarla al
español, elige **Edit > Settings…**, y en **Appearance**, bajo
**Language**, marca **Español**: cambia en el acto, y desde entonces
arranca en español. Para volver al inglés: **Edición > Ajustes… >
Apariencia > Idioma**.

## Comprobar una descarga

Cada versión incluye un archivo `SHA256SUMS.txt`: una huella de cada
descarga. Si quieres estar seguro de que una descarga llegó entera,
ponla en una carpeta junto con ese archivo y ejecuta, en Linux:

```sh
sha256sum --check --ignore-missing SHA256SUMS.txt
```

En un Mac, `shasum -a 256 <archivo>` imprime la huella de un archivo, y
en Windows PowerShell lo hace `Get-FileHash <archivo>`; debe coincidir
con la línea de ese archivo.

## Compilarlo tú

También puedes compilar Ferrous Lineup desde su código fuente, en
cualquiera de los tres sistemas. El libro [Ferrous Lineup por
dentro](https://naelolaiz.github.io/ferrous-lineup/es/building.html)
explica cómo.
