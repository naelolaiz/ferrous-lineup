# Paquetes de cada versión

Archivos: [`.github/workflows/release.yml`](https://github.com/naelolaiz/ferrous-lineup/blob/main/.github/workflows/release.yml)
y las carpetas de [`packaging/`](https://github.com/naelolaiz/ferrous-lineup/tree/main/packaging)

La mayoría de quienes usen Ferrous Lineup nunca van a instalar Rust. El
flujo de trabajo **Release** compila el programa para cuatro tipos de
computadora y prepara siete descargas:

| Paquete | Sistema |
|---------|--------|
| `ferrous-lineup-…-linux-x86_64.AppImage` | Linux en una PC (Intel o AMD): un solo archivo, nada que desempaquetar |
| `ferrous-lineup-…-linux-x86_64.tar.gz` | Linux en una PC, como una carpeta corriente |
| `ferrous-lineup_…_amd64.deb` | Debian, Ubuntu y sus derivadas |
| `ferrous-lineup-…-windows-x86_64-setup.exe` | Windows en una PC, con desinstalador y la asociación de archivos `.lineup` |
| `ferrous-lineup-…-windows-x86_64.zip` | Windows en una PC, portable: descomprimir y ejecutar |
| `ferrous-lineup-…-macos-arm64.dmg` | Mac con Apple Silicon (M1 y posteriores) |
| `ferrous-lineup-…-macos-x86_64.dmg` | Mac con procesador Intel |

Los `…` son la versión en una publicación (`v0.5.0`), o los siete
primeros caracteres del commit en cualquier otra compilación
(`1a2b3c4`), así una descarga siempre dice qué es. Cómo abrir cada una en
su sistema está en [Instalarlo](https://naelolaiz.github.io/ferrous-lineup/es/manual/installing.html),
en el manual; este capítulo trata de cómo se hacen.

## Dónde están las descargas

- **Cada ejecución** del flujo de trabajo las produce: cada pull
  request, cada push a main, y cada ejecución a mano (**Actions >
  Release > Run workflow** compila cualquier rama). Los paquetes son los
  **artefactos** (Artifacts) de la ejecución, al final de su página. Los
  artefactos necesitan una cuenta de GitHub para descargarlos, y GitHub
  los borra a los 90 días. Así un pull request se puede probar en un
  Mac o en Windows antes de fusionarlo.
- **Una etiqueta** que empieza con `v` también publica los paquetes en
  la página **Releases** del repositorio, con un `SHA256SUMS.txt` al
  lado. Una publicación se queda ahí para siempre, y cualquiera puede
  descargarla. La etiqueta se puede subir con un push, o el flujo de
  trabajo puede crearla él mismo en una ejecución a mano (ver más
  abajo).

## Publicar una versión

Una versión es una etiqueta: un nombre pegado a un commit. Hacer la
versión 0.5.0 lleva un pull request y una etiqueta.

**El pull request de la versión** cambia:

- la versión en el `Cargo.toml` raíz, bajo `[workspace.package]`, que
  cada crate toma con `version.workspace = true`;
- `Cargo.lock`, que lista la versión de cada crate, los nuestros
  también (un `cargo build` lo actualiza);
- las notas de la versión en el archivo AppStream,
  `packaging/linux/io.github.naelolaiz.FerrousLineup.metainfo.xml`, en
  inglés y en español, que muestran los centros de software;
- la versión por defecto en el script del instalador de Windows,
  `packaging/windows/ferrous-lineup.iss`;
- la versión en las recetas de AUR y la etiqueta en el manifiesto de
  Flathub;
- si la publicación debe empezar con unas palabras propias, un archivo
  `packaging/release-notes/v0.5.0.md` que las contenga, en Markdown.

**La etiqueta**, una vez fusionado eso, llega de una de dos maneras.
Desde un clon, con un push:

```sh
git fetch origin
git tag -a v0.5.0 -m "Ferrous Lineup 0.5.0" origin/main
git push origin v0.5.0
```

O, en GitHub, se abre **Actions > Release > Run workflow**, se deja la
rama en `main`, se escribe `v0.5.0` en el campo **tag** y se ejecuta.
Esto solo necesita el permiso de ejecutar flujos de trabajo, no el de
hacer push al repositorio, y el flujo de trabajo crea la etiqueta él
mismo, en el commit que compiló. Si además se marca **draft**, la
publicación se hace como **borrador**: la etiqueta y todas las descargas
están, pero solo la ven quienes pueden editar el repositorio, hasta que
uno de ellos la revisa y pulsa **Publish release**.

De cualquiera de las dos maneras, cuando todos los trabajos de
compilación pasaron, aparece la publicación, llamada «Ferrous Lineup
v0.5.0». Sus notas son el archivo de notas, si lo hay, seguido de la
lista de pull requests fusionados desde la versión anterior que escribe
GitHub; después se pueden editar en GitHub. Un último pull request
pequeño fija lo que solo existe una vez que existe la etiqueta: la suma
de comprobación del archivo de la etiqueta que hace GitHub, en la receta
de AUR, y el commit de la etiqueta en el manifiesto de Flathub.

Tres detalles:

- Una etiqueta con un guion, como `v0.5.0-beta.1`, hace una
  **prepublicación** (pre-release), que nunca se muestra como la última.
  Así escribe el [versionado semántico](https://semver.org) una versión
  que todavía no es definitiva, y Cargo lee las versiones de la misma
  manera.
- Si una etiqueta subida con push y `Cargo.toml` no coinciden, la
  publicación se hace igual, con un aviso en la página de la ejecución:
  la etiqueta ya existe, y detenerse solo la dejaría sin descargas. La
  etiqueta decide los nombres de los archivos, pero **Ayuda > Acerca de
  Ferrous Lineup** dice lo que decía `Cargo.toml`.
- Una ejecución a mano comprueba antes de crear nada: si la etiqueta
  pedida no es `v` más la versión de `Cargo.toml`, o la ejecución está
  en otra rama que main, el trabajo de publicación se detiene con un
  error, y no se crea ninguna etiqueta.

## Targets: para qué computadora es un programa

Un programa compilado es código máquina para un tipo de procesador, en
el formato de archivo de un sistema operativo. Rust nombra cada
combinación con una **target triple**,
arquitectura-fabricante-sistema(-entorno):

| Target | Procesador | Sistema |
|--------|-----------|--------|
| `x86_64-unknown-linux-gnu` | Intel o AMD de 64 bits | Linux, con la biblioteca C de GNU |
| `x86_64-pc-windows-msvc` | Intel o AMD de 64 bits | Windows, con las herramientas de Microsoft |
| `aarch64-apple-darwin` | ARM de 64 bits (Apple Silicon) | macOS («Darwin» es su núcleo) |
| `x86_64-apple-darwin` | Intel de 64 bits | macOS |

`rustc --version --verbose` muestra el target de tu propia computadora
en su línea `host:`. `cargo build --target <triple>` compila para otro,
una vez que `rustup target add <triple>` descargó la biblioteca estándar
para él, y deja el programa en `target/<triple>/release/`, así las
compilaciones para distintos targets nunca se mezclan.

En el flujo de trabajo cada target es una línea de una matriz (ver [Un
trabajo, tres
máquinas](ci.md#un-trabajo-tres-máquinas-la-matriz)):

```yaml
- name: Linux x86_64
  os: ubuntu-22.04
  target: x86_64-unknown-linux-gnu
  platform: linux-x86_64
  appimage: true
```

y la compilación es

```sh
cargo build --release --locked --target ${{ matrix.target }}
```

`--locked` compila exactamente las versiones de las bibliotecas que dice
`Cargo.lock`, y se detiene con un error si el archivo tuviera que
cambiar. Una versión se debe compilar a partir de lo que se probó, no de
las versiones más nuevas que haya el día de la etiqueta. Por eso
también el pull request de la versión tiene que incluir `Cargo.lock`:
con el archivo viejo, `--locked` se niega a compilar.

### El Mac con Intel, compilado en un Mac con Apple Silicon

Las máquinas macOS de GitHub son Apple Silicon. Compilar para otro
procesador que aquel en el que compilas es **compilación cruzada**, y en
un Mac es fácil: los compiladores y las bibliotecas del sistema de Apple
cubren los dos procesadores. Así que el trabajo de Intel corre en el
mismo tipo de máquina que el de Apple Silicon; solo cambia su target.
Los trocitos de C de la compilación, como la criptografía de `ring`
(para descargar el SoundFont por HTTPS), los compila el crate `cc`, que
le pasa el target al compilador de C de Apple. El flujo de trabajo no
puede ejecutar el programa de Intel, y no hace falta: los tests se
ejecutan en el CI, y el código es el mismo.

### Por qué Linux compila en un Ubuntu más viejo

Un programa de Linux llama a la biblioteca C, glibc, para los archivos,
la memoria y los hilos. Cada función de glibc lleva una versión, y un
programa anota la más nueva que usó: compilado en Ubuntu 24.04, Ferrous
Lineup pediría `GLIBC_2.39`. Una glibc más vieja se niega a arrancarlo
(«version `GLIBC_2.39' not found»), mientras que una más nueva lo
ejecuta sin problema. Así que la compilación de Linux corre en el Ubuntu
más viejo que ofrece GitHub, 22.04, con glibc 2.35, y el programa
funciona en Ubuntu 22.04, Debian 12 y cualquier cosa más nueva.

`ldd` lista las bibliotecas que un programa carga al arrancar:

```sh
$ ldd target/release/ferrous-lineup
	libasound.so.2 => /lib/x86_64-linux-gnu/libasound.so.2
	libgcc_s.so.1 => /lib/x86_64-linux-gnu/libgcc_s.so.1
	libm.so.6 => /lib/x86_64-linux-gnu/libm.so.6
	libc.so.6 => /lib/x86_64-linux-gnu/libc.so.6
```

Solo está ALSA. Las bibliotecas de ventanas (Wayland o X11, xkbcommon
para el teclado, OpenGL) no faltan: los crates las abren mientras el
programa se ejecuta, con `dlopen`, y usan las que tenga el escritorio.
Por eso una sola compilación de Linux funciona en escritorios Wayland y
X11.

### El runtime de C dentro del programa de Windows

Los programas de Rust en Windows usan el runtime de C de Microsoft para
algunas cosas básicas. Por defecto es una DLL aparte que viene con el
«Visual C++ Redistributable», que tienen la mayoría de las computadoras
con Windows, pero no todas. La línea de Windows de la matriz agrega una
opción del compilador:

```yaml
rustflags: -C target-feature=+crt-static
```

`crt-static` copia el runtime dentro del `.exe` (que crece un poco), así
el programa arranca en un Windows que nunca instaló nada. La variable de
entorno `RUSTFLAGS` le pasa opciones a cada `rustc` que ejecuta Cargo;
en las otras líneas está vacía.

## Empaquetar

Para Linux y Windows, un paso **Package** copia el programa, el README y
la licencia en una carpeta con el nombre de la etiqueta, y la empaqueta:

```bash
if [[ -n "$RELEASE_TAG" ]]; then
  label="$RELEASE_TAG"
else
  label="${COMMIT::7}"
fi
name="ferrous-lineup-$label-${{ matrix.platform }}"
```

`RELEASE_TAG` se define una vez, para todos los trabajos, al principio
del archivo:

```yaml
env:
  RELEASE_TAG: ${{ github.ref_type == 'tag' && github.ref_name || inputs.tag }}
```

Las expresiones de GitHub no tienen «si … si no», así que `a && b || c`
hace sus veces: cuando la ejecución es por una etiqueta subida con push,
el valor es el nombre de la etiqueta; si no, es el campo **tag** de una
ejecución a mano, que está vacío en cualquier otra ejecución (`inputs`
no tiene nada cuando nadie llenó un formulario). En Bash, `-n` es
verdadero para un texto que no está vacío.

`${COMMIT::7}` es Bash para «los primeros siete caracteres». `COMMIT` es
`${{ github.event.pull_request.head.sha || github.sha }}`: para un pull
request GitHub construye un commit de fusión propio, que nadie ve
nunca, así que la etiqueta toma el último commit de la rama; `||` elige
el primer valor que no esté vacío. Windows recibe un `.zip`, que abre
por sí solo. Linux recibe un `.tar.gz`, porque un archivo tar conserva el
permiso de «ejecutable» del programa.

Para macOS, **Package for macOS** construye un `Ferrous Lineup.app` de
verdad: el programa en `Contents/MacOS`, `Info.plist` y el icono de
`packaging/macos/`, y la versión de `Cargo.toml` escrita en el plist. Lo
firma **ad hoc** (`codesign --force --sign -`), lo pone en una carpeta
con un enlace a `/Applications`, y `hdiutil` hace el `.dmg`, así la gente
arrastra la aplicación a Aplicaciones.

El plist declara además el **tipo de documento** `.lineup`: una entrada
`CFBundleDocumentTypes` que nombra el tipo que abre la aplicación, y
una entrada `UTExportedTypeDeclarations` que define ese tipo,
`io.github.naelolaiz.FerrousLineup.project`, como los archivos que
terminan en `.lineup`. Con las dos, el Finder abre un proyecto con la
aplicación. En Linux y Windows el gestor de archivos arranca el
programa con la ruta del archivo en la línea de comandos, que lee
`main`. macOS lo arranca sin argumentos y manda los archivos en un
evento de Apple, «abrir documentos» (`aevt`/`odoc`), que llega también
a una aplicación que ya está abierta. El manejador propio de AppKit
para ese evento le pide al delegado de la aplicación que abra los
archivos; el delegado es de winit, la biblioteca de ventanas, y no abre
nada. Así que `open_files.rs` pone su propio manejador, en el momento
que dan las reglas de Apple, cuando la aplicación está por terminar de
arrancar (después de que AppKit pone el suyo, antes de que llegue el
evento del arranque). El manejador solo junta las rutas:

```rust
#[unsafe(method(openDocuments:withReplyEvent:))]
fn open_documents(
    &self,
    event: &NSAppleEventDescriptor,
    _reply: &NSAppleEventDescriptor,
) {
    // SAFETY: `paramDescriptorForKeyword:` takes a four-letter
    // code and gives a descriptor or nil.
    let files: Option<Retained<NSAppleEventDescriptor>> =
        unsafe { msg_send![event, paramDescriptorForKeyword: DIRECT_OBJECT] };
    let Some(files) = files else {
        return;
    };
    // A list of files, counted from 1, or a single one.
    let count = files.numberOfItems();
    let items: Vec<Retained<NSAppleEventDescriptor>> = if count == 0 {
        vec![files]
    } else {
        (1..=count)
            .filter_map(|index| files.descriptorAtIndex(index))
            .collect()
    };
    super::arrived(
        items
            .iter()
            .filter_map(|item| item.fileURLValue())
            .filter_map(|url| url.to_file_path()),
    );
}
```

`objc2` llama a Objective-C desde Rust: `define_class!` crea una clase
cuyos métodos AppKit puede llamar, y `msg_send!` manda un mensaje por
su nombre, que el compilador no puede comprobar, de ahí el `unsafe`.
`arrived` guarda las rutas en un `Mutex<Vec<PathBuf>>` y le pide a la
ventana que se vuelva a dibujar; `handle_dropped_files` las toma en el
cuadro siguiente y abre el archivo como si lo hubieran soltado sobre la
ventana. Todo está bajo `#[cfg(target_os = "macos")]`, así que en Linux
y Windows `listen` no hace nada.

Cada paquete se sube con `archive: false`:

```yaml
- uses: actions/upload-artifact@v7
  with:
    name: ${{ env.NAME }}
    path: ${{ env.PACKAGE }}
    archive: false
```

Por defecto un artefacto se comprime en zip al subirlo, así que un
`.tar.gz` se descargaría como un zip que contiene un `.tar.gz`. Con
`archive: false` el archivo se guarda tal cual, y el artefacto lleva el
nombre del archivo.

### La AppImage

Una **AppImage** es un programa y lo que necesita, empaquetados en un
solo archivo que arranca como un programa. El archivo son dos cosas
pegadas: un pequeño programa de arranque, el **runtime**, seguido de una
imagen de disco comprimida de una carpeta, el **AppDir**. Al
arrancarla, el runtime monta la imagen como una carpeta (con FUSE, que
permite que un programa normal ofrezca un sistema de archivos) y arranca
el programa que hay dentro. No se instala nada, así que borrar el
archivo lo desinstala. El AppDir está organizado como el `/usr` de un
sistema:

```text
AppDir/
├── AppRun -> usr/bin/ferrous-lineup          lo que arranca el runtime
├── io.github.naelolaiz.FerrousLineup.desktop -> usr/share/…  la entrada del menú
├── io.github.naelolaiz.FerrousLineup.svg -> usr/share/…      el icono
└── usr/
    ├── bin/ferrous-lineup
    ├── lib/libxkbcommon.so.0, libxkbcommon-x11.so.0, libxcb-xkb.so.1
    └── share/applications/, icons/, mime/, metainfo/
```

La **entrada de escritorio**
([`packaging/linux/io.github.naelolaiz.FerrousLineup.desktop`](https://github.com/naelolaiz/ferrous-lineup/blob/main/packaging/linux/io.github.naelolaiz.FerrousLineup.desktop))
es el archivo de texto que lee cada menú de Linux: el nombre de la
aplicación, su icono, su categoría y cómo arrancarla
(`Exec=ferrous-lineup %f`, donde `%f` es un archivo que abrir). El
archivo de tipo MIME le enseña al escritorio qué es un archivo
`.lineup`, y el archivo **metainfo** de AppStream describe la
aplicación y sus versiones para los centros de software. El flujo de
trabajo revisa los tres con `desktop-file-validate` y `xmllint`, porque
un error de tipeo en ellos falla en silencio en un escritorio.

`appimagetool` convierte el AppDir en la AppImage. Él y el runtime se
descargan fijados a una versión y se comprueban contra el SHA-256
escrito en el flujo de trabajo:

```bash
echo "$RUNTIME_SHA256  runtime" | sha256sum --check
```

Si alguien reemplazara el archivo en GitHub, la suma de comprobación no
coincidiría y la compilación se detendría, en lugar de publicar lo que
fuera que el archivo se hubiera vuelto.

#### Llevar una biblioteca consigo

En un sistema sin `libxkbcommon-x11`, el programa se detiene antes de
abrir una ventana. `ldd` no lista esa biblioteca, porque winit la abre
con `dlopen` cuando se crea la ventana. Una biblioteca abierta así se
busca como cualquier otra: en las carpetas de `LD_LIBRARY_PATH`, luego
en la **RUNPATH** escrita en el programa, luego en las carpetas del
sistema. Así que el flujo de trabajo copia las tres bibliotecas del
teclado en `AppDir/usr/lib` y escribe una RUNPATH con `patchelf`:

```bash
patchelf --set-rpath '$ORIGIN/../lib' AppDir/usr/bin/ferrous-lineup
patchelf --set-rpath '$ORIGIN' AppDir/usr/lib/libxkbcommon-x11.so.0
```

`$ORIGIN` quiere decir «la carpeta en la que está este archivo»,
calculada cuando el programa arranca, así la ruta vale dondequiera que
se monte la AppImage. Las comillas simples impiden que la shell
reemplace `$ORIGIN` por una variable vacía. Lo que la AppImage **no**
lleva: glibc, ALSA y los controladores gráficos. Esos tienen que
corresponder a la computadora, y la comunidad de AppImage mantiene una
lista de las bibliotecas que una AppImage nunca debe llevar.

### El paquete de Debian

El `.deb` lo construye `cargo-deb`, fijado a una versión, a partir del
mismo programa y la misma entrada de escritorio, icono, tipo MIME y
archivo AppStream:

```sh
cargo deb -p lineup-app --no-build --target x86_64-unknown-linux-gnu
```

`--no-build` toma el programa que la matriz acaba de compilar. La tabla
`[package.metadata.deb]` de `crates/lineup-app/Cargo.toml` lista los
archivos y aquello de lo que depende el paquete (ALSA, Wayland,
xkbcommon, el portal del escritorio y las herramientas que registran
entradas de menú, iconos y tipos MIME), así `apt` también los instala.

### El instalador de Windows

El programa de instalación lo hace **Inno Setup**, que las máquinas
Windows de GitHub ya tienen, a partir del script
[`packaging/windows/ferrous-lineup.iss`](https://github.com/naelolaiz/ferrous-lineup/blob/main/packaging/windows/ferrous-lineup.iss).
El flujo de trabajo le pasa la versión y la etiqueta:

```bash
"$iscc" /DAppVersion="$version" /DBuildLabel="$LABEL" packaging/windows/ferrous-lineup.iss
```

El script instala el programa para quien lo ejecuta, sin necesitar
permisos de administrador, y agrega una entrada en el menú Inicio, un
acceso directo opcional en el escritorio, un desinstalador y la
asociación de archivos `.lineup`.

## Publicar: un trabajo que espera a los demás

El segundo trabajo, `release`, se ejecuta solo para una etiqueta subida
con push o una ejecución a mano con etiqueta, y solo cuando todos los
trabajos de compilación pasaron:

```yaml
release:
  name: Publish the release
  if: github.ref_type == 'tag' || (github.event_name == 'workflow_dispatch' && inputs.tag != '')
  needs: build
```

`workflow_dispatch` es el nombre que GitHub le da a una ejecución a
mano. El formulario que muestra sale del disparador al principio del
archivo:

```yaml
workflow_dispatch:
  inputs:
    tag:
      description: "Publish a release with this tag (v0.5.0), made on the commit built. Empty: only build."
      required: false
      default: ""
    draft:
      description: "With a tag: make the tag, but leave the release as a draft, to check and publish by hand."
      type: boolean
      required: false
      default: false
```

Una entrada `boolean` se muestra como una casilla, y llega a los pasos
como el texto `true` o `false`; en una ejecución que no es a mano,
`inputs.draft` está vacío.

`needs: build` espera a cada línea de la matriz. Si una compilación
falla, no hay publicación en absoluto, en lugar de una publicación sin
Windows.

Los `permissions` del trabajo se refieren al **token** que recibe cada
ejecución, `GITHUB_TOKEN`, una contraseña que caduca cuando termina el
trabajo. Al principio del archivo, `permissions: contents: read` deja
que cada trabajo lea el código y nada más, así un paso de compilación
no puede cambiar el repositorio. Solo este trabajo pide
`contents: write`, que necesita una publicación.

Descarga los paquetes, escribe sus sumas de comprobación con
`sha256sum`, y se lo pasa todo a `gh`, la herramienta de línea de
comandos de GitHub:

```bash
if gh release view "$TAG" > /dev/null 2>&1; then
  gh release upload "$TAG" release/* --clobber
else
```

- `gh release view` tiene éxito cuando la publicación ya existe, hecha
  en el sitio web o por una ejecución anterior. Entonces los archivos se
  suben a ella, y `--clobber` reemplaza los que tengan el mismo nombre.
- Si no, `gh release create` la crea, con `--generate-notes` para las
  notas. Para una etiqueta subida con push agrega `--verify-tag`, que se
  detiene si la etiqueta no está en GitHub en lugar de crear una
  etiqueta nueva en el commit más reciente. Para una ejecución a mano
  agrega `--target "$GITHUB_SHA"`, y GitHub crea la etiqueta justo en el
  commit con que se compilaron los paquetes, aunque main haya avanzado
  desde entonces.
- Para una ejecución a mano con **draft** marcado agrega `--draft`. Un
  borrador no tiene etiqueta propia: GitHub crea la etiqueta cuando se
  publica el borrador. Publicado desde el sitio web, la etiqueta se
  crearía con la cuenta de esa persona, y el push de una etiqueta `v`
  nueva pondría otra vez en marcha este flujo de trabajo, que compilaría
  todos los paquetes de nuevo y reemplazaría los archivos ya revisados.
  Por eso el trabajo crea primero la etiqueta, con el propio token de la
  ejecución, y solo después el borrador, con `--verify-tag`:

  ```bash
  if ! gh api "repos/$GITHUB_REPOSITORY/git/ref/tags/$TAG" > /dev/null 2>&1; then
    gh api "repos/$GITHUB_REPOSITORY/git/refs" \
      -f ref="refs/tags/$TAG" -f sha="$GITHUB_SHA" > /dev/null
  fi
  flags+=(--draft --verify-tag)
  ```

  `gh api` llama a la API REST de GitHub: la primera llamada busca la
  etiqueta, la segunda la crea (`-f` envía un campo de la petición).
  Publicar el borrador encuentra entonces la etiqueta ya hecha y no pone
  nada en marcha.
- `--notes-file packaging/release-notes/<etiqueta>.md` se agrega cuando
  ese archivo existe; `gh` pone su texto antes de las notas generadas.
- `--prerelease` se agrega cuando la etiqueta tiene un guion.

Una etiqueta creada con el propio `GITHUB_TOKEN` de la ejecución no pone
en marcha ningún otro flujo de trabajo: lo que una ejecución hace con su
propio token no inicia ejecuciones nuevas (solo pasa una «ejecución a
mano» pedida explícitamente por la API de GitHub), para que un flujo de
trabajo no pueda dispararse a sí mismo sin fin. Por eso la ejecución que
crea la etiqueta es la que la publica; ninguna otra lo hará.

Una **suma de comprobación** (checksum) es una huella de 64 caracteres
de los bytes de un archivo: cambia un byte y la huella es completamente
distinta. `SHA256SUMS.txt` le permite a cualquiera comprobar que una
descarga llegó entera.

## Probar un flujo de trabajo antes de fusionarlo

Un flujo de trabajo de publicación solo hace su trabajo de verdad con
una etiqueta, que es un mal momento para encontrar un error de tipeo.
Dos cosas atrapan los errores antes:

- Cada pull request lo ejecuta, así un cambio que rompe la compilación
  de Windows o del Mac aparece en su pull request. Solo se salta el
  trabajo `release`, porque un pull request no es una etiqueta.
- [actionlint](https://github.com/rhysd/actionlint) revisa los archivos
  de flujos de trabajo y, con [ShellCheck](https://www.shellcheck.net),
  los scripts de shell que contienen:

  ```sh
  pip install actionlint-py shellcheck-py
  actionlint .github/workflows/release.yml
  ```

`concurrency` deja una sola ejecución por rama: cuando un pull request
recibe un push nuevo, se cancela la ejecución anterior, porque la nueva
la contiene. Las publicaciones nunca se cancelan, ni por una etiqueta
subida con push ni por una ejecución a mano con etiqueta:

```yaml
concurrency:
  group: release-${{ github.ref }}-${{ inputs.tag }}
  cancel-in-progress: ${{ github.ref_type != 'tag' && !inputs.tag }}
```

La etiqueta forma parte del nombre del grupo, así un push a main
mientras se compila una publicación desde main pone en marcha su propia
ejecución en lugar de esperar detrás de ella. `!inputs.tag` es verdadero
cuando el campo está vacío.

## Otras puertas de entrada: AUR y Flathub

En `packaging/` hay dos recetas más, para gestores de paquetes que
compilan el programa ellos mismos a partir del código:

- `packaging/aur/` tiene dos **PKGBUILD** para el repositorio de
  usuarios de Arch Linux: `ferrous-lineup`, compilado a partir de la
  etiqueta de una versión, y `ferrous-lineup-git`, compilado a partir de
  main.
- `packaging/flathub/` tiene un manifiesto de **Flatpak** para Flathub,
  con `cargo-sources.json`, cada crate de `Cargo.lock` con su URL y su
  suma de comprobación: Flathub compila sin red, así que cada descarga
  tiene que estar listada de antemano. `generate-sources.py` vuelve a
  escribir ese archivo después de que cambie `Cargo.lock`.

Las dos instalan la misma entrada de escritorio, icono, tipo MIME y
archivo AppStream que la AppImage y el `.deb`. Sus README explican cómo
publicarlas.

## Lo que los paquetes todavía no hacen

- Los programas no están **firmados** del todo. La aplicación de macOS
  tiene una firma ad hoc, que Apple Silicon exige, pero no está
  **notarizada**: eso necesita un Developer ID de Apple de pago, así que
  Gatekeeper pregunta la primera vez. Los programas de Windows no están
  firmados, así que SmartScreen avisa.
  [Instalarlo](https://naelolaiz.github.io/ferrous-lineup/es/manual/installing.html),
  en el manual, muestra cómo pasar cada aviso.
