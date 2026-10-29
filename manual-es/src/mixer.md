# El mezclador

El **mezclador** (**F9**, o **Ver > Mezclador**) es donde se equilibran
las partes de una canción: qué tan fuerte suena cada una, dónde se ubica
entre los dos altavoces, cuánto de ella se oye en una sala y cómo se
moldea su sonido. Cada cambio se puede deshacer, y se guarda con la
canción.

![El mezclador de la demostración El Choclo mientras suena: los medidores encendidos en cada línea, la reverberación activada, los Teclados en I30 con EQ en su columna Efectos, la Melodía en D30, el Bajo sin reverberación (0%), y los canales agrupados en Batería, Arpa y guitarras, Teclados y Cuerdas](images/mixer.png)

## Las líneas

Cada canal tiene una línea, y las líneas se agrupan como una partitura
agrupa sus instrumentos, desde las maderas arriba hasta las cuerdas:
**Maderas**, **Metales**, **Percusión**, **Batería**, **Arpa y
guitarras**, **Teclados**, **Sintetizadores**, **Voces** y **Cuerdas**.
El orden es solo cómo los muestra el mezclador; el rack de canales
conserva el suyo.

| Columna | Qué hace |
|---|---|
| **Instrumento** | lo que toca el canal. Haz clic para elegir otro, de la misma lista que el rack de canales (ver [Sonidos](sounds.md)). Un tambor dice **Percusión incorporada**, un sonido tuyo **Muestra**. |
| **Nivel** | un medidor de qué tan fuerte está sonando el canal ahora. Se pone ámbar en la parte más fuerte, y rojo cuando llega a lo máximo que puede tocar un canal. |
| **Volumen** | qué tan fuerte suena el canal, de 0 a 100%. |
| **Balance** | dónde se ubica entre los altavoces: **C** en el centro, **I100** todo a la izquierda, **D100** todo a la derecha. Arrástralo, o haz doble clic en la casilla y escribe, como en `I30`. Doble clic en el deslizador lo devuelve al centro. |
| **Reverberación** | cuánto de él va a la sala de la reverberación (ver más abajo). Al 0% suena seco, cerca de ti. |
| **Efectos** | su propio ecualizador, compresor y delay (ver más abajo). Dice **ninguno**, o cuáles están activados: **EQ**, **comp**, **delay**. |
| **Silenciar** | lo silencia. |
| **Solo** | **S** toca solo este canal; haz clic otra vez para oírlos a todos. |

Arriba, **Principal** es el volumen de toda la canción, el mismo que el
deslizador **VOL** de la barra de herramientas, con un medidor para cada
altavoz. **Restablecer mezcla** vuelve a la mezcla de siempre: los
volúmenes habituales, todos los balances al centro, la reverberación
apagada, sin efectos, sin solo, y nada silenciado salvo lo que la propia
banda deja fuera. **Ctrl+Z** te devuelve tu mezcla.

## Ubicar a la banda

Una mezcla se oye mejor cuando cada parte tiene su propio lugar.
Algunas costumbres que funcionan:

- **Deja el bajo, el bombo y la voz en el centro.** Sostienen la
  canción, y las notas graves suenan igual desde cualquier lado.
- **Lleva los acordes y las demás partes un poco hacia los lados**,
  como se ubicarían los músicos en un escenario: un piano en I30 y una
  guitarra o un acordeón en D30 dejan el centro a la melodía.
- **Baja antes de subir.** Cuando una parte se pierde, bajar las demás
  suele funcionar mejor que subirla a ella.
- **El acompañamiento más suave que la melodía.** La banda está para
  sostener la melodía: debe oírse, no taparla.

## La reverberación

Una **reverberación** imita una sala: el sonido vuelve desde sus
paredes, más tarde y más suave, durante un segundo o más. Sin ella, la
música hecha por una computadora puede sonar como si se tocara dentro
de un armario.

Marca **Reverberación** para activarla. Es una sola sala para toda la
canción:

- **Húmedo / seco**: cuánta sala se oye junto al sonido mismo. Al 20%
  la sala queda en el fondo; al 50% suena tan fuerte como la música,
  como en una iglesia.
- **Decaimiento**: cuánto tiempo sigue sonando la sala, en segundos.
- **Amortiguación**: qué tan pronto se apagan las frecuencias agudas.
  Las paredes blandas y las cortinas amortiguan una sala; la piedra no.
- **Predelay**: cuánto tarda la sala en responder, en milisegundos. Un
  poco de predelay mantiene claras las notas en una sala grande.

La columna **Reverberación** de cada canal dice cuánto de él va a la
sala. La batería y las cuerdas suenan naturales con mucha; un bajo
suele sonar mejor cerca y seco, con poca o ninguna.

## Efectos

Haz clic en **Efectos** de un canal para abrir sus efectos. Moldean su
sonido antes de que se una a los demás:

- **Ecualizador**: sube o baja tres bandas de frecuencias, hasta 12 dB:
  **Graves** (por debajo de 200 Hz, el peso de un bajo o de un bombo),
  **Medios** (alrededor de 1 kHz, el cuerpo de la mayoría de los
  instrumentos) y **Agudos** (por encima de 4 kHz, el aire y el brillo
  de los platillos). Bajar unos decibelios los graves de un piano deja
  lugar al bajo. **Plano** devuelve todas las bandas a 0 dB, y un doble
  clic devuelve una.
- **Compresor**: empareja qué tan fuerte suena la parte. Por encima del
  **Umbral** baja el sonido, tanto más cuanto mayor es la **Relación**
  (a 4:1, un pico 8 dB por encima del umbral sale 2 dB por encima). La
  **Ganancia** vuelve a subir toda la parte. Con una relación de 1:1 no
  hace nada.
- **Delay**: vuelve a tocar la parte un poco más tarde, a tempo con la
  canción, como un eco. **Tiempo** es cuánto después, en figuras de la
  canción (de una semicorchea a una blanca, incluidas la corchea y la
  negra con puntillo), así que el eco sigue al tempo.
  **Realimentación** dice qué tan fuerte es cada eco junto al anterior,
  y **Mezcla** qué tan fuertes son los ecos; al 0% el delay está
  apagado.

**Sin efectos** los apaga todos.

## Lo que cambia el mezclador

El mezclador cambia cómo suena la canción, no lo que toca: las notas
quedan como están. Lo que oyes, y un WAV o un video que exportes, suenan
como dice el mezclador. Un archivo MIDI que exportes deja fuera los
canales silenciados, o callados por un solo, pero no lleva volúmenes,
balances ni efectos: el programa que lo abre hace su propia mezcla.
