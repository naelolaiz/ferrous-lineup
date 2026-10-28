# Métricas irregulares: 5/4, 7/8 y las danzas de los Balcanes

Archivos: [`crates/lineup-core/src/meter.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/meter.rs),
[`crates/lineup-core/src/library/comping.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/comping.rs),
[`crates/lineup-core/src/library/rhythm.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/rhythm.rs),
[`crates/lineup-core/src/library/style.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/style.rs),
[`crates/lineup-core/src/library/chart.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/chart.rs),
[`crates/lineup-core/src/library/song.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/song.rs),
[`crates/lineup-core/src/library/song/balkan.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/song/balkan.rs),
[`crates/lineup-core/src/library/song/jazz.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/song/jazz.rs),
[`crates/lineup-core/src/library/song/folk.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/song/folk.rs),
[`crates/lineup-core/src/library/song/classical.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/song/classical.rs)
y [`crates/lineup-core/src/library/demo.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/demo.rs)

La app ya se podía poner a 5/4 o 7/8 a mano (mira [Cualquier compás, y
métricas irregulares](../step-8/any-time-signature.md#cómo-usarlo)),
pero la biblioteca no tenía nada que tocar en ellos: cambiar el Time de
una canción a 7/8 le daba un simple «basic beat», y ninguna demo estaba
en un compás irregular. Este cambio añade **once ritmos y estilos de
armonía** en compases irregulares, una manera de que la banda los
**toque**, y canciones para oírlos: **Take Five** en 5/4; **Ajde Jano**,
**Jovano Jovanke**, **Samiotissa** y la **Svatbarska rachenitsa** en
7/8; **Paidushko horo** en 5/8; **Daichovo horo** y **Entarisi ala
benziyor** en 9/8; **Gankino horo** en 11/8; **Bučimiš** en 15/8; y la
**Promenade** de Músorgski, cuyo compás cambia de un compás al
siguiente.

Una segunda ronda añadió más, con siete ritmos y estilos más: el
**Mission: Impossible theme** en 5/4; **Yerakina** y **Milo mou
kokkino**, dos kalamatianos griegos en 7/8; **Tsakonikos** en 5/8; dos
danzas rumanas, **Geamparaua de la Murfatlar** en 7/8 y **Mândrele** en
5/4; tres canciones populares turcas, **Sarı gelin** en 10/8,
**İzmir'in kavakları** en 9/8 y **Çayeli'nden öteye** en 7/8;
**Süsserl**, una danza bávara cuyos compases pasan de dos pulsos a tres
y vuelta; y el **waltz in five** de Chaikovski, el segundo movimiento de
su Sinfonía *Patética*.

## Cómo probarlo

Abre **File > Demos > Songs > Balkan and Turkish > Ajde Jano** y pulsa
Play. Cuenta con la batería: **1 2 3, 1 2, 1 2**. El compás tiene siete
corcheas, pero no cuentas siete: sientes tres pulsos, uno largo y dos
cortos. El big drum golpea el pulso largo; el bajo toca una nota en
cada pulso y el acordeón toca su acorde en las corcheas de en medio,
«oom-pa-pa, oom-pa, oom-pa».

Ahora abre **Take Five** (en Jazz). El compás tiene cinco negras, y el
vamp del piano las agrupa **1 2 3, 1 2**: Mi♭ menor durante tres pulsos,
Si♭ menor séptima durante dos.

Luego abre cualquier canción en 4/4, por ejemplo **Old MacDonald Had a
Farm**, abre la lista **Time**, deja marcado **Use matching style** y
haz clic en **7/8**. Antes, la canción recibía un basic beat; ahora
recibe una **rachenitsa**, y la lista Time muestra `2+2+3/8`: la manera
búlgara de contar siete.

## La música

### Pulsos largos y pulsos cortos

En 4/4 o 3/4 todos los pulsos duran lo mismo. En 6/8 las seis corcheas
forman dos pulsos de tres. Muchas danzas de los Balcanes, Grecia y
Turquía cuentan sus corcheas en pulsos de **dos y tres mezclados**:

| Compás | Contado | Danza |
|---|---|---|
| 5/8 | 2 + 3, rápido, lento | paidushko búlgaro |
| 5/8 | 3 + 2, lento, rápido | tsakonikos griego |
| 7/8 | 3 + 2 + 2, lento, rápido, rápido | kalamatianos griego, lesnoto macedonio, devr-i hindî turco |
| 7/8 | 2 + 2 + 3, rápido, rápido, lento | rachenitsa búlgara, geampara rumana |
| 9/8 | 2 + 2 + 2 + 3 | daichovo búlgaro, karşılama tracio, aksak turco (el zeybek) |
| 9/8 | 2 + 3 + 2 + 2 | raks aksağı turco |
| 10/8 | 3 + 2 + 2 + 3 | curcuna turco |
| 11/8 | 2 + 2 + 3 + 2 + 2 | kopanitsa búlgara (Gankino horo) |
| 15/8 | 2 + 2 + 2 + 2 + 3 + 2 + 2 | bučimiš búlgaro |

Un pulso largo dura **la mitad más** que uno corto, tres corcheas
contra dos, así que los pasos de un bailarín van largo, corto, corto, o
corto, corto, largo: la danza parece cojear. Los músicos turcos llaman
a estos compases **aksak**, «que cojea». El mismo 7/8 puede cojear de
dos maneras, y por eso la app escribe la agrupación en el compás:
`3+2+2/8` es un kalamatianos, `2+2+3/8` una rachenitsa. Escribe el
compás como lo cuenta un bailarín, porque la batería, el bajo y los
acordes siguen todos la agrupación.

El jazz llevó la idea más lejos en 1959, en el álbum *Time Out* del
Dave Brubeck Quartet: **Take Five** está en 5/4, contado 3 + 2, y
Brubeck escribió **Blue Rondo à la Turk** en un 9/8 que oyó tocar a
músicos callejeros turcos. El **Mission: Impossible theme** de Lalo
Schifrin (1966) también está en 5/4, pero reparte las diez corcheas del
compás en **3 + 3 + 2 + 2**: largo, largo, corto, corto. El ostinato
del piano lo dice, Sol, Sol, luego Si♭ y Do, y el nuevo ritmo **Five-four
spy** pone el bombo y una conga aguda en cada uno de esos cuatro
arranques.

5/4 no siempre es 3 + 2. **Mândrele**, una danza rumana, se mueve en
las tres primeras negras de cada compás y sostiene una blanca en las
dos últimas, 3 + 2 como Take Five; el **waltz in five** de Chaikovski
va al revés, **2 + 3**: un pulso corto se cuela antes de cada tres de
vals, así que la danza cojea con gracia, siempre inclinándose hacia el
pulso largo.

### Un vals en cinco

La Sexta Sinfonía de Chaikovski (1893) tiene, como segundo movimiento,
un vals al que le falta un pulso, o más bien le sobra uno: cinco negras
por compás, contadas 1 2, 1 2 3. La demo toca el movimiento entero, con
la melodía pasada de los violonchelos a las maderas, los violines y el
oboe. El acompañamiento marca solo los dos pulsos sentidos, el 1 y el
3. En la sección central, el trío, la tonalidad gira a Si menor, pero
los contrabajos sostienen **Re** desde su primer compás hasta el último:
una **nota pedal**, una nota de bajo que no se mueve mientras los
acordes por encima cambian, Si menor sobre Re, luego la séptima
disminuida La♯, Do♯, Mi, Sol sobre Re. En un lead sheet eso se escribe `Bm/D`
y `A#°7/D`, y el chord sheet lo muestra así.

### Métricas que cambian de un compás al siguiente

Alguna música cambia de compás según avanza. La **Promenade** de
Músorgski, el paseo entre los cuadros de *Cuadros de una exposición*
(1874), pone un compás de 5/4 junto a uno de 6/4: la melodía está
fraseada como el habla, o como una canción popular rusa, no como una
marcha. Sus pulsos son todos iguales, cinco y luego seis negras, cada
una apoyada: aquí no hay pulso largo y pulso corto, solo compases de
distinta longitud. En la app, cada compás de una parte así lleva su
propia indicación de compás, la demo pone un **time marker** dondequiera
que cambia (mira la regla), y la batería y la banda siguen cada uno.

### Compases de dos y compases de tres

Un **zwiefacher** (de Baviera y Austria) mezcla compases de 2/4 y 3/4
en un orden que fija la melodía. En un compás de tres la pareja baila
un paso de vals; en un compás de dos pivota, girando en dos pasos en
lugar de tres. **Süsserl** hace 2 2 3, 2 2 3, 2 2 3 3 en su primera
sección y 2 2 3 2 2 en la segunda, y los bailarines se lo saben de
memoria. El pulso sigue igual, solo cambian los compases, así que la
banda toca «oom-pa» en un compás de dos y «oom-pa-pa» en uno de tres, y
la regla muestra un time marker en cada cambio, como en la Promenade.

### Los modos de los Balcanes y Turquía

**Entarisi ala benziyor** es una canción popular turca en **makam
hicaz**. Un makam es más que una escala (tiene su propia manera de
moverse y de reposar), pero sus notas sobre La son La, Si♭, Do♯, Re, Mi, Fa,
Sol. El tono y medio entre Si♭ y Do♯ es lo que la hace sonar «oriental».
Esas son las notas del modo frigio con la tercera elevada, la misma
escala que la del flamenco, así que la tonalidad de la demo es La frigio.
**Jovano Jovanke** canta la misma escala sobre Mi: Mi, Fa, Sol♯, La, Si, Do, Re,
y su primera frase cae Sol♯, Fa, Mi a través de ese tono y medio.

La música turca divide el tono entero en nueve **comas**; el Si♭ y el Do♯
del hicaz están a cuatro comas de Si y Do, lo bastante cerca del semitono
de un piano.

Las canciones populares turcas se cantan y se tocan al unísono, sobre
un tambor. Los acordes de esta demo son **añadidos**: Re menor donde la
melodía corre entre Re y Fa, La mayor donde reposa sobre Do♯ y La, Si♭ mayor
bajo sus Si♭ largos. **Çayeli'nden öteye**, de la costa del mar Negro, y
la rumana **Geamparaua de la Murfatlar** usan la misma escala hicaz
sobre La.

Con las canciones turcas llegaron dos makams más. El **hüseyni**, en
**Sarı gelin**, es La, Si, Do, Re, Mi, Fa♯, Sol: cerca del modo **dórico**, una
escala menor con sexta mayor, así que su acorde de IV, Re7, es mayor. El
**segâh**, en **İzmir'in kavakları**, reposa sobre Si: Si, Do, Re, Mi, Fa♯,
Sol, La, el modo **frigio**, con su segunda bemol, Do, justo encima de la
nota sobre la que reposa. Su Si y su Mi están en realidad una **coma**
más bajos que los de un piano, y la app toca las teclas más cercanas.

Observa algo de esas escalas: Si frigio tiene las mismas notas que **Sol
mayor**, y el hicaz sobre La las mismas notas que **Re menor armónica**
(Re, Mi, Fa, Sol, La, Si♭, Do♯). Un modo es una escala oída desde otra nota.
Cuando el buscador de tonalidad de la app escucha estas canciones, oye
Sol mayor y Re menor, porque conoce las tonalidades mayores y menores, no
los makams: acierta las notas y falla la nota sobre la que reposan.

**Mândrele** está en Do con Fa♯, el modo **lidio**, una escala mayor con
la cuarta elevada; su Fa♯ reposa sobre un acorde de Re mayor (II), y
secciones posteriores traen un Si♭, que convierte Do en Do7.

**Gankino horo** tiene otro color: Re, Mi, Fa, Sol♯, La, Si, Do, una escala
menor con la cuarta elevada y un Si natural, el modo dórico con cuarta
sostenida (los músicos turcos lo llaman nikriz). El Sol♯ roza contra el
acorde de Re menor y lleva a Mi mayor (II), donde cada sección se
detiene.

### Tresillos

Algunas melodías necesitan **tresillos**: tres notas en el tiempo de
dos. Jovano Jovanke tiene uno en su primera línea, Fa, Mi, Fa en el tiempo
de una negra. Una melodía puede ahora escribir `(3` antes de tres
notas: `(3 G4:2 G4 G4` son tres corcheas en el tiempo de una negra. Sus
arranques caen entre los pasos de semicorchea de la cuadrícula, así que
se colocan al tick.

## El código

### Un compás escrito como constante

Las canciones y los ritmos son tablas `const`, construidas antes de que
el programa se ejecute. `TimeSignature::grouped(&[3, 2, 2], 8)`
devuelve un `Option`, que una constante no puede desenvolver. Un
segundo constructor es un **`const fn`**, una función que Rust puede
ejecutar en tiempo de compilación:

```rust
pub const fn of_groups(groups: &[u8], unit: u8) -> Self {
    let mut mask = 0_u64;
    let mut at = 0;
    let mut index = 0;
    // `for` loops are not allowed in a `const fn` yet: `while` is.
    while index < groups.len() {
        assert!(groups[index] > 0, "a group of no notes");
        mask |= 1 << at;
        at += groups[index] as usize;
        index += 1;
    }
    ...
}

const THREE_TWO_TWO: TimeSignature = TimeSignature::of_groups(&[3, 2, 2], 8);
```

La agrupación es una **máscara de bits**: el bit `n` está puesto cuando
un pulso empieza en la nota `n`. Para 3+2+2 son los bits 0, 3 y 5. Un
`assert!` que falla en un `const fn` detiene la **build**, no el
programa: una agrupación mal escrita nunca llega a un usuario.

### Los pulsos que siente un músico

`felt_groups()` dice cómo se siente un compás: su agrupación si tiene
una, pulsos de tres corcheas en 6/8, y si no, doses con un tres al
final, de modo que un 7/8 sin agrupar se siente 2 + 2 + 3 y un 5/4 como
2 + 3.

### Una banda que cojea

Los patrones de comping del piano eran tablas de hits que se repiten
cada tantos pasos, sea cual sea el compás: eso funciona en 4/4, pero un
patrón de dos pulsos derivaría a través de un compás de 7/8. Los nuevos
`Comping::Aksak` y `Bass::Aksak` se **construyen desde el compás**: el
bajo en la primera corchea de cada pulso sentido, el acorde en las
demás.

```rust
fn hits(self, time: TimeSignature) -> (usize, Cow<'static, [Hit<Voices>]>) {
    match self {
        Comping::Aksak => {
            let unit = time.unit_steps();
            // A chord a little shorter than its note, so the next
            // one is heard to start, but never under a step.
            let length = (unit - unit / 4).max(1);
            let mut hits = Vec::new();
            let mut at = 0;
            for group in time.felt_groups() {
                for note in 1..usize::from(group) {
                    let velocity = if note == 1 { 84 } else { 74 };
                    hits.push(hit!(at + note * unit, length, Voices::All, velocity));
                }
                at += usize::from(group) * unit;
            }
            (time.steps_per_bar(), Cow::Owned(hits))
        }
        _ => {
            let (period, hits) = self.pattern();
            (period, Cow::Borrowed(hits))
        }
    }
}
```

**`Cow`** («clone on write») guarda o bien un valor prestado o bien uno
propio. Los patrones viejos son tablas `'static`, así que se toman
prestados gratis (`Cow::Borrowed`); el patrón aksak se construye para
cada compás, así que es propio (`Cow::Owned`). El código que toca los
hits lee ambos de la misma manera. Para construirlo, `accompany`,
`Comping::play` y `Bass::play` reciben ahora el compás, y un patrón
aksak cuenta desde la **línea de compás**, no desde el acorde: un
acorde que empieza en el segundo pulso de un compás de 7/8 sigue
recibiendo su bajo en el pulso dos.

### Una parte cuyo compás cambia

El `Part` de una canción recibe una lista de metros, uno por compás, y
`Song::part_runs` corta la parte en **tramos** de compases en un mismo
metro, cada uno con su parte de la melodía y de los acordes. Un patrón
tiene un solo compás, así que la demo crea un patrón por tramo,
llamado «Promenade, bar 2» y así, y pone un time marker donde empieza
cada uno.

### Tresillos en la melodía

`parse_melody` contaba antes en pasos. Ahora cuenta en **ticks**, 24
por paso, así que una corchea de tresillo dura 32 ticks (el valor de
dos pasos, 48, por dos tercios). Una nota se crea con
`NoteEvent::exact`, que la coloca en cualquier tick.

### La lista Time elige un ritmo que cojea

`Rhythm::for_time` busca un ritmo en el compás nuevo. Un 7/8 escrito
sin agrupación no encajaba con ninguno, así que la canción recibía un
basic beat. Ahora, para un compás irregular sin agrupación, busca entre
los ritmos con **el mismo número de notas por compás**: primero uno del
tipo de la propia canción (una canción balcánica en 7/8 recibe un
kalamatianos, el primero que tiene), luego uno agrupado como se siente
el compás (7/8: 2+2+3, una rachenitsa), y luego cualquiera. 9/8 no es
irregular en este sentido: escrito así a secas significa tres pulsos de
tres, así que no recibe un daichovo.

### Usuls turcos en la darbuka

Un ciclo rítmico turco, un **usul**, se recita como golpes de tambor
graves y agudos: **düm** en el centro del tambor, **tek** en el borde.
Los ritmos nuevos los escriben en la línea de conga, `l` para el düm y
`h` para el tek, con el big drum doblando los düms:

```rust
rhythm(
    "Devr-i hindi",
    Balkan,
    120.0,
    &[
        (Kick, "X..... X... ...."),
        (Conga, "l.h.h. l... h..."),
        (Shaker, "x.o.o. x.o. x.o."),
    ],
    "",
    "The Turkish devr-i hindî, 7/8 counted 3 + 2 + 2: ...",
)
.in_time(TimeSignature::of_groups(&[3, 2, 2], 8)),
```

Cada carácter es un paso de semicorchea, así que una corchea son dos:
düm, tek, tek en las tres corcheas del pulso largo, y luego düm y tek
en los dos cortos.

### Solo acentos

`Comping::Accents` y `Bass::Accents` tocan un acorde y una fundamental
en cada pulso sentido, el primero el más fuerte, y nada en medio. Así
se acompaña el vals de Chaikovski, en su 2 + 3:

```rust
Comping::Accents => {
    let unit = time.unit_steps();
    let mut at = 0;
    let hits = time
        .felt_groups()
        .into_iter()
        .enumerate()
        .map(|(index, group)| {
            let hit = hit!(at, unit, Voices::All, if index == 0 { 104 } else { 88 });
            at += usize::from(group) * unit;
            hit
        })
        .collect();
    (time.steps_per_bar(), Cow::Owned(hits))
}
```

`felt_groups()` da `[2, 3]` para este 5/4, así que los hits caen en la
primera negra y en la tercera.

### Cómo se escribieron las canciones

Cada canción tiene un comentario `// Source:` con la transcripción de
la que salen sus notas: archivos ABC de las colecciones de danzas
populares de John Chambers y Henrik Norbeck, la parte de saxo alto de
Paul Desmond para Take Five, la edición en LilyPond del Mutopia Project
de Cuadros de una exposición, el corpus SymbTr de música makam turca,
un lead sheet de Wikifonia para el Mission: Impossible theme y la
partitura completa y el análisis armónico del MCTL de la sinfonía de
Chaikovski, donde las notas ganan siempre que el análisis discrepa de
ellas. Donde una fuente no tiene acordes, el comentario dice que los
acordes son una armonización sencilla. Muchas danzas balcánicas están
escritas en semicorcheas (7/16, 11/16); aquí están escritas en
corcheas, cada nota el doble de larga, lo que suena igual al tempo
dado.

### Derechos de autor

Las canciones y danzas populares son tradicionales, y la música de
Músorgski lleva mucho tiempo en el dominio público. **Take Five** (Paul
Desmond, 1959) y el **Mission: Impossible theme** (Lalo Schifrin, 1966)
siguen bajo derechos de autor, y un comentario encima de cada uno lo
dice.

## Ejercicios

1. Abre **Ajde Jano** y, en la lista Time, escribe `7` sobre `8`
   agrupado como `2+2+3` y haz clic en **Use 2+2+3/8**. La melodía
   conserva sus notas, pero la banda se apoya ahora en otras corcheas.
   ¿Qué pulsos de la melodía caen ahora entre las notas del bajo?
2. En la Library, reproduce los ritmos **Kalamatianos** y
   **Rachenitsa** uno detrás de otro. Aplaude con ellos: ¿dónde está el
   pulso largo en cada uno?
3. Elige `5/8` en la lista Time de una canción en 4/4, con **Use
   matching style** marcado. ¿Qué ritmo recibe, y cómo escribe la lista
   Time el compás?
4. Abre el **Mission: Impossible theme** y aplaude el ostinato del
   piano: largo, largo, corto, corto. Luego abre **Take Five**. Ambos
   están en 5/4: ¿dónde caen sus pulsos largos?
5. Abre **Süsserl** y mira la regla mientras suena. Cuenta los compases
   de dos y los compases de tres en la primera sección; ¿coinciden con
   2 2 3, 2 2 3, 2 2 3 3?
6. Escribe un estilo propio en `style.rs`: `Comping::Aksak` y
   `Bass::Aksak` en `TimeSignature::of_groups(&[3, 3, 2], 8)`, 8/8
   contado como un tresillo. ¿Sigue pasando
   `every_style_is_heard_as_its_chart`?
