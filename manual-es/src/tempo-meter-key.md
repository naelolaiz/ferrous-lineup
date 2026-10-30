# Tempo, compases y cambios de tonalidad

Qué tan rápido va una canción, cómo se cuentan sus pulsos y en qué
tonalidad está se pueden fijar para toda la canción, y cambiar a mitad
de camino, como los cambia una partitura.

![La franja de tempo de la canción inicial con un cambio en el compás 7 marcado «rit. → 90», y su menú abierto: Tempo desde el compás 7, ♩ = 90.0 BPM, Cómo llegar con De golpe, a lo largo de 1 pulso, a lo largo de ½ compás, a lo largo de 1 compás, a lo largo de 2 compases (encendido), a lo largo de 4 compases y a lo largo de 8 compases, la línea «De 130 a 90 BPM. Frenar de a poco es un ritardando (rit.).» y Quitar este cambio (volver a 130 BPM)](images/tempo.png)

## El tempo

El **tempo** es la velocidad de los pulsos, en **pulsos por minuto**
(BPM): a 60 BPM un pulso dura un segundo, a 120 medio segundo. Las
baladas lentas andan entre 60 y 80, la mayoría del pop entre 90 y 130,
la música de baile entre 120 y 130, y el swing rápido o el punk bastante
por encima de 160.

El cuadro **TEMPO** de la barra de herramientas es el tempo con el que
empieza la canción: arrastra el número hacia arriba o hacia abajo, o haz
doble clic y escribe uno. La hoja de acordes también lo muestra, junto
a la tonalidad. Para encontrar el tempo de una canción que oyes, haz
clic en **Pulsar** de la hoja de acordes en cada pulso (ver
[Practicar](practising.md)).

### Cambiar el tempo a mitad de camino

La franja **Tempo**, sobre los números de compás de la lista de
reproducción, dibuja el tempo como una línea: plana mientras no cambia,
un salto donde cambia de golpe, una pendiente donde cambia de a poco.
Haz clic en ella sobre un compás para abrir su menú:

- **♩ =** el tempo desde ahí: arrástralo, o haz clic y escribe uno. Un
  tempo nuevo añade un cambio en ese compás.
- **Cómo llegar**: **De golpe**, como después de una pausa o al empezar
  una sección, o de a poco, **a lo largo de 1 pulso**, **a lo largo de ½
  compás**, **a lo largo de 1 compás**, **a lo largo de 2 compases**,
  **a lo largo de 4 compases** o **a lo largo de 8 compases**.
- **Quitar este cambio** lo saca.

En el compás 1, el menú fija el tempo con el que empieza la canción.

La franja rotula cada cambio como lo hace una partitura: «♩ = 90» para
uno repentino, «rit. → 60» para un frenado gradual, un **ritardando**,
y «accel. → 140» para una aceleración gradual, un **accelerando**. Las
canciones suelen frenar justo al final, y un ritardando a lo largo de
los dos últimos compases hace que un final suene a final. Al tocar, y al
exportar un WAV o un archivo MIDI, se siguen los cambios.

## Los compases

Una **indicación de compás** dice cómo se cuentan los pulsos. El número
de arriba dice cuántas notas forman un compás, el de abajo qué nota:
**4** para negras, **8** para corcheas, **2** para blancas. 4/4 son
cuatro negras por compás; 3/4, tres, el vals; 6/8, seis corcheas
sentidas como dos pulsos de tres. [Ritmo y métrica](rhythm-and-meter.md)
los explica, y también los irregulares como 5/4 y 7/8.

### El compás de la canción

**Compás**, en las teclas de armonía, es la indicación de compás de la
canción, con cuántos pasos tiene un compás al lado. Ábrelo para elegir
otra:

- un clic para las habituales: **2/4**, **3/4**, **4/4**, **5/4**,
  **3/8**, **5/8**, **6/8**, **7/8**, **9/8** y **12/8**;
- u **Otro**: el número de arriba, el de abajo (**1**, **2**, **4**,
  **8** o **16**), y **Pulsos agrupados como**, para las métricas cuyos
  pulsos son grupos de dos y tres notas, como 7/8 contado **2+2+3**.
  **Usar 7/8** la aplica.

Un compás nuevo para toda la canción pone las notas de cada compás en un
compás del tipo nuevo, y **cada compás dura lo mismo que antes**: una
canción a 130 BPM en 4/4 suena a 97,5 en 3/4, tres pulsos en el tiempo
que llevaban cuatro. Los cambios de tempo se mueven con ella. Las
opciones de arriba de la lista, compartidas con la Biblioteca, dicen qué
más entra:

- **Batería**: la batería del estilo de la Biblioteca para esa métrica,
  como un vals para 3/4. Apagado, la canción conserva su propia batería,
  en la nueva métrica.
- **Partes de acordes y bajo**: cómo toca ese estilo los acordes y el
  bajo. Apagado, la canción conserva las suyas, en la nueva métrica.
- **Usar los instrumentos del estilo** y **Usar el swing del ritmo**:
  los sonidos del estilo y su swing.

La barra de estado dice lo que se hizo, como en «La canción toca ahora
Vals en 3/4, cada compás comprimido o estirado en un compás de 3/4».
Volver al primer compás de la canción devuelve exactamente la canción
que tenías, y **Ctrl+Z** lo deshace de una vez.

### Cambiar de compás a mitad de camino

Haz clic derecho en un número de compás de la lista de reproducción:
bajo **Compás desde el compás 5** está la misma elección, para ese
compás y los que siguen. Los compases anteriores quedan como están, y
también el estilo y los instrumentos. **Quitar este cambio** vuelve a
la indicación de compás anterior. Los números de compás muestran la
indicación de compás en el compás 1 y donde cambia, y la hoja de acordes
la escribe al principio del compás.

## Cambios de tonalidad

Elegir otra tonalidad en los menús de **Tonalidad** mueve toda la
canción a ella, y un **cambio de tonalidad** a mitad de camino nombra
una tonalidad nueva desde un compás, sin mover ninguna nota: los dos
están en [Tonalidades y
modos](keys-and-modes.md#cambios-de-tonalidad-en-una-canción). El mismo
menú de clic derecho de los números de compás tiene **Tonalidad desde
el compás 5**, y los números de compás escriben la tonalidad en azul
donde cambia.

## Insertar y eliminar compases

El menú de los números de compás también tiene **Insertar un compás
antes**, **Insertar un compás después** y **Eliminar este compás**, que
mueven todo lo que viene después, como en la hoja de acordes (ver [La
hoja de acordes](chord-sheet.md#cambiar-los-compases)).
