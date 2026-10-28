# Una armonía, cuatro nombres: acordes, conjuntos y micromodos en el chord sheet

Archivos: [`crates/lineup-core/src/chord_sheet/notation.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/chord_sheet/notation.rs),
[`crates/lineup-app/src/chord_sheet/notation.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/chord_sheet/notation.rs),
[`crates/lineup-app/src/chord_sheet.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/chord_sheet.rs)
y [`crates/lineup-app/src/app/ux_tests.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/ux_tests.rs)

El chord sheet ya podía guardar acordes, conjuntos de clases de alturas,
micromodos y formas de serie ([Series dodecafónicas y conjuntos de
clases de alturas](../step-8/twelve-tone.md),
[Micromodos](../step-8/micromodes.md)), pero escribía cada armonía de
una sola manera: como fue tecleada, o como la oyó [Oír
acordes](chord-hearing.md). Una canción tonal mostraba símbolos de
acorde, una pieza dodecafónica mostraba conjuntos, y no había manera de
mirar El Choclo como conjuntos ni a Webern como acordes.

Esta página añade ese conmutador, y un segundo que va más lejos: lee
las notas de la propia canción y las corta en conjuntos, ya sean las
notas que suenan juntas o las que vienen una tras otra.

Lo que gana Ferrous Lineup:

- Un menú **Show** en la fila superior del chord sheet: **As written**,
  **Chords**, **Pitch-class sets** o **Micromodes**. Las notas siguen
  igual, solo cambian sus nombres, y nada de la canción cambia.
- Para conjuntos y micromodos, un segundo menú dice qué notas forman
  cada grupo: **The sheet's harmonies**, **Notes together (vertical)** o
  **Notes in a line (serial)**.
- Cuando se leen las notas, puedes elegir **cuántas notas** tiene un
  grupo (de 2 a 12, o todas las notas que suenan juntas) y **qué
  pista** leer.
- Las elecciones se recuerdan la próxima vez que la app arranca.

![La fila superior del chord sheet con el menú Show abierto: As written, Chords, Pitch-class sets y Micromodes. Detrás, la demo Webern's trichords, sus compases escritos como los conjuntos con los que se tecleó: 3-3 T11, 3-3 T8I y así.](../images/step-9-notation-show-menu.png)

## Una armonía, cuatro nombres

La regla viene de [Una tonalidad como conjunto de clases de
alturas](../step-8/key-as-a-set.md): un conjunto de clases de alturas
es otra **notación**, no otra armonía. Mostrar Do mayor como conjunto no
cambia sus notas; escribe las mismas tres notas, Do, Mi y Sol, como su
clase de conjunto. Así que cada armonía del chord sheet tiene cuatro
nombres:

| Written | Chords | Pitch-class sets | Micromodes |
|---------|--------|------------------|------------|
| C | C | 3-11 T7I | Acorde menor-mayor T7I |
| Dm | Dm | 3-11 T2 | Acorde menor-mayor T2 |
| G | G | 3-11 T2I | Acorde menor-mayor T2I |
| Am | Am | 3-11 T9 | Acorde menor-mayor T9 |
| F | F | 3-11 T0I | Acorde menor-mayor T0I |
| G7 | G7 | 4-27 T7I | 4-27 T7I |
| E7 | E7 | 4-27 T4I | 4-27 T4I |
| 3-11 T2 | Dm | 3-11 T2 | Acorde menor-mayor T2 |
| Menor 3 | Menor 3 | 3-3 | Menor 3 |

Algunas cosas que notar:

- **Cómo leer Tn y TnI.** Toda clase de conjunto tiene una **forma
  prima** que empieza en Do. Para 3-11 es [0, 3, 7], Do Mi♭ Sol, una tríada
  menor. **T9** significa «la forma prima subida 9 semitonos»: La Do Mi,
  La menor. **TnI** significa «primero al revés, luego subida n». Al
  revés, [0, 3, 7] se convierte en 0, −3 y −7, es decir Do, La y Fa.
  Subida 7, eso es Sol, Mi y Do, Do mayor. Una tríada mayor es una tríada
  menor al revés, así que todo acorde mayor lleva una I en su nombre,
  y ningún acorde menor la lleva. Dm (3-11 T2) y G (3-11 T2I) comparten
  el número 2 pero no las notas. La propia forma prima, T0, se escribe
  sin ninguna T: Menor 3 es Do Re♭ Mi, 3-3 sobre Do.
- **Un micromodo siempre tiene tres notas.** Los doce micromodos de
  Kröpfl son las doce clases de tres notas, 3-1 a 3-12. El micromodo
  3-11 es **Acorde menor-mayor**, el acorde menor-mayor, porque las
  tríadas mayor y menor son una sola clase. G7 tiene cuatro notas, así
  que no es un micromodo: en la vista Micromodes se escribe como su
  conjunto, 4-27.
- **Chords** convierte conjuntos en acordes **cuando sus notas forman
  uno**. El conjunto 3-11 T2 es Re Fa La, así que la vista Chords escribe
  Dm, con la fundamental escrita como la tonalidad la escribe. Do Mi Sol La
  se escribe **Am7**: esas cuatro notas también son C6, y la app toma
  la primera calidad de acorde de su lista que encaje, donde los
  acordes de séptima van antes que los de sexta. Un conjunto que no
  forma ningún acorde que la app conozca, como 3-1 (Do Do♯ Re) o Menor 3
  (Do Re♭ Mi), conserva su nombre.
- **Sin acorde** sigue como N.C. en todas las vistas, y una **forma de
  serie** (P0, RI7) sigue como forma de serie: una serie es una línea
  de doce notas, no un grupo.

### Dos acordes a la vez siguen siendo dos

Un policordio ([Oír acordes](chord-hearing.md#escribir-dos-acordes-eb7fb))
es dos grupos de notas, uno sobre el otro. Fundirlos en un solo
conjunto de siete notas perdería lo que lo hace un policordio, así que
cada acorde se nombra por su cuenta y se escriben como se escribe el
policordio, **primero el de arriba**:

```text
Eb7|Fb   →   4-27 T3I|3-11 T11I
```

E♭7 (Mi♭ Sol Si♭ Re♭) es una séptima de dominante, el conjunto 4-27, y Fa♭
mayor (Fa♭ La♭ Do♭) es una tríada mayor, 3-11. La ayuda bajo el puntero lo
dice con palabras: «4-27 T3I: Sol Si♭ Re♭ Mi♭. Debajo, 3-11 T11I: Fa♭ La♭
Do♭».

### En Rust: un `match` decide

`Notation` es un enum de cuatro variantes. `#[derive(Default)]` con
`#[default]` en `Written` hace que el chord sheet empiece como siempre:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum Notation {
    #[default]
    Written,
    Chords,
    Sets,
    Micromodes,
}
```

Renombrar una armonía es un solo `match` sobre una **tupla** de la
notación y la armonía. Rust prueba los brazos desde arriba, así que su
orden es la regla: primero lo que nunca cambia, luego la vista Chords,
luego dos acordes a la vez, luego todo lo demás.

```rust
pub fn rename(self, harmony: HarmonyName, key: Scale) -> Vec<HarmonyName> {
    let group = |set: PitchSet| self.name_group(set, key);
    match (self, harmony) {
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
            let lower = notes_of(lower);
            vec![group(lower), group(upper.chord().pitch_set())]
        }
        (_, HarmonyName::Chord(symbol)) => vec![group(notes_of(symbol))],
        (_, HarmonyName::Set(set) | HarmonyName::Micromode(set)) => vec![group(set)],
    }
}
```

El primer brazo usa **or-patterns** a dos niveles: `A | B | C` entre
tuplas enteras, y `Chord(_) | Polychord(..)` dentro de una. La función
devuelve un `Vec` porque un policordio se convierte en dos nombres, y
todo lo demás en uno.

`chord_of` encuentra el acorde que forma un conjunto. Recorre las
calidades de acorde de la más simple (la tríada mayor) a la más
compleja y, para cada una, las notas del conjunto en forma normal como
posibles fundamentales. `find_map` se detiene en el primer `Some`:

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

`relative_to(root)` baja el conjunto de modo que `root` sea 0, y
`identify` dice qué calidad de acorde son esos intervalos, si es que lo
son. Como las calidades son el bucle exterior, gana la calidad listada
primero sea cual sea su fundamental: Do Mi Sol La se encuentra como séptima
menor sobre La antes de poder encontrarse como sexta sobre Do.

## Leer las notas mismas

Nombrar las armonías del chord sheet es solo la mitad de lo que hace
un analista con los conjuntos. La música sin símbolos de acorde, un
concierto de Webern o una página de la Consagración de la primavera,
se corta en conjuntos **desde sus notas**, y hay dos maneras de
cortarla:

- **Verticalmente**: las notas que suenan juntas. Así se oye un acorde
  apilado, o un cluster.
- **Horizontalmente**: las notas una tras otra, en línea. Así se corta
  una serie dodecafónica en sus tricordos o hexacordos.

Con **Pitch-class sets** o **Micromodes** elegidos, el segundo menú
elige una de las dos, y el chord sheet muestra entonces los grupos que
encuentra en la canción, cada uno donde empieza. Lo escrito en el
chord sheet no se toca: teclear en un compás sigue escribiendo su
armonía, y **As written** la muestra de nuevo.

![Los Augurs of Spring como conjuntos de clases de alturas de las notas que suenan juntas, en grupos de tres. Cada compás del acorde de las cuerdas lee 4-27 T3I|3-11 T11I, y la ayuda lo explica: las notas de cada pista sonando juntas, en grupos desde la más grave hacia arriba, escritos primero el de arriba; 4-27 T3I es Sol Si♭ Re♭ Mi♭, un acorde de séptima de dominante, y debajo 3-11 T11I es Fa♭ La♭ Do♭, una tríada mayor.](../images/step-9-notation-augurs.png)

### Notas juntas

Dondequiera que empiece una nota, el chord sheet toma **todas las
notas sostenidas en ese momento**, de la más grave hacia arriba,
conserva una de cada clase de alturas (un doblado de octava es la
misma nota en un conjunto) y las corta en grupos del tamaño elegido:
tres notas, cuatro, y así, o **All notes** en un solo grupo.

Dos reglas pequeñas mantienen los grupos musicales:

- **Una nota que sobra arriba se une al grupo de debajo.** Siete notas
  en grupos de tres serían 3 + 3 + 1, y una sola nota no es una
  armonía. Así que la última nota va con el grupo de debajo: 3 + 4.
- **Se salta un momento en el que suenan menos de dos notas
  distintas.** Una nota sola, o una nota doblada en octavas, no nombra
  ningún conjunto.

El acorde de los Augurs muestra por qué importa la primera regla. De
abajo arriba las cuerdas tocan Fa♭ La♭ Do♭, luego Sol Si♭ Re♭, luego Mi♭
arriba. En grupos de tres eso es Fa♭ La♭ Do♭ | Sol Si♭ Re♭ | Mi♭, y el Mi♭
solitario se une al grupo de debajo: Sol Si♭ Re♭ Mi♭, que es E♭7. El chord
sheet escribe **4-27 T3I|3-11 T11I**, los dos acordes que Stravinsky
puso juntos, encontrados solo desde las notas.

Con **All notes**, las siete notas son un solo conjunto, **7-32 T7**.
Esa es la clase de la escala menor armónica, y las siete notas son de
hecho exactamente la menor armónica de La♭: La♭ Si♭ Do♭ Re♭ Mi♭ Fa♭ Sol. Dos
maneras de mirar el mismo acorde: dos acordes conocidos apilados, o
una escala sonando a la vez.

El acorde de Petrushka funciona igual. Do Mi Sol bajo Fa♯ La♯ Do♯ en grupos
de tres es **3-11 T1I|3-11 T7I**, Fa♯ mayor sobre Do mayor, y las seis
notas juntas son **6-30 T1I**, la clase de conjunto a la que a menudo
se llama el propio acorde de Petrushka.

### Notas en línea

Las notas se toman una tras otra, en el orden en que empiezan (las
notas golpeadas a la vez, de la más grave hacia arriba), y cada
**tantas notas** forman un grupo: las tres primeras, las tres
siguientes, y así. Aquí importan otras dos reglas:

- **Una nota golpeada de nuevo justo después de sí misma, a la misma
  altura, cuenta una vez.** Es la misma nota tocada otra vez, no un
  miembro nuevo del grupo.
- **Cualquier otra repetición cuenta.** La misma nota en otra octava, o
  que vuelve más tarde, es una nota nueva en la línea.

La serie del *Concerto* de Webern, Op. 24, es el ejemplo clásico:

```text
B B♭ D | E♭ G F♯ | A♭ E F | C C♯ A
```

La demo Webern's trichords la toca un semitono más arriba, desde Do:
Do Si Mi♭ | Mi La♭ Sol | La Fa Fa♯ | Do♯ Re Si♭.

En grupos de tres, cada grupo es **Menor 3** (3-3, [0 1 4]): Webern
construyó toda la serie a partir de un solo micromodo, movido y
volteado ([Micromodos](../step-8/micromodes.md)). En grupos de seis,
las dos mitades son **6-20**, el conjunto hexatónico: Si Si♭ Re Mi♭ Sol Fa♯ y
La♭ Mi Fa Do Do♯ La. Las doce en un solo grupo son el agregado, 12-1.

![La demo Webern's trichords como micromodos, leída como notas en línea, tres notas por grupo, en el Lead. Cada grupo es Menor 3, movido o volteado (T8I, T2I…), con sus notas debajo: Do Si Mi♭, Mi La♭ Sol, La Fa Fa♯, Do♯ Re Si♭. Donde un grupo empieza demasiado cerca del siguiente para su nombre, se escribe como ….](../images/step-9-notation-webern-line.png)

### Qué notas

Un cuarto menú elige la pista. **Auto** lee la melodía para una línea
(la pista llamada Lead, o si no la más aguda, como la elige el lead
sheet), porque una línea a través de todos los instrumentos a la vez
mezcla la melodía con los acordes del acompañamiento. Para las notas
juntas, Auto lee **cada pista**, porque un acorde suele repartirse
entre instrumentos. También puedes elegir **All tracks** o un canal por
su nombre.

Un grupo que es otra vez el anterior se escribe una vez, donde
empieza, igual que el chord sheet escribe un acorde solo donde cambia.

## Cuando el nombre no cabe

Los nombres de micromodo son largos: «Acorde menor-mayor T9» no cabe
en un cuarto de compás. El chord sheet ya encoge los nombres para que
quepan ([Una tercera cacería de errores](bug-hunt-3.md)), pero solo
hasta un tamaño que aún se puede leer. Más allá, un conjunto o un
micromodo se escribe **solo por su clase**: «Acorde menor-mayor» o
«3-11», hasta 9 puntos. Más allá, se escribe como «…». La ayuda bajo
el puntero siempre da el nombre completo, sus notas y cuál es la
clase.

![El Choclo, en la menor armónica, como micromodos. Am es Acorde menor-mayor T9 y E7, cuatro notas, es el conjunto 4-27 T4I. En los compases más cargados, donde dos armonías comparten un compás, el micromodo se escribe solo por su clase, Acorde menor-mayor, o como … donde ni eso cabe.](../images/step-9-notation-el-choclo.png)

## Leer las notas solo cuando cambian

Cortar una canción entera en grupos lleva unos milisegundos (unos 5 ms
para Boléro en una compilación de release). Eso es demasiado lento
para hacerlo 60 veces por segundo, mientras la canción suena y el
marco del cabezal de reproducción se mueve. Así que la vista conserva
lo que leyó, y los datos desde los que lo leyó:

```rust
struct Read {
    from: (Notation, Grouping, Option<usize>, Option<usize>),
    notes: Vec<HeardNote>,
    keys: KeyMap,
    shown: Rc<[Shown]>,
}
```

Cada frame compara las elecciones, las notas y las tonalidades con
esos, y solo vuelve a leer las notas cuando uno de ellos ha cambiado:
una elección nueva en un menú, una nota editada en el piano roll, un
cambio de tonalidad.

El resultado es un **`Rc<[Shown]>`**: un puntero con contador de
referencias a un slice. `Rc` es el primo de un solo hilo de `Arc`
([Instrumentos](../step-2/instruments.md)). Clonarlo no copia los
nombres, solo suma uno a un contador. La ventana del chord sheet lo
clona en cada frame para dibujar los compases, mientras la caché
conserva su propia copia, y los nombres siguen en un solo lugar de la
memoria. `Rc` encaja aquí porque toda la interfaz de usuario corre en
un solo hilo; `Arc` funcionaría igual pero paga por una seguridad de
hilos que no hace falta.

### Encontrar los grupos de un compás con `partition_point`

Los grupos están ordenados por el paso en el que empiezan. Para
dibujar un compás, el chord sheet necesita los grupos que empiezan
dentro de él, y para enmarcar el que suena, el último que empieza en
el cabezal de reproducción o antes. `partition_point` responde a ambos
con una **búsqueda binaria**:

```rust
let first = heard.partition_point(|shown| shown.start < bar.start);
let end = heard.partition_point(|shown| shown.start < bar.end());
let shown = heard[first..end].to_vec();
```

`partition_point` toma una prueba que es `true` para una primera parte
del slice y `false` para el resto, y devuelve dónde empieza la parte
`false`. En un slice ordenado, «empieza antes del compás» es una
prueba así. Mira unos log₂(n) elementos en lugar de todos: unos 11
para una canción con 2000 grupos.

### Recordar las elecciones

Las elecciones son un struct pequeño guardado en la memoria de egui.
Con la feature `persistence` de eframe (encendida en el `Cargo.toml`
del workspace), esa memoria se guarda en disco y se vuelve a leer la
próxima vez que la app arranca:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(default)]
struct Choices {
    notation: Notation,
    grouping: Grouping,
    together: Option<usize>,
    line: usize,
}
```

`#[serde(default)]` en el struct significa que un campo que falta en
lo guardado toma su valor por defecto. Si una versión posterior añade
un campo, las elecciones guardadas por esta versión siguen cargando.

## Pruebas

El módulo core tiene diez pruebas, entre ellas:

- **las mismas notas en cada notación**: C, Dm y G7 en cada vista.
- **los conjuntos que forman un acorde se escriben como el acorde**:
  3-11 T2 es Dm, Do Mi Sol La es Am7, Mi La Re Sol es A7sus4, mientras Menor 3 y
  3-1 siguen siendo conjuntos.
- **dos acordes a la vez son dos conjuntos**: Eb7|Fb es
  4-27 T3I|3-11 T11I.
- **el acorde de Petrushka leído verticalmente en grupos de tres**, y
  como 6-30 todo junto.
- **el acorde de los Augurs en grupos de tres es F♭ bajo E♭7**, y
  7-32 T7 todo junto.
- **la serie de Webern en línea son cuatro grupos de Menor 3**, con
  6-20 para cada mitad y el agregado para las doce, y la **demo de
  Webern** lee igual desde el Lead y desde sus acordes.
- **toda nota de una línea cuenta salvo una repetida**.

Una prueba de la ventana (`ux_tests.rs`) abre Four chords, recorre el
menú Show haciendo clic y comprueba los nombres (3-11 T7I, 3-11 T2I,
3-11 T9, Acorde menor-mayor T7I…), luego vuelve atrás y comprueba que
el proyecto es exactamente como era.
