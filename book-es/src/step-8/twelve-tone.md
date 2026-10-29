# Series dodecafónicas y conjuntos de clases de alturas

Archivos: [`crates/lineup-core/src/harmony/row.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/harmony/row.rs),
[`crates/lineup-core/src/harmony/set_class.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/harmony/set_class.rs),
[`crates/lineup-core/src/harmony/pitch_set.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/harmony/pitch_set.rs),
[`crates/lineup-core/src/serial.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/serial.rs),
[`crates/lineup-core/src/harmony/scale.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/harmony/scale.rs),
[`crates/lineup-core/src/library/rows.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/rows.rs)
y [`crates/lineup-app/src/twelve_tone.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/twelve_tone.rs)

Todo en Ferrous Lineup hasta ahora suponía música en una tonalidad: una
nota base, acordes que tiran hacia ella, teclas de armonía numeradas de I a
vii°. Gran parte de la música del siglo XX no tiene tonalidad alguna.
Este capítulo añade las dos herramientas principales que los músicos
usan para pensar en ella:

- **series dodecafónicas** (también llamadas *series*): las doce notas
  en un orden del que está hecha toda la pieza. Los menús de tonalidad
  ahora ofrecen **serie dodecafónica** después de las escalas, y **Serie…**
  en el piano roll abre una ventana para teclear, dibujar o elegir una
  serie, ver su **matriz** con las 48 formas y escribir cualquier forma
  en el patrón, como melodía o como acordes, con las duraciones,
  dinámicas y octavas siguiendo también la serie si quieres
  (**serialismo**);
- **conjuntos de clases de alturas**: un nombre para cada grupo de
  notas, acorde o no. **Conjuntos…** lista los 224 por **número de Forte**,
  para dibujar con ellos o usarlos como escala, y el piano roll ahora
  nombra el conjunto de las notas que seleccionas y el de las notas
  bajo el puntero.

La Biblioteca tiene un grupo de estilos **Series dodecafónicas**, y **Archivo > Demostraciones > Dodecafónico** toca cada uno como una pieza corta.

![La ventana Serie junto al piano roll. La serie es Do Mi Sol Re Fa♯ La La♯ Do♯ Fa Si Re♯ Sol♯, cuatro tríadas en fila; la matriz muestra sus 48 formas, con P0 seleccionada. El piano roll numera sus notas del 1 al 12 sobre las teclas, y las harmony keys tocan sus cuatro tricordos.](../images/step-8-twelve-tone-row.png)

![La ventana Conjuntos, buscando «octatonic»: el conjunto 8-28, su forma prima, su vector de intervalos y cuál es su complemento. Bajo la barra de herramientas del piano roll, las notas del paso bajo el puntero se nombran como conjunto 3-11.](../images/step-8-pitch-class-sets.png)

## Música sin nota base

En Do mayor, Do es la base. Sol tira hacia ella, Si tira con fuerza, y una
pieza que termina en Do suena terminada. Hacia 1921, Arnold Schoenberg
buscó una manera de escribir música en la que ninguna nota fuera la
base. Su respuesta fue usar las doce notas por igual, en un orden fijo
que elegía para cada pieza: la **serie**. Ninguna nota puede volver
hasta que las otras once hayan sonado, así que ninguna llega a ser el
centro.

Esta es la serie de la Suite para piano, Op. 25, de Schoenberg, de
1925, una de las primeras piezas construidas enteramente a partir de
una:

```text
E  F  G  C#  F#  D#  G#  D  B  C  A  A#
```

Una serie sería aburrida si solo pudiera tocarse de una manera, así que
viene en cuatro:

| forma | cómo | la serie de Schoenberg en Mi |
|------|-----|------------------------|
| **P**, original | tal como está escrita | Mi Fa Sol Do♯ Fa♯ Re♯ Sol♯ Re Si Do La La♯ |
| **I**, inversión | cada intervalo al revés: donde P sube un semitono, I baja uno | Mi Re♯ Do♯ Sol Re Fa Do Fa♯ La Sol♯ Si La♯ |
| **R**, retrógrada | P al revés | La♯ La Do Si Re Sol♯ Re♯ Fa♯ Do♯ Sol Fa Mi |
| **RI**, inversión retrógrada | I al revés | La♯ Si Sol♯ La Fa♯ Do Fa Re Sol Do♯ Re♯ Mi |

Cada una de las cuatro puede empezar en cualquiera de las doce notas, lo
que da **48 formas**. Se nombran con una letra y un número: el número es
la nota en la que empieza la forma, con Do como 0, Do♯ como 1, hasta Si
como 11. Así que la serie de arriba es **P4** (empieza en Mi), e **I0**
es su inversión empezando en Do. Una retrógrada recibe el número de la
forma que invierte: **R4** es P4 al revés, así que *termina* en Mi.

Algunos libros numeran las formas de otra manera, desde la primera nota
de la propia serie, de modo que la serie tal como está escrita sea
siempre P0. Entonces la serie de Schoenberg es P0 y lo que Ferrous
Lineup llama I11 allí es I7. Las formas son las mismas; solo se mueven
sus números. Ferrous Lineup usa la primera manera (la nota en la que
empieza una forma), porque así la etiqueta te dice una nota que puedes
encontrar en el teclado.

### La matriz

Las 48 formas caben en un cuadrado de 12×12. La serie va por arriba, su
inversión desde la misma nota baja por el lado izquierdo, y cada línea
*i* es la serie movida para empezar en la nota de la izquierda de esa
línea:

```text
      I4  I5  I7  I1  I6  I3  I8  I2  I11 I0  I9  I10
P4    E   F   G   C#  F#  D#  G#  D   B   C   A   A#    R4
P3    D#  E   F#  C   F   D   G   C#  A#  B   G#  A     R3
P1    C#  D   E   A#  D#  C   F   B   G#  A   F#  G     R1
P7    G   G#  A#  E   A   F#  B   F   D   D#  C   C#    R7
...
      RI4 RI5 RI7 RI1 ...
```

Lee una línea de izquierda a derecha y es una forma P; de derecha a
izquierda, una forma R. Lee una columna de arriba abajo y es una forma
I; de abajo arriba, una forma RI. Las etiquetas alrededor del cuadrado
dicen cuál: P a la izquierda, R a la derecha, I arriba, RI abajo. En la
ventana Serie, haz clic en cualquier etiqueta para elegir esa forma, y su
línea o columna se ilumina.

La diagonal de arriba a la izquierda a abajo a la derecha es la misma
nota todo el camino (Mi, aquí). Es una comprobación rápida de que una
matriz se rellenó correctamente.

## Una serie en Rust: `ToneRow`

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct ToneRow([u8; 12]);
```

Una serie son doce números, los semitonos por encima de Do de sus notas.
Es un **newtype** alrededor de un array: el array es privado, así que
las únicas maneras de crear un `ToneRow` son funciones que lo
comprueban:

```rust
pub fn new(semitones: [u8; 12]) -> Option<Self> {
    let set: PitchSet = PitchSet::from_semitones(&semitones);
    (set.len() == 12 && semitones.iter().all(|&n| n < 12)).then_some(ToneRow(semitones))
}
```

Reúne los doce números en un conjunto de clases de alturas (abajo), y
si el conjunto tiene doce miembros, ninguna nota se repitió. Cada
`ToneRow` del programa es, por tanto, una serie real: el **invariante**
se comprueba una vez, aquí, y nada más tiene que preocuparse por él.

### Archivos: `try_from` e `into`

Los dos atributos de serde del struct dicen: «para guardar un
`ToneRow`, conviértelo en un `String`; para leer uno, lee un `String` e
intenta convertirlo en un `ToneRow`». Las conversiones son
implementaciones de traits ordinarias:

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

Así que un archivo de proyecto contiene `"4 5 7 1 6 3 8 2 11 0 9 10"`,
que una persona puede leer, en lugar de una lista de doce números entre
corchetes. Y como la lectura pasa por `parse`, un archivo con una nota
escrita dos veces se rechaza con un mensaje, en lugar de producir una
serie rota. `PitchSet` recibe el mismo par, así que un conjunto se
guarda como `"0 3 6 9"`.

`parse` es también lo que lee la caja de arriba de la ventana Serie.
Acepta nombres de notas (`E F G Db Gb Eb Ab D B C A Bb`), números
(`4 5 7 1 …`) o la forma compacta que usan los libros de dodecafonismo,
un carácter por nota con **T** y **Mi** para diez y once
(`45716382E09T`). Cuando falta una nota dice cuál, y cuál vino dos
veces en su lugar.

**Desde las notas** lee una serie del piano roll: las primeras doce
clases de alturas distintas entre las notas seleccionadas (o todas
ellas), en el orden en que empiezan. `ToneRow::from_notes` se salta una
nota cuya clase de alturas ya está en la serie, así que puedes dibujar
con libertad y repetir notas, mientras aparezcan las doce.

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

Cada nota se mide primero como `up`, su distancia por encima de la
primera nota de la serie (de 0 a 11). Una forma P la pone a la misma
distancia por encima del nuevo inicio `n`; una forma I la pone a la
misma distancia *por debajo*. Eso es todo lo que es una inversión. El
`+ 12` antes de cada `%` evita que los números bajen de cero: son `u8`,
y `4 - 7` no daría la vuelta a la octava, sino que detendría el programa
en una compilación de depuración.

`self.0.map(...)` es el `map` de un **array**, no de un iterador: llama
a la closure sobre cada uno de los doce elementos y devuelve un nuevo
`[u8; 12]`, sin `Vec` y sin `collect`. La longitud es parte del tipo,
así que el resultado sigue siendo exactamente doce notas.

Con `form`, la matriz son dos líneas:

```rust
pub fn matrix(self) -> [[u8; 12]; 12] {
    let inversion = self.form(RowLabel::new(RowForm::Inversion, self.0[0]));
    inversion
        .0
        .map(|start| self.starting_on(PitchClass::from_semitone(start)).0)
}
```

La inversión desde la primera nota de la serie es la columna izquierda;
cada una de sus notas inicia una línea.

Y en la otra dirección, «¿qué forma de esta serie es esa?», prueba las
48 hasta que una coincide:

```rust
pub fn label_of(self, other: ToneRow) -> Option<RowLabel> {
    RowForm::ALL.into_iter().find_map(|form| {
        (0..12)
            .map(|n| RowLabel::new(form, n))
            .find(|&label| self.form(label) == other)
    })
}
```

## Qué hace interesante a una serie

La ventana Serie lista qué tiene de especial la serie que estás mirando.
Cada línea es una función corta.

**Los pasos.** `intervals()` da los once pasos de nota a nota, en
semitonos hacia arriba. Para la serie de Schoenberg son
`1 2 6 5 9 5 6 9 1 9 1`. Se construye con `std::array::from_fn`, que
hace un array llamando a una closure con cada índice:

```rust
pub fn intervals(self) -> [u8; 11] {
    std::array::from_fn(|i| (self.0[i + 1] + 12 - self.0[i]) % 12)
}
```

**Series de todos los intervalos.** Cuando los once pasos son todos
distintos, cada intervalo desde un semitono hasta una séptima mayor
aparece exactamente una vez. La Suite lírica de Alban Berg (1927)
empieza con una serie así: Fa Mi Do La Sol Re Sol♯ Do♯ Re♯ Fa♯ La♯ Si, con el tritono
justo en el medio. Son raras: de las 39,916,800 series que empiezan en
Do, solo 3,856 tienen todos los intervalos. Así que el botón
**Todos los intervalos** no puede limitarse a barajar notas hasta encontrar una;
construye la serie nota a nota:

```rust
fn all_interval_from(notes: &mut Vec<u8>, random: &mut Random) -> Option<Vec<u8>> {
    if notes.len() == 12 {
        return Some(notes.clone());
    }
    let used: Vec<u8> = notes.windows(2).map(|w| (w[1] + 12 - w[0]) % 12).collect();
    let mut intervals: Vec<u8> = (1..12).filter(|i| !used.contains(i)).collect();
    // ... shuffle `intervals` ...
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

Esto es **backtracking**: prueba cada paso aún no usado, en orden
aleatorio; si la nota a la que llega está libre, la añade y sigue desde
ahí (la función se llama a sí misma); si eso no lleva a ninguna parte,
retira la nota (`pop`) y prueba el siguiente paso. Cuando ningún paso
funciona, devuelve `None` para que quien la llamó retire *su* nota.
`windows(2)` entrega cada par de vecinos, `[a, b]`, que es exactamente
lo que es un paso.

**Series aleatorias.** El botón **Al azar** baraja las doce notas con la
barajación de Fisher-Yates: recorre del último lugar al primero e
intercambia cada uno con un lugar elegido al azar entre los que aún no
están hechos. Todos los órdenes quedan entonces igual de probables. Los
números aleatorios salen de un generador diminuto, *SplitMix64*, escrito
en quince líneas en lugar de añadir una biblioteca:

```rust
fn next(&mut self) -> u64 {
    self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
    let mut z = self.0;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}
```

`wrapping_add` y `wrapping_mul` dicen «aquí desbordar está bien, solo da
la vuelta»: un `+` normal que se desbordara detendría una compilación de
depuración. La misma semilla da siempre la misma serie, lo que hace
repetibles las pruebas.

**Grupos de notas.** Los compositores suelen tocar una serie en grupos:
cuatro tricordos (tres notas cada uno), tres tetracordos, dos
hexacordos. `segments(size)` corta la serie con `chunks_exact`, y la
ventana nombra la clase de conjuntos de cada grupo (mira la segunda
mitad de este capítulo). Los grupos de la serie de Schoenberg son tres
tetracordos distintos, 4-12, 4-Z15 y 4-1; la serie Cuatro tríadas son
cuatro grupos de 3-11, las tríadas mayor y menor.

**Combinatorialidad.** Toma las seis primeras notas de una serie. Alguna
otra forma de la misma serie puede empezar exactamente con las *otras*
seis. Toca las dos formas a la vez, y cada mitad de la música contiene
las doce notas, sin que ninguna se repita antes de tiempo. Schoenberg
construyó sus últimas piezas sobre esto, y se llama **combinatorialidad
hexacordal**:

```rust
pub fn combinatorial_forms(self) -> Vec<RowLabel> {
    let missing = PitchSet::from_semitones(&self.0[..6]).complement();
    [RowForm::Prime, RowForm::Inversion, RowForm::RetrogradeInversion]
        .into_iter()
        .flat_map(|form| (0..12).map(move |n| RowLabel::new(form, n)))
        .filter(|&label| PitchSet::from_semitones(&self.form(label).0[..6]) == missing)
        .collect()
}
```

La retrógrada se deja fuera porque siempre funciona (su primera mitad es
la segunda mitad de la serie), así que no dice nada sobre la serie. La
serie del Op. 25 de Schoenberg es combinatoria con I11; la serie Lineup,
escrita para este capítulo, con I3. El estilo **Pads de hexacordios** toca
exactamente eso: una mitad de la serie como acorde, y las otras seis
notas de I3 como melodía encima.

El `move` de `move |n|` es necesario porque la closure interior sale de
la exterior a través de `flat_map`, y usa `form`: `move` hace que se
lleve su propia copia de `form` en lugar de tomar prestada una variable
que está a punto de desaparecer.

### Las series para empezar

El menú **Series famosas y de ejemplo** contiene siete series. Tres vienen
de piezas famosas, y cuatro se escribieron para Ferrous Lineup para
mostrar lo que puede hacer una serie: una combinatoria, una de cuatro
tríadas, un palíndromo y una de todos los intervalos.

| serie | de | qué muestra |
|-----|------|---------------|
| Mi Fa Sol Do♯ Fa♯ Re♯ Sol♯ Re Si Do La La♯ | Schoenberg, Suite Op. 25 (1925) | una de las primeras piezas dodecafónicas; Si Do La La♯ deletrean BACH con los nombres alemanes de las notas, en desorden |
| Fa Mi Do La Sol Re Sol♯ Do♯ Re♯ Fa♯ La♯ Si | Berg, Suite lírica (1927) | una serie con todos los intervalos |
| La Fa♯ Sol Sol♯ Mi Fa Si La♯ Re Do♯ Do Re♯ | Webern, Sinfonía Op. 21 (1929) | un palíndromo: al revés, es la misma serie un tritono más arriba (R9 es P3) |

Las tres se publicaron antes de 1930, así que son de dominio público en
Estados Unidos, y sus compositores murieron en 1951, 1935 y 1945, así
que también son de dominio público donde el copyright dura 70 años tras
la muerte del autor. Y solo se usan las series: doce notas cada una.

La serie de Webern muestra que una serie no siempre tiene 48 formas
*distintas*. Como su R9 es su P3, toda forma R es también una forma P, y
toda forma RI una forma I: solo 24 son distintas. `label_of` prueba
primero las formas P, así que una forma así se nombra como la forma P
que también es.

## De una serie a notas: el serialismo

Una serie dice qué nota viene después, y nada más. Para hacer música con
ella, alguien tiene que decidir aún cuánto dura cada nota, qué tan
fuerte, en qué octava, y si algunas notas suenan juntas. `RowWriting`
guarda esas elecciones, y los controles bajo la matriz las fijan:

```rust
pub struct RowWriting {
    pub label: RowLabel,       // the form to start with, such as P0
    pub chain: Chain,          // then: once, repeat, or P, R, I, RI in turn
    pub together: usize,       // 1 for a melody, 3 for trichords, 6 for hexachords
    pub step: usize,           // how long each note or chord lasts, in steps
    pub register: Register,    // close, smooth or wide
    pub low: Note,             // the lowest note
    pub durations: bool,       // serialism: durations follow the row
    pub dynamics: bool,        // dynamics follow the inversion
    pub octaves: bool,         // octaves follow the retrograde
}
```

Los tres **registros** deciden las octavas cuando la serie no lo hace:

- **Cerrar** pone cada nota en la octava por encima de la nota más grave;
- **Suave** pone cada nota tan cerca como puede de la anterior, lo que
  hace una línea cantable (un tritono, justo a la mitad, sube);
- **Amplio** manda las notas grave, aguda, media, grave, aguda, media a lo
  largo de tres octavas, las líneas saltarinas de Anton Webern.

### Serialismo integral

Hacia 1950, Olivier Messiaen, Pierre Boulez y Milton Babbitt llevaron la
idea más lejos: si las alturas siguen una serie, ¿por qué no también las
duraciones, las dinámicas y las octavas? Esto es el serialismo
**integral** (o total). Las piezas reales asignan series a duraciones y
dinámicas de muchas maneras; Ferrous Lineup usa tres simples, cada una
leyendo una forma distinta de la serie para que las cuatro series no
avancen al unísono:

- **Duraciones**: los números de la serie (la distancia de cada nota por
  encima de la primera, de 0 a 11) se convierten en longitudes. La nota
  *k*-ésima dura tantos pasos como el número *k*-ésimo, más uno, así que
  de 1 a 12 semicorcheas.
- **Dinámicas**: los números de la inversión se convierten en doce
  velocidades, de muy suave a muy fuerte: `28 + 9 * level`.
- **Octavas**: los números de la retrógrada, contados de tres en tres
  (`% 3`), eligen una de tres octavas.

Dentro de `write_row`, cada nota hace esas preguntas por turno:

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

`place` cuenta notas a lo largo de las formas, así que con
**P, R, I, RI** la serie de duraciones continúa de una forma a la
siguiente en lugar de empezar de nuevo.

`write_row` es una función pura: entran una serie, las elecciones y una
longitud de patrón; sale una lista de `NoteEvent`. La ventana la llama
en cada frame para mostrar un aviso cuando el patrón es demasiado corto
para la serie completa («El patrón solo tiene espacio para 9 de las 12
notas»), y el botón **Escribir… en el patrón actual** envía las mismas notas al piano roll, que
reemplaza las notas del canal mediante la edición habitual, así que
**Ctrl+Z** trae de vuelta las anteriores.

## Una serie como tonalidad

Los menús de tonalidad ahora listan dos tipos más de tonalidad bajo las
escalas: **serie dodecafónica** y **conjunto de clases de alturas**. En el código, son
dos variantes nuevas de `ScaleKind` y, a diferencia de las otras, llevan
datos:

```rust
pub enum ScaleKind {
    Major,
    NaturalMinor,
    // ... the other scales ...
    Chromatic,
    /// A twelve-tone row, stored starting on C. The scale's root is the
    /// note it starts on.
    Row(ToneRow),
    /// A pitch-class set used as a scale: its notes as semitones above
    /// the root, the root among them.
    Set(PitchSet),
}
```

La serie se guarda empezando en Do, y la raíz del `Scale` dice dónde
empieza de verdad, así que «la serie P4» es `Scale { root: E, kind:
Row(...) }` y el menú de raíz la mueve como a cualquier otra tonalidad.
Tanto `ToneRow` (12 bytes) como `PitchSet` (2 bytes) son `Copy`, así que
`ScaleKind` y `Scale` siguen siendo `Copy` también, y nada del código
que pasa un `Scale` de un lado a otro tuvo que cambiar.

Tres cosas sí cambiaron, y cada una dice algo sobre Rust:

- **`ScaleKind::ALL` ya no lista todos los tipos.** Una lista de todas
  las series tendría 39,916,800 entradas. `ALL` ahora lista las trece
  escalas de la música tonal, y los menús añaden la serie y el conjunto
  después de un separador, con la serie o el conjunto usados en último
  lugar (guardados en la memoria de egui, como en [Adaptarse a cualquier
  ventana](fitting-the-window.md)).
- **`intervals()` devuelve un `Vec<u8>` en lugar de un `&'static [u8]`.**
  Las notas de las escalas eran arrays constantes guardados con el
  programa, así que tomar prestado de ellos con `'static` era gratis.
  Las notas de una serie solo se conocen mientras el programa corre,
  dentro de un valor que va a desaparecer; no hay nada `'static` que
  tomar prestado, así que la función tiene que entregar su propia copia.
  Los arrays constantes se mudaron a un `tonal_intervals()` privado.
- **`is_tonal()`** responde «¿tiene esta tonalidad una nota base?» con
  `matches!`. `is_heptatonic()`, a la que los acordes de la tonalidad y
  los números romanos ya preguntaban antes de hacer nada, ahora le
  pregunta también: un conjunto de siete notas no es una escala mayor,
  aunque tenga siete notas.

### Qué cambia la tonalidad

Con una serie como tonalidad, el piano roll numera sus notas de **1 a
12** sobre las teclas, en el orden de la serie, así que puedes dibujar
la serie siguiendo los números. `degree_of` ya daba el lugar de una nota
en una escala; para una serie, la «escala» es la serie en su propio
orden, así que el lugar en la serie sale gratis.

Las teclas de armonía tocan los grupos de la serie en lugar de acordes:
cuatro teclas para los tricordos (**1–3**, **4–6**, **7–9**, **10–12**),
o tres para tetracordos, o dos para hexacordos. La opción **3 notes / 4
notes / 6 notes** ocupa el lugar de Tríadas, Séptimas y Novenas. Para un
conjunto, cada tecla apila una de cada dos notas del conjunto desde una
de sus notas, igual que los acordes de una tonalidad apilan una de cada
dos notas de la escala.

### Cambiar de tonalidad entre tipos

[Cambiar de tonalidad](changing-key.md) mueve cada nota a la nueva
tonalidad. Pero de Do mayor a una serie, ¿adónde iría Mi? No hay una
respuesta sensata, así que `change_key` ahora pregunta primero si los
dos tipos tienen algo en común:

```rust
pub fn notes_follow(self, other: ScaleKind) -> bool {
    match (self, other) {
        (ScaleKind::Row(a), ScaleKind::Row(b)) => a == b,
        (ScaleKind::Set(_), ScaleKind::Set(_)) => true,
        (a, b) => a.is_tonal() && b.is_tonal(),
    }
}
```

Hacer pattern matching sobre una **tupla** de dos valores cubre cada par
de tipos en tres líneas: la misma serie en otra nota mueve las notas
(eso es transponer), dos conjuntos también, dos tonalidades tonales se
comportan como antes, y cualquier otra cosa deja las notas donde están y
solo cambia lo que el piano roll resalta.

### Formato 6

Un proyecto cuya tonalidad es una serie o un conjunto se guarda como
**formato 6**. Las versiones antiguas de Ferrous Lineup no conocen estos
tipos, así que no podrían leer la tonalidad; con el número nuevo dicen
que el archivo es de una versión más reciente en lugar de fallar de
forma confusa. Como siempre, `format_for` escribe el formato más bajo
que necesita un proyecto, así que los proyectos en una tonalidad
ordinaria siguen siendo legibles por versiones anteriores. En el
archivo, la tonalidad se ve así:

```text
scale: (root: C, kind: Set("0 3 6 9")),
```

## Conjuntos de clases de alturas

Los nombres de acordes sirven para los acordes de la música tonal.
Mucha música del siglo XX usa grupos de notas que no tienen nombre de
acorde, como Do, Do♯ y Fa♯. La **teoría de conjuntos de clases de alturas**,
elaborada por Milton Babbitt, Allen Forte, John Rahn y otros, da un
nombre a cada grupo de notas preguntando qué permanece igual cuando lo
mueves.

**Las clases de alturas como números.** Do es 0, Do♯ es 1, hasta Si, 11.
Las octavas y las grafías no cuentan: todo Do es 0, y Do♯ y Re♭ son ambos
1. Un **conjunto** es una elección de algunas de las doce; hay 2¹² =
4,096, del conjunto vacío a las doce.

**Mover un conjunto.** *Transponer* (T*n*) mueve cada nota *n* semitonos
hacia arriba: Do Mi Sol subido 3 es Mi♭ Sol Si♭. *Invertir* (I) pone cada
intervalo al revés: Do Mi Sol (sube 4, luego sube 3) se convierte en Do La♭ Fa
(baja 4, luego baja 3), que es Fa menor. Al revés, una tríada mayor es
una tríada menor.

**Clases de conjuntos.** Un conjunto junto con todas sus transposiciones
e inversiones es una **clase de conjuntos**. Las 24 tríadas mayores y
menores son una sola clase. Los 4,096 conjuntos se reparten en **224
clases**.

### Forma normal, forma prima y números de Forte

Para nombrar una clase, cada conjunto de ella debe salir con el mismo
nombre. Dos pasos llevan hasta ahí. Toma el acorde de séptima mayor de
Do, Mi Sol Si Do, como ejemplo.

La **forma normal** es la manera más compacta de escribir el conjunto.
Escrito alrededor de un círculo, un conjunto puede empezar en cualquiera
de sus notas. Empieza en cada una por turno y mide la amplitud de la
primera nota a la última:

| empezando en | notas | amplitud | penúltima |
|-------------|-------|------|----------------|
| Do | Do Mi Sol Si | 11 | |
| Mi | Mi Sol Si Do | 8 | 7 |
| Sol | Sol Si Do Mi | 9 | |
| Si | Si Do Mi Sol | 8 | 5 |

Mi y Si empatan con una amplitud de 8, así que se compara la amplitud
hasta la nota anterior a la última: de Si a Mi es 5, de Mi a Si es 7. La
forma normal es **Si Do Mi Sol**. (Este desempate, comparando desde el final,
es de John Rahn; Allen Forte comparaba desde el principio. Discrepan en
seis clases. Ferrous Lineup usa el de Rahn, como hacen ahora la mayoría
de los libros de texto.)

La **forma prima** va un paso más allá: toma las formas normales del
conjunto *y de su inversión*, mueve ambas para que empiecen en 0 y
quédate con la más compacta. Para Cmaj7, es **[0158]**: Si Do Mi Sol movido
hacia abajo para empezar en 0 es 0 1 5 8. Todo acorde de séptima mayor,
sobre cualquier raíz, y su inversión, tiene la forma prima [0158].

Los **números de Forte** listan las clases por tamaño, y luego en el
orden de Forte: [0158] es **4-20**, la vigésima clase de cuatro notas.
Las tríadas son 3-11, la séptima disminuida 4-28, la escala mayor 7-35,
la escala de tonos enteros 6-35. Los libros sobre música del siglo XX
usan estos números todo el tiempo, así que verlos en el piano roll hace
que esos libros sean más fáciles de seguir.

### El vector de intervalos

Cada par de notas de un conjunto forma un intervalo. Contarlos por
tamaño da seis números: cuántas segundas menores, segundas mayores,
terceras menores, terceras mayores, cuartas y tritonos. Una quinta
cuenta como una cuarta (es una cuarta al revés) y una séptima mayor como
una segunda menor, así que bastan seis tamaños. Este es el **vector de
intervalos**, y es un resumen justo de cómo *suena* un conjunto. Para
Do Mi Sol Si:

```text
C-E tercera mayor   C-G cuarta   C-B segunda menor
E-G tercera menor   E-B cuarta   G-B tercera mayor
<1 0 1 2 2 0>, escrito <101220>
```

De él se siguen dos curiosidades:

- **Los conjuntos Z-relacionados** tienen el mismo vector de intervalos
  sin estar en la misma clase. 4-Z15 [0146] y 4-Z29 [0137] contienen
  ambos un intervalo de cada tamaño, `<111111>` (son los dos
  **tetracordos de todos los intervalos**), y aun así ninguno puede
  moverse hasta el otro. La **Z** de sus nombres los marca.
- **Complementos**: las notas que un conjunto deja fuera forman otro
  conjunto. Las teclas blancas (7-35) dejan fuera las negras, la escala
  pentatónica (5-35). Un hexacordo y su complemento siempre tienen el
  mismo vector de intervalos, y por eso los hexacordos importan tanto
  para la combinatorialidad.

### Conjuntos en bits

Desde el [paso 3](../step-3/theory.md), un `PitchSet` son doce bits de
un `u16`: el bit 0 para Do, el bit 1 para Do♯, hasta el bit 11 para Si.
Resulta que la teoría de conjuntos encaja notablemente bien con esos
bits.

**Transponer es rotar los bits.** Mueve cada nota `shift` semitonos
hacia arriba: desplaza los bits a la izquierda, y los que se caen por
arriba (más allá de Si) vuelven a entrar por abajo (Do):

```rust
pub fn transpose(self, semitones: u8) -> PitchSet {
    let shift = (semitones % 12) as u32;
    let bits = self.0 as u32;
    let rotated = (bits << shift) | (bits >> (12 - shift));
    PitchSet(rotated as u16 & ALL_BITS)
}
```

**El complemento es `!`.** «Not» invierte cada bit, y la máscara
conserva los doce que significan notas:

```rust
pub fn complement(self) -> PitchSet {
    PitchSet::from_bits(!self.bits())
}
```

**La forma más compacta es el número más pequeño.** Esta es la parte
elegante. Mueve un conjunto hacia abajo para que su primera nota sea Do
(bit 0). Su última nota es ahora su bit más alto, así que una amplitud
menor significa un bit más alto menor, lo que significa un número menor.
En un empate, el siguiente bit hacia abajo es la nota anterior a la
última, que es exactamente el desempate de Rahn. Comparar dos números
*es* la regla de Rahn:

| empezando en | movido a Do | bits | número |
|-------------|-----------|------|--------|
| Do | 0 4 7 11 | `1000_1001_0001` | 2193 |
| Mi | 0 3 7 8 | `0001_1000_1001` | 393 |
| Sol | 0 4 5 9 | `0010_0011_0001` | 561 |
| Si | 0 1 5 8 | `0001_0010_0011` | 291 |

Así que la forma normal es «el inicio que da el número más pequeño», y
la forma prima es «el número más pequeño de las 24 transposiciones e
inversiones»:

```rust
pub fn normal_form(self) -> Vec<PitchClass> {
    let Some(start) = self
        .iter()
        .min_by_key(|start| self.relative_to(*start).bits())
    else {
        return Vec::new();
    };
    self.relative_to(start)
        .iter()
        .map(|pitch_class| pitch_class.transpose(start.semitone() as i32))
        .collect()
}

pub fn prime_form(self) -> PitchSet {
    let inverted = self.invert();
    (0..12)
        .flat_map(|n| [self.transpose(n), inverted.transpose(n)])
        .min_by_key(|set| set.bits())
        .unwrap_or(self)
}
```

`min_by_key` recorre un iterador y se queda con el elemento de clave más
pequeña. En `prime_form`, `flat_map` convierte cada `n` en un array de
dos conjuntos, así que el iterador produce 24 conjuntos, y el más
pequeño es la forma prima. Sin ordenar, sin listas de intervalos: unas
pocas comparaciones de enteros.

El vector de intervalos es el único lugar que cuenta pares por el camino
lento, un bucle dentro de otro bucle, y con doce notas como máximo son
como mucho 66 pares:

```rust
pub fn interval_vector(self) -> [u8; 6] {
    let notes = self.semitones();
    let mut vector = [0; 6];
    for (i, low) in notes.iter().enumerate() {
        for high in &notes[i + 1..] {
            let interval = high - low;
            let class = interval.min(12 - interval);
            vector[class as usize - 1] += 1;
        }
    }
    vector
}
```

`interval.min(12 - interval)` pliega un intervalo y su inversión en uno:
7 (una quinta) se convierte en 5 (una cuarta).

### La tabla de 224 clases, y cómo fiarse de ella

La forma prima se puede calcular, pero el *número* de Forte no: es solo
el lugar en la lista de Forte. Así que `set_class.rs` guarda la lista,
como un array constante de 224 entradas:

```rust
const SET_CLASSES: [SetClass; 224] = [
    SetClass::new(0, 1, false, 0),
    SetClass::new(1, 1, false, 0b000000000001),
    SetClass::new(2, 1, false, 0b000000000011),
    // ...
    SetClass::new(4, 20, false, 0b000100100011),  // [0158]
    // ...
    SetClass::new(12, 1, false, 0b111111111111),
];
```

`SetClass::new` y `PitchSet::from_bits` son `const fn`, así que toda la
tabla se construye al compilar y se guarda con el programa; buscar un
conjunto no cuesta nada al arrancar. El `false`/`true` dice si la clase
tiene una Z en su nombre.

La tabla se generó a partir de la de music21, una biblioteca musical
para Python (licencia BSD), y 224 líneas de números son fáciles de
equivocar. Así que las pruebas la comprueban contra la teoría en lugar
de fiarse de ella:

- cada entrada es su propia forma prima, y las entradas están en orden
  de tamaño y número;
- hay 1, 1, 6, 12, 29, 38, 50, 38, 29, 12, 6, 1 y 1 clases de 0 a 12
  notas, las cuentas que da cualquier libro de texto;
- **los 4,096 conjuntos** se buscan, y cada uno pertenece exactamente a
  una clase del tamaño correcto;
- una clase tiene una Z exactamente cuando otra clase de su tamaño tiene
  el mismo vector de intervalos.

Gracias a la tercera prueba, la búsqueda puede decir lo que espera sin
manejar un «no encontrado» que no puede ocurrir:

```rust
pub fn of(set: PitchSet) -> &'static SetClass {
    let prime = set.prime_form();
    SET_CLASSES
        .iter()
        .find(|class| class.prime == prime)
        .expect("every prime form is in the table")
}
```

`expect` hace panic con ese mensaje si el `Option` es `None`. Usarlo es
una promesa, y la prueba es lo que cumple la promesa.

La función devuelve `&'static SetClass`: un préstamo de una entrada de
la tabla constante, que vive tanto como el programa, así que quienes la
llaman pueden quedársela todo lo que quieran.

## El navegador de conjuntos, y nombrar lo que tocas

**Conjuntos…** en el piano roll abre una lista de cada clase del tamaño que
elijas (de 2 a 10 notas), cada una con su número de Forte, forma prima,
vector de intervalos y, para las famosas, un nombre: «3-11 [037]
<001110> tríada mayor o menor», «6-35 [02468T] <060603> escala de tonos
enteros». La caja de búsqueda encuentra una clase por su número de Forte
o por su nombre («triad», «octatonic», «4-Z15»), y **Mostrarlo** encuentra
la clase de las notas seleccionadas en el piano roll. Al elegir una se
ve su complemento, su pareja Z si la tiene, y si es **symmetric** (al
revés es solo una transposición, como en la séptima disminuida). **Transportarlo** e **Invertido** eligen qué miembro de la clase usar: T3 es el
conjunto tres semitonos arriba, T3I su inversión movida tres arriba.
Luego puedes:

- **Dibujar con él**: el conjunto se convierte en el sello de acordes del
  piano roll, así que cada clic coloca el conjunto entero por encima de
  la nota en la que hiciste clic. Es una variante nueva,
  `ChordStamp::Set(PitchSet)`, junto a los sellos de acordes del
  [paso 6](../step-6/tools.md);
- **Usar como tonalidad**: el conjunto se convierte en la escala, así que el
  piano roll resalta sus notas y las teclas de armonía tocan acordes
  apilados desde él.

Y el piano roll ahora nombra cualquier grupo de notas. Bajo la barra de
herramientas, junto al acorde del paso bajo el puntero, muestra el
conjunto de ese paso cuando suenan allí tres notas o más, y el conjunto
de las notas seleccionadas cuando hay dos o más seleccionadas:

```text
Conjunto: 3-11 · forma normal G# B D# · forma prima [037] · intervalos <001110> · tríada mayor o menor
```

`vector_text` escribe las seis cuentas como dígitos, `<001110>`, como
hacen los libros, hasta que una de ellas llega a 10: las doce notas
tienen doce de cada intervalo salvo tritonos, y eso se escribe con
espacios, `<12 12 12 12 12 6>`, para que no pueda leerse mal.

## Estilos y demos dodecafónicos

La Biblioteca tiene un grupo nuevo, **Series dodecafónicas**, con seis estilos:

| estilo | qué hace |
|-------|--------------|
| Melodía de la serie | la serie Lineup como melodía en corcheas, a través de P, R, I y RI |
| Coral de tríadas | la serie Cuatro tríadas como acordes, sobre su retrógrada en el bajo: música dodecafónica que suena casi tonal |
| Pads de hexacordios | media serie como acorde, y las otras seis notas de I3 como melodía: combinatorialidad hexacordal |
| Puntillismo | notas sueltas saltando a lo largo de tres octavas, duraciones y dinámicas de la serie, como en Webern |
| Serialismo integral | alturas, duraciones, dinámicas y octavas, todas de la serie de todos los intervalos |
| Lo-fi dodecafónico | los tricordos de la serie sobre un pulso lo-fi: una serie también puede tener groove |

Un estilo de armonía era una progresión de acordes y una manera de
tocarla. Un estilo de serie no tiene progresión, así que es una serie y
una lista de **voces**, cada una diciendo cómo toca la serie un
instrumento. Como con el resto de la biblioteca, son datos:

```rust
RowStyle {
    name: "Triad chorale",
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
    // ...
},
```

`voice` crea una voz sencilla (una melodía en corcheas desde el Do
central), y cada método cambia una cosa y devuelve la voz nueva:

```rust
impl RowVoice {
    const fn chain(self, chain: Chain) -> Self {
        RowVoice { chain, ..self }
    }
    const fn chords(self, together: usize, step: usize) -> Self {
        RowVoice { together, step, ..self }
    }
    // ...
}
```

Recibir `self` por valor y devolver una copia cambiada con la **sintaxis
de actualización de struct** (`..self`) permite encadenar las llamadas,
como un builder. Y como son `const fn`, toda la tabla puede ser un
`static`: los estilos se construyen al compilar, sin código ejecutándose
al arrancar.

### Aritmética de formas

Los demos tocan cada estilo cuatro veces, como las secciones de una
canción: **P: la serie**, **R: al revés**, **I: dada vuelta** y **RI: las dos**, y luego P una vez más. Pero las voces de un estilo ya usan
formas distintas: el bajo del Coral de tríadas toca R. ¿Qué toca el bajo en
la sección R? R de R, que es la serie hacia delante otra vez: P.

Las cuatro formas se comportan como dos interruptores, «al revés» y
«hacia atrás», y combinar dos formas cambia cada interruptor una vez por
cada forma que lo tenga encendido:

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
exactamente uno es verdadero. Los matemáticos llaman a este pequeño
sistema el *grupo de Klein de cuatro elementos*: cada forma se deshace a
sí misma (I de I es P), y el orden no importa. Un detalle más: la
melodía de Pads de hexacordios toca I3, tres semitonos *por encima* de la
primera nota de la serie. Al revés, «tres por encima» se convierte en
«tres por debajo», así que en las secciones I y RI ese desplazamiento se
convierte en 12 − 3 = 9.

### Tocar una canción entera como una serie

Como con los otros estilos, un estilo de serie se puede probar de dos
maneras: parado, se escribe en un patrón nuevo propio; mientras la
canción suena, se apodera de toda la canción. `use_row_style` recorre
cada patrón que tiene acordes, una línea de bajo o una melodía, y
escribe allí el estilo, leyendo los patrones por turno como P, R, I y RI
para que la canción no se repita.

Probarlo en la app en marcha encontró dos problemas. **Melodía de la serie** no
cambiaba nada: la primera versión escribía los acordes y el bajo de un
estilo en la canción y conservaba la melodía propia de la canción, así
que un estilo que solo es una melodía no tenía nada que escribir. Y el
**Coral de tríadas** escribía sus acordes en un canal Teclados propio mientras
los acordes de la canción seguían sonando en el suyo: tríadas de Do mayor
debajo de una serie solo chocan. El arreglo sigue una regla: el estilo
se apodera de la armonía. Los canales de acordes y de bajo que el estilo
no usa se callan en ese patrón, y los acordes del estilo van al canal
que estaba tocando los acordes de la canción:

```rust
if chords.is_some() || plays(project, bass) || melody {
    let before = project.pattern(pattern).cloned();
    let quiet: Vec<usize> = [(KEYS, chords), (BASS, bass)]
        .into_iter()
        .filter(|(name, _)| !uses(name))
        .filter_map(|(_, channel)| channel)
        .collect();
    // ... empty the `quiet` channels, write the voices ...
    if project.pattern(pattern) != before.as_ref() {
        written += 1;
        change.harmony += 1;
    }
}
```

Para saber si un patrón cambió de verdad, guarda una copia de antes
(`cloned()` convierte el `Option<&Pattern>` en un `Option<Pattern>`) y
compara al final. `as_ref()` convierte la copia de nuevo en un
`Option<&Pattern>`, así que los dos lados del `!=` tienen el mismo tipo.

## Mantener delante una ventana nueva

Un bug no tenía nada que ver con la música. Al hacer clic en **Serie…**,
la ventana Serie se abría *detrás* del piano roll, donde no se podía ver.

egui mantiene sus ventanas en capas y dibuja en último lugar a la que
«quiere estar encima». Dos ventanas querían eso en el mismo frame: la
ventana Serie nueva, que egui eleva cuando aparece por primera vez, y el
piano roll, porque acababan de hacer clic en su botón **Serie…**, y un
clic eleva la ventana en la que cae. egui ordena las ventanas que
quieren la parte superior con una ordenación *estable*, que conserva su
orden cuando empatan, y el piano roll iba primero. Así que el arreglo
mantiene la ventana nueva delante durante sus primeros frames:

```rust
#[derive(Default)]
struct Front {
    /// The frame the window was last drawn in.
    drawn_in: u64,
    /// The frame it opened in.
    opened_in: u64,
}

impl Front {
    fn keep(&mut self, ctx: &egui::Context, id: egui::Id) {
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

`cumulative_pass_nr` cuenta los frames que ha dibujado egui. Una ventana
que no se dibujó en el frame anterior acaba de abrirse, así que recuerda
ese frame; durante sus tres primeros frames mueve su propia capa arriba
y pide otro frame, así que los tres llegan seguidos en lugar de esperar
a que se mueva el ratón. Para encontrar su capa, la ventana necesita
saber su id, así que ambas ventanas ahora fijan uno con `.id(...)`,
creado con `egui::Id::unique`. (egui 0.37 lo llama *unique* para dejar
claro que nombra una sola cosa en todo el programa, como debe hacerlo el
id de una ventana.)

La ventana Serie también quedó más alta que algunas pantallas. Ahora se
desplaza (`.vscroll(true)`), puede hacerse más alta o más baja pero no
más ancha (`.resizable([false, true])`) y se queda dentro de la ventana
principal (`.constrain_to(area)`).

## Cómo se probó

- **Series**: las cuatro formas de Schoenberg se comprueban nota a nota
  contra las publicadas; cada una de las 48 formas de la serie Lineup
  vuelve a nombrarse con `label_of`; cada línea de la matriz de Berg es
  una forma P y cada columna una forma I; la serie de Webern es un
  palíndromo (R9 es P3); la serie de Berg, la serie Todos los intervalos y
  cincuenta series aleatorias de todos los intervalos tienen todos los
  intervalos; la serie Lineup es combinatoria solo con I3; las series se
  leen desde nombres, números y la forma compacta, y las equivocadas dan
  el mensaje correcto; las series aleatorias son series, y la misma
  semilla da la misma serie; una serie con una nota escrita dos veces no
  se puede leer desde un archivo.
- **Conjuntos**: las pruebas de la tabla de arriba, más clases conocidas
  por nombre, formas normales que dan la vuelta a la octava, parejas Z,
  complementos, clases simétricas, y la forma prima de Rahn para 5-20
  donde el libro de Forte difiere.
- **Serialismo**: una melodía en corcheas, las cuatro formas llenando el
  patrón por turno, tricordos como acordes, las tres series, y notas que
  nunca pasan del final del patrón.
- **Estilos y demos**: cada estilo usa una serie de los preajustes y escribe
  sus voces en cualquier tonalidad; combinar formas; Pads de hexacordios tiene
  las doce notas en cada compás; una canción recibe un estilo de serie, y
  el Coral de tríadas reemplaza los acordes de la canción en su propio canal
  en lugar de tocar por encima de ellos; cada estilo hace un demo con
  cinco secciones que se guarda como formato 6.
- **En la app en marcha**: los menús de tonalidad, los números sobre las
  teclas, las teclas de armonía de una serie, la matriz y la escritura de
  cada forma, las propiedades de la serie de Schoenberg, conjuntos
  encontrados por nombre, usados como tonalidad y para dibujar, el
  conjunto de la selección, los estilos de la Biblioteca tanto parados como
  con la canción sonando, los demos, y las ventanas abriéndose delante,
  también a 1280×800.

## Pruébalo

1. Abre **Archivo > Demostraciones > Dodecafónico > Coral de tríadas** y pulsa Espacio.
   Mira los nombres de las secciones en la lista de reproducción: la misma música,
   luego al revés, de cabeza, y ambas cosas.
2. Abre el piano roll (**F7**) y haz clic en **Serie…**. Elige **Series famosas y de ejemplo > Suite de Schoenberg**. La ventana dice que sus
   tetracordos son 4-12, 4-Z15 y 4-1, y que es combinatoria con I11.
3. Haz clic en **I11** encima de la matriz, pon **Juntas** en 6 y haz
   clic en **Escribir I11 en el patrón actual**: dos hexacordos como acordes.
   Pulsa **Ctrl+Z** para deshacerlo.
4. Marca las tres casillas de **Serialismo**, pon **Octavas** en **Amplio**,
   y escribe P4 de nuevo: puntos de sonido al estilo de Webern.
5. Haz clic en **Conjuntos…**, escribe `octatonic`, y haz clic en **Usar como tonalidad**. Las teclas de armonía ahora tocan acordes apilados desde la
   escala octatónica.
6. Selecciona unas cuantas notas en el piano roll y lee su conjunto bajo
   la barra de herramientas.

## Ejercicios

1. **Formas distintas (lineup-core, pequeño).** Escribe una prueba que
   cuente cuántas de las 48 formas de una serie son distintas (reúnelas
   en un `HashSet<ToneRow>`). Comprueba que la serie de Webern tiene 24
   y la de Schoenberg 48. ¿Puede alguna serie tener menos de 24?
   (Pista: ¿podría una serie ser una de sus propias formas I? Escribe lo
   que eso significa para cada nota, y verás que como mucho dos notas
   podrían encajar.)
2. **Series derivadas (lineup-core, mediano).** El Concierto, Op. 24, de
   Webern usa una serie cuyos cuatro tricordos pertenecen todos a la
   misma clase, 3-3 [014]. Añade `ToneRow::is_derived(size)`, verdadero
   cuando cada segmento de ese tamaño tiene la misma clase de conjuntos,
   y un `random_derived` que construya una por backtracking, como
   `random_all_interval`.
3. **El método de Forte (lineup-core, pequeño).** En seis clases, la
   forma prima de Forte difiere de la de Rahn. Escribe `forte_prime()`,
   que desempata desde el principio del conjunto en lugar de desde el
   final, y muéstrala en los detalles del navegador de conjuntos cuando
   difiera. ¿Por qué el truco del «número más pequeño» no puede hacer
   esto directamente? (Piensa en qué bits mira primero una comparación.)
4. **Multiplicar por 5 (lineup-core, mediano).** Multiplicar cada clase
   de alturas por 5 (mod 12) convierte la escala cromática en el círculo
   de cuartas: 0 5 10 3 8 1 … Esta operación **M5** es una quinta manera
   de transformar una serie o un conjunto, usada por algunos
   compositores posteriores a Webern. Añade `PitchSet::multiply` y un
   `ToneRow::multiply`, y averigua qué clases de conjuntos convierte M5
   en una clase distinta. (Pista: las parejas Z-relacionadas merecen un
   vistazo.)
5. **Una serie propia (lineup-core y lineup-app, mediano).** En
   `write_row`, las duraciones siempre leen los números de la propia
   serie. Deja que el usuario teclee una segunda serie, usada solo para
   las duraciones, y muestra ambas series en la ventana. `RowWriting`
   deriva `Copy`; ¿por qué sigue compilando una vez que guarda un
   segundo `ToneRow`, y por qué no lo haría si la segunda serie fuera un
   `Vec<u8>`?
