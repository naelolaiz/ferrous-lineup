# Series dodecafónicas y sus reglas

Archivos: [`crates/lineup-core/src/harmony/row.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/harmony/row.rs),
[`crates/lineup-core/src/serial.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/serial.rs),
[`crates/lineup-core/src/library/rows.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/rows.rs),
[`crates/lineup-core/src/harmony/voice_leading/serial.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/harmony/voice_leading/serial.rs)
y [`crates/lineup-app/src/twelve_tone.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/twelve_tone.rs)

Casi todo Ferrous Lineup supone música en una tonalidad: una nota que es
la casa, acordes que se inclinan hacia ella, teclas de armonía numeradas
de I a vii°. Una gran parte de la música del siglo XX no tiene
tonalidad, y mucha se escribió con **series dodecafónicas**
(*dodecafonismo*; en inglés, *twelve-tone rows*): las doce notas en un
orden del que está hecha toda la obra. En la aplicación, **serie
dodecafónica** es una de las clases de tonalidad; **Serie…**, en el
piano roll, abre una ventana para escribir, dibujar o elegir una serie,
ver su **matriz** de 48 formas y escribir cualquier forma en el patrón;
la Biblioteca tiene un grupo de estilos de series, y la revisión de
**Conducción de voces** conoce las reglas con las que escribían los
compositores dodecafónicos. Este capítulo trata las cuatro cosas, y el
Rust que hay detrás: un newtype que siempre es una serie de verdad,
aritmética con doce números, backtracking, un builder que corre al
compilar, conjuntos de bits y programación dinámica.

## La música, en pocas palabras

En Do mayor, Do es la casa. Sol tira hacia ella, Si tira fuerte, y una
obra que termina en Do suena terminada. Hacia 1921, Arnold Schoenberg
buscó una manera de escribir música en la que ninguna nota fuera la
casa. Su respuesta fue usar las doce notas por igual, en un orden fijo
que elegía para cada obra: la **serie**. Ninguna nota vuelve hasta que
hayan sonado las otras once, así que ninguna llega a ser el centro.

Esta es la serie de la Suite para piano op. 25 de Schoenberg, de 1925,
una de las primeras obras construidas enteras a partir de una:

```text
Mi  Fa  Sol  Do♯  Fa♯  Re♯  Sol♯  Re  Si  Do  La  La♯
```

Una serie sería aburrida si solo pudiera tocarse de una manera, así que
viene en cuatro:

| forma | cómo | la serie de Schoenberg sobre Mi |
|-------|------|---------------------------------|
| **P**, original | como está escrita | Mi Fa Sol Do♯ Fa♯ Re♯ Sol♯ Re Si Do La La♯ |
| **I**, inversión | cada intervalo dado vuelta: donde P sube un semitono, I baja uno | Mi Re♯ Do♯ Sol Re Fa Do Fa♯ La Sol♯ Si La♯ |
| **R**, retrogradación | P de atrás hacia adelante | La♯ La Do Si Re Sol♯ Re♯ Fa♯ Do♯ Sol Fa Mi |
| **RI**, retrogradación de la inversión | I de atrás hacia adelante | La♯ Si Sol♯ La Fa♯ Do Fa Re Sol Do♯ Re♯ Mi |

Cada una de las cuatro puede empezar en cualquiera de las doce notas,
lo que da **48 formas**. Se nombran con una letra y un número: el
número es la nota en que empieza la forma, con Do como 0, Do♯ como 1,
hasta Si como 11. Así, la serie de arriba es **P4** (empieza en Mi), e
**I0** es su inversión empezando en Do. Una retrogradación toma el
número de la forma que da vuelta: **R4** es P4 al revés, así que
*termina* en Mi.

Algunos libros numeran las formas desde la primera nota de la propia
serie, de modo que la serie tal como está escrita sea siempre P0.
Entonces la serie de Schoenberg es P0, y lo que Ferrous Lineup llama
I11 allí es I7. Las formas son las mismas; solo se mueven sus números.
Ferrous Lineup numera una forma por la nota en que empieza, porque así
una etiqueta nombra una nota que puedes encontrar en el teclado.

### La matriz

Las 48 formas caben en un cuadrado de 12×12. La serie va arriba, su
inversión desde la misma nota baja por el lado izquierdo, y cada fila
*i* es la serie movida para empezar en la nota que está a la izquierda
de esa fila:

```text
      I4   I5   I7   I1   I6   I3   I8   I2   I11  I0   I9   I10
P4    Mi   Fa   Sol  Do♯  Fa♯  Re♯  Sol♯ Re   Si   Do   La   La♯    R4
P3    Re♯  Mi   Fa♯  Do   Fa   Re   Sol  Do♯  La♯  Si   Sol♯ La     R3
P1    Do♯  Re   Mi   La♯  Re♯  Do   Fa   Si   Sol♯ La   Fa♯  Sol    R1
P7    Sol  Sol♯ La♯  Mi   La   Fa♯  Si   Fa   Re   Re♯  Do   Do♯    R7
...
      RI4  RI5  RI7  RI1  ...
```

Una fila leída de izquierda a derecha es una forma P; de derecha a
izquierda, una forma R. Una columna leída de arriba abajo es una forma
I; de abajo arriba, una forma RI. Las etiquetas alrededor del cuadrado
dicen cuál: P a la izquierda, R a la derecha, I arriba, RI abajo. En la
ventana Serie dodecafónica, haz clic en cualquier etiqueta para elegir
esa forma, y su fila o su columna se ilumina.

La diagonal desde arriba a la izquierda hasta abajo a la derecha es la
misma nota de punta a punta (Mi, aquí): una manera rápida de comprobar
que una matriz está bien hecha.

Los grupos de notas tomados de una serie, como sus cuatro grupos de
tres, son **conjuntos de clases de altura**, y la teoría que les pone
nombre tiene un capítulo propio: [Conjuntos de clases de altura, y una
armonía con cuatro nombres](sets.md).

## Una serie en Rust: `ToneRow`

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct ToneRow([u8; 12]);
```

Una serie son doce números, los semitonos de sus notas por encima de
Do. Es un **newtype** alrededor de un array: el array es privado del
módulo, así que las únicas maneras de hacer un `ToneRow` son funciones
que lo comprueban:

```rust
pub fn new(semitones: [u8; 12]) -> Option<Self> {
    let set: PitchSet = PitchSet::from_semitones(&semitones);
    (set.len() == 12 && semitones.iter().all(|&n| n < 12)).then_some(ToneRow(semitones))
}
```

Se juntan los doce números en un conjunto de clases de altura, y si el
conjunto tiene doce miembros, ninguna nota se repitió. Todo `ToneRow`
del programa es, entonces, una serie de verdad: el **invariante** se
comprueba una vez, aquí, y nada más tiene que preocuparse por él.

### Archivos: `try_from` e `into`

Los dos atributos de serde del struct dicen: «para guardar un
`ToneRow`, conviértelo en un `String`; para leer uno, lee un `String` e
intenta convertirlo en un `ToneRow`». Las conversiones son
implementaciones de traits comunes:

```rust
impl TryFrom<String> for ToneRow {
    type Error = String;

    fn try_from(text: String) -> Result<Self, String> {
        ToneRow::parse(&text)
    }
}

impl From<ToneRow> for String {
    fn from(row: ToneRow) -> Self {
        let numbers: Vec<String> = row.0.iter().map(u8::to_string).collect();
        numbers.join(" ")
    }
}
```

Así, un archivo de proyecto guarda `"4 5 7 1 6 3 8 2 11 0 9 10"`, que
una persona puede leer, en lugar de una lista de doce números entre
corchetes. Guardar no puede fallar, así que es un `From` simple; leer
sí puede, así que es un `TryFrom` con un error. Como la lectura pasa
por `parse`, un archivo con una nota escrita dos veces se rechaza con un
mensaje, en vez de dar una serie rota. `PitchSet` tiene el mismo par,
así que un conjunto se guarda como `"0 3 6 9"`.

`parse` también lee el cuadro de arriba de la ventana Serie
dodecafónica. Acepta nombres de notas en letras
(`E F G Db Gb Eb Ab D B C A Bb`) o en solfeo (`Mi Fa Sol Reb Solb Mib
Lab Re Si Do La Sib`), números (`4 5 7 1 …`) o la forma compacta de los
libros de dodecafonismo, un carácter por nota con **T** y **E** para
diez y once (`45716382E09T`). Cuando una nota viene dos veces, dice
cuál, y qué nota falta en su lugar.

Un nombre se lee primero en letras, y en solfeo cuando eso falla.
Ninguna sílaba es también un nombre en letras, porque ninguna empieza
con una letra de la A a la G salvo `Do` y `Fa`, cuya segunda letra no
es un sostenido ni un bemol, así que las dos nunca chocan. Antes de
leer solfeo, `parse_name` escribe el nombre con una mayúscula y el
resto en minúsculas, así que `SOL#` y `sib` se leen como `Sol#` y
`Sib`, donde la `b` minúscula es un bemol:

```rust
fn parse_name(token: &str) -> Result<u8, String> {
    parse_letters(token).or_else(|error| {
        let mut chars = token.chars();
        let first = chars.next().map(|c| c.to_uppercase().collect::<String>());
        let written = first.unwrap_or_default() + &chars.as_str().to_lowercase();
        NoteName::read_as(&written, NoteNaming::Solfege)
            .filter(|(_, rest)| rest.is_empty())
            .map(|(name, _)| name.pitch_class().semitone())
            .ok_or(error)
    })
}
```

`or_else` corre su closure solo ante un error, y `ok_or(error)`
devuelve el error de las letras cuando también falla el solfeo, así que
un nombre equivocado se explica en los términos que más gente escribe.

**Desde las notas** lee una serie del piano roll: las doce primeras
clases de altura distintas entre las notas seleccionadas (o todas), en
el orden en que empiezan. `ToneRow::from_notes` se salta una nota cuya
clase de altura ya está en la serie, así que puedes dibujar libremente
y repetir notas, mientras aparezcan las doce. Las dos terminan en una
tercera conversión, desde un `Vec<u8>`:

```rust
impl TryFrom<Vec<u8>> for ToneRow {
    type Error = String;

    fn try_from(semitones: Vec<u8>) -> Result<Self, String> {
        let array: [u8; 12] = semitones
            .try_into()
            .map_err(|v: Vec<u8>| trf("a row has 12 notes, not {}", &[&v.len()]))?;
        ToneRow::new(array).ok_or_else(|| tr("a row has each of the 12 notes once").to_string())
    }
}
```

Un `Vec` puede tener cualquier cantidad de notas y un array exactamente
doce, así que el `try_into` de la biblioteca estándar de uno al otro
puede fallar. Cuando falla, devuelve el vector sin cambios como error,
y `map_err` lo convierte en un mensaje que dice cuántas notas había.
Después, `ok_or_else` convierte también el `None` de `new` en un
error. Los closures solo corren cuando algo está mal, así que una serie
correcta nunca arma un mensaje.

### Todas las formas desde una función

```rust
pub fn form(self, label: RowLabel) -> ToneRow {
    let first = self.0[0];
    let n = label.number;
    let mut notes = self.0.map(|note| {
        let up = (note + 12 - first) % 12;
        if label.form.is_inverted() {
            (n + 12 - up) % 12
        } else {
            (n + up) % 12
        }
    });
    if label.form.is_backwards() {
        notes.reverse();
    }
    ToneRow(notes)
}
```

Primero se mide cada nota como `up`, su distancia por encima de la
primera nota de la serie (de 0 a 11). Una forma P la pone a la misma
distancia por encima del nuevo comienzo `n`; una forma I, a la misma
distancia *por debajo*. Eso es todo lo que es una inversión. El `+ 12`
antes de cada `%` evita que los números bajen de cero: son `u8`, y
`4 - 7` no daría la vuelta a la octava, sino que detendría el programa
en una compilación de depuración.

`self.0.map(...)` es el `map` de un **array**, no de un iterador:
llama al closure con cada uno de los doce elementos y devuelve un
`[u8; 12]` nuevo, sin `Vec` y sin `collect`. La longitud es parte del
tipo, así que el resultado sigue siendo exactamente doce notas.

Con `form`, la matriz son dos líneas:

```rust
pub fn matrix(self) -> [[u8; 12]; 12] {
    let inversion = self.form(RowLabel::new(RowForm::Inversion, self.0[0]));
    inversion
        .0
        .map(|start| self.starting_on(PitchClass::from_semitone(start)).0)
}
```

La inversión desde la primera nota de la serie es la columna de la
izquierda; cada una de sus notas empieza una fila.

En el otro sentido, «¿qué forma de esta serie es esa?» prueba las 48
hasta que una coincide:

```rust
pub fn label_of(self, other: ToneRow) -> Option<RowLabel> {
    RowForm::ALL.into_iter().find_map(|form| {
        (0..12)
            .map(|n| RowLabel::new(form, n))
            .find(|&label| self.form(label) == other)
    })
}
```

`find_map` se detiene en la primera forma que da `Some`, y `find` en el
primer número que coincide, así que la búsqueda termina en cuanto
encuentra la forma.

## Qué hace interesante a una serie

Debajo de la matriz, la ventana Serie dodecafónica dice qué tiene de
especial la serie. Cada línea es una función corta.

**Los pasos.** `intervals()` da los once pasos de nota a nota, en
semitonos hacia arriba. Para la serie de Schoenberg son
`1 2 6 5 9 5 6 9 1 9 1`. Se construye con `std::array::from_fn`, que
hace un array llamando a un closure con cada índice:

```rust
pub fn intervals(self) -> [u8; 11] {
    std::array::from_fn(|i| (self.0[i + 1] + 12 - self.0[i]) % 12)
}
```

El tipo de retorno dice once, así que `from_fn` llama al closure once
veces, e `i + 1` nunca pasa de la duodécima nota.

**Series de todos los intervalos.** Cuando los once pasos son todos
distintos, cada intervalo, de un semitono a una séptima mayor, aparece
exactamente una vez. La Suite lírica de Alban Berg (1927) empieza con
una serie así: Fa Mi Do La Sol Re Sol♯ Do♯ Re♯ Fa♯ La♯ Si, con el
tritono justo en el medio. Son raras: de las 39.916.800 series que
empiezan en Do, solo 3.856 son de todos los intervalos. Así que el botón
**🎲 Todos los intervalos** no puede limitarse a barajar notas hasta
dar con una; construye la serie nota a nota:

```rust
fn all_interval_from(notes: &mut Vec<u8>, random: &mut Random) -> Option<Vec<u8>> {
    if notes.len() == 12 {
        return Some(notes.clone());
    }
    let used: Vec<u8> = notes.windows(2).map(|w| (w[1] + 12 - w[0]) % 12).collect();
    let mut intervals: Vec<u8> = (1..12).filter(|i| !used.contains(i)).collect();
    for i in (1..intervals.len()).rev() {
        intervals.swap(i, random.below(i + 1));
    }
    let last = *notes.last()?;
    for interval in intervals {
        let next = (last + interval) % 12;
        if notes.contains(&next) {
            continue;
        }
        notes.push(next);
        if let Some(row) = all_interval_from(notes, random) {
            return Some(row);
        }
        notes.pop();
    }
    None
}
```

Esto es **backtracking** (vuelta atrás): probar cada paso todavía no
usado, en un orden al azar; si la nota en la que cae está libre,
añadirla y seguir desde ahí (la función se llama a sí misma); si eso no
lleva a ninguna parte, quitar la nota (`pop`) y probar el paso
siguiente. Cuando ningún paso sirve, devolver `None` para que quien
llamó quite *su* nota. `windows(2)` entrega cada par de vecinas,
`[a, b]`, que es justo lo que es un paso. Todas las llamadas comparten
un vector a través de `&mut`, así que probar una nota cuesta un `push`
y un `pop`, no una copia.

**Series al azar.** El botón **🎲 Al azar** baraja las doce notas con
el **algoritmo de Fisher-Yates**, el mismo bucle que baraja los
intervalos de arriba: recorrer desde el último lugar hasta el primero,
e intercambiar cada uno con un lugar elegido al azar entre los que aún
no se han hecho. Así todos los órdenes son igual de probables. Los
números al azar vienen de un generador diminuto, *SplitMix64*, escrito
en unas pocas líneas en vez de añadir una biblioteca:

```rust
pub(crate) fn next(&mut self) -> u64 {
    self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
    let mut z = self.0;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}
```

`wrapping_add` y `wrapping_mul` dicen «aquí desbordar está bien, que dé
la vuelta»: un `+` normal que desborda detendría una compilación de
depuración. La misma semilla da siempre la misma serie, lo que hace que
las pruebas se puedan repetir. `pub(crate)` deja que otros módulos de
`lineup-core` usen el generador (el entrenamiento auditivo elige sus
preguntas con él) sin que forme parte de la API pública del crate.

**Grupos de notas.** Los compositores suelen tocar una serie en grupos:
cuatro tricordios (tres notas cada uno), tres tetracordios, dos
hexacordios. `segments(size)` corta la serie con `chunks_exact`, que
entrega porciones de exactamente `size` notas y deja fuera una última
más corta, y la ventana nombra la clase de conjunto de cada grupo (ver
[Conjuntos de clases de altura](sets.md)). Los
tetracordios de la serie de Schoenberg son tres clases distintas, 4-12,
4-Z15 y 4-1; los tricordios de la serie Cuatro tríadas son todos 3-11,
las tríadas mayores y menores.

**Combinatoriedad.** Toma las seis primeras notas de una serie. Puede
que otra forma de la misma serie empiece exactamente con las *otras*
seis. Toca las dos formas a la vez, y cada mitad de la música tiene las
doce notas, sin que ninguna vuelva antes de tiempo. Schoenberg
construyó sus obras tardías sobre esto, y se llama **combinatoriedad
hexacordal**:

```rust
pub fn combinatorial_forms(self) -> Vec<RowLabel> {
    let missing = PitchSet::from_semitones(&self.0[..6]).complement();
    [
        RowForm::Prime,
        RowForm::Inversion,
        RowForm::RetrogradeInversion,
    ]
    .into_iter()
    .flat_map(|form| (0..12).map(move |n| RowLabel::new(form, n)))
    .filter(|&label| PitchSet::from_semitones(&self.form(label).0[..6]) == missing)
    .collect()
}
```

La retrogradación queda fuera porque siempre funciona (su primera mitad
es la segunda mitad de la serie), así que no dice nada de la serie. La
serie de la op. 25 de Schoenberg es combinatoria con I11: sus seis
primeras notas, Mi Fa Sol Do♯ Fa♯ Re♯, y las seis primeras de I11, Si
La♯ Sol♯ Re La Do, forman las doce. La Serie de Lineup, escrita para
Ferrous Lineup, es combinatoria con I3.

El `move` de `move |n|` hace falta porque `flat_map` saca el closure de
dentro fuera del de afuera, y ese closure usa `form`: `move` hace que
se lleve su propia copia de `form` en vez de tomar prestada una
variable que está a punto de desaparecer.

### La compañera

`combinatorial_forms` responde por la serie tal como está escrita. Para
tocar cualquier forma con la forma que la completa, cada forma necesita
su propia compañera, y `partner` la encuentra. Prueba tres tipos de
forma, en el orden en que los usaban los compositores: dada vuelta y en
el mismo sentido (P con I, R con RI), la que más usaba Schoenberg;
luego del mismo lado (P0 con P6, por ejemplo); y por último dada vuelta
y al revés. La retrogradación de la propia forma, que siempre la
completa, nunca es la compañera, porque son las mismas notas:

```rust
pub fn partner(self, label: RowLabel) -> Option<RowLabel> {
    let missing = PitchSet::from_semitones(&self.form(label).0[..6]).complement();
    let shape = |inverted: bool, backwards: bool| match (inverted, backwards) {
        (false, false) => RowForm::Prime,
        (true, false) => RowForm::Inversion,
        (false, true) => RowForm::Retrograde,
        (true, true) => RowForm::RetrogradeInversion,
    };
    let (inverted, backwards) = (label.form.is_inverted(), label.form.is_backwards());
    [
        shape(!inverted, backwards),
        shape(inverted, backwards),
        shape(!inverted, !backwards),
    ]
    .into_iter()
    .flat_map(|form| (0..12).map(move |n| RowLabel::new(form, n)))
    .filter(|&other| other != label)
    .find(|&other| PitchSet::from_semitones(&self.form(other).0[..6]) == missing)
}
```

`shape` son otra vez los dos interruptores del [grupo de
Klein](#aritmética-de-formas), leídos al revés: de «¿dada vuelta?» y
«¿al revés?» a una forma. Si una forma tiene compañera, todas tienen
una del mismo tipo: hacerles lo mismo a dos formas (moverlas a las dos,
darlas vuelta o invertir su sentido) mantiene sus mitades
complementarias. Para la Serie de Lineup, P0 va con I3 y R0 con RI3.
Una serie cuya primera mitad es 6-Z3, Do Do♯ Re Re♯ Fa Fa♯, no tiene
ninguna: las seis notas que deja fuera son otra clase, 6-Z36, así que
ninguna forma de la serie puede empezar con ellas.

**Con su compañera, I3**, en la ventana de la serie, escribe la
compañera junto con la forma, una octava más abajo y nota por nota con
el mismo ritmo, así que las seis primeras notas de las dos forman las
doce, y también las seis últimas (`RowWriting::partner`). La compañera
de cada forma de una cadena es la suya, así que P, R, I, RI con
compañeras empareja P0 con I3, luego R0 con RI3, y así sucesivamente.
Cada grupo de la compañera baja de octava en octava hasta quedar por
debajo de la nota más grave de la forma en ese lugar, mientras siga
dentro del teclado:

```rust
if let Some(floor) = floor {
    while pitches.iter().any(|pitch| *pitch >= floor)
        && pitches.iter().all(|pitch| pitch.midi() >= 24 + 12)
    {
        pitches = pitches.iter().map(|pitch| Note::new(pitch.midi() - 12)).collect();
    }
}
```

### Las series para empezar

El menú **Series famosas y de ejemplo** tiene ocho series. Cuatro
vienen de obras famosas, y cuatro se escribieron para Ferrous Lineup
para mostrar lo que puede hacer una serie. `ROW_PRESETS` es un array
`const` de ellas, cada una con un nombre, de dónde viene y una frase
sobre ella.

| serie | de | qué muestra |
|-------|----|-------------|
| Mi Fa Sol Do♯ Fa♯ Re♯ Sol♯ Re Si Do La La♯ | Schoenberg, Suite para piano op. 25 (1925) | una de las primeras obras dodecafónicas; sus cuatro últimas notas, Si Do La La♯, deletrean BACH al revés con los nombres alemanes de las notas, donde B es Si♭ y H es Si |
| Fa Mi Do La Sol Re Sol♯ Do♯ Re♯ Fa♯ La♯ Si | Berg, Suite lírica (1927) | una serie de todos los intervalos, con el tritono en el medio |
| La Fa♯ Sol Sol♯ Mi Fa Si La♯ Re Do♯ Do Re♯ | Webern, Sinfonía op. 21 (1929) | un palíndromo: al revés, es la misma serie un tritono más arriba (R9 es P3) |
| Si La♯ Re Re♯ Sol Fa♯ Sol♯ Mi Fa Do Do♯ La | Webern, Concierto op. 24 (1934) | cuatro grupos de tres notas, todos del mismo micromodo, Menor 3 |
| Do Re Fa♯ Sol♯ Fa Mi La Do♯ Si Re♯ Sol La♯ | Serie de Lineup | combinatoria por hexacordios con I3 |
| Do Mi Sol Re Fa♯ La La♯ Do♯ Fa Si Re♯ Sol♯ | Cuatro tríadas | cuatro tríadas que aun así usan cada nota una vez |
| Do Mi Re♯ Sol Sol♯ Si Fa Re Do♯ La La♯ Fa♯ | Serie espejo | un palíndromo, como la de Webern |
| Do Re Fa Mi Si La Re♯ Sol Sol♯ Do♯ La♯ Fa♯ | Todos los intervalos | una serie de todos los intervalos |

Las obras de 1925 a 1929 son de dominio público en los Estados Unidos
(95 años después de su publicación), y sus compositores murieron en
1951, 1935 y 1945, así que también lo son donde los derechos de autor
duran 70 años después de la muerte del autor. El Concierto de Webern se
publicó en 1948, así que es de dominio público donde los derechos duran
70 años después de la muerte del autor. Solo se usan las series: doce
notas cada una.

Algunas series no tienen 48 formas *distintas*. Como la R9 de Webern es
su P3, cada forma R de esa serie es también una forma P, y cada forma
RI una forma I: solo 24 son distintas. Con la serie de Berg y la Serie
espejo pasa lo mismo: leída al revés, cada una es ella misma un tritono
más arriba. `label_of` prueba primero las formas P, así que una forma
así se nombra como la forma P que también es.

## De una serie a notas: el serialismo

Una serie dice qué nota viene después, y nada más. Para hacer música
con ella, alguien todavía tiene que decidir cuánto dura cada nota, qué
tan fuerte suena, en qué octava, y si algunas notas suenan juntas.
`RowWriting` guarda esas decisiones, y los controles de debajo de la
matriz las fijan:

```rust
pub struct RowWriting {
    /// The form to start with, such as `P0`.
    pub label: RowLabel,
    pub chain: Chain,
    /// How many notes of the row sound together: 1 for a melody, 3 to
    /// play the row as four three-note chords (trichords), 6 for two
    /// hexachords.
    pub together: usize,
    /// How long each note (or chord) lasts, in steps, when the durations
    /// do not follow the row.
    pub step: usize,
    pub register: Register,
    /// The lowest note the row is written from.
    pub low: Note,
    /// Whether the durations follow the row (see the module documentation).
    pub durations: bool,
    /// Whether the velocities follow the row's inversion.
    pub dynamics: bool,
    /// Whether the octaves follow the row's retrograde.
    pub octaves: bool,
    /// Whether each form sounds with its combinatorial partner (see
    /// [`ToneRow::partner`]) an octave under it, note for note in the
    /// same rhythm, so that every half of the two holds all twelve
    /// notes. A form without a partner sounds alone.
    pub partner: bool,
}
```

**Después** (`chain`) dice qué viene tras la forma hasta llenar el
patrón: nada (**Una vez**), la misma forma otra vez (**Repetir**), o
las cuatro formas por turno con el mismo número (**P, R, I, RI**). La
cuarta opción, **Un plan de formas**, es un plan de la pieza, como una
progresión es el plan de una pieza tonal: las formas escritas en un
campo, `P0 RI5 I7 R0`, o elegidas con un clic alrededor de la matriz,
cada una agregada al final. `read_plan` lee el campo:

```rust
pub fn read_plan(text: &str) -> Result<Vec<RowLabel>, String> {
    text.split(|c: char| c.is_whitespace() || matches!(c, ',' | '|' | ';'))
        .filter(|word| !word.is_empty())
        .map(|word| {
            RowLabel::parse(&word.to_ascii_uppercase()).ok_or_else(|| {
                trf(
                    "\"{}\" is not a form of the row: write forms as P0, I5, R11 or RI3.",
                    &[&word],
                )
            })
        })
        .collect()
}
```

`split` recibe un cierre que dice qué caracteres cortan el texto, así
que los espacios, las comas y las barras de la hoja de acordes separan
formas, y `filter` descarta las palabras vacías que dejan dos
separadores seguidos. Cada palabra se vuelve un `Result`, y recolectar
`Result`s en `Result<Vec<_>, _>` se detiene en el primer error, como
recolectar `Option`s ([Herramientas de edición como funciones
simples](tools.md#cuando-una-herramienta-no-puede-recolectar-options)):
la ventana muestra ese error debajo del campo. `RowLabel::parse` es el
mismo lector que usa la hoja de acordes para una forma escrita en un
compás; el plan pasa antes cada palabra a mayúsculas, cosa que la hoja
no puede hacer, porque ahí una `i7` minúscula es un número romano.
Después `write_plan` escribe las formas una tras otra, y otra vez desde
la primera, hasta llenar el patrón.
**Juntas** es `together`, y **Cada** es `step`. Los tres **registros**
(el menú **Octavas**) deciden las octavas cuando la serie no lo hace:

- **Cerrado** pone cada nota en la octava por encima de la nota más
  grave;
- **Suave** pone cada nota tan cerca como pueda de la anterior, lo que
  da una línea cantable (un tritono, justo a mitad de camino, sube);
- **Amplio** manda las notas grave, aguda, media, grave, aguda, media
  por tres octavas, las líneas a saltos de Anton Webern.

### Serialismo integral

Hacia 1950, Olivier Messiaen, Pierre Boulez y Milton Babbitt llevaron
la idea más lejos: si las alturas siguen una serie, ¿por qué no también
las duraciones, las dinámicas y las octavas? Eso es el serialismo
**integral** (o total). Las obras reales traducen las series a
duraciones y dinámicas de muchas maneras; Ferrous Lineup usa tres
sencillas, cada una leyendo una forma distinta de la serie para que las
series no avancen a la par:

- **Duraciones**: los números de la serie (la distancia de cada nota
  por encima de la primera, de 0 a 11) se vuelven duraciones. La nota
  *k*-ésima dura tantos pasos como el número *k*-ésimo, más uno, así
  que de 1 a 12 semicorcheas.
- **Dinámicas**: los números de la inversión se vuelven doce
  velocidades, de muy suave a muy fuerte: `28 + 9 * level`, de 28 a
  127.
- **Octavas**: los números de la retrogradación, contados de tres en
  tres (`% 3`), eligen una de tres octavas.

Para la serie de Schoenberg los números son 0 1 3 9 2 11 4 10 7 8 5 6,
así que con **Duraciones** marcada sus notas duran 1, 2, 4, 10, 3, 12…
pasos. Dentro de `write_row`, cada nota hace las tres preguntas por
turno:

```rust
let lasts = if writing.durations {
    usize::from(numbers[place % 12]) + 1
} else {
    step
};
// ...
let octave = if writing.octaves {
    // The retrograde's numbers, in threes.
    Some(numbers[11 - k % 12] % 3)
} else if writing.register == Register::Wide {
    // Low, high, middle, and round again, against the row.
    Some([0, 2, 1][k % 3])
} else {
    None
};
// ...
let velocity = if writing.dynamics {
    // The inversion's numbers.
    dynamic((12 - numbers[k % 12]) % 12)
} else {
    crate::DEFAULT_VELOCITY
};
```

`place` cuenta notas a través de las formas, así que con **P, R, I,
RI** la serie de duraciones sigue de una forma a la siguiente en vez de
volver a empezar. `k` es `place` más el lugar de la nota en su acorde,
así que cada nota de un acorde tiene su propia octava y su propia
intensidad.

`write_row` es una función pura: entra una serie, las decisiones y la
duración de un patrón; sale una lista de `NoteEvent`. La ventana la
llama en cada fotograma para avisar cuando el patrón es demasiado corto
para toda la serie («El patrón solo tiene lugar para 9 de las 12
notas»), y **Escribir P0 en el patrón actual** manda las mismas notas
al piano roll, que reemplaza las notas del canal con la edición de
siempre, así que **Ctrl+Z** trae de vuelta las anteriores. **Guardar en
un patrón nuevo** las pone en un patrón nuevo.

## Una serie como tonalidad

**serie dodecafónica** en los menús de tonalidad, o **Usar como
tonalidad** en la ventana Serie dodecafónica para la forma que
elegiste, hace de la serie la tonalidad. En el código es una clase más
de tonalidad, `ScaleKind::Row(ToneRow)`, que lleva su serie consigo
([Escalas, modos y tonalidades](scales.md) muestra cómo la
trata el resto del código). La serie se guarda empezando en Do, y la
fundamental de la tonalidad dice dónde empieza de verdad, así que P4 de
la serie de Schoenberg se guarda así (el archivo escribe las notas
siempre con letras):

```text
scale: (root: E, kind: Row("0 1 3 9 2 11 4 10 7 8 5 6")),
```

Con una serie como tonalidad, el piano roll numera sus notas del **1 al
12** en el teclado, en el orden de la serie, así que puedes dibujar la
serie siguiendo los números. Las teclas de armonía tocan los grupos de
la serie en lugar de acordes, como arman sus acordes los compositores
dodecafónicos: **3 notas** da cuatro teclas para los tricordios,
**1–3**, **4–6**, **7–9** y **10–12**; **4 notas** da tres teclas, y
**6 notas**, dos. Salen de cortar la serie en orden:

```rust
ScaleKind::Row(_) => {
    let group = row_group(size);
    Some(
        notes
            .chunks(group)
            .enumerate()
            .map(|(i, chunk)| {
                let label = format!("{}–{}", i * group + 1, i * group + chunk.len());
                GroupKey::new(scale, Some(label), chunk.to_vec())
            })
            .collect(),
    )
}
```

Cambiar la tonalidad de Do mayor a una serie no mueve ninguna nota,
porque no hay un lugar sensato al que pueda ir Mi en una serie; cambiar
a la misma serie sobre otra nota las mueve todas, que es transportar,
y cambiar a otra serie toca la misma pieza sobre ella ([sección
siguiente](#otra-serie-para-la-misma-pieza)). Pasar de una serie a **conjunto de clases de alturas** se queda con las
tres primeras notas de la serie como conjunto, que son las que tocaban
las teclas de armonía (ver [Una tonalidad como conjunto: las mismas
notas, otro
nombre](sets.md#una-tonalidad-como-conjunto-las-mismas-notas-otro-nombre)).
La serie del Concierto de Webern sobre Do da Do Si Mi♭, nombrada como
conjunto, 3-3; **Menor 3** entre los micromodos del menú nombra las
mismas tres notas como el micromodo que también son (ver [Tres notas:
un conjunto o un
micromodo](sets.md#tres-notas-un-conjunto-o-un-micromodo)). Elegir otra
vez **serie dodecafónica** trae de vuelta la serie de la canción, cada
nota en su lugar: los menús de tonalidad recuerdan la última serie que
fue la tonalidad, cada vez que se dibujan, y solo una canción que nunca
tuvo una serie recibe la Serie de Lineup.

### Otra serie para la misma pieza

En una tonalidad, un modo nuevo deja cada nota en su grado: de Do mayor
a Do menor bajan Mi, La y Si y el resto se queda ([Escalas, modos y
tonalidades](scales.md#cambiar-de-tonalidad-mover-la-canción)). Los
grados de una serie son sus lugares en cada forma. Por eso, cambiar la
tonalidad de una serie a otra deja cada nota en la forma de la que es
parte, en el mismo lugar: la quinta nota de RI5 pasa a ser la quinta
nota de RI5 de la serie nueva. El plan de formas, el ritmo, los acordes
y el registro se quedan; solo cambia la serie, así que la pieza suena
como habría sonado si el compositor hubiera elegido la otra serie.

```rust
pub fn change_row(note: Note, label: Option<RowLabel>, from: Scale, to: Scale) -> Note {
    let (Some(old), Some(new)) = (from.tone_row(), to.tone_row()) else {
        return change_key(note, from, to);
    };
    let label = label.unwrap_or(old.label());
    let up = from.root.semitones_up_to(to.root);
    let place = old.form(label).position(note.pitch_class());
    let moved = RowLabel::new(label.form, label.number + up);
    let target = new.form(moved).pitch_classes()[place];
    let shift = nearest_shift(note.pitch_class(), target);
    note.transpose(shift)
        .or_else(|| note.transpose(shift - 12 * shift.signum()))
        .unwrap_or(note)
}
```

Cada forma tiene cada clase de altura una vez, así que `position`
encuentra el lugar de la nota en su forma. Las formas se cuentan desde
la nota de la tonalidad: si esa nota pasa de Do a Mi, cada forma sube
cuatro semitonos con ella, y RI5 pasa a ser RI9 (`RowLabel::new` da la
vuelta pasado el 11). Después la nota va por el camino más corto a su
nueva clase de altura, hasta un tritono, así que la línea conserva su
forma en el teclado; solo en el borde del registro MIDI va hacia el
otro lado.

De qué forma es parte una nota no está escrito en ningún lado de un
patrón, pero la [revisión dodecafónica](#leer-la-serie-como-lo-haría-un-analista)
lo lee, como lo haría un analista: `Project::change_key` se lo pregunta
para cada patrón antes de cambiar la tonalidad, y cada nota toma la
forma que toca su pista ahí, o la que comparten todas las pistas. Donde
la revisión no encontró nada, se usa la forma escrita ahí en la hoja de
acordes, después una forma que toca otra pista ahí (una línea doblada
en octavas se lee una sola vez, para una de las dos), y al final la
forma propia de la tonalidad, que es `label` cuando es `None`.

Algunas series les dan dos nombres a cada forma. La de Berg es una: su
segunda mitad es la primera hacia atrás, un tritono más arriba, así que
cada forma R es también una forma P. La revisión lee esa forma por uno
de sus nombres, que no tiene por qué ser el que tenía al escribirla, y
volver a la primera serie usa entonces ese nombre. La pieza sigue hecha
de formas de la serie, pero no siempre de las mismas que antes;
**Ctrl+Z** es la forma de volver a las notas exactamente como estaban.

Los cambios de tonalidad más adelante en la canción siguen a la
tonalidad de la canción como en una canción tonal: una sección en la
serie vieja toma la nueva, movida con la nota de la tonalidad, y una
sección en una serie propia la conserva.

### Hacia atrás y dado vuelta en el piano roll

El menú **Herramientas** del piano roll hace las otras formas a partir
de notas que dibujaste ([Herramientas de edición como funciones
simples](tools.md#hacia-atrás-y-dado-vuelta)): **Hacia atrás** las toca
hacia atrás en el tiempo, con su ritmo también, así que P0 queda R0;
**Alturas hacia atrás** conserva el ritmo y da vuelta solo las alturas,
como escriben los compositores dodecafónicos una retrogradación con un
ritmo propio; y **Dado vuelta** da vuelta cada intervalo desde la
primera nota, así que P0 queda I0. **Dado vuelta en la tonalidad**
cuenta notas de la tonalidad, y en una serie, donde todas las notas
están en la tonalidad, da lo mismo.

## Estilos y demostraciones dodecafónicos

El grupo **Series dodecafónicas** de la Biblioteca tiene siete estilos:

| estilo | qué hace |
|--------|----------|
| Melodía de la serie | la Serie de Lineup como melodía en corcheas, por P, R, I y RI |
| Coral de tríadas | la serie Cuatro tríadas como acordes, sobre su retrogradación en el bajo: música dodecafónica que suena casi tonal |
| Pads de hexacordios | media serie como acorde, las otras seis notas desde su compañera (I3 para la Serie de Lineup) como melodía: cada compás tiene las doce notas |
| Puntillismo | notas sueltas que saltan por tres octavas, con duraciones y dinámicas de la serie, como en Webern |
| Serialismo integral | alturas, duraciones, dinámicas y octavas, todas de la serie de todos los intervalos, sobre su retrogradación de la inversión en acordes de dos notas |
| Lo-fi dodecafónico | los tricordios de la serie sobre un ritmo lo-fi, con su forma original en el bajo: una serie también puede tener groove |
| Los tricordios de Webern | la serie del Concierto de Webern, con su melodía saltando por tres octavas sobre sus grupos de tres como acordes |

Un estilo armónico es una progresión de acordes y una manera de
tocarla. Un estilo de serie no tiene progresión, así que es una serie y
una lista de **voces**, cada una diciendo cómo toca la serie un
instrumento. Como el resto de la Biblioteca, son datos:

```rust
RowStyle {
    name: "Triad chorale",
    // ...
    row: "Four triads",
    tempo: 72.0,
    time: TimeSignature::COMMON,
    bars: 4,
    voices: &[
        voice(KEYS, RowForm::Prime)
            .chain(Chain::FourForms)
            .chords(3, 4)
            .from(Register::Nearest, 55),
        voice(BASS, RowForm::Retrograde)
            .step(8)
            .from(Register::Close, 36),
    ],
    rhythm: "",
},
```

`voice` hace una voz simple (una melodía en corcheas desde el Do
central), y cada método cambia una cosa y devuelve la voz nueva:

```rust
impl RowVoice {
    const fn offset(self, offset: u8) -> Self {
        RowVoice { offset, ..self }
    }
    const fn chain(self, chain: Chain) -> Self {
        RowVoice { chain, ..self }
    }
    // ...
}
```

Tomar `self` por valor y devolver una copia cambiada con la **sintaxis
de actualización de structs** (`..self`) deja encadenar las llamadas,
como un builder. Y como son `const fn`, toda la tabla puede ser un
`static`: `pub static ROW_STYLES: [RowStyle; 7]` se construye al
compilar, sin código que corra al arrancar.

### Aritmética de formas

Las demostraciones de **Archivo > Demostraciones > Dodecafónico** tocan
cada estilo cuatro veces, como las secciones de una canción: **P: la
serie**, **R: al revés**, **I: dada vuelta** y **RI: las dos**, y luego
P una vez más para terminar. Con un ritmo, la batería entra después de
la primera sección, con un fill antes de la última. Pero las voces de
un estilo ya usan formas distintas: el bajo del Coral de tríadas toca
R. ¿Qué toca el bajo en la sección R? R de R, que es la serie hacia
adelante otra vez: P.

Las cuatro formas se comportan como dos interruptores, «dada vuelta» y
«al revés», y combinar dos formas cambia cada interruptor una vez por
cada forma que lo tiene encendido:

```rust
pub fn combine(section: RowForm, voice: RowForm) -> RowForm {
    let flags = |form| match form {
        RowForm::Prime => (false, false),
        RowForm::Inversion => (true, false),
        RowForm::Retrograde => (false, true),
        RowForm::RetrogradeInversion => (true, true),
    };
    let (a, b) = (flags(section), flags(voice));
    match (a.0 != b.0, a.1 != b.1) {
        (false, false) => RowForm::Prime,
        (true, false) => RowForm::Inversion,
        (false, true) => RowForm::Retrograde,
        (true, true) => RowForm::RetrogradeInversion,
    }
}
```

`a.0 != b.0` sobre dos `bool` es un *o exclusivo*: verdadero cuando
exactamente uno lo es. Los matemáticos llaman a este pequeño sistema el
**grupo de Klein**: cada forma se deshace a sí misma (I de I es P), y
el orden no importa. Un detalle más: la forma de una voz también se
puede mover. La forma propia de la melodía de Pads de hexacordios es
I3, tres semitonos *por encima* de la primera nota de la serie, y la
toca cuando la serie no tiene [compañera](#la-compañera). Dada vuelta, «tres por encima» pasa a ser «tres por debajo», así que en
las secciones I y RI ese desplazamiento pasa a ser 12 − 3 = 9:

```rust
let offset = if inverted {
    (12 - self.offset % 12) % 12
} else {
    self.offset % 12
};
```

### En una canción, la banda toca las formas de la hoja

Parado, un estilo de serie se prueba en un patrón propio, como cualquier
estilo. En una canción es de la banda, como un [estilo de
micromodo](micromodes.md#en-una-canción-la-banda-toca-las-formas-de-la-hoja):
la hoja de acordes guarda el **plan** de la pieza, una forma de la
serie en cada compás o parte (`P2 | RI7 | I5 | R2`, escritas como
acordes), y las voces del estilo tocan esas formas, cada una mientras
dure en la hoja. El plan lo escribes tú, como los acordes en un estilo
tonal.

Cada voz lee su propia forma como la forma de la hoja lee la serie,
con la misma [aritmética de formas](#aritmética-de-formas): donde la
hoja dice `I5`, los acordes del Coral de tríadas tocan `I5` y su bajo,
que toca la serie al revés, toca `RI5`. Es un método de la voz, que
recibe la serie y la forma de la parte:

```rust
pub fn writing(&self, row: ToneRow, section: RowLabel) -> RowWriting {
    if self.partner
        && let Some(partner) = row.partner(section)
    {
        return RowWriting {
            label: partner,
            ..self.writing_from(PitchClass::from_semitone(section.number), section.form)
        };
    }
    self.writing_from(PitchClass::from_semitone(section.number), section.form)
}
```

Una voz que toca la **compañera** (la melodía de Pads de hexacordios)
toca la [compañera combinatoria](#la-compañera) de la forma de la parte
cuando la serie tiene una, así que el compás conserva las doce notas
sea cual sea la serie; una serie sin compañera vuelve a la forma propia
de la voz, I3 de la parte.

La banda las toca en `band/row.rs`, junto a `band/micromode.rs`. Los dos
tipos de estilo comparten cómo entra una banda de voces y cómo toma el
compás, el tempo y el swing (`bring_a_band_of_voices` y
`take_timing`); lo que cambia es la música. Para cada parte de la hoja
que es una forma de la serie donde la tonalidad es una serie, una voz
escribe su forma con [`write_row`](#de-una-serie-a-notas-el-serialismo),
tan larga como la parte, y lleva las notas a donde empieza la parte:

```rust
for &(start, stop, harmony) in &score.spans {
    let (HarmonyName::Row(label), ScaleKind::Row(row)) =
        (harmony, score.keys.key_at(start).kind)
    else {
        continue;
    };
    let written = write_row(row, &voice.writing(row, label), stop - start);
    notes.extend(written.into_iter().map(|note| NoteEvent {
        start: note.start + start,
        ..note
    }));
}
```

`score.spans` es la hoja tal como está escrita, cada armonía desde
donde empieza hasta donde empieza la siguiente. Los demás músicos de la
banda ven una forma de la serie como cuatro acordes de tres notas, en
el orden de la serie (ver [Una serie como
tonalidad](#una-serie-como-tonalidad)), que es lo que golpean los
golpes y las notas tenidas de la hoja; la voz de un estilo necesita la
forma misma. Comparar con un par, `(harmony, kind)`, con `let ... else`
desarma los dos a la vez y salta la parte salvo que ambos sean lo que
la voz puede tocar.

**Tu serie.** Un estilo trae una serie propia, pero cuando la tonalidad
ya es una serie, toca esa: las texturas son del estilo, las notas
tuyas. Con la casilla **Progresión de acordes** marcada, o en una
tonalidad tonal, trae la suya, empezando en la nota de la tonalidad:

```rust
pub fn row_in(&self, key: Scale, own: bool) -> ToneRow {
    match key.tone_row() {
        Some(row) if !own => row,
        _ => self.row().starting_on(key.root),
    }
}
```

La misma regla vale cuando un estilo se prueba en un patrón. Como con
los estilos tonales, el segundo estilo probado en el mismo patrón de
prueba toca lo que dejó el primero: el primero hizo de su serie la
tonalidad.

**El plan.** Cuando la hoja no tiene formas de la serie (una canción
tonal, o una vacía), `apply_row_style` escribe allí el plan del estilo,
desde el primer compás hasta el final de la canción: la serie como está
escrita, al revés, dada vuelta y las dos cosas, cada parte tan larga
como una pasada del estilo:

```rust
let plan: Vec<ChordMarker> = bars
    .chunks(style.bars.max(1))
    .zip(SECTIONS.iter().cycle())
    .map(|(part, &form)| {
        let label = RowLabel::new(form, row.first().semitone());
        ChordMarker::new(part[0], HarmonyName::Row(label))
    })
    .collect();
```

`chunks` corta los compases en partes, `cycle` repite P, R, I y RI
tantas veces como partes haya, y `zip` los empareja, parando con el más
corto, los compases. Una hoja que ya tiene formas de la serie
(lo comprueba `sheet_holds_row_forms`) las conserva, salvo que
**Progresión de acordes** esté marcada. Una banda que entra silencia
los acordes y el bajo propios de la canción, como cualquier banda; la
melodía sigue sonando, y un estilo con melodía propia la toca en lugar
del solista.

Una forma de la hoja suele durar una parte entera, más que el compás en
que empieza, mientras que la hoja de acordes termina al final del
compás en que empieza su última armonía. Por eso el plan también le
dice a la banda que toque hasta el final de la última parte
(`band.end`). El archivo guarda por su nombre el estilo que toca la
banda, desde el formato 31.

## Una ventana que se abre delante

egui pone delante de las demás una ventana que se abre. Pero la ventana
cuyo botón **Serie…** la abrió recibió un clic en el mismo fotograma, y
un clic sube la ventana donde cae, así que las dos quieren estar
arriba, y la nueva puede quedar detrás. `Front` mantiene delante una
ventana nueva durante sus primeros fotogramas:

```rust
#[derive(Default)]
pub(crate) struct Front {
    /// The frame the window was last drawn in.
    drawn_in: u64,
    /// The frame it opened in.
    opened_in: u64,
}

impl Front {
    /// Called each frame the window `id` is drawn, before it is.
    pub(crate) fn keep(&mut self, ctx: &egui::Context, id: egui::Id) {
        let frame = ctx.cumulative_pass_nr();
        if self.drawn_in + 1 < frame {
            self.opened_in = frame;
        }
        self.drawn_in = frame;
        if frame - self.opened_in < 3 {
            ctx.move_to_top(egui::LayerId::new(egui::Order::Middle, id));
            ctx.request_repaint();
        }
    }
}
```

`cumulative_pass_nr` cuenta los fotogramas que egui ha dibujado. Una
ventana que no se dibujó en el fotograma anterior acaba de abrirse, así
que recuerda ese fotograma; durante sus tres primeros fotogramas sube
su propia capa arriba de todo y pide otro fotograma, de modo que los
tres llegan enseguida en vez de esperar a que se mueva el ratón. Para
encontrar su capa, la ventana necesita su id, así que las ventanas
Serie dodecafónica, Conjuntos de clases de alturas y Micromodos ponen
cada una el suyo con `.id(...)`, hecho con `egui::Id::unique` (*unique*
porque nombra una sola cosa en todo el programa, como debe hacerlo el
id de una ventana). La lista de reproducción hace lo mismo cuando un
doble clic abre un clip en el piano roll ([Doble clic en un clip para
editarlo](../program/editors.md#doble-clic-en-un-clip-para-editarlo)).

La ventana Serie dodecafónica es tan ancha como la matriz y se desplaza
cuando la pantalla es baja, para que sus botones de abajo sigan a mano:
`.resizable([false, true])` la deja hacerse más alta o más baja pero no
más ancha, y `.constrain_to(area)` la mantiene dentro de la ventana
principal.

## Las reglas de la escritura dodecafónica

La [revisión de la conducción de voces](voice-leading.md)
marca lo que evita la escritura a varias voces clásica: quintas
paralelas, una sensible que no sube, una séptima que no baja. Cada una
de esas reglas tiene que ver con una tonalidad. En la música escrita
con una serie no hay tónica, así que allí no significan nada. La música
dodecafónica tiene reglas propias. Schoenberg escribía con ellas, y
también sus alumnos Berg y Webern; los cursos de composición todavía
las enseñan. Así que cuando la tonalidad es una **serie dodecafónica**,
la casilla **Conducción de voces** revisa esas reglas en su lugar.

### Lo que ves

- **Una línea Serie** debajo de los nombres de los acordes. Nombra
  cada forma de la serie que la revisión encuentra en las notas, como
  **P4** o **P10**, encima de las notas que la tocan. Pasa el puntero
  por encima para leer las notas de la forma: «P4: la serie que empieza
  en Mi: Mi Fa Sol Do# Fa# Mib Lab Re Si Do La Sib.»
- **La línea Voces**, debajo, marca lo que evita la escritura
  dodecafónica: **orden** para una nota fuera del orden de la serie,
  **rep.** para una nota repetida antes de tiempo, **hueco** para una
  forma sin terminar, **8va** para una nota duplicada a la octava,
  **tonal** para un acorde que suena a armonía tonal.
- **Contornos** alrededor de las notas en cuestión. Dos notas
  duplicadas se unen con una línea, y también las notas de un acorde
  tonal. Un contorno punteado muestra la nota que la serie pedía en su
  lugar.
- **La línea de resumen**, encima de las filas de acordes, lo resume
  todo: «Reglas dodecafónicas, P4, P10, a través de todas las pistas
  juntas: fuera del orden de la serie (2), duplicación a la octava (1),
  suena tonal (1).» Pasa el puntero por un paso con un hallazgo y la
  línea dice qué pasa allí; pasa el puntero por la línea para leer por
  qué.

Con un **conjunto de clases de alturas** o un micromodo como
tonalidad no hay un orden que respetar, pero hay un color: la revisión
lee las formas del conjunto en las notas y marca las notas que quedan
fuera de ellas (ver [Revisar el color](micromodes.md#revisar-el-color)).

### Las cinco reglas

**La serie fija el orden.** Una obra avanza por formas de la serie,
cada una tocada nota tras nota, en su orden, para que todo lo que oyes
venga de la serie. Notas vecinas en la serie pueden sonar juntas como
acorde, en cualquier orden de abajo arriba, y una nota puede repetirse
enseguida, o hacer un trino con la nota anterior: sigue siendo la misma
nota de la serie. Dos errores se cuentan con cuidado. Si **intercambias**
dos notas, o **dejas una fuera**, eso es una nota fuera de lugar, no
todas las que vienen después: con Si tocado antes que Re en P4, la
revisión marca el Si («Melodía toca Si4 donde P4 pide Re, su nota 8»)
y encuentra el resto de la serie en orden otra vez.

**Ninguna nota vuelve antes de tiempo.** Antes de que una nota vuelva,
suenan las otras once. La serie se hizo para eso: ninguna nota se oye
más que las demás, así que ninguna se vuelve tónica. Una nota que
vuelve después de otras notas, antes de que la forma haya hecho sonar
las doce, se marca con **rep.**

**Una forma se toca entera.** Cuando una forma se corta y empieza la
siguiente, las notas que dejó fuera faltan entre las doce. La revisión
marca la última nota de una forma así con **hueco** y nombra lo que
nunca sonó: «Serie sin terminar: P4 se detiene después de 8 de sus 12
notas, con Melodía en Re5; Si, Do, La y Sib nunca suenan antes de que
empiece la forma siguiente.»

**Sin duplicaciones a la octava.** Duplicar una nota a la octava le da
peso. Como explicó Schoenberg, una nota con peso puede oírse como
fundamental, o incluso como tónica. La revisión marca dos partes que
tocan la misma nota a la vez, y también una parte que toca una nota
mientras otra parte todavía la sostiene. Una parte que duplica a otra
de principio a fin, una línea en octavas, se dice una sola vez, donde
empieza.

**Sin acordes de armonía tonal.** Una tríada mayor o menor, o una
séptima de dominante, le recuerda al oído la música tonal, y entonces
el oído espera que la música siga como seguiría la música tonal.
Schoenberg y Webern los evitaban. La revisión marca un momento en que
todo lo que suena a la vez forma uno de esos acordes, y lo nombra.
**Berg** los usaba a propósito: la serie de su Concierto para violín
(1935) está hecha de tríadas, Sol menor, Re mayor, La menor y Mi mayor.
Así que lee **tonal** como «esto suena a una tonalidad», no como «esto
está mal».

### Leer la serie como lo haría un analista

Nada en las notas dice qué forma de la serie suena, ni dónde termina
una forma y empieza la siguiente. Un estudiante que analiza una
partitura tiene que averiguarlo, y la revisión también. Corta las notas
en **grupos**, notas que empiezan juntas, prueba luego todas las
maneras de repartir los grupos entre formas de la serie, y se queda con
la que tiene menos problemas: cada nota fuera de orden, cada nota
repetida antes de tiempo y cada nota que una forma deja fuera cuenta
uno. Algunas cosas acercan la lectura a lo que hacen de verdad los
compositores:

- **Las formas pueden solaparse en la unión.** Una forma puede empezar
  en la nota, o el acorde, en que terminó la forma anterior, de modo
  que las dos la comparten: una **elisión**. Si P4 termina con el
  acorde La Si♭ Do y P10 empieza en Si♭, P10 toma el Si♭ de ese acorde
  y sigue con Si.
- **Un acorde puede compartir notas con el anterior.** Esas notas
  cuentan para la forma nueva si las necesita, o como notas repetidas
  si no.
- **Una serie, o una por pista.** Todas las pistas pueden compartir una
  forma, como suelen hacerlo una melodía y los acordes que la
  acompañan, o cada pista puede tocar formas propias, como en un canon.
  La revisión lee la serie de las dos maneras, se queda con la lectura
  con menos problemas, y la línea de resumen dice cuál eligió.
- **Los patrones se enlazan.** Cada patrón se revisa por separado, pero
  una serie puede seguir de un patrón al siguiente. Así, la primera
  forma puede haber empezado en el patrón anterior, siempre que sus
  notas de aquí estén en orden: la línea Serie dice entonces «…P4», y
  el resumen «P4 desde la nota 3». La última forma puede seguir en el
  patrón siguiente, así que nunca se marca como sin terminar.
- **Empates.** Cuando dos lecturas tienen la misma cantidad de
  problemas, la revisión prefiere formas que empiezan en su primera
  nota. Si dos formas siguen empatadas, nombra la primera en el orden
  P, I, R, RI. Los acordes pueden hacer que dos formas no se puedan
  distinguir: la serie del Concierto de Webern son cuatro grupos de
  tres notas, y sus formas R0 e I1 tienen las mismas tres notas en cada
  grupo, en otro orden. En el estilo Los tricordios de Webern, la
  melodía y los acordes tocan los dos R0 después de P0. Las notas de la
  melodía llegan de a una, así que solo pueden ser R0; los acordes
  hacen sonar cada grupo de una vez, así que encajan igual de bien con
  R0 que con I1, y la revisión los nombra I1, que va primero.
  Cualquiera de los dos nombres es correcto.

La revisión sigue las reglas clásicas del método de Schoenberg. No
juzga la **música atonal libre**, escrita sin ninguna serie (para eso
elige un conjunto de clases de alturas como tonalidad); el
**serialismo más allá de las alturas**, si las duraciones, las
dinámicas o los registros siguen también una serie; ni la
**combinatoriedad**: en el estilo Pads de hexacordios lee la forma
propia de cada pista, lo cual es correcto, pero no dice que las dos
mitades juntas forman las doce notas.

## Cómo lee la serie la revisión

### Los lugares de una forma en dieciséis bits

`Walk` recorre una forma de la serie, grupo por grupo, y recuerda cuáles
de los doce lugares han sonado. Son doce respuestas de sí o no, así que
las guarda como los bits de un `u16`, un bit por lugar:

```rust
#[derive(Debug, Clone, Copy)]
struct Walk {
    /// One bit for each place in the form, from the first.
    used: u16,
    /// The place after the furthest one sounded.
    next: usize,
    errors: usize,
}

impl Walk {
    fn from(from: usize) -> Walk {
        Walk {
            used: (1 << from) - 1,
            next: from,
            errors: 0,
        }
    }

    fn heard(self) -> usize {
        self.used.count_ones() as usize
    }

    fn has(self, place: usize) -> bool {
        self.used & (1 << place) != 0
    }
    // ...
}
```

`Walk::from(3)` empieza un recorrido cuyos tres primeros lugares ya
sonaron, en el patrón anterior o como las notas compartidas de una
elisión: `(1 << 3) - 1` es `0b111`. `Walk` es `Copy`, dos palabras de
máquina y un conjunto de bits, así que probar una forma no cuesta nada
de preparar. `count_ones` cuenta los lugares oídos en una sola
instrucción del procesador en la mayoría de las máquinas. Los
[conjuntos de clases de
altura](pitch.md#un-conjunto-de-clases-de-altura-en-doce-bits)
usan la misma idea, con doce bits para las doce notas.

### ¿Cuándo están en orden las notas nuevas?

Un grupo de *n* notas nuevas está en orden cuando llena los *n* lugares
siguientes de la forma, en cualquier orden entre ellas. «Los lugares
siguientes» puede querer decir dos cosas, y `window` prueba las dos:

```rust
fn open(self, places: std::ops::Range<usize>, count: usize) -> u16 {
    places
        .filter(|&place| !self.has(place))
        .take(count)
        .fold(0, |bits, place| bits | 1 << place)
}

fn window(self, taken: u16, count: usize) -> u16 {
    let first_open = self.open(0..12, count);
    let ahead = self.open(self.next..12, count);
    match (taken & ahead).count_ones() > (taken & first_open).count_ones() {
        true => ahead,
        false => first_open,
    }
}
```

`open` arma un conjunto de lugares con un `fold` que añade con OR un bit
por lugar. `first_open` son los primeros lugares todavía abiertos, lo
que recupera una nota saltada justo antes. `ahead` son los lugares
después de la nota más lejana oída, lo que sigue después de un salto.
Gana el que tiene más notas del grupo.

Toma P4 con Re y Si intercambiadas: Mi Fa Sol Do♯ Fa♯ Mi♭ La♭ **Si Re**
Do La Si♭. En Si, la forma pide Re, así que Si está fuera de lugar. En
Re, el primer lugar abierto es el de Re, así que Re está en orden, y
también todo lo que sigue.

Ahora toma P4 sin el Re: Mi Fa Sol Do♯ Fa♯ Mi♭ La♭ **Si** Do La Si♭. Si
está otra vez fuera de lugar. En Do, el primer lugar abierto sigue
siendo el de Re, que nunca llega. Pero el lugar después de la nota más
lejana oída, Si, es el de Do, así que Do está en orden, y también La y
Si♭. Solo se marca Si. Con `first_open` solo, cada nota después del Re
que falta estaría fuera de lugar.

### Un recorrido, dos trabajos

`take` recibe un grupo y encuentra lo que está fuera de lugar:

```rust
fn take(&mut self, form: &Form, group: &Group, mut tell: impl FnMut(Misplaced)) {
    // ...
    for &played in &group.notes {
        let place = place_of(&played);
        let kind = if repeat(&played) {
            continue;
        } else if self.has(place) {
            SlipKind::EarlyRepeat
        } else if window & (1 << place) == 0 {
            SlipKind::RowOrder
        } else {
            continue;
        };
        self.errors += 1;
        tell(Misplaced {
            kind,
            played,
            place,
            wanted,
            heard: self.heard(),
        });
    }
    self.used |= fresh;
    if fresh != 0 {
        self.next = self.next.max(16 - fresh.leading_zeros() as usize);
    }
}
```

`let kind = if ... { continue; } else ...` funciona porque `continue`
nunca da un valor: su tipo es `!`, «nunca», que encaja en lugar de
cualquier tipo, así que las otras ramas deciden que `kind` es un
`SlipKind`. Una nota que solo repite la nota recién tocada se salta de
esta manera, y también una nota en su lugar. Al final,
`16 - fresh.leading_zeros()` es el lugar que sigue al bit más alto de
`fresh`, el lugar más lejano al que llegó este grupo.

El mismo `take` cuenta los problemas mientras se comparan las lecturas,
y los informa para la lectura que gana. Hace las dos cosas con el
closure `tell`, al que llama por cada nota fuera de lugar. Al comparar,
el closure no hace nada: `walk.take(form, &groups[end - 1], |_| {})`.
Al informar, convierte cada nota en un hallazgo: `walk.take(form,
group, |misplaced| slips.push(misplaced.slip(form, key)))`. Ese segundo
closure cambia `slips`, un vector que toma prestado, así que es un
`FnMut`, no solo un `Fn`. Como `take` es genérica sobre el closure
(`impl FnMut`), el compilador hace una copia de `take` para cada uno, y
la vacía no cuesta nada.

### La mejor lectura: programación dinámica

`read` tiene que elegir dónde empieza y termina cada forma, y qué forma
es. `best[end]` es la menor cantidad de problemas en los primeros `end`
grupos, y `came_from[end]` es el último tramo en el camino hasta ahí:

```rust
for start in 0..n {
    if best[start] == usize::MAX {
        continue;
    }
    // ...
    for from in 0..12 {
        for (index, form) in forms.iter().enumerate() {
            // ... may this form start at its place `from` here? ...
            let mut walk = Walk::from(from);
            for end in start + 1..=reach {
                walk.take(form, &groups[end - 1], |_| {});
                // ...
                let missing = if end == n { 0 } else { 12 - walk.heard() };
                let cost = best[start] + walk.errors + missing;
                if cost < best[end] {
                    best[end] = cost;
                    came_from[end] = Stretch {
                        start,
                        end,
                        form: index,
                        from,
                    };
                }
            }
        }
    }
}
```

Cada tramo se prueba una vez por forma. El recorrido sigue de un `end`
al siguiente en vez de volver a empezar, así que una forma probada
sobre veinte grupos cuesta veinte pasos, no doscientos. Una forma puede
empezar en un lugar posterior (`from` mayor que 0) solo al principio de
todo, traída del patrón anterior, o donde las notas anteriores son las
que compartió el grupo anterior (una elisión). Las formas desde su
primera nota se prueban primero, y solo un costo estrictamente menor
reemplaza una lectura: así se llevan los empates. Recorrer `came_from`
hacia atrás desde el último grupo da las formas en orden.

`forms_of` hace la lista de formas a probar, saltando una forma que
tenga las mismas notas que otra ya incluida, así que el palíndromo de
Webern se lee con 24 formas, no 48. Y `read_rows` corre `read` dos
veces, una con todas las notas y otra por pista, y se queda con la más
barata:

```rust
if apart.iter().map(|(_, reading)| reading.cost).sum::<usize>() < together.cost {
```

La misma idea, guardar la mejor manera de llegar a cada punto, reparte
las notas de un acorde entre las partes en la revisión clásica
([Conducción de voces y movimiento de las
partes](voice-leading.md)).

### Notas que empiezan juntas: `chunk_by`

Los grupos salen de las notas ordenadas por comienzo, cortadas donde
cambia el comienzo:

```rust
for chunk in played.chunk_by(|a, b| a.note.start_tick() == b.note.start_tick()) {
```

`chunk_by` entrega porciones de vecinas para las que el closure dice
«mismo grupo», sin copiar nada.

## Cómo se prueba

`row.rs` comprueba la teoría contra los libros: las cuatro formas de
Schoenberg, nota por nota, contra las publicadas; cada una de las 48
formas de la Serie de Lineup, nombrada otra vez por `label_of`; cada
fila de la matriz de Berg una forma P y cada columna una forma I, con
la primera nota de la serie a lo largo de toda la diagonal; la serie de
Webern un palíndromo (R9 es P3); la serie de Berg, la serie Todos los
intervalos y cincuenta series al azar de todos los intervalos, de todos
los intervalos; la Serie de Lineup combinatoria solo con I3; series
leídas desde nombres, números y la forma compacta, y las equivocadas
rechazadas con el mensaje correcto; la misma semilla dando la misma
serie al azar; y una serie con una nota escrita dos veces rechazada al
leerla de un archivo:

```rust
#[test]
fn webern_is_a_palindrome() {
    let webern = RowPreset::named("Webern's Symphony").unwrap().row;
    let r9 = webern.form(RowLabel::new(RowForm::Retrograde, 9));
    assert_eq!(webern.label_of(r9).unwrap().to_string(), "P3");
    let mirror = RowPreset::named("Mirror row").unwrap().row;
    let backwards = mirror.form(RowLabel::new(RowForm::Retrograde, 0));
    assert_eq!(mirror.label_of(backwards).unwrap().to_string(), "P6");
}
```

`row.rs` también comprueba que cada forma de la Serie de Lineup tiene
una compañera de su mismo tipo, con las doce notas en cada mitad de las
dos, y que la serie 6-Z3 no tiene ninguna. `serial.rs` escribe una
melodía en corcheas, las cuatro formas por turno llenando el patrón,
tricordios como acordes, las tres series, una forma con su compañera
debajo (las doce notas en cada mitad), un plan de formas, y comprueba
que ninguna nota pasa del final del patrón. `library/rows.rs`
comprueba que cada estilo usa una serie de las predefinidas y escribe
sus voces en cualquier tonalidad; que toca la serie de la tonalidad
cuando la tonalidad es una serie, y la suya con Progresión de acordes;
y que Pads de hexacordios tiene las doce notas en cada compás.
`band/row.rs` comprueba que la banda toca el plan de cada estilo; que
toca un plan escrito a mano sobre la serie de Berg, forma por forma;
que Pads de hexacordios conserva las doce notas en cada compás con otra
serie; que una canción tonal toma un estilo de serie, parte tras parte;
que un canal Keys bloqueado conserva su parte; y que una banda así se
guarda y se vuelve a leer. Una prueba une las
dos mitades de este capítulo: escribe cada estilo, sobre Do y sobre
Fa♯, y le pasa la revisión de la conducción de voces, que tiene que
leer cada uno como formas de su serie sin ninguna nota fuera de orden,
así que los estilos y la revisión siguen las mismas reglas.

Las pruebas de las reglas en `voice_leading/serial.rs` escriben cada
regla y sus excepciones como notas sobre la serie de Schoenberg: una
serie en orden, acordes de notas vecinas, dos notas intercambiadas, una
nota que falta, repeticiones y trinos, una elisión, una forma sin
terminar, una melodía y unos acordes que comparten una forma, un canon,
duplicaciones a la octava, acordes tonales, y las reglas clásicas
apagadas en series y conjuntos. Sus nombres se leen como las reglas:
`two_notes_swapped_are_one_note_out_of_order`,
`a_note_may_be_repeated_or_trilled_but_not_brought_back_later`,
`one_form_may_end_on_the_note_the_next_begins_with`.

Los ejercicios sobre series están en [Ejercicios: la teoría musical en
código](exercises.md).
