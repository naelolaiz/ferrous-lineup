# Una banda más grande: guitarra, cuerdas y órgano

Archivos: [`crates/lineup-core/src/band/guitar.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/band/guitar.rs),
[`crates/lineup-core/src/band/extras.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/band/extras.rs),
[`crates/lineup-core/src/band.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/band.rs),
[`crates/lineup-core/src/swing.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/swing.rs)
y [`crates/lineup-core/src/file.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/file.rs)

Hasta ahora todos los estilos de banda tenían los mismos tres músicos:
un piano, un bajo y una batería. Eso es un trío de jazz, y la sección
rítmica de la mayoría de las bandas, pero una banda de rock sin su
guitarra o un tango sin sus violines no suena a sí misma. Ahora un
estilo puede traer hasta tres músicos más: una **guitarra**, unas
**cuerdas** y un **órgano**, cada uno tocando como la música los usa.

![El menú Band del chord sheet, abierto en «Reggae and ska». La pista sobre «One drop» dice: «Nada en el pulso 1: el kick y el rim caen juntos en el pulso 3. 4/4, unos 76 BPM. Con una guitarra en los pulsos 2 y 4 y un órgano burbujeando». Detrás del menú, la fila «You play» lista Piano, Bass, Drums, Guitar y Strings.](../images/step-9-bigger-band-styles.png)

## Cómo probarlo

Pulsa **Ctrl+N** para un chord sheet nuevo (la banda toca **Pop beat**)
y luego **Progression > Pop > Four chords**. Junto al piano, el bajo y
la batería, el playlist tiene una cuarta pista, **Guitar**, y el
channel rack un canal **Guitar** en la guitarra acústica de cuerdas de
acero. Con **Edit > Settings… > Audio > SoundFont instruments**
activado, suena como una.

Pasa el ratón sobre un estilo en el menú **Band**: su pista termina
ahora con quién más toca, como en «Con una guitarra rasgueada y cuerdas
en la variación b». Cuerdas en la variación b significa que esperan al
estribillo: haz clic derecho en un compás, elige **Variation b**, y
desde ahí entran las cuerdas, sosteniendo los acordes una octava por
encima del piano. En la fila **You play**, un músico que aún no toca
aparece en gris.

Prueba unos cuantos estilos sobre los mismos acordes:

| Estilo | Quién entra | Qué tocan |
|---|---|---|
| Pop beat | guitarra de cuerdas de acero, cuerdas en b | acordes rasgueados, cuerdas sostenidas |
| Rock beat | guitarra saturada, órgano de rock en b | power chords en corcheas |
| Shuffle, Rock and roll | guitarra eléctrica limpia | el boogie shuffle |
| Swing | guitarra de jazz | el four to the bar |
| Funk, Disco | guitarra muteada, y cuerdas en disco | semicorcheas, scratching |
| One drop, Steppers, Rocksteady | guitarra limpia, órgano | el chop en 2 y 4, la burbuja del órgano |
| Bossa nova | guitarra de cuerdas de nailon | el ritmo de bossa |
| Tangos | cuerdas | acordes sostenidos |
| Chorale | órgano de iglesia | acordes sostenidos |
| Hip hop, la mayoría de los estilos electrónicos | nadie | |

Cambia a un estilo sin guitarra y la pista y el canal de la guitarra
desaparecen; deshacer los trae de vuelta. Una parte que editaste en el
piano roll se queda, como siempre, hasta que haces clic en **Play it
again**. Las canciones guardadas con estos músicos son de formato 17;
una canción con solo el piano, el bajo y la batería se guarda aún como
formato 16, así que las versiones anteriores siguen abriéndola.

## La música: cómo toca un acorde un guitarrista

### Seis cuerdas y cuatro dedos

Un pianista puede tocar cualquier nota de un acorde: diez dedos, cada
tecla al alcance. Un guitarrista tiene seis cuerdas, afinadas desde la
más grave

| Cuerda | 6 | 5 | 4 | 3 | 2 | 1 |
|---|---|---|---|---|---|---|
| Nota al aire | Mi2 | La2 | Re3 | Sol3 | Si3 | Mi4 |
| MIDI | 40 | 45 | 50 | 55 | 59 | 64 |

y una mano izquierda con cuatro dedos para apretarlas contra los
**trastes**, un semitono cada uno. Cada cuerda toca una nota a la vez,
o ninguna. Una cuerda sin apretar suena **al aire**, gratis; por eso
las tonalidades de Mi, La, Re y Sol, cuyos acordes usan muchas cuerdas al
aire, son las tonalidades de casa de la guitarra.

Así que un acorde de guitarra es una **forma** (shape): un traste (o
nada) por cuerda. Los libros de acordes lo escriben como seis
caracteres desde la cuerda grave de Mi, con `x` para una cuerda que no
se toca y `0` para una al aire:

```text
C    x32010     la cuerda A en el traste 3 (C), D en el 2 (E), G al aire (G),
                B en el 1 (C), E aguda al aire (E): C E G C E
G    320003
Am   x02210
F    133211     el índice plano sobre el traste 1: una «ceja»
```

La cuerda más grave que suena debe ser el **bajo** del acorde, la nota
que toca el bajista (la fundamental, o, en un acorde con barra, la nota
después de la barra). Por eso la forma de Do deja fuera la cuerda grave
de Mi: un Mi ahí lo convertiría en C/E. Los libros de acordes sí escriben
C/E así, `032010`.

### Encontrar la forma

Ferrous Lineup no guarda un libro de acordes. Encuentra las formas como
las aprende un guitarrista: probando los trastes al alcance y
quedándose con los más fáciles. Para cada **posición** del mástil (el
traste en el que se sienta el dedo índice), prueba todas las maneras de
poner una nota del acorde en cada cuerda dentro de cuatro trastes, se
queda con las que una mano puede tocar y las puntúa:

- toda nota **necesaria** debe estar: la fundamental, la tercera (o la
  segunda o la cuarta de un acorde sus), la sexta o la séptima, una
  quinta que no sea justa, y la extensión más alta, que da nombre al
  acorde (la 13 de un G13);
- una quinta justa y las extensiones más bajas son **bienvenidas**,
  y las formas con más de ellas son mejores;
- las formas bajas del mástil, con **cuerdas al aire**, y en más
  cuerdas son mejores;
- hasta cuatro cuerdas apretadas está bien. Más necesitan una **ceja**,
  un dedo plano sobre el traste más bajo, que pisa cada cuerda por
  encima de él, así que ninguna de esas puede estar al aire, y deja
  tres dedos para el resto.

Una regla más mantiene el sonido claro. Dos notas juntas suenan
embarradas abajo: una tercera entre Mi2 y Sol♯2 retumba, mientras que una
quinta ahí está bien. Los arreglistas tienen una tabla de la nota más
grave desde la que cada intervalo suena claro, los **límites graves de
los intervalos**, y una forma que los baja cuesta un poco más. Un poco,
no mucho: el Sol al aire, `320003`, empieza con Sol2 y Si2, una tercera por
debajo de su límite (Si♭2), y sigue siendo el Sol que toca todo
guitarrista. Su sonido es parte del sonido de la guitarra.

Con esas reglas, la búsqueda encuentra los acordes al aire de los
libros de acordes (C `x32010`, G `320003`, D `xx0232`, Am `x02210`, G7
`320001`, Cmaj7 `x32000`), los acordes con ceja en otras tonalidades (F
`133211`, la forma de E al aire un traste más arriba, y B `x24442`, la
forma de A), y una forma para cada tipo de acorde en cada tonalidad.
Las pruebas las comprueban todas.

### Abajo y arriba

Un guitarrista no golpea las cuerdas a la vez. La púa las barre: en un
**rasgueo hacia abajo** de las cuerdas graves a las agudas, y en uno
**hacia arriba** de vuelta desde las agudas, y la mano que sube rara
vez llega a las graves. Los ritmos de la biblioteca ya marcan qué hits
son los ligeros, los de «arriba», entre los pulsos. La guitarra toca
cada rasgueo hacia abajo en todas las cuerdas, de grave a aguda, a un
tick de distancia (unos 5 ms a 120 BPM, así que un rasgueo de seis
cuerdas tarda unos 25 ms, como uno real), y cada rasgueo hacia arriba
en las cuatro de arriba, de aguda a grave. Cada cuerda se detiene donde
se detiene el hit.

### Qué toca el guitarrista de cada estilo

- **Acordes rasgueados** (pop, folk, country): la forma completa en el
  ritmo del estilo.
- **Power chords** (rock, punk, metal): solo la fundamental, su quinta
  y la octava, `A5` = La2 Mi3 La3. La distorsión añade notas nuevas en las
  sumas y diferencias de las frecuencias tocadas. Para una quinta,
  cuyas frecuencias están cerca de 3:2, caen en línea con la
  fundamental, una o dos octavas por debajo; para una tercera caen en
  notas desafinadas con el acorde, y zumba y se agria. Así que los
  guitarristas de rock dejan la tercera al bajo y al cantante, y «La5»
  significa «La sin su tercera».
- **Four to the bar** (swing): acordes de tres notas, la fundamental,
  la tercera y la séptima, golpeados en cada pulso, el segundo y el
  cuarto un poco más fuerte. Así llevaba el tiempo Freddie Green en la
  banda de Count Basie durante casi cincuenta años. Las formas son los
  **shells** de jazz: desde una fundamental en la cuerda grave de Mi, la
  séptima viene a continuación y la tercera una octava arriba (G7 = Sol2
  Fa3 Si3); desde la cuerda de La, la tercera y luego la séptima (C7 = Do3
  Mi3 Si♭3).
- **El boogie shuffle** (blues, rock and roll): solo dos cuerdas, la
  fundamental con su quinta en un pulso y con su sexta en el siguiente,
  ambas corcheas de cada pulso: La y Mi, La y Mi, La y Fa♯, La y Fa♯. Es la
  parte de guitarra de incontables discos de blues y de los primeros
  rock and roll. Sobre un acorde menor se queda en la quinta.
- **El chop** (reggae, soul, Motown, bluegrass): un acorde corto en las
  cuatro cuerdas de arriba en los pulsos 2 y 4, con la snare. Las
  cuerdas graves se dejan al bajo. El country llama a lo mismo el
  «chick» del «boom-chick»: el bajo toca «boom» en el 1 y el 3, la
  guitarra «chick» en el 2 y el 4. En el **ska** la guitarra toca entre
  todos los pulsos, en su lugar.
- **El scratching** (funk, disco): semicorcheas en una guitarra
  muteada, algunas acentuadas, la mayoría solo un «chk» percusivo.

### Cuerdas y órgano

Las **cuerdas** sostienen cada acorde mientras dure, una octava por
encima del piano, suavemente: un **pad**, un cojín de sonido bajo la
banda. El pop las usa para levantar el estribillo, así que en Pop beat
y Motown solo tocan en la variación b. Las baladas usan las cuerdas de
arranque más lento, y los estilos de sintetizador, las de sintetizador.

El **órgano** también sostiene acordes, como un órgano de iglesia en un
chorale o un Hammond en un blues lento. En el reggae toca la
**burbuja**: acordes cortos entre todos los pulsos abajo, y en los
pulsos 2 y 4 una octava más arriba, con el chop de la guitarra. En un
rock beat espera a la variación b, como un órgano rellena un estribillo.

### Qué instrumento

Cada músico toca un instrumento General MIDI (mira [Cada voz en su
propio canal, con un instrumento real](../step-8/voices-and-instruments.md)),
elegido por el estilo. La tabla los numera desde 1, y los mensajes MIDI
desde 0, así que el código dice 24 para la guitarra de nailon:

| Programa (desde 0) | Instrumento | Estilos |
|---|---|---|
| 24 | Guitarra acústica (nailon) | bossa nova, samba, bolero, folklore, rumba flamenca |
| 25 | Guitarra acústica (acero) | pop, folk, country, bluegrass, waltz |
| 26 | Guitarra eléctrica (jazz) | swing |
| 27 | Guitarra eléctrica (limpia) | shuffle, rock and roll, surf, Motown, reggae, ska |
| 28 | Guitarra eléctrica (muteada) | funk, disco |
| 29, 30 | Guitarra saturada, con distorsión | rock, punk, metal |
| 48, 49, 50 | Cuerdas, cuerdas lentas, cuerdas de sintetizador | pop, tango, baladas, R&Si, trance |
| 16, 17, 18, 19 | Órgano drawbar, percusivo, de rock, de iglesia | blues, reggae, rock, chorale |

## Cómo funciona

### Seis músicos en lugar de tres

`Player` gana tres variantes más, y la banda guarda un canal y una
huella por músico en arrays indexados por ellas:

```rust
pub const COUNT: usize = 6;
pub const ALL: [Player; Player::COUNT] = [
    Player::Piano, Player::Bass, Player::Drums,
    Player::Guitar, Player::Strings, Player::Organ,
];

pub(crate) channels: [Option<usize>; Player::COUNT],
pub(crate) written: [Option<u64>; Player::COUNT],
```

Antes, la banda tenía un campo para cada uno, `piano` y `bass`, y el
código que preguntaba «¿qué canal?» hacía match sobre el músico para
elegir el campo. Con un array, `band.channels[player.index()]` responde
por cualquier músico, y un séptimo músico solo necesitaría un `COUNT`
más grande. La longitud del array es parte de su tipo,
`[Option<usize>; 6]`, así que el compilador comprueba que `ALL` lista
de verdad `COUNT` músicos.

`write_band`, que escribe la parte de cada músico, ya iteraba sobre
`Player::ALL`. Gana tres brazos más en su `match player`, y todo lo que
viene después —los toques humanos de `interpret::perform`, el «más
fuerte en b», los patrones, las pistas y las huellas que mantienen
congelada una parte editada— funciona para los músicos nuevos sin
cambios.

### Quién toca en cada estilo: una tabla de constantes

Los músicos de cada estilo son un valor de este tipo:

```rust
pub struct Extra<Part> {
    pub part: Part,
    pub program: u8,
    pub only_in_b: bool,
}

pub struct Extras {
    pub guitar: Option<Extra<GuitarPart>>,
    pub strings: Option<Extra<PadPart>>,
    pub organ: Option<Extra<PadPart>>,
}
```

`Extra` es **genérico**: `Part` es un parámetro de tipo, que se rellena
como `GuitarPart` para la guitarra y `PadPart` para las cuerdas y el
órgano. Los tres campos «qué toca, en qué instrumento, espera a la b»
se escriben una sola vez para ambos.

Los estilos se listan en una tabla que existe cuando el programa
arranca, construida por el compilador:

```rust
const NONE: Extras = Extras { guitar: None, strings: None, organ: None };

const fn with_guitar(part: GuitarPart, program: u8) -> Extras {
    Extras { guitar: guitar(part, program), ..NONE }
}

const BY_STYLE: &[(&str, Extras)] = &[
    ("Pop beat", Extras {
        guitar: guitar(STRUMMED, STEEL_GUITAR),
        strings: pad_in_b(STRINGS),
        ..NONE
    }),
    ("Bossa nova", with_guitar(GuitarPart::Strum(Comping::Bossa), NYLON_GUITAR)),
    // ...
];
```

Un **`const fn`** se puede llamar donde Rust necesita un valor en
tiempo de compilación, como en un `const`. `..NONE` es **sintaxis de
actualización de struct**: los campos que no se escriben toman sus
valores de `NONE`, así que cada estilo nombra solo los músicos que
tiene. Una prueba comprueba que cada nombre de la tabla es un estilo de
banda real, de modo que una errata no puede dejar en silencio a un
estilo sin su guitarra.

### Enums que llevan datos

Lo que toca la guitarra es un enum cuyas variantes pueden llevar un
valor:

```rust
pub enum GuitarPart {
    Strum(Comping),   // full chords in this rhythm
    StylesRhythm,     // full chords, in the rhythm of the style's piano
    Chop,             // the top four strings on 2 and 4
    Power(Comping),   // power chords in this rhythm
    FourToTheBar,
    Boogie,
}
```

`Strum(Comping::Bossa)` y `Strum(Comping::Waltz)` son una misma
variante con ritmos distintos dentro. Un `match` puede mirar dentro, y
los patrones `|` se pueden anidar, como en la variación b, donde los
acordes sostenidos y arpegiados se vuelven rasgueados:

```rust
match (variation, self) {
    (Variation::B, GuitarPart::Power(Comping::Held)) => GuitarPart::Power(Comping::Pulse),
    (Variation::B, GuitarPart::Strum(Comping::Held | Comping::ArpUp | Comping::ArpUpDown)) =>
        GuitarPart::Strum(Comping::Strum),
    (_, part) => part,
}
```

### Una función como parámetro

Los acordes rasgueados, los power chords y los shells solo difieren en
qué notas recibe cada armonía. El resto, tocar el ritmo y rasguearlo,
es igual, así que `strummed` recibe el voicing como parámetro:

```rust
fn strummed(
    rhythm: Comping,
    bars: &[Bar],
    harmonies: &[Played],
    voice: fn(&Played) -> Vec<Note>,
) -> Vec<NoteEvent>
```

`fn(&Played) -> Vec<Note>` es un tipo de **puntero a función**:
cualquier función simple con esa firma sirve, así que `play` pasa
`shape`, `power_chord`, `shell` o `top_strings` por su nombre. Encontrar
una forma prueba miles de combinaciones de trastes, y una canción
repite sus acordes, así que `strummed` recuerda las formas que
encontró en un `HashMap`:

```rust
let voicing = shapes.entry(key).or_insert_with(|| voice(harmony)).clone();
```

`entry` encuentra el sitio de `key` en el mapa, y `or_insert_with`
llama al closure solo si ese sitio está vacío. Cada acorde distinto se
busca una vez.

### Probar cada traste: recursión con un callback

Cuántas cuerdas usa una forma no se sabe de antemano, así que la
búsqueda no puede ser un número fijo de bucles `for` anidados. Es una
función que elige un traste para una cuerda y se llama a sí misma para
la siguiente:

```rust
fn try_frets(
    choices: &[Vec<u8>],
    low: usize,
    top: usize,
    frets: &mut Vec<u8>,
    found: &mut impl FnMut(&[u8]),
) {
    let string = low + frets.len();
    if string > top {
        found(frets);
        return;
    }
    for &fret in &choices[string] {
        frets.push(fret);
        try_frets(choices, low, top, frets, found);
        frets.pop();
    }
}
```

`frets` es un único `Vec` compartido por todos los niveles: un nivel
apila el traste de su cuerda, deja que los niveles más profundos
terminen y lo desapila antes de probar el siguiente. Cuando cada cuerda
tiene traste, `found` recibe la forma completa. `found` es un closure
que la puntúa y se queda con la mejor, así que cambia algo fuera de sí
mismo: eso lo convierte en un **`FnMut`**, y se pasa como `&mut` para
que cada nivel de la recursión pueda llamar al mismo closure, en lugar
de que cada uno se lleve su propia copia.

### Rasgueos y swing

Un rasgueo arranca sus cuerdas a un tick de distancia, así que un
rasgueo en la segunda corchea de un pulso empieza en los ticks 48, 49,
50 y así. El swing ([Swing](swing.md)) movía solo las notas exactamente
en el tick 48: habría tocado la primera cuerda tarde y habría dejado a
las demás donde estaban, desgarrando el rasgueo. El swing mueve ahora
toda nota a menos de 6 ticks (una semifusa) del contratiempo, lo que
también les va bien a las notas tocadas a mano, rara vez exactamente a
tiempo.

### Guardar: formato 17, solo cuando hace falta

La parte de la banda del archivo conservó sus campos viejos y ganó unos
opcionales:

```rust
#[serde(default, skip_serializing_if = "Option::is_none")]
pub guitar: Option<usize>,
// strings, organ the same
pub written: [Option<u64>; 3],          // piano, bass, drums, as before
#[serde(default, skip_serializing_if = "none_written")]
pub written_too: [Option<u64>; 3],      // guitar, strings, organ
```

`skip_serializing_if` deja un campo fuera del archivo cuando la función
nombrada lo dice, y `default` lo rellena cuando un archivo no lo tiene.
Una banda de tres se escribe por tanto exactamente como antes, y el
archivo dice formato 16. Solo una canción cuya banda tiene guitarra,
cuerdas u órgano es de formato 17: una versión anterior la rechaza con
un mensaje en lugar de dejar caer en silencio el canal de la guitarra.

## Ejercicios

1. Crea un chord sheet con **C Am F G** y prueba **Pop beat**, **Folk**,
   **Rock beat** y **Swing**. Haz clic en **Piano**, **Bass** y
   **Drums** en la fila **You play**, de modo que solo quede la
   guitarra. ¿Cómo toca cada guitarrista los mismos cuatro acordes?
2. Mira las notas de la guitarra en el piano roll para Sol y para Fa en
   Pop beat. Escribe sus formas como hacen los libros de acordes
   (`320003`). ¿Por qué la forma de Fa no tiene cuerdas al aire?
3. ¿Qué forma tocarías para **E7**, y qué notas del acorde dobla?
   Comprueba tu respuesta con la prueba
   `open_chords_are_the_ones_in_chord_books`.
4. En `extras.rs`, cambia la guitarra de Rock beat de
   `GuitarPart::Power(Comping::Pulse)` a
   `GuitarPart::Strum(Comping::Pulse)`: formas completas a través de la
   guitarra saturada. Escucha con un SoundFont. ¿Por qué los
   guitarristas de rock evitan esto?
5. En el boogie shuffle, la sexta sobre un acorde mayor hace que la
   línea se balancee. ¿Cómo sonaría una sexta menor (8 semitonos) sobre
   un acorde menor, y por qué es arriesgada? (Pista: ¿qué acorde son La,
   Do, Mi y Fa?)
6. (Rust) Dale a **Lo-fi** una guitarra de nailon que arpegie los
   acordes, `GuitarPart::Strum(Comping::ArpUp)`, en `BY_STYLE`. Luego
   escribe mal el nombre del estilo: ¿qué prueba falla, y qué imprime?
7. (Rust) `try_frets` recibe `found: &mut impl FnMut(&[u8])`. Prueba
   `found: impl FnMut(&[u8])` en su lugar. ¿Qué dice el compilador
   sobre la llamada recursiva, y por qué?
8. (Rust, más difícil) Añade un **segundo teclado**: un piano eléctrico
   (programa 4) que toca el comping del estilo una octava arriba en los
   estilos R&Si y Neo soul. ¿Qué partes de `band.rs` y `file.rs` deben
   cambiar? Comprueba que una canción con solo tres músicos se sigue
   guardando como formato 16.
