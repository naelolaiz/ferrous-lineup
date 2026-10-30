# Abrir archivos MIDI y grabaciones

Ferrous Lineup puede abrir música hecha en otra parte: un **archivo
MIDI** de una obra, una **hoja de acordes** guardada como texto o, como
experimento, una **grabación**. Cada uno se vuelve una canción nueva,
con sus acordes en la hoja de acordes y su tonalidad en las teclas de
armonía, para que la estudies como una demostración. Si la canción
abierta tiene cambios sin guardar, primero se te pregunta.

## Archivos MIDI

Un archivo MIDI tiene las notas de una obra, no su sonido: qué nota
empieza cuándo, cuánto dura, con qué fuerza se toca y qué instrumento
debería tocarla. Muchas obras se encuentran como archivos MIDI, y la
mayoría de los programas de música pueden escribir uno.

**Archivo > Importar archivo MIDI…** abre uno (.mid, .midi, .kar o
.rmi). También puedes soltarlo sobre la ventana, o abrirlo con Ferrous
Lineup desde tu gestor de archivos. La canción que resulta:

- tiene un **canal para cada parte** del archivo, con el nombre de su
  pista o de su instrumento, con el instrumento que pide el archivo, y
  su volumen. La batería se vuelve un canal por tambor, como en el rack
  de canales de una canción nueva;
- conserva **cada nota** como se tocó: con qué fuerza, y exactamente
  cuándo, aunque caiga entre dos pasos de la cuadrícula;
- se corta en las barras de compás en **patrones**, colocados uno tras
  otro en la lista de reproducción; un pasaje que vuelve nota por nota
  toca otra vez el mismo patrón;
- toma los **cambios de tempo**, los **compases** y las **armaduras**
  del archivo. Un archivo sin tonalidad, o uno que dice Do mayor cuando
  sus notas están claramente en otra (muchos programas escriben Do
  mayor cuando nadie eligió), recibe la tonalidad a la que mejor se
  ajustan sus notas;
- recibe en su **hoja de acordes** los acordes que la aplicación oye en
  él.

La barra de estado lo resume, como en *canon-in-d: 3 canales, 77
notas, 9 compases, 4/4, 72 BPM, Re mayor*:

![El Canon de Pachelbel abierto desde un archivo MIDI: la hoja de acordes delante con los acordes que se oyen en él, Re La Sim Fa#m Sol Re Sol La, con sus números I V vi iii IV I IV V, la tonalidad Re mayor y 72 en el visor de tempo, el patrón Compases 1–9 en la barra de herramientas, y la barra de estado que dice «canon-in-d: 3 canales, 77 notas, 9 compases, 4/4, 72 BPM, Re mayor»](images/import-midi.png)

Desde ahí es una canción como cualquier otra: tócala, bájale la
velocidad, mira cada parte en el piano roll, cambia sus sonidos en el
mezclador, prueba sobre ella un estilo de la Biblioteca, y guárdala
como archivo .lineup.

### Corregir los acordes

La aplicación oye los acordes como los oiría un músico a lo largo de un
tramo de música, no nota por nota, y acierta la mayoría. Donde una obra
se mueve rápido, o su armonía es poco común, revisa lo que oyó: haz
clic en un compás y escribe el acorde correcto (ver [La hoja de
acordes](chord-sheet.md)).

## Hojas de acordes en texto

**Archivo > Importar hoja de acordes…** abre un esquema de acordes
guardado como texto, como `| C | Am7 | Dm7 G7 |`, y la banda lo toca.
Ese texto es lo que escribe **Archivo > Exportar hoja de acordes…**, y
lo que se encuentra en cancioneros y en sitios web. El formato está en
[La hoja de acordes](chord-sheet.md#hojas-de-acordes-como-texto).

## Grabaciones (experimental)

**Archivo > Importar grabación…** abre un archivo de sonido (MP3, WAV,
FLAC u Ogg) e intenta tocar junto con él. También puedes soltar el
archivo sobre la ventana, o abrirlo con Ferrous Lineup desde tu gestor
de archivos. (Un archivo WAV de diez segundos o menos soltado sobre la
ventana se toma como un sonido para un canal: ver [El rack de
canales](channel-rack.md#tus-propios-sonidos).) Escucha durante unos segundos
(*Escuchando …* en la barra de estado), y luego arma una canción con dos
pistas:

- **Grabación**: un canal que toca toda la grabación;
- **Armonía**: un acorde en cada pulso, con los acordes que oyó.

También adivina el tempo, el compás, la tonalidad y los cambios de
tonalidad, para que las barras de compás, la hoja de acordes y las
teclas de armonía sigan la música. La tonalidad de la canción es
aquella en la que la grabación se queda más tiempo. La grabación suena
desde donde empieces: desde el principio, desde un compás en el que
hagas clic, o después de un salto. La barra de estado dice lo que
encontró, como en *canon-in-d: 63.9 BPM, 4/4, Re mayor, acordes en 43 de
43 pulsos*.

Oír una grabación completa es mucho más difícil que leer un archivo
MIDI, y el resultado suele estar mal: el tempo puede salir a la mitad
o al doble, y se pueden oír acordes donde solo pasa una nota de la
melodía. Tómalo como un punto de partida para corregir, y confía en tu
oído más que en él.

La grabación se guarda de nuevo como un archivo WAV junto al original,
con un nombre como *cancion.ferrous-lineup.wav* (o en la carpeta
temporal del sistema si no se puede escribir en esa carpeta), con un
poco de silencio delante para que su primer pulso fuerte caiga en una
barra de compás. La canción toca ese archivo y solo dice dónde está,
así que guárdalo junto con la canción.
