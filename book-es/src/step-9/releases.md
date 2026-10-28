# Compilaciones de lanzamiento: una descarga para cada sistema

Archivo: [`.github/workflows/release.yml`](https://github.com/naelolaiz/ferrous-lineup/blob/main/.github/workflows/release.yml)

Hasta ahora, la única manera de probar Ferrous Lineup era instalar Rust
y compilarlo. El workflow de CI adjuntaba a cada ejecución el programa
que compilaba ([Demostrarlo: la matriz de
CI](../step-4/platforms.md#demostrarlo-la-matriz-de-ci)), pero solo como un
archivo pelado, sin build de Mac Intel, y sin nada que pudiera
encontrar una persona sin cuenta de GitHub. Un segundo workflow,
**Release**, ejecuta ahora cuatro trabajos de build (Linux, Windows, y
macOS para Apple Silicon e Intel) y empaqueta siete descargas:

| Paquete | Sistema |
|---------|---------|
| `ferrous-lineup-…-linux-x86_64.AppImage` | Linux en un PC (Intel o AMD): un archivo, nada que descomprimir |
| `ferrous-lineup-…-linux-x86_64.tar.gz` | Linux en un PC, como una carpeta pelada |
| `ferrous-lineup_…_amd64.deb` | Debian, Ubuntu y sus derivados |
| `ferrous-lineup-…-windows-x86_64-setup.exe` | Windows en un PC, con desinstalador y asociaciones de archivos |
| `ferrous-lineup-…-windows-x86_64.zip` | Windows en un PC, portable: descomprime y ejecuta |
| `ferrous-lineup-…-macos-arm64.dmg` | Macs con Apple Silicon (M1 y posteriores) |
| `ferrous-lineup-…-macos-x86_64.dmg` | Macs con procesador Intel |

El `.tar.gz` y el `.zip` contienen una carpeta con el programa, el
README y la licencia; un `.dmg` contiene `Ferrous Lineup.app`. El
AppImage es el programa en un solo archivo, y el `.deb` y el Setup
`.exe` lo instalan. El `…` es la versión en un release (`v0.2.0`), o los
siete primeros caracteres del commit en cualquier otro build
(`1a2b3c4`), para que siempre sepas qué descargaste.

## Dónde están las descargas

- **Cada pipeline** ejecuta el workflow: cada pull request, cada push a
  main, y cada ejecución manual (**Run workflow** en la página del
  workflow compila cualquier rama bajo demanda). Abre la pestaña
  **Actions** del repositorio, elige **Release**, y después la
  ejecución, o haz clic en **Details** junto a las comprobaciones
  «Build» de un pull request. Al pie de la página de la ejecución, bajo
  **Artifacts**, están los siete paquetes, cada uno una descarga propia.
  Los artifacts necesitan iniciar sesión en GitHub para descargarse, y
  GitHub los borra a los 90 días.
- **Un tag** que empieza por `v` publica también los paquetes en la
  página **Releases** del repositorio, con un `SHA256SUMS.txt` junto a
  ellos. Un release se queda allí para siempre, y cualquiera puede
  descargarlo, con sesión iniciada o sin ella.

## Sacar un release

Un release es un tag: un nombre pegado a un commit. Para hacer la
versión 0.2.0:

1. **Cambia la versión** en el `Cargo.toml` de la raíz, bajo
   `[workspace.package]`:

   ```toml
   [workspace.package]
   version = "0.2.0"
   ```

   Cada crate toma su versión de allí (`version.workspace = true`).
2. **Compila una vez** (`cargo build`). `Cargo.lock` lista la versión de
   cada crate, también las nuestras, así que cambia con ellas. Haz
   commit de ambos archivos y fusiona el cambio en main.
3. **Pon un tag al commit de main y sube el tag:**

   ```sh
   git checkout main
   git pull
   git tag v0.2.0
   git push origin v0.2.0
   ```

El push pone en marcha el workflow. Cuando los cuatro trabajos de build
han pasado, el release aparece en la página Releases, llamado «Ferrous
Lineup v0.2.0», con notas que listan los pull requests fusionados desde
el release anterior.

También puedes hacer el release en la web de GitHub: **Releases > Draft
a new release**, escribe un tag nuevo como `v0.2.0`, escribe tus propias
notas y publica. Publicar crea el tag, que pone en marcha el mismo
workflow, y el workflow añade los paquetes al release que escribiste.

Dos detalles:

- Un tag con un guion, como `v0.2.0-beta.1`, crea un **pre-release**,
  mostrado como tal y nunca como el release más reciente. Así escribe
  [semantic versioning](https://semver.org) una versión que aún no es
  definitiva, y Cargo lee las versiones de la misma manera.
- Si el tag y `Cargo.toml` no coinciden (un tag `v0.2.0` sobre un
  commit cuya versión sigue siendo 0.1.0), el release se hace
  igualmente, con un aviso en la página de la ejecución. El tag decide
  los nombres de los archivos, así que los paquetes están bien, pero se
  olvidó el paso 1.

Si un tag apunta al commit equivocado, borra el release en GitHub y el
tag (`git push origin :refs/tags/v0.2.0`, después `git tag -d v0.2.0`),
y vuelve a poner el tag. Volver a ejecutar el workflow para el mismo tag
reemplaza los archivos del release.

## Abrir lo que descargaste

Los programas de Linux y Windows no están **firmados**, y la app de
macOS está **firmada ad-hoc** pero no **notarizada**. Una firma ad-hoc
es gratis y es lo que exige Apple Silicon; la notarización es la
comprobación extra que hace Apple por un Developer ID de pago, así que
Gatekeeper sigue preguntando la primera vez. SmartScreen de Windows
pregunta también por el programa Setup sin firmar. Ambos sistemas te
dejan ejecutarlo.

- **Linux, el AppImage**: un archivo descargado nunca es ejecutable,
  así que permítelo una vez y arráncalo:

  ```sh
  chmod +x ferrous-lineup-v0.2.0-linux-x86_64.AppImage
  ./ferrous-lineup-v0.2.0-linux-x86_64.AppImage
  ```

  (En un gestor de archivos: **Properties > Permissions > Allow
  executing file as program**, y haz doble clic.) Un AppImage se monta
  a sí mismo con FUSE, que los sistemas de escritorio tienen; donde
  falta FUSE, como en algunos contenedores, añade
  `--appimage-extract-and-run`.
- **Linux, el `.deb`**: instálalo con `apt`, que instala también las
  bibliotecas que necesita:

  ```sh
  sudo apt install ./ferrous-lineup_0.2.0-1_amd64.deb
  ```

  Pone `ferrous-lineup` en `/usr/bin`, añade la entrada de menú y el
  icono, e instala el tipo de proyecto `.lineup` para que los gestores
  de archivos puedan abrir un proyecto con él. Quítalo con
  `sudo apt remove ferrous-lineup`; tus archivos de proyecto se quedan.
- **Linux, el `.tar.gz`**: descomprímelo y arráncalo:

  ```sh
  tar -xzf ferrous-lineup-v0.2.0-linux-x86_64.tar.gz
  ./ferrous-lineup-v0.2.0-linux-x86_64/ferrous-lineup
  ```

  Las tres descargas de Linux necesitan la biblioteca de ALSA
  (`libasound2`, en todo Linux de escritorio) y cargan las bibliotecas
  de ventana y de gráficos que ya tiene el escritorio. La carpeta
  pelada necesita además la biblioteca de teclado `libxkbcommon-x11-0`,
  que el AppImage trae consigo; el `.deb` la declara y apt la instala.
- **Windows, el Setup `.exe`**: ejecútalo. Instala el programa, una
  entrada del menú Inicio, un desinstalador bajo **Settings > Apps**, y
  la asociación de archivos `.lineup` (y la antigua `.loom`), de modo
  que hacer doble clic en un proyecto lo abre en Ferrous Lineup. Si una
  ventana azul dice «Windows protected your PC», elige **More info**, y
  después **Run anyway**.
- **Windows, el `.zip`**: descomprímelo y haz doble clic en
  `ferrous-lineup.exe`. Es el mismo programa sin instalador ni
  asociación de archivos.
- **macOS**: abre el `.dmg`, y arrastra **Ferrous Lineup** a la carpeta
  **Applications** de la misma ventana. Arráncalo desde Applications.
  Si macOS dice que no puede comprobar el programa en busca de software
  malicioso, intenta abrirlo una vez, y permítelo después en **System
  Settings > Privacy & Security** (en versiones anteriores, haz clic
  derecho en la app y elige **Open**). También puedes quitar la marca
  que macOS pone a los archivos descargados:

  ```sh
  xattr -dr com.apple.quarantine "/Applications/Ferrous Lineup.app"
  ```

Para comprobar que una descarga llegó entera, compárala con el checksum
que lista el release. En Linux, en la carpeta con las descargas y
`SHA256SUMS.txt`:

```sh
sha256sum --check --ignore-missing SHA256SUMS.txt
```

En un Mac, `shasum -a 256 <file>` imprime el checksum de un archivo, y
en Windows PowerShell lo hace `Get-FileHash <file>`; compáralo con la
línea de ese archivo en `SHA256SUMS.txt`. Un **checksum** es una huella
de 64 caracteres de los bytes del archivo: cambia un byte y la huella
es completamente distinta.

## Cómo funciona

### Targets: para qué ordenador es un programa

Un programa compilado es código máquina para una clase de procesador,
en el formato de archivo de un sistema operativo. Rust nombra cada
combinación con un **target triple**, escrito
architecture-vendor-system(-environment):

| Target | Procesador | Sistema |
|--------|------------|---------|
| `x86_64-unknown-linux-gnu` | Intel/AMD de 64 bits | Linux, con la biblioteca Do de GNU |
| `x86_64-pc-windows-msvc` | Intel/AMD de 64 bits | Windows, con las herramientas de Microsoft |
| `aarch64-apple-darwin` | ARM de 64 bits (Apple Silicon) | macOS («Darwin» es su núcleo) |
| `x86_64-apple-darwin` | Intel de 64 bits | macOS |

`rustc --version --verbose` muestra el target de tu propio ordenador en
la línea `host:`. `cargo build` compila para él; `cargo build --target
<triple>` compila para otro, una vez que `rustup target add <triple>` ha
traído la biblioteca estándar compilada para ese target. Con
`--target`, el programa acaba en `target/<triple>/release/` en lugar de
`target/release/`, de modo que los builds de targets distintos nunca se
mezclan.

En el workflow, cada target es una línea de una **matriz**, la misma
idea que en CI: GitHub arranca un trabajo por línea, en la máquina que
nombra la línea, y `${{ matrix.target }}` es el target de esa línea:

```yaml
matrix:
  include:
    - name: Linux x86_64
      os: ubuntu-22.04
      target: x86_64-unknown-linux-gnu
      platform: linux-x86_64
    # … Windows, macOS Apple Silicon, macOS Intel
```

El paso de toolchain instala el target (`targets: ${{ matrix.target }}`),
y el build es

```sh
cargo build --release --locked --target ${{ matrix.target }}
```

`--locked` hace que Cargo compile exactamente las versiones de
bibliotecas escritas en `Cargo.lock`, y se detenga con un error si el
archivo tendría que cambiar. Un release debería compilarse desde lo que
se probó, no desde las versiones que sean más nuevas el día del tag.
Por eso también un cambio de versión debe commitear `Cargo.lock`: con
el archivo viejo, `--locked` se niega a compilar.

### El build de Mac Intel, hecho en un Mac con Apple Silicon

Las máquinas macOS de GitHub son Apple Silicon. Compilar para un
procesador distinto del que usas para compilar se llama
**cross-compiling**, y en un Mac es fácil: los compiladores de Apple y
sus bibliotecas de sistema cubren ambos procesadores, porque los Macs
pasaron de Intel a Apple Silicon hace solo unos años. Así que el
trabajo de Intel se ejecuta en la misma clase de máquina que el de
Apple Silicon; solo difiere su target. Los trozos de código en Do del
build, como la criptografía de `ring` (para descargar el SoundFont por
HTTPS), los compila el crate `cc`, que pasa el target al compilador de
Do de Apple.

El workflow no puede ejecutar el programa de Intel (no hay ningún Mac
Intel donde ejecutarlo), lo cual está bien: las pruebas se ejecutan en
CI, y el código es el mismo.

### Por qué Linux se compila en un Ubuntu viejo

Un programa de Linux llama a la biblioteca Do, glibc, para archivos,
memoria e hilos. Cada función de glibc lleva una versión, y un programa
registra la versión más nueva que usó: compilado en Ubuntu 24.04,
Ferrous Lineup pide `GLIBC_2.39`:

```sh
$ objdump -T target/release/ferrous-lineup | grep -o 'GLIBC_[0-9.]*' | sort -Vu | tail -1
GLIBC_2.39
```

Un glibc más viejo se niega a arrancarlo («version `GLIBC_2.39' not
found»), mientras que uno más nuevo lo ejecuta sin problemas. Así que
el build de Linux corre en el Ubuntu más viejo que todavía ofrece
GitHub, 22.04, con glibc 2.35, y el paquete funciona en Ubuntu 22.04,
Debian 12 y cualquier cosa más nueva. Cuando GitHub retire 22.04, la
línea se moverá a 24.04.

`ldd` lista las bibliotecas que carga un programa al arrancar:

```sh
$ ldd target/release/ferrous-lineup
	libasound.so.2 => /lib/x86_64-linux-gnu/libasound.so.2
	libgcc_s.so.1 => /lib/x86_64-linux-gnu/libgcc_s.so.1
	libm.so.6 => /lib/x86_64-linux-gnu/libm.so.6
	libc.so.6 => /lib/x86_64-linux-gnu/libc.so.6
```

Solo está ALSA. Las bibliotecas de ventana (Wayland o X11, xkbcommon
para el teclado, OpenGL) no faltan: los crates las abren mientras el
programa corre (con `dlopen`), y eligen la que tenga el escritorio. Por
eso un solo build de Linux funciona tanto en escritorios Wayland como
X11.

### El runtime de Do dentro del programa de Windows

Los programas de Rust en Windows usan el runtime de Do de Microsoft para
unas pocas cosas básicas. Por defecto es una DLL aparte,
`vcruntime140.dll`, que viene con el «Visual C++ Redistributable» que
tienen la mayoría, pero no todos, de los ordenadores con Windows. La
línea de Windows de la matriz añade una flag de compilador:

```yaml
rustflags: -C target-feature=+crt-static
```

`crt-static` copia el runtime dentro del propio `.exe` (crece un poco),
de modo que el programa arranca en un Windows que nunca instaló nada.
La variable de entorno `RUSTFLAGS` pasa flags a cada `rustc` que
ejecuta Cargo; en las demás líneas, `matrix.rustflags` está vacío. No
se abre ninguna ventana de consola detrás del programa, gracias a la
línea `windows_subsystem` de `main.rs` del paso 4 ([Sin ventana de
consola en
Windows](../step-4/platforms.md#sin-ventana-de-consola-en-windows-cfg_attr)).

### Empaquetar

El paso **Package** se ejecuta para Linux y Windows (`if: runner.os !=
'macOS'`; Windows también corre Bash, desde Git for Windows, con
`shell: bash`). Elige la etiqueta, copia el programa, el README y la
licencia a una carpeta, y la empaqueta:

```bash
if [[ "$GITHUB_REF_TYPE" == tag ]]; then
  label="$GITHUB_REF_NAME"         # v0.2.0
else
  label="${COMMIT::7}"             # 1a2b3c4
fi
name="ferrous-lineup-$label-${{ matrix.platform }}"
```

`${COMMIT::7}` es la manera de decir en Bash «los siete primeros
caracteres»: el id corto del commit, como lo muestra `git log
--oneline`. `COMMIT` es
`${{ github.event.pull_request.head.sha || github.sha }}`: para un pull
request, GitHub compila un merge commit propio (la rama fusionada en
main, para probar lo que main llegaría a ser), que tú nunca ves, así
que el nombre toma en su lugar el commit más reciente de la rama. `||`
elige el primer valor que no está vacío.

Windows recibe un `.zip`, que Windows abre por sí mismo. Linux recibe
un `.tar.gz`, porque un archivo tar conserva los permisos de archivo:
el bit de «ejecutable» del programa sobrevive, y arranca sin un
`chmod +x`.

macOS no usa ese paso. **Package for macOS** construye un
`Ferrous Lineup.app` de verdad: el programa en `Contents/MacOS`,
`Info.plist` y el icono en `Contents/Resources`, el README y la
licencia junto a ellos, y la versión de `Cargo.toml` escrita en el
plist. Firma la app ad-hoc (`codesign --force --sign -`), la pone en
una carpeta con un enlace a `/Applications`, y `hdiutil` hace el
`.dmg`. En la imagen montada, arrastrar la app sobre ese enlace la
copia a Applications. `COPYFILE_DISABLE=1` impide que las herramientas
de archivado añadan archivos ocultos `._` con información del Finder.

El AppImage y el `.deb` los compilan pasos propios, más abajo. El
paquete se sube con `archive: false`:

```yaml
- uses: actions/upload-artifact@v7
  with:
    path: ${{ env.PACKAGE }}
    archive: false
```

Por defecto, un artifact se comprime en zip al subirlo, de modo que un
`.tar.gz` se descargaría como un zip que contiene un `.tar.gz`, y el
zip perdería de todas formas el bit ejecutable. Con `archive: false` el
archivo se guarda tal cual, y el artifact recibe el nombre del archivo.

### El AppImage

Un **AppImage** es un programa y todo lo que necesita, empaquetado en
un solo archivo que arranca como un programa. El archivo son dos cosas
pegadas: un programita de arranque, el **runtime**, seguido de una
imagen de disco comprimida (SquashFS) de una carpeta, el **AppDir**.
Cuando arrancas el AppImage, el runtime monta la imagen como una
carpeta (con FUSE, que deja que un programa normal ofrezca un sistema
de archivos) y arranca el programa de dentro. No se instala nada, así
que borrar el archivo lo desinstala.

El AppDir se organiza como el `/usr` de un sistema:

```text
AppDir/
├── AppRun -> usr/bin/ferrous-lineup          lo que arranca el runtime
├── io.github.naelolaiz.FerrousLineup.desktop -> usr/share/…  la entrada de menú
├── io.github.naelolaiz.FerrousLineup.svg -> usr/share/…      el icono
└── usr/
    ├── bin/ferrous-lineup
    ├── lib/libxkbcommon.so.0, libxkbcommon-x11.so.0, libxcb-xkb.so.1
    └── share/applications/…, share/icons/hicolor/scalable/apps/…
```

La **entrada de escritorio**
([`packaging/linux/io.github.naelolaiz.FerrousLineup.desktop`](https://github.com/naelolaiz/ferrous-lineup/blob/main/packaging/linux/io.github.naelolaiz.FerrousLineup.desktop))
es el archivo de texto pequeño que lee todo menú de Linux: el nombre de
la app, su icono, su categoría, y cómo arrancarla
(`Exec=ferrous-lineup %f`, donde `%f` es un archivo que abrir). El
icono
([`packaging/icon/ferrous-lineup.svg`](https://github.com/naelolaiz/ferrous-lineup/blob/main/packaging/icon/ferrous-lineup.svg))
es una Fa y un trompetista, en el naranja de la app. Las herramientas
que añaden AppImages al menú (AppImageLauncher, por ejemplo) usan
ambos, y también lo hace el `.deb`.

`appimagetool` convierte el AppDir en el AppImage. Él y el runtime se
descargan de sus releases de GitHub, fijados a una versión, y se
comprueban contra el SHA-256 escrito en el workflow:

```bash
echo "$RUNTIME_SHA256  runtime" | sha256sum --check
```

Si alguien reemplazara el archivo en GitHub, el checksum diferiría y el
build se detendría, en lugar de enviar lo que fuera en lo que se hubiera
convertido el archivo.

#### Traer una biblioteca consigo

El AppImage lleva tres bibliotecas propias. En un sistema sin
`libxkbcommon-x11`, el programa se detiene antes de abrir una ventana:

```text
Library libxkbcommon-x11.so could not be loaded.
```

`ldd` no la listaba, porque winit no enlaza la biblioteca de teclado
cuando se compila el programa: la abre con **`dlopen`** cuando se crea
la ventana, de modo que un programa funciona con X11 o Wayland, el que
haya. Una biblioteca abierta así se busca como cualquier otra, en este
orden (simplificado): las carpetas de `LD_LIBRARY_PATH`, el **RUNPATH**
escrito en el programa, y después las carpetas del sistema. Así que el
workflow copia las bibliotecas a `AppDir/usr/lib` y escribe un RUNPATH
en el programa con `patchelf`:

```bash
patchelf --set-rpath '$ORIGIN/../lib' AppDir/usr/bin/ferrous-lineup
patchelf --set-rpath '$ORIGIN' AppDir/usr/lib/libxkbcommon-x11.so.0
```

`$ORIGIN` significa «la carpeta en la que está este archivo», calculada
cuando arranca el programa, así que la ruta vale dondequiera que se
monte el AppImage. La segunda línea deja que `libxkbcommon-x11`
encuentre a sus propios compañeros, `libxkbcommon` y `libxcb-xkb`, a su
lado. Las comillas simples evitan que el shell reemplace `$ORIGIN` por
una variable (vacía).

Lo que el AppImage **no** trae: la biblioteca Do (glibc), ALSA, y los
drivers de gráficos (OpenGL, Vulkan). Esos tienen que coincidir con el
ordenador: un driver de gráficos pertenece a su tarjeta gráfica y su
kernel, y ALSA a la configuración de sonido del sistema. La comunidad
de AppImage mantiene una lista de esas bibliotecas, la *excludelist*,
que un AppImage nunca debería llevar.

### El paquete Debian

El `.deb` lo compila `cargo-deb`, fijado en 3.8.0, desde el mismo
binario de release y la misma entrada de escritorio, icono, tipo MIME y
archivo AppStream:

```bash
cargo deb -p lineup-app --no-build --target x86_64-unknown-linux-gnu
```

`--no-build` usa el binario que acaba de compilar la matriz. La tabla
`[package.metadata.deb]` en `crates/lineup-app/Cargo.toml` lista los
archivos y las dependencias que necesita el paquete: ALSA, Wayland,
xkbcommon, el portal de escritorio, `desktop-file-utils`,
`hicolor-icon-theme` y `shared-mime-info`. Por eso
`sudo apt install ./ferrous-lineup_0.2.0-1_amd64.deb` instala también
esas, mientras que `dpkg -i` solo las dejaría faltando.

### Publicar: un trabajo que espera a los demás

El segundo trabajo, `release`, se ejecuta solo para tags, y solo cuando
han pasado los cuatro trabajos de build:

```yaml
release:
  if: github.ref_type == 'tag'
  needs: build
  permissions:
    contents: write
```

`needs: build` espera a cada línea de la matriz. Si un build falla, no
hay release en absoluto, en lugar de un release al que le falta
Windows.

`permissions` trata del **token** que recibe cada ejecución,
`GITHUB_TOKEN`, una contraseña que caduca cuando acaba el trabajo.
Arriba del archivo, `permissions: contents: read` deja que cada trabajo
lea el código y nada más, de modo que un paso de build no puede cambiar
el repositorio. Solo este trabajo pide `contents: write`, que es lo que
necesita un release.

El trabajo descarga los siete paquetes (cada uno a una carpeta con su
propio nombre; `skip-decompress` evita que el `.zip` de Windows se
descomprima), escribe sus checksums con `sha256sum`, y se lo entrega
todo a `gh`, la herramienta de línea de comandos de GitHub, que tienen
todas las máquinas de GitHub:

```bash
if gh release view "$TAG" > /dev/null 2>&1; then
  gh release upload "$TAG" release/* --clobber
else
  gh release create "$TAG" release/* --title "Ferrous Lineup $TAG" \
    --generate-notes --verify-tag "${flags[@]}"
fi
```

- `gh release view` tiene éxito cuando el release ya existe, hecho en
  la web o por una ejecución anterior. Entonces los archivos se suben a
  él, y `--clobber` reemplaza los archivos del mismo nombre.
- `--generate-notes` escribe las notas a partir de los pull requests
  fusionados desde el release anterior.
- `--verify-tag` se detiene si el tag no está en GitHub, en lugar de
  crear un tag nuevo en el commit más nuevo, de modo que un release se
  hace siempre en el commit que taggeaste.
- `flags` es un **array** de Bash: `--prerelease` cuando el tag tiene un
  guion, y nada si no. `"${flags[@]}"` se expande a sus elementos, cada
  uno como un argumento, o a nada en absoluto cuando está vacío.

### Probar un workflow antes de fusionarlo

Un workflow de release solo hace su trabajo de verdad con un tag, que
es mal momento para encontrar una errata. Dos cosas atrapan antes los
errores:

- Cada pull request lo ejecuta, así que el pull request que lo añadió
  compiló todos los paquetes antes de fusionarse, y un cambio que rompe
  el build de Windows o el de Mac aparece en su pull request. Solo se
  salta el trabajo `release`, ya que un pull request no es un tag.
- [actionlint](https://github.com/rhysd/actionlint) comprueba los
  archivos de workflow, y, con [ShellCheck](https://www.shellcheck.net)
  instalado, los scripts de shell que hay dentro:

  ```sh
  pip install actionlint-py shellcheck-py
  actionlint .github/workflows/release.yml
  ```

Por último, `concurrency` mantiene una ejecución por rama: cuando un
pull request recibe un push nuevo, o dos pull requests se fusionan en
main seguidos, la ejecución más vieja se cancela, ya que la más nueva
la contiene. Los tags nunca se cancelan.

## Ejercicios

1. **Linux en ARM.** GitHub tiene máquinas Linux con ARM
   (`ubuntu-24.04-arm`). Añade una línea de la matriz para
   `aarch64-unknown-linux-gnu`. ¿Qué nombre de archivo recibiría, y qué
   significa para el glibc más viejo en el que funciona?
2. **Un programa de Mac para ambos.** `lipo -create a b -output c` une
   un programa de Apple Silicon y uno de Intel en un solo programa
   «universal». ¿Dónde en el workflow podrían reunirse los dos builds
   de Mac para hacerlo, si se ejecutan en máquinas distintas?
3. **Notarizar la app de Mac.** El workflow firma la app ad-hoc con
   `codesign --force --sign -`. Con un Developer ID de Apple de pago,
   podría firmar con `codesign --sign "Developer ID Application: …"`,
   enviar el `.dmg` con `xcrun notarytool submit … --wait`, y grapar el
   ticket de Apple con `xcrun stapler staple`. ¿Por qué una firma
   ad-hoc no se puede notarizar, y cuál de esos pasos necesita la
   cuenta de Apple?
4. **Detenerse con una versión equivocada.** Haz que la comprobación de
   versión falle en lugar de avisar. ¿Dónde debería ejecutarse, para
   que un tag equivocado se detenga antes de 20 minutos de build, y no
   después?
5. **Leer la versión en la app.** `env!("CARGO_PKG_VERSION")` es la
   versión del crate, fijada en tiempo de compilación. Muéstrala en el
   título de la ventana o en una ventana Help > About. ¿Por qué es
   `env!` y no `std::env::var`?
