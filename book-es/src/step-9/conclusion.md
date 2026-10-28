# Dónde acaba la banda

Archivos: [`Cargo.toml`](https://github.com/naelolaiz/ferrous-lineup/blob/main/Cargo.toml)
y [`README.md`](https://github.com/naelolaiz/ferrous-lineup/blob/main/README.md)

Ferrous Lineup empezó como un step sequencer con un tono de prueba.
Acaba como banda de acompañamiento: un chord sheet, una biblioteca de
estilos y canciones, un playlist, un piano roll, un mixer, MIDI de
entrada y salida, y una manera de oír lo que toca la banda. Los últimos
capítulos trataron menos de música nueva que de hacer fiable esa banda:
recuperación, proyectos recientes, una tarjeta de sonido que puede
volver, y comandos que llegan.

## Cinco crates

El workspace son cinco crates, cada uno con un trabajo que puede
probarse por su cuenta:

- **`lineup-core`** guarda la música y la canción: notas, escalas,
  acordes y armonía, el chord sheet, los patrones y el playlist, los
  archivos de proyecto y los diffs, el historial de deshacer, los
  archivos MIDI, los lead sheets, y la biblioteca de estilos, ritmos y
  canciones. No tiene audio ni interfaz, y depende solo de `serde` y
  `ron`, así que `cargo test -p lineup-core` es rápido.
- **`lineup-audio`** es el motor en tiempo real (`cpal`) y el
  renderizador sin conexión: baterías y samplers, el Lineup Osc, los
  instrumentos SoundFont, la reproducción de canciones y patrones, la
  exportación WAV, y la traza de cada nota tocada.
- **`lineup-listen`** decodifica una grabación (MP3, WAV, FLAC u Ogg
  Vorbis) y escucha sus pulsos, tonalidad, acordes y compás. Es
  experimental: los resultados son aproximados y están pensados como
  punto de partida.
- **`lineup-video`** pinta las vistas de la app en memoria y las
  codifica como H.264 y MP3 en un archivo MP4. Los codificadores se
  compilan desde sus fuentes en Do, así que no hace falta ningún códec
  del sistema.
- **`lineup-app`** es el programa de escritorio, `ferrous-lineup`,
  construido con `egui`, con MIDI a través de `midir`. Es dueño de las
  ventanas y del estado que une los otros cuatro crates.

## Qué hace ahora

Una canción puede empezar en el chord sheet, en un archivo MIDI, en un
chart de acordes pegado como texto, en una grabación, o en una de las
demos. Escribe acordes compás a compás, elige un estilo, y la banda los
toca en cualquier tonalidad y tempo, con una intro, estribillos, un
final, variaciones, swing, shots y pushes. Cada parte cae en una pista
propia, donde el piano roll puede cambiarla a mano.

Alrededor están las herramientas que la banda fue haciendo crecer: un
editor de estilos para tus propias partes que se repiten, un mixer con
paneo, reverb y medidores, una barra de práctica con cuenta de entrada,
metrónomo y loops, y las ventanas de armonía que nombran lo que oyes y
lo que podría venir después. Los teclados MIDI tocan hacia dentro; los
canales MIDI Out y un SoundFont tocan hacia fuera. Una canción puede
salir como WAV, MIDI, un PDF de lead sheet, o un vídeo MP4. Un proyecto
puede sobrevivir a un cierre inesperado, y el sonido puede volver
después de desenchufar la interfaz.

## Los números

La biblioteca integrada está escrita como datos en `lineup-core`:

- **177** estilos de armonía,
- **112** ritmos, más rellenos,
- **343** demos de canciones.

Eso basta para tocar *Ode to Joy* como pista de house o *When the
Saints* como Dixieland. El capítulo de la [biblioteca](../step-8/library.md)
explica cómo funcionan las tablas; el capítulo de las
[Demos](demo-search.md) explica cómo buscarlas.

## Ejecutarlo y probarlo

Desde el repositorio:

```sh
cargo run --release
```

Usa `--release`: el código de audio y vídeo es demasiado lento en una
compilación debug. `cargo run --release -- --demos` lista cada demo, y
`cargo run --release -- song.lineup` abre un proyecto directamente.

```sh
cargo test --workspace
```

Las pruebas se ejecutan sin pantalla ni tarjeta de sonido. Las pruebas
de interfaz dibujan la app sin pantalla con `Audio::Unavailable`, así
que funcionan en CI. El proyecto también ejecuta
`cargo fmt --all --check` y
`cargo clippy --workspace --all-targets -- -D warnings`, y luego
compila el binario de release, como describe [Pruebas e integración
continua](../step-1/ci.md).

## Los apéndices del libro

Los capítulos son el curso; la parte de atrás del libro es para buscar
cosas. El [glosario de Rust](../glossary.md) y el [glosario
musical](../music-glossary.md) definen los términos, con enlaces a los
capítulos. El [formato de archivo de
proyecto](../appendix-project-format.md), los [atajos de
teclado](../appendix-shortcuts.md) y la [resolución de
problemas](../troubleshooting.md) son referencias. La [hoja de
ruta](../roadmap.md) es la tabla de estado y las ideas que aún no se
han construido.

También está en línea y como un solo PDF, hecho desde el mismo
Markdown por el flujo de [El libro en línea y como PDF](book-online.md).

## Qué queda

La hoja de ruta se cuida de decir que ninguna de sus ideas está
planeada todavía. Algunas son pequeñas; algunas cambiarían la forma de
la app:

- un solista que ya no sea un experimento;
- automatización, dibujada o grabada sobre las pistas;
- clips de audio en las pistas, para tocar junto a una grabación;
- grabar en modo canción.

También está el hueco honesto del paso 7: MIDI se construyó en una
computadora sin hardware MIDI, así que los teclados y sintetizadores
reales se probaron por otros medios. La app ha crecido mucho desde
entonces; el hueco sigue mereciendo la pena cerrarlo.

Ferrous Lineup es software libre bajo la GNU General Public License,
versión 3 o posterior. La banda es tuya para cambiarla: cada estilo,
ritmo y canción de la biblioteca es una línea de una tabla, y cada
capítulo señala el código que la hace.
