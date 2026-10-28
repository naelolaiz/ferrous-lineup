# Entrenamiento auditivo

Archivos: [`crates/lineup-core/src/ear_training.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/ear_training.rs),
[`crates/lineup-audio/src/phrase.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-audio/src/phrase.rs),
[`crates/lineup-app/src/ear_training.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/ear_training.rs)
y [`crates/lineup-app/src/app/ear.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/ear.rs)

Todo lo de hasta ahora te ayuda a *ver* la armonía: nombres de acordes
en el sheet, numerales y colores en las harmony keys, nombres de
conjuntos en la ventana dodecafónica. Los músicos también necesitan
*oírla*: distinguir una tercera menor de una mayor, un acorde
disminuido de uno menor, y el V de una canción de su IV, sin mirar. Es
una habilidad propia, y se aprende como la enseñan los juegos de
entrenamiento auditivo: algo suena, tú lo nombras, y al momento se te
dice si acertaste.

Este es el punto 13 del plan. Es **experimental** y no estorba: vive
en su propia ventana, que está cerrada hasta que la abres, y no cambia
nada ni en la canción ni en el archivo.

## Lo que ves

**View > Ear training** abre la ventana. Tres pestañas están arriba:
**Intervals**, **Chords** y **Progressions**. Debajo, un cuadro dice
qué preguntar; luego **Start** hace la primera pregunta, y la toca. Si
la canción estaba sonando, se pausa, para que la pregunta se oiga
sola.

![La pestaña Intervals: m3, M3, P4, P5 y P8 elegidos, tocados Up. La respuesta P5 está en verde, y el texto explica: «Si♭3 hasta Fa4: una quinta justa, 7 semitonos. Abierta y estable: la fundamental y la quinta de todo acorde mayor y menor. Como Twinkle, Twinkle, Little Star, en su primer salto».](../images/step-9-ear-intervals.png)

**Intervals.** Suenan dos notas, y los botones grandes son las
respuestas. El cuadro de arriba elige qué intervalos se pueden
preguntar (una tercera menor, una tercera mayor, una cuarta, una quinta
y la octava para empezar), y cómo se tocan: **Up** (primero la nota
grave), **Down** (primero la aguda) o **Together**. Menos opciones lo
hacen más fácil; empieza con unas pocas que suenen muy distintas, y
añade las demás de una en una.

Después de responder, tu botón se pone verde o rojo, el correcto se
pone verde, y un cuadro explica la respuesta: las dos notas con sus
octavas, el nombre completo del intervalo, cuántos semitonos abarca,
qué le da su sonido y, hacia arriba, una melodía que empieza con él.
Luego cada botón de respuesta toca *su propio* intervalo desde la misma
nota, para que oigas el que has dicho junto al correcto. **Play again**
toca la pregunta tantas veces como quieras, antes o después de
responder; **Next** pregunta otra (es **Skip** antes de responder, y una
pregunta saltada no cuenta).

![La pestaña Chords con los nueve tipos y Inversions marcado. La respuesta «maj» está en rojo, «dim7» en verde, y el texto: «Ddim7: séptima disminuida de Re, en posición fundamental. Re Fa La♭ Do♭: una tercera menor, una tercera menor y una tercera menor apiladas desde Re. Do♭ es la tecla Si, escrito Do♭ porque es la séptima del acorde...»](../images/step-9-ear-chords.png)

**Chords.** Suena un acorde, y tú nombras su tipo: mayor, menor,
disminuido, aumentado, o uno de cinco acordes de séptima. La
fundamental cambia de pregunta a pregunta; solo cuenta el tipo.
**Note by note** toca el acorde nota a nota desde el bajo hacia
arriba, cada una mantenida, de modo que acaba como el acorde entero.
Con **Inversions** marcado, la nota más grave no es siempre la
fundamental (más sobre eso abajo).

![La pestaña Progressions mientras toca la banda: cuatro casillas, la primera mostrando I, la segunda enmarcada en blanco porque su acorde está sonando, y debajo siete harmony keys que muestran solo I ii iii IV V vi vii° y hogar/lejos/tensión, con iii y vii° atenuados.](../images/step-9-ear-progression-playing.png)

**Progressions.** Este es el ejercicio que pedía el plan: la banda
toca unos cuantos acordes en una tonalidad, un compás cada uno, y tú
los nombras por sus numerales. La pregunta dice la tonalidad («In F
major») y el primer acorde es siempre I, el hogar, así que sabes dónde
empiezas. Las respuestas son las harmony keys que conoces de la barra
de abajo de la ventana, con sus colores y su trabajo (hogar, lejos,
tensión) pero con los nombres de los acordes ocultos. Las teclas de
acordes que no se pueden preguntar están atenuadas. Mientras toca la
banda, un marco blanco camina por las casillas, como hace en las
harmony keys, para que sepas qué compás estás oyendo.

Haz clic en las teclas en orden, una por cada acorde después de I.
Luego cada casilla se pone verde o roja, una roja dice cuál era el
acorde («era IV»), y el cuadro explica los acordes, sus numerales y por
qué cada uno sigue al anterior, con las palabras de las sugerencias de
las harmony keys:

![La progresión contestada: casillas I, vi (era IV), IV (era V), V (era I) y el texto «F – Bb – C – F: I IV V I en Fa mayor. De I a IV: alejarse del hogar, hacia la dominante. De IV a V: un paso hacia la dominante, listo para volver al hogar. De V a I: el final más fuerte. La sensible sube a la fundamental».](../images/step-9-ear-progression.png)

Después de responder, cada casilla y cada tecla tocan su acorde en el
piano, y **Hear the key** toca I, IV, V e I: una cadencia que dice
dónde está el hogar, lo que ayuda cuando te pierdes.

El cuadro de arriba elige las **Keys** (mayor, menor o ambas), los
**Chords** que pueden salir (I IV V; o I ii IV V vi; o los siete), si
son **Sevenths** (V7, ii7, Imaj7...), quién los toca (**Piano**,
**Piano and bass** o la **Band** con batería), cuántos acordes (4 u 8)
y el **Tempo**.

**El marcador.** La línea de abajo cuenta las respuestas correctas, el
porcentaje y cuántas seguidas. Cada pestaña lleva el suyo, y en una
progresión cada acorde cuenta como una respuesta. **Start again** lo
pone de nuevo a cero. El marcador se olvida cuando se cierra la app;
las opciones de los cuadros se recuerdan.

## La teoría en las respuestas

La app enseña con sus respuestas, así que tienen que ser exactas. Esto
es lo que dicen, y por qué.

### Intervalos

El nombre de un intervalo tiene dos partes. El **número** cuenta
nombres de nota, incluyendo los dos extremos: de Do hasta Mi son Do, Re, Mi, tres
nombres, una *tercera*. De Do hasta Mi♭ es una tercera también: los mismos
nombres. La **cualidad** viene luego de los semitonos: una tercera de 4
semitonos es *mayor*, de 3 *menor*.

Las segundas, terceras, sextas y séptimas vienen en mayor y menor. Los
unísonos, las cuartas, las quintas y las octavas se llaman **justas**:
dados la vuelta (la nota grave subida una octava), un intervalo justo
da otro justo, mientras que uno mayor se vuelve menor y uno menor,
mayor. Los números de un intervalo y de su gemelo invertido suman
nueve, y sus semitonos doce: una tercera mayor (4) dada la vuelta es
una sexta menor (8), una cuarta justa (5) una quinta justa (7). Un
semitono más que mayor o justo es *aumentado*, uno menos que menor o
justo, *disminuido*.

| Botón | Nombre | Semitonos | Desde Do | Una melodía que empieza con él |
|-------|--------|-----------|---------|--------------------------------|
| m2 | segunda menor | 1 | Do–Re♭ | Jaws, las dos notas del tiburón |
| M2 | segunda mayor | 2 | Do–Re | Happy Birthday, en su primer paso hacia arriba |
| m3 | tercera menor | 3 | Do–Mi♭ | Greensleeves |
| M3 | tercera mayor | 4 | Do–Mi | When the Saints Go Marching In |
| P4 | cuarta justa | 5 | Do–Fa | Here Comes the Bride |
| TT | tritono (cuarta aumentada) | 6 | Do–Fa♯ | Maria, de West Side Story |
| P5 | quinta justa | 7 | Do–Sol | Twinkle, Twinkle, Little Star, en su primer salto |
| m6 | sexta menor | 8 | Do–La♭ | The Entertainer, en su primer salto hacia arriba |
| M6 | sexta mayor | 9 | Do–La | My Bonnie Lies over the Ocean |
| m7 | séptima menor | 10 | Do–Si♭ | Somewhere, de West Side Story |
| M7 | séptima mayor | 11 | Do–Si | |
| P8 | octava | 12 | Do–Do | Over the Rainbow |

Las melodías empiezan con el intervalo hacia arriba, así que la app
nombra una solo cuando la pregunta fue hacia arriba. La séptima mayor
no tiene ninguna: pocas melodías conocidas empiezan con ella.

El **tritono** es el único intervalo con dos números. Seis semitonos
por encima de Do son la tecla Fa♯ o la tecla Sol♭: de Do a Fa♯ hay cuatro
nombres (Do Re Mi Fa), una *cuarta aumentada*; de Do a Sol♭ hay cinco (Do Re Mi Fa
Sol), una *quinta disminuida*. Las mismas teclas, el mismo sonido, dos
escrituras. Su nombre dice lo que es: tres tonos enteros, exactamente
media octava.

**La escritura.** La explicación escribe las notas como lo haría un
músico. La nota grave es una tecla blanca, o una tecla negra escrita
con un sostenido o un bemol, lo que escriba la *pareja* con menos de
ellos: de Do♯ a Mi es una tercera menor (escribirlo de Re♭ a Fa♭ llevaría
dos bemoles), pero de Re♭ a Fa es una tercera mayor (de Do♯ a Mi♯ llevaría
dos sostenidos). Cuando ambos son igual de simples, una tecla negra
conserva su nombre habitual: de Si♭ a Re♭, no de La♯ a Do♯. El tritono es
una cuarta aumentada o una quinta disminuida por la misma regla: de Fa a
Si (sin alteración) antes que de Fa a Do♭, y una cuarta aumentada cuando
ambos son igual de simples: de Do a Fa♯ antes que de Do a Sol♭, de Si♭ a Mi
antes que de Si♭ a Fa♭ o de La♯ a Mi.

### Acordes

Todo acorde que toca el ejercicio está construido con terceras,
apiladas desde la fundamental. Los nueve tipos, sobre Do:

| Botón | Nombre | Símbolo | Notas | Terceras, desde abajo |
|-------|--------|---------|-------|-----------------------|
| maj | mayor | Do | Do Mi Sol | mayor, menor |
| m | menor | Cm | Do Mi♭ Sol | menor, mayor |
| dim | disminuido | Cdim | Do Mi♭ Sol♭ | menor, menor |
| aug | aumentado | Caug | Do Mi Sol♯ | mayor, mayor |
| 7 | séptima de dominante | C7 | Do Mi Sol Si♭ | mayor, menor, menor |
| maj7 | séptima mayor | Cmaj7 | Do Mi Sol Si | mayor, menor, mayor |
| m7 | séptima menor | Cm7 | Do Mi♭ Sol Si♭ | menor, mayor, menor |
| m7b5 | séptima semidisminuida | Cm7b5 (Cø7) | Do Mi♭ Sol♭ Si♭ | menor, menor, mayor |
| dim7 | séptima disminuida | Cdim7 (C°7) | Do Mi♭ Sol♭ Si♭♭ | menor, menor, menor |

Dos acordes que a menudo se enseñan pronto se **dejan fuera a
propósito**: los acordes de sexta y los suspendidos. C6 es Do Mi Sol La, las
mismísimas teclas que Am7 (La Do Mi Sol); Csus2 (Do Re Sol) tiene las teclas de
Gsus4 (Sol Do Re). Tocados solos, nada los distingue: lo que hace que uno
sea C6 y el otro Am7 es el bajo y la música de alrededor. Un juego que
los preguntara marcaría mal respuestas correctas. Una prueba comprueba
que no hay dos de los nueve tipos que compartan sus notas sobre dos
fundamentales cualesquiera.

Algunas escrituras parecen raras al principio: **Si♭♭** en Cdim7, **Do♭**
en Fdim. No son errores. La séptima de un acorde de Do es algún tipo de
Si (Do Re Mi Fa Sol La Si, siete nombres), y una séptima disminuida está un
semitono por debajo de la séptima menor Si♭, así que se escribe Si doble
bemol, aunque es la tecla La. De la misma manera la quinta de Fa es algún
tipo de Do, y una quinta disminuida está un semitono por debajo de Do:
Do♭, la tecla Si. La explicación lo dice siempre que escribe una nota con
un nombre distinto de los habituales de su tecla, porque el
nombre dice qué trabajo hace la nota en el acorde.

**Inversiones.** Un acorde está *invertido* cuando una nota distinta de
su fundamental es la más grave: la tercera (primera inversión), la
quinta (segunda inversión) o la séptima (tercera inversión). El símbolo
nombra el bajo tras una barra: C/E es Do mayor con Mi como nota más grave
([Nombres de acordes](chord-names.md), [Inversiones](inversions.md)).
Un acorde invertido conserva su tipo, así que la respuesta no cambia,
pero suena menos asentado y es más difícil de nombrar; para eso está la
casilla **Inversions**.

Dos tipos son especiales. Las notas del acorde aumentado están
espaciadas por igual (a una tercera mayor, y a una tercera mayor desde
la nota de arriba de vuelta a la fundamental), y las de la séptima
disminuida también (a una tercera menor). Pon cualquiera de sus notas
en el bajo y obtienes las mismísimas teclas que el acorde en posición
fundamental sobre *esa* nota: Caug/E es también Eaug (Mi Sol♯ Si♯, donde Si♯
es la tecla Do). El oído no puede saber cuál nota es la fundamental, así
que la explicación de un acorde aumentado o de séptima disminuida
invertido dice qué otro acorde es también. El tipo sigue siendo seguro,
así que la pregunta sigue siendo justa.

### Progresiones

Los numerales y sus colores vienen de las harmony keys ([Las harmony
keys](../step-8/harmony-keys.md)): mayúsculas para los acordes mayores,
minúsculas para los menores, ° para los disminuidos, y tres trabajos,
hogar (azul), lejos (verde) y tensión (naranja). En Do mayor:

```text
  I    ii    iii   IV    V     vi    vii°
  C    Dm    Em    F     G     Am    Bdim
```

Una tonalidad menor se lee `i ii° bIII iv V bVI bVII`. Los numerales
cuentan a lo largo de la escala *mayor* desde la misma tónica, así que
los acordes sobre las notas tercera, sexta y séptima rebajadas reciben
un bemol: en La menor, Do es bIII. Y V es mayor, como en casi toda la
música en tonalidades menores: Mi mayor en La menor, con Sol♯ tirando hacia
La. Con **Sevenths**, las teclas pasan a `Imaj7 ii7 iii7 IVmaj7 V7 vi7
viiø7` en mayor y `i7 iiø7 bIIImaj7 iv7 V7 bVImaj7 bVII7` en menor.

**Cómo se hace una progresión.** Empieza en I. Cada acorde siguiente es
normalmente uno de los movimientos que las harmony keys sugieren
después del acorde anterior (la misma lista detrás de sus sugerencias
amarillas tenues, [Ahora y
después](../step-8/harmony-keys.md#ahora-y-después)), el movimiento más
habitual la mayoría de las veces. Una de cada cuatro veces es un acorde
cualquiera elegido en su lugar, para que también se oigan acordes a los
que los movimientos habituales rara vez llegan, como iii. Nunca toca el
mismo acorde dos veces seguidas, vuelve a I solo al final, y acaba en I
o en V: un punto final, o una semicadencia, una pregunta esperando su
respuesta.

**Qué toca la banda.** Cada acorde dura un compás de cuatro pulsos. El
piano lo toca en los pulsos 1 y 3, cada acorde con un voicing cercano
al anterior, como se mueven las manos de un pianista ([Tocar un acorde:
voice
leading](../step-8/harmony-keys.md#tocar-un-acorde-voice-leading)). El
bajo toca la fundamental en los mismos pulsos, en su octava grave. La
batería lleva el tiempo: bombo en 1 y 3, caja en 2 y 4, hi-hat en cada
corchea. El último acorde se mantiene todo el compás sobre un crash.
Con solo el **Piano**, el piano toca también la fundamental bajo el
acorde: sin bajo, un voicing cerrado podría oírse como una inversión.

**Sugerencias para escuchar.**

- **Sigue el bajo.** Toca las fundamentales, así que su línea *es* la
  progresión. Una fundamental que cae una quinta (de V a I, de ii a V)
  suena a llegada.
- **I y V son la pareja más fácil.** V es tensión, I es descanso; V
  casi pide I.
- **IV y ii llevan los dos lejos y hacia V.** Comparten dos notas (Fa La
  en Do mayor: Fa La Do y Re Fa La), así que suenan parecidos; ii es el más
  oscuro.
- **vi es un hogar triste.** Comparte dos notas con I (Do Mi Sol y La Do Mi),
  e iii también (Mi Sol Si). Si un acorde suena a hogar pero menor,
  probablemente es vi.
- Pulsa **Hear the key** siempre que pierdas la pista de dónde está el
  hogar.

## Cómo está construido

El trabajo se reparte por las líneas que la app ya tiene.
`lineup-core` hace las preguntas y sus explicaciones y no sabe nada de
ventanas ni de sonido. `lineup-audio` gana una manera de tocar unas
pocas notas aparte de la canción. `lineup-app` dibuja la ventana y une
las dos.

### Una pregunta a partir de un número

Toda pregunta sale de una **semilla** de 64 bits:

```rust
pub fn new(settings: &IntervalSettings, seed: u64) -> Self {
    let mut random = Random(seed);
    let intervals = settings.intervals.places(Interval::ALL.len());
    let directions = settings.directions.places(Direction::ALL.len());
    let interval = Interval::ALL[intervals[random.below(intervals.len())]];
    let direction = Direction::ALL[directions[random.below(directions.len())]];
    let low = Note::new(INTERVAL_LOWEST + random.below(INTERVAL_SPAN) as u8);
    IntervalQuestion { low, interval, direction }
}
```

`Random` es el generador SplitMix64 pequeño con el que ya barajan las
series dodecafónicas ([Series
dodecafónicas](../step-8/twelve-tone.md)): la misma semilla da siempre
los mismos números, así que la misma pregunta. Eso hace las pruebas
simples, porque pueden hacer miles de preguntas y comprobar cada una, y
una semilla que falla se puede reproducir exactamente. La ventana toma
su primera semilla del reloj y suma una constante impar grande por cada
pregunta nueva. Si la pregunta que recibe es la misma que la última, lo
intenta de nuevo, hasta ocho veces, para que rara vez salga la misma
pregunta dos veces seguidas.

### Un conjunto de respuestas en dieciséis bits

Qué intervalos o acordes se preguntan es un conjunto de lugares en una
lista. La lista es corta (doce intervalos como mucho), así que el
conjunto es un `u16`, un bit por lugar, igual que los conjuntos de
clases de alturas eran doce bits ([Conjuntos en
bits](../step-8/twelve-tone.md#conjuntos-en-bits)):

```rust
pub struct Choices(u16);

impl Choices {
    pub fn has(self, place: usize) -> bool {
        self.0 & (1 << place) != 0
    }

    /// Adds the answer at `place`, or takes it away. The last one left
    /// stays: there is always something to ask.
    pub fn toggle(&mut self, place: usize) {
        let toggled = self.0 ^ (1 << place);
        if toggled != 0 {
            self.0 = toggled;
        }
    }
}
```

`^` (o exclusivo) invierte un bit. Negarse a apagar el último significa
que siempre se puede preguntar algo, sin un caso especial en ningún
otro sitio.

### Cues: qué tocar, en segundos

Una pregunta no se toca sola: devuelve **cues**, valores planos que
dicen quién toca qué, cuándo y a qué volumen.

```rust
pub struct Cue {
    pub part: Part,      // Piano, Bass or Drum(Drum)
    pub start: f64,      // seconds from the start of the question
    pub length: f64,
    pub note: Note,
    pub velocity: u8,
}
```

Mantener el sonido fuera de `lineup-core` significa que las pruebas
pueden comprobar *qué* se tocaría sin tocarlo: que cada compás de una
progresión toca su acorde sobre su fundamental, que el piano solo añade
la fundamental, que la nota más grave de una pregunta de acordes es el
bajo que nombra su símbolo.

Las respuestas con las que comparar usan la misma idea.
`cues_for(interval)` toca otro intervalo desde la misma nota grave (o,
hacia abajo, hasta la misma nota aguda), y `cues_as(quality)` otro tipo
de acorde sobre la misma fundamental con la misma nota del acorde en el
bajo. Así, cuando haces clic en una respuesta equivocada después de
responder, oyes exactamente lo que dijiste junto a lo que sonó.

### Una frase en el hilo de audio

La canción está hecha de patrones y clips sobre una línea de tiempo.
Una pregunta no: son unas pocas notas que empiezan *ahora*, con
instrumentos propios, mientras los canales de la canción siguen como
están. Así que `lineup-audio` recibe un tipo nuevo, un `Phrase`: sus
instrumentos y sus notas, con sus tiempos en frames.

Como siempre, el hilo de audio no debe reservar ni liberar memoria ([El
hilo de audio, ya adulto](../step-2/realtime.md)). Así que el hilo de
la UI construye la frase entera, instrumentos incluidos, y la envía en
un solo comando:

```rust
pub enum Command {
    // ...
    PlayPhrase(Box<Phrase>),
    StopPhrase,
}
```

El `Box` importa. Un `Command` es tan grande como su variante más
grande, y una prueba los mantiene en 80 bytes o menos para que la cola
siga siendo pequeña. Un `Phrase` tiene dos `Vec`s y unos cuantos
contadores, unos 80 bytes él solo; dentro de un `Box`, el comando solo
lleva un puntero.

En el hilo de audio, la frase solo arranca cada nota cuando llega su
momento y añade sus instrumentos a la mezcla:

```rust
pub fn next_sample(&mut self) -> f32 {
    while let Some(note) = self.notes.get(self.next)
        && note.start <= self.clock
    {
        if let Some(instrument) = self.instruments.get_mut(note.instrument) {
            instrument.note_on(note.pitch, note.velocity, note.gate, self.sample_rate);
        }
        self.next += 1;
    }
    self.clock += 1;
    let mix: f32 = self.instruments.iter_mut().map(Instrument::next_sample).sum();
    mix * VOLUME
}
```

Las notas están ordenadas por comienzo, así que `next` solo avanza:
una comparación por muestra cuando no empieza nada. `while let ... &&
...` es una let chain en un loop: sigue mientras *hay* una nota
siguiente *y* le toca. A cada nota se le da su longitud (`gate`) cuando
empieza, y el instrumento la suelta por sí mismo, así que la frase no
necesita una lista de notas que detener.

Cuando llega una frase nueva, la vieja vuelve al hilo de la UI por la
cola de basura para ser liberada. Una frase que ha tocado su última
nota espera dos segundos, para que el sonido resuene, y luego va por el
mismo camino. **Stop** (cerrar la ventana) suelta todas las notas y
descarta las que aún no han empezado.

Los instrumentos vienen de `build_instrument`, la misma función que
usan los canales ([app/ear.rs](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/ear.rs)):
el piano es un canal con el programa General MIDI 0, el bajo el
programa 33, la batería el kit incorporado. Así que con **Edit >
Settings… > Audio > SoundFont instruments** activado, las preguntas
tocan el piano, el bajo y la batería del SoundFont, como la banda
([Sonidos de instrumentos reales](soundfont.md)).

### La ventana: acciones hacia fuera

La ventana no guarda el motor de audio. Devuelve lo que quiere oír
como una lista de acciones, y la app las reproduce:

```rust
pub enum EarAction {
    Play(Vec<Cue>),
    Stop,
}
```

Dentro, la pregunta y las respuestas dadas hasta ahora son un solo
enum, `Asked`. Dibujar las respuestas necesita cambiarlas *y* llamar a
métodos de la ventana (para sumar al marcador, para tocar un sonido),
lo que Rust no permite mientras las respuestas están prestadas fuera de
`self`. La ventana saca la pregunta de `self` durante el tiempo que se
dibuja, y la devuelve:

```rust
match self.asked.take() {
    Some(Asked::Interval { question, mut answer }) => {
        self.interval_answers(ui, &question, &mut answer, actions);
        self.asked = Some(Asked::Interval { question, answer });
    }
    // ... chords and progressions the same way
    None => {}
}
```

`Option::take` deja un `None` detrás, así que durante esas pocas líneas
`self` y la pregunta son dos valores separados, y ambos se pueden
prestar libremente.

Las opciones de la ventana se recuerdan como las de los ajustes del
SoundFont: un struct con `#[serde(default)]` guardado en la memoria de
egui, que eframe guarda junto con el tamaño de la ventana.

## Cómo se probó

El core tiene 21 pruebas. Las que protegen la teoría:

- **Todo intervalo desde toda nota.** Para las 12 notas graves y los 12
  intervalos, las dos notas escritas son las teclas correctas, ninguna
  necesita un doble sostenido o bemol, y el nombre sacado de las dos
  notas escritas (por `spelled_interval_name`, de sus nombres de nota y
  semitonos) es el nombre del intervalo; para el tritono, cuarta
  aumentada o quinta disminuida.
- **Escrituras a mano**: Do♯ Mi, Re♭ Fa, Fa Si, Mi La♯, Do Fa♯, Mi Do, Mi♭ Re, Fa♯ Fa♯,
  Si♭ Re♭, Si♭ Mi, La♭ Mi♭.
- **Acordes contra el analizador.** Quinientas preguntas de acordes en
  posición fundamental se le dan al analizador que la app usa para
  nombrar lo que tocas en un teclado MIDI, que debe nombrar el mismo
  tipo. (No invertidos: el analizador oye con razón Am7/C como C6.)
- **Toda pregunta de acordes toca su acorde con el bajo correcto**,
  sobre 2.000 semillas con las inversiones activadas.
- **Explicaciones escritas**: Bdim7, Ebmaj7/G, Fdim/Cb, el Si♭♭ de
  Cdim7, Caug/E (también Eaug), Ddim7/Cb (también Bdim7).
- Los **numerales** coinciden con las harmony keys en mayor y menor,
  tríadas y séptimas.
- Las **progresiones** empiezan en I, nunca repiten un acorde, vuelven
  a I solo al final, acaban en I o en V, y usan cada acorde elegido;
  cada compás toca su propio acorde sobre su fundamental.
- **Salen todas las tonalidades, fundamentales y notas graves**: 24
  tonalidades, 12 fundamentales, 16 notas graves.

Tres pruebas en el motor de audio comprueban que una frase suena sin
canción cargada y se tira después de resonar, que una frase nueva ocupa
el lugar de la última, y que una frase detenida no toca nada más. En la
app corriendo, una grabación de la salida tras **Start** contenía dos
notas, Re4 y luego Fa4: la tercera menor que preguntaba la ventana.

## Ejercicios

1. **Una melodía para la séptima mayor.** La tabla no tiene ninguna.
   Encuentra una canción cuya melodía empiece con una séptima mayor
   hacia arriba, compruébala en un piano, y añádela a `Interval::tune`.
   ¿La prueba `going_up_names_a_tune` necesita cambiar?
2. **Melodías descendentes.** Las melodías ayudan también hacia abajo:
   el comienzo de la Quinta Sinfonía de Beethoven, o «Swing Low, Sweet
   Chariot», para una tercera mayor hacia abajo. Añade un `tune_down` y
   úsalo en `explain` cuando la pregunta fue hacia abajo.
3. **Acordes de sexta, con justicia.** Los acordes de sexta se dejan
   fuera porque C6 y Am7 comparten sus teclas. Se podrían preguntar si
   el acorde siempre sonara sobre su fundamental en el bajo, una octava
   más abajo. ¿Qué notas daría entonces `notes_as`? ¿Las inversiones
   seguirían teniendo sentido?
4. **Lo que fallas.** El marcador cuenta las respuestas correctas.
   Guarda, para cada intervalo, cuántas veces se preguntó y cuántas se
   acertó, y pregunta más a menudo los flojos. ¿Dónde vivirían los
   conteos, y cómo los usaría `IntervalQuestion::new` sin perder la
   regla de «misma semilla, misma pregunta»?
5. **Tu propia progresión.** Que el ejercicio de progresiones toque los
   acordes del chord sheet de la canción, con los nombres ocultos, en
   lugar de uno inventado. ¿Qué necesitaría de `chord_sheet.rs`?
