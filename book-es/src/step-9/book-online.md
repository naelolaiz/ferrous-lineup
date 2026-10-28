# El libro en línea y como PDF

Archivos: [`.github/workflows/book.yml`](https://github.com/naelolaiz/ferrous-lineup/blob/main/.github/workflows/book.yml),
[`book/book.toml`](https://github.com/naelolaiz/ferrous-lineup/blob/main/book/book.toml),
[`book/print.css`](https://github.com/naelolaiz/ferrous-lineup/blob/main/book/print.css),
[`book/print-pdf.py`](https://github.com/naelolaiz/ferrous-lineup/blob/main/book/print-pdf.py)
y [`book/index-words.txt`](https://github.com/naelolaiz/ferrous-lineup/blob/main/book/index-words.txt)

Hasta ahora este libro vivía solo como archivos Markdown y como un sitio
web que construías tú mismo con `mdbook build`. Ahora vive en la carpeta
`book/` del repositorio, junto al código que describe, y un workflow,
**Book**, lo convierte en dos cosas que cualquiera puede abrir:

| Qué | Dónde |
|------|-------|
| El sitio web | <https://naelolaiz.github.io/ferrous-lineup/es/> |
| Un PDF con todos los capítulos | <https://naelolaiz.github.io/ferrous-lineup/es/ferrous-lineup-book-es.pdf>, y como descarga en cada ejecución, en la pestaña **Actions** |

## Una rama propia, y luego una carpeta

Durante su primera semana en línea, el libro tuvo una rama propia,
`book`, que no compartía historia con `main`: su primer commit fue el
primer capítulo del libro, no la primera línea de código de la app. La
Git no le importa. Una rama solo es un nombre para un commit, y nada
dice que todos los commits de un repositorio tengan que compartir una
raíz.

Eso tenía un costo. GitHub muestra un pull request como lo que su rama
cambia desde el último commit que comparte con la rama a la que va. Una
rama del libro y una rama de la app no comparten ningún commit, así que
GitHub no puede compararlas: una página del libro nunca podía ser un
pull request apilado sobre el pull request del cambio que describe, y
apuntar un pull request del libro a una rama de la app hacía que GitHub
lo cerrara. Así que el libro se mudó a `book/` en `main`, con su
historia:

```sh
git merge -s ours --no-commit --allow-unrelated-histories origin/book
git read-tree --prefix=book/ -u origin/book
git commit
```

- `--allow-unrelated-histories` permite que `git merge` una dos
  historias sin ningún commit en común; sin él, Git se niega.
- `-s ours` (la estrategia "ours") registra el merge pero deja los
  archivos de `main` exactamente como están, para que el `src/` del
  libro no quede en la raíz del repositorio.
- Luego `git read-tree --prefix=book/ -u origin/book` coloca cada
  archivo del libro bajo `book/`, en el índice y (`-u`) en la carpeta.

El commit tiene dos padres, el último commit de `main` y el último
commit de `book`, así que todos los commits del libro siguen ahí.
`git blame book/src/tour.md` nombra el commit que escribió cada línea, y
`git log --full-history -- src/tour.md` lista los commits de un capítulo
anteriores a la mudanza, bajo la ruta que tenía entonces.

## Paso a paso

El workflow tiene dos jobs. **build** se ejecuta en cada push a `main` y
en cada pull request que cambia algo en `book/` (o en el propio
workflow); **deploy** se ejecuta solo después de un push, o cuando
inicias el workflow a mano (**Actions > Book > Run workflow**).

```yaml
on:
  push:
    branches: [main]
    paths:
      - "book/**"
      - ".github/workflows/book.yml"
  pull_request:
    paths:
      - "book/**"
      - ".github/workflows/book.yml"
  workflow_dispatch:
```

`paths` es la razón por la que un cambio en el código de la app no
construye el libro, y una página del libro no espera a las pruebas de la
app. `pull_request` no nombra ninguna rama, así que una página del libro
apilada sobre otro pull request también se construye, vaya a donde vaya.
El job de build ejecuta sus comandos en `book/`
(`defaults: run: working-directory: book`), como si el libro fuera todo
el repositorio; las rutas que se dan a las actions (`with: path:`)
siguen empezando en la raíz, así que dicen `book/book`.

1. **mdBook.** `taiki-e/install-action` descarga un mdBook publicado,
   fijado a una versión, para que el libro se construya igual en cada
   ejecución por mucho que mdBook cambie después. `mdbook build` escribe
   el sitio web en `book/book/`, exactamente igual que en tu
   computadora.
2. **El PDF.** mdBook también escribe `print.html`, todos los capítulos
   en una sola página; es lo que abre el botón de impresión de arriba a
   la derecha del sitio web. El workflow construye el libro una segunda
   vez, en `pdf/`, con el tema claro: el sitio web se queda en azul
   oscuro, pero un PDF debe ser negro sobre blanco. mdBook lee cualquier
   ajuste de una variable de entorno en lugar de `book.toml` cuando
   lleva el nombre del ajuste:
   `MDBOOK_OUTPUT__HTML__DEFAULT_THEME` es `default-theme` bajo
   `[output.html]`, con `__` entre los niveles y `_` en lugar de `-`.
   Luego `print-pdf.py` añade a esa página una portada, el índice de
   contenidos y un índice de términos, y hace que Chrome, que las
   máquinas de GitHub ya tienen, la abra sin ventana (`--headless`) y la
   imprima en un archivo (`--print-to-pdf`), como hace **Print** en un
   navegador, sin la fecha y la dirección de arriba y abajo de cada
   página (`--no-pdf-header-footer`). Por último, añade los marcadores
   ([Orientarte en el PDF](#orientarte-en-el-pdf) más abajo).
3. **Fuentes.** El libro escribe los silencios y las notas como símbolos
   musicales (𝄾, ♩) y tiene algunos emoji. Una fuente sin ellos imprime
   un cuadro vacío, así que el job instala primero FreeSerif y Noto
   Color Emoji.
4. **Dos subidas.** El PDF se sube por su cuenta, como descarga de la
   ejecución. Luego `upload-pages-artifact` empaqueta `book/book/`, con
   el PDF copiado dentro, como el sitio web.
5. **Publicación.** `deploy-pages` entrega ese paquete a GitHub Pages. El
   job pide dos permisos que solo él necesita: `pages: write` para
   publicar e `id-token: write` para demostrar a Pages qué workflow está
   publicando. Todos los demás jobs solo leen el código
   (`permissions: contents: read` arriba del todo). Solo se ejecuta un
   despliegue a la vez (`concurrency: group: pages` en este job): uno más
   nuevo espera en lugar de cancelar a otro a mitad de camino. Las builds
   de los pull requests no esperan.

`book/print.css` es el único estilo propio del libro en el sitio web.
Dice qué es una página impresa, La4 con un margen alrededor del texto y
el número de página abajo, para que el PDF y el botón de impresión del
sitio web hagan las mismas páginas:

```css
@page {
    size: A4;
    margin: 18mm 16mm;

    @bottom-center {
        content: counter(page);
        font-family: "Open Sans", sans-serif;
        font-size: 9pt;
        color: #555;
    }
}
```

## Orientarte en el PDF

El primer PDF era la página de impresión del sitio web, impresa: más de
mil páginas sin números de página, sin índice de contenidos y sin índice
de términos. En el sitio web, la barra lateral y el buscador encuentran
un capítulo; en un lector de PDF, o en papel, nada lo hacía.
`book/print-pdf.py`, un script de Python, ahora imprime el PDF y añade
lo que un libro impreso necesita para encontrar cosas en él:

| Qué | De dónde sale |
|------|-----------|
| Una portada y el **índice de contenidos**: todos los capítulos, y el índice de términos, con la página en la que empiezan | `SUMMARY.md`, la lista que muestra la barra lateral |
| Un **número de página** al pie de cada página | `print.css` |
| Un **índice de términos** al final: palabras, y las páginas que las explican | los glosarios de [música](../music-glossary.md) y de [Rust](../glossary.md), y `index-words.txt` |
| **Marcadores**, el esquema que un lector de PDF muestra junto a las páginas | las partes, los capítulos y sus secciones |

### Números de página: CSS para papel

Cuando un navegador imprime una página web, vierte el texto sobre las
páginas, y la regla `@page` dice qué son esas páginas. Dentro de ella,
las *cajas de margen* nombran los espacios alrededor del texto:
`@bottom-center` es el centro del margen inferior, y `counter(page)` es
el número de la página que se está imprimiendo. Eso es todo, en el
`print.css` de arriba. Chrome dibuja las cajas de margen desde la
versión 131. La portada no tiene número: el script añade
`@page :first { @bottom-center { content: none; } }`, una regla solo
para la primera página.

### El índice de contenidos: imprimir dos veces

El índice de contenidos tiene que decir "Step 9 empieza en la página
543" antes de que se imprima la página, pero solo la impresión decide
dónde cae Step 9: la longitud de cada capítulo anterior depende de las
fuentes, las imágenes y dónde se parte cada línea. El script hace lo que
hace LaTeX, el programa de composición tipográfica con el que están
escritos muchos libros: imprime dos veces.

1. La primera vez, todos los números de página del índice de contenidos
   y del índice de términos son `0000`.
2. El script lee en ese PDF dónde cayó cada capítulo, y cada palabra del
   índice de términos.
3. La segunda vez, los números reales ocupan el lugar de los huecos.
   `0000` es tan ancho como cualquier número de página del libro, así
   que ninguna línea se alarga, nada se mueve y los números quedan bien.
   El script también lee el segundo PDF y, si algo se movió, imprime una
   tercera vez.

¿Cómo lees en un PDF dónde cayó un capítulo? La través de sus enlaces.
mdBook da a cada encabezado un `id` y un enlace a sí mismo:

```html
<h1 id="step-9-the-band"><a class="header" href="#step-9-the-band">Step 9: The band</a></h1>
```

Cuando Chrome imprime una página con un enlace a `#step-9-the-band`,
escribe un *destino con nombre* en el PDF: el nombre, la página y en qué
punto de la página. Así es como un clic en un enlace de un PDF sabe
adónde ir, y pypdf, una biblioteca de Python para archivos PDF, los
vuelve a leer:

```python
def destinations(reader: PdfReader) -> dict[str, tuple[int, object]]:
    found = {}
    for name, dest in reader.named_destinations.items():
        id = urllib.parse.unquote(name.lstrip("/"))
        found[id] = (reader.get_destination_page_number(dest), dest)
    return found
```

Los nombres vuelven como `/step-9-the-band`, y con cada letra fuera de
ASCII escrita como código `%` (la `ö` de `kröpfls` es `%C3%B6`), así que
`unquote` los convierte de nuevo en los `id` del HTML.

Chrome solo escribe un destino para un `id` al que apunte algún enlace
de la página. Cada encabezado enlaza a sí mismo, y el índice de
contenidos y el índice de términos enlazan a lo que listan; cuando el
índice de términos muestra un número para dos lugares de la misma
página, una lista oculta de enlaces (`<div hidden>`) sigue apuntando a
ambos, para que la siguiente impresión sepa dónde está cada uno.

### El índice de términos

Un índice de términos lista palabras con las páginas que hablan de
ellas. El libro ya tiene dos listas de palabras, sus glosarios: tablas
con la palabra en negrita, lo que significa y un enlace "Explained in".
El script las lee de `print.html` y da a cada palabra:

- la página a la que va su enlace "Explained in", en **negrita**;
- todas las páginas donde un capítulo escribe la palabra en negrita. Así
  presenta este libro una palabra (`A **closure** is...` en el
  Markdown), así que esas son las páginas donde un capítulo la explica.
  Cada una de esas palabras en negrita recibe un `id`, como
  `pdf-ix-17`, para que el índice de términos enlace a ella.

El plural también cuenta: una "inversions" en negrita es un lugar para
"Inversion". Una palabra que es todo código solo coincide con código: el
`windows` del glosario, un método de Rust, no es el Windows en negrita
de "Linux, macOS y Windows".

`book/index-words.txt` añade las palabras que no están en ningún
glosario, sobre todo las partes de la app, con los capítulos que las
explican:

```text
Chord sheet: step-9/chord-sheet.md, step-9/text-chord-sheets.md, ...
Key marker: step-8/key-changes.md#a-key-marker
Modulation: see Key change
Function: not step-5/diff.md
```

- Un lugar es un capítulo, por el archivo que enlaza `SUMMARY.md`, o una
  sección suya, por el `id` de su encabezado tras `#`.
- `see` envía al lector a otra entrada.
- `not` deja fuera un capítulo donde su palabra en negrita significa
  otra cosa. En [Decir al hilo de audio lo que cambió](../step-5/diff.md),
  la "function" en negrita es una función de Rust, no lo que hace un
  acorde en la tonalidad.

Una línea que nombre un capítulo o una sección que el libro no tiene
detiene el script con un error, así que un encabezado renombrado hace
fallar el workflow Book en lugar de enviar a los lectores a la página
equivocada.

### Marcadores

La mayoría de los lectores de PDF muestran un *esquema* junto a las
páginas: un árbol de títulos para hacer clic. Chrome no escribe uno para
esta página, así que el script lo añade con pypdf después de la última
impresión:

```python
writer = PdfWriter(clone_from=reader)
add_bookmarks(writer, chapters, dests)
writer.page_mode = "/UseOutlines"
```

`add_outline_item` recibe un título, una página, el marcador bajo el que
va y en qué punto de la página aterrizar (el destino del encabezado).
Las partes están arriba del todo, cada capítulo bajo su parte o su
step, y las secciones de un capítulo, sus encabezados `##`, bajo el
capítulo. `/UseOutlines` pide al lector que muestre el panel cuando abra
el PDF.

### Enlaces que sigue cualquier lector

Un enlace de PDF puede decir adónde va de dos maneras. Chrome escribe la
primera: "ve al lugar llamado `step-9-the-band`", y el PDF guarda una
tabla de nombres al final (`/Dests`) que da una página a cada uno. La
otra manera es más antigua y más simple: "ve a la página 543, hasta aquí
abajo". Los lectores de Chrome y Firefox buscan un nombre en la tabla;
no todos los lectores lo hacen, y en esos el índice de contenidos, el
índice de términos y los enlaces entre capítulos del primer PDF no
hacían nada al hacer clic. Así que el último paso del script, después de
los marcadores, busca cada nombre por sí mismo y escribe el lugar en el
enlace:

```python
for page in writer.pages:
    for annotation in page.get("/Annots", []):
        link = annotation.get_object()
        name = link.get("/Dest")
        ...
        place = names.get(name)
        ...
        link[NameObject("/Dest")] = ArrayObject(place)
```

Un enlace es una *anotación* en su página: un rectángulo con lo que
hacer cuando se hace clic en él. `/Dest` es adónde va. El valor de la
tabla ya tiene la forma simple, un array como `[page /XYZ 11 826 0]`
(la página, y luego "mantén el zoom y pon este punto arriba a la
izquierda"), así que el script lo copia en el enlace. Los marcadores se
escribieron así desde el principio.

### En el workflow

```yaml
- name: Install pypdf
  run: |
    python3 -m venv "$RUNNER_TEMP/pdf-tools"
    "$RUNNER_TEMP/pdf-tools/bin/pip" install --quiet pypdf==6.17.0
```

El Python de Ubuntu no instala paquetes para todo el sistema, donde se
mezclarían con los que instala el propio Ubuntu, así que pypdf va a un
*entorno virtual*: una carpeta con un Python y unos paquetes propios. Su
versión está fijada, como la de mdBook. El paso de impresión ejecuta el
script con ese Python:

```yaml
mdbook build --dest-dir pdf
"$RUNNER_TEMP/pdf-tools/bin/python" print-pdf.py pdf book/ferrous-lineup-book.pdf
```

La app es Rust; el libro se construye con lo que hace cada trabajo de la
forma más simple: YAML para el workflow, Python y pypdf para el PDF.

## Antes del primer push

Dos ajustes en GitHub, que se hacen una vez:

- **Settings > Pages > Source**: GitHub Actions. Sin él,
  `configure-pages` detiene la ejecución y dice que Pages no está
  configurado.
- **Settings > Environments > github-pages > Deployment branches and
  tags**: activar Pages crea este environment, y deja publicar a la rama
  por defecto, `main`. Mientras el libro tuvo su propia rama, `book`
  también tuvo que añadirse ahí; ahora `main` basta.

`site-url` en `book.toml` es la ruta desde la que Pages sirve el libro,
`/ferrous-lineup/`. La mayoría de las páginas encuentran sus estilos con
enlaces relativos, pero la página de "no encontrado" puede mostrarse
para cualquier dirección, así que necesita saber dónde empieza el libro.

## Leerlo antes de hacer push

Nada de esto necesita GitHub para probarse:

```sh
cd book
mdbook serve --open
```

construye el sitio web y lo abre, y lo vuelve a construir cada vez que
guardas un capítulo. **Print** en el navegador, en cualquier página, o
en el botón de impresión, muestra las páginas del PDF tal y como se
verán.

Para imprimir el PDF completo como hace el workflow, con su índice de
contenidos, su índice de términos y sus marcadores, necesitas Python
3.11 o más nuevo y Chrome o Chromium:

```sh
python3 -m venv ~/pdf-tools
~/pdf-tools/bin/pip install pypdf
cd book
MDBOOK_OUTPUT__HTML__DEFAULT_THEME=light \
  MDBOOK_OUTPUT__HTML__PREFERRED_DARK_THEME=light \
  mdbook build --dest-dir pdf
~/pdf-tools/bin/python print-pdf.py pdf book/ferrous-lineup-book.pdf
```

Si en tu sistema Chrome no se llama `google-chrome`, di dónde está:
`CHROME=chromium ~/pdf-tools/bin/python print-pdf.py ...`.

## Ejercicios

1. En `book/`, ejecuta `MDBOOK_OUTPUT__HTML__DEFAULT_THEME=ayu mdbook build`
   y abre `book/index.html`. ¿Qué tema muestra? ¿Ha cambiado
   `book.toml`?
2. Cambia `size: A4` por `size: letter` en `print.css` e imprime una
   página desde el navegador. ¿Qué cambia y qué se queda igual?
3. Abre un pull request que solo cambie un archivo de `crates/`. ¿Qué
   workflows se ejecutan, y qué línea de `book.yml` impide que se
   ejecute el workflow Book?
4. Ejecuta `git log --oneline --graph -5` en el commit que movió el
   libro (su mensaje empieza por "Bring the book's history"). ¿Cuántos
   padres tiene, y de dónde sale cada línea del grafo?
5. Añade `Metronome: step-9/practice.md#where-the-clicks-come-from` a
   `index-words.txt` e imprime el PDF. ¿Dónde cae "Metronome" en el
   índice de términos, y su número de página te lleva a la sección?
   Luego escribe mal la sección e imprime otra vez: ¿qué dice el script?
6. Cambia `BLANK` en `print-pdf.py` de `"0000"` a `"0"`. ¿Cuántas veces
   imprime ahora el script el libro, y por qué un hueco más estrecho
   puede mover las páginas?
