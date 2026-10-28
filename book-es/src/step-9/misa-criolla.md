# La Misa Criolla: una misa al compás de las danzas de Argentina

Archivos: [`crates/lineup-core/src/library/song/misa_criolla.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/song/misa_criolla.rs),
[`crates/lineup-core/src/library/song.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/song.rs),
[`crates/lineup-core/src/library/demo.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/demo.rs),
[`crates/lineup-core/src/library/rhythm.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/rhythm.rs)
y [`crates/lineup-core/src/library/style.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/style.rs)

Ariel Ramírez escribió la **Misa Criolla** en 1963 y 1964: una misa,
las cinco partes cantadas del servicio católico, en español, para dos
solistas, un coro y los instrumentos del folclore argentino. Cada parte
está puesta sobre una danza o una canción del país. Las demos tienen
las cinco, completas, cada voz e instrumento en un canal propio (PRs
#154, #156 y #159):

| Demo | Su ritmo | Compás y tonalidad |
|---|---|---|
| **Misa Criolla: Kyrie** | vidala-baguala | 3/4, La menor, el medio en La mayor |
| **Misa Criolla: Gloria** | carnavalito, con un yaraví en el medio | 2/4, La menor; la introducción y el yaraví en 3/4 |
| **Misa Criolla: Credo** | chacarera trunca | 3/4 después de una anacrusa de un pulso; La menor, luego La mayor, luego Mi mayor |
| **Misa Criolla: Sanctus** | carnaval cochabambino | 3/4, La mayor, tres compases de 4/4 |
| **Misa Criolla: Agnus Dei** | estilo pampeano | 3/4, La menor, sin batería |

Las palabras no están en las demos: las voces cantan sin ellas.

## Probarlo

1. Abre **File > Demos**, escribe `Misa` en la caja de búsqueda y elige
   **Misa Criolla: Kyrie**. Pulsa Play.
2. Abre el **mixer**. Lead, Soloist 2, Choir y Low tom tienen una tira
   cada uno. Lead es el primer solista donde cantan los solistas, y la
   voz superior del coro donde no lo hacen; Choir tiene las otras voces.
   Silencia Choir y escucha a los dos solistas solos: casi siempre a una
   tercera de distancia, el segundo debajo del primero.
3. Abre **Misa Criolla: Gloria** y mira el compás en la regla: 3/4 para
   la introducción, 2/4 para la danza, 3/4 otra vez para el yaraví lento
   desde el compás 73, y 2/4 hasta el final.
4. Abre **Misa Criolla: Credo** y mira la tonalidad en la barra de
   armonía: La menor, luego La mayor desde el compás 78 y Mi mayor desde el
   compás 122.
5. En la **Library**, los estilos nuevos están iluminados para sus
   demos: **Carnavalito (2/4)**, **Yaraví**, **Chacarera trunca**,
   **Carnaval cochabambino** y **Estilo**. Prueba uno en una canción
   tuya.

## La música

### Kyrie: un bajo que sube por semitonos

El Kyrie es una **vidala-baguala**, la canción lenta del noroeste andino
de Argentina, cantada con un tambor de mano. Aquí el bombo toca el
primer y el tercer pulso de cada compás de 3/4 y nada más.

El coro canta solo primero, suavemente, seis compases que son una
pequeña lección de armonía. Sigue el bajo en el piano roll del canal
Choir: La, Si, Do, Do sostenido, Re, Re sostenido, Mi. Sube de la tónica a la
dominante, por semitonos desde Do. Encima:

| Compás | Bajo | Acorde | Qué hace |
|---|---|---|---|
| 3 | La, luego Si | La menor, luego Mi mayor sobre Si | i, y un V6/4 de paso |
| 4 | Do, luego Do sostenido | La menor sobre Do, luego La7 sobre Do sostenido | i6, luego V7 de Re menor: una **dominante secundaria** |
| 5 | Re | Re menor | iv, adonde llevaba el La7 |
| 6 | Re sostenido | Fa, La, Do y Re sostenido | una **sexta aumentada alemana**: Fa y Re sostenido, a una sexta aumentada de distancia, se abren hacia las dos Mi del compás siguiente |
| 7 | Mi | La menor sobre Mi | la **cuarta y sexta cadencial**, i6/4, que retrasa la dominante |
| 8 | Mi | Mi, La y Re | V7 con una **cuarta suspendida**: La en lugar de Sol sostenido |
| 9 | Mi | Mi, Sol sostenido y Re | V7: la suspensión resuelve, La baja a Sol sostenido |

Luego cantan los solistas. La parte del medio, el **Christe**, pasa a La
mayor, la **tónica mayor**: la misma tónica, con Do sostenido, Fa
sostenido y Sol sostenido en lugar de Do, Fa y Sol. El Kyrie vuelve en La
menor, y su último acorde es otra vez La mayor, bajo una fermata: una
pieza menor que termina en el acorde mayor de su tónica termina en una
**tercera de Picardía**.

### Gloria: la danza de carnaval, y un lamento en el medio

El **carnavalito** es la danza rápida de carnaval de la Quebrada de
Humahuaca, en el extremo norte de Argentina. Está en 2/4: dos pulsos por
compás. El bombo golpea cada pulso y lo responde con dos semicorcheas
rápidas, y las guitarras rasguean **La menor y Mi mayor, un pulso cada
una**: i y V, tónica y dominante, cada compás. Luego la armonía se mueve
a Do mayor, la **relativa mayor** (la tonalidad mayor con las mismas
notas que La menor), a través de Fa, Sol y Do: IV, V y I de Do, o bVI, bVII y
bIII contados desde La.

La Library ya tenía un carnavalito, escrito como pares de compases de
2/4 en un compás de 4/4. **Carnavalito (2/4)** conserva los compases
como los escribe el Gloria, para que una canción en 2/4 pueda usarlo.

En el medio el tempo cae de una negra a 88 a 40, y el compás pasa a 3/4:
un **yaraví**, el lamento lento de los Andes. El bajo sostiene un La
durante compases enteros, un **pedal**, el bombo toca corcheas suaves, y
las voces cantan encima en La menor, casi siempre sobre las notas de la
escala **pentatónica menor** (La, Do, Re, Mi y Sol).

### Credo: una chacarera cortada

La **chacarera** es la danza de Santiago del Estero. El Credo es una
**chacarera trunca**, «cortada», en un 3/4 rápido, una negra a 152.
Escucha dónde se detienen las frases cantadas: en el segundo compás de
cada par, las voces sueltan antes del tercer pulso, mientras el bombo
sigue. El patrón del bombo dura dos compases: su parche en los dos
primeros pulsos de un compás y en el tercero del siguiente, su aro
respondiendo con cuatro semicorcheas. El ritmo nuevo **Chacarera
trunca** es ese patrón.

La introducción oscila entre La menor y **Re mayor**. El Fa sostenido de Re
mayor no está en La menor: La menor con Fa sostenido es **La dórico**, el
modo menor con la sexta elevada, el modo de mucha música de los Andes.

Las voces empiezan en La menor, pasan a **La mayor** en el compás 78 y a
**Mi mayor** en el compás 122. El final se ralentiza de 152 a una negra a
46, oscilando entre Mi mayor y La mayor sobre un Mi en el bajo, y cierra en
Mi mayor. Oído desde La, donde empezó el Credo, Mi es la dominante: el
movimiento termina abierto.

### Sanctus: el carnaval de Cochabamba

El **carnaval cochabambino** viene de Cochabamba, en Bolivia. El Sanctus
está en La mayor y en un 3/4 rápido. Las guitarras y el clavecín abren
con el acorde de La mayor, una flauta toca encima de ellas, y la batería
entra de una en una: una caja china (tocada aquí en el aro), una caja
con un estallido de cuatro semicorcheas fuertes en el segundo pulso, y
dos tambores respondiéndose. Tres compases de **4/4** ensanchan la
música hasta un acorde sostenido; luego las voces cantan solas,
suavemente y luego fuerte, ralentizándose, y la batería vuelve para los
últimos compases.

### Agnus Dei: la guitarra del gaucho

El **estilo** es la canción lenta de las Pampas, cantada con la
guitarra, sin batería. La guitarra abre en La menor, luego toca tres
acordes a un pulso cada uno: **Re menor, La menor y Mi séptima**, iv, i y
V7. Esa es toda la armonía de la tonalidad menor en un compás: la
**subdominante**, la **tónica** y la **dominante**. Una segunda guitarra
toca encima una línea descendente, con giros, y el bajo baja caminando:
La, Re, Do, Si.

Entre las estrofas la guitarra corre en semicorcheas, en tiempo libre:
escucha cómo cada compás se acelera hacia su mitad y se ralentiza al
final. Las marcas de tempo de la demo lo hacen, dos glides en cada
compás, como lo toca la secuencia de la que viene. Los últimos compases
son muy suaves, y el coro cierra de Re menor a La menor: iv a i, un
**cierre plagal**, el final de «amén».

## Cómo funciona

### Líneas: los demás instrumentos, escritos

Hasta ahora una parte de demo tenía una melodía, un cuadro de acordes, a
veces una mano izquierda para las Keys, y rounds para las voces extra.
La Misa Criolla tiene solistas, un coro de cuatro voces, guitarras, un
clavecín y un bajo a la vez. El PR #154 le da a un [`Part`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/song.rs)
una lista de **líneas**: la parte de cada instrumento escrita, en un
canal propio.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Line {
    pub name: &'static str,
    pub program: u8,
    pub notes: &'static str,
}

pub const fn line(name: &'static str, program: u8, notes: &'static str) -> Line {
    Line { name, program, notes }
}
```

Una línea tiene el nombre de su canal, el **program** de General MIDI
que la toca, y sus notas en la misma notación de texto que una melodía,
donde `&` empieza otra voz en el mismo canal. El alto, el tenor y el
bajo del coro son una línea con dos `&`. Las líneas con el mismo nombre,
en cualquier parte, tocan en el mismo canal.

`&'static str` es un string que vive tanto como el programa: texto
escrito en el código fuente. `const fn` es una función que Rust puede
ejecutar mientras compila, así que una canción entera puede ser una
constante:

```rust
Part::new("Kyrie", "-:8 !mf E4:4 | E4:6 D4:2 C4:4 | ...", "")
    .with_lines(&[
        line("Soloist 2", gm::VOICE_OOHS, "-:8 !mf C4:4 | C4:6 B3:2 A3:4 | ..."),
        line("Choir", gm::VOICE_OOHS, "!p E4:8 -:4 | ... & ... & ..."),
    ])
```

`with_lines` también es un `const fn`. Copia la parte y cambia un campo,
con la **sintaxis de actualización de struct** de Rust:
`Part { lines, ..self }` significa «el campo `lines` es este, los demás
campos como en `self`».

Cuando se construye una demo ([`song_demo`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/demo.rs)),
cada nombre de línea recibe un canal, con el program de su primera
línea, y las líneas de cada parte se interpretan como los demás
acompañamientos y siguen las dinámicas de la partitura. Una parte con
líneas está **escrita**: el comping y los vientos del estilo se callan
en ella, porque la partitura ya dice lo que toca cada uno.

Una canción cuyas partes están todas escritas, sin mano izquierda, nunca
toca acordes en las Keys, así que no recibe ningún canal Keys:

```rust
let keys_play = song.parts.iter().any(|part| {
    !part.left.is_empty() || (part.lines.is_empty() && !part.chords.trim().is_empty())
});
let keys = keys_play
    .then(|| apply::synth_named(&mut project, apply::KEYS))
    .flatten();
```

`any` pregunta si alguna parte necesita las Keys. `bool::then` convierte
`true` en el `Some(...)` de lo que devuelve la closure y `false` en
`None`; `synth_named` devuelve a su vez un `Option`, así que `flatten`
convierte el `Option<Option<usize>>` en un `Option<usize>`.

### De una secuencia a una demo

Los cinco movimientos vienen de una secuencia MIDI de toda la misa, una
pista por cada voz e instrumento. Un script la leyó, compás a compás, y
escribió las partes:

- cada nota colocada en la semicorchea más cercana, o en el tercio de
  una más cercano donde la secuencia toca un tresillo, ya que los
  tresillos se escriben exactamente en una secuencia;
- el mapa de tempo leído como las marcas de la partitura: una caída
  profunda que vuelve es una **fermata**, una racha de cambios pequeños
  un **glide** (*rit.* o *accel.*), y cualquier otra cosa un tempo
  nuevo;
- las velocidades leídas como dinámicas, de *pp* a *ff*;
- un **trémolo**, el mismo acorde golpeado más rápido que una
  semicorchea, escrito como un acorde sostenido, porque la rejilla de la
  aplicación se detiene en la semicorchea.

Las palabras se dejaron fuera a propósito.

### Dos cosas que una demo debe conservar

Dos pruebas dieron forma a las partes.

**Una exportación MIDI debe volver nota por nota.** El importador corta
una canción larga en patrones en las líneas de compás, de 8 a 16
compases de largo, eligiendo una línea de compás que ninguna nota cruza.
Una nota que cruza el corte se corta en dos y se vuelve a tocar. En el
Credo el bajo liga por encima de casi cada línea de compás, así que
entre los compases 21 y 37 no había una línea de compás limpia, el
importador cortó una nota sostenida y el archivo volvió con una nota de
más. El arreglo estaba en la demo: un límite de parte en el compás 30,
donde entran los solistas. Una demo vuelve a tocar las notas sostenidas
en sus propios límites de parte, así que esas líneas de compás siempre
están limpias.

**Una demo conserva un tempo sensato**: nunca más lento que tres décimas
de su tempo inicial. La secuencia sostiene el último acorde del Credo a
una negra de 20, un octavo de 152; la demo lo sostiene a 46.

## Comprobarlo

Cada movimiento se reprodujo con la [traza de reproducción](playback-trace.md)
y se comparó con la secuencia, nota por nota, en pasos:

```text
ferrous-lineup --trace "Misa Criolla: Credo" --out credo.tsv
```

La traza enumera cada nota que el reproductor empieza: su paso, altura,
canal y program. Las cinco coinciden con la secuencia excepto donde la
demo difiere a propósito: los trémolos son acordes sostenidos, los
unísonos cantados por dos voces del coro suenan una vez (son un canal),
y una línea de bajo que la secuencia duplica en dos pistas se toca una
vez.

## Ejercicios

1. En el Kyrie, silencia todos los canales menos Choir y toca los
   compases 3 a 9. Canta o toca el bajo solo, de La a Mi. Luego toca los
   acordes de la tabla de arriba en el chord sheet, uno por compás, y
   escucha cómo la sexta alemana se abre hacia el Mi.
2. Aplica **Estilo** a una canción tuya en una tonalidad menor, en 3/4.
   Luego escribe un compás de tres acordes a un pulso cada uno, iv, i y
   V7, como en el Agnus Dei.
3. Encuentra los compases del Credo donde las voces se detienen antes
   del tercer pulso. ¿Cuántos compases dura cada frase?
4. Rust: escribe un `const fn` que tome un `Line` y lo devuelva con otro
   program, usando la sintaxis de actualización de struct.
