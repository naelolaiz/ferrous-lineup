# Cinco crates, y qué puede saber cada uno

Archivos: [`Cargo.toml`](https://github.com/naelolaiz/ferrous-lineup/blob/main/Cargo.toml),
el `Cargo.toml` de cada crate en
[`crates/`](https://github.com/naelolaiz/ferrous-lineup/tree/main/crates),
y [`crates/lineup-core/src/lib.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/lib.rs)

Ferrous Lineup es un solo programa, `ferrous-lineup`, construido a
partir de cinco crates de Rust. Este capítulo trata de cómo están
divididos y por qué: qué es Cargo, qué son un crate, un workspace y un
módulo, un primer tipo pequeño y el Rust que muestra, y las pocas reglas
que respeta todo el programa, cada una
explicada en un capítulo propio más adelante en el libro. La última
sección es un mapa de dónde vive cada parte de la aplicación.

## Cargo

**Cargo** es la herramienta de compilación y el gestor de paquetes de
Rust en uno. Lee los archivos `Cargo.toml`, descarga dependencias de
[crates.io](https://crates.io), compila todo en el orden correcto,
ejecuta las pruebas y mucho más. Casi nunca llamarás al compilador
`rustc` directamente.

## Crates

Un **crate** es la unidad de compilación de Rust: una librería o un
programa. Ferrous Lineup son cinco crates, cada uno con un trabajo que
puede probarse por su cuenta:

| Crate | Tipo | Función |
|-------|------|---------|
| `lineup-core` | librería | La música y la canción: notas, escalas, acordes y armonía, la hoja de acordes, los patrones y la lista de reproducción, los archivos de proyecto y los diffs, el historial de deshacer, los archivos MIDI, las partituras guía, la banda, y la biblioteca de estilos, ritmos y canciones. Sin audio y sin interfaz. |
| `lineup-audio` | librería | El motor en tiempo real y el renderizador sin conexión: baterías y samplers, el Lineup Osc, los instrumentos SoundFont, la reproducción de canciones y patrones, los efectos, la exportación WAV, la traza de cada nota tocada, y el micrófono para las tomas. |
| `lineup-listen` | librería | Decodifica una grabación (MP3, WAV, FLAC u Ogg Vorbis), oye las notas cantadas o silbadas en ella, y guarda las tomas como archivos WAV. |
| `lineup-video` | librería | Pinta las vistas de la aplicación en memoria y las codifica como H.264 y MP3 en un archivo MP4. |
| `lineup-app` | binario (programa) | La ventana, construida con `egui`, y el MIDI a través de `midir`. Es dueño del estado que une los otros cuatro, y produce el ejecutable `ferrous-lineup`. |

¿Por qué cinco crates en lugar de uno?

- **Límites claros.** `lineup-core` *no puede* depender de la interfaz
  por accidente, porque no incluye `eframe` como dependencia. El
  compilador hace cumplir la arquitectura por nosotros.
- **Pruebas más rápidas.** `lineup-core` depende solo de `serde` y
  `ron`, así que `cargo test -p lineup-core` compila un crate pequeño,
  no todo el toolkit de GUI, los drivers de audio ni los codificadores
  de video.
- **Las dependencias pesadas quedan donde hacen falta.** Solo
  `lineup-listen` decodifica formatos de audio, y solo `lineup-video`
  compila los codificadores de H.264 y MP3 desde sus fuentes en C.
- **Reutilización.** Sobre `lineup-core` y `lineup-audio` podría
  construirse un renderizador de línea de comandos o un plugin sin tocar
  el código de la ventana. La traza de reproducción ya funciona así:
  `ferrous-lineup --trace` toca una canción con `lineup-audio` sin abrir
  ninguna ventana (ver [La traza de reproducción](../sound/trace.md)).

Las dependencias solo apuntan en una dirección:

```text
lineup-app ──▶ lineup-audio ──▶ lineup-core
    │ ├──────▶ lineup-listen ──▶ lineup-core
    │ └──────▶ lineup-video
    └────────────────────────▶ lineup-core
```

`lineup-video` no sabe nada de canciones: recibe imágenes y sonido y
hace un archivo con ellos.

### Nombre del paquete y nombre del programa

El programa se compila a partir del paquete `lineup-app`, pero el
archivo que escribe Cargo se llama `ferrous-lineup`. Los dos nombres
están en lugares distintos de
[`crates/lineup-app/Cargo.toml`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/Cargo.toml):

```toml
[package]
name = "lineup-app"

[[bin]]
name = "ferrous-lineup"
path = "src/main.rs"
```

`[package] name` es como Cargo y los demás crates se refieren al
paquete (`cargo test -p lineup-app`). `[[bin]]` describe un programa
que compila el paquete; los corchetes dobles son la forma de TOML de
decir «una entrada de una lista», porque un paquete puede compilar
varios programas. Sin una entrada `[[bin]]`, Cargo llamaría al programa
como el paquete, `lineup-app`. Un crate llamado `lineup-core` se
escribe `lineup_core` en las líneas `use`, porque un guion es un signo
menos en Rust y no puede formar parte de un nombre; Cargo convierte los
guiones en guiones bajos por ti.

## El workspace

Un **workspace** es un grupo de crates que se compilan juntos y comparten
un archivo `Cargo.lock` y una carpeta de salida `target/`. Mira el
[`Cargo.toml`](https://github.com/naelolaiz/ferrous-lineup/blob/main/Cargo.toml)
de nivel superior:

```toml
[workspace]
resolver = "3"
members = [
    "crates/lineup-core",
    "crates/lineup-audio",
    "crates/lineup-listen",
    "crates/lineup-video",
    "crates/lineup-app",
]

[workspace.package]
version = "0.4.0"
edition = "2024"
license = "GPL-3.0-or-later"
repository = "https://github.com/naelolaiz/ferrous-lineup"
rust-version = "1.95"

[workspace.dependencies]
lineup-core = { path = "crates/lineup-core" }
cpal = "0.18"
...
```

- `members` enumera los crates.
- `[workspace.package]` contiene los ajustes compartidos por todos los
  crates. Cada crate escribe `version.workspace = true` para heredarlos,
  de modo que el número de versión vive en un solo lugar. `rust-version`
  es el compilador de Rust más antiguo que puede compilar el código; uno
  más antiguo se detiene con un mensaje claro en lugar de un error
  confuso.
- `[workspace.dependencies]` fija la versión de cada dependencia una
  vez. Un crate escribe `cpal.workspace = true` para usarla. Así todos
  los crates usan la misma versión de cada librería.

Una versión como `"0.18"` significa «0.18.0 o cualquier 0.18.x más
nueva». Las versiones exactas que eligió Cargo quedan registradas en
`Cargo.lock`, que se incluye en el repositorio para que todos compilen
exactamente el mismo código.

### Ediciones

`edition = "2024"` elige la **edición** del lenguaje Rust. Las ediciones
permiten a Rust hacer pequeños cambios incompatibles cada pocos años sin
romper el código antiguo: cada crate declara en qué edición está escrito,
y los crates de ediciones distintas funcionan bien juntos.

## Módulos

Dentro de un crate, el código se organiza en **módulos**. Un módulo suele
ser un archivo. En
[`crates/lineup-core/src/lib.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/lib.rs):

```rust
pub mod pitch;   // loads src/pitch.rs as the module `pitch`
pub mod tempo;   // loads src/tempo.rs as the module `tempo`
// ... and about thirty more

pub use pitch::Note;   // re-export: users can write lineup_core::Note
pub use tempo::Tempo;
```

- `mod pitch;` le dice al compilador que busque `pitch.rs` junto a
  `lib.rs`.
- `pub` lo hace visible fuera del crate. Sin `pub`, todo en Rust es
  **privado** por defecto, visible solo dentro de su propio módulo.
- `use` trae un nombre al ámbito, como `import` en otros lenguajes.
  `pub use` además lo reexporta.

Un módulo puede tener módulos propios. `harmony.rs` declara
`pub mod chord;`, y el compilador lo encuentra en `harmony/chord.rs`: la
carpeta lleva el nombre del módulo al que pertenece.

`lib.rs` es la **raíz del crate** de una librería; `main.rs` es la raíz
del crate de un programa. Observa que el crate se llama `lineup-core` en
`Cargo.toml` pero `lineup_core` en el código: el código Rust no puede
contener `-` en los nombres, así que Cargo lo sustituye por `_`.

## Comentarios de documentación

Los comentarios que empiezan por `///` documentan el elemento que tienen
debajo, y `//!` documenta el módulo o crate que los contiene. Están
escritos en Markdown. Ejecuta `cargo doc --open` para verlos renderizados
como un sitio web navegable: cada tipo y función de Ferrous Lineup
aparece con su explicación.

## Un primer tipo: `Note`

Casi todo `lineup-core` son tipos pequeños con funciones propias. El más
pequeño, `Note` en `pitch.rs`, muestra las piezas que usa cada capítulo
posterior. Una nota se guarda como su número MIDI, de 0 a 127, y el do
central es el 60:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(from = "u8", into = "u8")]
pub struct Note(u8);
```

`Note` es un **tuple struct**: un struct cuyos campos no tienen nombre y
se leen como `.0`, `.1`, etc. Con un solo campo que le da a un número un
tipo propio, es un *newtype*, que [Clases de altura, notas y su
escritura](../theory/pitch.md#una-nota-un-número-midi-con-un-tipo-propio)
explica junto con la música de la nota. El campo no tiene `pub`, así
que el código fuera de `pitch.rs` no puede escribir `Note(200)`.

### Métodos: bloques `impl`

Las funciones de un tipo se escriben aparte de sus datos, en un bloque
`impl`:

```rust
impl Note {
    /// The lowest MIDI note.
    pub const MIN: Note = Note(0);
    /// The highest MIDI note.
    pub const MAX: Note = Note(127);
    /// Middle C.
    pub const C4: Note = Note(60);
    // ...

    /// Creates a note from a MIDI number, clamping it to 0..=127.
    pub fn new(midi: u8) -> Self {
        Note(midi.min(Self::MAX.0))
    }

    /// The MIDI note number.
    pub fn midi(self) -> u8 {
        self.0
    }
    // ...
}
```

- `pub const C4` es una **constante asociada**, que se escribe
  `Note::C4`.
- `Self` es el tipo para el que es el `impl`, aquí `Note`.
- Una función cuyo primer parámetro es `self` es un **método**, y se
  llama como `note.midi()`. Una sin `self`, como `new`, es una **función
  asociada**, y se llama como `Note::new(60)`. Rust no tiene
  constructores; `new` es solo el nombre de costumbre.
- La última expresión de una función, escrita sin `;`, es lo que
  devuelve.
- `midi` recibe `self` por valor, no `&self`: una `Note` es `Copy`, así
  que pasarla copia un solo byte.

### Traits, y `derive`

Un **trait** es un conjunto de métodos que un tipo promete tener, como
una interfaz en otros lenguajes. El `fmt::Display` de la biblioteca
estándar dice cómo escriben un valor `{}` en `format!` y `to_string()`:

```rust
impl fmt::Display for Note {
    /// Writes the note as a name and octave, such as `C#4`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.pitch_class(), self.octave())
    }
}
```

Así, `Note::new(61).to_string()` es `"C#4"` (o `"Do#4"` si las notas se
nombran en solfeo), y cada lugar que muestra una nota la escribe de la
misma manera.

Muchos traits son iguales para cualquier tipo: comparar campo por
campo, copiar, imprimir para depurar. **`#[derive(...)]`** le pide al
compilador que los escriba por ti:

| Trait | Da |
|-------|----|
| `Debug` | la impresión con `{:?}`, para depurar y para los tests que fallan |
| `Clone`, `Copy` | una copia del valor; con `Copy`, asignar copia en lugar de mover |
| `PartialEq`, `Eq` | `==` y `!=` |
| `PartialOrd`, `Ord` | `<`, `>` y ordenar |
| `Hash` | usarlo como clave en un `HashMap` |

`Serialize` y `Deserialize` vienen de serde, no del compilador
([Guardar](saving.md#serde-describir-los-datos-una-sola-vez)).
`Default` es el trait de «un valor razonable para empezar»; `Tempo`
escribe el suyo, 130 BPM:

```rust
impl Default for Tempo {
    fn default() -> Self {
        Tempo { bpm: 130.0 }
    }
}
```

### Números: `as`

Rust nunca cambia el tipo de un número por su cuenta. `as` lo dice,
como en `octave`:

```rust
pub fn octave(self) -> i8 {
    (self.0 / 12) as i8 - 1
}
```

`self.0 / 12` es un `u8`, que no puede ser negativo, y la octava de las
notas más graves es -1, así que el resultado de la división se vuelve
un `i8` antes de restarle 1. Los índices de una lista deben ser
`usize`, así que `as usize` aparece donde un número elige un elemento.

### Pánicos

Cuando un programa llega a algo que nunca debe pasar, como un índice
más allá del final de una lista, Rust entra en **pánico** (*panic*):
detiene el hilo y escribe dónde y por qué. `assert!`, `assert_eq!`,
`unwrap` y `expect` entran en pánico a propósito cuando su condición
falla, que es como falla un test ([Cómo se prueba el
código](../shipping/testing.md)). Un pánico significa un error del
programa, nunca una entrada mala: un archivo de proyecto dañado es un
error que la aplicación muestra, y los tests se aseguran de que ningún
archivo pueda hacer entrar en pánico al lector
([Guardar](saving.md#los-archivos-dañados-nunca-hacen-caer-la-aplicación)).

## Reglas que respeta todo el programa

Unas pocas decisiones dan forma a cada parte del código. Cada una tiene
un capítulo que la muestra en acción:

1. **El hilo de audio nunca espera.** La tarjeta de sonido pide muestras
   cada pocos milisegundos, y una respuesta tardía es un clic. El hilo
   de audio nunca toma un lock, nunca reserva memoria y nunca hace una
   llamada al sistema; lee comandos de una cola y deja sus mensajes en
   otras. Ver [Dos hilos](threads.md).
2. **La ventana es dueña del proyecto; el hilo de audio guarda su propia
   copia** de lo justo que necesita para sonar. Los dos nunca comparten
   datos, así que nunca necesitan un lock.
3. **Una sola manera de avisar al hilo de audio.** Cada edición, y cada
   deshacer y rehacer, compara el proyecto antes y después y envía las
   diferencias. Hay un solo trozo de código que acertar, y se prueba por
   su cuenta. Ver [Avisar al hilo de audio de lo que
   cambió](sync.md).
4. **Guardar copias, no instrucciones.** Deshacer guarda copias enteras
   del proyecto, no una lista de cómo revertir cada edición. Una copia
   no puede revertirse mal. Ver [Deshacer y rehacer](undo.md).
5. **Las cosas que se refieren unas a otras por número cambian
   juntas.** Un clip dice «patrón 2»; si se elimina el patrón 1, el clip
   tiene que decir ahora «patrón 1». El proyecto es dueño de todas sus
   listas y solo deja que cambien a través de sus métodos. Ver [El
   proyecto](project.md).
6. **Un archivo es un contrato con el futuro.** El formato de archivo
   tiene sus propios tipos, separados de los que usa el programa, y un
   número de versión que solo cambia cuando hace falta. Ver
   [Guardar](saving.md).
7. **Dibujar está separado de cambiar.** El código que dibuja una
   ventana devuelve una lista de acciones, y la aplicación las aplica en
   un solo lugar, con la misma función de edición. Ver [Los
   editores](editors.md).
8. **La teoría musical como tipos.** Las clases de alturas, las escalas,
   los acordes y «lo que está sonando» son tipos pequeños de Rust con
   métodos, probados como cualquier otro código. Ver [Clases de altura,
   notas y su escritura](../theory/pitch.md).
9. **Las herramientas son funciones simples.** Cada herramienta de
   edición recibe notas y devuelve notas nuevas, sin saber nada de
   ventanas ni de ratones. Ver [Herramientas de edición como funciones
   simples](../theory/tools.md).
10. **El conocimiento como datos.** Los estilos, los ritmos y las
    canciones se escriben como cadenas cortas en tablas que una persona
    puede leer y revisar, en números romanos y pasos de un compás, de
    modo que la misma tabla suena en cualquier tonalidad y compás. Ver
    [La biblioteca](../band/library.md).
11. **La banda solo escribe patrones.** Probar un estilo o tocar una
    hoja de acordes es una edición corriente, así que deshacer, guardar y
    el hilo de audio funcionan sin saber que la banda existe. Ver [Cómo
    toca la banda una hoja de acordes](../band/band.md).
12. **El mismo código, más rápido que el tiempo real.** Exportar un
    archivo WAV, un video o una traza ejecuta exactamente el mismo
    código que usa la tarjeta de sonido, en un hilo propio, tan rápido
    como puede el equipo. Ver [Exportar](../sound/export.md).

## Dónde vive cada parte

Cada archivo fuente empieza con un comentario `//!` que dice para qué
sirve; este mapa está hecho con sus primeras líneas. `lineup-core`:

| Módulo | Qué contiene |
|---|---|
| `pitch.rs`, `tempo.rs`, `meter.rs` | las notas y sus nombres, el tempo, los compases |
| `pattern.rs`, `project.rs`, `playlist.rs`, `arrangement.rs` | patrones, canales, pistas y clips, y la canción tal como suena |
| `sequencer.rs`, `swing.rs` | dónde está la reproducción, paso a paso |
| `diff.rs`, `history.rs`, `file.rs` | qué cambió, deshacer, guardar |
| `harmony/` | escalas, acordes, cifrados, tonalidades, conjuntos, series, micromodos, oír la armonía, conducción de voces |
| `tools.rs` | las herramientas del piano roll |
| `chord_sheet/`, `band/`, `bars.rs` | la hoja de acordes y la banda que la toca |
| `library/` | estilos, ritmos, canciones y demostraciones, escritos como datos |
| `lead_sheet/` | la melodía en notación |
| `midi.rs`, `midi_file.rs`, `record.rs` | mensajes MIDI, archivos MIDI, grabación |
| `mixer.rs`, `effects.rs`, `score_order.rs` | los ajustes del mezclador |
| `i18n/` | el castellano de cada texto que escribe la aplicación |

`lineup-audio`: `engine.rs` (la tarjeta de sonido y el hilo de audio),
`instrument.rs`, `drums.rs`, `timbre.rs`, `sample.rs` y `soundfont.rs`
(los instrumentos), `effects.rs` y `reverb.rs`, `levels.rs`,
`metronome.rs`, `render.rs` (tocar sin tarjeta de sonido) y `trace.rs`.

`lineup-app`: `app.rs` y los módulos de `app/` (menús, archivos,
edición, exportación, MIDI, ajustes), un módulo por ventana
(`channel_rack.rs`, `piano_roll.rs`, `playlist.rs`, `chord_sheet.rs`,
`harmony_keys.rs`, `library_browser.rs`, `lead_sheet.rs`,
`twelve_tone.rs`, `micromodes.rs`, `style_editor.rs`,
`ear_training.rs`), y las piezas compartidas: `theme.rs`, `widgets.rs`,
`icons.rs` e `i18n.rs`.
