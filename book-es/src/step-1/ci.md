# Pruebas e integración continua

## Ejecutar las pruebas

```sh
cargo test                 # every crate
cargo test -p lineup-core    # one crate
cargo test frequency       # only tests whose name contains "frequency"
```

El paso 1 tenía pruebas en cuatro archivos:

| Archivo | Qué comprueba |
|---------|---------------|
| `lineup-core/src/pitch.rs` | El La4 está a 440 Hz, el do central a 261.63 Hz, las octavas duplican la frecuencia, los nombres de las notas |
| `lineup-core/src/tempo.rs` | El Tempo se limita a su rango, las duraciones de pulso son correctas |
| `lineup-audio/src/synth.rs` | El tono de prueba, en este paso: en silencio cuando está apagado, entra en fundido sin clic, se mantiene dentro de su volumen, reproduce la frecuencia correcta (hoy las pruebas del sintetizador viven en `instrument.rs`) |
| `lineup-audio/src/engine.rs` | Los comandos llegan al hilo de audio; los formatos de muestra enteros usan su rango completo |

La prueba de frecuencia del tono de prueba es un buen ejemplo de probar
audio sin escucharlo. Renderiza un segundo de un tono de 100 Hz y cuenta
cuántas veces la onda cruza el cero subiendo. Debería ocurrir 100 veces:

```rust
let rising_crossings = buffer
    .windows(2)
    .filter(|w| w[0] < 0.0 && w[1] >= 0.0)
    .count();
```

Es una **cadena de iteradores**. `windows(2)` produce cada par de
muestras vecinas, `filter` se queda con los pares que pasan de negativo
a no negativo, y `count` los cuenta. Los iteradores son perezosos: no
pasa nada hasta que `count` pide valores, y el compilador convierte toda
la cadena en un bucle simple, tan rápido como uno escrito a mano.

## Formato y linting

Dos herramientas más mantienen el código ordenado:

- `cargo fmt` reescribe el código en el estilo estándar de Rust, para
  que nadie discuta dónde van las llaves.
- `cargo clippy` es el **linter** oficial. Conoce cientos de patrones
  que compilan bien pero podrían ser más simples, rápidos o correctos, y
  explica cada sugerencia.

## Integración continua

Archivo:
[`.github/workflows/ci.yml`](https://github.com/naelolaiz/ferrous-lineup/blob/main/.github/workflows/ci.yml)

En este paso, cada push y pull request ejecutaba un trabajo de Linux,
**check**: instalaba las librerías del sistema, y luego ejecutaba
`cargo fmt --check`, `cargo clippy -- -D warnings` (las advertencias
cuentan como errores), `cargo test` y una compilación de release. El
programa compilado se adjuntaba a la ejecución como artefacto de Linux
(`ferrous-loom-linux` entonces, `ferrous-lineup-linux` hoy; ver
[Un nombre nuevo](../step-9/new-name.md)).

> **Hoy:** `.github/workflows/ci.yml` sigue teniendo un trabajo `check`,
> ahora una matriz de Linux, macOS y Windows. Cada runner sube su propio
> artefacto (`ferrous-lineup-linux`, `ferrous-lineup-macos` y
> `ferrous-lineup-windows`), y el runner de Linux sigue comprobando el
> formato. El libro no es un trabajo de ese workflow: tiene el suyo,
> [`.github/workflows/book.yml`](https://github.com/naelolaiz/ferrous-lineup/blob/main/.github/workflows/book.yml),
> donde **build** hace el sitio web y el PDF a partir de `book/` y
> **deploy** los publica en GitHub Pages (ver [El libro en línea y como
> PDF](../step-9/book-online.md)).

Un pull request muestra una marca verde cuando el trabajo pasa. El paso 4
añade macOS y Windows a la matriz.
