# Clases de altura, notas y su escritura

Archivos: [`crates/lineup-core/src/pitch.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/pitch.rs)
y [`crates/lineup-core/src/harmony/pitch_set.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/harmony/pitch_set.rs)

Esta parte del libro trata de la teoría musical que hay dentro del
programa: escalas, acordes, números romanos, conjuntos, series y las
reglas de la conducción de voces, cada una en unos pocos tipos pequeños
de Rust con métodos, probados como cualquier otro código. La teoría en
sí se enseña en los capítulos de [Aprender
armonía](https://naelolaiz.github.io/ferrous-lineup/es/manual/notes-and-scales.html)
del manual de usuario; cada capítulo de aquí cuenta la idea musical en
pocas líneas y luego muestra cómo la guarda el código.

Todo lo demás se construye sobre los tipos de este capítulo: una nota,
las doce teclas de una octava, el nombre con que se escribe una tecla y
un conjunto de teclas.

## La música, en pocas palabras

Un piano tiene 88 teclas, pero solo doce notas distintas, que se
repiten en cada octava. La distancia de una tecla a la siguiente,
blanca o negra, es un **semitono**:

```text
 Do Do♯ Re Re♯ Mi Fa Fa♯ Sol Sol♯ La La♯ Si Do
 0  1   2  3   4  5  6   7   8    9  10  11 12
```

El nombre de una nota sin su octava es su **clase de altura**: todos los
Do del piano tienen la clase de altura Do. La armonía trabaja sobre todo
con clases de altura; un acorde de Do mayor es Do, Mi y Sol, en
cualquier octava y en cualquier orden. La octava se escribe después del
nombre: **Do4** es el Do central y **La4** el La con el que afina una
orquesta, 440 Hz.

Cada tecla negra tiene dos nombres: la tecla entre Do y Re es Do♯ y
también Re♭. Qué nombre es el correcto depende de la música que la
rodea, y la regla es que una escala usa cada uno de los siete nombres
una vez. Fa mayor tiene un Si♭, nunca un La♯, porque ya tiene un La. La
séptima nota de Fa♯ mayor es Mi♯, la tecla que suele llamarse Fa,
porque Fa♯ ya usa el nombre Fa. De vez en cuando una nota necesita dos
signos: la séptima de Sol♯ menor armónica es Fa𝄪 (Fa doble sostenido,
la tecla que suele llamarse Sol), y la séptima de un acorde de séptima
disminuida sobre Do es Si𝄫 (Si doble bemol, que suena como La). El
capítulo [Notas, intervalos y
escalas](https://naelolaiz.github.io/ferrous-lineup/es/manual/notes-and-scales.html)
del manual lo cuenta con más calma.

Así que el código necesita dos cosas distintas: qué **tecla** del piano
es una nota, y cómo se **llama**. Son tipos distintos.

## Doce clases de altura: un enum

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum PitchClass {
    C,
    CSharp,
    D,
    DSharp,
    E,
    F,
    FSharp,
    G,
    GSharp,
    A,
    ASharp,
    B,
}
```

Hay exactamente doce clases de altura, así que un enum encaja: un
`PitchClass` nunca puede ser 13 ni -1. Las variantes no llevan datos (un
enum **sin campos**), y Rust las numera desde 0 en el orden en que están
escritas. La palabra clave `as` convierte un enum así en su número, que
es la cantidad de semitonos por encima de Do. En el código las
variantes llevan las letras con que se nombran las notas en inglés: `C`
es Do, `CSharp` es Do♯, y así hasta `B`, que es Si.

```rust
pub fn semitone(self) -> u8 {
    self as u8
}
```

Ir en la otra dirección pide cuidado, porque no todo número es una
clase de altura. `from_semitone` primero da la vuelta y luego busca la
respuesta en el array `ALL`, así que no puede fallar:

```rust
pub fn from_semitone(semitone: u8) -> Self {
    Self::ALL[(semitone % 12) as usize]
}
```

`transpose` suma semitonos y da la vuelta a la octava. Recibe un `i32`
para poder bajar también. Con un número negativo, `%` da un resto
negativo (`-1 % 12` es `-1` en Rust), así que usa `rem_euclid`, que
siempre da de 0 a 11:

```rust
pub fn transpose(self, semitones: i32) -> Self {
    let wrapped = (self.semitone() as i32 + semitones).rem_euclid(12);
    Self::from_semitone(wrapped as u8)
}
```

`semitones_up_to` responde la pregunta contraria: cuánto hay que subir
de una clase de altura a otra. De La hasta Do son 3 semitonos (La♯, Si,
Do), y de Do hasta La son 9; sumar 12 antes de restar evita que el `u8`
baje de cero:

```rust
pub fn semitones_up_to(self, other: PitchClass) -> u8 {
    (other.semitone() + 12 - self.semitone()) % 12
}
```

### Un match sobre dos valores a la vez

El nombre de todos los días de una clase de altura depende de dos
cosas: cuál es, y si las teclas negras se escriben como sostenidos o
como bemoles. `match` puede mirar las dos a la vez si se ponen en una
**tupla**:

```rust
pub fn name(self, spelling: Spelling) -> &'static str {
    use PitchClass::*;
    use Spelling::*;
    match (self, spelling) {
        (C, _) => "C",
        (CSharp, Sharps) => "C#",
        (CSharp, Flats) => "Db",
        (D, _) => "D",
        // ...
        (B, _) => "B",
    }
}
```

- `use PitchClass::*;` dentro de la función le permite escribir `C` en
  lugar de `PitchClass::C`, sin repartir esos nombres cortos por todo el
  archivo.
- `_` encaja con cualquier cosa: una tecla blanca tiene el mismo nombre
  de las dos maneras.
- El compilador comprueba que las ramas cubran las 24 combinaciones.
  Si falta `(GSharp, Flats)`, se niega a compilar y dice qué caso falta.
- `&'static str` es una cadena prestada que vive tanto como el programa:
  los literales de cadena se guardan dentro del propio programa.

`Spelling` es un enum de dos variantes, `Sharps` (el `#[default]`) y
`Flats`. Cuál usa cada tonalidad, y la elección más fina de una letra
para cada nota, está en [Escalas, modos y
tonalidades](scales.md).

`is_black_key` usa la macro `matches!`, una forma corta de preguntar
«¿encaja este valor con este patrón?» como un `bool`. La `|` significa
«o»:

```rust
pub fn is_black_key(self) -> bool {
    use PitchClass::*;
    matches!(self, CSharp | DSharp | FSharp | GSharp | ASharp)
}
```

`PitchClass` también implementa `Display`, así que `{}` en `format!` lo
escribe con sostenidos, con letras o en solfeo según los ajustes ([Dos
idiomas: traducciones, y letras o
solfeo](../program/languages.md#dos-ajustes-guardados-como-el-de-los-números-romanos)).

## Una nota: un número MIDI con un tipo propio

Una nota con su octava se guarda como su número de nota MIDI, de 0 a
127, envuelto en un struct propio:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(from = "u8", into = "u8")]
pub struct Note(u8);
```

Un struct de un solo campo que se usa para dar a un valor un tipo propio
es un **newtype**. Un `Note` es un `u8` en memoria, pero el compilador no
deja pasar una nota donde se espera una velocidad (también un `u8`), ni
al revés. El campo es privado, así que las únicas formas de crear una
nota son las que ofrece `Note`, y la mantienen dentro del rango:

```rust
pub fn new(midi: u8) -> Self {
    Note(midi.min(Self::MAX.0))
}
```

Lo mismo vale para los archivos. `#[serde(from = "u8", into = "u8")]`
le dice a serde que guarde una nota como su número y que la lea a través
de `From<u8>`, que llama a `Note::new`, así que ni siquiera un archivo de
proyecto editado a mano puede tener una nota por encima de 127:

```rust
impl From<u8> for Note {
    fn from(midi: u8) -> Self {
        Note::new(midi)
    }
}
```

Algunas notas tienen nombre en el código: `Note::C4` es el Do central
(60), `Note::A4` el La de afinación (69), `Note::MIN` y `Note::MAX` los
extremos del rango.

### Octavas

La clase de altura es el número módulo 12, y la octava es el número
dividido entre 12, menos uno, porque la nota MIDI 60 es el Do central y
el Do central se llama Do4:

```rust
pub fn octave(self) -> i8 {
    (self.0 / 12) as i8 - 1
}
```

Así, la nota MIDI más grave es Do-1 y la más aguda Sol9. La octava va
con el nombre de la nota, no con la tecla: cambia entre Si y Do, así que
la tecla un semitono por encima de Si3 es Do4.

### Mover una nota: `Option` para «puede que no exista»

Una clase de altura siempre tiene otra siete semitonos más arriba; una
nota no. Siete semitonos por encima de Sol9 está más allá del final de
MIDI. `transpose` devuelve un `Option<Note>`, y `None` cuando la nota
nueva quedaría fuera del rango:

```rust
pub fn transpose(self, semitones: i32) -> Option<Note> {
    let midi = self.0 as i32 + semitones;
    (0..=Self::MAX.0 as i32)
        .contains(&midi)
        .then_some(Note(midi as u8))
}
```

`contains` pregunta si un rango contiene un número, y `then_some`
convierte un `bool` en un `Option`: `Some` del valor si es verdadero,
`None` si es falso. Quien llama decide qué significa una nota imposible:
un acorde construido cerca del extremo agudo del teclado deja fuera sus
notas más altas, y la herramienta de transportar se niega a mover una
selección entera si una de sus notas se saldría del teclado, así que no
se pierde ninguna nota ([Herramientas de edición como funciones
simples](tools.md)).

### La frecuencia

Ferrous Lineup afina como se afinan los pianos, en **temperamento igual
de doce tonos**: cada semitono multiplica la frecuencia por el mismo
número, la raíz duodécima de dos (alrededor de 1,0595), así que doce
semitonos la duplican exactamente. La4 es 440 Hz:

```rust
pub fn frequency(self) -> f32 {
    let semitones_from_a4 = self.0 as f32 - Self::A4.0 as f32;
    440.0 * 2.0_f32.powf(semitones_from_a4 / 12.0)
}
```

El Do central está nueve semitonos por debajo de La4, así que su
frecuencia es 440 · 2^(-9/12), unos 261,63 Hz. El sintetizador toca esta
frecuencia, y un sampler toca una grabación más deprisa o más despacio
con la misma regla ([Instrumentos: batería, sampler y Lineup
Osc](../sound/instruments.md)).

## Letras y nombres

Cómo se llama una nota es una letra y algunos sostenidos o bemoles. Las
letras son un enum propio, las siete teclas blancas. En el código son
las letras de la nomenclatura inglesa, de `C` (Do) a `B` (Si); en
solfeo cada letra es una sílaba, que da `Letter::solfege`.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Letter {
    C,
    D,
    E,
    F,
    G,
    A,
    B,
}
```

Las letras dan la vuelta de siete en siete como las clases de altura
dan la vuelta de doce en doce, así que tienen el mismo tipo de métodos:
el semitono por encima de Do de su tecla blanca, leído de una tabla, y
subir o bajar con `rem_euclid`:

```rust
pub fn semitone(self) -> u8 {
    [0, 2, 4, 5, 7, 9, 11][self as usize]
}

pub fn up(self, steps: i32) -> Letter {
    Self::ALL[(self as i32 + steps).rem_euclid(7) as usize]
}
```

Dos letras por encima de La está Do (La, Si, Do), y `steps_up_to` cuenta
en el otro sentido: de La a Do hay 2.

Un nombre es una letra y una cuenta de signos, positiva para los
sostenidos y negativa para los bemoles:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NoteName {
    pub letter: Letter,
    /// Sharps if above zero, flats if below: 1 is "#", -2 is "bb".
    pub accidental: i8,
}
```

Un solo `i8` guarda las dos direcciones, así que un nombre nunca puede
ser «un sostenido y un bemol» a la vez, y Fa𝄪 es simplemente
`NoteName::new(Letter::F, 2)`. La tecla del piano que nombra es la tecla
blanca de su letra movida por los signos:

```rust
pub fn pitch_class(self) -> PitchClass {
    PitchClass::from_semitone(self.letter.semitone()).transpose(i32::from(self.accidental))
}
```

### Un nombre para una tecla, con una letra dada

Casi siempre el código sabe la tecla y la letra que quiere, y necesita
los signos. Si con la letra de Do es Do♭ (un bemol); Fa con la letra de
Mi es Mi♯ (un sostenido). `with_letter` cuenta cuánto sube la clase de
altura desde la tecla blanca de la letra, y toma el camino más corto:
hasta cinco semitonos hacia arriba son otros tantos sostenidos, y seis o
más son bemoles hacia abajo:

```rust
pub fn with_letter(pitch_class: PitchClass, letter: Letter) -> Self {
    let up = (pitch_class.semitone() + 12 - letter.semitone()) % 12;
    // Up to five semitones up is sharps; six or more is flats below.
    let accidental = if up <= 5 { up as i8 } else { up as i8 - 12 };
    NoteName::new(letter, accidental)
}
```

Sol con la letra de Fa está dos arriba, Fa𝄪; La con la letra de Si está
diez arriba, es decir, dos abajo: Si𝄫.

Sobre él se construyen dos nombres más sencillos. `plain` toma la letra
del nombre de todos los días, así que da Do♯ o Re♭ para la tecla negra y
su propia letra a las teclas blancas. `chromatic` es el nombre que se usa
cuando no hay tonalidad que seguir, como en la música dodecafónica: Do,
Do♯, Re, Mi♭, Mi, Fa, Fa♯, Sol, La♭, La, Si♭, Si, con sostenidos para dos
teclas negras y bemoles para las otras tres:

```rust
pub fn chromatic(pitch_class: PitchClass) -> Self {
    let spelling = match pitch_class {
        PitchClass::CSharp | PitchClass::FSharp => Spelling::Sharps,
        _ => Spelling::Flats,
    };
    NoteName::plain(pitch_class, spelling)
}
```

Elegir la letra es trabajo de la tonalidad: cada nota de una tonalidad
toma la letra siguiente, y una nota de fuera de la tonalidad toma la
letra que los músicos usan más a menudo para ella. Eso es
`Scale::spell`, en [Escalas, modos y tonalidades](scales.md).

### Los intervalos también cuentan letras

Un intervalo tiene un tamaño en semitonos y un tamaño en letras. Una
tercera mayor son cuatro semitonos y dos letras: sobre Do es Mi, sobre
La es Do♯ (no Re♭, que está tres letras más arriba), y sobre Mi♭ es Sol.
`above` mueve la tecla los semitonos, la letra las letras, y deja que
`with_letter` encuentre los signos:

```rust
pub fn above(self, semitones: u8, letters: u8) -> NoteName {
    let pitch_class = self.pitch_class().transpose(i32::from(semitones));
    NoteName::with_letter(pitch_class, self.letter.up(i32::from(letters)))
}
```

La séptima disminuida son nueve semitonos y seis letras: sobre Si es
La♭, y sobre Do es Si𝄫. Así se escribe cada acorde a partir de su
fundamental ([Acordes: construirlos, nombrarlos e
invertirlos](chords.md)), y así se lleva una hoja de acordes
a otra tonalidad, letra por letra, como la transporta un músico.

Un símbolo de acorde transportado a veces cae en un nombre con dos
signos. Llevar una hoja de acordes de Mi menor a Sol♯ menor sube todo
cuatro semitonos y dos letras, así que el D♯°7 de Mi menor pasa a ser
F𝄪°7. En una partitura eso es lo correcto, pero un símbolo de acorde se
lee de un vistazo, así que los símbolos toman el nombre más sencillo de
la misma tecla:

```rust
pub fn simplified(self) -> NoteName {
    if self.accidental.abs() < 2 {
        return self;
    }
    let spelling = if self.accidental > 0 {
        Spelling::Sharps
    } else {
        Spelling::Flats
    };
    NoteName::plain(self.pitch_class(), spelling)
}
```

Mi𝄫 pasa a ser Re, Fa𝄪 pasa a ser Sol, y el D♯°7 de antes, G°7.

### La octava de una nota escrita

El nombre propio de una nota, `Note::name`, usa el nombre de todos los
días, así que el Do central es siempre «Do4». Pero cuando una tonalidad
escribe el Do central como Si♯, hay que escribirlo «Si♯3»: el número de
octava va con la letra, y Si♯3 es el Si de la octava de abajo, subido un
semitono. Del mismo modo, el Si por debajo del Do central escrito como
Do♭ es «Do♭4». `name_as` quita la letra y los signos para encontrar el
Do del comienzo de la octava de la letra, y numera la octava desde ahí:

```rust
pub fn name_as(self, name: NoteName) -> String {
    let natural =
        i32::from(self.0) - i32::from(name.letter.semitone()) - i32::from(name.accidental);
    format!("{name}{}", natural.div_euclid(12) - 1)
}
```

Para el Do central escrito Si♯, eso es 60 - 11 - 1 = 48, el número MIDI
de Do3, así que el nombre es Si♯3. La comprobación de la conducción de
voces y el entrenamiento auditivo escriben las notas así.

### Escribir y leer un nombre

Escribir un nombre repite el signo tantas veces como dice la cuenta:

```rust
pub fn format(self, naming: NoteNaming) -> String {
    let sign = if self.accidental > 0 { "#" } else { "b" };
    let accidentals = sign.repeat(self.accidental.unsigned_abs() as usize);
    match naming {
        NoteNaming::Letters => format!("{}{accidentals}", self.letter.as_char()),
        NoteNaming::Solfege => format!("{}{accidentals}", self.letter.solfege()),
    }
}
```

`unsigned_abs` convierte -2 en 2 como número sin signo, lo que nunca
desborda (`abs` sobre un `i8` desbordaría con -128). Sin signos,
`repeat(0)` da una cadena vacía.

Leer es el camino inverso, y lo que la aplicación escribe tiene que
poder leerlo de vuelta. En Sol♯ menor armónica el acorde sobre la
séptima nota es F𝄪°7, que se guarda como `F##dim7`, así que `read` toma
hasta dos signos, los dos en la misma dirección. Lee un nombre al
comienzo de un texto y devuelve el resto, así que `"F#m7"` da Fa♯ y
`"m7"`:

```rust
let sign = |c| match c {
    '#' | '♯' => 1,
    'b' | '♭' => -1,
    _ => 0,
};
let mut accidental = 0;
for _ in 0..2 {
    let mut chars = rest.chars();
    let step = chars.next().map_or(0, sign);
    if step == 0 || (accidental != 0 && step != accidental) {
        break;
    }
    accidental += step;
    rest = chars.as_str();
}
```

- `sign` es un closure guardado en una variable. El tipo de su argumento
  no está escrito: el compilador lo deduce de cómo se usa el closure,
  con caracteres.
- `chars()` recorre el texto carácter a carácter, y `as_str()` devuelve
  lo que aún no se ha recorrido. Es la forma segura de pasar por encima
  de «♯», que ocupa tres bytes en UTF-8, donde cortar la cadena por una
  cuenta de bytes podría partir un carácter por la mitad.
- `map_or(0, sign)` da 0 cuando el texto se terminó, y el signo del
  carácter siguiente en otro caso.
- El bucle se detiene en un carácter que no es un signo, en un signo en
  la otra dirección, y después de dos signos. Así, `"Ebbb"` lee Mi𝄫 y
  deja `"b"`, y `"F#b"` lee Fa♯ y deja `"b"`.

La letra tiene que ser mayúscula. `Letter::from_char` rechaza `'c'`,
porque en un símbolo de acorde una «b» minúscula es un bemol: «Bb» es B♭
(Si♭), no dos letras B. La misma lectura en solfeo, «Sib» para Si♭, y
cómo lee la aplicación un nombre escrito de cualquiera de las dos
maneras, están en [Dos idiomas: traducciones, y letras o
solfeo](../program/languages.md#leer-un-nombre-de-las-dos-maneras).

## Un conjunto de clases de altura en doce bits

Los acordes y las escalas son conjuntos de clases de altura. Un conjunto
de doce cosas como mucho cabe en los bits de un entero: el bit 0
significa «Do está en el conjunto», el bit 1 «Do♯ está en el conjunto»,
y así hasta el bit 11 para Si.

```rust
pub struct PitchSet(u16);
```

```rust
pub fn insert(&mut self, pitch_class: PitchClass) {
    self.0 |= 1 << pitch_class.semitone();
}

pub fn contains(self, pitch_class: PitchClass) -> bool {
    self.0 & (1 << pitch_class.semitone()) != 0
}
```

`1 << n` es el número con solo el bit `n` encendido (1 desplazado `n`
posiciones a la izquierda). `|=` enciende ese bit, y `&` deja solo los
bits que comparten dos números, así que `x & (1 << n) != 0` pregunta si
el bit `n` está encendido. Un acorde de Do mayor, Do, Mi y Sol, son los
bits 0, 4 y 7:

```text
bit:      11   10   9    8    7    6    5    4    3    2    1    0
          Si   La♯  La   Sol♯ Sol  Fa♯  Fa   Mi   Re♯  Re   Do♯  Do
Do mayor: 0    0    0    0    1    0    0    1    0    0    0    1
          = 0b0000_1001_0001
```

Otras preguntas sobre conjuntos son una instrucción cada una.
`count_ones()` cuenta los miembros, y `a & !b == 0` pregunta si cada
miembro de `a` está también en `b` (`!` invierte cada bit, así que
`a & !b` es lo que `a` tiene y a `b` le falta):

```rust
pub fn is_subset_of(self, other: PitchSet) -> bool {
    self.0 & !other.0 == 0
}
```

Así pregunta el código si un acorde pertenece a una tonalidad: el La, Do
y Mi de La menor están todos en la escala de Do mayor; el Sol♯ de Mi
mayor no. Y como un conjunto es un número pequeño, comparar dos acordes
es un solo `==`, y un conjunto puede ser la clave de un `HashMap`.

`iter` da los miembros desde Do hacia arriba, quedándose con las clases
de altura que el conjunto contiene:

```rust
pub fn iter(self) -> impl Iterator<Item = PitchClass> {
    PitchClass::ALL
        .into_iter()
        .filter(move |&pitch_class| self.contains(pitch_class))
}
```

`move` le da al closure su propia copia de `self` (un `PitchSet` es
`Copy`), porque el iterador se devuelve y vive más que la llamada.

### Ver un acorde desde su fundamental

El truco para nombrar acordes es mover un conjunto entero hacia arriba o
hacia abajo. En bits, eso es una **rotación** de los doce bits: los que
pasan de Si vuelven a entrar por Do.

```rust
pub fn transpose(self, semitones: u8) -> PitchSet {
    let shift = (semitones % 12) as u32;
    let bits = self.0 as u32;
    // Rotate the 12 bits left by `shift`: bits that go past B come
    // back in at C.
    let rotated = (bits << shift) | (bits >> (12 - shift));
    PitchSet(rotated as u16 & ALL_BITS)
}
```

`bits << shift` sube cada miembro; los que pasaron del bit 11 los quita
la máscara `ALL_BITS` (doce unos). `bits >> (12 - shift)` vuelve a meter
exactamente esos por abajo. Do mayor subido 9 tiene La (bit 9), Do♯ (bit
13, que vuelve a entrar como bit 1) y Mi (bit 16, que vuelve a entrar
como bit 4): La mayor.

`relative_to` baja un conjunto hasta que una fundamental dada se
convierte en Do:

```rust
pub fn relative_to(self, root: PitchClass) -> PitchSet {
    self.transpose(12 - root.semitone())
}
```

Mi menor, Mi Sol Si, visto desde Mi es Do Re♯ Sol: los semitonos 0, 3 y
7, la forma de cualquier tríada menor. Visto desde su fundamental, cada
acorde de un mismo tipo es el mismo número, sea cual sea su fundamental.
[Acordes: construirlos, nombrarlos e invertirlos](chords.md)
nombra un acorde buscando la fundamental desde la que su conjunto
coincide con una forma conocida.

### Hacer que `collect` construya un `PitchSet`

`collect()` puede construir cualquier tipo que implemente el trait
`FromIterator`; `Vec`, `String` y `HashMap` lo hacen. Implementarlo para
`PitchSet` permite que cualquier código convierta notas en un conjunto
en una línea, como `notes.map(|note| note.pitch_class()).collect::<PitchSet>()`:

```rust
impl FromIterator<PitchClass> for PitchSet {
    fn from_iter<I: IntoIterator<Item = PitchClass>>(iter: I) -> Self {
        let mut set = PitchSet::EMPTY;
        for pitch_class in iter {
            set.insert(pitch_class);
        }
        set
    }
}
```

`from_semitones(&[0, 4, 7])` construye un conjunto a partir de números
de la misma manera, convirtiendo cada uno en una clase de altura y
recogiéndolos.

### En un archivo

Un archivo de proyecto guarda un conjunto como sus semitonos por encima
de Do, por ejemplo `"0 3 6 9"`, que es más fácil de leer y de escribir a
mano que el número 585. El atributo `#[serde(try_from = "String", into
= "String")]` lo guarda a través de un `String` y lo lee a través de
`TryFrom<String>`, que rechaza cualquier cosa que no sea un número de 0
a 11. El mismo par de conversiones, y por qué la de lectura es
`try_from`, está en [Series dodecafónicas y sus
reglas](twelve-tone.md#archivos-try_from-e-into).

## Cómo se prueba

Los tests de `pitch.rs` y `pitch_set.rs` se leen como las reglas de este
capítulo. Nombres y octavas, en los dos extremos del rango:

```rust
assert_eq!(Note::new(61).to_string(), "C#4");
assert_eq!(Note::new(61).name(Spelling::Flats), "Db4");
assert_eq!(Note::MIN.to_string(), "C-1");
assert_eq!(Note::MAX.to_string(), "G9");
```

Nombres con cualquier letra, y los nombres cromáticos:

```rust
assert_eq!(name(F, Letter::E), "E#");
assert_eq!(name(B, Letter::C), "Cb");
assert_eq!(name(G, Letter::F), "F##");
assert_eq!(name(A, Letter::B), "Bbb");
```

```rust
assert_eq!(chromatic.join(" "), "C C# D Eb E F F# G Ab A Bb B");
```

Intervalos contados en letras, y la octava de una nota escrita:

```rust
// A major third above A is C#, not Db.
assert_eq!(a.above(4, 2).to_string(), "C#");
```

```rust
assert_eq!(c4.name_as(NoteName::new(Letter::B, 1)), "B#3");
assert_eq!(Note::new(59).name_as(NoteName::new(Letter::C, -1)), "Cb4");
```

Y la lectura, incluido lo que no se debe leer:

```rust
assert_eq!(read("F##dim7"), Some(("F##".to_string(), "dim7")));
assert_eq!(read("Ebbb"), Some(("Ebb".to_string(), "b")), "two at most");
assert_eq!(read("F#b"), Some(("F#".to_string(), "b")), "not mixed");
assert_eq!(read("c"), None, "lower case is a flat, not a note");
```

Los tests de los conjuntos comprueban que transportar da la vuelta (Do
mayor subido 9 es La mayor, subido 12 es él mismo) y que Mi menor visto
desde Mi es el conjunto `0 3 7`.

Los ejercicios sobre estos tipos están en [Ejercicios: la teoría musical
en código](exercises.md).
