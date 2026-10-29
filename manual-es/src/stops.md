# Golpes, sostenidos, silencios y anticipaciones

Un estilo toca el mismo ritmo compás tras compás, y eso es lo que lo
hace un estilo. Pero escucha a una banda de verdad: al final de una
frase todos atacan un acorde juntos y se paran en seco, o sostienen un
acorde debajo de quien canta, o se callan un compás para que el
baterista o el solista queden solos, o entran con el acorde siguiente un
poco antes del pulso. Los arregladores escriben estos momentos en el
esquema, y son lo que hace que una banda suene arreglada y no a un bucle.
La hoja de acordes tiene una marca para cada uno.

![Una hoja de acordes con una anticipación en Fa, un golpe en Sol7, un sostenido en Lam, un silencio en Sol7 y una anticipación de una semicorchea en Do, cada uno con su palabra bajo el acorde, y abierto el menú del clic derecho Banda en Lam con Según el estilo, Golpe, Sostenido, Silencio, En el pulso, Anticipación y Anticipación de una semicorchea](images/stops.png)

## Escribir las marcas

Los puntos después de una armonía detienen a la banda; los acentos
circunflejos antes de ella la hacen entrar antes:

| Escribe | Marca | Qué hace la banda |
|---|---|---|
| `Do.` | **golpe** | ataca el acorde una vez, corto, y calla hasta la armonía siguiente |
| `Do..` | **sostenido** | lo ataca y lo sostiene hasta la armonía siguiente |
| `Do...` | **silencio** | calla hasta la armonía siguiente |
| `^Do` | **anticipación** | lo toca una corchea antes, y lo sostiene por encima del pulso |
| `^^Do` | **anticipación** | lo toca una semicorchea antes |

Una anticipación se puede juntar con una parada: `^Sol7.` es un golpe
una corchea antes. Debajo de cada armonía marcada, una palabra dice qué
hace ahí la banda (**golpe**, **sostenido**, **silencio**,
**anticipación**); señálala para leer la frase entera. `N.C.` conserva
sus propios puntos: son parte de su nombre.

Pruébalo: pulsa **Ctrl+N** y escribe estos compases, uno por compás:

```text
Do   ^Fa   Sol7.   Lam..   Fa Sol7...   ^^Do   Fa Sol7   Do
```

- compás 2: la banda toca Fa en el «y» del pulso 4 del compás 1, una
  corchea antes, y lo deja sonar por encima de la barra de compás;
- compás 3: toda la banda ataca Sol7 una vez, con un bombo y un crash,
  y luego nada hasta el compás 4;
- compás 4: Lam atacado y sostenido todo el compás;
- compás 5: Fa en el estilo durante dos pulsos, luego silencio donde
  está Sol7;
- compás 6: Do una semicorchea antes;
- compases 7 y 8: otra vez el estilo.

## Ponerlas desde un menú

Haz clic derecho en un compás y abre **Banda en Sol7** (hay uno por cada
armonía del compás) para elegir **Golpe (.)**, **Sostenido (..)** o
**Silencio (...)**, y **Anticipación (^)** o **Anticipación de una
semicorchea (^^)**. **Según el estilo** y **En el pulso** quitan las
marcas. Como con cualquier cambio en la hoja, la banda vuelve a tocar,
**Ctrl+Z** lo deshace, y las marcas se copian con los compases, también
como texto.

## La música

**Golpes.** Un golpe (en inglés *shot*, *hit*, *stab* o *kick*) es un
acorde que toda la banda toca a la vez, corto y acentuado, seguido de
silencio. Los arreglos de big band están llenos de ellos, y el baterista
los toca con la banda en el platillo crash y el bombo. Una serie de
golpes, uno al principio de cada compás o cada dos, se llama
**stop-time**: la banda toca solo los golpes y deja el resto del compás
a quien canta, al solista o a un bailarín de claqué. Las estrofas de
*Hoochie Coochie Man*, de Muddy Waters, son un stop-time famoso. En
Ferrous Lineup cada músico toca el golpe a su manera, el piano su
acorde, el bajo su nota, la guitarra un rasgueo, un poco más fuerte que
el estilo, y la batería toca un bombo y un crash. La armonía sigue ahí
para quien toque sobre el silencio.

**Sostenidos.** Un sostenido es un acorde atacado y mantenido: al final
de una frase, debajo de la última palabra de un verso, o para dejar que
un solista toque libre sobre una armonía. No es un calderón: un
calderón detiene el pulso, y la banda espera al director, mientras que
un sostenido dura exactamente hasta la armonía siguiente, y el pulso
sigue por debajo.

**Silencios.** Un silencio es la banda que deja de tocar, batería
incluida. Los arregladores lo llaman **break** (corte): en el jazz la
banda se detiene un compás o dos y el solista toca solo, y en el funk y
el hip hop, los compases donde tocaba solo el baterista le dieron su
nombre al *breakbeat*. Escribe tú el break de la batería en el piano
roll, o tócalo con **Tocas tú** (ver [Bloquear una pista y tocar tu
parte](locks.md)).

**Anticipaciones.** Una anticipación es un acorde tocado antes del
pulso al que pertenece y sostenido por encima de él. Sin ella, el primer
pulso es el momento más fuerte del compás; con ella, el acorde llega
antes y el pulso 1 queda en silencio, y la música se inclina hacia
adelante. Eso es una **síncopa**, un acento fuera del pulso, y las
anticipaciones están por todas partes en el pop, el rock, el funk y la
música latina: en el **tumbao** de la salsa, el bajo toca la
fundamental del acorde siguiente en el pulso 4, sostenida por encima de
la barra de compás. Una anticipación de una semicorchea es más sutil,
el empuje del funk y de la música latina rápida.

El acento circunflejo se parece al **marcato**, el acento que los
músicos escriben sobre una nota para tocarla fuerte, y una anticipación
se toca con acento.
