# Acordes prestados, dominantes secundarias y acordes pivote

Archivos: [`crates/lineup-core/src/chord_sheet/function.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/chord_sheet/function.rs),
[`crates/lineup-core/src/harmony/scale.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/harmony/scale.rs)
y [`crates/lineup-app/src/chord_sheet.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/chord_sheet.rs)

Un número romano dice dónde se encuentra un acorde en la tonalidad. En
Do mayor, Rem es ii y Sol7 es V7. La mayoría de los acordes de una canción
son propios de la tonalidad, construidos solo con sus siete notas. Los
que no lo son suelen ser los que recuerdas: el iv menor que oscurece la
última línea de una balada, el Re7 que lleva al puente de un estándar.
La armonía tiene nombres para explicar por qué están ahí esos acordes,
y el chord sheet los escribe ahora debajo de los acordes:

- **Acordes prestados**, tomados de la tonalidad menor sobre la misma
  tónica.
- **Dominantes secundarias**, que hacen que otro acorde de la tonalidad
  suene por un momento como su hogar.
- **El acorde napolitano**, un acorde mayor sobre el segundo grado
  rebajado.
- **Acordes pivote**, donde la música cambia de tonalidad a través de
  un acorde que pertenece a ambas tonalidades.

![Dieciséis compases en Do mayor que van a Sol mayor. Fam y La♭ están marcados como «prestados» en azul, Mi7 y Re7/Fa♯ son dominantes secundarias en amarillo (V7/vi, V⁶₅/V), Re♭/Fa es la sexta napolitana en rosa (N⁶) y Lam en el compás 14 es el acorde pivote, «vi = ii», en naranja. El cambio de tonalidad a Sol mayor está marcado en el compás 15.](../images/step-9-chord-functions.png)

## Cómo probarlo

1. Crea una canción nueva, abre el chord sheet (**F5**) y escribe estos
   dieciséis compases, un acorde en cada uno: Do, Fam, La♭, Sol7, Do, Mi7, Lam,
   Re7/Fa♯, Sol, Do, Re♭/Fa, Sol, Do, Lam, Re7, Sol.
2. Haz clic derecho en el compás 15 de los números de compás del
   playlist y elige la tonalidad **Sol mayor** de ahí en adelante. El
   chord sheet escribe «→ Sol mayor» en el compás 15.
3. Señala un número romano coloreado para leer qué hace el acorde.
   Señala «→ Sol mayor» para leer cómo cambia la tonalidad.

Los números romanos se colorean según lo que hace el acorde: **azul**
para un acorde prestado, **amarillo** para una dominante secundaria,
**rosa** para el napolitano y **naranja**, el color de los cambios de
tonalidad, para un acorde pivote. Los acordes propios de la tonalidad
quedan en gris.

## Acordes prestados: mixtura modal

Do mayor y Do menor comparten su tónica, Do, y su dominante, Sol. Se llaman
**tonalidades homónimas** (en inglés, *parallel keys*). La música en una
tonalidad mayor suele tomar un acorde de su menor homónima sin salir de la tonalidad. Esto se llama
**préstamo**, o **mixtura modal**: los dos modos se mezclan.

| Acorde | En Do mayor | Sus notas | De Do menor |
|---|---|---|---|
| Fam | iv | Fa La♭ Do | La♭ |
| La♭ | ♭VI | La♭ Do Mi♭ | La♭ y Mi♭ |
| Si♭ | ♭VII | Si♭ Re Fa | Si♭ |
| Mi♭ | ♭III | Mi♭ Sol Si♭ | Mi♭ y Si♭ |
| Re° | ii° | Re Fa La♭ | La♭ |
| Bdim7 | vii°7 | Si Re Fa La♭ | La♭ |

El ♭ de ♭VI dice que el acorde se construye sobre un sexto grado
rebajado: La♭, donde Do mayor tiene La. El tooltip nombra las notas que
vienen de la tonalidad menor:

> Fam: Fa menor, iv en Do mayor. Un acorde prestado: Do mayor lo toma de Do
> menor, la tonalidad menor sobre la misma tónica (esto se llama
> mixtura modal). Su La♭ viene de Do menor, no de Do mayor: oscurece la
> tonalidad mayor por un momento sin salir de ella.

El préstamo también va en la otra dirección. Una pieza en La menor que
termina en La mayor toma prestado su último acorde de La mayor. Ese final
tiene nombre propio, la **tercera de Picardía**, y el tooltip lo dice
cuando una pieza menor termina así.

Bdim7 está en la lista porque una tonalidad mayor también toma
prestado de la menor **armónica**, la escala menor con la séptima
elevada. En Do menor esa séptima elevada es Si, la sensible, y Bdim7
(Si Re Fa La♭) es el acorde de sensible de Do menor.

## Dominantes secundarias: V de V

En Do mayor, Sol7 es V7, la dominante. Su tercera, Si, es la sensible, un
semitono por debajo de Do, y tira hacia arriba, hacia Do. Ese tirón es lo
que hace que Do suene como hogar.

Cualquier acorde mayor o menor de la tonalidad puede tener su propio
dominante. Re7 es la dominante de Sol: su tercera, Fa♯, está un semitono
por debajo de Sol y tira hacia arriba, hacia él. Fa♯ no está en Do mayor,
así que por un momento Sol suena como una tónica, el hogar de una
tonalidad propia. Esto se llama **tonicización**. Re7 en Do mayor se
escribe **V7/V**, «cinco siete de cinco»: el V7 del acorde que es V
en Do.

![Señalando a Re7/Fa♯: «Una dominante secundaria: Re7/Fa♯ es la dominante de Sol, el V de Do mayor, como si Sol fuera una tónica. Su tercera, Fa♯, es la sensible de Sol, un semitono por debajo, y tira hacia arriba, hacia ella, así que Sol suena por un momento como una tonalidad propia (una tonicización). Aquí sigue hacia Sol, como una dominante que va a su tónica».](../images/step-9-chord-functions-secondary.png)

Las cifras de una inversión van sobre el V. Re7 con Fa♯ en el bajo es
V⁶₅/V, y el Fa♯ del bajo sube directo a Sol.

Todo acorde mayor y menor de la tonalidad puede ser tonicizado, salvo
la tónica, que no necesita ayuda, y vii°, que es disminuido y no puede
ser tónica. En Do mayor:

| Acorde | Su dominante | Escrito | Su sensible |
|---|---|---|---|
| Rem (ii) | La7 | V7/ii | Do♯ |
| Mim (iii) | Si7 | V7/iii | Re♯ |
| Fa (IV) | Do7 | V7/IV | Mi |
| Sol (V) | Re7 | V7/V | Fa♯ |
| Lam (vi) | Mi7 | V7/vi | Sol♯ |

Do7 es el caso raro. Su sensible, Mi, ya está en Do mayor. Lo que hace que
Do7 suene como una dominante es su séptima, Si♭, que el tooltip nombra:
«Su Si♭ no está en Do mayor».

Una dominante secundaria no siempre va adonde apunta. Mi7 que va a Fa en
lugar de Lam es un movimiento **deceptivo**, como V que va a vi. El
acorde sigue siendo V7/vi, y el tooltip añade: «Aquí va a Fa en lugar
de Lam».

También puede ir ahí pasando por un **acorde de sexta y cuarta**: una
tríada con su quinta en el bajo, como Do/Sol. Re7 que va a Do/Sol y luego a Sol7
es el camino de libro hacia una cadencia. El bajo ya ha llegado a Sol, y
el Do y el Mi por encima caen a Si y Re un momento después. Esta es la
**sexta y cuarta cadencial**. El tooltip dice que el bajo ha llegado y
que el acorde de Sol llega cuando las dos notas por encima caen un grado.

### Acordes de sensible secundarios

Un acorde disminuido también puede hacer el trabajo de una dominante. En
Do mayor, Sidim es vii°, construido sobre la sensible, y tira hacia Do
como hace V. Fa♯dim7 se construye sobre Fa♯, la sensible de Sol, así que
tira hacia Sol. Se escribe **vii°7/V**.

A diferencia de una dominante secundaria, solo se nombra así cuando va a
su acorde, o a un acorde de sexta y cuarta sobre él. Autumn Leaves
tiene Fa♯m7♭5 que va a Si7 y luego a Mim, un ii–V–i en Mi menor. Leído en Do
mayor, como en la página sobre [pegar chord sheets](text-chord-sheets.md),
Fa♯m7♭5 sigue siendo allí el ii de Mi menor, no el vii de Sol. Así que el
chord sheet mantiene su número llano, ♯ivø7.

### En una tonalidad menor

Una tonalidad menor tiene tres escalas, y la música en menor toma sus
acordes de todas ellas. De la menor **natural**, La Si Do Re Mi Fa Sol: Do
(III), Rem (iv), Fa (VI) y Sol (VII). De la menor **armónica**, con Sol♯: la
dominante Mi o Mi7 (V) y Sol♯dim7 (vii°7). De la menor **melódica**, con
Fa♯ y Sol♯: Sim (ii) y Re (IV), normalmente de camino hacia Mi. Todos son
acordes propios de la tonalidad, sea cual sea la escala a la que esté
ajustada. Así que El Choclo, en La menor armónica, tiene su Do y su Sol7
leídos como III y VII7, no como dominantes secundarias de Fa y Do. Un
acorde secundario trae una nota que ninguna de las tres tiene: en La
menor, Do♯, Re♯ o La♯. La única excepción es un acorde de la menor
melódica que va adonde iría un acorde secundario: Re que sigue hacia Sol
es el V de Sol.

En un **modo**, como Re dórico o Mi frigio, el chord sheet escribe solo
los números. Los acordes prestados y las dominantes secundarias son
nombres de las tonalidades mayores y menores, y los acordes de un modo
hacen otras cosas: Mi mayor en Mi frigio es el acorde de tónica de gran
parte del flamenco, no la dominante de La menor.

El chord sheet lee las dominantes secundarias contra la escala menor
natural. La dominante de la propia dominante es Si7, con Re♯, escrito V7/V.
Cuando Si7 va a Mim, un v menor, se escribe V7/v.

Un acorde mayor de tónica en menor es dos cosas a la vez. La7 que va a
Rem es V7/iv, la dominante de la subdominante. La mayor al final de la
pieza es la tercera de Picardía, tomado en préstamo de La mayor.

## La sexta napolitana

El acorde **napolitano** es un acorde mayor sobre el segundo grado
rebajado: Re♭ mayor en Do, donde la tonalidad tiene Re. Lleva el nombre de
los compositores de ópera napolitanos del siglo XVIII, que lo usaban a
menudo. Se toca casi siempre con su tercera en el bajo, Re♭/Fa, así que
se lo conoce como **sexta napolitana**, escrita N⁶ (un acorde con su
tercera en el bajo es un acorde de «sexta» en el bajo cifrado). Va
hacia la dominante, y su Re♭ cae a Si, la sensible. Esa caída es una
tercera disminuida, un paso melódico llamativo.

El chord sheet escribe N⁶ solo para un acorde mayor escrito sobre la
segunda letra de la tonalidad, Re♭ en Do. Do♯ mayor son las mismas notas
en el piano, pero escrito sobre Do es un acorde cromático sobre la
tónica, no un napolitano.

## Acordes pivote: cambiar de tonalidad

Una **modulación** es un paso a una tonalidad nueva en la que la música
se queda. El camino más suave pasa por un acorde que las dos
tonalidades comparten. En Do mayor, Lam es vi. En Sol mayor, Lam es ii.
Cuando la música hace Do, Lam, Re7, Sol, el oído oye Lam como vi en Do. Luego
llega Re7, que Do mayor no tiene, y el oído oye que Lam era en realidad ii
en Sol. Lam es el **acorde pivote**: hace girar la música de una tonalidad
a la otra.

El chord sheet busca el pivote alrededor de cada cambio de tonalidad.
Encuentra el primer acorde cerca del cambio que pertenece a la
tonalidad nueva y no a la vieja. Ese acorde es la **llegada** (aquí
Re7). El acorde anterior es el pivote si pertenece a ambas tonalidades.
El pivote lleva los dos números, primero el de la tonalidad vieja:
`vi = ii · pivot`.

![Señalando al acorde pivote: «Lam: La menor, vi en Do mayor. El acorde pivote de la modulación a Sol mayor: Lam es vi en Do mayor y ii en Sol mayor, un acorde de ambas tonalidades. El oído lo oye en Do mayor; Re7, que Do mayor no tiene, viene a continuación y muestra que era el ii de Sol mayor».](../images/step-9-chord-functions-pivot.png)

Dónde va el marcador de tonalidad es cosa tuya. Los músicos marcan la
tonalidad nueva donde sienten que ha llegado, que puede ser un compás o
dos después del pivote. Si el marcador está sobre el Sol, el Re7 anterior
ya pertenece a Sol mayor. El chord sheet lo lee igualmente en Sol mayor y
escribe debajo `V7 · in G major`, en naranja.

No todo cambio de tonalidad tiene pivote:

- **Una modulación directa** va directa a un acorde de la tonalidad
  nueva desde uno que no está en ella, como hace una canción cuando
  sube un semitono para su último estribillo. El tooltip del cambio de
  tonalidad dice «una modulación directa, sin acorde pivote».
- **Un cambio de modo** mantiene la tónica y cambia el modo, de Do mayor
  a Do menor. No hace falta ningún acorde que lleve hasta allí.

Un acorde prestado también puede ser pivote. Fam es iv tomado en
préstamo de Do menor, y es vi en La♭ mayor. En Do, Fa, Fam, Mi♭7, La♭, Fam es
el pivote `iv = vi`, y Mi♭7 es la llegada.

## En el código

### Leer toda la hoja de una vez

El número de un acorde solía depender solo del acorde y de la
tonalidad. Ahora también depende de sus vecinos: un acorde de sensible
secundario se nombra por adónde va, y un pivote se encuentra mirando
los acordes alrededor de un cambio de tonalidad. Así que
`Functions::read` lee todo el chord sheet de una pasada, y la hoja le
pregunta por cada acorde:

```rust
pub struct Function {
    pub start: usize,
    pub symbol: ChordSymbol,
    pub key: Scale,
    pub role: Role,
    pub next: Option<ChordSymbol>,
    pub last: bool,
    pub early: bool,
    pub pivot: Option<Modulation>,
}
```

`next` es el siguiente acorde escrito que no es el mismo acorde otra
vez: Do en un compás y Do en el siguiente no han avanzado.

### Un enum con datos en sus variantes

Lo que hace un acorde es un `Role`:

```rust
pub enum Role {
    Plain,
    Borrowed { from: Scale },
    SecondaryDominant { of: Chord },
    SecondaryLeadingTone { of: Chord },
    Neapolitan,
}
```

Cada variante lleva exactamente lo que necesita. Un acorde prestado
conoce la tonalidad de la que viene, un acorde secundario conoce el
acorde al que lleva, y el napolitano no necesita nada más. Un `match`
saca los datos donde se usan, de modo que `of` solo existe en los
brazos donde significa algo:

```rust
match self.role {
    Role::SecondaryDominant { of } => Some(with_figures(
        &format!("V{}/{}", chord.quality.numeral_suffix(), self.target_numeral(of)?),
        figures,
    )),
    Role::Neapolitan => Some(with_figures("N", figures)),
    Role::Plain | Role::Borrowed { .. } => figured(self.key, self.symbol),
    // ...
}
```

`Role::Borrowed { .. }` encaja con un acorde prestado venga de la
tonalidad que venga: `..` ignora los campos.

### Qué papel va primero

Primero, `role` despacha los acordes que no necesitan más nombre que su
número: todo acorde en un modo, un acorde de la tonalidad y, en una
tonalidad menor, un acorde de su menor natural o armónica:

```rust
let major_or_minor = key.kind == ScaleKind::Major || key.is_minor_key();
if !has_chords(key)
    || !major_or_minor
    || key.is_chord_of_key(chord)
    || in_minor(key, chord, &[NaturalMinor, HarmonicMinor])
{
    return Role::Plain;
}
```

`in_minor` construye las escalas menores sobre la tónica de la
tonalidad y pregunta si el acorde pertenece a uno de los tipos que
recibe.

Algunos acordes podrían ser entonces dos cosas. En La menor, La mayor es
a la vez la dominante de Rem y el acorde mayor de tónica tomado en
préstamo de La mayor. Re mayor es el IV de la menor melódica, y el
dominante de Sol. `role` decide por adónde va el acorde:

```rust
let goes_to = |of: Chord| next.is_some_and(|next| arrives(next, of));
let dominant = key.secondary_dominant_target(chord);
let leading = key.secondary_leading_tone_target(chord);
let secondary = match (dominant, leading) {
    (Some(of), _) => Some(Role::SecondaryDominant { of }),
    (_, Some(of)) if goes_to(of) => Some(Role::SecondaryLeadingTone { of }),
    _ => None,
};
if let Some(of) = dominant.or(leading)
    && goes_to(of)
{
    return secondary.unwrap_or(Role::Plain);
}
if in_minor(key, chord, &[MelodicMinor]) {
    return Role::Plain;
}
if let Some(from) = borrowed_from(key, symbol) {
    return Role::Borrowed { from };
}
if is_neapolitan(key, symbol) {
    return Role::Neapolitan;
}
secondary.unwrap_or(Role::Plain)
```

Un acorde que va al acorde al que lleva es un acorde secundario.
Después, un acorde de la menor melódica es propio de la tonalidad.
Luego viene el préstamo, y después el napolitano. En último lugar queda
una dominante secundaria que va a otro sitio. `match (dominant, leading)`
encaja con una tupla de dos `Option`, y la guarda `if goes_to(of)`
añade una condición a un patrón.

`is_some_and` pregunta qué hay dentro de un `Option` y responde `false`
para `None`, así que el último acorde, sin acorde siguiente, no va a
ninguna parte.

`borrowed_from` comprueba la escritura además del sonido. Sol aumentado
(Sol Si Re♯) tiene las mismas tres teclas del piano que Mi♭ aumentado
(Mi♭ Sol Si), un acorde de Do menor armónica. Pero su Re♯ sube a Mi, donde un
Mi♭ caería: no es una nota de Do menor. Así que cada nota de un acorde
prestado debe estar escrita sobre la letra que le dan las escalas de la
tonalidad, contando hacia arriba desde la tónica.

### Encontrar el pivote

`modulation` mira los acordes desde el cambio de tonalidad anterior
hasta el siguiente. Dos closures dicen qué es cada acorde para las dos
tonalidades:

```rust
let new_only = |chord: &Function| {
    let chord = chord.symbol.chord();
    belongs(to, chord) && !heard_in(from, chord)
};
let common = |chord: &Function| {
    let chord = chord.symbol.chord();
    belongs(to, chord) && heard_in(from, chord)
};
```

`belongs` significa un acorde de la tonalidad, en una tonalidad menor
de cualquiera de sus tres escalas. `heard_in` cuenta también como
propios los acordes prestados de la tonalidad vieja. Por eso Fam,
prestado en Do mayor, puede ser un pivote hacia La♭ mayor. La llegada es
el primer acorde `new_only`. Se busca primero justo antes del marcador,
por si la tonalidad nueva llegó pronto, y después de él. El pivote es
el acorde anterior a la llegada, si es `common`:

```rust
change.pivot = arrival
    .checked_sub(1)
    .map(|before| range[before])
    .filter(|before| common(before))
    .map(|before| (before.start, before.symbol));
```

`checked_sub(1)` da `None` cuando la llegada es el primer acorde, así
que un cambio que empieza en su llegada no tiene pivote, y ningún
índice baja nunca de cero.

## ¿Cómo sabemos que está bien?

Las pruebas de `function.rs` son pequeños chord sheets al estilo de un
libro de texto de armonía. Cada una comprueba los números y lo que
dicen los tooltips:

- Do Lam La♭ Fam Sol7 Do: ♭VI e iv están prestados de Do menor.
- Do Mi7 Lam Re7/Fa♯ Sol7 Do La7 Rem: V7/vi, V⁶₅/V y V7/ii, cada uno yendo a su
  acorde.
- Mi7 que va a Fa: sigue siendo V7/vi, y el tooltip dice que va a Fa en
  lugar de Lam.
- Do7 que va a Fa: V7/IV, cuya Si♭ se nombra.
- Fa♯dim7, Sol♯dim y Do♯m7♭5: vii°7/V, vii°/vi y viiø7/ii.
- Autumn Leaves en Do: Fa♯m7♭5 que va a Si7 mantiene ♯ivø7.
- Re7 y Fa♯dim7 que van a Do/Sol: la sexta y cuarta cadencial.
- Re♭/Fa en Do menor es N⁶, y Do♯ en Do mayor no es un napolitano.
- Si7 en La menor es V7/V, o V7/v yendo a Mim.
- La7 en La menor yendo a Rem es V7/iv, y La mayor al final es una tercera
  de Picardía.
- Lam Do Sol7 Do Fa Rem Mi7 Lam en La menor, ajustada como menor natural o
  armónica: todos los acordes son propios de la tonalidad. Re es IV
  yendo a Mi7 y V de Sol yendo a Sol. El tooltip de Si7 nombra solo Re♯ como
  venido de fuera de La menor.
- Mi mayor en Mi frigio no tiene nombre secundario: los acordes prestados
  y secundarios se nombran solo en tonalidades mayores y menores.
- Sol aumentado en Do mayor no es prestado: su Re♯ no es el Mi♭ de Do menor.
  Bdim7, cuya La♭ sí lo es, sí lo es.
- Do, Lam, Re7, Sol hacia Sol mayor con el marcador sobre el Lam, el Re7 o el Sol:
  Lam es el pivote en cada caso.
- Fam como pivote prestado hacia La♭ mayor, una modulación directa a Re♭
  mayor y un cambio de modo de Do mayor a Do menor.

## Ejercicios

1. Escribe `C Fm C` y luego `C F C`. Escucha los dos. La única nota que
   cambia es La a La♭. ¿Qué le hace al ambiente? Encuentra una canción
   que conozcas que haga esto.
2. En Sol mayor, ¿qué acorde es V7/V? ¿Qué nota suya no está en Sol mayor,
   y adónde quiere ir esa nota? Escríbelo en el chord sheet y
   compruébalo.
3. Escribe una modulación de Do mayor a Fa mayor pasando por un pivote.
   ¿Qué acordes comparten las dos tonalidades? ¿Qué acorde muestra que
   Fa mayor ha llegado? (Pista: ¿qué nota tiene Fa mayor que Do mayor no
   tiene?)
4. `is_neapolitan` comprueba la escritura del acorde con
   `key.tonic_name().letter.steps_up_to(symbol.root.letter) == 1`.
   Explica por qué Do♯ mayor en Do falla esa prueba mientras que Re♭ mayor
   la pasa, aunque ambas son las mismas teclas del piano.
5. El chord sheet no nombra los acordes de **sexta aumentada**
   (italiano, francés y alemán), que también van a la dominante. Busca
   sus notas en Do menor. ¿Cuál de ellos suena como un acorde de
   séptima de dominante sobre La♭, y qué en su escritura los distingue?
