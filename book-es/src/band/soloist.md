# El solista

Archivos: [`crates/lineup-core/src/harmony/chord_scale.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/harmony/chord_scale.rs),
[`crates/lineup-core/src/band/solo.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/band/solo.rs),
[`crates/lineup-core/src/band.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/band.rs),
[`crates/lineup-core/src/file.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/file.rs),
[`crates/lineup-app/src/chord_sheet.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/chord_sheet.rs)
y [`crates/lineup-app/src/piano_roll.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/piano_roll.rs)

La banda acompaña: toca los acordes para que otro toque la melodía o
improvise sobre ellos. El **solista** es ese otro, un músico que
improvisa una línea sobre la hoja de acordes. Es experimental y está
apagado hasta que lo enciendes, porque un solo hecho con reglas es un
estudio, no una interpretación. Pero es un estudio útil: cada nota que
toca tiene una razón, y el piano roll muestra la razón, así que puedes
ver cómo se construye una línea improvisada y probar las mismas reglas
en tu propio instrumento.

## Probarlo

Pulsa **Ctrl+N** para una canción nueva con banda, y escribe estos
compases en la hoja de acordes, un acorde por compás:

```text
| Dm7 | G7 | Cmaj7 | A7 | Dm7 | G7 | C6 | C6 |
```

En la **Biblioteca**, aplica **Swing** (en Jazz). Luego abre el menú
**🎷 Banda** de la hoja y marca **Solista (experimental)**. Pasa antes
el ratón por encima: la pista dice quién es el solista en este estilo,
un saxo tenor. Aparece una pista **Solista** en la lista de
reproducción, y el rack de canales recibe un canal Solista con el saxo
tenor. Pulsa Espacio: sobre la banda, el saxo toca frases de dos
compases en corcheas con swing, con una respiración entre ellas.

**Otro solo**, en el mismo menú, toca uno distinto; deshacer trae de
vuelta el anterior. Desmarca el solista y la pista y el canal se van.
Como toda parte de la banda, el solo se vuelve a tocar cuando cambia la
hoja, y si editas sus notas se queda como lo dejaste hasta que pulsas
**Volver a tocarla** (u **Otro solo**).

Ahora haz doble clic en el clip del solista para abrir su parte en el
piano roll. Sus notas tienen el color de lo que son:

- **verde** para las notas del acorde, marcadas con cuál es: `R` para
  la fundamental, `3`, `5`, `7` (y `6`, `9` cuando el acorde las
  tiene), o `•` para una nota de un conjunto, que no tiene fundamental
  desde la que contar;
- **azul** para las demás notas de la escala del acorde: `p` una nota
  de paso, `n` una bordadura, `s` cualquier otra;
- **naranja** para las notas cromáticas, de fuera de la escala: `a`
  una aproximación, `cp` una nota de paso cromática.

Señala una nota, o toca la canción, y la línea sobre la cuadrícula dice
qué es, en una frase:

![El piano roll de «Soloist 1» sobre los compases 1 a 5. Las notas son verdes, azules o naranjas con marcas encima: «7», «p», «5», «3», «R», «B3 3», «D4 5», «cp», etc. Sobre la cuadrícula: «Bar 3, beat 4, step 3 · Bb3: a chromatic passing note, a half step from the notes either side. Over Cmaj7 the scale is C major (Ionian): C D E F G A B.» y la leyenda «Soloist: chord tones green (R 3 5 7), scale notes blue (p passing, n neighbour, s other), chromatic orange (a approach, cp passing)»](../images/step-9-soloist-piano-roll.png)

En este solo, el compás 3 termina con Si3 (la 7ª de Cmaj7) en el pulso
4, luego Si♭3 en el «y», y luego La3 en el pulso 1 del compás 4, la
fundamental de A7: tres notas a un semitono una de otra, bajando hacia
el acorde siguiente. El Si♭ no está en ninguno de los dos acordes, y no
pasa nada: está en la mitad débil del pulso y lleva a algún sitio.

## La música: cómo se construye una línea improvisada

### Las notas del acorde en el pulso

Pregúntale a un profesor de jazz cómo empezar a improvisar sobre una
progresión, y la primera regla es casi siempre la misma: **toca notas
del acorde en los pulsos fuertes, y lo que las una en medio**. En el
pulso, el oído compara la melodía con la armonía; entre los pulsos la
melodía está en movimiento y el oído perdona más. Una línea que cae en
una nota del acorde en cada pulso «dibuja los acordes»: se oyen en ella
aun sin la banda.

Las notas del acorde que más dicen son la **3ª** y la **7ª**, las
**notas guía**. Dicen si un acorde es mayor o menor y qué tipo de
séptima tiene, y de un acorde al siguiente de un ii–V–I se mueven por
semitonos: la 7ª de Dm7 (Do) cae a la 3ª de G7 (Si), y la 7ª de G7 (Fa)
cae a la 3ª de Cmaj7 (Mi). Muchos solos son notas guía unidas.

El solista elige, para cada pulso, la nota del acorde **más cercana** a
donde va: la línea sube o baja, y mantiene su dirección hasta girar en
el borde del registro del instrumento, o de vez en cuando por su
cuenta. Una de cada cinco veces se salta una nota del acorde y salta a
la siguiente. Eso da líneas que se mueven sobre todo por pasos
pequeños, como las melodías, con un salto aquí y allá.

### Escalas de acorde

Entre las notas del acorde van otras notas, y cuáles depende del
acorde: sobre Dm7, ¿la nota entre La y Do es un Si o un Si♭? La
enseñanza del jazz responde con las **escalas de acorde**: cada acorde
recibe una escala de siete notas que lo contiene, y las notas de en
medio salen de esa escala.

La escala no se elige solo a partir del acorde, sino del acorde **en
su tonalidad**. Sobre Dm7 en Do mayor, el Si es natural, porque la
música está en Do: la escala es Do mayor desde Re, Re Mi Fa Sol La Si
Do, llamada **Re dórico**. Los modos de Do mayor son lo que tocan los
acordes de Do mayor:

| Acorde en Do mayor | Su escala     | Notas                  |
|--------------------|---------------|------------------------|
| Cmaj7 (I)          | Do jónico     | Do Re Mi Fa Sol La Si  |
| Dm7 (ii)           | Re dórico     | Re Mi Fa Sol La Si Do  |
| Em7 (iii)          | Mi frigio     | Mi Fa Sol La Si Do Re  |
| Fmaj7 (IV)         | Fa lidio      | Fa Sol La Si Do Re Mi  |
| G7 (V)             | Sol mixolidio | Sol La Si Do Re Mi Fa  |
| Am7 (vi)           | La eólico     | La Si Do Re Mi Fa Sol  |
| Bm7♭5 (vii)        | Si locrio     | Si Do Re Mi Fa Sol La  |

Siete nombres para las mismas siete notas. Los nombres importan porque
dicen cuál es la fundamental y dónde caen los semitonos desde ella: el
dórico es una escala menor con 6ª mayor (Si sobre Re), el lidio una
escala mayor con 4ª aumentada (Si sobre Fa), el mixolidio una escala
mayor con 7ª menor (Fa sobre Sol). Esas son las notas que le dan a cada
acorde su color en la tonalidad.

Los demás acordes reciben su escala como la elegiría un músico que
piensa en la tonalidad, en este orden:

1. **Un acorde de la tonalidad** toca la escala de la tonalidad desde
   su fundamental, como en la tabla. En una tonalidad menor, un acorde
   que usa la 7ª elevada (E7 en La menor, con Sol♯) toca la **menor
   armónica** de la tonalidad desde su fundamental: La menor armónica
   desde Mi, Mi Fa Sol♯ La Si Do Re, llamada **Mi frigio dominante**.
2. **Un acorde prestado de la tonalidad homónima** toca la escala de
   esa tonalidad: Fm en Do mayor viene de Do menor, así que toca Do
   menor desde Fa, que es Fa dórico. B♭7 en Do mayor (también de Do
   menor) toca Si♭ mixolidio.
3. **Una dominante secundaria** toca la escala del acorde al que lleva.
   D7 en Do mayor es el V7 de Sol, así que toca Sol mayor desde Re, Re
   mixolidio. A7 en Do mayor es el V7 de Dm, un acorde menor, así que
   toca Re menor armónica desde La: La Si♭ Do♯ Re Mi Fa Sol, La frigio
   dominante. Sobre A7, un Si♭ es una nota de la escala, que apunta al
   acorde de Re menor que sigue.
4. **Cualquier otro** toca la escala que suele recibir su tipo:
   **lidio** para un acorde mayor de fuera de la tonalidad (el Re♭ de la
   sexta napolitana en Do), **lidio dominante** para una séptima de
   dominante que viene de fuera y no lleva a ninguna parte,
   **disminuida semitono-tono** para 7♭9 y 7♯9, **disminuida
   tono-semitono** para una séptima disminuida, **de tonos enteros**
   para un acorde aumentado, **locrio** para uno semidisminuido,
   **dórico** para un acorde menor. En una tonalidad que no es una
   escala de siete notas (una escala de blues, una pentatónica), cada
   acorde recibe la escala habitual de su tipo, con los acordes mayores
   sobre la escala mayor (jónica) y las dominantes sobre la mixolidia.

Una regla más: **las notas del propio acorde ganan**. E9 en Do mayor
lleva a Am, así que parte de La menor armónica (Mi Fa Sol♯ La Si Do Re),
pero su 9ª es Fa♯, y Fa y Fa♯ no pueden estar los dos en la escala con
el mismo nombre. El Fa♯ del acorde reemplaza al Fa: Mi Fa♯ Sol♯ La Si
Do Re, **mixolidia ♭6**.

Hay una segunda escuela, que da a cada acorde su propia escala sin
preguntar por la tonalidad (dórico para todo m7, mixolidio para todo
7). Es más fácil de enseñar y a menudo suena igual. Pensar en la
tonalidad está más cerca de cómo se escribieron los temas, y mantiene
iguales de un acorde a otro las notas que los acordes de la tonalidad
tienen en común.

### Las notas entre los pulsos

Cada tipo de nota entre las notas del acorde tiene un nombre, y la
mayoría son más viejos que el jazz: los libros de armonía las llaman
**notas extrañas al acorde**.

- Una **nota de paso** llena el hueco entre dos notas del acorde a una
  tercera, por grado conjunto, en la misma dirección: Do **Re** Mi sobre
  C.
- Una **bordadura** (o nota auxiliar) se aleja un paso de una nota del
  acorde y vuelve: Mi **Fa** Mi.
- Una **nota de paso cromática** llena un tono entero con el semitono
  de en medio: Re **Re♯** Mi.
- Una **aproximación cromática** entra en una nota del acorde desde un
  semitono por debajo (o por encima): **Fa♯** Sol. Cuando el solista
  empieza una frase con una nota justo antes del pulso, una
  **anacrusa**, suele ser una de estas.
- Cualquier otra nota de la escala, alcanzada o dejada con un salto, se
  marca `s`, y una nota de fuera de la escala que no hace nada de esto
  se marca `x`. El solista no toca notas `x`, pero una parte que
  edites sí puede tenerlas.

Aquí un paso es un semitono o un tono, o, en la menor armónica, la
segunda aumentada entre su 6ª y su 7ª (Fa y Sol♯ en La), porque esas
dos están una al lado de la otra en la escala.

Las notas de paso cromáticas en el «y» del pulso son parte del sonido
del **bebop**, el jazz de Charlie Parker y Dizzy Gillespie: un semitono
añadido entre dos notas de la escala mantiene las notas del acorde en
los pulsos cuando una línea corre en corcheas. Los profesores de jazz
lo escribieron más tarde como las «escalas bebop».

### Frases y respiración

Quien canta tiene que respirar, y quien toca un instrumento de viento
también, así que las líneas vienen en **frases**. El solista toca
frases de dos compases. Cada una puede empezar un pulso tarde o con una
anacrusa, y deja uno o dos pulsos vacíos al final. Una línea que nunca
se detiene suena a ejercicio; los silencios son lo que la hace sonar a
alguien que habla. Luego las corcheas llevan swing como la banda (ver
[Swing](../sound/tempo.md#swing)), y el fraseo de melodía de la
biblioteca da forma a su volumen, como hace con las melodías de las
demostraciones.

El solista toca sobre las vueltas, no sobre la introducción ni el final
(ver [La
forma](song-form.md#la-forma-una-introducción-vueltas-y-un-final)),
y sigue tocando a través de los golpes y los silencios (ver [Golpes,
sostenidos, silencios y anticipaciones](stops.md)): la banda
se detiene precisamente para que un solista se oiga solo. Sobre «sin
acorde» se queda callado. Sobre un conjunto de clases de alturas, un
micromodo o una forma de serie no hay escala que elegir, y el solista
toca las notas del propio conjunto, en los pulsos y entre ellos.

### Quién toca

Cada familia de estilos tiene su solista, como instrumento General MIDI
y con el registro en el que toca. La familia es la de los instrumentos
de la banda (su **Formación de instrumentos**):

| Estilos                 | Solista             | Registro      |
|-------------------------|---------------------|---------------|
| Pop                     | saxo alto           | Sol3 a Sol5   |
| Rock                    | guitarra saturada   | La3 a Do6     |
| Blues                   | armónica            | Do4 a Do6     |
| Jazz                    | saxo tenor          | Re3 a Fa5     |
| Soul, funk y R&B        | saxo alto           | Sol3 a Sol5   |
| Hip hop y lo-fi         | vibráfono           | Do4 a Do6     |
| Electrónica             | solista de sierra   | Do4 a Do6     |
| Latina                  | trompeta            | Si♭3 a La5    |
| Tango                   | acordeón de tango   | Sol3 a Do6    |
| Folclore latinoamericano | flauta de pan      | Mi4 a Mi6     |
| Flamenco                | guitarra de nailon  | Mi3 a La5     |
| Balcánica y turca       | clarinete           | Re3 a Si♭5    |
| Reggae y ska            | trompeta            | Si♭3 a La5    |
| Country y folk          | violín folk         | Sol3 a Mi6    |
| Clásica y de cine       | flauta              | Fa4 a Fa6     |

El acordeón de tango es el sustituto que General MIDI tiene para el
bandoneón. Los registros son el centro cómodo de cada instrumento, no
todo él: los solos se quedan donde un instrumento canta.

## Cómo funciona

### Elegir una escala de acorde

`chord_scale`, en `harmony/chord_scale.rs`, sigue las cuatro reglas de
arriba, un `if` detrás de otro:

```rust
pub fn chord_scale(chord: Chord, key: Scale) -> PitchSet {
    if !key.kind.is_tonal() || !key.is_heptatonic() {
        return usual_scale(chord, false);
    }
    let harmonic = |root: PitchClass| Scale::new(root, ScaleKind::HarmonicMinor);
    let source = if key.is_chord_of_key(chord) {
        match key.uses_raised_seventh(chord) {
            true => harmonic(key.root),
            false => key,
        }
    } else if let Some(parallel) = key.borrowed_from(chord) {
        match parallel.is_diatonic(chord) {
            true => parallel,
            false => harmonic(key.root),
        }
    } else if let Some(target) = key.secondary_dominant_target(chord) {
        match target.quality.is_minor() {
            true => harmonic(target.root),
            false => Scale::new(target.root, ScaleKind::Major),
        }
    } else {
        return usual_scale(chord, true);
    };
    with_chord_notes(source, chord)
}
```

Casi toda la teoría musical está en otra parte: `is_chord_of_key`,
`borrowed_from` y `secondary_dominant_target` son lo que usa la hoja de
acordes para nombrar acordes prestados y dominantes secundarias (ver
[Los métodos de la
tonalidad](../theory/numerals.md#los-métodos-de-la-tonalidad)). Esta
función solo los pone en el orden en que un músico haría las
preguntas.

Tres trozos de Rust que merecen atención:

- Un `if` es una **expresión** en Rust: `let source = if ... { a } else
  if ... { b } else { ... };` le da a `source` el valor de la rama que
  se ejecutó. La última rama no da un valor; hace `return` desde toda
  la función, lo que está permitido porque `return` tiene el tipo `!`
  («never», nunca), que encaja en cualquier sitio.
- `else if let Some(parallel) = key.borrowed_from(chord)` comprueba y
  desempaqueta a la vez: la rama se ejecuta solo cuando hay una
  tonalidad homónima, y dentro de ella `parallel` es esa tonalidad.
- `harmonic` es un **closure**, una función pequeña escrita en su
  sitio, usada tres veces. `|root: PitchClass| Scale::new(...)` recibe
  una fundamental y da una escala.

`usual_scale` es un solo `match` sobre la especie del acorde, con una
condición para los acordes de fuera de la tonalidad:

```rust
Major | Add9 | Major6 | Six9 | Major7 | Major9 | Major13 if outside => LYDIAN,
Major | Add9 | Major6 | Six9 | Major7 | Major9 | Major13 | Sus2 => IONIAN,
```

y termina uniendo la escala y el acorde. Un `PitchSet` son doce bits en
un número, uno por cada clase de alturas, así que unir dos conjuntos es
un o bit a bit de los números:

```rust
PitchSet::from_bits(scale.bits() | chord.pitch_set().bits())
```

### Las notas del acorde ganan, letra a letra

```rust
fn with_chord_notes(source: Scale, chord: Chord) -> PitchSet {
    let names = source.chord_notes(chord);
    let mut set = chord.pitch_set();
    for pitch_class in source.pitch_classes() {
        let letter = source.spell(pitch_class).letter;
        let replaced = names.iter().any(|name| {
            name.letter == letter
                && name.pitch_class() != pitch_class
                && !source.contains(name.pitch_class())
        });
        if !replaced {
            set.insert(pitch_class);
        }
    }
    set
}
```

El conjunto empieza con las notas del acorde, y cada nota de la escala
de origen se le une salvo que una nota del acorde tenga la **misma
letra** pero otra altura, de fuera de la escala. Las letras, no las
alturas, deciden qué nota se reemplaza: el Fa♯ de E9 reemplaza a Fa, no
a Sol. Por eso la escala se escribe con nombres de nota (`NoteName`,
una letra y una alteración) antes de compararla, y no solo con
números.

La última condición, `!source.contains(...)`, evita que las grafías
enarmónicas hagan daño. Fdim7 en Do mayor se escribe Fa La♭ Do♭ Mi♭♭,
y Do♭ y Mi♭♭ tienen las letras de Do y de Mi: sin la condición sacarían
Do y Mi de la escala, aunque Do♭ es Si y Mi♭♭ es Re, las dos ya en Do
mayor. El test `every_chord_scale_holds_its_chord` toca cada especie de
acorde sobre cada fundamental en varias tonalidades y comprueba que
cada escala contiene su acorde y tiene de seis a nueve notas.

### Nombrar la escala

Los nombres son una tabla de slices, los semitonos sobre la fundamental
y el nombre:

```rust
pub(crate) const NAMES: &[(&[u8], &str)] = &[
    (IONIAN, "major (Ionian)"),
    (DORIAN, "Dorian"),
    (&[0, 1, 3, 5, 7, 8, 10], "Phrygian"),
    // ...
    (&[0, 1, 4, 5, 7, 8, 10], "Phrygian dominant"),
    // ...
];

pub fn chord_scale_name(scale: PitchSet, root: PitchClass) -> Option<&'static str> {
    let relative = scale.relative_to(root);
    NAMES
        .iter()
        .find(|(steps, _)| PitchSet::from_semitones(steps) == relative)
        .map(|(_, name)| *name)
}
```

`relative_to(root)` gira la escala para que la fundamental del acorde
sea 0. Entonces es el mismo conjunto que una de las filas de la tabla,
o ninguno. `&'static str` dice que el nombre vive tanto como el
programa: es texto escrito en el código, así que la función puede
darlo sin copiarlo. La interfaz en español lo traduce al mostrarlo:
«dórico», «frigia dominante».

### Qué es cada nota

Los tipos de nota son un `enum`, y uno de ellos lleva datos:

```rust
pub enum NoteRole {
    ChordTone(Option<u8>),
    Passing,
    Neighbour,
    ChromaticPassing,
    Approach,
    ScaleTone,
    Outside,
}
```

`ChordTone(Some(3))` es la 3ª; `ChordTone(None)` es una nota de un
conjunto, que no tiene fundamental desde la que contar. `note_roles`
lee una línea y le da un papel a cada nota. Para una nota de la escala
mira a sus vecinas, `before` y `after`, que son `None` al principio de
una frase o después de un silencio:

```rust
match (before, after) {
    (Some(before), Some(after))
        if before == after && is_step(before, pitch, over.scale) =>
    {
        NoteRole::Neighbour
    }
    (Some(before), Some(after))
        if (before < pitch) == (pitch < after)
            && before != pitch
            && pitch != after
            && is_step(before, pitch, over.scale)
            && is_step(pitch, after, over.scale) =>
    {
        NoteRole::Passing
    }
    _ => NoteRole::ScaleTone,
}
```

Hacer `match` sobre una **tupla** de dos `Option` comprueba los dos a
la vez, y el `if` después de un patrón, una **guarda**, añade una
condición. Los brazos se prueban en orden, así que la bordadura (la
misma nota antes y después) se encuentra antes que la nota de paso.
`(before < pitch) == (pitch < after)` compara dos `bool`: es verdadero
cuando la línea sube las dos veces o baja las dos veces, que es lo que
hace pasar a una nota de paso.

`note_roles` lee cualquier línea de la misma manera, la del solista o
una que hayas editado, así que el piano roll puede pintar cada nota del
solista a partir de ella. Su color es un `match` sobre un
`Option<NoteRole>`, también con guardas:

```rust
let mut fill = match role {
    Some(role) if role.is_chord_tone() => NOTE,
    Some(role) if role.is_chromatic() => SOLO_CHROMATIC,
    Some(_) => SOLO_SCALE,
    None if scale.contains(note.pitch.pitch_class()) => NOTE,
    None => NOTE_OUTSIDE,
};
```

Los brazos `None` son lo que recibe cualquier otra parte: verde en la
tonalidad, rojo fuera de ella.

### Dados que siempre sacan lo mismo

Un solo improvisado necesita azar: qué nota del acorde, si saltar, si
empezar tarde. Pero la banda tiene que tocar las mismas notas cada vez
que se abre la canción, o las partes que congelaste ya no coincidirían
con la hoja. Así que el solista tira dados **pseudoaleatorios**: una
función pequeña que convierte un número, la **semilla**, en una
sucesión de números que parecen al azar pero son siempre los mismos a
partir de la misma semilla.

```rust
struct Dice(u64);

impl Dice {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut x = self.0;
        x = (x ^ (x >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        x = (x ^ (x >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        x ^ (x >> 31)
    }

    fn chance(&mut self, percent: u64) -> bool {
        self.next() % 100 < percent
    }
}
```

Es **SplitMix64**, un generador conocido: suma una constante impar
grande, y luego revuelve los bits con desplazamientos, o exclusivos
(`^`) y multiplicaciones. En Rust, un `+` o un `*` corriente que se
desborda detiene una compilación de depuración con un panic, porque un
desbordamiento suele ser un error. Aquí es lo que se busca, así que
`wrapping_add` y `wrapping_mul` lo dicen: el resultado da la vuelta
pasado el `u64` más grande, como espera el algoritmo. Los guiones bajos
de `0x9E37_79B9_7F4A_7C15` son solo para leerlo.

`struct Dice(u64)` es un **struct tupla**: un struct con un solo campo
sin nombre, al que se llega con `self.0`. Envolver el número en un tipo
propio quiere decir que lo único que se puede hacer con él es `next`,
`chance` y `below`.

La semilla se guarda en el proyecto (`Band.solo: Option<u64>`, `None`
sin solista, 1 cuando lo marcas), forma parte de la huella de la banda,
y **Otro solo** le suma uno. Una huella nueva quiere decir que la banda
vuelve a tocar, así que una semilla nueva toca un solo nuevo, y
deshacer, que devuelve la semilla anterior, devuelve el solo anterior.

### Improvisar

`improvise` corta la vuelta en frases de dos compases con `chunks(2)`,
y cada frase en **pulsaciones**: una pulsación es un pulso, o una negra
de un pulso más largo (una blanca en 2/2 tiene dos), así que el
solista se mueve en corcheas sea cual sea el compás. La línea es un
struct con lo que un músico lleva en la cabeza: hacia dónde va y la
última nota que tocó.

```rust
struct Line {
    soloist: Soloist,
    up: bool,
    last: Option<u8>,
    notes: Vec<NoteEvent>,
}
```

`phrase` primero elige una nota del acorde para cada pulsación, y solo
después rellena entre ellas, porque lo que va entre un pulso y el
siguiente depende de dónde cae el siguiente. Empieza decidiendo cuánto
de la frase tocar:

```rust
let late = usize::from(count >= 4 && dice.chance(35));
let rest = match count {
    6.. => 1 + dice.below(2),
    3.. => 1,
    _ => 0,
};
let Some(window) = pulses.get(late..count - rest) else {
    return;
};
```

`usize::from(bool)` convierte `true` en 1 y `false` en 0. `6..` y `3..`
son **patrones de rango**: 6 o más, después 3 o más (los brazos se
prueban en orden). `let ... else` desempaqueta un `Option` o sale:
`get` con un rango da un slice de las pulsaciones, o `None` si el rango
no cabe, y entonces la frase no se toca.

La anacrusa usa una **let chain**: varias condiciones y un `let` unidos
con `&&` en un solo `if`.

```rust
if index == 0
    && pickup
    && pulse.start >= 2
    && let Some(over) = over_at(overs, pulse.start - 2)
{
    let approach = approach(target, over.scale, dice);
    self.play(approach, pulse.start - 2, 2, BETWEEN);
}
```

Se lee como una frase: si es el primer pulso, y salió una anacrusa, y
hay sitio antes, y suena una armonía ahí, toca una nota de aproximación
una corchea antes del pulso. Antes de la edición 2024 de Rust, el `let`
necesitaba un `if` propio dentro del primero.

`fill` elige qué va entre dos notas del acorde: notas de paso cuando
las notas de la escala de en medio caben en el tiempo y cada una está a
un paso de la siguiente, una bordadura cuando las dos notas del acorde
son la misma, el semitono de en medio cuando están a un tono, y, para
un salto demasiado grande para recorrerlo, las notas de la escala justo
antes del destino o un semitono por debajo de él.

### Unirse a la banda

El solista es un `Player` más, después del piano, el bajo, la batería,
la guitarra, las cuerdas y el órgano. Un estilo nunca lo cuenta entre
sus músicos (`Player::Soloist | Player::Horns => false` en
`BandStyle::players`); la banda lo añade cuando hay semilla:

```rust
if self.has_soloist() {
    players.push(Player::Soloist);
}
```

Todo lo que la banda más grande le da a cada músico viene con él: su
propio canal con su programa General MIDI, su pista y su patrón, quedar
congelado cuando se edita, **Tocas tú** para silenciarlo y tocar el
solo tú. Dos cosas son distintas. Las notas del solista pasan por el
fraseo de **melodía** de la biblioteca en lugar del de acompañamiento,
y los golpes, los sostenidos y los silencios no lo tocan.

Una canción con solista necesita al menos el formato 19. Sin solista,
los campos nuevos quedan fuera y el archivo queda como sería sin
ellos.

## Cómo se prueba

En `harmony/chord_scale.rs`, un test por regla:
`chords_of_the_key_play_its_modes`,
`the_dominant_of_a_minor_key_plays_harmonic_minor`,
`borrowed_chords_play_the_parallel_key`,
`secondary_dominants_play_the_scale_of_their_target` y
`other_chords_play_the_scale_of_their_kind`, cada uno comprobando las
notas escritas de una escala, como `D E F G A B C`; y
`every_chord_scale_holds_its_chord`, descrito arriba.

En `band/solo.rs`, `roles_name_what_each_note_does` lee líneas cortas
escritas a mano y comprueba cada papel, y
`a_role_is_described_with_its_chord_and_scale` la frase que muestra el
piano roll. Tres tests tocan muchos solos, sobre varias progresiones y
semillas: `chord_tones_on_the_beats_and_nothing_unexplained_between`
comprueba la regla de las notas del acorde en los pulsos y que ninguna
nota quede sin explicar, `a_solo_is_one_line_in_range_that_breathes`
que las notas nunca se pisan, se quedan a menos de un tono del registro
del solista, callan donde no hay acorde y dejan un silencio de una
corchea o más cada dos compases, y `the_same_seed_plays_the_same_solo`
los dados.

En `band.rs`, `a_soloist_improvises_when_asked`,
`the_soloist_plays_the_chorus_only`,
`over_a_set_the_soloist_plays_its_notes_on_the_beats` y
`a_soloist_is_saved_in_format_nineteen` comprueban el solista como
miembro de la banda.
