# Swing: corcheas tocadas largo-corto

Archivos: [`crates/lineup-core/src/swing.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/swing.rs),
[`crates/lineup-core/src/diff.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/diff.rs),
[`crates/lineup-core/src/library/rhythm.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/rhythm.rs),
[`crates/lineup-app/src/app/swing.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/swing.rs)
y [`crates/lineup-app/src/playlist.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/playlist.rs)

Da palmas con un disco de jazz o de blues y di «da-da da-da» con las
corcheas. No encajará: la banda toca «du-ba du-ba», una nota larga y una
corta, en cada pulso. Esa desigualdad es el **swing**, y es la razón de
que un shuffle rebote y una canción pop no. Hasta ahora Ferrous Lineup
tocaba cada pareja de corcheas igual, o, en los ritmos con swing de su
biblioteca, como una corchea con puntillo y una semicorchea, que es
demasiado entrecortado. Ahora una canción tiene una cantidad de **Swing**
única, desde parejo («straight») hasta un swing de tresillo completo.

![El chord sheet con la banda tocando Swing a 110 BPM. Junto al tempo, «Swing 100%», con su ayuda: «Swing: the second eighth note of every beat comes late, so pairs of eighths are played long-short, as in jazz, blues and shuffles. "straight" plays them even, 50% is a light swing and 100% a triplet: two thirds and one third. Each style remembers what you set.»](../images/step-9-swing-sheet.png)

## Probarlo

Abre **Archivo > Demostraciones > Canciones > Blues > Midnight Shuffle**. La lista de reproducción
dice **Swing 100%** junto a **Soltura**. Arrastra el número hacia abajo
hasta *straight* mientras suena, y luego hacia arriba: la batería, el
piano, el bajo y la armónica cambian todos su sensación a la vez, pero
ni una sola nota se mueve en el piano roll. Con una banda, el mismo
control está junto al tempo en la hoja de acordes.

Cada estilo de banda trae su propio swing: **Swing**, **Balada de jazz**,
**Vals de jazz** y **Shuffle** al 100%, **Rock and roll** al 50%, y todos
los demás estilos straight. Cámbialo, y el estilo lo recuerda: la
próxima vez que lo elijas, en cualquier canción, toca con tu cantidad.
La cantidad también forma parte de la canción, así que se guarda con
ella, deshacer la devuelve, y las exportaciones WAV y MIDI la tocan.

## La música: ¿cuánto dura la nota larga?

El swing trata de las dos corcheas de un pulso. Tocadas straight, cada
una dura medio pulso. Tocadas con swing, la primera se alarga y la
segunda empieza más tarde y es más corta. Cuánto más larga es la
pregunta entera:

| Swing | Las dos corcheas | Proporción | Segunda corchea a 120 BPM |
|---|---|---|---|
| straight | 1/2 + 1/2 del pulso | 1 : 1 | 250 ms después del pulso |
| 50% | 7/12 + 5/12 | 7 : 5 | 292 ms |
| 100% | 2/3 + 1/3: un tresillo | 2 : 1 | 333 ms |
| (una corchea con puntillo y una semicorchea) | 3/4 + 1/4 | 3 : 1 | 375 ms |

La fila del 100% es como los libros de texto explican el swing: divide
el pulso en tres (un **tresillo**) y toca las dos primeras partes como
una sola nota. Por eso los charts de jazz escriben corcheas normales con
«Swing» sobre el primer compás, o un signo pequeño que muestra dos
corcheas iguales a la negra y la corchea de un tresillo. Escribir cada
pareja con swing como tresillo llenaría la página de corchetes y de 3, y
los músicos saben lo que significa «Swing».

Los intérpretes reales no se ciñen a una proporción. Las mediciones de
bateristas de jazz muestran que tocan con más swing en tempos lentos y
se igualan en los rápidos, donde no queda tiempo para una pareja
larga-corta. Una balada puede pasar de 2:1, y una línea de bebop muy
rápida es casi straight. El control te deja probar todo el rango.

La última fila es lo que tocaba Ferrous Lineup antes. Una **corchea con
puntillo y una semicorchea** es lo más cerca del swing que puede llegar
una cuadrícula de semicorcheas, pero es más dura que un tresillo y suena
rígida, como una marcha. También es un ritmo real por sí mismo: el
«Hap-py» de Happy Birthday está escrito así, y aquí sigue así, porque el
swing solo mueve las notas que caen en la segunda corchea de un pulso (o
a unos pocos ticks de ella, mira más abajo).

Un **shuffle** es el nombre que da el blues a la misma sensación
larga-corta, normalmente un tresillo completo, con el hi-hat del
baterista o los acordes de la guitarra machacando las dos notas. Algunos
blues se escriben en **12/8** (*Trouble in Mind* en las demos): cuatro
pulsos de tres corcheas cada uno, la sensación de tresillo escrita tal
cual. Por eso el control está deshabilitado en 6/8, 9/8 o 12/8: los
pulsos ya se dividen en tres, así que no hay parejas a las que aplicar
swing. Lo mismo vale para 5/8 o 7/8, cuyos pulsos son las propias
corcheas. Funciona en todo compás contado en negras o más largo, como
4/4, 3/4 o 2/2.

El hip hop, el funk y el «new jack swing» tocan con swing sus
**semicorcheas**: las parejas dentro de cada corchea. Este control solo
aplica swing a las corcheas.

## Cómo funciona

### Un número, y las notas tal como se tocan

El proyecto tiene un campo más, junto a `loose_timing`:

```rust
pub swing: u8, // 0 to MAX_SWING (100)
```

Las notas de los patrones no cambian nunca. En su lugar, todo lo que
*toca* la canción pide las notas tal como se tocan:

```rust
pub fn played_notes(&self, pattern: usize, channel: usize) -> Cow<'_, [NoteEvent]> {
    let Some(written) = self.pattern(pattern).map(|p| p.notes(channel)) else {
        return Cow::Borrowed(&[]);
    };
    match self.pattern_swing(pattern) {
        0 => Cow::Borrowed(written),
        swing => Cow::Owned(swung(written, swing, self.plays_hits(channel))),
    }
}
```

`Cow` es «clone on write» de la biblioteca estándar: un valor que está
**prestado** (borrowed) u **poseído** (owned). La mayoría de las
canciones no tocan con swing, y para ellas `played_notes` devuelve la
propia lista del patrón sin copiar una sola nota. Solo un patrón con
swing recibe un `Vec` nuevo. Las dos clases se leen igual, como slice, y
quien llama y necesita un `Vec` propio llama a `.into_owned()`, que
copia solo si estaba prestado.

El `'_` de `Cow<'_, ...>` dice que el caso prestado toma el préstamo de
`self`: el resultado no puede sobrevivir al proyecto del que salió.

El hilo de audio obtiene sus notas de `played_notes` (cuando se carga un
proyecto y cada vez que cambian las notas), y lo mismo hacen la
exportación MIDI y el acompañamiento que sigue los acordes que tocas. El
piano roll, los botones de step y la hoja de acordes leen las notas
escritas, de modo que la cuadrícula que editas sigue siendo una
cuadrícula.

### Mover la segunda corchea

Ferrous Lineup cuenta el tiempo en ticks, 24 por step y 96 por negra. La
segunda corchea de un pulso empieza en el tick 48; la tercera nota de un
tresillo empieza en el tick 64. Así que el swing completo llega 16 ticks
tarde:

```rust
const BEAT: usize = 4 * TICKS_PER_STEP;              // 96
const OFFBEAT: usize = BEAT / 2;                       // 48
const FULL_DELAY: usize = BEAT * 2 / 3 - OFFBEAT;      // 16

pub fn delay(swing: u8) -> usize {
    let swing = usize::from(swing.min(MAX_SWING));
    (FULL_DELAY * swing + usize::from(MAX_SWING) / 2) / usize::from(MAX_SWING)
}
```

Sumar la mitad de 100 antes de dividir por 100 redondea al tick más
cercano en lugar de siempre hacia abajo: 50% son 8 ticks exactos, 70% es
11,2 y se vuelve 11.

`swung` mueve entonces todo inicio y final que cae en un offbeat. La
primera versión hacía match exacto:

```rust
let played = |tick: usize| match tick % BEAT {
    OFFBEAT => tick + late,
    _ => tick,
};
```

`OFFBEAT` en un brazo de `match` es una constante, así que el brazo
compara con su valor, 48. Si hubiera sido un nombre en minúscula como
`offbeat`, Rust lo leería como una *variable nueva* que coincide con
cualquier cosa, y el brazo `_` nunca se ejecutaría. (El compilador avisa
exactamente de eso, y la convención de nombres, mayúsculas para las
constantes, mantiene separadas las dos cosas.)

La guitarra de la banda necesitó más tarde que «en el offbeat»
significara «cerca de él»: un rasgueo arranca sus cuerdas a un tick de
distancia, 48, 49, 50..., y solo la primera cuerda se habría movido
([Una banda más grande](bigger-band.md#rasgueos-y-swing)). El brazo es
ahora un **match guard**:

```rust
const NEAR: usize = BEAT / 16; // 6 ticks, a sixty-fourth note

let played = |tick: usize| match tick % BEAT {
    offset if offset.abs_diff(OFFBEAT) < NEAR => tick + late,
    _ => tick,
};
```

Aquí `offset` *es* una variable nueva que coincide con cualquier cosa, a
propósito, y el `if` que viene después decide si el brazo se aplica.
`abs_diff` da la distancia entre dos números sea cual sea el mayor:
`offset - OFFBEAT` con un `usize` por debajo de 48 haría underflow y
panic en una compilación de depuración.

Una nota que acaba en el offbeat acaba también más tarde, así que la
primera corchea es de verdad larga, legato hacia la segunda. Un golpe de
batería es distinto: una batería sampleada suena hasta su final dure lo
que dure la nota sostenida, así que en los canales de batería solo se
mueven los inicios. La nota se reconstruye entonces desde sus ticks
nuevos con `NoteEvent::exact`, la función que usa la importación MIDI
para las notas entre steps: la corchea con swing del step 2 se vuelve el
step 2 con un `nudge` de 16 ticks, que el hilo de audio ya sabía tocar
tarde.

### Ningún comando nuevo para el hilo de audio

El hilo de audio se entera de cada cambio por
[`diff`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/diff.rs),
que compara el proyecto antes y después de una edición. Ahora compara
las notas *tocadas*:

```rust
let old_notes = match old_index {
    Some(old) => before.played_notes(old, old_channel),
    None => Cow::Borrowed(&[][..]),
};
if old_notes != after.played_notes(index, channel) {
    changes.push(Change::Notes { pattern: index, channel });
}
```

Cambiar el swing no necesita por tanto un comando propio: los patrones
cuyas notas se mueven se envían otra vez, y los demás se quedan como
están. Lo mismo vale para un patrón movido a 6/8, que deja de tocar con
swing. Y deshacer, que cambia por un proyecto anterior entero, no es más
que otro diff.

### La biblioteca, reescrita en corcheas iguales

Los ritmos con swing de la biblioteca estaban escritos de la única
manera que permitía una cuadrícula de semicorcheas, como parejas con
puntillo:

```text
Swing ride   "x... x..x x... x..x"     (antes)
Swing ride   "x... x.x. x... x.x."     (ahora, con .swung(100))
```

Ahora están escritos como los escribe la partitura, en corcheas iguales,
y un ritmo lleva la cantidad con la que toca swing, `Rhythm::swing`. La
banda, un ritmo o estilo elegido de la biblioteca, y una demo dan todos
a la canción el swing del ritmo que tocan. Esto arregló también un
choque real: en las demos de blues, el bajo boogie tocaba corcheas
iguales debajo de las parejas con puntillo de la batería, dos
sensaciones distintas a la vez. Ahora todo toca con swing a la vez.

Dos demos escritas para Ferrous Lineup, *Midnight Shuffle* y *Rolling
Boogie*, tenían sus melodías en parejas con puntillo por la misma razón;
ahora son corcheas iguales. Las melodías tomadas de partituras conservan
los ritmos que imprime su partitura. Las canciones folk y clásicas
conservan las corcheas iguales sea cual sea el pulso que haya debajo,
así que *Scarborough Fair*, tocada sobre el patrón de ride de un vals de
jazz, no se convierte en jazz.

### Recordada por estilo

La app guarda las cantidades que fijas, por nombre de estilo, en un
`BTreeMap`:

```rust
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
struct Remembered(BTreeMap<String, u8>);
```

egui guarda los ajustes entre ejecuciones por un `Id` *y un tipo*, así
que el mapa se envuelve en un struct pequeño propio, un **newtype**: un
`BTreeMap<String, u8>` guardado por otra parte de la app nunca podría
confundirse con este. Un `BTreeMap` mantiene sus claves ordenadas, así
que el archivo de ajustes lista los estilos en el mismo orden cada vez.

Cuando eliges un estilo de banda, un ritmo o un estilo de la biblioteca,
la canción toma primero el swing propio del estilo (en `lineup-core`,
que no sabe nada de ajustes), y luego la app lo sustituye por la
cantidad recordada, si la hay, en el mismo paso de deshacer.

## Ejercicios

1. Abre *Midnight Shuffle* y tócala en *straight*, 50% y 100%. ¿Qué
   instrumento muestra más el cambio? ¿Qué ajuste suena como una marcha
   y cuál como un shuffle?
2. A 96 BPM, ¿cuántos milisegundos después del pulso empieza la segunda
   corchea en straight, 50% y 100%? Comprueba tu respuesta del 100%
   contra la tabla: ¿por qué aquí no son 333 ms?
3. En una canción con swing al 100%, escribe en el piano roll una
   corchea con puntillo y una semicorchea, y luego dos corcheas. ¿Qué
   pareja se mueve? Muchos músicos de jazz leen las dos de la misma
   manera, como un tresillo. ¿Qué necesitaría saber Ferrous Lineup para
   hacer eso, y por qué podría salir mal en una marcha?
4. Pon una canción con swing en 6/8 con el cuadro Compás. ¿Qué les pasa al
   control Swing y al sonido? Escribe el mismo groove en 12/8 a mano:
   ¿dónde van las notas largas y las cortas?
5. (Rust) `played_notes` devuelve un `Cow`. ¿Qué cambiaría si devolviera
   un `Vec<NoteEvent>`? Cuenta cuántas notas copiaría `diff` para una
   canción de 20 patrones y 8 canales, en cada edición.
6. (Rust) Escribe una prueba de que una nota sostenida desde la segunda
   semicorchea de un pulso (step 1) hasta el offbeat acaba 16 ticks más
   tarde con swing completo en un canal de piano, pero no en un canal de
   batería.
7. (Rust, más difícil) Añade swing de semicorcheas: qué ticks de un
   pulso se mueven, cuánto con swing completo, y qué debe decir
   `swings_in` sobre 6/8, donde las semicorcheas también vienen en
   parejas.
