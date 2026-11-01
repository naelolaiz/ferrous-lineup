# Conjuntos de clases de altura, y una armonía con cuatro nombres

Archivos: [`crates/lineup-core/src/harmony/set_class.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/harmony/set_class.rs),
[`crates/lineup-core/src/harmony/pitch_set.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/harmony/pitch_set.rs),
[`crates/lineup-core/src/chord_sheet/notation.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/chord_sheet/notation.rs),
[`crates/lineup-app/src/twelve_tone.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/twelve_tone.rs)
y [`crates/lineup-app/src/chord_sheet/notation.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/chord_sheet/notation.rs)

Los nombres de acordes sirven para los acordes de la música tonal.
Mucha música del siglo XX usa grupos de notas que no tienen nombre de
acorde, como Do, Do♯ y Fa♯. La **teoría de conjuntos de clases de
altura**, desarrollada por Milton Babbitt, Allen Forte, John Rahn y
otros, da un nombre a cualquier grupo de notas. En Ferrous Lineup un
conjunto es una manera más de *escribir* una armonía, junto a los
cifrados, los números romanos y los micromodos: pasar a conjuntos nunca
mueve una nota. Este capítulo trata de la teoría, de las pocas líneas
de aritmética de bits que la calculan, y de los lugares donde la usa la
aplicación: el explorador de conjuntos, una tonalidad mostrada como
conjunto, y las cuatro notaciones de la hoja de acordes.

## La música, en pocas palabras

**Las clases de altura como números.** Do es 0, Do♯ es 1, y así hasta
Si, 11. Las octavas y la escritura no cuentan: todo Do es 0, y Do♯ y
Re♭ son los dos 1. Un **conjunto** es una elección de algunas de las
doce; hay 2¹² = 4096, desde el conjunto vacío hasta las doce.

**Mover un conjunto.** *Transportar* (T*n*) sube cada nota *n*
semitonos: Do Mi Sol subido 3 es Mi♭ Sol Si♭. *Invertir* (I) da vuelta
cada intervalo: Do Mi Sol (sube 4, luego sube 3) se vuelve Do La♭ Fa
(baja 4, luego baja 3), que es Fa menor. Dada vuelta, una tríada mayor
es una tríada menor.

**Clases de conjuntos.** Un conjunto junto con todas sus
transposiciones e inversiones es una **clase de conjuntos**. Las 24
tríadas mayores y menores son una sola clase. Los 4096 conjuntos se
reparten en **224 clases**.

La **forma normal** es la manera más compacta de escribir un conjunto.
Escrito alrededor de un círculo, un conjunto puede empezar en cualquiera
de sus notas; se empieza en cada una por turno y se mide la distancia
de la primera nota a la última. Tomemos Do mayor con séptima mayor, Do
Mi Sol Si:

| empezando en | notas | distancia | hasta la penúltima |
|-------------|-------|------|----------------|
| Do | Do Mi Sol Si | 11 | |
| Mi | Mi Sol Si Do | 8 | 7 |
| Sol | Sol Si Do Mi | 9 | |
| Si | Si Do Mi Sol | 8 | 5 |

Mi y Si empatan con una distancia de 8, así que se compara la distancia
hasta la penúltima nota: de Si a Mi hay 5, de Mi a Si hay 7. La forma
normal es **Si Do Mi Sol**. (Este desempate, comparando desde el final,
es el de John Rahn; Allen Forte comparaba desde el principio, y los dos
difieren en seis clases. Ferrous Lineup usa el de Rahn, como hoy la
mayoría de los libros.)

La **forma prima** va un paso más allá: se toman las formas normales del
conjunto *y de su inversión*, se mueven las dos para que empiecen en 0,
y se queda la más compacta. Para Cmaj7 es **[0158]**: Si Do Mi Sol
bajado para empezar en 0 es 0 1 5 8. Todo acorde de séptima mayor, sobre
cualquier fundamental, tiene la forma prima [0158].

Los **números de Forte** ordenan las clases por tamaño y después según
la lista de Forte: [0158] es **4-20**, la vigésima clase de cuatro
notas. Las tríadas son 3-11, el acorde de séptima disminuida 4-28, la
escala mayor 7-35, la escala de tonos enteros 6-35. Los libros sobre la
música del siglo XX usan estos números en todas partes.

**El vector interválico.** Cada par de notas de un conjunto forma un
intervalo. Contarlos por tamaño da seis números: cuántas segundas
menores, segundas mayores, terceras menores, terceras mayores, cuartas
y tritonos. Una quinta cuenta como cuarta (es una cuarta dada vuelta) y
una séptima mayor como segunda menor, así que seis tamaños bastan. El
vector es un buen resumen de cómo *suena* un conjunto. Para Do Mi Sol
Si:

```text
Do-Mi tercera mayor   Do-Sol cuarta   Do-Si segunda menor
Mi-Sol tercera menor  Mi-Si cuarta    Sol-Si tercera mayor
<1 0 1 2 2 0>, escrito <101220>
```

De ahí salen dos curiosidades:

- Las clases en **relación Z** tienen el mismo vector interválico sin
  ser la misma clase. 4-Z15 [0146] y 4-Z29 [0137] tienen cada una un
  intervalo de cada tamaño, `<111111>` (son los dos **tetracordios de
  todos los intervalos**), y sin embargo ninguna se puede mover sobre la
  otra. La **Z** de sus nombres las señala.
- **Complementos**: las notas que deja afuera un conjunto forman otro
  conjunto. Las teclas blancas (7-35) dejan afuera las negras, la escala
  pentatónica (5-35). Un conjunto de seis notas y su complemento tienen
  siempre el mismo vector interválico.

## La teoría de conjuntos en doce bits

Un `PitchSet` son doce bits de un `u16`, del bit 0 para Do al bit 11
para Si, y transportar es rotar los bits (ver [Clases de altura, notas
y su
escritura](pitch.md#un-conjunto-de-clases-de-altura-en-doce-bits)).
El resto de la teoría de conjuntos encaja igual de bien en esos bits.

**El complemento es `!`.** El «no» invierte cada bit, y `from_bits` se
queda con los doce que significan notas:

```rust
pub fn complement(self) -> PitchSet {
    PitchSet::from_bits(!self.bits())
}
```

**Invertir** convierte cada clase de altura *p* en 12 − *p*, lo que
refleja el conjunto alrededor de Do:

```rust
pub fn invert(self) -> PitchSet {
    self.iter()
        .map(|pitch_class| PitchClass::from_semitone(12 - pitch_class.semitone()))
        .collect()
}
```

**La forma más compacta es el número más pequeño.** Esta es la parte
ingeniosa. Se baja un conjunto para que su primera nota sea Do, el bit
0. Su última nota es ahora su bit más alto, así que una distancia menor
significa un bit más alto menor, y eso significa un número menor. En un
empate, el siguiente bit hacia abajo es la penúltima nota, que es
exactamente el desempate de Rahn. Comparar dos números *es* la regla de
Rahn:

| empezando en | bajado a Do | bits | número |
|-------------|-----------|------|--------|
| Do | 0 4 7 11 | `1000_1001_0001` | 2193 |
| Mi | 0 3 7 8 | `0001_1000_1001` | 393 |
| Sol | 0 4 5 9 | `0010_0011_0001` | 561 |
| Si | 0 1 5 8 | `0001_0010_0011` | 291 |

Así que la forma normal es «el comienzo que da el número más pequeño»,
y la forma prima «el número más pequeño de las 24 transposiciones e
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

`min_by_key` recorre un iterador y se queda con el elemento de clave
más pequeña. En `prime_form`, `flat_map` convierte cada `n` en un array
de dos conjuntos, así que el iterador da 24 conjuntos, y el más pequeño
es la forma prima. Sin ordenar nada y sin listas de intervalos: unas
pocas comparaciones de enteros. `relative_to(start)` baja el conjunto
hasta que `start` es Do.

Cómo se mueve un conjunto sobre otro, como T*n* o T*n*I, es una búsqueda
entre los mismos 24 movimientos:

```rust
pub fn moved_to(self, other: PitchSet) -> Option<(u8, bool)> {
    let inverted = self.invert();
    (0..12)
        .find(|&n| self.transpose(n) == other)
        .map(|n| (n, false))
        .or_else(|| {
            (0..12)
                .find(|&n| inverted.transpose(n) == other)
                .map(|n| (n, true))
        })
}
```

`or_else` ejecuta su closure solo cuando la primera búsqueda no
encontró nada, así que siempre se prefiere una transposición a una
inversión. Una clase es **simétrica** cuando invertir su forma prima da
una transposición de ella, como pasa con la tríada disminuida: dada
vuelta, es el mismo conjunto movido.

El vector interválico es el único lugar que cuenta pares de la manera
sencilla, un bucle dentro de otro. Con doce notas como mucho, son 66
pares como mucho:

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

`interval.min(12 - interval)` junta un intervalo con su inversión: 7,
una quinta, se vuelve 5, una cuarta.

## 224 clases en una tabla

La forma prima se puede calcular, pero el *número* de Forte no: es solo
un lugar en la lista de Forte. Así que `set_class.rs` guarda la lista,
un array constante de 224 entradas:

```rust
const SET_CLASSES: [SetClass; 224] = [
    SetClass::new(0, 1, false, 0),
    SetClass::new(1, 1, false, 0b000000000001),
    SetClass::new(2, 1, false, 0b000000000011),
    // ...
    SetClass::new(4, 20, false, 0b000100100011),
    // ...
    SetClass::new(12, 1, false, 0b111111111111),
];
```

`SetClass::new` y `PitchSet::from_bits` son `const fn`, así que la
tabla entera se construye al compilar y se guarda con el programa;
buscar un conjunto no cuesta nada al arrancar. El `false` o `true` dice
si la clase lleva una Z en su nombre. `0b` escribe un número en binario,
así que la forma prima [0158] se puede leer en los bits.

Una tabla de 224 líneas de números es fácil de equivocar, así que las
pruebas la comprueban contra la teoría en lugar de confiar en ella:

- cada entrada es su propia forma prima, y las entradas están en orden
  de tamaño y de número;
- hay 1, 1, 6, 12, 29, 38, 50, 38, 29, 12, 6, 1 y 1 clases de 0 a 12
  notas, las cantidades que da cualquier libro;
- se buscan **los 4096 conjuntos**, y cada uno cae en una clase de su
  propio tamaño;
- una clase lleva Z exactamente cuando otra clase de su tamaño tiene el
  mismo vector interválico;
- para 5-20, donde Forte y Rahn ordenan las notas de otra manera, la
  forma prima es la de Rahn.

Gracias a la tercera prueba, la búsqueda puede decir lo que espera en
lugar de ocuparse de un «no encontrado» que no puede ocurrir:

```rust
pub fn of(set: PitchSet) -> &'static SetClass {
    let prime = set.prime_form();
    SET_CLASSES
        .iter()
        .find(|class| class.prime == prime)
        .expect("every prime form is in the table")
}
```

`expect` provoca un pánico con ese mensaje si el `Option` es `None`.
Usarlo es una promesa, y la prueba es lo que la mantiene. La función
devuelve `&'static SetClass`: un préstamo de una entrada de la tabla
constante, que vive tanto como el programa, así que quien la llama
puede guardarla todo el tiempo que quiera.

La clase escribe sus propios nombres. `vector_text` escribe las seis
cantidades como dígitos, `<001110>`, como lo hacen los libros, hasta que
una llega a 10: las doce notas tienen doce de cada intervalo menos del
tritono, y eso se escribe con espacios, `<12 12 12 12 12 6>`, para que
no se pueda leer mal. Algunas clases tienen además un nombre conocido,
de una lista corta de ejemplos: «tríada mayor o menor», «escala de
tonos enteros», «escala octatónica (disminuida)», «acorde de
Petrushka».

## El explorador de conjuntos

**Conjuntos…**, en la barra de herramientas del piano roll, abre la
ventana **Conjuntos de clases de alturas**: todas las clases del tamaño
elegido (de 2 a 10 notas), cada una con su número de Forte, su forma
prima, su vector interválico y su nombre conocido, como «3-11 [037]
<001110> tríada mayor o menor». El cuadro de búsqueda encuentra una
clase por número o por nombre («4-Z15», «tríada», «octatónica»), y con
dos o más notas seleccionadas en el piano roll, **Mostrarlo** salta a su
clase. Al elegir una clase se muestra su pareja Z si la tiene, la clase
de su complemento, y si es simétrica. **Transportarlo** e **Invertido
(I)** eligen un miembro de la clase: T3 es el conjunto tres semitonos
más arriba, T3I su inversión subida tres. Después:

- **▶ Escucharlo** toca el conjunto con el instrumento seleccionado;
- **Dibujar con él** convierte el conjunto en el sello de acordes del
  piano roll, así que cada clic pone el conjunto entero sobre la nota
  pulsada (`ChordStamp::Set`, junto a los sellos de [Acordes:
  construirlos, nombrarlos e
  invertirlos](chords.md#sellos-de-acorde));
- **Usar como tonalidad** convierte el conjunto en la tonalidad de la
  canción: el piano roll resalta sus notas y las teclas de armonía
  muestran sus formas por grado, como para un micromodo (ver
  [Micromodos](micromodes.md#un-micromodo-como-tonalidad-grados)), o,
  con **Acordes**, acordes apilados con ellas.
  Elegido aquí, un conjunto de tres notas se nombra como conjunto, 3-3,
  no como el micromodo que también es (ver [Tres notas: un conjunto o un
  micromodo](#tres-notas-un-conjunto-o-un-micromodo)).

Un conjunto de tres notas es además uno de los micromodos de Kröpfl, y
la ventana enlaza con él (ver [Micromodos](micromodes.md)).

El piano roll nombra conjuntos también sin la ventana. Debajo de la
barra de herramientas, junto al acorde del paso que está bajo el
puntero, muestra el conjunto de ese paso cuando suenan tres notas o
más, y el de las notas seleccionadas cuando hay dos o más:

```text
Conjunto: 3-11 · forma normal Sol♯ Si Re♯ · forma prima [037] · intervalos <001110> · tríada mayor o menor
```

## Una tonalidad como conjunto: las mismas notas, otro nombre

Elegir **conjunto de clases de alturas** en un menú Tonalidad podría
significar dos cosas: tocar en otra escala, o nombrar de otra manera las
mismas notas. Ferrous Lineup toma el segundo sentido, porque un conjunto
es una notación. Do mayor ya *es* un conjunto de clases de altura, Do
Re Mi Fa Sol La Si, la clase 7-35. Así que pasar la tonalidad de Do
mayor a «conjunto de clases de alturas» deja quietas todas las notas de
la canción y solo muestra la misma armonía en notación de conjuntos.
Las teclas de armonía tocan los mismos acordes que antes, nombrados como
conjuntos (la última columna es como los llamaba Do mayor; las teclas
ya no lo hacen):

| Tecla de armonía | Notas | Clase | En Do mayor |
|---|---|---|---|
| 3-11 T7I | Do Mi Sol | 3-11 | C |
| 3-11 T2 | Re Fa La | 3-11 | Dm |
| 3-11 T4 | Mi Sol Si | 3-11 | Em |
| 3-11 T0I | Fa La Do | 3-11 | F |
| 3-11 T2I | Sol Si Re | 3-11 | G |
| 3-11 T9 | La Do Mi | 3-11 | Am |
| 3-10 T11 | Si Re Fa | 3-10 | Bdim |

La tabla es una lección en sí misma. Las tríadas mayores y menores son
la **misma** clase, 3-11, porque una tríada menor es una tríada mayor
dada vuelta, y una clase cuenta un conjunto y su inversión como uno
solo. La tríada disminuida es otra clase, 3-10.

Cambiar de escala es distinto: de mayor a menor, Mi, La y Si pasan a
Mi♭, La♭ y Si♭, y la canción se mueve (ver [Escalas, modos y
tonalidades](scales.md#cambiar-de-tonalidad-mover-la-canción)).
Un conjunto que eliges en el explorador con **Usar como tonalidad**
puede ser cualquier conjunto.

`set_key`, en la aplicación, elige el conjunto al que pasa el menú:

```rust
pub(crate) fn set_key(ctx: &egui::Context, scale: Scale, root: PitchClass) -> Scale {
    let up = scale.root.semitones_up_to(root);
    if let Some(group) = scale
        .tone_row()
        .and_then(|row| row.segments(3).first().copied())
    {
        return Scale::set(root, group.transpose(up)).named_as_set();
    }
    if let ScaleKind::Set(_) = scale.kind {
        return Scale::set(root, scale.pitch_set().transpose(up)).named_as_set();
    }
    if scale.kind.is_tonal() && scale.kind != ScaleKind::Chromatic {
        let set = Scale::set(root, scale.pitch_set().transpose(up));
        // Gb major as a set still writes its notes Gb Ab Bb Cb...
        return if up == 0 {
            set.with_spelling(scale.spelling())
        } else {
            set
        };
    }
    let set = remembered_set(ctx);
    let start = set.normal_form().first().copied().unwrap_or(root);
    Scale::set(root, set.transpose(start.semitones_up_to(root))).named_as_set()
}
```

Desde una tonalidad es la escala de la propia tonalidad; desde una
serie dodecafónica, las tres primeras notas de la serie, que son lo que
tocaban las teclas de armonía; desde un micromodo, sus propias tres
notas; desde la escala cromática, que tiene todas las notas y no dice
nada de la armonía, el último conjunto usado. Un conjunto cuyas notas
son exactamente una escala sobre su fundamental se escribe como esa
escala, y sigue como esa tonalidad cuando la tonalidad vuelve a cambiar
(`set_as_scale` y `as_key`, en [Escalas, modos y
tonalidades](scales.md)).

### Tres notas: un conjunto o un micromodo

Todo grupo de tres notas es uno de los doce micromodos de Kröpfl (ver
[Micromodos](micromodes.md)), así que tres notas solas no pueden decir
si la tonalidad es el conjunto 3-3 o el micromodo Menor 3: los dos son
Do Do♯ Mi. Lo dice la tonalidad, con un campo de `Scale` junto al que
escribe Sol♭ mayor al revés:

```rust
/// Whether a key of three notes is named as a pitch-class set, "set
/// 3-3 on C", rather than as the micromode it also is, "Menor 3 on
/// C" (see [`Scale::micromode`]).
#[serde(default, skip_serializing_if = "std::ops::Not::not")]
as_set: bool,
```

`Scale::micromode` da el micromodo con que se nombra una tonalidad, o
`None`, y todo lo que nombra una tonalidad o sus formas se lo pregunta:
el nombre de la tonalidad en los números de compás, las teclas de
armonía, las formas que escribe la hoja de acordes
(`HarmonyName::form_in`) y la revisión del color. `named_as_set` y
`named_as_micromode` dan la misma tonalidad nombrada de la otra
manera, así que elegir **conjunto de clases de alturas** en un menú
Tonalidad mientras la tonalidad es Menor 3 nombra sus tres notas 3-3, y
elegir **Menor 3** las vuelve a nombrar. Ninguno de los dos mueve una
nota. Un cambio de tonalidad nombrado como la tonalidad de la canción
la sigue (`key_map::moved_key`), y uno que pasa a otra fundamental
conserva su nombre (`named_like`).

`serde` escribe el campo solo cuando es `true`, así que una tonalidad
nombrada como su micromodo escribe lo de siempre, y una versión
anterior de Ferrous Lineup lee `as_set:true` como algo que no conoce y
nombra la tonalidad como su micromodo, con las mismas notas.

Qué opción muestra elegida un menú Tonalidad es una función,
`KindChoice::of`, y lo que hace una opción es otra, `KindChoice::key`,
que usan los dos menús Tonalidad y el menú de cambio de tonalidad de
los números de compás. Una prueba elige cada opción desde cada clase de
tonalidad (una tonalidad mayor, una escrita al revés, la escala
cromática, una serie, un micromodo, un conjunto, las mismas notas
nombradas de las dos maneras) y comprueba que la opción elegida es la
que se muestra: el error que escondía **conjunto de clases de alturas**
detrás de Menor 3 no puede volver sin que se note.

### Cualquier conjunto, por su tamaño

Debajo de **conjunto de clases de alturas**, un menú Tonalidad lista las
clases de conjuntos de un tamaño como lista los micromodos debajo: una
fila de tamaños de 2 a 10 notas, un interruptor **Invertido (I)** y un
botón por clase con su nombre de Forte. Una clase todavía no es un
conjunto: no tiene lugar. Elegir una la pone sobre la tónica:

```rust
/// The class's prime form, or its inversion, moved so that the first
/// note of its normal form goes to `root`: the set a key of this
/// class on `root` plays.
pub fn on(&self, root: PitchClass, inverted: bool) -> PitchSet {
    let form = if inverted {
        self.prime.invert()
    } else {
        self.prime
    };
    let first = form
        .normal_form()
        .first()
        .copied()
        .unwrap_or(PitchClass::C);
    form.transpose(first.semitones_up_to(root))
}
```

3-11 sobre Do es Do Mi♭ Sol, la tríada menor, e invertido Do Mi Sol, la
mayor. Una clase que es igual al revés, como 4-28, da el mismo conjunto
de las dos maneras. `is_inverted` responde al revés, si un conjunto de
la clase es su inversión movida, y para una clase así dice que no, de
modo que el menú marca la clase de la tonalidad esté como esté el
interruptor. Una prueba pone cada clase sobre cada nota, de las dos
maneras, y comprueba que el conjunto tiene esa nota y sigue en su
clase.

La elección es `KindChoice::Class { class, inverted }`. Termina en
`set_choice`, con las otras maneras de elegir un conjunto: si la
tonalidad ya es ese conjunto sobre esa nota, no pasa nada, y si es el
micromodo con esas notas, solo se nombra como conjunto.

El tamaño y el interruptor viven en la memoria de egui mientras el menú
está abierto, y empiezan desde la tonalidad cada vez que se abre: Do
mayor se abre en los conjuntos de siete notas. Un clic en un tamaño no
debe cerrar el menú, así que el `ComboBox` se cierra solo con un clic
fuera de él (`PopupCloseBehavior::CloseOnClickOutside`), y una elección
lo cierra con `ui.close()`.

**El conjunto de la propia canción.** Entre los conjuntos de un tamaño,
el que la canción usa más aparece recuadrado. `notation::song_set` lee
las notas de las dos maneras que describe
[Leer las propias notas](#leer-las-propias-notas): todas las notas que
suenan juntas, y la línea de cada canal por separado, porque una
melodía o un bajo es una sola línea, y juntar dos líneas en una
cortaría grupos entre ellas. Solo cuentan los grupos de exactamente ese
número de notas, porque un acorde de tres no dice nada de los conjuntos
de cuatro. Cada grupo dura hasta que empieza el siguiente, y dos de una
misma clase a la vez cuentan una vez, como en
[la clase más usada](#la-clase-más-usada). Gana la lectura en la que una
clase suena más tiempo.

```rust
pub struct SongSet {
    pub class: &'static SetClass,
    /// The class's set the song uses most, where it is.
    pub set: PitchSet,
    pub steps: usize,
    pub length: usize,
    /// How the notes were read: together, or in a line.
    pub grouping: Grouping,
    /// The channel whose line it was read in, for a line.
    pub channel: Option<usize>,
}
```

Además de la clase, guarda el conjunto de la clase que la canción usa
más, donde está. En Los tricordios de Webern, de a tres, es 3-3 y Si Do
Mi♭, el primer tricordio de la serie; de a seis es 6-20, la primera
mitad de la serie, leída en la línea de la Melodía. Elegir el botón
recuadrado es `KindChoice::Found(set)`, que toma ese conjunto tal como
está, sobre la tónica si el conjunto la tiene y si no sobre su propia
primera nota, así las notas de la canción siguen encajando en la
tonalidad. La lectura se hace cuando el menú se abre o cambia el
tamaño, no en cada cuadro, y se guarda en la memoria de egui con el
tamaño para el que se leyó.

## Un conjunto como tonalidad: sus formas

La música construida sobre un solo conjunto, como la de Webern y buena
parte de la de Kröpfl, hace su armonía con las **formas** del propio
conjunto: el conjunto transportado, o dado vuelta y transportado. Una
tonalidad que es un micromodo muestra sus formas en las teclas de
armonía y las nombra por su grado (ver [Un micromodo como tonalidad:
grados](micromodes.md#un-micromodo-como-tonalidad-grados)). Cualquier
conjunto se puede leer igual, así que una tonalidad que es un conjunto
de 2 a 11 notas tiene un selector **Acordes**/**Formas**: acordes
apilados con sus notas, como en la tabla de arriba, o sus formas. Un
conjunto de tres notas ya es un acorde, así que siempre muestra sus
formas.

Elegir un conjunto nuevo como tonalidad, en el menú Tonalidad o con
**Usar como tonalidad**, pasa el selector a **Formas**, porque las
formas del propio conjunto son de lo que está hecha su música. Nombrar
las mismas notas como conjunto, como cuando Do mayor pasa a ser 7-35,
lo pasa a **Acordes**, los acordes que la canción tenía antes. Toda
manera de cambiar la tonalidad termina en un mismo lugar,
`RollAction::SetScale` o `HarmonyAction::SetScale` en `app.rs`, así que
una sola llamada ahí las cubre todas:

```rust
pub fn key_picked(&mut self, old: Scale, new: Scale) {
    if !matches!(new.kind, ScaleKind::Set(_)) {
        return;
    }
    if new.pitch_set() != old.pitch_set() {
        self.forms = true;
    } else if !matches!(old.kind, ScaleKind::Set(_)) {
        self.forms = false;
    }
}
```

Una serie o un conjunto es música sin tonalidad, así que sus teclas
nunca toman los nombres de la música tonal. Un acorde apilado con las
notas de un conjunto se llama por el conjunto que forma, como lo
escribe la hoja de acordes: Do Mi Sol en una tonalidad conjunto es
**3-11 T7I**, no C, con su forma prima, [037], debajo. Un grupo de una
serie se llama por su lugar en la serie, «1–3», con su clase debajo,
incluso en la serie *Four triads*, cuyos grupos suenan como tríadas. La
regla es del dueño del proyecto: los nombres tonales y los de conjuntos
no se mezclan salvo que se pida. `GroupKey::new` nombra el grupo con
`HarmonyName::Set(set).to_string()`, el mismo texto que escribe la hoja
de acordes, así que las teclas y la hoja coinciden.

Los grados se cuentan como en un micromodo: **T*n*** es la forma propia
de la tonalidad subida *n* semitonos, **T*n*I** la forma de la
tonalidad dada vuelta alrededor de la nota de la tonalidad y después
subida *n*. Pasar la tonalidad a otro conjunto conserva el grado de
cada forma, en la hoja y en las notas, como lo hace cambiar el modo de
una tonalidad (ver [Otro color, los mismos
grados](micromodes.md#otro-color-los-mismos-grados)).

Con un conjunto como tonalidad, la casilla **Conducción de voces** del
piano roll revisa el **color** del conjunto: lee qué forma suena dónde,
muestra las formas por su grado en una línea Formas y marca las notas
fuera de la forma que se oye a su alrededor. Un micromodo además tiene
que sonar entero, así que también se marca una forma suya sin terminar
(ver [Revisar el color](micromodes.md#revisar-el-color)).

### Qué tan cerca: las notas que conserva una forma

Las formas van de casa a la más lejana según las notas que cada una
conserva de casa. Para Do mayor como conjunto es una vieja conocida:

| Forma | Escala | Notas que conserva de Do mayor |
|---|---|---|
| T0 | Do mayor | 7 |
| T5, T7 | Fa mayor, Sol mayor | 6 |
| T10, T2 | Si♭ mayor, Re mayor | 5 |
| T3, T9 | Mi♭ mayor, La mayor | 4 |
| T8, T4 | La♭ mayor, Mi mayor | 3 |
| T1, T11, T6 | Re♭ mayor, Si mayor, Fa♯ mayor | 2 |

Cada paso en el círculo de quintas cambia una nota, así que las notas
comunes cuentan las quintas entre dos tonalidades: el círculo de
quintas es la cercanía de las formas de 7-35. **Cercanas** muestra las
formas que conservan al menos la mitad de las notas de casa, redondeada
hacia abajo (tres de siete; para tres notas, al menos una), aquí las
nueve tonalidades hasta cuatro quintas de distancia; **Todas las
formas** muestra las doce.

Otros conjuntos dan mapas menos conocidos. Do Mi Sol Si♭ como conjunto,
4-27, las notas de una séptima de dominante, tiene 24 formas: las doce
séptimas de dominante y, dadas vuelta, las doce séptimas
semidisminuidas. Dado vuelta alrededor de Do, Do Mi Sol Si♭ se vuelve
Do La♭ Fa Re, Dm7b5, y un tono más arriba eso es Mi Sol Si♭ Re, Em7b5:
**T2I**, la única forma que conserva tres notas de casa. Otras nueve
conservan dos (Eb7, F#7, A7 y seis séptimas semidisminuidas), siete
conservan una y seis ninguna.

En la aplicación, una tecla nunca llama a una forma de un conjunto por
el acorde o la escala a la que suena. Su tecla muestra el grado, las
notas en forma normal y, debajo, el conjunto que es toda forma de la
tonalidad, 4-27. El nombre propio de la forma como lo escribe la hoja
de acordes, siendo T2I de Do Mi Sol Si♭ Re Mi Sol Si♭, **4-27 T2**, está
en la ayuda de la tecla, no en la tecla: el T*n* de la hoja cuenta desde
la forma prima de la clase sobre Do y el grado desde la tonalidad, así
que en una tonalidad que no es la forma prima sobre Do los dos no
coinciden. *Los tricordios de Webern* con su 6-20 como tonalidad lo
mostró: T0, la forma propia de la tonalidad, es 6-20 T3. `form_footer`
da el conjunto (o el micromodo, o la tríada de un micromodo acorde);
`describe_form` agrega «En la hoja de acordes: 6-20 T3.» a la ayuda
detallada.

Las teclas de una fila son tan anchas como lo piden sus notas más
anchas: `keys_width` mide las notas de cada tecla al tamaño de letra de
la tecla y da a todas las teclas de la fila la más ancha, como mínimo
los 76 puntos de siempre, así que seis notas en solfeo, «Mib Mi Sol Lab
Si Do», se escriben enteras en lugar de meterse en la tecla de al lado.
Una fila demasiado larga para la ventana pasa a una segunda línea. Una forma
de más de cinco notas que es la escala de la tonalidad transportada se
sigue escribiendo como esa escala (7-35 subido una quinta desde Do mayor
se escribe Sol La Si Do Re Mi Fa♯, con Fa♯, no Sol♭), pero no se llama
Sol mayor. Solo los tres micromodos acorde de Kröpfl (Acorde
disminuido, menor-mayor y aumentado), que son tríadas por su nombre,
muestran la tríada que es cada forma, deletreada desde su fundamental
(ver [Un micromodo como tonalidad:
grados](micromodes.md#un-micromodo-como-tonalidad-grados)); las mismas
tres notas nombradas como conjunto, 3-11, no.

### Progresiones de formas

En una tonalidad que es un conjunto, el botón **Progresión** de la hoja
de acordes no ofrece las progresiones de la Biblioteca, que son acordes
en una tonalidad mayor o menor. Ofrece maneras de encadenar las formas
del conjunto, los recorridos de `micromodal.rs` (ver [Música con un
solo micromodo](micromodes.md#música-con-un-solo-micromodo)), ahora para
cualquier conjunto, y escribe ocho compases de ellas:

- **Conservar todas las notas menos una**: cada forma conserva todas
  las notas de la anterior menos una. Do mayor como conjunto recorre el
  círculo de quintas hacia los bemoles: Fa mayor, Si♭ mayor, Mi♭
  mayor… Para tres notas es el recorrido de los micromodos, **Conservar
  dos notas, mover una**.
- **Conservar todas las notas que se pueda**, que aparece en su lugar
  cuando las formas del conjunto conservan todas las notas menos una
  solo de a pares. 4-27 es un conjunto así: C7 y Em7b5 conservan tres
  notas la una de la otra y de ninguna otra forma, así que un recorrido
  que conservara tres se mecería entre ellas para siempre. Como nunca
  vuelve enseguida a la forma anterior, sigue conservando dos: C7,
  Em7b5, A7, C#m7b5, C#7, Fm7b5, Ab7, Cm7b5. Las séptimas de dominante
  y las semidisminuidas se turnan, y Em7b5 a A7 es el ii∅–V7 de Re
  menor.
- **Conservar una nota, mover dos**, para tres notas: un pivote sobre
  una nota.
- **Las doce notas, cada *N* formas**: ver más abajo.
- **Secuencias**: todas las notas suben un semitono, un tono, una
  tercera menor, una cuarta o una quinta en cada compás.

Cada compás recibe su forma como conjunto (o como micromodo), con su
grado y las notas que conserva debajo, y la banda toca las formas como
acordes cuyas notas comunes se quedan en las mismas teclas
(`micromodal::voice`, la disposición de un estilo de micromodo), sobre
la fundamental cuando la forma es un acorde y si no sobre la primera
nota de su forma normal.

### Las doce notas: agregados

La música dodecafónica cuenta en **agregados**: tramos que tienen las
doce notas. Un conjunto de tres, cuatro o seis notas puede hacer uno con
sus propias formas cuando algunas de sus formas no comparten ninguna
nota y entre todas cubren las doce. Menor 3 lo hace de 18 maneras, con
cuatro formas cada vez; la serie del Concierto de Webern, Op. 24, es
cuatro formas de él, una tras otra. Un hexacordo cuyo complemento, las
seis notas que deja fuera, es una forma del mismo hexacordo es
**combinatorio**, la palabra de Milton Babbitt para lo que las series
de Schoenberg hacen con sus mitades: los seis semitonos de Do a Fa y
los seis de Fa♯ a Si son un par así. Veinte de las cincuenta clases de
hexacordos lo son; las otras treinta, las relacionadas en Z, dejan fuera
seis notas de otra clase. Peter Schat construyó su *Reloj tonal* (*Tone
Clock*) sobre grupos así de tricordios, y llamó a su manera de
hacerlos, un tricordio movido por los intervalos de otro, **dirigir**
(*steering*).

| Conjunto | Maneras de repartir las doce notas |
|---|---|
| Menor 3 (Do Do♯ Mi), y las tríadas mayores y menores | 18 cada uno |
| la tríada aumentada | 1: las cuatro tríadas aumentadas |
| la tríada disminuida | ninguna: sus formas siempre se superponen |
| cuatro semitonos (Do Do♯ Re Mi♭) | 4 |
| el acorde de séptima disminuida | 1: las tres séptimas disminuidas |
| 4-27, la séptima de dominante | ninguna |
| seis semitonos (de Do a Fa) | 6: cada uno con los seis a un tritono |
| la escala de tonos enteros | 1: las dos escalas de tonos enteros |

**Las doce notas, cada *N* formas** solo aparece para los conjuntos que
pueden hacerlo. Toca una manera de repartir las doce notas, cada forma
yendo a la más cercana de las que quedan del grupo, y después empieza el
grupo siguiente desde una forma de otra manera que conserve todas las
notas que pueda de la última forma, para que los agregados se unan
suavemente.

### En Rust: un rango en un `match`, y una búsqueda que retrocede

Qué formas son acordes es un `match` sobre el tamaño del conjunto:

```rust
pub fn form_chord(form: PitchSet) -> Option<Chord> {
    match form.len() {
        3 => Micromode::of(form).and_then(|micromode| micromode.chord(form)),
        4..=5 => match analyze(stacked(&form.normal_form())) {
            Harmony::Chord { chord, .. } if chord.pitch_set() == form => Some(chord),
            _ => None,
        },
        _ => None,
    }
}
```

`4..=5` es un **patrón de rango**: coincide con 4 y con 5. El `if`
después del patrón interno es una **guarda**: la rama se toma solo
cuando el acorde encontrado tiene exactamente las notas de la forma,
así que cuatro notas que solo hacen un acorde dejando una fuera no
reciben el nombre de ese acorde. Tres notas van al micromodo, que solo
llama acorde a una tríada: Do Do♯ Mi es Menor 3, no un C#m(maj7)
incompleto.

La banda la usa para poner en el bajo la fundamental de un acorde. Las
teclas de armonía solo la consultan en una tonalidad que es un
micromodo:

```rust
let triad = scale.micromode().and(form_chord(form));
```

`Option::and` da la segunda opción cuando la primera es `Some`, y
`None` cuando es `None`. `Scale::micromode` es `None` para una
tonalidad nombrada como conjunto, así que sus formas no reciben nombre
de acorde.

Encontrar las maneras de repartir las doce notas es una búsqueda que
prueba una forma, sigue, y saca la forma cuando la manera está completa
o trabada:

```rust
fn split(
    held: u16,
    chosen: &mut Vec<PitchSet>,
    forms: &[PitchSet],
    ways: &mut Vec<Vec<PitchSet>>,
) {
    let Some(lowest) = (0..12).find(|&note| held & (1 << note) == 0) else {
        ways.push(chosen.clone());
        return;
    };
    for &form in forms {
        if form.bits() & held == 0 && form.bits() & (1 << lowest) != 0 {
            chosen.push(form);
            split(held | form.bits(), chosen, forms, ways);
            chosen.pop();
        }
    }
}
```

`held` tiene un bit por cada nota ya tomada. La nota más grave que
todavía no está tomada tiene que estar en la forma siguiente, así que
cada manera se encuentra una sola vez, con sus formas en un solo orden,
y nunca se prueba una forma que comparte una nota con `held`. Cuando
todas las notas están tomadas, `find` da `None` y `let ... else` anota
la manera. Un solo vector `chosen` sirve para toda la búsqueda: `push`
antes de bajar un nivel y `pop` después, así que siempre tiene las
formas de la manera que se está armando. Esto es **vuelta atrás**
(*backtracking*). `split` es una función dentro de una función: solo se
usa ahí, y no puede capturar variables, así que todo lo que necesita se
le pasa.

## Una armonía, cuatro nombres

La hoja de acordes aplica la misma regla a cada armonía. Su fila
superior tiene un menú **Mostrar**: **Según la tonalidad**, **Como está
escrito**, **Acordes**, **Conjuntos de clases de alturas** o
**Micromodos**. Las notas siguen
siendo las mismas, solo cambian sus nombres, y nada de la canción se
toca:

| Escrito | Acordes | Conjuntos | Micromodos |
|---------|--------|------------------|------------|
| C | C | 3-11 T7I | Acorde menor-mayor T7I |
| Dm | Dm | 3-11 T2 | Acorde menor-mayor T2 |
| G | G | 3-11 T2I | Acorde menor-mayor T2I |
| Am | Am | 3-11 T9 | Acorde menor-mayor T9 |
| Bdim | Bdim | 3-10 T11 | Acorde disminuido T11 |
| G7 | G7 | 4-27 T7I | 4-27 T7I |
| 3-11 T2 | Dm | 3-11 T2 | Acorde menor-mayor T2 |
| Menor 3 | Menor 3 | 3-3 | Menor 3 |

Cómo leerla:

- **T*n* y T*n*I.** Cada clase tiene una forma prima que empieza en Do.
  Para 3-11 es [0, 3, 7], Do Mi♭ Sol, una tríada menor. **T9** significa
  «la forma prima subida 9 semitonos»: La Do Mi, La menor. **T*n*I**
  significa «dada vuelta primero, y después subida *n*». Dada vuelta,
  [0, 3, 7] es 0, −3 y −7: Do, La y Fa. Subido 7, eso es Sol, Mi y Do,
  Do mayor. Así que todo acorde mayor lleva una I en su nombre, y ningún
  acorde menor la lleva. Dm (T2) y G (T2I) comparten el número 2 pero no
  las notas. La forma prima misma, T0, se escribe sin T: Menor 3 es Do
  Re♭ Mi, 3-3 sobre Do.
- **Un micromodo tiene siempre tres notas.** Los doce micromodos de
  Kröpfl son las doce clases de tres notas, de 3-1 a 3-12, con los
  nombres en castellano que él les dio. El micromodo 3-11 es el **Acorde
  menor-mayor**, porque la tríada mayor y la menor son una sola clase.
  G7 tiene cuatro notas, así que no es un micromodo, y la vista
  Micromodos escribe su conjunto.
- **Acordes** vuelve a convertir un conjunto en acorde **cuando sus
  notas forman uno**: el conjunto 3-11 T2 es Re Fa La, así que la vista
  escribe Dm, con la fundamental escrita como la escribe la tonalidad.
  Un conjunto que no forma ningún acorde que conozca la aplicación, como
  3-1 o Menor 3, conserva su nombre. **N.C.** (sin acorde) queda como
  N.C. en todas las vistas, y una forma de serie (P0, RI7) queda como forma de
  serie: una serie es una línea de doce notas, no un grupo.

Toda canción se abre **Según la tonalidad**: cada armonía se nombra en
la notación de la tonalidad en que está. En una tonalidad mayor o
menor, o en un modo, son acordes; en un conjunto de clases de alturas o
una serie dodecafónica, conjuntos; en un micromodo, micromodos. La hoja
de una canción en Do mayor muestra C, Am y F, la de una demostración de
micromodos sus formas, Menor 3 T2, y una canción que cambia de
tonalidad a mitad de camino cambia de notación con ella. Otra opción
vale solo para la canción en pantalla: la siguiente vuelve a abrirse
según su tonalidad, así que una canción en Do mayor no se muestra en
micromodos porque la anterior se mostraba así. `Notation::in_key` es la
regla:

```rust
pub fn in_key(self, key: Scale) -> Notation {
    match self {
        Notation::Key if key.micromode().is_some() => Notation::Micromodes,
        Notation::Key if matches!(key.kind, ScaleKind::Set(_) | ScaleKind::Row(_)) => {
            Notation::Sets
        }
        Notation::Key => Notation::Chords,
        other => other,
    }
}
```

Los brazos de un `match` se prueban en orden, y un brazo con una
**guarda** (`if` después del patrón) solo encaja si además la guarda es
verdadera. Una tonalidad que es un micromodo también es un conjunto,
así que el primer brazo tiene que ir antes que el segundo. Toda función
que nombra una armonía (`rename`, `name_group`, `name_heard`) empieza
con `self.in_key(key)`, con la tonalidad del compás en que está la
armonía, así que ninguna tiene que saber de `Notation::Key`.

Un poliacorde son dos grupos de notas, uno sobre otro, y juntarlos en un
solo conjunto perdería lo que lo hace un poliacorde. Así que cada acorde
se nombra por separado, y se escriben el de arriba primero, como se
escribe el poliacorde:

```text
Eb7|Fb   →   4-27 T3I|3-11 T11I
```

### En Rust: un solo `match` decide

`Notation` es un enum de cinco variantes, con `#[default]` en `Key`
para que toda canción empiece según su tonalidad. Renombrar una armonía
primero convierte `Key` en la notación de la tonalidad de la armonía
(`in_key`, más arriba), y después es un solo `match` sobre una
**tupla** de la notación y la armonía. Rust prueba los
brazos de arriba abajo, así que su orden es la regla: primero lo que
nunca cambia, después la vista Acordes, después dos acordes a la vez, y
después todo lo demás.

```rust
pub fn rename(self, harmony: HarmonyName, key: Scale) -> Vec<HarmonyName> {
    let notation = self.in_key(key);
    let group = |set: PitchSet| notation.name_group(set, key);
    match (notation, harmony) {
        (Notation::Written, _)
        | (_, HarmonyName::NoChord)
        | (_, HarmonyName::Row(_))
        | (Notation::Chords, HarmonyName::Chord(_) | HarmonyName::Polychord(..)) => {
            vec![harmony]
        }
        (Notation::Chords, HarmonyName::Set(set) | HarmonyName::Micromode(set)) => {
            match chord_of(set) {
                Some(chord) => vec![HarmonyName::Chord(ChordSymbol::new(
                    key.spell_root(chord),
                    chord.quality,
                ))],
                None => vec![harmony],
            }
        }
        (_, HarmonyName::Polychord(upper, lower)) => {
            // The lower chord with its bass, then the upper chord.
            let lower = notes_of(lower);
            vec![group(lower), group(upper.chord().pitch_set())]
        }
        (_, HarmonyName::Chord(symbol)) => vec![group(notes_of(symbol))],
        (_, HarmonyName::Set(set) | HarmonyName::Micromode(set)) => vec![group(set)],
    }
}
```

El primer brazo usa **patrones «o»** en dos niveles: `A | B | C` entre
tuplas enteras, y `Chord(_) | Polychord(..)` dentro de una. Un patrón
«o» que da un nombre, como `Set(set) | Micromode(set)`, tiene que darlo
en todas las alternativas, con el mismo tipo. La función devuelve un
`Vec` porque un poliacorde se vuelve dos nombres y todo lo demás uno.

`chord_of` encuentra el acorde que forma un conjunto. Recorre las
clases de acorde de la más sencilla, la tríada mayor, a la más
compleja, y para cada una las notas del conjunto, en forma normal, como
fundamentales posibles:

```rust
pub fn chord_of(set: PitchSet) -> Option<Chord> {
    let roots = set.normal_form();
    ChordQuality::ALL.into_iter().find_map(|quality| {
        roots
            .iter()
            .find(|&&root| ChordQuality::identify(set.relative_to(root)) == Some(quality))
            .map(|&root| Chord::new(root, quality))
    })
}
```

`identify` dice qué clase de acorde forman los intervalos sobre una
fundamental, si forman alguna. Como las clases de acorde son el bucle de
fuera, gana la que está primero en la lista, sea cual sea su
fundamental: Do Mi Sol La es a la vez Am7 y C6, y se encuentra como Am7,
porque los acordes de séptima van antes que los de sexta en la lista.
`&&root` en el patrón del closure desarma una referencia a una
referencia: `iter` da `&PitchClass`, y `find` le pasa a su closure una
referencia a eso.

### En el piano roll

El carril de acordes del piano roll, sobre la cuadrícula, también tiene
un menú **Mostrar**, junto a **Conducción de voces**: **Según la
tonalidad**, con el que empieza, **Acordes**, **Conjuntos de clases de
alturas** o **Micromodos**. (No hay Como está
escrito: el carril nombra lo que oye, y ahí no hay nada escrito.)

![El piano roll de la demostración Bruma de tonos enteros con Mostrar en Conjuntos de clases de alturas: la fila Teclados nombra notas sueltas, y la fila Todo nombra 2-2, 3-6, 2-2, 2-2 T2, 3-6 T2 y así, un bloque por pulso; la línea bajo la barra de herramientas dice «Paso 22: Teclados: Re. Todos: Re subiendo a Mi: una segunda mayor. Conjuntos de clases de alturas: 2-2 T2.»](../images/step-9-piano-roll-sets.png)

El carril oye la armonía como un `analysis::Harmony`, no como un
`HarmonyName`, así que tiene su propia entrada a la misma regla:

```rust
pub fn name_heard(
    self,
    harmony: &Harmony,
    sounding: PitchSet,
    key: Scale,
) -> Option<Vec<HarmonyName>> {
    if !self.groups() {
        return None;
    }
    let heard = |harmony: Harmony| {
        let all = harmony.pitch_set();
        let heard = all.intersection(sounding);
        self.name_group(if heard.len() >= 2 { heard } else { all }, key)
    };
    match *harmony {
        Harmony::Silence | Harmony::Single(_) => None,
        Harmony::Polychord { upper, lower, bass } => Some(vec![
            heard(Harmony::Chord { chord: lower, bass }),
            heard(Harmony::Chord {
                chord: upper,
                bass: upper.root,
            }),
        ]),
        other => Some(vec![heard(other)]),
    }
}
```

`None` quiere decir «deja el nombre propio del carril»: en la vista
Acordes, y para el silencio y una sola nota, que no forman conjunto.
Dos notas son un conjunto de dos, una clase de intervalo: Do y Mi son
2-4. Un acorde y su bajo son un solo conjunto, así que C y C/E, dos
bloques como acordes, son un solo bloque como 3-11 T7I.

¿Por qué `sounding`? Una armonía oída es el **nombre** de un acorde, y
un nombre puede tener una nota que no suena. Do, Re y Mi juntos se oyen
como Cadd9, como los oiría un músico: Do mayor con una novena agregada,
sin su quinta. Pero las notas de Cadd9 son Do, Re, Mi y Sol, el
conjunto 4-22, y ningún Sol suena. Un conjunto nombra las notas mismas,
así que las notas del acorde se recortan a las que suenan:
`intersection` es un AND bit a bit de los doce bits de los dos
conjuntos (ver [La teoría de conjuntos en doce
bits](#la-teoría-de-conjuntos-en-doce-bits)). Do Re Mi es 3-6, el
micromodo Mayor 1, que es de lo que está hecha Bruma de tonos enteros.
Si quedaran menos de dos notas, se nombran las del acorde, porque si no
no habría conjunto que nombrar.

El nombre completo de un conjunto es largo para un bloque de un pulso,
así que un bloque demasiado estrecho para él muestra su clase sola,
**3-11** por 3-11 T7I, como hace la hoja de acordes cuando mete más
compases en una línea (`stacked_classes`). Apuntar al bloque lo nombra
entero en la línea bajo la barra de herramientas. Cómo dibuja el carril
sus bloques está en [El carril de acordes: `chunk_by`, y luego
medir](../program/editors.md#el-carril-de-acordes-chunk_by-y-luego-medir).

## Leer las propias notas

Renombrar las armonías de la hoja es la mitad de lo que hace un
analista con los conjuntos. La música sin cifrado, un concierto de
Webern o una página de *La consagración de la primavera*, se corta en
conjuntos **a partir de sus notas**, de dos maneras:

- **verticalmente**, las notas que suenan juntas, como se oye un acorde
  apilado o un cluster;
- **horizontalmente**, las notas una detrás de otra en una línea, como
  se corta una serie dodecafónica en grupos de tres o de seis.

La música construida sobre un conjunto mezcla las dos: una forma de
Menor 3 puede ser dos notas tocadas juntas y una tercera después, o una
melodía cuyas notas sostienen los acordes de debajo. Así que donde la
tonalidad es un conjunto o un micromodo hay una tercera manera, las
formas del conjunto de la tonalidad como las lee la [revisión del
color](micromodes.md#revisar-el-color).

Con **Conjuntos de clases de alturas** o **Micromodos** a la vista, un
segundo menú elige cuál: **Las armonías de la hoja** (renombradas, como
antes), **Notas a la vez (vertical)**, **Notas en línea (serial)** o
**Formas de la tonalidad (mixtas)**. Al
leer notas, la hoja muestra los grupos que encuentra, cada uno donde
empieza, y deja en paz lo escrito: escribir en un compás sigue
escribiendo su armonía, y **Como está escrito** la muestra otra vez.
Dos elecciones más dicen cuántas notas forman un grupo (de 2 a 12, o
**Todas las notas** que suenan a la vez; un micromodo tiene siempre
tres) y de quién son las notas que se leen. **Automático** lee la
melodía para una línea (la pista de la melodía, Lead, o si no la más
aguda,
como la elige la partitura guía), porque una línea a través de todos los
instrumentos a la vez mezclaría la melodía con los acordes; para las
notas a la vez lee todas las pistas, porque un acorde suele estar
repartido entre instrumentos.

### Notas a la vez

Donde empieza una nota, `together` toma todas las notas que suenan en
ese momento, de la más grave hacia arriba, se queda con una de cada
clase de altura (una duplicación a la octava es la misma nota en un
conjunto) y las corta en grupos del tamaño elegido. Dos reglas pequeñas
hacen que los grupos tengan sentido musical:

- una sola nota que sobra arriba se une al grupo de debajo: siete notas
  de tres en tres son 3 + 4, no 3 + 3 + 1, porque una nota no es una
  armonía;
- un momento en el que suenan menos de dos notas distintas se salta.

```rust
fn chunks(notes: Vec<Note>, size: Option<usize>) -> Vec<NoteGroup> {
    if notes.is_empty() {
        return Vec::new();
    }
    let Some(size) = size else {
        return vec![NoteGroup::new(notes)];
    };
    let mut groups: Vec<NoteGroup> = notes
        .chunks(size.max(1))
        .map(|chunk| NoteGroup::new(chunk.to_vec()))
        .collect();
    if groups.len() > 1 && groups.last().is_some_and(|last| last.notes.len() == 1) {
        let left = groups.pop().map(|last| last.notes).unwrap_or_default();
        if let Some(under) = groups.last_mut() {
            under.notes.extend(left);
        }
    }
    groups
}
```

`chunks` sobre un slice lo corta en trozos de `size`, el último quizá
más corto. `pop` saca el último grupo, y `last_mut` llega al que ahora
es el último, para añadirle la nota que sobra.

El acorde de los Augurios muestra por qué importa la primera regla. De
abajo arriba las cuerdas tocan Fa♭ La♭ Do♭, luego Sol Si♭ Re♭, y Mi♭
arriba del todo. De tres en tres eso es Fa♭ La♭ Do♭ | Sol Si♭ Re♭ | Mi♭,
y el Mi♭ solitario se une al grupo de debajo: Sol Si♭ Re♭ Mi♭, que es
E♭7. La hoja escribe **4-27 T3I|3-11 T11I**, los dos acordes que
Stravinsky superpuso, encontrados solo a partir de las notas. Con
**Todas las notas**, las siete notas son un solo conjunto, **7-32 T7**:
la clase de la escala menor armónica, y las notas son exactamente La♭
menor armónica. Dos maneras de mirar un mismo acorde: dos acordes
conocidos apilados, o una escala sonando entera a la vez.

El acorde de Petrushka funciona igual. Do Mi Sol debajo de Fa♯ La♯ Do♯,
de tres en tres, es **3-11 T1I|3-11 T7I**, Fa♯ mayor sobre Do mayor, y
las seis notas juntas son **6-30 T1I**, la clase a la que a menudo se
llama el acorde de Petrushka.

### Notas en línea

`in_line` toma las notas una detrás de otra, en el orden en que
empiezan (las que suenan juntas, de la más grave hacia arriba), y cada
tantas notas forman un grupo. Aquí importan dos reglas:

- una nota repetida justo después de sí misma, a la misma altura, cuenta
  una vez: es la misma nota tocada otra vez;
- cualquier otra repetición cuenta: la misma nota en otra octava, o que
  vuelve más tarde, es una nota nueva de la línea.

La serie del Concierto op. 24 de Webern es el ejemplo clásico:

```text
Si Si♭ Re | Mi♭ Sol Fa♯ | La♭ Mi Fa | Do Do♯ La
```

De tres en tres, cada grupo es **Menor 3** (3-3, [014]): Webern
construyó toda la serie con una sola célula de tres notas, movida y dada
vuelta. De seis en seis, las dos mitades son **6-20**, el conjunto
hexatónico. Las doce en un solo grupo son el total cromático, 12-1.

Un grupo que repite el anterior se escribe una vez, donde empieza, como
la hoja escribe un acorde solo donde cambia (`merge_repeats`).

### Formas de la tonalidad, mixtas

`mixed` no corta las notas por su cuenta: se lo pide a la revisión del
color, que lee cada tramo de la canción en una tonalidad como formas
del conjunto de la tonalidad, vengan como vengan sus notas, juntas o
una tras otra (ver [Leer las formas: la lectura más
barata](micromodes.md#leer-las-formas-la-lectura-más-barata)). La hoja
escribe, donde empieza una forma, todas las formas que se oyen ahí.
Donde dos pistas, o una línea de arriba y los acordes debajo, están
cada una en una forma propia (ver [De quién son las notas de una
forma](micromodes.md#de-quién-son-las-notas-de-una-forma)), son dos
formas a la vez, escritas la de arriba primero como dos acordes a la
vez: una melodía en la T0 de Menor 3 sobre acordes en su T7 es
**3-3|3-3 T7**, con sus notas **Do Do♯ Mi | Sol La♭ Si**. El menú del
tamaño no tiene nada que elegir: una forma tiene tantas notas como el
conjunto de la tonalidad. Donde ninguna tonalidad de la canción es un
conjunto, el menú lo dice, y los compases no muestran nada.

En la demostración **Color Menor 3**, cuya banda toca las formas
escritas en la hoja, en parte como acordes y en parte como melodía, las
formas leídas de las notas son las escritas, compás por compás; una
prueba lo comprueba. Un grupo se nombra por su forma, no por las notas
oídas, porque la primera y la última forma de un tramo pueden no sonar
enteras; es el único campo que tiene un `NoteGroup` además de sus
notas:

```rust
pub struct NoteGroup {
    pub notes: Vec<Note>,
    /// The form of the key's set the notes were read as, for
    /// [`Grouping::Mixed`]: the group is named by it, though a first or
    /// a last form may not sound whole.
    pub form: Option<PitchSet>,
}
```

Sus notas son las de la parte en que se leyó, una de cada clase de
altura de la forma, en el orden en que llegan.

### La clase más usada

Se agrupen como se agrupen las notas, la pregunta que un analista se
hace después es sobre qué clase vive la música: el micromodo al que un
pasaje vuelve una y otra vez, o el conjunto del que está hecho. Debajo
de la fila de la notación, una línea lo responde:

> Más usados: **Menor 3 72%** · 2-3 6% · Menor 2 6% · 2-1 4% · 5 más

Es la melodía de **Color Menor 3** leída **en línea**, de a tres, como
micromodos. Cada grupo dura desde donde empieza hasta donde empieza el
siguiente, como la hoja escribe una armonía que dura hasta la próxima,
así que la parte de una clase es cuánto de la canción ocupa. La primera
se dibuja más brillante y las demás más tenues; al señalar una se ve
cuántas veces empieza y sus formas, primero la más usada ("Menor 3 9%,
Menor 3 T5 9%, Menor 3 T9I 6%, …"). Las díadas son lo que sobra de la
línea: donde la melodía vuelve a tocar una nota dentro de un grupo,
Do Mi♭ Do, el grupo tiene dos clases de altura, 2-3.

Leída como **Las armonías de la hoja** o como **Formas de la tonalidad
(mixtas)**, la misma demostración es Menor 3 en un 100%: cada armonía
escrita y cada forma leída es una de sus formas. En Do mayor, con
**Mostrar** en conjuntos de clases de alturas, la canción de inicio es
3-11 en un 100% también: las tríadas mayores y menores son una sola
clase. Una clase se cuenta una vez donde suenan dos grupos suyos a la
vez, así que las dos formas de **3-3|3-3 T7** ocupan el compás una vez,
no dos, aunque sean dos de sus veces. Los acordes, las formas de la
serie y N.C. quedan fuera: la línea cuenta lo que la hoja nombra como
conjuntos o micromodos, y en una canción mostrada como acordes no
aparece.

`most_used` recibe las armonías como la hoja las nombra, un paso y los
grupos nombrados ahí, y suma cada clase:

```rust
pub struct ClassUse {
    pub class: String,
    pub steps: usize,
    pub times: usize,
    pub forms: Vec<(HarmonyName, usize)>,
}

pub fn most_used(harmonies: &[(usize, Vec<HarmonyName>)], end: usize) -> Tally {
    let mut classes: Vec<ClassUse> = Vec::new();
    for (index, (start, names)) in harmonies.iter().enumerate() {
        let until = harmonies
            .get(index + 1)
            .map_or(end, |(next, _)| *next)
            .min(end);
        let steps = until.saturating_sub(*start);
        let mut counted: Vec<String> = Vec::new();
        for name in names {
            let Some(class) = class_of(name) else {
                continue;
            };
            // ... find or add the class's entry ...
            entry.times += 1;
            if !counted.contains(&class) {
                entry.steps += steps;
                counted.push(class);
            }
            // ... and add the steps to its form ...
        }
    }
    // ... sort the forms, then the classes, the longest first ...
}
```

`harmonies.get(index + 1)` es la armonía siguiente, o `None` después de
la última; `map_or(end, …)` convierte ese `None` en el final de la
canción, así que la última armonía dura hasta que la canción termina.
`saturating_sub` resta sin bajar de cero, por si una armonía empieza
después del final. `let Some(class) = class_of(name) else { continue;
}` es un **let-else**: si el nombre no es un conjunto ni un micromodo,
el bucle pasa al nombre siguiente; si lo es, `class` guarda su clase
para el resto del bucle. La lista `counted` es lo que hace que dos
grupos de una clase a la vez cuenten sus pasos una sola vez.

Las clases se ordenan por tres claves, una tras otra:

```rust
classes.sort_by(|a, b| {
    b.steps
        .cmp(&a.steps)
        .then(b.times.cmp(&a.times))
        .then_with(|| a.class.cmp(&b.class))
});
```

`b.steps.cmp(&a.steps)`, con `b` primero, ordena de más pasos a menos.
Donde dos clases suenan lo mismo, `then` compara cuántas veces empieza
cada una; donde eso también coincide, `then_with` compara sus nombres,
así el orden nunca depende de qué clase apareció primero. `then_with`
recibe un cierre y solo lo ejecuta cuando hace falta, lo que vale la
pena para comparar cadenas.

Donde los compases muestran las armonías de la propia hoja,
`written_names` renombra cada una en la notación elegida, en la
tonalidad de su paso, y la última dura hasta el final de su compás o de
la canción, lo que llegue más tarde. Donde muestran grupos leídos de
las notas, `heard_names` nombra los mismos grupos que muestran los
compases, así la línea y los compases nunca se contradicen.

### Leer solo cuando algo cambia

Cortar una canción entera en grupos y nombrarlos lleva milisegundos en
una canción larga: demasiado para repetirlo sesenta veces por segundo
mientras suena. Así que la vista guarda lo que leyó, de qué lo leyó, y
el recuento de sus clases:

```rust
struct Read {
    from: (Notation, Grouping, Option<usize>, Option<usize>),
    notes: Vec<HeardNote>,
    keys: KeyMap,
    shown: Rc<[Shown]>,
    tally: Rc<Tally>,
    pass: u64,
}
```

En cada fotograma reúne las notas de la canción, que es rápido, y las
compara, junto con las elecciones y las tonalidades, con lo que leyó la
última vez. Solo cuando algo es distinto, una elección nueva en un menú,
una nota editada en el piano roll, un cambio de tonalidad, vuelve a
cortar y a nombrar los grupos, y a contar sus clases.

La línea de los más usados se dibuja encima de los compases, así que
pide los grupos primero, y los compases los vuelven a pedir. Nada de la
canción cambia entre las dos veces: la aplicación aplica lo que se
pulsó después de dibujar el fotograma. Así que `pass` guarda el número
del fotograma en que se revisó la lectura por última vez, el
`cumulative_pass_nr` de egui, y una segunda pregunta en el mismo
fotograma recibe la misma respuesta sin volver a reunir las notas.

El resultado es un **`Rc<[Shown]>`**, un puntero con contador de
referencias a un slice. `Rc` es el primo de un solo hilo de `Arc` (ver
[Instrumentos: batería, sampler y Lineup
Osc](../sound/instruments.md#el-sampler)): clonarlo no copia los
nombres, solo suma uno a un contador. La hoja de acordes lo clona en
cada fotograma para dibujar los compases mientras la caché guarda el
suyo, y los nombres se quedan en un solo lugar de la memoria. Toda la
interfaz corre en un solo hilo, así que `Rc` basta; `Arc` también
funcionaría, pero pagaría por una seguridad entre hilos que no hace
falta.

Los grupos están ordenados por el paso en que empiezan. Para dibujar un
compás, la hoja necesita los grupos que empiezan dentro de él, y
`partition_point` los encuentra por **búsqueda binaria**:

```rust
let first = heard.partition_point(|shown| shown.start < bar.start);
let end = heard.partition_point(|shown| shown.start < bar.end());
let shown = heard[first..end].to_vec();
```

En un slice ordenado, «empieza antes del compás» es `true` para una
primera parte y `false` para el resto, y `partition_point` devuelve
dónde empieza la parte `false`. Mira unos log₂(n) elementos en lugar de
todos: unos 11 para una canción con 2000 grupos.

Los nombres de los micromodos son largos, y «Acorde menor-mayor T9» no
cabe en un cuarto de compás. Donde el nombre entero no cabe, un conjunto
o un micromodo se escribe solo con su clase, «Acorde menor-mayor» o
«3-11», y más allá como «…»; la indicación bajo el puntero da siempre el
nombre entero y sus notas. Cómo hace caber la hoja los nombres en los
compases está en [La hoja de acordes: modelo, distribución y
texto](../band/chord-sheet.md).

Los tamaños de los grupos son un pequeño struct guardado en la memoria
de egui, que la aplicación guarda en el disco, así que siguen ahí la
próxima vez que arranca. La notación y la agrupación no están en él: son
de cada canción, y toda canción se abre **Según la tonalidad** con las
armonías de la hoja (ver [Una armonía, cuatro
nombres](#una-armonía-cuatro-nombres)).

```rust
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(default)]
struct Choices {
    /// How many different notes a group of notes sounding together
    /// holds, or `None` for all of them.
    together: Option<usize>,
    /// How many notes a group of a line holds.
    line: usize,
}
```

`#[serde(default)]` sobre el struct significa que un campo que falta en
lo guardado toma su valor de `Default`. Si una versión posterior añade
un campo, las elecciones guardadas por esta siguen cargando.

## Cómo se prueba

`set_class.rs` prueba la propia teoría: cada conjunto pertenece a una
clase de su tamaño, la tabla guarda formas primas en orden con las
cantidades de los libros, las clases conocidas tienen sus nombres
conocidos (3-11, 4-28, 4-Z15, 7-35, 6-35, 5-35), Rahn y Forte difieren
para 5-20, las clases en relación Z comparten sus intervalos, las formas
normales juntan las notas al máximo, y el complemento de la escala
mayor es la escala pentatónica.

`chord_sheet/notation.rs` prueba las vistas: las mismas notas en cada
notación, y cada armonía renombrada con exactamente las notas de la
escrita; los conjuntos que forman un acorde escritos como el acorde
(3-11 T2 es Dm, Do Mi Sol La es Am7, las cuartas 4-23 T2 son A7sus4);
dos acordes a la vez como dos conjuntos; los acordes de Petrushka y de
los Augurios leídos en vertical; la serie de Webern leída en línea; y
que cada nota de una línea cuenta salvo una repetida:

```rust
#[test]
fn sets_that_make_a_chord_are_written_as_the_chord() {
    assert_eq!(renamed(Notation::Chords, "3-11 T2"), "Dm");
    assert_eq!(renamed(Notation::Chords, "{C E G}"), "C");
    assert_eq!(renamed(Notation::Chords, "Acorde disminuido T11"), "Bdim");
    // C E G A is both C6 and Am7: the simpler, more common chord.
    assert_eq!(renamed(Notation::Chords, "{C E G A}"), "Am7");
    // ...
}
```

Una prueba de la ventana (`ux_tests.rs`) toca el estilo Cuatro acordes,
recorre el menú Mostrar, comprueba los nombres, y después vuelve atrás y
comprueba que el proyecto está exactamente como estaba.

Las formas de una tonalidad que es un conjunto se prueban donde se
hacen. `keys.rs` comprueba Do mayor como conjunto (sus formas son las
doce escalas mayores, ordenadas por el círculo de quintas, T7 escrita
con Fa♯, nueve cercanas, ninguna nombrada como escala), C7 (casa,
después Re Mi Sol Si♭ como T2I, 24 formas en total, ninguna nombrada
como acorde), un hexacordo sin nombre, y que los acordes apilados con
un conjunto se nombran como conjuntos («3-11 T7I») y los grupos de una
serie solo por su lugar. Una prueba de la aplicación elige 6-20 en el
menú Tonalidad y comprueba que el menú dice «conjunto 6-20» y que las
teclas de armonía muestran sus formas. `micromodal.rs` comprueba
las maneras en que los conjuntos reparten las doce notas (18 para Menor
3, 1 para la tríada aumentada, ninguna para la disminuida, 6 para el
hexacordo cromático, cada uno a un tritono de su otra mitad), que cada
grupo de un recorrido de las doce notas las tiene todas, que Do mayor
recorre las quintas, que el recorrido desde C7 va a Em7b5 y después
sigue sin volver enseguida atrás, y que el bajo toca la fundamental de
C7 y de Em7b5. `band.rs` comprueba que la banda deja las notas que
comparten dos formas en las mismas teclas y que una progresión de
formas llega a la hoja; una prueba de la aplicación pasa una tonalidad
C7 a **Formas** y escribe una progresión desde el menú **Progresión**.

Los ejercicios sobre conjuntos están en [Ejercicios: la teoría musical
en código](exercises.md).
