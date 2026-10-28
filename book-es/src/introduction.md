# Introducción

**Ferrous Lineup** es un programa de código abierto, escrito en Rust,
para tocar canciones con una **banda de acompañamiento**. Escribe los
acordes de una canción compás a compás, elige un estilo, y una banda
(bajo, batería y piano, con guitarra, cuerdas u órgano cuando el estilo
los tiene) la toca en cualquier tonalidad y tempo. Cambia el estilo, la
tonalidad o el tempo, y la banda vuelve a tocar la canción de la nueva
manera. Cada parte que toca la banda queda en una pista propia, donde
puedes abrirla en el piano roll, modificarla a mano, o dejarla fuera y
tocarla tú mismo.

También tiene un segundo objetivo: ayudarte a **aprender armonía**
mientras tocas. Pone nombre a cada acorde que oyes, con su numeral en la
tonalidad y lo que hace en ella (hogar, lejos o tensión), y conoce
también las otras maneras que tiene la música de nombrar la armonía:
conjuntos de clases de alturas, series dodecafónicas y micromodos de
Kröpfl.

Los pasos 1 a 8 construyen aquello sobre lo que se apoya la banda: el
sonido, los patrones y las pistas, un piano roll con ayuda de armonía,
MIDI y una biblioteca de estilos, ritmos y canciones. El paso 9 pone a
la banda al frente: un chord sheet donde escribir la canción, y una
banda que la toca.

Hasta el paso 9 la aplicación se llamaba **Ferrous Loom**
([Un nombre nuevo](step-9/new-name.md) cuenta cómo se renombró).

Este libro tiene un tercer objetivo: es un **curso de Rust construido
alrededor de un proyecto real**. Cada paso del proyecto añade un
capítulo que explica:

- *qué* se construyó y *por qué* se diseñó así, y
- *qué conceptos de Rust* usa el código, apuntando a los archivos
  reales.

No necesitas saber Rust antes de leerlo. Si conoces otro lenguaje de
programación, podrás seguir el hilo. Cada paso termina con unos cuantos
ejercicios que puedes probar en tu propia copia del código.

## Cómo leer este libro

Puedes leerlo de cuatro maneras:

1. Como archivos Markdown sin más, en la carpeta `src` del libro.
2. Como un sitio web en tu computadora: ejecuta `mdbook build` en la
   carpeta del libro y abre `book/index.html` en un navegador.
3. En línea, en <https://naelolaiz.github.io/ferrous-lineup/es/>.
4. Como un único PDF con todos los capítulos en orden, para leer sin
   conexión o imprimir:
   [ferrous-lineup-book-es.pdf](https://naelolaiz.github.io/ferrous-lineup/es/ferrous-lineup-book-es.pdf).
   Tiene números de página, el índice al principio, un índice de
   palabras al final y marcadores para cada capítulo en el panel lateral
   de tu lector de PDF.

[El libro en línea y como PDF](step-9/book-online.md) cuenta cómo se
hacen los dos últimos.

Los enlaces al código fuente de los capítulos apuntan a la rama `main`
actual, que es el programa terminado. El libro es una historia: cada
paso indica cuándo un paso posterior cambió el código que muestra, así
que un enlace es un lugar para seguir leyendo, no una promesa de que el
archivo sigue teniendo exactamente el mismo aspecto que el fragmento.

## Por dónde empezar

- **Para tocar y aprender música**: lee [Primeros pasos](getting-started.md),
  luego [Un recorrido por la aplicación](tour.md) y luego el
  [Paso 9](step-9/README.md). Los pasos anteriores explican cómo se
  construyó; no los necesitas para usar la aplicación.
- **Para aprender Rust**: lee los pasos 1 a 9 en orden. Mantén abierto
  el [Glosario de Rust](glossary.md) y prueba los ejercicios del final de
  cada paso.
- **Para contribuir o buscar algo**: la [Hoja de ruta](roadmap.md) dice
  hacia dónde va el proyecto; el
  [formato de archivo de proyecto](appendix-project-format.md), los
  [atajos de teclado](appendix-shortcuts.md), la
  [resolución de problemas](troubleshooting.md) y el
  [glosario musical](music-glossary.md) son las páginas de referencia.

## El plan

| Paso | Qué se construye | Temas de Rust |
|------|-----------------|-------------|
| 1 | Ventana, motor de audio, tono de prueba, CI | Cargo, crates, módulos, structs, enums, traits, closures, hilos, errores |
| 2 | Channel rack y step sequencer | Arrays, combinadores de `Option`, `Arc`, atómicos, closures como `FnMut`, widgets personalizados, temporización a la muestra |
| 3 | Piano roll con ayudas de armonía | Enums y coincidencia de patrones para la teoría musical, conjuntos de bits, iteradores, tus propias traits y genéricos, máquinas de estados |
| 4 | Playlist, guardar proyectos, compilaciones para macOS y Windows | Mantener reglas entre listas, serde y macros derive, enums de errores, newtypes, trait objects, `FnOnce`, `cfg` y features, una matriz de CI |
| 5 | Deshacer y rehacer, exportación WAV | Genéricos sin límites, `VecDeque`, `mem::swap`, punteros a `fn`, probar contra un modelo, `HashMap` y `Borrow`, hilos, `Send` y `'static`, closures `move`, `JoinHandle` |
| 6 | Herramientas de armonía: selecciones, transponer, arpegios, inversiones, sugerencias, notas fantasma, velocidades | Funciones puras, recolectar `Option`s, `checked_add_signed`, `find_map`, `cycle`, `step_by` y `zip`, el conocimiento como tablas `const`, `Option::take` contra el borrow checker, `flat_map` y lifetimes en iteradores devueltos |
| 7 | MIDI de entrada y salida: teclados, sintetizadores externos, grabación, archivos `.mid` | Patrones de slice, cadenas de bytes y `to_be_bytes`, un `f64` en un `AtomicU64`, `Option::as_mut`, `Mutex` y el poisoning, canales `mpsc`, `thread::Builder`, closures que capturan campos, `Box` para una variante grande de enum, casts `as` de float |
| 8 | Harmony keys, compases, una biblioteca de estilos, ritmos y canciones, demos | El conocimiento como datos, `macro_rules!`, promoción de constantes, tablas `static`, sintaxis de actualización de structs, `Display`, `is_multiple_of` e `ilog2`, atributos de campo de serde, variantes de enum con datos, referencias `&'static` en acciones, comparar snapshots con `PartialEq` |
| 9 | La banda: nombres de acordes, un chord sheet, la forma de la canción, swing, sonidos de instrumentos, y luego el repertorio, el mixer, el vídeo y la fiabilidad | `FromStr` y `str::parse`, `rsplit_once`, closures como parámetros `impl Fn`, destinos `[[bin]]`, cadenas de bytes como etiquetas de formato de archivo, codificación H.264/MP3, dibujo de PDF, estado de recuperación, colas de comandos y de MIDI |

El capítulo de la [Hoja de ruta](roadmap.md) mantiene esta tabla al día.

## Nombre y licencia

"Ferrous" significa "hecho de hierro", y el rust es óxido de hierro; un
lineup es la formación de una banda, la banda que toca tus canciones. El
primer nombre, Ferrous Loom, venía del playlist: un telar entreteje
hilos igual que el playlist entreteje patrones hasta formar una canción.

Ferrous Lineup es software libre, publicado bajo la Licencia Pública
General de GNU, versión 3 o posterior.
