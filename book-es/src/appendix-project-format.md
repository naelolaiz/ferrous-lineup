# Formato de archivo de proyecto

Un proyecto de Ferrous Lineup es un archivo de texto en **RON**, "Rusty
Object Notation", que se parece a la propia sintaxis de structs y enums
de Rust. La aplicación lo escribe mediante `serde`, lo lee mediante los
mismos tipos, y lo guarda con la extensión `.lineup`.

```text
(
    format: 28,
    tempo: 130.0,
    scale: (root: C, kind: Major),
    channels: [
        (name: "Kick", kind: Sampler(Builtin(Kick)), volume: 0.8, muted: false),
        // ...
    ],
    // ...
)
```

[Guardar y cargar proyectos](step-4/saving.md) presenta RON, serde y el
struct `ProjectFile` que media entre el archivo y el `Project` en vivo.
Esta página es la referencia para el número de arriba: `FORMAT_VERSION`
es **28** en
[`crates/lineup-core/src/file.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/file.rs).

## El número de formato

El número dice qué características puede contener un archivo. No es la
versión de la aplicación: 0.1 y 0.2 escriben ambas el formato 1 cuando
la canción no usa nada nuevo.

`format_for` escribe el formato **más bajo** que puede contener el
proyecto, no siempre el 28. Un proyecto sencillo se queda en el formato
1, de modo que un Ferrous Lineup más antiguo todavía puede abrirlo. Una
canción con chord sheet pero sin banda es el formato 12; una banda es el
formato 13; un canal bloqueado es al menos el formato 23. El número es
la característica más alta de la canción, y las características son
acumulativas.

Los formatos posteriores añadieron campos con valores por defecto de
serde, de modo que el texto de una canción antigua no cambia. Un campo
en su valor por defecto se omite, y un campo ausente recibe su valor por
defecto al leer el archivo. Así creció el formato sin un paso de
migración: los archivos viejos son simplemente viejos, y los archivos
nuevos llevan solo lo que necesitan.

| Formato | Qué añadió | Qué haría mal una versión anterior | Dónde se explica |
|---:|---|---|---|
| 1 | El primer diseño: canales, patrones, pistas, clips, tempo y tonalidad. | No existe nada más antiguo; este es el suelo. | [Guardar y cargar proyectos](step-4/saving.md) |
| 2 | Canales que tocan un sintetizador MIDI externo. | No conocería el canal `MidiOut` y no enviaría las notas. | [Teclas en vivo](step-7/live.md), [MIDI Out para cada canal](step-8/midi-out-every-channel.md) |
| 3 | Nueve baterías integradas, patrones de más de cuatro compases, y compases. | No conocería las baterías, cortaría los patrones largos sin avisar, y dibujaría cada compás en cuatro. | [Los compases](step-8/time-signatures.md), [La biblioteca](step-8/library.md) |
| 4 | Secciones, clips que se repiten, y una región de loop. | Reproduciría cada repetición una sola vez y perdería el loop. | [La estructura de la canción](step-8/song-structure.md) |
| 5 | Marcadores de tiempo, que cambian el compás a mitad de una canción. | Dibujaría cada compás en el primer compás de la canción. | [Cambiar el compás a mitad de camino](step-8/time-markers.md) |
| 6 | Una serie dodecafónica o un conjunto de clases de alturas como tonalidad. | No conocería ninguno de los dos tipos de tonalidad y no podría abrir el archivo. | [Series dodecafónicas y conjuntos de clases de alturas](step-8/twelve-tone.md) |
| 7 | Cambios de tempo a lo largo de una canción, y temporización loose. | Reproduciría toda la canción a su primer tempo y pondría cada nota en la cuadrícula. | [Cambios de tempo, y tocar como una persona](step-8/tempo-and-feel.md) |
| 8 | Cambios de tonalidad a lo largo de una canción. | Mostraría y reproduciría toda la canción en su primera tonalidad. | [Cambiar de tonalidad](step-8/changing-key.md), [Cambios de tonalidad a mitad de camino](step-8/key-changes.md) |
| 9 | Un instrumento General MIDI en un canal. | Reproduciría cada Lineup Osc con el mismo sonido y no enviaría program changes. | [Cada voz en su propio canal](step-8/voices-and-instruments.md) |
| 10 | Notas que empiezan o terminan entre pasos, como los tresillos de un archivo MIDI importado. | Las movería a la cuadrícula y cambiaría el ritmo. | [Abrir archivos MIDI](step-8/midi-import.md) |
| 11 | Pulsos agrupados de forma irregular, como 7/8 contado 2+2+3. | Leería el 7/8 pero dibujaría una línea de pulso en cada corchea. | [Cualquier compás, y métricas irregulares](step-8/any-time-signature.md) |
| 12 | El chord sheet: la armonía de la canción, compás a compás. | Perdería el chord sheet sin avisar al guardar. | [El chord sheet](step-9/chord-sheet.md) |
| 13 | La banda que toca el chord sheet, y los patrones que escribe. | Conservaría los patrones de la banda como patrones normales y dejaría de seguir el chord sheet. | [La banda toca el chord sheet](step-9/band.md) |
| 14 | La forma de la canción de la banda: intro, estribillos repetidos y un final. | Reproduciría la canción una vez, tal como está escrita, sin repeticiones ni final. | [La forma de la canción](step-9/song-form.md) |
| 15 | La segunda variación del estilo, elegida con marcadores en el chord sheet. | Reproduciría toda la canción en la primera variación. | [Variaciones a y b](step-9/variations.md) |
| 16 | El swing, las corcheas tocadas largo-corto. | Reproduciría cada par de corcheas parejo. | [Swing](step-9/swing.md) |
| 17 | La guitarra, las cuerdas y el órgano de la banda. | Solo conocería el piano, el bajo y la batería, y no podría leer sus partes. | [Una banda más grande](step-9/bigger-band.md) |
| 18 | Shots, holds, silencios y pushes en el chord sheet. | Soltaría las marcas y tocaría el estilo de corrido. | [Shots, holds, silencios y pushes](step-9/stops.md) |
| 19 | El solista de la banda. | No conocería ni al músico solista ni la parte del solista. | [El solista](step-9/soloist.md) |
| 20 | El ritmo de una banda separado de sus instrumentos, el estilo de armonía exacto, y los acentos sencillos. | Usaría un estilo para cada parte y perdería el ajuste de solo acentos. | [Una orquesta para la música que cambia de métrica](step-9/orchestra.md), [Los estilos conservan el tempo de la canción](step-9/style-tempo.md) |
| 21 | El master del mixer, el solo y la reverb estéreo guardados. | Reproduciría la canción con el master y la reverb por defecto, no con la mezcla que guardaste. | [El mixer](step-9/mixer.md) |
| 22 | Patrones e instrumentos de estilos de usuario incrustados con la banda. | Recurriría al estilo que conoce por nombre, así que la banda tocaría otra cosa. | [El editor de estilos](step-9/style-editor.md) |
| 23 | Canales bloqueados. | Dejaría que los estilos y la banda escribieran encima de la parte bloqueada. | [Bloquear una pista](step-9/locks.md) |
| 24 | Un estilo distinto para una parte de una canción. | Reproduciría toda la canción en el propio estilo de la banda. | [Un estilo para cada sección](step-9/section-styles.md) |
| 25 | Dos acordes a la vez en el chord sheet, escritos `Eb7` sobre `Fb`. | No podría leer un poliacorde, así que no podría abrir el archivo. | [Oír acordes](step-9/chord-hearing.md) |
| 26 | Un paneo y un envío de reverb en cada canal. | Reproduciría cada canal en el centro con todo él en la reverb. | [El mixer](step-9/mixer.md) |
| 27 | Enviar una batería integrada por MIDI. | Reproduciría la batería él mismo en lugar de enviarla al sintetizador externo. | [Enviar un canal por MIDI](step-9/midi-out-menus.md) |
| 28 | Una sección de vientos en la banda, y EQ, compresión y delay en un canal. | No tocaría vientos, y cada canal sin sus efectos. | [Una sección de vientos](step-9/horn-section.md), [EQ, compresión y delay](step-9/channel-effects.md) |

Un archivo también puede decir de dónde vinieron las partes de la
canción, en cualquier formato. Las versiones anteriores se saltan ese
registro y no pierden nada que pudieran reproducir. Lo mismo ocurre con
una tonalidad escrita al revés, como sol bemol mayor en lugar de fa
sostenido mayor: la tonalidad se guarda como `(root: FSharp, kind:
Major, respelled: true)`, y una versión anterior la lee como fa
sostenido mayor, con las mismas notas
([Tonalidades con dos nombres](step-9/before-the-release.md#tonalidades-con-dos-nombres)).

## Archivos del futuro

Un Ferrous Lineup más nuevo puede escribir un formato que este no ha
visto nunca. Un archivo así puede contener nombres y campos que esta
versión no puede analizar, e incluso si los analiza, el significado
puede haber cambiado. Así que el lector comprueba el número de formato y
rechaza cualquier cosa por encima de `FORMAT_VERSION` con:

```text
this file uses format 29, made by a newer version of Ferrous Lineup (this one reads up to format 28)
```

Comprueba después de leer el archivo entero, de modo que un error de RON
en un archivo del futuro se explica por su número de formato en lugar de
por un desconcertante campo ausente. Un archivo que no se puede analizar
también se escanea en busca del número de su comienzo, sin una lectura
RON completa; eso evita que una canción larga tarde un minuto en
informar del problema real.

## Guardar con seguridad

`file::save` escribe en un archivo temporal junto al destino, y luego lo
renombra. Un cierre inesperado o un disco lleno dejan, por tanto,
intacto el proyecto antiguo, y un archivo a medio escribir nunca se
confunde con uno guardado. El temporal termina en `.lineup.tmp` y se
elimina si el guardado falla.
Los finales de línea son siempre `\n`, sea cual sea el sistema, y una
marca de orden de bytes al comienzo de un archivo se ignora al leer.

El mismo patrón lo usan la exportación WAV y de vídeo, y la copia de
recuperación. Mira [Resolución de problemas](troubleshooting.md) si
falla una exportación.
