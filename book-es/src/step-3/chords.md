# Acordes, y nombrar lo que escuchas

Archivos: [`crates/lineup-core/src/harmony/chord.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/harmony/chord.rs)
y [`crates/lineup-core/src/harmony/analysis.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/harmony/analysis.rs)

## La música

Un **acorde** son varias notas tocadas juntas. La mayoría de los acordes
se construyen con **terceras**: una tercera mayor son 4 semitonos, una
tercera menor son 3.

| Acorde | Construido desde la fundamental | Ejemplo sobre Do | Suena |
|--------|---------------------------------|-----------------|-------|
| Mayor | tercera mayor + tercera menor (0, 4, 7) | Do Mi Sol | brillante, asentado |
| Menor | tercera menor + tercera mayor (0, 3, 7) | Do Mi♭ Sol | más oscuro |
| Disminuido | dos terceras menores (0, 3, 6) | Do Mi♭ Sol♭ | tenso |
| Aumentado | dos terceras mayores (0, 4, 8) | Do Mi Sol♯ | inestable, soñador |
| Séptima dominante | tríada mayor + tercera menor (0, 4, 7, 10) | Do Mi Sol Si♭ | quiere seguir adelante |
| Séptima mayor | tríada mayor + tercera mayor (0, 4, 7, 11) | Do Mi Sol Si | suave, jazzy |
| Séptima menor | tríada menor + tercera menor (0, 3, 7, 10) | Do Mi♭ Sol Si♭ | apacible |

La nota más grave es la **fundamental** cuando el acorde está en
**posición fundamental**. Toca Do mayor como Mi Sol Do (Mi abajo) y sigue
siendo Do mayor, en su primera **inversión**. Los símbolos de acordes
escriben las inversiones con una barra: C/E es «Do mayor con Mi en el
bajo».

El **número romano** de un acorde dice sobre qué grado de la tonalidad
está construido, en mayúsculas para los acordes mayores y en minúsculas
para los menores. En Do mayor, Am es **vi** y G7 es **V7**. Los numerales
te permiten hablar de progresiones en cualquier tonalidad a la vez:
I, vi, IV, V es C, Am, F, G en Do mayor y G, Em, C, D en Sol mayor.

## Cualidades de acorde: una tabla en un `match`

```rust
pub enum ChordQuality {
    Major, Minor, Diminished, Augmented, Sus2, Sus4,
    Major6, Minor6, Dominant7, Major7, Minor7, MinorMajor7,
    HalfDiminished7, Diminished7, AugmentedMajor7, Dominant7Sus4,
    Add9, MinorAdd9, Dominant9, Major9, Minor9,
}

pub fn intervals(self) -> &'static [u8] {
    use ChordQuality::*;
    match self {
        Major => &[0, 4, 7],
        Minor => &[0, 3, 7],
        Dominant7 => &[0, 4, 7, 10],
        Dominant9 => &[0, 4, 7, 10, 14],
        // ...
    }
}
```

Cada cualidad tiene otros tres métodos con `match` en el mismo estilo:
`symbol` («m7»), `name` («séptima menor») y `numeral_suffix` (el «°» en
«vii°»). Escribir los datos como brazos de `match` en lugar de, digamos,
una lista de structs tiene una gran ventaja: añade una variante nueva y
el compilador enumera todos los métodos que todavía no la manejan.

`numeral_suffix` termina con un **patrón de enlace**:

```rust
match self {
    Major | Minor => "",
    Diminished => "°",
    // ...
    other => other.symbol(),
}
```

`other` coincide con todo lo que queda y le da un nombre, como `_` pero
utilizable.

Algunos acordes tienen intervalos mayores que una octava: una novena son
14 semitonos. En un `PitchSet` eso da la vuelta hasta 2, lo cual está
bien, porque las clases de alturas ignoran las octavas de todos modos.
Sí significa que Do add9 (Do Re Mi Sol como clases de alturas) solo se puede
distinguir de los demás acordes por su conjunto, lo que lleva a esta
prueba:

```rust
#[test]
fn no_two_qualities_have_the_same_notes() {
    for (i, a) in ChordQuality::ALL.iter().enumerate() {
        for b in &ChordQuality::ALL[i + 1..] {
            assert_ne!(a.pitch_set(), b.pitch_set(), "{a:?} and {b:?}");
        }
    }
}
```

`&ChordQuality::ALL[i + 1..]` es un slice del array desde `i + 1` hasta
el final, así que cada par se compara una sola vez.

Encontrar la cualidad de un conjunto de notas (visto desde la
fundamental, ver el capítulo anterior) es entonces una línea. `find`
devuelve el primer elemento para el que el closure es verdadero:

```rust
pub fn identify(relative: PitchSet) -> Option<ChordQuality> {
    Self::ALL
        .into_iter()
        .find(|quality| quality.pitch_set() == relative)
}
```

## ¿Qué está sonando? Un enum para la respuesta

El piano roll necesita nombrar lo que suene en cada paso. Puede ser
nada, una nota, dos notas, un acorde que conocemos o un grupo del que no
tenemos nombre. Cada respuesta necesita datos distintos, que es un
trabajo para un enum cuyas variantes llevan datos:

```rust
pub enum Harmony {
    Silence,
    Single(PitchClass),
    Interval { low: PitchClass, high: PitchClass, semitones: u8 },
    Chord { chord: Chord, bass: PitchClass },
    Unknown(PitchSet),
}
```

`analyze` recibe cualquier iterador de notas y devuelve un `Harmony`:

```rust
pub fn analyze(notes: impl IntoIterator<Item = Note>) -> Harmony {
    let mut pitch_set = PitchSet::EMPTY;
    let mut lowest: Option<Note> = None;
    for note in notes {
        pitch_set.insert(note.pitch_class());
        lowest = Some(match lowest {
            Some(low) => low.min(note),
            None => note,
        });
    }

    let Some(lowest) = lowest else {
        return Harmony::Silence;
    };
    let bass = lowest.pitch_class();

    match pitch_set.len() {
        1 => Harmony::Single(bass),
        2 => { /* an interval */ }
        _ => match find_chord(pitch_set, bass) {
            Some(chord) => Harmony::Chord { chord, bass },
            None => Harmony::Unknown(pitch_set),
        },
    }
}
```

`impl IntoIterator<Item = Note>` en un *argumento* significa «cualquier
cosa sobre la que puedas iterar que dé notas»: un `Vec<Note>`, un array
o un iterador directo desde el patrón, sin recolectar nada por el medio.
Un solo bucle construye el conjunto y encuentra la nota más grave, y el
`match` sobre un número (`1`, `2`, `_` para todo lo demás) elige el tipo
de respuesta.

## Encontrar el acorde

Cualquier nota de un acorde podría ser su fundamental. `find_chord`
prueba cada una, empezando por el bajo, porque los acordes suelen
tocarse con la fundamental abajo:

```rust
fn find_chord(pitch_set: PitchSet, bass: PitchClass) -> Option<Chord> {
    let roots = || {
        (0..12)
            .map(move |semitones| bass.transpose(semitones))
            .filter(move |&root| pitch_set.contains(root))
    };

    let exact = roots().find_map(|root| {
        ChordQuality::identify(pitch_set.relative_to(root))
            .map(|quality| Chord::new(root, quality))
    });

    exact.or_else(|| {
        roots().find_map(|root| {
            let mut with_fifth = pitch_set;
            with_fifth.insert(root.transpose(7));
            if with_fifth == pitch_set {
                return None; // the fifth was already there
            }
            ChordQuality::identify(with_fifth.relative_to(root))
                .filter(|quality| quality.intervals().len() >= 4)
                .map(|quality| Chord::new(root, quality))
        })
    })
}
```

- `roots` es un **closure que fabrica un iterador**. Los iteradores se
  gastan a medida que los recorres, así que para buscar dos veces, el
  código llama a `roots()` de nuevo para obtener uno fresco.
- `find_map` es `find` y `map` en uno: llama al closure con cada
  elemento y devuelve el primer `Some` que consigue.
- `or_else` en un `Option` ejecuta su closure solo si la opción es
  `None`. La segunda búsqueda, que permite una quinta ausente, solo se
  ejecuta cuando la búsqueda exacta no encontró nada. Los músicos suelen
  omitir la quinta en los acordes de séptima, y Do, Mi, Si♭ se sigue
  escuchando como C7.
- `Option::filter` conserva un `Some` solo si el closure dice que sí.
  Aquí evita que «Do y Mi, más un Sol adivinado» se nombre como un acorde
  de Do mayor completo; adivinar solo se permite en acordes de cuatro
  notas o más.

Empezar por el bajo resuelve ambigüedades reales. Do Mi Sol La es a la vez C6
y Am7; tocado sobre un Do es C6, sobre un La es Am7. Las pruebas fijan
estos casos:

```rust
assert_eq!(label(&[60, 64, 67, 69]).unwrap(), "C6");
assert_eq!(label(&[57, 60, 64, 67]).unwrap(), "Am7");
assert_eq!(label(&[64, 67, 72]).unwrap(), "C/E");
```

## Etiquetas: guardas de match y patrones anidados

El carril de acordes sobre el piano roll muestra una etiqueta corta en
cada paso:

```rust
pub fn label(&self, spelling: Spelling) -> Option<String> {
    match *self {
        Harmony::Silence => None,
        Harmony::Single(pitch_class) => Some(pitch_class.name(spelling).to_string()),
        Harmony::Interval { low, semitones: 7, .. } => Some(format!("{}5", low.name(spelling))),
        Harmony::Interval { low, high, .. } => {
            Some(format!("{}+{}", low.name(spelling), high.name(spelling)))
        }
        Harmony::Chord { chord, bass } if bass != chord.root => {
            Some(format!("{}/{}", chord.name(spelling), bass.name(spelling)))
        }
        Harmony::Chord { chord, .. } => Some(chord.name(spelling)),
        Harmony::Unknown(_) => Some("?".to_string()),
    }
}
```

Tres características de patrones en una función:

- `semitones: 7` dentro de un patrón coincide solo con ese valor. Una
  fundamental más su quinta es un «power chord», escrito Do5. Los demás
  intervalos caen al siguiente brazo.
- `..` significa «y los demás campos, sean los que sean».
- `if bass != chord.root` después de un patrón es una **guarda de
  match**: el brazo solo coincide si la condición también es verdadera.
  Las inversiones llevan barra, los acordes en posición fundamental no.

Los brazos se prueban de arriba abajo, así que el brazo más específico
debe ir primero. Si el simple `Harmony::Chord { chord, .. }` fuera
primero, el brazo de la barra nunca podría coincidir.

`match *self` hace match sobre el valor detrás de la referencia.
Funciona porque todos los campos son `Copy`; los patrones copian los
campos hacia fuera.

`describe` usa la tonalidad para escribir una frase completa, como
«Am: La menor, acorde vi de Do mayor» o «Mi: Mi mayor, III prestado de
fuera de Do mayor». Un acorde cuya fundamental está en la tonalidad pero
cuyas notas no están todas en ella (Mi mayor tiene un Sol♯, que Do mayor
no tiene) está **prestado** de otra tonalidad. Los acordes prestados son
comunes y coloridos; el mensaje solo los nombra.

## Sellos de acordes

Lo que coloca un clic en el piano roll es otro enum, con una variante
que lleva un `ChordQuality`:

```rust
pub enum ChordStamp {
    Single,
    ScaleTriad,
    ScaleSeventh,
    Fixed(ChordQuality),
}

pub fn notes_at(self, note: Note, scale: Scale) -> Vec<Note> {
    match self {
        ChordStamp::Single => vec![note],
        ChordStamp::ScaleTriad => scale.stack_thirds(note, 3).unwrap_or_else(|| vec![note]),
        ChordStamp::ScaleSeventh => scale.stack_thirds(note, 4).unwrap_or_else(|| vec![note]),
        ChordStamp::Fixed(quality) => Chord::new(note.pitch_class(), quality).notes_from(note),
    }
}
```

Los sellos de «acorde en la tonalidad» son donde el piano roll enseña
armonía: haz clic en cualquier fila resaltada y obtienes el acorde que
la tonalidad te da sobre esa nota, sin necesidad de saber antes su
nombre. El nombre aparece entonces en el carril de acordes.

El menú Dibujar enumera todos los sellos. `all()` construye esa lista
encadenando dos iteradores, los tres sellos especiales y luego un
`Fixed` por cada cualidad:

```rust
pub fn all() -> impl Iterator<Item = ChordStamp> {
    [ChordStamp::Single, ChordStamp::ScaleTriad, ChordStamp::ScaleSeventh]
        .into_iter()
        .chain(ChordQuality::ALL.into_iter().map(ChordStamp::Fixed))
}
```

`map(ChordStamp::Fixed)` pasa la propia variante como función: una
variante de tupla como `Fixed` *es* una función de `ChordQuality` a
`ChordStamp`.
