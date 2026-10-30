# Introducción

**Ferrous Lineup** es un programa de código abierto, escrito en Rust,
para tocar canciones con una **banda de acompañamiento**. Escribe los
acordes de una canción compás por compás, elige un estilo, y una banda
(bajo, batería y piano, con guitarra, cuerdas, vientos u órgano cuando
el estilo los tiene) la toca en cualquier tonalidad y tempo. Cada parte
que toca la banda cae en una pista propia, donde puedes abrirla en el
piano roll, cambiarla a mano, o dejarla afuera y tocarla tú.

También es una manera de **aprender armonía** mientras tocas. Nombra
cada acorde que oyes, con su número romano en la tonalidad y lo que hace
ahí, y conoce también las otras maneras en que la música nombra la
armonía: conjuntos de clases de altura, series dodecafónicas y los
micromodos de Kröpfl.

Este libro trata del interior: **cómo está organizado el código, cómo
está diseñado y por qué**, y cómo está escrita en Rust la teoría musical
que conoce la aplicación. Explica el Rust que usa cada parte a medida
que avanza, señalando los archivos reales, así que también es un curso
de Rust construido alrededor de un programa de verdad. No hace falta
saber Rust antes de leerlo; si conoces otro lenguaje de programación,
puedes seguirlo. Para *usar* la aplicación, lee en cambio el [manual de
usuario](https://naelolaiz.github.io/ferrous-lineup/es/manual/): no
tiene nada de código.

## Cómo está organizado el libro

Los capítulos siguen el código, de la forma del programa a cómo se
distribuye. Cada uno dice qué hace una parte de la aplicación, con un
enlace a la página del manual sobre cómo usarla; cómo está diseñada y
por qué; el Rust que usa; los archivos donde vive; y los tests que la
protegen. Cada parte termina con ejercicios.

| Parte | De qué trata | El Rust del camino |
|------|-------|--------------------|
| I. La forma del programa | los cinco crates, los dos hilos, los datos del proyecto, deshacer, guardar, las ventanas, dos idiomas | workspaces y crates, módulos, enums con datos, reglas guardadas por un solo portero, `Arc` y un ring buffer sin bloqueos, `History<T>`, serde, enums de errores, `Drop`, dibujo en modo inmediato, opciones por hilo |
| II. El sonido | el secuenciador, el tempo y el compás, los instrumentos, los SoundFonts, el mezclador, MIDI, exportar, la traza de reproducción | temporización a la muestra, atómicos, closures `FnMut`, un `Mutex` y colas `mpsc`, cadenas de bytes y patrones de slices, el único bloque `unsafe`, tipos `Copy` con reglas |
| III. La teoría musical en código | la altura, escalas y tonalidades, acordes, números romanos y funciones, herramientas de edición, oír la armonía, conjuntos, series, micromodos, conducción de voces, importar una grabación | tablas en un `match`, conjuntos de bits, iteradores (`find_map`, `cycle`, `chunk_by`), recoger `Option`s, `FromStr` y `Display`, `try_from`, programación dinámica |
| IV. La banda | la biblioteca, la hoja de acordes, cómo toca la banda, la orquesta, la forma, las paradas, el solista, tus propios estilos, las herramientas de práctica, la partitura guía | tablas `static` y `const fn`, `macro_rules!`, hashes y generadores con semilla, leer texto, dibujar un PDF |
| V. Las canciones como datos | cómo se escribe una demostración, y cómo una interpretación MIDI o una partitura se convierte en una | datos `const` en módulos hijos, sintaxis de actualización de structs, `LazyLock`, ordenar con `Ordering` |
| VI. Compilar, probar y distribuir | los tests, los tres sistemas y su CI, los paquetes de cada versión, cómo se construyen estos libros | `#[cfg(test)]`, tests de integración, `catch_unwind`, `cfg` y features, scripts de compilación |

Al final están el [formato del archivo de
proyecto](reference/project-format.md), un [glosario de
Rust](reference/rust-glossary.md), un [glosario de
música](reference/music-glossary.md) e [ideas para lo que
viene](reference/ideas.md).

## Por dónde empezar

- **Para aprender Rust:** lee las partes en orden, con el [glosario de
  Rust](reference/rust-glossary.md) abierto, y prueba los
  ejercicios al final de cada parte. [Compilarlo desde el código
  fuente](building.md) pone primero el código en marcha.
- **Para aprender cómo se programa la teoría musical:** empieza por la
  Parte III, y ten cerca el [glosario de
  música](reference/music-glossary.md). La teoría en sí, sin
  código, se enseña en el manual.
- **Para cambiar algo:** busca la parte de la aplicación en el índice,
  lee su capítulo, y ejecuta sus tests antes y después de tu cambio.

## Cómo leerlo

1. En línea, en <https://naelolaiz.github.io/ferrous-lineup/es/>.
2. Como un solo PDF, para leer sin conexión o imprimir:
   [ferrous-lineup-book-es.pdf](https://naelolaiz.github.io/ferrous-lineup/es/ferrous-lineup-book-es.pdf).
   Tiene números de página, el índice de contenido al principio, un
   índice alfabético al final, y marcadores para cada capítulo.
3. Como archivos Markdown, en la carpeta `book-es/src` del repositorio,
   o como sitio web en tu computadora con `mdbook serve --open` en
   `book-es/`.

El libro también está en inglés, en
<https://naelolaiz.github.io/ferrous-lineup/>. [Cómo se construyen los
libros](shipping/books.md) cuenta cómo se hacen los sitios web y
los PDF.

Los enlaces al código apuntan a la rama `main`, y los fragmentos de los
capítulos están copiados de ella. Donde un fragmento deja líneas afuera,
lo dice con `// ...`.

## Nombre y licencia

«Ferrous» quiere decir «de hierro» en inglés, y el óxido (*rust*) es
óxido de hierro; un *lineup* es la formación de una banda, la banda que
toca tus canciones.

Ferrous Lineup es software libre, publicado bajo la Licencia Pública
General de GNU, versión 3 o posterior.
