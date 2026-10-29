# El solista (experimental)

La banda acompaña: toca los acordes para que otra persona cante la
melodía o improvise sobre ellos. El **solista** es esa otra persona,
improvisando una línea sobre la hoja de acordes. Es experimental y está
apagado hasta que lo enciendes, porque un solo hecho con reglas es un
estudio, no una interpretación. Pero es un estudio útil: cada nota que
toca tiene un motivo, y el piano roll muestra el motivo, así que puedes
ver cómo se construye una línea improvisada y probar las mismas reglas
en tu propio instrumento.

## Probarlo

1. Pulsa **Ctrl+N** y escribe estos compases, un acorde por compás:

   ```text
   Rem7   Sol7   Domaj7   La7   Rem7   Sol7   Do6   Do6
   ```

2. En la Biblioteca, aplica **Swing**.
3. Abre **🎷 Banda** y señala **Solista (experimental)**: la ayuda dice
   quién es el solista en este estilo, un saxo tenor. Márcalo. Una pista
   **Solista** entra en la lista de reproducción, y un canal Solista en
   el rack de canales.
4. Pulsa **Espacio**. Sobre la banda, el saxo toca frases de dos
   compases en corcheas con swing, con una respiración entre ellas.

**Otro solo**, en el mismo menú, toca uno distinto; **Ctrl+Z** trae de
vuelta el anterior. Desmarca el solista y su pista y su canal se van.
Como toda parte de la banda, el solo se vuelve a tocar cuando cambia la
hoja, y si editas sus notas queda como lo dejaste hasta que haces clic
en **Volver a tocarla** u **Otro solo**.

## Leer el solo

Haz doble clic en el clip del solista para abrir su parte en el piano
roll. Sus notas tienen colores según lo que son:

- **verde** para las notas del acorde, marcadas con cuál son: **R**
  para la fundamental, **3**, **5**, **7**, y **6** o **9** cuando el
  acorde las tiene;
- **azul** para las demás notas de la escala del acorde: **p** una nota
  de paso, **n** una bordadura, **s** cualquier otra;
- **naranja** para las notas cromáticas, de fuera de la escala: **a**
  una aproximación, **cp** una nota de paso cromática.

Señala una nota, o toca la canción, y la línea de arriba de la
cuadrícula dice qué es, en una frase, y qué escala toca el acorde.

En este solo, el compás 3 termina con Si3, la séptima de Domaj7, en el
pulso 4, luego Si♭3 en el «y», luego La3 en el pulso 1 del compás 4, la
fundamental de La7: tres notas a medio tono una de otra, bajando hacia
el acorde siguiente. El Si♭ no pertenece a ninguno de los dos acordes,
y no pasa nada: cae en la mitad débil del pulso, y lleva a alguna parte.

![La parte del solista en el piano roll: notas en verde, azul y naranja, cada una marcada R, 3, 5, 7, p, n, a o cp, y sobre la cuadrícula una frase que explica la nota bajo el puntero y la escala sobre su acorde](images/soloist.png)

## Cómo se construye una línea improvisada

### Notas del acorde en el pulso

Pregúntale a un profesor de jazz cómo empezar a improvisar sobre
acordes, y la primera regla es casi siempre la misma: **toca notas del
acorde en los pulsos fuertes, y entre ellas cualquier cosa que las
una**. En el pulso, el oído compara la melodía con la armonía; entre
los pulsos, la melodía está en movimiento y se le perdona más. Una línea
que cae en una nota del acorde en cada pulso «marca los cambios»: se
oyen los acordes en ella aun sin la banda.

Las notas del acorde que más dicen son la **tercera** y la **séptima**,
las **notas guía**. Dicen si un acorde es mayor o menor y qué clase de
séptima tiene, y en un ii–V–I se mueven por semitonos: la séptima de
Rem7 (Do) baja a la tercera de Sol7 (Si), y la séptima de Sol7 (Fa)
baja a la tercera de Domaj7 (Mi).

El solista elige, para cada pulso, la nota del acorde más cercana a
donde va. La línea mantiene su dirección hasta que gira en el borde del
registro del instrumento, o de vez en cuando por su cuenta, y cada tanto
salta por encima de una nota del acorde a la siguiente. Así se mueve
sobre todo por pasos pequeños, como las melodías, con algún salto aquí y
allá.

### Escalas de acorde

Entre las notas del acorde van otras notas, y cuáles dependen del
acorde: sobre Rem7, ¿la nota entre La y Do es Si o Si♭? La enseñanza
del jazz responde con las **escalas de acorde**: cada acorde recibe una
escala de siete notas que lo contiene, y las notas de en medio salen de
esa escala. La escala sale del acorde **en su tonalidad**. Sobre Rem7 en
Do mayor el Si es natural, porque la música está en Do: la escala es Do
mayor desde Re, llamada **Re dórico**. Los modos de la tonalidad son lo
que tocan sus acordes:

| Acorde en Do mayor | Su escala | Notas |
|---|---|---|
| Domaj7 (I) | Do jónico | Do Re Mi Fa Sol La Si |
| Rem7 (ii) | Re dórico | Re Mi Fa Sol La Si Do |
| Mim7 (iii) | Mi frigio | Mi Fa Sol La Si Do Re |
| Famaj7 (IV) | Fa lidio | Fa Sol La Si Do Re Mi |
| Sol7 (V) | Sol mixolidio | Sol La Si Do Re Mi Fa |
| Lam7 (vi) | La eolio | La Si Do Re Mi Fa Sol |
| Sim7♭5 (vii) | Si locrio | Si Do Re Mi Fa Sol La |

Siete nombres para las mismas siete notas. Los nombres importan porque
dicen qué nota es la fundamental y dónde caen los semitonos a partir de
ella: eso es lo que le da a cada acorde su color en la tonalidad (ver
[Tonalidades y modos](keys-and-modes.md)).

Los demás acordes reciben su escala como la elegiría un músico que
piensa en la tonalidad:

1. **Un acorde de la tonalidad** toca la escala de la tonalidad desde
   su fundamental, como en la tabla. En una tonalidad menor, un acorde
   con la séptima elevada (Mi7 en La menor, con Sol♯) toca la **menor
   armónica** desde su fundamental: Mi Fa Sol♯ La Si Do Re, llamada **Mi
   frigio dominante**.
2. **Un acorde prestado de la tonalidad menor de la misma nota** toca la
   escala de esa tonalidad: Fam en Do mayor viene de Do menor, así que
   toca Fa dórico.
3. **Una dominante secundaria** toca la escala del acorde al que lleva:
   La7 en Do mayor es el V7 de Rem, así que toca Re menor armónica desde
   La, La Si♭ Do♯ Re Mi Fa Sol. Por eso un solo sobre La7 tiene aquí un
   Si♭: apunta al acorde de Re menor que sigue.
4. **Cualquier otro** toca la escala que suele recibir su tipo: **lidia**
   para un acorde mayor de fuera de la tonalidad, **lidia dominante**
   para una séptima de dominante que no lleva a ninguna parte,
   **disminuida semitono-tono** para 7♭9 y 7♯9, **disminuida
   tono-semitono** para una séptima disminuida, **de tonos enteros** para
   un acorde aumentado, **locria** para uno semidisminuido y **dórica**
   para un acorde menor.

Y las notas propias del acorde siempre ganan: Mi9 en Do mayor parte de
La menor armónica, pero su novena es Fa♯, así que Fa♯ reemplaza a Fa en
su escala.

[Acordes de fuera de la tonalidad](outside-the-key.md) explica los
acordes prestados y las dominantes secundarias.

### Notas entre los pulsos

Cada tipo de nota entre las notas del acorde tiene un nombre, y la
mayoría son más antiguos que el jazz; los libros de armonía las llaman
**notas extrañas** (o notas de adorno):

- una **nota de paso** llena el hueco entre dos notas del acorde a una
  tercera, por grado conjunto, en la misma dirección: Do **Re** Mi sobre
  Do;
- una **bordadura** se aparta un grado de una nota del acorde y vuelve:
  Mi **Fa** Mi;
- una **nota de paso cromática** llena un tono con el semitono de en
  medio: Re **Re♯** Mi;
- una **aproximación cromática** llega a una nota del acorde desde un
  semitono abajo o arriba: **Fa♯** Sol. Una frase que empieza justo
  antes del pulso, una **anacrusa**, suele empezar con una.

Las notas de paso cromáticas en el «y» del pulso son parte del sonido
del **bebop**, el jazz de Charlie Parker y Dizzy Gillespie: un semitono
añadido entre dos notas de la escala mantiene las notas del acorde en
los pulsos cuando una línea corre en corcheas.

### Frases y respiración

Cantantes y vientistas tienen que respirar, así que las líneas vienen
en **frases**. El solista toca frases de dos compases, cada una
empezando en el pulso, un pulso tarde o con una anacrusa, y deja un
pulso o dos vacíos al final. Los silencios son lo que lo hace sonar como
alguien que habla. Sus corcheas tienen el swing de las de la banda.

El solista toca sobre la vuelta, no sobre la introducción ni el final
(ver [Forma de la canción y secciones](song-form.md)), y sigue tocando
durante los golpes y los silencios (ver [Golpes, sostenidos, silencios
y anticipaciones](stops.md)): para eso se detiene una banda. Sobre
**N.C.** calla. Sobre un conjunto de clases de alturas, un micromodo o
una forma de la serie toca las notas propias del conjunto en los
pulsos, con notas cromáticas entre ellas.

## Quién toca

| Estilos | Solista | Registro |
|---|---|---|
| Pop | saxo alto | Sol3 a Sol5 |
| Rock | guitarra saturada | La3 a Do6 |
| Blues | armónica | Do4 a Do6 |
| Jazz | saxo tenor | Re3 a Fa5 |
| Soul, funk y R&B | saxo alto | Sol3 a Sol5 |
| Hip hop y lo-fi | vibráfono | Do4 a Do6 |
| Electrónica | lead de diente de sierra | Do4 a Do6 |
| Latina | trompeta | Si♭3 a La5 |
| Tango | acordeón de tango | Sol3 a Do6 |
| Folclore latinoamericano | flauta de pan | Mi4 a Mi6 |
| Flamenco | guitarra de nailon | Mi3 a La5 |
| Reggae y ska | trompeta | Si♭3 a La5 |
| Country y folk | violín folk | Sol3 a Mi6 |
| Clásica y de cine | flauta | Fa4 a Fa6 |

El acordeón de tango es lo que General MIDI pone en lugar del
bandoneón. Los registros son el centro cómodo de cada instrumento,
donde canta. Haz clic en la línea **Solista** del menú Banda para darle
otro sonido.
