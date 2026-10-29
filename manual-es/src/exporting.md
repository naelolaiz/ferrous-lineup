# Exportar

Para llevar una canción fuera de Ferrous Lineup, el menú **Archivo** la
escribe como archivo de sonido, como video, como archivo MIDI o como
hoja de acordes en texto. Una canción sin nada en la lista de
reproducción no tiene nada que exportar: los elementos aparecen
atenuados, con **Coloca primero algunos clips en la lista de
reproducción**.

| Elemento | Escribe |
|---|---|
| **Exportar WAV…** | el sonido de la canción, para escucharlo en cualquier parte |
| **Exportar video…** | el sonido, con la hoja de acordes u otras vistas acompañándolo |
| **Exportar canción como MIDI…** | las notas de toda la canción, para otro programa de música |
| **Exportar patrón como MIDI…** | las notas del patrón seleccionado |
| **Exportar hoja de acordes…** | los acordes como texto, para compartirlos o pegarlos en cualquier parte |

Cada uno pregunta dónde guardar, y añade la terminación del archivo si
no la escribes.

## Un archivo de sonido

**Exportar WAV…** guarda lo que oyes cuando suena la canción, de
principio a fin, como un archivo WAV: calidad de CD (44,1 kHz, 16 bits,
estéreo), con cada repetición, la banda, los volúmenes, balances,
reverberación y efectos del mezclador, y un par de segundos al final
para que se apaguen las últimas notas y la sala. Siempre va al tempo
propio de la canción, aunque practiques a una velocidad más lenta.

Se hace en segundo plano, más rápido de lo que dura la canción, mientras
sigues trabajando: la barra de estado dice **Exportando** y el nombre
del archivo, con un botón **Detener**. Cualquier reproductor de música
toca el WAV, y cualquier programa de audio puede convertirlo en MP3.

## Un video

**Exportar video…** hace un video MP4 de la canción, con vistas de la
aplicación que siguen la música, listo para publicar o para mostrar en
una clase:

![La ventana Exportar video para la demostración El Choclo: Hoja de acordes y Teclas de armonía marcadas, cada una con su zoom al 100%, y la vista previa de la imagen debajo: la línea de título «El Choclo», «Compás 1, pulso 1», la tonalidad La menor armónica, 4/4 y 112 BPM, la hoja de acordes en el recuadro de arriba y las teclas de armonía en el de abajo; luego Apilar, Vista previa en, los tamaños 1280 × 720 y 1920 × 1080, y Exportar…](images/export-video.png)

- **Las vistas.** Marca las que quieras mostrar: **Hoja de acordes**
  (con el compás que suena enmarcado), **Partitura guía** (la melodía
  en un pentagrama con sus acordes), **Lista de reproducción**, **Piano
  roll** y **Teclas de armonía** (con el acorde que suena encendido).
  Cada una tiene un **zoom**, de 50% a 250%; la partitura guía, en
  cambio, crece con su recuadro.
- **La imagen.** La vista previa muestra un cuadro. Arrastra una vista
  para moverla, arrastra su esquina para agrandarla o achicarla, y gira
  la rueda sobre ella para hacer zoom. **Apilar** vuelve a poner las
  vistas una encima de la otra. **Vista previa en** muestra la imagen
  en otro lugar de la canción.
- **Tamaño**: **1280 × 720** o **1920 × 1080**.

Una línea arriba nombra la canción, y muestra el número de compás y el
pulso, la tonalidad, el compás y el tempo a medida que cambian. El video es un
archivo MP4, con imágenes H.264, 30 por segundo, y sonido MP3, que se
reproduce casi en cualquier parte; no hace falta instalar nada más para
hacerlo. La ventana recuerda tus elecciones para la próxima vez.

## Un archivo MIDI

**Exportar canción como MIDI…** escribe las notas de la canción, para
otro programa de música, un programa de notación o un sintetizador:

- cada canal recibe una pista propia, con su instrumento General MIDI,
  así que el archivo se abre con los sonidos correctos;
- la batería va en el canal 10, como batería General MIDI; un canal que
  envías por MIDI conserva su canal, y los demás toman 1, 2, 3, etc.;
- cada repetición se escribe completa, con el swing de la canción en
  las notas, y el tempo, el compás y la tonalidad se escriben a medida
  que cambian;
- las pistas y los canales silenciados, y los canales callados por un
  solo, quedan fuera, como cuando suena la canción. Los volúmenes,
  balances y efectos del mezclador no se escriben: el programa que abre
  el archivo hace su propia mezcla.

**Exportar patrón como MIDI…** escribe solo el patrón seleccionado,
desde su primer paso: un ritmo o una parte para usar en otro lugar.

Ferrous Lineup puede volver a abrir el archivo (ver [Abrir archivos
MIDI y grabaciones](importing.md)), y conserva la tonalidad exacta en
la que estaba.

## Una hoja de acordes en texto

**Exportar hoja de acordes…** escribe la hoja de acordes de la canción
como un archivo de texto, compás por compás, como en `| C | Am7 | Dm7
G7 |`. Pégala en un mensaje, imprímela, o vuelve a abrirla con
**Archivo > Importar hoja de acordes…** (ver [La hoja de
acordes](chord-sheet.md#hojas-de-acordes-como-texto)). Siempre usa los
nombres en inglés, con letras, que cualquier músico puede leer.

## Partituras guía e impresión

La **partitura guía**, la melodía en un pentagrama con sus acordes, se
imprime o se guarda como PDF desde su propia ventana, en sonido real o
para un instrumento en Si♭, Mi♭ o Fa: ver [Partituras
guía](lead-sheets.md).
