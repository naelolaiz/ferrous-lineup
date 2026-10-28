# Paso 9: La banda

Los pasos 1 a 8 construyeron las partes: loops en un step sequencer y
un piano roll, ordenados en las pistas de un playlist, con una
biblioteca de estilos detrás. Lo que el proyecto estaba destinado a ser
desde el principio es una **banda de acompañamiento**. Escribes los
acordes de una canción compás a compás, eliges un estilo, pulsas Play,
y una banda (bajo, batería, piano, guitarra, cuerdas) toca la canción
en ese estilo. Cambia el estilo, la tonalidad o el tempo, y la banda la
toca otra vez de la manera nueva.

Casi toda una banda ya estaba dentro de la app. En este paso la
biblioteca convierte un chart de acordes en acordes con voces y una
línea de bajo en 145 estilos, añade baterías de 81 ritmos, rellenos y
finales, y construye canciones enteras desde charts ([La biblioteca: el
conocimiento como datos](../step-8/library.md)). Desde entonces ha
crecido, hasta 177 estilos y 112 ritmos. Lo que faltaba es el frente:
un sitio donde escribes los acordes, que la banda luego toca. El paso 9
lo trae al frente, un cambio a la vez:

1. **Nombres de acordes**: leer acordes como los escriben los lead
   sheets, "Am7", "F#m7b5", "G/B" ([Nombres de acordes: leer un lead
   sheet](chord-names.md)), y luego un nombre nuevo para la app ([Un
   nombre nuevo](new-name.md)),
2. **un chord sheet**: una ventana de compases donde escribes los
   acordes, guardados en el proyecto. Un compás también puede contener
   un conjunto de clases de alturas, un micromodo, una forma de la
   serie, o dos acordes a la vez, un poliacorde (`Eb7|Fb`), las otras
   maneras que tiene Ferrous Lineup de nombrar la armonía ([El chord
   sheet: escribir una canción compás a compás](chord-sheet.md),
   [oír acordes](chord-hearing.md)),
3. **la banda toca el chord sheet**: los estilos se dividen en «band
   styles» (cómo tocar) y «progressions» (qué tocar) ([La banda toca el
   chord sheet](band.md)),
4. **una barra de práctica**: cuenta de entrada, metrónomo, tempo como
   porcentaje, tap tempo, un interruptor para tocar tú una parte, y
   hacer loop de los compases que elijas ([Practicar con la
   banda](practice.md)),
5. **la forma de la canción**: una intro, estribillos y un final ([La
   forma de la canción: una intro, estribillos y un final](song-form.md)),
   y una segunda variación del estilo para las partes que necesitan más
   energía ([Variaciones a y b: la banda entra en faena](variations.md)),
6. **swing**, desde straight hasta tresillo ([Swing: corcheas tocadas
   largo-corto](swing.md)),
7. **sonidos de instrumentos reales** desde un SoundFont, y la banda
   por MIDI out hacia un sintetizador externo. El SoundFont llegó
   pronto, antes de los puntos 4 a 6 ([Sonidos de instrumentos reales:
   un SoundFont](soundfont.md)),
8. **una banda más grande**: una guitarra, cuerdas y un órgano, cada
   uno donde el estilo los tiene ([Una banda más grande: guitarra,
   cuerdas y órgano](bigger-band.md)),
9. **silencios, shots, holds y pushes** escritos en el chord sheet
   ([Shots, holds, silencios y pushes](stops.md)),
10. **chord sheets como texto plano**, para pegar y compartir ([Chord
    sheets como texto: para pegar y compartir](text-chord-sheets.md)),
11. **un lead sheet**: la melodía en notación, los acordes encima,
    listo para imprimir ([El lead sheet: la melodía en
    notación](lead-sheet.md)),
12. **un solista** y 13. **entrenamiento auditivo**, ambos
    experimentales y apagados por defecto ([El solista: una línea
    improvisada sobre los acordes](soloist.md)). El entrenamiento
    auditivo llegó pronto, en una ventana propia ([Entrenamiento
    auditivo](ear-training.md)).

Nada desaparece. El playlist con sus pistas, el channel rack, el piano
roll, las harmony keys, las series dodecafónicas, los conjuntos de
clases de alturas y los micromodos se quedan todos como están; el chord
sheet es una ventana más junto a ellos.

## Después del plan

Con esos trece en su sitio, tocar canciones con la banda mostró qué más
hacía falta. Los capítulos que vienen después de este se leen mejor
como un mapa de ese trabajo, no como una segunda lista por la que ir
avanzando:

- **Más música**: [swing y big band](big-band-songs.md),
  [canciones latinoamericanas](latin-american-songs.md), [métricas
  irregulares](odd-meters.md), [un tema de
  televisión](big-bang-theory-theme.md) y [la sala de
  conciertos](concert-hall.md), con una [búsqueda](demo-search.md) para
  encontrarlas;
- **Las herramientas propias de la banda**: [el editor de
  estilos](style-editor.md), donde se hace y se guarda un estilo;
  [bloquear una pista](locks.md); [un estilo para cada
  sección](section-styles.md); [una orquesta para la música que cambia
  de métrica](orchestra.md); y [una sección de vientos para cualquier
  estilo](horn-section.md);
- **Armonía y el sheet**: [oír acordes](chord-hearing.md), [una armonía,
  cuatro nombres](harmony-notations.md), [numerales en tonalidades
  menores](minor-numerals.md), un [sheet que llena la
  ventana](chord-sheet-layout.md), [compases insertados y
  borrados](insert-delete-bars.md), [lead sheets para instrumentos en
  Si♭, Mi♭ y Fa](transposing-instruments.md), [acordes prestados,
  dominantes secundarias y acordes pivote](chord-functions.md) y [cómo
  se mueve cada parte de un acorde a otro](part-motion.md);
- **El sonido y la pantalla**: [el mixer en orden de
  partitura](mixer-score-order.md), [un mixer con paneo, reverb y
  medidores](mixer.md), [EQ, compresión y delay en cada
  canal](channel-effects.md), [cualquier canal enviado por
  MIDI](midi-out-menus.md), [exportar vídeo](video-export.md), [una
  traza de la reproducción](playback-trace.md), [luces de paso en el
  channel rack](rack-lights.md), [casillas de la
  Library](library-filters.md) y [trabajar más rápido](workflow.md);
- **Cuando algo sale mal**: [recuperación, proyectos recientes y
  reiniciar el audio](recovery.md), [una segunda cacería de
  errores](bug-hunt.md) y [una tercera](bug-hunt-3.md);
- **Descargas y el libro**: [compilaciones de lanzamiento](releases.md)
  para cada sistema, y [este libro, en línea y como PDF](book-online.md).

## Probarlo

Los cambios de este paso se probaron primero en una rama propia, y
ahora están todos en `main`:

```sh
git checkout main
git pull
cargo run --release
```

## El final del plan

Los trece cambios de arriba son el plan que este paso se propuso hacer,
y el mapa de arriba es lo que vino después. [La conclusión](conclusion.md)
mira atrás a la app que hicieron, lo que este libro deja fuera, y a
dónde ir después.
