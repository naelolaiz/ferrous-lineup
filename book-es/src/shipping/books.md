# Cómo se construyen los libros

Archivos: [`.github/workflows/book.yml`](https://github.com/naelolaiz/ferrous-lineup/blob/main/.github/workflows/book.yml),
[`book-es/book.toml`](https://github.com/naelolaiz/ferrous-lineup/blob/main/book-es/book.toml),
[`book-es/print.css`](https://github.com/naelolaiz/ferrous-lineup/blob/main/book-es/print.css),
[`book-es/print-pdf.py`](https://github.com/naelolaiz/ferrous-lineup/blob/main/book-es/print-pdf.py)
y [`book-es/index-words.txt`](https://github.com/naelolaiz/ferrous-lineup/blob/main/book-es/index-words.txt)

En el repositorio viven cuatro libros, junto al código que describen:

| Carpeta | Libro | En la web |
|---|---|---|
| `book/` | este libro, en inglés | <https://naelolaiz.github.io/ferrous-lineup/> |
| `book-es/` | este libro | <https://naelolaiz.github.io/ferrous-lineup/es/> |
| `manual/` | el manual de usuario, en inglés | <https://naelolaiz.github.io/ferrous-lineup/manual/> |
| `manual-es/` | el manual de usuario | <https://naelolaiz.github.io/ferrous-lineup/es/manual/> |

Cada uno es un sitio web y un PDF con índice de contenido, números de
página, índice alfabético y marcadores. Un cambio en la aplicación y las
páginas que lo describen van en el mismo pull request, o en un pull
request del libro justo después, así los libros nunca describen un
programa que ya no existe.

## mdBook

[mdBook](https://rust-lang.github.io/mdBook/) convierte una carpeta de
archivos Markdown en un sitio web. `book.toml` guarda la configuración,
y `src/SUMMARY.md` el orden de los capítulos, que se convierte en la
barra lateral:

```text
# Índice

[Introducción](introduction.md)

[Compilarlo desde el código fuente](building.md)

# Parte I: La forma del programa

- [Cinco crates, y qué puede saber cada uno](program/crates.md)
```

Un enlace en una línea propia, antes de la primera parte, es un capítulo
sin número, como la introducción. Una línea que empieza con `#` es el
título de una parte, y cada `- [...](...)` un capítulo numerado (uno con
sangría sería un capítulo dentro del de arriba). `mdbook build` escribe el sitio web en la carpeta `book/` del
libro, y `mdbook serve --open` lo construye, lo abre, y lo vuelve a
construir cada vez que se guarda una página: la manera de leer una
página antes de subirla.

`site-url` en `book.toml` es la ruta desde la que GitHub Pages sirve el
libro, `/ferrous-lineup/es/` (o `/ferrous-lineup/` en inglés). La
mayoría de las páginas encuentran sus estilos con enlaces relativos,
pero la página de «no encontrada» se puede mostrar en cualquier
dirección, así que necesita saber dónde empieza el libro.

Una página que se mueve conserva su dirección vieja. `book.toml` lista
las rutas viejas, cada una con adónde fue su página, y mdBook escribe en
la ruta vieja una pequeña página que lleva al navegador a la nueva:

```toml
[output.html.redirect]
"/step-9/band.html" = "../band/band.html"
```

## El flujo de trabajo

El flujo de trabajo **Book** tiene dos trabajos. **build** se ejecuta en
cada push a main y en cada pull request que cambia un libro, un manual o
el propio flujo de trabajo; **deploy** se ejecuta solo después de un
push a main, o cuando se lanza a mano (**Actions > Book > Run
workflow**).

```yaml
pull_request:
  paths:
    - "book/**"
    - "book-es/**"
    - "manual/**"
    - "manual-es/**"
    - ".github/workflows/book.yml"
```

`paths` es la razón por la que un cambio en el código de la aplicación
no construye los libros, y una página del libro no espera los tests de
la aplicación. `pull_request` no nombra ninguna rama, así que también se
construye una página del libro apilada sobre otro pull request.

1. **mdBook**, fijado a una versión, lo descarga ya compilado
   `taiki-e/install-action`, que comprueba su suma de comprobación. La
   misma versión en cada ejecución construye los mismos libros, cambie
   como cambie mdBook después.
2. **Los sitios web:** `mdbook build` en cada una de las cuatro
   carpetas.
3. **Tipografías.** Los libros escriben silencios y notas con símbolos
   musicales (𝄾, ♩) y tienen algunos emoji. Una tipografía sin ellos
   imprime un recuadro vacío, así que el trabajo instala FreeSerif y Noto
   Color Emoji.
4. **pypdf**, una biblioteca de Python para archivos PDF, también
   fijada. El Python de Ubuntu no instala paquetes para todo el sistema,
   así que va en un **entorno virtual**: una carpeta con un Python y
   paquetes propios.
5. **Los PDF.** mdBook también escribe `print.html`, todos los capítulos
   en una página, que es lo que abre el botón de imprimir del sitio web.
   Cada libro se construye una segunda vez, en `pdf/`, con el tema claro,
   porque el sitio web es oscuro pero el papel es blanco. mdBook lee una
   opción de una variable de entorno con su nombre, así que
   `MDBOOK_OUTPUT__HTML__DEFAULT_THEME=light` reemplaza `default-theme`
   bajo `[output.html]` solo para esa construcción (`__` entre los
   niveles). Después `print-pdf.py` imprime esa página en un PDF.
6. **Subidas.** Cada PDF es una descarga de la ejecución, también en los
   pull requests. Los cuatro sitios web, con sus PDF, se juntan en un
   solo sitio: el libro en inglés en la raíz, el español bajo `es/`, y
   cada manual bajo `manual/` en su idioma.
7. **Publicar.** `deploy-pages` le entrega el sitio a GitHub Pages. Ese
   trabajo es el único que pide `pages: write` e `id-token: write`, para
   publicar y para demostrar qué flujo de trabajo publica; todos los
   demás trabajos solo leen el código. Se hace una publicación a la vez
   (`concurrency: group: pages`): una más nueva espera en lugar de
   cancelar otra a medias.

## Orientarse en el PDF

La página de impresión, impresa tal cual, son cientos de páginas sin
números, sin índice de contenido y sin índice alfabético. En el sitio
web la barra lateral y el cuadro de búsqueda encuentran un capítulo; en
papel no lo encontraría nada. `print-pdf.py` agrega lo que necesita un
libro impreso:

| Qué | Hecho a partir de |
|------|-----------|
| Una portada y el **índice de contenido**, cada capítulo con la página donde empieza | `SUMMARY.md` |
| Un **número de página** al pie de cada página | `print.css` |
| Un **índice alfabético** al final: palabras, y las páginas que las explican | los glosarios de [música](../reference/music-glossary.md) y de [Rust](../reference/rust-glossary.md), e `index-words.txt` |
| **Marcadores**, el esquema que muestra un lector de PDF junto a las páginas | las partes, los capítulos y sus secciones |

### Números de página: CSS para el papel

Cuando un navegador imprime, vuelca la página en hojas de papel, y la
regla `@page` dice cómo son esas hojas:

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

Dentro de ella, las *cajas de margen* nombran los espacios alrededor del
texto: `@bottom-center` es el centro del margen inferior, y
`counter(page)` el número de la página que se imprime. La portada no
tiene número: el script agrega una regla solo para la primera página,
`@page :first`.

### El índice de contenido: imprimir dos veces

El índice de contenido tiene que decir en qué página empieza cada
capítulo antes de imprimir el libro, pero solo imprimir decide dónde cae
un capítulo: la longitud de cada página anterior depende de las
tipografías, las imágenes y dónde se corta cada línea. El script hace lo
que hace LaTeX: imprime dos veces.

1. La primera vez, cada número de página del índice de contenido y del
   índice alfabético es `0000`.
2. El script lee en ese PDF dónde cayó cada capítulo y cada palabra del
   índice.
3. La segunda vez, los números de verdad ocupan el lugar de los blancos.
   `0000` es tan ancho como cualquier número de página del libro, así
   que ninguna línea crece, nada se mueve, y los números son correctos.
   El script lee también ese PDF, y si algo se movió, vuelve a imprimir,
   hasta cuatro veces.

Dónde cayó un capítulo se lee a través de sus enlaces. mdBook le da a
cada título un `id` y un enlace a sí mismo, y cuando Chrome imprime una
página con un enlace a `#la-hoja-de-acordes`, escribe un **destino con
nombre** en el PDF: el nombre, la página, y dónde en la página. pypdf
los vuelve a leer:

```python
def destinations(reader: PdfReader) -> dict[str, tuple[int, object]]:
    found = {}
    for name, dest in reader.named_destinations.items():
        id = urllib.parse.unquote(name.lstrip("/"))
        found[id] = (reader.get_destination_page_number(dest), dest)
    return found
```

Los nombres vuelven como `/la-hoja-de-acordes`, con cada letra fuera de
ASCII escrita como códigos `%` (la `ó` de `canción` es `%C3%B3`), así
que `unquote` los convierte de nuevo en los `id` del HTML. Chrome solo
escribe un destino para un `id` al que va algún enlace. Cada título se
enlaza a sí mismo, y el índice de contenido y el alfabético enlazan a lo
que listan; cuando el índice alfabético muestra un solo número para dos
lugares de la misma página, una lista oculta de enlaces (`<div hidden>`)
sigue apuntando a los dos.

### El índice alfabético

Los libros ya tienen listas de palabras: sus glosarios, tablas con la
palabra en negrita, lo que significa, y un enlace «Se explica en». El
script los lee de `print.html` y le da a cada palabra:

- la página adonde va su enlace «Se explica en», en **negrita**;
- cada página donde un capítulo escribe la palabra en negrita, que es
  como estos libros presentan una palabra. Cada una de esas palabras en
  negrita recibe un `id`, como `pdf-ix-17`, para que el índice la
  enlace.

El plural también cuenta, y una palabra que es toda código solo
coincide con código: el `windows` del glosario, un método de Rust, no
es el Windows en negrita de «Linux, macOS y Windows». `index-words.txt`
agrega las palabras que no tiene ningún glosario, sobre todo las partes
de la aplicación, con los capítulos que las explican:

```text
Hoja de acordes: band/chord-sheet.md, ...
Modulación: véase Cambio de tonalidad
Función: no program/sync.md
```

- Un lugar es un capítulo, por el archivo que enlaza `SUMMARY.md`, o una
  sección suya, por el `id` de su título después de `#`.
- `véase` manda al lector a otra entrada.
- `no` deja afuera un capítulo donde la palabra en negrita quiere decir
  otra cosa: ahí «función» es una función de Rust, no lo que hace un
  acorde en una tonalidad.

Una línea que nombra un capítulo o una sección que el libro no tiene
detiene el script con un error, así un título renombrado hace fallar el
flujo de trabajo en lugar de mandar a los lectores a la página
equivocada.

### Marcadores y enlaces

Chrome no escribe un esquema para esta página, así que el script agrega
uno con pypdf después de la última impresión: las partes arriba, cada
capítulo bajo su parte, y las secciones `##` de un capítulo bajo el
capítulo.

```python
writer = PdfWriter(clone_from=reader)
add_bookmarks(writer, chapters, dests)
point_links_at_pages(writer)
writer.page_mode = "/UseOutlines"
```

`/UseOutlines` le pide al lector que abra con los marcadores a la vista.
`point_links_at_pages` arregla los enlaces mismos. Chrome escribe un
enlace como «ir al lugar llamado `la-hoja-de-acordes`», y una tabla al
final del PDF le da un lugar a cada nombre. Los lectores de Chrome y de
Firefox buscan el nombre, pero no todos lo hacen, y en esos un enlace
así no hace nada. Así que el script busca él mismo cada nombre y
escribe el lugar en el enlace, «ir a la página 543, a esta altura», que
todos los lectores siguen:

```python
link[NameObject("/Dest")] = ArrayObject(place)
```

## Un script por idioma

El libro en español tiene su propio `print-pdf.py`, una copia con las
palabras en español: «Contenido» e «Índice alfabético» para los títulos,
`véase` y `no` en `index-words.txt`, los glosarios en español, y dos
reglas que el inglés no necesita. Una palabra terminada en *-ión* tiene
su plural en *-iones* (inversión, inversiones), y las tildes no mueven
una palabra en el índice: «Índice» se ordena con las íes.

Los manuales no tienen script propio. Cada uno se imprime con el script
de su idioma, al que se le da la carpeta del manual como tercer
argumento, donde el script encuentra el `book.toml` y el
`index-words.txt` del manual.

## Imprimir un PDF por tu cuenta

Nada de esto necesita GitHub. Para imprimir el PDF del libro como lo
hace el flujo de trabajo, necesitas mdBook, Python 3.11 o más nuevo, y
Chrome o Chromium (para este libro, en `book-es/` en lugar de `book/`):

```sh
python3 -m venv ~/pdf-tools
~/pdf-tools/bin/pip install pypdf
cd book
MDBOOK_OUTPUT__HTML__DEFAULT_THEME=light \
  MDBOOK_OUTPUT__HTML__PREFERRED_DARK_THEME=light \
  mdbook build --dest-dir pdf
~/pdf-tools/bin/python print-pdf.py pdf book/ferrous-lineup-book.pdf
```

Si Chrome no se llama `google-chrome` en tu sistema, di dónde está:
`CHROME=chromium ~/pdf-tools/bin/python print-pdf.py ...`.

La aplicación es Rust; los libros se construyen con lo que hace cada
tarea de la manera más simple: Markdown para las páginas, YAML para el
flujo de trabajo, CSS para el papel, Python y pypdf para el PDF.
