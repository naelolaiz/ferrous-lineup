# Más demos: canciones para todos los géneros

Archivos: [`crates/lineup-core/src/library/song.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/song.rs),
los archivos de género en [`crates/lineup-core/src/library/song/`](https://github.com/naelolaiz/ferrous-lineup/tree/main/crates/lineup-core/src/library/song)
y [`crates/lineup-app/src/app/library.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/library.rs)

Las primeras demos eran 16 canciones. Se oían bien, pero la mayoría de
los géneros de la biblioteca no tenían ninguna canción: había un *estilo*
de tango, pero ningún tango con melodía del que aprender. Ahora **File >
Demos > Songs** tiene 62 canciones, con un submenú para cada género y al
menos dos canciones en cada uno.

## De dónde salen las canciones

La mayoría son melodías de **dominio público**, elegidas para que cada
género tenga algo que escuchar:

| Género | Canciones |
|---|---|
| Pop | Auld Lang Syne, London Bridge, Old MacDonald, Deck the Halls |
| Rock | Drunken Sailor, Bella Ciao, Beethoven's Fifth, Yankee Doodle |
| Blues | Midnight Shuffle\*, Rolling Boogie\*, Minor Mood Blues\* |
| Jazz | Jingle Bells, When the Saints, The Entertainer, Happy Birthday, Two-Five Walk\* |
| Soul, funk y R&Si | Amazing Grace, Joy to the World, Dorian Strut\*, Sunday Seventh\* |
| Hip hop y lo-fi | Twinkle Twinkle, Gymnopédie, Prelude in C, Sakura Sakura, Dies Irae |
| Electrónica | Ode to Joy, Korobeiniki, Symphony No. 40, Eine kleine Nachtmusik, Toccata in D minor, Turkish March |
| Latina | La Cucaracha, Rio Afternoon\*, Carnival Morning\* |
| Tango | Habanera, Tango del telar\*, Milonga del río\*, Vals del puerto\* |
| Flamenco | Rumba del sur\*, Noche frigia\* |
| Reggae y ska | Mary Had a Little Lamb, This Old Man, Valley Dub\* |
| Country y folk | Frère Jacques, Greensleeves, Oh! Susanna, Scarborough Fair, Clementine, Camptown Races, Row Row Row Your Boat, Silent Night, We Wish You a Merry Christmas |
| Clásica y cine | Canon in D, Für Elise, Mountain King, Minuet in G, The Blue Danube, Brahms' Lullaby, Morning Mood, New World Largo, Shchedryk, Funeral March |

Las marcadas con \* dicen "Written for Ferrous Lineup" en su tooltip.
Algunos géneros tienen muy pocas melodías famosas lo bastante antiguas
para ser libres: los grandes tangos, bossa novas y canciones de blues
datan en su mayoría de los años 1910 en adelante, y sus compositores
murieron hace demasiado poco. En lugar de dejar esos géneros vacíos,
recibieron melodías nuevas, escritas como las escriben esos géneros:

- **Sobre los propios acordes del estilo.** *Tango del telar* usa la
  tabla del estilo «Tango», `i | V7 | V7 | i | iv | i | V7 | i`,
  *Dorian Strut* la del «Funk vamp», *Noche frigia* la del «Phrygian
  flamenco». Abre la canción y el estilo uno al lado del otro: la
  armonía es la misma, la canción añade una melodía.
- **Notas del acorde en los pulsos fuertes.** Una melodía suena «bien»
  cuando las notas que caen en el pulso pertenecen al acorde que hay
  debajo. Las notas de en medio son libres de pasar: en *Midnight
  Shuffle*, el Sol y el Do de la escala de blues chocan a propósito con
  el acorde de La7, que es el sonido del blues.
- **El ritmo del género en la melodía.** *Milonga del río* toca el 3 +
  3 + 2 de la milonga también en su melodía: `E5:3 E5 G5:2 C6:8`, dos
  corcheas con puntillo, una corchea y luego una nota larga.

## Un archivo por género

62 canciones harían `song.rs` de miles de líneas. Así que las canciones
se mudaron a una carpeta, un archivo por género, junto a `song.rs`:

```text
library/
  song.rs          los tipos Song y Part, SONGS, las pruebas
  song/
    blues.rs
    classical.rs
    ...
    tango.rs
```

Un archivo `song.rs` y una carpeta `song/` con el mismo nombre funcionan
juntos: `mod tango;` dentro de `song.rs` hace que Rust busque
`song/tango.rs`. Cada archivo de género es un **módulo hijo** de `song`.

### Cada canción es un `const`

En un archivo de género, cada canción es una constante:

```rust
pub(super) const HABANERA: Song = Song {
    name: "Habanera",
    origin: "Georges Bizet, Carmen, 1875",
    feel: "Habanera",
    genre: Genre::Tango,
    tonic: D,
    mode: NaturalMinor,
    ...
};
```

`pub(super)` lo hace visible para el módulo padre, `song`, y nada más:
nada fuera de la biblioteca necesita nombrar `HABANERA`, solo
encontrarla en `SONGS`. Luego `song.rs` las enumera todas:

```rust
pub static SONGS: &[Song] = &[
    pop::AULD_LANG_SYNE,
    pop::LONDON_BRIDGE,
    ...
    classical::FUNERAL_MARCH,
];
```

¿Por qué no hacer que cada archivo haga su propia lista y juntarlas?
Porque `SONGS` es un `static`, construido cuando se compila el programa,
y Rust no tiene forma de pegar slices en tiempo de compilación.
Enumerar cada `const` por su nombre es la manera sencilla de hacerlo. Un
`const` se copia en cada sitio donde se usa, así que escribir
`tango::HABANERA` en la lista pone la propia canción en `SONGS`; el
`static` tiene entonces un único lugar fijo en memoria, y por eso el
menú puede repartir referencias `&'static Song` a él.

### `use super::*` y qué pueden ver los hijos

Cada archivo de género empieza con una línea:

```rust
use super::*;
```

`super` es el módulo padre, `song`, y `*` es un **glob import**: trae
todos los nombres que el padre puede ver. Eso incluye `Song`, `Part`,
`Comping`, `Bass` y `Genre`, los compases, y también los elementos
*privados* del padre, como el `const COMMON` y sus propias líneas `use`.
Un elemento privado en Rust es privado para su módulo **y los hijos de
ese módulo**, así que un hijo siempre puede usar lo que su padre se
guarda para sí.

Las líneas `use` del padre importan aquí:

```rust
use PitchClass::*;
use ScaleKind::*;
```

Permiten que una canción escriba `tonic: A` y `mode: Dorian` en lugar de
`PitchClass::A` y `ScaleKind::Dorian`. `Genre` no se importa así, a
propósito: `Genre::Blues` y `ScaleKind::Blues` se llamarían ambos
`Blues`, y dos glob imports con el mismo nombre son un error en cuanto
lo usas. Así que los géneros se escriben siempre completos, `genre:
Genre::Blues`, y `mode: Blues` significa la escala.

## El menú, por género

Cada canción lleva ahora su `genre`, así que el menú Songs hace lo que
ya hacía el menú Styles: un submenú por género, con las canciones de ese
género dentro.

```rust
for genre in Genre::ALL {
    ui.menu_button(genre.name(), |ui| {
        for song in SONGS.iter().filter(|song| song.genre == genre) {
            ...
        }
    });
}
```

`filter` recorre las 62 canciones una vez por género, 13 veces en total.
Son 806 comparaciones de dos enums pequeños por frame mientras el menú
está abierto, demasiado pocas para importar, y mantiene una sola lista
como única verdad.

## Pruebas para los datos nuevos

Las pruebas de [la biblioteca](library.md#está-bien) ya comprueban
cada compás de cada melodía y que las melodías se mantengan en su
mayoría en la tonalidad, así que las 46 canciones nuevas se comprobaron
en el momento de escribirlas. Tres comprobaciones nuevas vinieron con
ellas:

- **cada género tiene al menos dos canciones**, para que ningún submenú
  esté nunca vacío,
- **`SONGS` está ordenada por género**, para que su orden coincida con
  el menú:

  ```rust
  SONGS.windows(2).all(|pair| pair[0].genre <= pair[1].genre)
  ```

  `windows(2)` da cada pareja de vecinos, `[a, b]`, `[b, c]` y así.
  `<=` funciona con los géneros porque `Genre` deriva `PartialOrd`, que
  ordena las variantes de un enum en el orden en que están escritas,
- **la batería de cada canción está en su compás**: una canción de vals
  que pidiera un ritmo de 4/4 tocaría un compás de batería que no se
  alinea con sus compases, algo que ninguna otra prueba notaría.

## Fieles a la partitura

La primera tanda de canciones se escribió de memoria, y la memoria es un
mal copista. Tocadas contra su batería, varias demos sonaban un pulso
antes, algunos acordes estaban claramente mal, y piezas que cambian de
tonalidad en el original se quedaban en una sola tonalidad de principio
a fin. Cada melodía se comprobó entonces contra una transcripción de
referencia, y 26 canciones se corrigieron. Tres de los arreglos
necesitaron un cambio en la propia biblioteca, y uno en la app.

### Las anacrusas van en una parte propia

Muchas melodías empiezan antes de su primer compás: las tres notas
cortas antes de «La cucaracha», el «Una mattina» de Bella Ciao, el «In
a» de Clementine. Esas notas son una *anacrusa* (un *pickup*), y la
primera sílaba acentuada cae en el pulso fuerte del compás uno. Los
datos antiguos ponían la anacrusa al principio del compás uno, lo que
empujaba toda la melodía un pulso antes contra la batería y los cambios
de acorde, así que una canción sonaba bien sola y mal en cuanto entraba
la batería.

Una anacrusa es ahora una parte propia de un compás, casi toda silencio:

```rust
Part::new("Pickup", "-:12 D5:2 C#5", "i"),
```

En 4/4 un compás son 16 pasos, así que un silencio de 12 pasos seguido
de dos corcheas pone la anacrusa en el último pulso, y el primer compás
real de la siguiente parte empieza en el pulso fuerte. La forma nombra
la anacrusa primero, `&["Pickup", "Aria", ...]`, y la batería entra
después de ella, que es el siguiente cambio.

### Batería desde el compás uno

`Song` tiene un campo `drums_from`: la entrada de `form` en la que
empiezan los clips de batería. La primera tanda lo ponía a 1 para cada
canción, con la idea de que la melodía sonara sola durante una pasada
antes de que entrara la batería. En la práctica, lo primero que se oía
era un estribillo entero de silencio de la pista de batería, lo bastante
largo para pensar que la demo no había cargado ningún ritmo. Ahora cada
canción empieza su batería en la entrada 0, o en la entrada 1 cuando la
entrada 0 es una anacrusa o una intro. La comprobación en `song_demo` no
cambia:

```rust
if entry >= song.drums_from
    && let Some(&(_, drum)) = drums.iter().find(|(part, _)| part == name)
{
    project.add_clip(Clip::new(drum, DRUM_TRACK, at));
}
```

Un `&&` seguido de `let` es una *let chain*: la segunda condición a la
vez comprueba si esta parte tiene un patrón de batería y lo liga a
`drum` de una vez. Siete piezas de piano y corales conservan `rhythm:
""`: un preludio de Chopin con batería no es la idea que nadie tiene de
una mejora.

### Una parte en otra tonalidad

In the Hall of the Mountain King sube de Si menor a Fa sostenido mayor
en su segunda mitad; la Marcha turca tiene un estribillo en La mayor; la
Habanera de Bizet pasa de Re menor a Re mayor cada vez que vuelve su
estribillo. Los acordes de esas partes no se pueden escribir como
numerales en la tonalidad de la canción sin mentir, así que una `Part`
puede llevar ahora su propia tonalidad, igual que los [marcadores de
tiempo](time-markers.md) le dejan llevar un compás:

```rust
pub struct Part {
    ...
    pub time: Option<TimeSignature>,
    pub key: Option<(PitchClass, ScaleKind)>,
}

impl Part {
    const fn in_key(self, tonic: PitchClass, mode: ScaleKind) -> Part {
        Part {
            key: Some((tonic, mode)),
            ..self
        }
    }
}
```

`..self` es *sintaxis de actualización de struct*: todo campo no
nombrado se copia de `self`, así que `in_key` cambia un campo y conserva
el resto. Puede ser `const` porque solo mueve valores de un lado a otro,
lo que permite que una canción siga siendo un `const` con
`.in_key(FSharp, Major)` al final de una de sus partes.

La canción responde en qué tonalidad está una parte:

```rust
pub fn part_scale(&self, part: &Part) -> Scale {
    match part.key {
        Some((tonic, mode)) => Scale::new(tonic, mode),
        None => self.scale(),
    }
}
```

y `song_demo` usa la tónica de esa parte cuando acompaña la tabla,
`accompany(&part.spans(bar), tonic, ...)`, así que `I` en la parte de Fa
sostenido mayor es un acorde de Fa sostenido. El propio proyecto
conserva la tonalidad principal de la canción, como una partitura
conserva su armadura, y la prueba de que las melodías se mantienen en su
mayoría en la tonalidad comprueba ahora cada parte contra su propia
escala en lugar de la de la canción.

### Cancelar significa quedarse

Cargar una demo mientras el proyecto actual tiene cambios sin guardar
pregunta "Save changes?", con Save, Don't save y Cancel. Cancel, Escape
y un clic fuera del diálogo no hacían nada, lo cual era correcto, y
silencioso. Sin nada que mirar, era fácil creer que la demo se había
cargado y conservaba el ritmo antiguo. Ahora Cancel lo dice en la barra
de avisos:

```rust
Some(Choice::Cancel) => {
    self.file.pending = None;
    self.notice = Some(format!("Cancelled: \"{name}\" stays open as it is."));
}
```

`{name}` dentro de `format!` lee una variable de ese nombre del scope
circundante, así que no hace falta pasarla como argumento. La propia
carga ya era concienzuda: `replace_project` limpia el motor y le envía
cada patrón, clip y ritmo del proyecto nuevo, así que nada del anterior
sobrevive a una carga.
