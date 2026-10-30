# Formato del archivo de proyecto

Un proyecto de Ferrous Lineup es un archivo de texto en **RON**, «Rusty
Object Notation», que se parece a la propia sintaxis de structs y enums
de Rust. La aplicación lo escribe mediante `serde`, lo lee mediante los
mismos tipos, y lo guarda con la extensión `.lineup`.

```text
(
    format: 29,
    tempo: 130.0,
    scale: (root: C, kind: Major),
    channels: [
        (name: "Kick", kind: Sampler(Builtin(Kick)), volume: 0.8, muted: false),
        // ...
    ],
    // ...
)
```

[Guardar](../program/saving.md) explica RON, serde y el struct
`ProjectFile` que media entre el archivo y el `Project` en vivo. Esta
página es la referencia del número de arriba: `FORMAT_VERSION` es **29**
en
[`crates/lineup-core/src/file.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/file.rs).

## El número de formato

El número dice qué características puede contener un archivo. No es la
versión de la aplicación.

`format_for` escribe el formato **más bajo** que puede contener el
proyecto, no siempre el 29. Un proyecto sencillo se queda en el formato
1, así que cualquier versión de Ferrous Lineup puede abrirlo. Una
canción con hoja de acordes pero sin banda es el formato 12; una banda
es el formato 13; un canal bloqueado es al menos el formato 23. El
número es el de la característica más alta que usa la canción, y cada
formato puede contener todo lo que contienen los de debajo.

Cada campo que añade un formato tiene un valor por defecto de serde. Un
campo en su valor por defecto se omite al escribir, y un campo ausente
recibe su valor por defecto al leer. Así, una canción que no usa nada
nuevo se escribe exactamente igual que antes, y un archivo no necesita
ningún paso de conversión al abrirse: lleva solo lo que usa.

| Formato | Qué añade | Qué haría mal una versión que solo lee los formatos de debajo | Se explica en |
|---:|---|---|---|
| 1 | La base: canales, patrones, pistas, clips, tempo y tonalidad. | Nada; toda versión lo lee. | [Guardar](../program/saving.md) |
| 2 | Canales que tocan un sintetizador MIDI externo. | No conocería el canal `MidiOut` y no enviaría las notas. | [MIDI](../sound/midi.md#salida-midi-un-instrumento-con-bandeja-de-salida) |
| 3 | Nueve baterías integradas más, patrones de más de cuatro compases, e indicaciones de compás. | No conocería las baterías, cortaría los patrones largos sin avisar, y dibujaría cada compás en cuatro. | [Tempo, compás, swing y sensación](../sound/tempo.md#timesignature-dos-números-y-una-agrupación) |
| 4 | Secciones, clips que se repiten, y una región de bucle. | Tocaría cada repetición una sola vez y perdería el bucle. | [El proyecto](../program/project.md#secciones-repeticiones-y-bucles) |
| 5 | Marcas de compás, que cambian el compás a mitad de una canción. | Dibujaría cada compás en el primer compás de la canción. | [Tempo, compás, swing y sensación](../sound/tempo.md#meter-indicaciones-de-compás-que-cambian) |
| 6 | Una serie dodecafónica o un conjunto de clases de altura como tonalidad. | No conocería ninguno de los dos tipos de tonalidad y no podría abrir el archivo. | [Series dodecafónicas](../theory/twelve-tone.md#una-serie-como-tonalidad), [Conjuntos de clases de altura](../theory/sets.md#una-tonalidad-como-conjunto-las-mismas-notas-otro-nombre) |
| 7 | Cambios de tempo a lo largo de una canción, y la soltura. | Tocaría toda la canción a su primer tempo y pondría cada nota en la cuadrícula. | [Tempo, compás, swing y sensación](../sound/tempo.md#cambios-de-tempo) |
| 8 | Cambios de tonalidad a lo largo de una canción. | Mostraría y tocaría toda la canción en su primera tonalidad. | [Escalas, modos y tonalidades](../theory/scales.md#cambios-de-tonalidad-a-mitad-de-camino-marcas-de-tonalidad) |
| 9 | Un instrumento General MIDI en un canal. | Tocaría cada Lineup Osc con el mismo sonido y no enviaría cambios de programa. | [SoundFonts y General MIDI](../sound/soundfonts.md#el-instrumento-de-un-canal) |
| 10 | Notas que empiezan o terminan entre pasos, como los tresillos de un archivo MIDI importado. | Las movería a la cuadrícula y cambiaría el ritmo. | [MIDI](../sound/midi.md#notas-entre-pasos) |
| 11 | Pulsos agrupados de forma irregular, como 7/8 contado 2+2+3. | Leería el 7/8 pero dibujaría una línea de pulso en cada corchea. | [Tempo, compás, swing y sensación](../sound/tempo.md#pulsos-de-distinta-longitud) |
| 12 | La hoja de acordes: la armonía de la canción, compás a compás. | Perdería la hoja de acordes sin avisar al guardar. | [La hoja de acordes](../band/chord-sheet.md#en-el-archivo) |
| 13 | La banda que toca la hoja de acordes, y los patrones que escribe. | Conservaría los patrones de la banda como patrones normales y dejaría de seguir la hoja. | [Cómo toca la banda](../band/band.md#en-el-archivo) |
| 14 | La forma de la canción de la banda: una intro, vueltas repetidas y un final. | Tocaría la canción una vez, tal como está escrita, sin repeticiones ni final. | [Forma](../band/song-form.md#la-forma-una-introducción-vueltas-y-un-final) |
| 15 | La segunda variación del estilo, elegida con marcas en la hoja de acordes. | Tocaría toda la canción en la primera variación. | [Forma](../band/song-form.md#variaciones-a-y-b-la-banda-aprieta) |
| 16 | El swing, las corcheas tocadas largo-corto. | Tocaría cada par de corcheas parejo. | [Tempo, compás, swing y sensación](../sound/tempo.md#swing) |
| 17 | La guitarra, las cuerdas y el órgano de la banda. | Solo conocería el piano, el bajo y la batería, y no podría leer sus partes. | [Cómo toca la banda](../band/band.md#una-banda-más-grande-guitarra-cuerdas-y-órgano) |
| 18 | Golpes, sostenidos, silencios y anticipaciones en la hoja de acordes. | Perdería las marcas y tocaría el estilo de corrido. | [Golpes, sostenidos, silencios y anticipaciones](../band/stops.md#dos-marcas-opcionales-en-un-marcador) |
| 19 | El solista de la banda. | No conocería ni al músico solista ni su parte. | [El solista](../band/soloist.md) |
| 20 | El ritmo de una banda tomado de un estilo y sus instrumentos de otro, el estilo de armonía exacto, y solo los acentos. | Usaría un único estilo para todas las partes y perdería el ajuste de solo acentos. | [Cómo toca la banda](../band/band.md#una-banda-en-la-canción), [La orquesta](../band/orchestra.md) |
| 21 | El master del mezclador, el solo y la reverberación estéreo. | Tocaría la canción con el master y la reverberación por defecto, no con la mezcla que guardaste. | [El mezclador](../sound/mixer.md#qué-se-guarda) |
| 22 | Tus propios estilos, guardados con la banda. | Recurriría al estilo que conoce por su nombre, así que la banda tocaría otra cosa. | [Tus propios estilos](../band/style-editor.md#el-estilo-viaja-con-la-canción) |
| 23 | Canales bloqueados. | Dejaría que los estilos y la banda escribieran encima de la parte bloqueada. | [Forma](../band/song-form.md#bloquear-una-pista-los-estilos-tocan-a-su-alrededor) |
| 24 | Un estilo distinto para una parte de una canción. | Tocaría toda la canción en el estilo propio de la banda. | [Forma](../band/song-form.md#un-estilo-para-cada-sección) |
| 25 | Dos acordes a la vez en la hoja de acordes, escritos `Eb7` sobre `Fb`. | No podría leer un poliacorde, así que no podría abrir el archivo. | [Oír la armonía](../theory/hearing.md#dos-acordes-a-la-vez) |
| 26 | Un balance y un envío de reverberación en cada canal. | Tocaría cada canal en el centro y con todo su sonido en la reverberación. | [El mezclador](../sound/mixer.md#el-balance-dos-altavoces-un-canal) |
| 27 | Enviar una batería integrada por MIDI. | Tocaría la batería ella misma en lugar de enviarla al sintetizador externo. | [MIDI](../sound/midi.md#un-canal-desde-su-menú-de-instrumento) |
| 28 | Una sección de vientos en la banda, y EQ, compresión y delay en un canal. | No tocaría vientos, y tocaría cada canal sin sus efectos. | [Cómo toca la banda](../band/band.md#una-sección-de-vientos), [El mezclador](../sound/mixer.md#los-efectos-de-cada-canal) |
| 29 | Una banda que toca la batería de otro estilo que sus partes de acordes y bajo, o que deja la batería a la canción. | Tocaría la batería del estilo propio de la banda. | [Cómo toca la banda](../band/band.md#una-banda-en-la-canción), [La biblioteca](../band/library.md#qué-partes-tomar) |

Dos registros pueden estar en un archivo de cualquier formato, porque
una versión que se los salta sigue tocando la canción exactamente
igual. Uno dice de dónde vino cada parte de la canción ([La
biblioteca](../band/library.md#de-dónde-vino-cada-parte)). El otro es
una tonalidad escrita al revés, como sol bemol mayor en lugar de fa
sostenido mayor: la tonalidad se guarda como `(root: FSharp, kind:
Major, respelled: true)`, y una versión que no conoce `respelled` la lee
como fa sostenido mayor, con las mismas notas ([Escalas, modos y
tonalidades](../theory/scales.md#tonalidades-que-se-pueden-escribir-de-dos-maneras)).

## Archivos de una versión más nueva

Un Ferrous Lineup más nuevo puede escribir un formato que este no ha
visto nunca. Un archivo así puede contener nombres y campos que esta
versión no sabe leer, e incluso si los lee, su significado puede haber
cambiado. Así que el lector comprueba el número de formato y rechaza
todo lo que esté por encima de `FORMAT_VERSION`:

```text
this file uses format 30, made by a newer version of Ferrous Lineup (this one reads up to format 29)
```

Primero se lee el archivo entero, y el número se comprueba después.
Cuando el texto no se puede leer en absoluto, el lector busca el número
al principio sin leer el resto, así que un archivo de una versión más
nueva se explica por su número de formato y no por un desconcertante
campo desconocido ([Guardar](../program/saving.md#el-número-de-versión)).

## Guardar con seguridad

`file::save` escribe en un archivo temporal junto al destino, terminado
en `.lineup.tmp`, y luego lo renombra. Un cierre inesperado o un disco
lleno dejan, por tanto, el proyecto anterior como estaba, y un archivo a
medio escribir nunca se confunde con uno guardado; si el guardado falla,
el archivo temporal se elimina. Los finales de línea son siempre `\n`,
sea cual sea el sistema, y una marca de orden de bytes al principio de
un archivo se ignora al leer ([Guardar sin perder el archivo
viejo](../program/saving.md#guardar-sin-perder-el-archivo-viejo)).
