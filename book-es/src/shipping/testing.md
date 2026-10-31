# Cómo se prueba el código: tests unitarios, tests de ventana, trazas

Archivos: el `mod tests` al final de la mayoría de los archivos de
`crates/*/src`,
[`crates/lineup-core/tests/`](https://github.com/naelolaiz/ferrous-lineup/tree/main/crates/lineup-core/tests),
[`crates/lineup-listen/tests/`](https://github.com/naelolaiz/ferrous-lineup/tree/main/crates/lineup-listen/tests)
y [`crates/lineup-app/src/app/ux_tests.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/ux_tests.rs)

Ferrous Lineup tiene más de 1300 tests: unos 900 en `lineup-core`, 150
en `lineup-audio`, 260 en `lineup-app` y unas pocas decenas en los otros
dos crates. Cada capítulo de este libro termina con los tests de lo que
describe. Este trata de los tipos de test, por qué existe cada uno, y
cómo ejecutarlos.

## Ejecutarlos

```sh
cargo test                       # every crate
cargo test -p lineup-core        # one crate
cargo test frequency             # only tests whose name contains "frequency"
cargo test -- --nocapture        # show what the tests print
```

Un test es una función corriente marcada con `#[test]`. Pasa si
termina, y falla si entra en pánico, que es lo que hacen `assert!`,
`assert_eq!` y `unwrap` sobre un error. `cargo test` compila los crates
en un modo de prueba, ejecuta cada test en hilos en paralelo, y lista
los que fallaron con sus mensajes.

Los tests se ejecutan normalmente en una compilación de **depuración**
(debug), y la integración continua los ejecuta así también. Eso
importa: en una compilación de depuración un `+` o un `*` que desborda
entra en pánico, y se comprueba cada `debug_assert!`, mientras que una
compilación release da la vuelta al número y se salta los
`debug_assert!`. Un error de ese tipo pasa en release y falla en
depuración, así que los tests deben ejecutarse en depuración. Los pocos
crates que harían una compilación de depuración demasiado lenta para
tocar música, como el sintetizador de SoundFonts, se compilan
optimizados incluso entonces (ver [Compilaciones de
depuración](../sound/soundfonts.md#compilaciones-de-depuración)). Los
informes largos, más abajo, son la excepción: se ejecutan con
`--release`.

## Tests unitarios, junto al código

La mayoría de los tests están al final del archivo que prueban:

```rust
#[cfg(test)]
mod tests {
    use super::*;
```

`#[cfg(test)]` deja el módulo afuera de toda compilación salvo
`cargo test`, así que los tests no le cuestan nada al programa.
`use super::*` trae todo lo del archivo, también lo privado, porque un
módulo hijo ve los elementos privados de su padre. Así un test unitario
puede comprobar una función auxiliar que el resto del programa nunca
ve.

El sonido se puede probar sin escucharlo. Una onda senoidal cruza el
cero dos veces en cada ciclo, así que contar cuántas veces cambia de
signo la señal mide su altura:

```rust
fn sign_changes(samples: &[f32]) -> usize {
    samples
        .windows(2)
        .filter(|pair| (pair[0] < 0.0) != (pair[1] < 0.0))
        .count()
}
```

`windows(2)` da cada par de muestras vecinas, `filter` se queda con los
pares a cada lado del cero, y `count` los cuenta. Un segundo de A4 (el
la de 440 Hz) tiene que cambiar de signo unas 880 veces, dos veces 440:

```rust
// A4 is 440 Hz: about 880 sign changes in one second.
assert!((860..=900).contains(&changes), "got {changes}");
```

El margen deja lugar al ataque al comienzo de la nota. Los iteradores
son perezosos: nada se ejecuta hasta que `count` pide valores, y el
compilador convierte la cadena en un bucle tan rápido como uno escrito a
mano.

## Tests sobre datos

Buena parte de Ferrous Lineup son datos: 343 canciones, 217 estilos
armónicos, 141 ritmos, los estilos dodecafónicos y de micromodos. Los
tests los leen todos, así que un error en una tabla falla antes de que
nadie lo oiga: un compás que no suma, un acorde que no se puede leer, un
nombre repetido, un estilo cuyo ritmo no existe (ver [Cómo se
prueba](../band/library.md#cómo-se-prueba) para la biblioteca y
[Cómo se prueba](../songs/demos.md#cómo-se-prueba) para las
canciones). Sus mensajes nombran la entrada, así que un fallo dice qué
canción abrir.

## Tests en `tests/`, desde afuera

Un archivo en la carpeta `tests/` de un crate es un **test de
integración**: un programa propio que usa el crate como lo haría
cualquier otro crate, así que solo ve lo que el crate hace público.

| Archivo | Qué comprueba |
|------|----------------|
| `lineup-core/tests/chord_reading.rs` | qué tan bien nombra la hoja de acordes lo que tocan las demostraciones más difíciles: el acorde de los Augurios, el acorde de Petrushka, una pila de cuartas |
| `lineup-core/tests/key_finding.rs` | cuántos compases de las demostraciones oye el detector de tonalidad en su tonalidad escrita, y que encuentre las modulaciones claras en su compás |
| `lineup-core/tests/damaged_files.rs` | que un proyecto, un archivo MIDI o una hoja de acordes dañados terminen en un error o en un proyecto, nunca en un cierre inesperado |
| `lineup-listen/tests/key_changes.rs` | qué tan bien se encuentran los cambios de tonalidad en demostraciones convertidas en audio |

### Archivos dañados

`damaged_files.rs` lee cientos de copias dañadas de proyectos, archivos
MIDI y hojas de acordes buenos, como hace un **fuzzer**, y comprueba que
cada una termine en un proyecto o en un error, nunca en un cierre
inesperado. Cómo los daña, y el desbordamiento que encontró y que solo
ve una compilación de depuración, están en [Los archivos dañados nunca
hacen caer la
aplicación](../program/saving.md#los-archivos-dañados-nunca-hacen-caer-la-aplicación).

### Informes

Algunos tests miden en lugar de comprobar. `report`, en
`key_finding.rs` y en `chord_reading.rs`, imprime una tabla, canción por
canción, de qué tan bien se oye cada demostración. Están marcados con
`#[ignore]`, así que `cargo test` se los salta, y se ejecutan aparte:

```sh
cargo test --release -p lineup-core --test key_finding report -- --ignored --nocapture
```

Los tests corrientes de al lado impiden que la cifra baje:
`most_bars_of_the_demos_are_heard_in_their_key` falla si menos del 87%
de los compases se oyen en su tonalidad. El informe dice por qué un
cambio movió esa cifra; el test se asegura de que nunca se mueva para el
lado equivocado sin que nadie lo note (ver [Qué tan bien
escucha](../theory/hearing.md#qué-tan-bien-escucha)).

## Tests de ventana

egui dibuja la ventana entera de nuevo en cada fotograma, a partir del
estado de la aplicación (ver [Modo
inmediato](../program/egui.md#modo-inmediato)). Eso hace que la
ventana misma sea fácil de probar: un test puede ejecutar fotogramas sin
pantalla, darles clics y teclas, y leer lo que se dibujó. `ux_tests.rs`
tiene un `Screen` para eso:

```rust
fn new() -> Self {
    let ctx = egui::Context::default();
    theme::apply(&ctx);
    ctx.all_styles_mut(|style| style.animation_time = 0.0);
    let mut app = LineupApp::with_audio(&ctx, None, Audio::Unavailable("test".into()));
```

La aplicación arranca como lo hace sin tarjeta de sonido
(`Audio::Unavailable`), con las animaciones apagadas para que un
fotograma muestre el final de cada animación, y nunca lee ni escribe una
copia de recuperación de verdad ni descarga un SoundFont. `draw` ejecuta
un fotograma con los eventos dados; egui devuelve las formas que
pintaría, y `text_rect` busca una etiqueta entre ellas:

```rust
fn text_rect(output: &FullOutput, label: &str) -> Option<Rect> {
    output
        .shapes
        .iter()
        .rev()
        .find_map(|shape| match &shape.shape {
            Shape::Text(text) if text.galley.text() == label => {
```

`click` dibuja un fotograma, busca la etiqueta, y envía una pulsación y
una liberación del puntero en su centro. Así un test se lee como una
persona usando la aplicación:

```rust
fn f1_and_the_help_menu_show_the_keyboard_shortcuts() {
    let mut screen = Screen::new();
    let output = screen.draw(vec![]);
    assert!(text_rect(&output, "Keyboard shortcuts").is_none());

    screen.key(Key::F1);
    let output = screen.draw(vec![]);
    assert!(screen.app.shortcuts_open);
```

126 tests de ese archivo abren demostraciones, aplican estilos, hacen
clic en el mezclador y el rack, cambian el tamaño de las ventanas y
reinician la aplicación. Otros más pequeños están junto
a un solo widget, como el selector de compás o la lista de
reproducción, con una función auxiliar `frame` que dibuja solo ese
widget en una ventana propia.

## Trazas: lo que se tocó

Un test del secuenciador puede contar notas, pero el sonido de una
canción es lo que el reproductor envía a los instrumentos, paso a paso,
al tempo de cada momento. La [traza de
reproducción](../sound/trace.md) escribe exactamente eso, una línea
por nota, tocando sin ventana como lo hace **Archivo > Exportar WAV…**.
Es la herramienta para una pregunta que un test unitario no puede hacer
con facilidad, como «¿esta demostración sigue sonando igual que antes?»:
se traza antes y después de un cambio y se comparan las dos tablas. El
MIDI se prueba igual, sin sintetizador: una canción se exporta a bytes,
se vuelve a leer y se compara (ver [Probar sin
sintetizador](../sound/midi.md#probar-sin-sintetizador)).

## Formato y lints

Dos herramientas más mantienen el código en orden, y la integración
continua ejecuta las dos:

- `cargo fmt` reescribe el código en el estilo estándar de Rust, así
  nadie discute dónde van las llaves. `cargo fmt --all --check` solo
  dice si algo cambiaría.
- `cargo clippy` es el **linter** de Rust. Conoce cientos de patrones
  que compilan pero podrían ser más simples, más rápidos o más
  correctos, y explica cada uno. `-D warnings` convierte cada aviso en
  un error.

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
```

`--all-targets` revisa también los tests y los scripts de compilación.
La integración continua instala el Rust estable más nuevo en cada
ejecución, y un Rust nuevo a menudo trae lints nuevos, así que ejecuta
`rustup update` antes de revisar: un lint que tu Clippy más viejo no
conoce hará fallar la ejecución igual.
