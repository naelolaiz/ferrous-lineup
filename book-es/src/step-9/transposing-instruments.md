# Lead sheets para instrumentos en Si♭, Mi♭ y Fa

Archivos: [`crates/lineup-core/src/lead_sheet/transpose.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/lead_sheet/transpose.rs),
[`crates/lineup-core/src/lead_sheet/score.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/lead_sheet/score.rs),
[`crates/lineup-core/src/lead_sheet/layout.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/lead_sheet/layout.rs)
y, en la app, [`crates/lineup-app/src/lead_sheet.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/lead_sheet.rs)

Pon un [lead sheet](lead-sheet.md) de una canción en Do delante de un
trompetista y un pianista, pídeles que toquen juntos la primera nota, y
no se pondrán de acuerdo. El pianista toca Do; la trompeta, leyendo el
mismo Do, suena el Si♭ un tono entero más grave. La trompeta es un
**instrumento transpositor**: las notas que lee no son las notas que
suena. Para tocar con la banda, el trompetista necesita una hoja
escrita un tono entero más agudo, en Re.

Este capítulo añade esa hoja. La ventana Lead sheet gana una elección
**For**, y la misma canción se puede escribir para instrumentos en Si♭,
Mi♭ y Fa, cada uno en la tonalidad que sus músicos leen.

## Por qué algunos instrumentos leen en otras tonalidades

Los instrumentos de viento vienen en familias de distintos tamaños: los
saxofones del soprano al barítono, los clarinetes, las trompas. Cada
tamaño suena más grave o más agudo, pero los dedos son los mismos: en
todos los saxofones, los mismos dedos tocan la nota escrita Do. Si la
música se escribiera como suena, un músico que cambia del alto al tenor
tendría que aprender digitaciones nuevas para cada nota. Así que la
música se escribe para los dedos, y cada instrumento lleva el nombre de
la nota que suena cuando toca un Do escrito: una trompeta **en Si♭**, un
saxofón alto **en Mi♭**, una trompa **en Fa**.

La música escrita como suena está en **afinación de concierto**, o «en
Do». El piano, la guitarra, el bajo, la voz, la flauta, el violín y el
trombón la leen todos.

| Instrumentos | Un Do escrito suena | La parte se escribe |
|---|---|---|
| Piano, guitarra, bajo, voz, flauta, violín, trombón | Do | como suena |
| Trompeta, corneta, fiscorno, clarinete, saxofón soprano | el Si♭ de abajo | una segunda mayor más aguda |
| Saxofón tenor | el Si♭ una octava y una segunda abajo | una novena mayor más aguda |
| Saxofón alto | el Mi♭ de abajo | una sexta mayor más aguda |
| Saxofón barítono | el Mi♭ una octava y una sexta abajo | una octava y una sexta más aguda |
| Trompa, corno inglés | el Fa de abajo | una quinta justa más aguda |

La parte se escribe siempre más aguda de lo que suena, y dentro de una
familia con el mismo intervalo más o menos una octava, así que una
canción en Do se escribe en Re para todo instrumento en Si♭, en La para
todo instrumento en Mi♭ y en Sol para todo instrumento en Fa.

## Cómo usarlo

Abre **View > Lead sheet**. Junto a **Clef** está ahora **For**:

![La barra de herramientas de la ventana Lead sheet con el menú For abierto: Concert pitch, B♭ instruments, E♭ instruments y F instruments.](../images/step-9-transposing-for-menu.png)

- **Concert pitch** escribe la canción como suena, como antes.
- **B♭ instruments**: trompeta, clarinete, saxofones soprano y tenor.
- **Mi♭ instruments**: saxofones alto y barítono.
- **Fa instruments**: trompa y corno inglés.

Pasa el ratón sobre una elección para ver para qué instrumentos es y
cuánto más aguda está escrita. La hoja cambia al momento: la armadura,
la melodía y los símbolos de acorde se mueven juntos, y la esquina
superior izquierda de la primera página dice para quién es la parte,
«B♭ instruments», donde las partes impresas nombran su instrumento.
**Export PDF…** ofrece el nombre del archivo con los instrumentos
dentro, `I Got Rhythm (Bb instruments).pdf`, para que las partes de una
misma canción no se pisen unas a otras. La elección se recuerda, como
el papel y el zoom: un saxofonista que abre otra canción se la
encuentra escrita para él otra vez.

Aquí está la demo I Got Rhythm para instrumentos en Si♭. La canción está
en Si♭ mayor, dos bemoles; la parte está en Do mayor, sin sostenidos ni
bemoles en absoluto. Esa es una razón por la que tantas canciones de
jazz están en tonalidades con bemoles: el Si♭ de concierto es Do para la
trompeta y el saxofón tenor, y el Mi♭ de concierto es Do para el saxofón
alto.

![El lead sheet de I Got Rhythm para instrumentos en B♭: «B♭ instruments» arriba a la izquierda, sin armadura, los acordes Am, E7, Am, Dm, luego Am6, F7, Am, F, Am6, Fm6.](../images/step-9-transposing-b-flat.png)

La demo empieza con una introducción sobre acordes de Sol menor (Solm, Re7,
Dom). En la parte en Si♭ son Lam, Mi7 y Rem; en la parte en Mi♭, en Sol mayor,
Mim, Si7 y Lam; en la parte en Fa, en Fa mayor, Rem, La7 y Solm.

La canción en sí, su archivo y lo que toca la banda se quedan como
están. Solo la hoja se escribe de otra manera.

## La tonalidad: nunca más de seis sostenidos o bemoles

Mover la tonalidad es sumar semitonos: dos para los instrumentos en Si♭
(de Si♭ hasta Do), nueve para los instrumentos en Mi♭ (de Mi♭ hasta Do) y
siete para los instrumentos en Fa (de Fa hasta Do).

```rust
pub fn semitones(self) -> u8 {
    self.note().pitch_class().semitones_up_to(PitchClass::C)
}

pub fn key(self, key: Scale) -> Scale {
    Scale::new(key.root.transpose(i32::from(self.semitones())), key.kind)
}
```

`note()` es la nota que suena un Do escrito, Si♭ para
`Transposition::BFlat`, así que los nombres de los instrumentos y sus
distancias salen de un mismo sitio. El tipo de tonalidad se mantiene: Fa
menor es Sol menor para una trompeta, Re dórico es Si dórico para un
saxofón alto.

La tonalidad nueva es una `Scale` como cualquier otra, así que se
nombra como la app nombra toda tonalidad ([¿Qué escritura usa una
tonalidad?](../step-3/theory.md#qué-ortografía-usa-una-tonalidad)): cada
una de las doce tonalidades tiene un nombre con como mucho seis
sostenidos o bemoles, y es ese el que se usa. Esto importa para las
partes, porque contar hacia arriba por letras daría a veces tonalidades
en las que nadie escribe:

| Canción | Parte en Si♭ | Parte en Mi♭ | Parte en Fa |
|---|---|---|---|
| Do | Re | La | Sol |
| Fa | Sol | Re | Do |
| Si♭ | Do | Sol | Fa |
| Mi♭ | Fa | Do | Si♭ |
| La♭ | Si♭ | Fa | Mi♭ |
| Re♭ | Mi♭ | Si♭ | La♭ |
| Sol | La | Mi | Re |
| Re | Mi | Si | La |
| La | Si | Fa♯ | Mi |
| Mi | Fa♯ | Re♭ | Si |
| Si | Re♭ | La♭ | Fa♯ |
| Fa♯ | La♭ | Mi♭ | Re♭ |

Una canción en Mi para un saxofón alto, una sexta mayor arriba, estaría
estrictamente en Do♯ mayor, siete sostenidos. Re♭ mayor tiene las mismas
notas con cinco bemoles, y es lo que escribe un copista. Una canción en
Fa♯ para una trompeta estaría en Sol♯ mayor, que necesita un Fa doble
sostenido en su armadura; La♭ mayor, cuatro bemoles, tiene las mismas
notas. Una prueba recorre las doce tonalidades mayores y menores para
cada instrumento y comprueba que ninguna parte tiene más de seis.

## Los acordes: movidos con letras y todo

Un acorde debe conservar su lugar en la tonalidad: el V7 de la canción
es el V7 de la parte. [Nombres de acordes](chord-names.md) mostró cómo
se mueve un chart a otra tonalidad, contando letras además de
semitonos, y el chord sheet ya lo usa cuando la canción cambia de
tonalidad (`HarmonyName::moved`). La parte usa la misma función, de la
tonalidad de la canción a la de la parte:

```rust
pub fn harmony(self, harmony: HarmonyName, key: Scale) -> HarmonyName {
    harmony.moved(key, self.key(key))
}
```

Como va de tónica a tónica, sigue también la escritura de la tonalidad.
De Do a Re toda letra sube una, así que Rem7 Sol7 Domaj7 se vuelve Mim7 La7
Remaj7. De Fa♯ a La♭ toda letra sube dos, así que el V7 de Fa♯, Do♯7, se
vuelve Mi♭7, el V7 de La♭, y nunca Re♯7. La demo de bossa nova Garota de
Ipanema está en Re♭ en la app; para un saxofón alto, en Si♭, su Re♭maj7
Mi♭7 Mi♭m7 Re7 son Si♭maj7 Do7 Dom7 Si7.

## Las notas: cada intervalo conserva su nombre

La melodía se mueve los mismos semitonos, pero sus notas necesitan
letras también, y aquí un atajo saldría mal. La manera obvia es mover
cada nota hacia arriba y luego volver a escribirla en la tonalidad
nueva con las reglas del [lead sheet](lead-sheet.md#qué-letra-spellingrs).
El Turkish March de Mozart, en La menor, muestra el problema. Su melodía
se apoya en Mi desde el semitono de abajo, Re♯–Mi. En Si menor, la
tonalidad de la trompeta, los mismos sonidos se reescribirían como
Fa–Fa♯: las reglas dejan llana una tecla blanca del piano fuera de la
tonalidad, así que escriben Fa en lugar de Mi♯. Pero Re♯–Mi es una segunda
menor, una nota que se apoya en la siguiente, y Fa–Fa♯ es un unísono
aumentado, una letra doblada hacia arriba. Un músico lee una línea
distinta.

Así que las notas se escriben primero en afinación de concierto, donde
las reglas conocen la tonalidad y los acordes en los que se escribió la
canción, y luego cada nombre se mueve las mismas letras y semitonos que
la tonalidad:

```rust
pub fn name(self, name: NoteName, key: Scale) -> NoteName {
    let written = self.key(key);
    if written == key {
        return name;
    }
    // ... (a row or a set has no letters: its notes take their usual names)
    let (from, to) = (key.spell(key.root), written.spell(written.root));
    let moved = name.above(
        from.pitch_class().semitones_up_to(to.pitch_class()),
        from.letter.steps_up_to(to.letter),
    );
    if moved.accidental.abs() >= 2 && written.spell(moved.pitch_class()) != moved {
        moved.simplified()
    } else {
        moved
    }
}
```

`NoteName::above` es la misma función que mueve las fundamentales de
los acordes, así que las notas y los acordes se mueven exactamente las
mismas letras. El Re♯–Mi del Turkish March es Mi♯–Fa♯ para una trompeta,
Si♯–Do♯ para un saxofón alto y La♯–Si para una trompa: una segunda menor en
cada caso, como lo escribió Mozart.

Las últimas líneas se ocupan de las tonalidades que se renombraron. Una
canción en Mi tiene una sexta rebajada, Do. Para un saxofón alto, en Re♭
en lugar de Do♯, las letras se mueven de Mi a Re, y Do se vuelve Si doble
bemol (Si♭♭), la sexta rebajada de Re♭ escrita estrictamente. Nadie
quiere leer eso, y es la misma nota que La, así que el nombre se
simplifica, como se hace con los símbolos de acorde. Un doble sostenido
o doble bemol que la tonalidad nueva sí tiene se queda: la sensible de
Do♯ menor armónica es Si♯, y en la parte de trompeta, en Re♯ menor
armónica, es Do doble sostenido (Do♯#), la sensible de esa tonalidad.

Para comprobar todo esto, la melodía de cada demo se escribió para cada
instrumento y se comparó con la hoja de concierto: la altura de cada
nota era correcta, y cada nombre se movía con las letras de la
tonalidad salvo los pocos que necesitaban el nombre simplificado. Las
hojas de concierto de las 343 demos están exactamente como estaban
antes de este cambio.

## La octava: dónde se sienta la melodía en el pentagrama

El intervalo dice qué notas escribir, no en qué octava. Un saxofón
tenor suena una novena por debajo de sus notas escritas, una octava más
grave que una trompeta, y sin embargo en los libros de lead sheets para
instrumentos en Si♭ ambos leen la misma página: el tenor simplemente la
suena una octava más grave. Lo que importa en la página es que la
melodía se asiente en el pentagrama, no en una escalera de líneas
adicionales por encima.

Así que la parte se escribe según el intervalo del instrumento, movida
por octavas enteras hasta que la nota central de la melodía queda lo
más cerca posible de la línea central del pentagrama (Si4 en la clave de
sol):

```rust
pub fn shift(self, pitches: &[Note], clef: Clef) -> i32 {
    let interval = i32::from(self.semitones());
    // ... (concert pitch is written as it sounds)
    let middle = i32::from(clef.middle_line().midi());
    let median = i32::from(median.midi());
    (-4..=4)
        .map(|octaves| interval + 12 * octaves)
        .min_by_key(|shift| ((median + shift - middle).abs(), (shift - interval).abs()))
        .unwrap_or(interval)
}
```

La nota central es la **mediana**, la que queda en medio una vez
ordenadas las notas, como para elegir la clave: unas pocas notas muy
agudas o muy graves no la arrastran. Para una melodía en el registro de
una voz el resultado es el propio intervalo del instrumento: una
segunda mayor arriba para los instrumentos en Si♭, una sexta mayor
arriba para los instrumentos en Mi♭, una quinta arriba para los
instrumentos en Fa, de modo que trompeta, saxofón alto y trompa suenan
la melodía donde está. El Turkish March sube hasta Do6, y para los
instrumentos en Si♭ sale una séptima menor *más grave*, las mismas
letras una octava abajo, donde una trompeta puede tocarla.

`min_by_key` se queda con el candidato cuya clave de ordenación es más
pequeña, y la clave de ordenación aquí es un par. Los pares se comparan
como palabras de un diccionario: por el primer elemento, y por el
segundo solo cuando los primeros son iguales. Así que decide la
distancia a la línea central, y cuando dos octavas quedan igual de
cerca, gana la más cercana al propio intervalo del instrumento.

Las partes para instrumentos transpositores usan la clave de sol, que
todos ellos leen; **Clef** puede elegir aún la clave de fa, y entonces
la melodía se centra en la línea central de ese pentagrama, Re3.

## Qué cambió y dónde

El lead sheet ya estaba dividido en pasos (mira [De las notas a la
notación](lead-sheet.md#de-las-notas-a-la-notación)). Un paso es nuevo, y
solo dos de los viejos necesitaban saber de los instrumentos:

- `transpose.rs` es nuevo: el tipo `Transposition` con `key`,
  `harmony`, `name` y `shift`.
- `score.rs` le pide la tonalidad de cada compás, los acordes, los
  nombres de las notas y la octava. Dos closures lo dejan corto:

  ```rust
  let key_at = |step: usize| transposition.key(keys.key_at(step));
  let harmony_of =
      |marker: ChordMarker| transposition.harmony(marker.harmony, keys.key_at(marker.start));
  ```

  Un **closure** es una función pequeña escrita en el sitio. Estos dos
  toman prestados `keys` y `transposition` de la función que los rodea,
  así que cada llamada solo dice qué paso o qué marcador.
- `layout.rs` escribe «B♭ instruments» arriba a la izquierda de la
  primera página, con el bemol de la fuente musical como en los
  símbolos de acorde. Cuando el título es demasiado ancho para dejar
  sitio a su lado, las palabras pasan a una línea debajo.
- `melody.rs` y `rhythm.rs` no cambiaron, y `spelling.rs` solo aprendió
  dónde está la línea central de cada clave: la melodía se lee, se
  escribe en afinación de concierto, se redondea y se corta en valores
  de nota de la misma manera para cada instrumento.

En la app, `Transposition` se guarda con el papel y el zoom en los
ajustes de la ventana:

```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
struct Settings {
    paper: Paper,
    zoom: f32,
    transposition: Transposition,
}
```

`#[serde(default)]` es lo que deja que un campo nuevo llegue sin romper
nada: los ajustes guardados por una versión anterior no tienen
`transposition`, y serde lo rellena desde `Settings::default()`,
afinación de concierto. El propio `Transposition` recibe su valor por
defecto de un atributo sobre una variante:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum Transposition {
    #[default]
    Concert,
    BFlat,
    EFlat,
    F,
}
```

El archivo de proyecto no cambia: quién lee la hoja es una elección del
lector, no parte de la canción.

## Cómo se probó

Trece pruebas nuevas en `lineup-core` y una en la app:

- el intervalo de cada instrumento, y las tonalidades de una canción en
  Do (Re, La, Sol), en tonalidades menores y modos;
- ninguna tonalidad de parte con más de seis sostenidos o bemoles, para
  las doce tonalidades mayores y menores y cada instrumento; Mi es Re♭
  para un saxofón alto, Fa♯ es La♭ para una trompeta;
- los acordes conservan su lugar: ii–V–I en Do es Mim7 La7 Remaj7, Do♯7 en
  Fa♯ es Mi♭7 en La♭, un poliacorde y N.Do. también se escriben;
- las notas conservan sus intervalos: Re♯–Mi en La menor se vuelve Mi♯–Fa♯,
  Si♯–Do♯ y La♯–Si; la sexta rebajada de Mi es La en Re♭; Do♯# se queda en Re♯
  menor armónica;
- la octava para una melodía alrededor de Sol4, una melodía aguda y una
  línea de bajo, en ambas claves; la afinación de concierto no cambia
  nada;
- hojas enteras: una parte de trompeta con su armadura de Re mayor y sin
  sostenidos de más, un cambio de tonalidad escrito en las tonalidades
  de las partes (La mayor a Re♭ mayor para un saxofón alto), la clave de
  sol para las partes, y las palabras arriba de la página, al lado o
  debajo del título;
- el nombre del archivo del PDF, `Op. 24 (Bb instruments).pdf`.

Luego se miraron las partes en la app para I Got Rhythm, el Turkish
March, con sus muchos cambios de tonalidad, y Garota de Ipanema.

![El Turkish March para instrumentos en Mi♭: «Mi♭ instruments» arriba a la izquierda, Fa♯ menor con tres sostenidos, Do♯ menor con cuatro desde el compás 4, Fa♯ menor otra vez desde el compás 6, y los acordes Fa♯m, Do♯m, Sol♯ y Do♯.](../images/step-9-transposing-key-changes.png)

## Ejercicios

1. **La octava exacta.** Añade una elección que escriba cada parte
   exactamente como la toca el instrumento: una novena arriba para el
   saxofón tenor, una octava y una sexta arriba para el barítono. ¿Qué
   función cambia? Pruébalo con una melodía alrededor de Do5 para el
   tenor: ¿cuántas líneas adicionales necesita, y por qué no hacen esto
   los libros de lead sheets?
2. **El clarinete en Mi♭** suena una tercera menor *más aguda* de lo que
   lee, así que su parte se escribe una tercera menor más grave. ¿Es
   eso un `Transposition` nuevo, o las mismas notas que la parte del
   saxofón alto en otra octava? ¿Qué tendría que saber `shift` para
   distinguirlas?
3. **Instrumentos en Re y Sol.** Una trompeta en Re suena un tono entero
   más agudo de lo escrito, una flauta alta en Sol una cuarta más grave.
   Añádelos. ¿Qué devuelven `note()` y `semitones()`, y qué más de la
   ventana y la etiqueta tiene que saber de ellos?
4. **Un chord sheet transpuesto como texto.** [Chord sheets como
   texto](text-chord-sheets.md) escribe los acordes para pegar. Dale la
   misma elección de instrumentos. ¿Qué única función hace el trabajo
   para cada acorde?
5. **La tonalidad de concierto en la parte.** Algunas partes imprimen
   la tonalidad propia de la canción debajo del instrumento, «Concert
   key: Si♭». ¿Dónde iría en `layout.rs`, y qué necesita saber `lay_out`
   que no sabe ahora?
