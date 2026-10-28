# Cargo, workspaces y crates

## Cargo

**Cargo** es la herramienta de compilación y el gestor de paquetes de
Rust en uno. Lee los archivos `Cargo.toml`, descarga dependencias de
[crates.io](https://crates.io), compila todo en el orden correcto,
ejecuta las pruebas y mucho más. Casi nunca llamarás al compilador
`rustc` directamente.

## Crates

Un **crate** es la unidad de compilación de Rust: una librería o un
programa. En este paso, Ferrous Lineup estaba dividido en tres crates:

| Crate | Tipo | Función |
|-------|------|---------|
| `lineup-core` | librería | Conceptos musicales: notas, tempo, más adelante patrones y acordes. Sin audio, sin interfaz. |
| `lineup-audio` | librería | Habla con la tarjeta de sonido y genera sonido. |
| `lineup-app` | binario (programa) | La ventana. Produce el ejecutable `ferrous-lineup`. |

> **En este paso / hoy:** el workspace tenía estos tres crates y la
> versión 0.1.0. Hoy hay cinco: se unieron `lineup-listen` (decodifica y
> analiza una grabación para **File > Import recording**) y
> `lineup-video` (pinta imágenes de egui en un MP4), y la versión es la
> 0.2.0. Los enlaces al código de este capítulo van a `main`, de modo que
> los archivos que abren son los finales, no la instantánea del paso 1.
> [Un nombre nuevo](../step-9/new-name.md) explica por qué `loom-*` pasó
> a ser `lineup-*`.

¿Por qué tres crates en lugar de uno?

- **Límites claros.** `lineup-core` *no puede* depender de la interfaz
  por accidente, porque no incluye `eframe` como dependencia. El
  compilador hace cumplir la arquitectura por nosotros.
- **Pruebas más rápidas.** `cargo test -p lineup-core` compila solo un
  crate diminuto, no todo el toolkit de GUI.
- **Reutilización.** Más adelante podríamos construir un renderizador de
  línea de comandos o un plugin sobre `lineup-core` y `lineup-audio` sin
  tocar el código de la ventana.

Las dependencias solo apuntan en una dirección:

```text
lineup-app ──▶ lineup-audio ──▶ lineup-core
    └─────────────────────────▲
```

## El workspace

Un **workspace** es un grupo de crates que se compilan juntos y comparten
un archivo `Cargo.lock` y una carpeta de salida `target/`. Mira el
[`Cargo.toml`](https://github.com/naelolaiz/ferrous-lineup/blob/main/Cargo.toml)
de nivel superior:

```toml
[workspace]
resolver = "3"
members = ["crates/lineup-core", "crates/lineup-audio", "crates/lineup-app"]

[workspace.package]
version = "0.1.0"
edition = "2024"
license = "GPL-3.0-or-later"
...

[workspace.dependencies]
lineup-core = { path = "crates/lineup-core" }
cpal = "0.18"
...
```

- `members` enumera los crates.
- `[workspace.package]` contiene los ajustes compartidos por todos los
  crates. Cada crate escribe `version.workspace = true` para heredarlos,
  de modo que el número de versión vive en un solo lugar.
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

pub use pitch::Note;   // re-export: users can write lineup_core::Note
pub use tempo::Tempo;
```

- `mod pitch;` le dice al compilador que busque `pitch.rs` junto a
  `lib.rs`.
- `pub` lo hace visible fuera del crate. Sin `pub`, todo en Rust es
  **privado** por defecto, visible solo dentro de su propio módulo.
- `use` trae un nombre al ámbito, como `import` en otros lenguajes.
  `pub use` además lo reexporta.

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
