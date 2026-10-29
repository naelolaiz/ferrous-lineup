# Tangos y canciones folclóricas a partir de archivos MIDI

Archivos: [`crates/lineup-core/src/library/song/tango_played.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/song/tango_played.rs),
[`crates/lineup-core/src/library/song/tango_arranged.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/song/tango_arranged.rs),
[`crates/lineup-core/src/library/song/folklore_arranged.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/song/folklore_arranged.rs)
y [`crates/lineup-core/src/library/song.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/song.rs)

Un montón de archivos MIDI de tangos y canciones folclóricas argentinas
se convirtió en 28 demos nuevas, cada una completa tal como la toca su
archivo, nota por nota. Los archivos venían en tres clases, y cada
clase necesitó su propia manera de encontrar el pulso:

| Módulo | Demos | Cómo se hizo el archivo |
|---|---|---|
| `tango_arranged` | Mi noche triste, Sentimiento gaucho, Zorro gris | un arreglo de banda, secuenciado con clic |
| `folklore_arranged` | Cinco siglos igual, La cultura es la sonrisa, El Noi de la Mare, Zamba de mi esperanza, Zamba por vos, Que nadie sepa mi sufrir | un arreglo de banda, secuenciado con clic |
| `folklore_arranged` | Balderrama, Acuarela del río | un arreglo de banda con pulso constante que no cae en la cuadrícula propia del archivo |
| `tango_played` | Uno, Sur, Soledad, Nostalgias, Pasional, Patotero sentimental, Qué falta que me hacés, Qué me van a hablar de amor, Marionetas, Tiempos viejos, Yira yira, Los mareados, Mano a mano, La casita de mis viejos, Lluvia de estrellas, La pulpera de Santa Lucía, Malena | un pianista tocando libremente, sin clic |

La melodía, la línea que canta el cantante, está siempre en el canal
**Melodía**. En los arreglos de banda, cada instrumento restante tiene un
canal propio, con su programa de General MIDI, y el bombo o la
percusión toca en la batería. En el piano, todo lo que hay bajo la
melodía es la mano izquierda, en los Teclados.

Las hojas de acordes se leyeron desde las notas con un script. Para once de
las canciones de piano, la lectura se comprobó después contra una partitura guía, y esas canciones están cortadas en sus estrofas y estribillos,
cada una en su propia tonalidad. Las otras seis, y los arreglos de
banda, siguen siendo una lectura solo de las notas, así que
trata sus acordes como un borrador.

## Probarlo

1. Abre **Archivo > Demostraciones**, escribe `Uno` y elige **Uno**. Pulsa ▶ y
   mira el tempo en el transporte: cambia casi cada compás, entre 78 y
   98 por negra, porque el pianista se frena y se apura con la letra.
   Eso es **rubato**, tiempo «robado».
2. Abre **Malena** y el **mezclador**. Melodía es la melodía del pianista;
   Bandoneón, Guitarra, Contrabajo y Cuerdas son los otros instrumentos
   del arreglo, cada uno en una franja propia.
3. Abre **Balderrama** y mira la tonalidad en la barra de armonía:
   **Mi menor**. Después abre el piano roll del canal Melodía en los
   compases 21 y 22 y encuentra los do sostenidos.
4. Abre **Acuarela del río**. Pon en solo **Contrabajo**, luego
   **Teclados**, luego la batería, y escucha dónde golpea cada uno en el
   compás.
5. Abre **Zamba de mi esperanza** y silencia **Melodía**. El canal
   **Segunda voz** canta la nota grave allí donde la melodía se canta
   a dos voces.
6. Abre **Yira, yira** y mira pasar las partes en la lista de reproducción: Estrofa
   1a, Estrofa 1b, Estribillo 1, y otra vez la canción. La barra de armonía
   muestra **Si menor** en las estrofas y **Re mayor** en los
   estribillos.

## La música

### Tango en 2/4, escrito dos compases en uno

La mayoría de estos tangos están escritos en **2/4**: dos pulsos por
compás. El estilo Tango de la app está en 4/4, así que aquí cada par de
compases de 2/4 es un compás de 4/4. Nada se mueve: los pulsos están
donde estaban, y una de cada dos líneas de compás de la partitura cae
en mitad de un compás de la demo.

### El vals criollo: el 3/8 se convierte en 3/4

**La pulpera de Santa Lucía** es un **vals criollo**, el vals argentino,
rápido y ligero. Su archivo está en **3/8**: tres corcheas por compás.
Aquí sus corcheas son las negras de un compás de 3/4. La música es la
misma, leída con una nota más larga como pulso, así que el tempo es de
unas 196 por negra: tres pulsos rápidos por compás, sentidos como uno.

### Seis corcheas, dos maneras: la zamba y la polca

Un compás de seis corcheas se puede agrupar de dos maneras:

```text
6/8:  1 2 3 4 5 6     dos pulsos:  ONE . . TWO . .
3/4:  1 2 3 4 5 6     tres pulsos: ONE . TWO . THREE .
```

La música de Argentina y Paraguay juega con ambas a la vez, o una
después de la otra. Eso se llama **sesquialtera**, «una y media»: tres
pulsos en el tiempo de dos.

La **zamba** es la danza lenta de los pañuelos del norte de Argentina,
escrita en 6/8. El archivo de **Zamba de mi esperanza** está escrito en
3/4. La demo lo escribe en 6/8, el compás propio de la zamba: los
compases tienen las mismas seis corcheas, y solo cambia la agrupación
en el papel.

**Acuarela del río** es una **polca**, la danza en 6/8 de Paraguay y el
Litoral. Escucha su banda, compás a compás:

| Instrumento | Golpes en la corchea | Agrupación |
|---|---|---|
| percusión, acentuada | 1 y 4 | 6/8: dos pulsos |
| contrabajo | 1, 3 y 5 | 3/4: tres pulsos |
| piano | 2 y 5 | los contratiempos que empujan el baile |

El bajo camina en 3/4 mientras la batería cuenta 6/8, en el mismo
compás.

### Balderrama: una nota cromática no es un modo nuevo

**Balderrama** está en Mi menor, la menor natural: Mi Fa♯ Sol La Si
**Do** Re. Sin embargo, su melodía canta do sostenido 24 veces y do
natural solo 8. ¿Está en **Mi dórico**, Mi Fa♯ Sol La Si **Do♯** Re, la
escala menor con la sexta elevada?

Mira dónde están los do sostenidos. Todos aparecen en un giro de la
melodía, en los compases 21 y 22, que vuelve cinco veces más adelante
en la canción:

```text
G A B | D C# C# B  C# B C# E
```

La melodía cae de Re a Si a través del do sostenido, y después da
vueltas alrededor de Si. Debajo, la banda toca un acorde de **Sol
mayor**: Sol, Si y Re. El do sostenido no está en ese acorde, y no está
en Mi menor: es una **nota de paso cromática**, una nota ajena a la
escala que rellena el hueco entre dos notas de ella. Los propios
acordes mantienen el do natural: donde La es la fundamental, la banda
toca La menor, el **iv** de Mi menor, nunca La mayor.

Un modo es la escala en la que vive la armonía, no las notas que una
melodía toca de paso. Una pieza en dórico tendría la sexta elevada
también en sus acordes, como hace la introducción del Credo de la Misa
Criolla con su acorde de Re mayor en La menor ([La Misa
Criolla](misa-criolla.md)). Balderrama mantiene allí su do natural, así
que está en Mi menor, con un poco de color cromático en una frase.

### Acuarela del río: una tónica, dos modos

En **Acuarela del río**, la flauta toca la melodía con do, fa y sol, y
el clarinete con do sostenido, fa sostenido y sol sostenido. Las
secciones de la flauta están en **La menor** y las del clarinete en
**La mayor**, dos veces cada una: la misma tónica, dos modos. Las
tonalidades que comparten tónica son tonalidades **homónimas**. Cada
parte de la demo lleva su propia tonalidad con `in_key`, así que la
barra de armonía sigue el cambio.

## Cómo funciona

Las demos las escribió un script en Python, con la biblioteca
[mido](https://mido.readthedocs.io/) para leer archivos MIDI. El script
no forma parte de la app: lo que escribió sí, como constantes normales
de Rust.

### Cómo es una demo

```rust
pub(super) const BALDERRAMA: Song = Song {
    name: "Balderrama",
    origin: "Gustavo Leguizamón, words by Manuel J. Castilla",
    feel: "Zamba",
    genre: Genre::Folklore,
    tonic: E,
    mode: NaturalMinor,
    time: SIX_EIGHT,
    tempo: 98.0,
    // ...
    band: Band::new(gm::TANGO_ACCORDION, gm::NYLON_GUITAR, gm::ACOUSTIC_BASS),
    style: "Zamba",
    parts: &[
        Part::new("Part 1", "-:12 | - | - | ...", "Em | Em | ...")
            .with_lines(&[
                line("Shakuhachi", 77, "-:12 | - | - | -:8 E5:1 F#5 G5 A5 | ..."),
                line("Double bass", gm::ACOUSTIC_BASS, "-:4 E1+E2:2 - E1+E2:4 | ..."),
                // ...
            ])
            .with_drums(&[(Drum::Kick, "..."), (Drum::Rim, "..."), (Drum::LowTom, "...")]),
        // ...
    ],
    // ...
};
```

`Band::new` nombra los programas de la Melodía, los Teclados y el Bajo. Es un
`const fn`, así que se ejecuta mientras se compila el programa y la
banda puede vivir dentro de una constante. `gm::TANGO_ACCORDION` es el
programa 23 de General MIDI, lo más cercano que tiene el estándar a un
bandoneón; un programa sin constante en
[`general_midi.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/general_midi.rs),
como el 77 de la shakuhachi, se escribe como un número pelado.

Una parte que cambia de tonalidad lo dice:

```rust
Part::new("Part 2", "...", "...").in_key(A, HarmonicMinor)
```

y una parte donde se mueve el tempo lleva marcas de tempo, cada una en
un paso (una semicorchea) desde el principio de la parte:

```rust
.with_tempo(&[tempo(0, 97), tempo(48, 98)])
```

`tempo(at, bpm)` también es un `const fn`, que devuelve un `TempoMark`;
`&[...]` es un slice prestado, una vista de un array que vive en la
constante.

### Encontrar el pulso de una interpretación libre

Un archivo tocado con clic tiene sus líneas de compás dentro: la
cuadrícula del secuenciador. Un pianista que toca libremente no tiene
ninguna, y la cuadrícula del archivo no significa nada. El script
encuentra el pulso como lo haría un oyente, siguiendo el «Beat tracking
by dynamic programming» de Dan Ellis (2007):

1. Una **envolvente de onsets**: por cada centésima de segundo, cuánto
   sonido nuevo empieza ahí (las notas más fuertes y más graves cuentan
   más).
2. El **periodo** más constante: el desplazamiento de tiempo en el que
   la envolvente se parece más a sí misma (su **autocorrelación**),
   cerca de un tempo probable.
3. El **camino de pulsos**: la cadena de pulsos que aterriza en la
   mayor cantidad de onsets manteniéndose cerca de ese periodo de pulso
   a pulso, encontrado por programación dinámica, como encuentra un GPS
   la ruta más barata.

Cada compás recibe después el tempo al que se tocó. Se escribe una
marca de tempo nueva allí donde el tempo se mueve medio pulso por
minuto o más, así que **Uno** tiene 61 marcas y **La pulpera de Santa
Lucía** 84.

Donde el mapa de tempo de un archivo ya sigue al intérprete, como en la
mayoría de los archivos de piano, se usan sus propios pulsos.

### Un pulso constante fuera de la cuadrícula

Balderrama y Acuarela del río parecían al principio interpretaciones
libres: sus notas no caen cerca de la cuadrícula del archivo, y el
rastreador de pulsos le dio a Balderrama un tempo que saltaba de 98 a
130 a mitad de camino. Pero el bombo contaba otra historia. Sus golpes
venían cada 293 o 294 ticks, del primer compás al último. Los archivos
se secuenciaron con un clic constante, a 98 y 130 por negra, y se
guardaron a un tempo de 120, así que los pulsos del clic miden una
fracción de tick y derivan a través de los compases propios del
archivo.

Para encontrar ese pulso con exactitud, numera los golpes por el pulso
en el que caen, `k = 0, 1, 2, ...`, y ajusta una línea recta a través
de ellos, tick contra `k`. La línea de **mínimos cuadrados** es la de
menor suma de desvíos al cuadrado, y su pendiente, la longitud del
pulso, tiene una fórmula:

```text
period = Σ (k − k̄)(t − t̄) / Σ (k − k̄)²
```

donde `k̄` y `t̄` son las medias. Para Balderrama la corchea dura
293,966 ticks, y los golpes se desvían de la línea 3,5 ticks por lo
general, menos de cuatro milésimas de segundo: una máquina, no un
intérprete.

La línea da el pulso pero no el compás: ¿cuál de las seis corcheas es
la primera? La respuesta vino del coro. De sus notas largas sostenidas,
208 de 242 empiezan en una misma corchea de las seis, y la armonía
cambia en el tiempo fuerte. Esa corchea es la primera del compás. La
melodía empieza entonces dos corcheas antes de una línea de compás, con
una **anacrusa**, un pickup, como suele hacer la zamba.

### La melodía y la tonalidad

La **melodía** es la línea superior: de las notas que suenan en cada
instante en la pista de la melodía, la más aguda (un **skyline**). Las
notas que tiene debajo y que la melodía dobla, como la segunda voz de
Zamba de mi esperanza o el clarinete bajo la flauta de Acuarela del
río, van a una línea propia.

La **tonalidad** sale de cuánto suena cada una de las doce clases de
alturas, comparado con los **perfiles de tonalidad** de
Krumhansl–Kessler: cuánto encaja cada nota en cada tonalidad, medido
por oyentes en los años 1980. Gana la tonalidad cuyo perfil
correlaciona mejor. Donde eligió mal, la tonalidad se puso a mano, y
donde una parte cambia de tonalidad recibió su propio `in_key`.

Una prueba lo mantiene honesto:

```rust
#[test]
fn melodies_are_mostly_in_their_key() {
    for song in SONGS {
        let mut total = 0;
        let mut inside = 0;
        for part in song.parts {
            let scale = song.part_scale(part);
            // ...
            for note in part.melody().notes {
                total += 1;
                if scale.contains(note.pitch.pitch_class()) /* ... */ {
                    inside += 1;
                }
            }
        }
        assert!(inside * 100 >= total * 80, /* ... */);
    }
}
```

Al menos cuatro de cada cinco notas de toda melodía tienen que estar en
la tonalidad de su parte. `inside * 100 >= total * 80` compara números
enteros, así que no hay redondeo del que preocuparse. Balderrama la
suspendería en Mi menor armónica, la menor habitual de la app (228 de
288 notas), porque su melodía canta re natural, la séptima de la menor
natural, con mucha más frecuencia que re sostenido. En Mi menor natural
la pasa.

### Los acordes

Para cada compás, medio compás o pulso, el script puntúa cada acorde
común (mayor, menor, séptimas, disminuido y unos pocos más, sobre las
doce fundamentales): las notas dentro del acorde cuentan a favor, las
de fuera en contra, y la nota de bajo en la fundamental del acorde
cuenta extra. Los acordes de la tonalidad reciben un bono pequeño, para
que una nota de paso de una interpretación no convierta La menor en
algo exótico. Un compás se parte solo donde la partición puntúa
claramente mejor.

Esa es una primera lectura. Solo las notas nombran cada color como un
acorde propio: una sexta que añade el pianista convierte Bm en Bm6, una
nota de la melodía de paso añade una novena bemol a F#7, y una nota de
paso puede crear un acorde que no está en absoluto.

### Comprobar los acordes contra una partitura guía

Una **partitura guía** es la melodía con los nombres de los acordes encima,
la página desde la que tocan un cantante y una banda. Sus acordes son
los que la canción debería tener, pero sus compases no se alinean con
una interpretación libre: el pianista añade una introducción, toca una
sección dos veces, estira un compás. Así que los acordes de la partitura no
se pusieron sobre los compases uno a uno. En su lugar, la partitura le da
al lector su **vocabulario**: solo se pueden elegir los acordes que usa
la partitura.

La partitura suele estar en otra tonalidad que el pianista. La partitura de Uno
está en Sol y el pianista toca en Do, cinco semitonos más agudo, así
que cada acorde de la partitura se mueve una cuarta hacia arriba antes de
usarse: G se convierte en C, D7 en G7, Em en Am. Mover todos los
acordes por el mismo intervalo es **transportar**. Conserva el papel de
cada acorde en la tonalidad, así que la canción es la misma canción.

Yira, yira muestra lo que hace el vocabulario. Su partitura usa Bm, F#7,
A7, D, G, B7 y Em. Los compases 38 a 40, leídos solo desde las notas,
eran

```text
Bm6 F#7b9 | Bm F#7 | Bm A7
```

y limitados a los acordes de la partitura son

```text
Bm F#7 | Bm F#7 | Bm A7
```

La mano izquierda del pianista colorea el Si menor con un sol
sostenido, su sexta, y la melodía toca sol al final del compás, una
novena bemol sobre F#7. Bajo esos colores están los acordes sobre los
que está construida la canción, los que nombra la partitura: Bm y F#7, **i**
y **V7** de Si menor, la tónica y la séptima de dominante.

Cinco canciones se cortaron también donde cambia su forma: Yira, yira,
Sur, Malena, Soledad y Los mareados. Cada parte tiene su propia
tonalidad, así que la barra de armonía muestra la tonalidad en la que
está la música, y los acordes de esa tonalidad ganan las decisiones
dudosas. Las estrofas de Yira, yira están en Si menor y su estribillo
en Re mayor, la **relativa mayor**: la misma armadura de clave, dos
sostenidos, con Re como tónica en lugar de Si. Sur va al revés, de una
estrofa en Re mayor a un estribillo en Re menor, la **menor homónima**:
la misma tónica con tercera, sexta y séptima bemoles. Unos pocos
compases se arreglaron a mano, como los cuatro primeros compases de
Yira, yira, donde la mano izquierda entra tarde en cada compás y la
lectura la perdía.

Para las canciones sin partitura guía, las notas se comprobaron una a una.
Tiempos viejos acaba en Re, Fa, La y Si. El script lo había llamado
**Bm7b5** (Si Re Fa La), y son las mismas cuatro notas, pero el bajo
toca Re debajo y llega después de A7, la dominante de Re menor, así que
es **Dm6**: Re menor con una sexta añadida. (La última nota de la
melodía, un Mi agudo, añade una novena encima.) El nombre sigue al bajo
y a lo que hace el acorde, no solo a las notas.

### Deletrear las notas

Un archivo MIDI tiene números, no nombres. La nota 70 es la sostenido o
si bemol, y el archivo no dice cuál. La tonalidad nombra las siete
notas de su escala; las otras cinco necesitan una regla, y la regla es
el acorde alrededor de la nota:

- **Una nota del acorde se deletrea como la tiene el acorde.** En F#7
  (Fa♯, La♯, Do♯, Mi) la nota 70 es **la sostenido**, la tercera del
  acorde. En Gm (Sol, Si♭, Re) es **si bemol**.
- **Un acorde sobre una nota ajena a la tonalidad se nombra por donde
  va.** Una séptima de dominante resuelve una quinta abajo, así que G#7
  que va a C#m es sol sostenido, porque Sol♯ es la quinta de Do♯. Un
  acorde de séptima que cae un semitono sobre su objetivo es un
  **sustituto de tritono**: Ab7 que va a Sol es la bemol, no sol
  sostenido. Una séptima disminuida conduce un semitono arriba: G#dim7
  va a La.
- **Una nota ajena al acorde se deletrea por donde va.** Una **nota de
  paso cromática** entre dos notas a un tono de distancia es un
  sostenido de subida y un bemol de bajada: La, La♯, Si y Sol, Sol♭,
  Fa.
- **Ni mi sostenido, si sostenido, do bemol ni fa bemol para una nota
  de paso.** En la página parecen erratas de Fa, Do, Si y Mi. Los
  nombres propios de la tonalidad sí pueden usarlos: en Si menor, la
  nota entre Mi y Fa sostenido, de subida, es mi sostenido, la cuarta
  elevada que conduce a Fa sostenido, la dominante.
- **Dos notas en una misma línea del pentagrama reciben nombres
  propios.** Mi y mi sostenido sonando juntos se sentarían en la misma
  línea, así que el sostenido pasa a ser Fa.

## Comprobarlo

Cada demo se tocó con la [traza de la reproducción](playback-trace.md):

```text
ferrous-lineup --trace "Balderrama" --out balderrama.tsv
```

y cada nota del archivo MIDI se buscó en la traza, a la misma altura,
dentro de un quinto de segundo. Están casi todas. Lo que falta es lo
que la app toca una vez a propósito: unísonos que un archivo dobla en
un canal (las guitarras de Balderrama golpean muchas de sus notas dos
veces en el mismo instante), rasgueos más rápidos que una semicorchea,
y golpes de batería que caen en el mismo paso de un carril. Acuarela
del río perdió 10 notas de 2408.

## Ejercicios

1. Toca **Uno** y apunta el tempo de sus ocho primeros compases desde
   el transporte. ¿Dónde se frena el pianista? ¿Qué pasa en la letra
   allí?
2. Da palmadas con las dos agrupaciones de seis corcheas, 3+3 y 2+2+2,
   y después ambas a la vez con un amigo. Luego toca **Acuarela del
   río** y da palmadas con el bajo, y después con la batería.
3. Escribe las escalas de Mi menor natural, Mi menor armónica y Mi
   dórica. ¿Qué nota difiere en cada una? Después toca los compases 21
   a 23 de Balderrama y nombra el acorde bajo los do sostenidos.
4. Elige una de las seis canciones de piano sin partitura guía aún
   (Patotero sentimental, Qué falta que me hacés, Qué me van a hablar
   de amor, Tiempos viejos, La casita de mis viejos, Lluvia de
   estrellas), encuentra una partitura guía y comprueba una parte contra ella.
   ¿Qué acordes fallaron las notas solas?
5. En Do mayor, nombra la tecla negra entre Fa y Sol: sobre un acorde
   D7; en la línea Sol, ?, Fa; y sobre un Ab7 que va a G. Después
   comprueba tus respuestas contra las reglas de arriba.
6. Rust: `in_key` y `with_tempo` son métodos `const fn` que toman
   `self` por valor y devuelven un `Part` nuevo. ¿Por qué eso permite
   encadenarlos dentro de una constante, y por qué no funcionaría allí
   `&mut self`?
