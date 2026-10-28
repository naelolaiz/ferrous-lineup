# Cómo se mueve cada parte de acorde a acorde

Archivos: [`crates/lineup-core/src/harmony/voice_motion.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/harmony/voice_motion.rs)
y [`crates/lineup-app/src/piano_roll.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/piano_roll.rs)

La [comprobación de conducción de voces](voice-leading.md) marca lo que la
escritura de partes clásica evita. Es la pluma roja del profesor. No
dice nada cuando las partes se mueven bien, y la buena escritura de
partes es casi todo lo que hay que aprender: cada parte moviéndose lo
menos posible, las partes externas yendo en direcciones opuestas, la
sensible subiendo y la séptima bajando. Ahora el piano roll muestra
cada movimiento, bueno o no. Marca **Voice leading**, apunta a un
cambio de acorde, y cada parte se nombra con adónde va.

![Los dieciséis compases del ejemplo de funciones de acordes en el piano roll, cuatro partes en una pista Piano. Las notas de cada parte se unen con una línea de su propio color, y los movimientos del compás 5, donde Sol7 va a Do, van en negrita. La línea sobre los nombres de los acordes dice «Piano: superior Re5→Do5 · 2.ª F4→Mi4 · 3.ª Si3→Sol3 · inferior Sol2→Do3; partes externas en movimiento contrario».](../images/step-9-part-motion.png)

El ejemplo son los dieciséis compases de [Acordes prestados,
dominantes secundarias y acordes
pivote](chord-functions.md), escritos como cuatro partes en una pista
Piano: un bajo en la mano izquierda y tres partes en la derecha. La
comprobación no encuentra nada que marcar: «Voice leading, 4 partes:
nada de lo que la escritura de partes clásica evita».

## Lo que ves

- **Una línea de color para cada parte**, de cada una de sus notas a
  la siguiente, sobre las notas. El ojo puede seguir una parte por
  todo el patrón, como en una partitura. Una nota mantenida no tiene
  línea: la parte se queda donde está.
- **Los movimientos del cambio al que apuntas**, dibujados más
  gruesos. Apuntar a cualquier parte de un acorde elige el cambio que
  llevó a él.
- **Una línea de movimientos** sobre los nombres de los acordes.
  Nombra cada parte de arriba abajo, con adónde va: «superior Re5→Do5 ·
  2.ª F4→Mi4 · 3.ª Si3→Sol3 · inferior Sol2→Do3». En una sola pista, las
  partes se nombran por lugar. Con varias pistas, se nombran por
  pista. Luego nombra el movimiento entre las partes externas.
- **Su tooltip**, que dice cada movimiento con palabras. Mueve el
  puntero desde las notas hacia arriba, a la línea: la línea mantiene
  el cambio al que apuntaste por última vez mientras lo lees.

![El tooltip de la línea de movimientos en el cambio de Sol7 a Do: «Piano (voz superior): baja una segunda mayor de Re5 a Do5, un grado. Piano (voz 2 de 4): baja una segunda menor de F4 a Mi4, un grado. Piano (voz 3 de 4): baja una tercera mayor de Si3 a Sol3, un salto. Piano (voz inferior): sube una cuarta justa de Sol2 a Do3, un salto grande». Luego el movimiento entre cada par de partes contiguas, y entre las partes externas: movimiento contrario.](../images/step-9-part-motion-tooltip.png)

Donde la comprobación encuentra algo en el paso al que apuntas, la
línea dice eso en su lugar, en rojo, como en la página de
[conducción de voces](voice-leading.md).

## Grado conjunto, salto y salto grande

Cuánto se mueve una parte tiene un nombre:

| Movimiento | Intervalo | Semitonos | Ejemplo |
|---|---|---|---|
| **Mantenida** | la misma nota | 0 | Sol3 a Sol3 |
| **Grado conjunto** | una segunda | 1 o 2 | F4 a Mi4 |
| **Salto** | una tercera | 3 o 4 | Si3 a Sol3 |
| **Salto grande** | una cuarta o más | 5 o más | Sol2 a Do3 |

Las partes internas de un coral se mueven sobre todo por grado o
mantienen sus notas. El bajo salta más, porque se mueve de
fundamental a fundamental: Sol2 subiendo una cuarta a Do3 en una cadencia
es uno de los movimientos de bajo más comunes que hay.

El tooltip escribe el intervalo en la tonalidad. Si3 a Sol3 es una
tercera mayor, no «cuatro semitonos», y Fa♯ a Sol es una segunda
menor, no «un semitono». Una nota tocada de nuevo se muestra como
mantenida en la línea. El tooltip dice «vuelve a tocar Do5», para que
puedas distinguirla de una nota mantenida.

## Movimiento entre dos partes

Dos partes que se mueven a la vez van juntas de una de cuatro maneras:

| Movimiento | Las dos partes | Ejemplo |
|---|---|---|
| **Paralelo** | se mueven igual y mantienen el mismo intervalo | terceras que van a terceras |
| **Similar** | se mueven igual, a otro intervalo | una sexta que va a una quinta, ambas hacia abajo |
| **Contrario** | se mueven en direcciones opuestas | el bajo hacia arriba, la voz superior hacia abajo |
| **Oblicuo** | una se mueve, la otra mantiene | una nota mantenida bajo una que se mueve |

La escritura de partes prefiere el movimiento **contrario** y el
**oblicuo** entre las partes externas, la voz superior y el bajo,
porque el oído sigue esas dos más que ninguna. Moverse en direcciones
opuestas las mantiene como dos líneas. Las terceras y sextas paralelas
están bien en cualquier parte, y suenan dulces. Las quintas y octavas
paralelas son lo que la comprobación de conducción de voces marca, porque
hacen que dos partes suenen como una.

«El mismo intervalo» significa el mismo **número** de intervalo tal
como la tonalidad lo escribe, no el mismo número de semitonos. En el
Sol7 a Do de la imagen, la voz superior va de Re5 a Do5 y la segunda de F4
a Mi4. Re sobre Fa es una sexta mayor y Do sobre Mi es una sexta menor:
una sexta en ambos casos, así que eso es movimiento paralelo, sextas
paralelas. Un profesor de armonía lo cuenta igual.

## Lo que muestra el ejemplo

En el cambio de Sol7 a Do (compás 5):

- La **séptima** de Sol7, F4, baja un grado a Mi4.
- La **sensible**, Si3, está en una parte interna, y baja una tercera a
  Sol3 en lugar de subir a Do. Una sensible interna puede hacer esto en
  V7 a I: entonces el acorde de Do recibe su quinta, Sol, y tiene las
  tres notas. La comprobación de conducción de voces deja en paz a una
  sensible interna por esta razón.
- El **bajo** salta una cuarta de fundamental a fundamental, y la voz
  **superior** baja un grado: las partes externas están en movimiento
  contrario.

Apunta a otros cambios y compara. De La♭ a Sol7 (compás 4), el Mi♭ de la
contralto sube a Fa en lugar de bajar a Re. Abajo, haría quintas
paralelas con el bajo, La♭ a Mi♭ y luego Sol a Re. De Re♭/Fa a Sol
(compás 12), la voz superior baja de Re♭5 a Si4, la tercera disminuida
por la que se conoce la sexta napolitana.

## En el código

### Siguiendo las partes por cada cambio

La comprobación de conducción de voces ya divide cada pista en partes
(`Voice`s). `voice_motion::changes` recorre los momentos en los que
una nota nueva empieza en cualquier parte. Para cada parte toma la
nota que tenía justo antes y la nota que tiene ahí:

```rust
fn before(voice: &Voice, tick: usize) -> Option<NoteEvent> {
    let index = voice
        .notes
        .partition_point(|note| note.start_tick() < tick)
        .checked_sub(1)?;
    let note = voice.notes[index];
    (tick < note.end_tick() + LINE_BREAK).then_some(note)
}
```

Las notas de una parte están ordenadas por comienzo, así que
`partition_point` encuentra con una búsqueda binaria cuántas empiezan
antes de `tick`. La anterior a ese recuento es la última nota de la
parte. `LINE_BREAK` es una negra: tras un silencio tan largo la línea
ha terminado, y la nota siguiente empieza una nueva.
`bool::then_some` convierte la condición en un `Option`, `Some(note)`
cuando se cumple y `None` cuando no.

### Nombrar el movimiento

```rust
pub fn motion(upper: &Move, lower: &Move, key: Scale) -> Motion {
    let (a, b) = (upper.semitones(), lower.semitones());
    match (a.signum(), b.signum()) {
        (0, 0) => Motion::Static,
        (0, _) | (_, 0) => Motion::Oblique,
        (x, y) if x != y => Motion::Contrary,
        _ => {
            let number = |upper: Note, lower: Note| {
                let (upper, lower) = (
                    key.spell(upper.pitch_class()),
                    key.spell(lower.pitch_class()),
                );
                lower.letter.steps_up_to(upper.letter)
            };
            if number(upper.from.pitch, lower.from.pitch) == number(upper.to.pitch, lower.to.pitch)
            {
                Motion::Parallel
            } else {
                Motion::Similar
            }
        }
    }
}
```

`signum` es −1, 0 o 1: abajo, mantenida o arriba. Hacer match sobre el
par de signos resuelve primero los casos fáciles. Ambas mantenidas.
Una mantenida (el `|` de un patrón significa «o»). Signos opuestos.
Solo cuando ambas van en la misma dirección importa el intervalo.
Entonces el closure `number` escribe ambas notas en la tonalidad y
cuenta letras de la inferior a la superior: Re sobre Fa es
`F.steps_up_to(D)`, 5 letras arriba, una sexta. Contar letras, no
semitonos, es lo que hace que una sexta mayor que va a una sexta menor
sea paralela.

### Mantener la línea mientras la lees

La línea muestra el cambio en el paso bajo el puntero. Pero para leer
su tooltip, el puntero tiene que dejar las notas por la línea, y
entonces no hay ningún paso bajo él. Así que el piano roll recuerda el
último paso al que se apuntó, y la línea lo usa mientras el puntero
está sobre la propia línea:

```rust
let line = Rect::from_min_size(
    ui.cursor().min,
    vec2(ui.available_width(), ui.text_style_height(&TextStyle::Body)),
);
let step = self
    .hovered_step
    .or(drawing.playhead)
    .or(self.last_step.filter(|_| ui.rect_contains_pointer(line)));
```

`ui.cursor()` es donde egui pondrá el siguiente widget, así que `line`
es el rectángulo que la línea está a punto de ocupar, calculado antes
de dibujarla. `Option::filter` conserva `last_step` solo cuando el
closure dice que sí: cuando el puntero está dentro de ese rectángulo.

## Ejercicios

1. Escribe Do a Fa a Sol7 a Do en cuatro partes, con el bajo Do3 F2 Sol2
   Do3. Mantén cada parte interna moviéndose por grado o manteniendo.
   Mira la línea de movimientos en cada cambio: ¿están las partes
   externas alguna vez en movimiento paralelo o similar?
2. En el ejemplo, mueve el F4 de la contralto en el compás 4 a Re4, de
   modo que Sol7 sea Sol2 Si3 Re4 Re5. ¿Qué dice la línea de movimientos en
   el compás 4, y qué encuentra la comprobación de conducción de voces?
3. `Move::size` llama grado a uno o dos semitonos. Una segunda
   aumentada, La♭ a Si, son tres semitonos, pero su nombre dice
   «segunda». ¿Qué tamaño le da el código, y cuál le daría un profesor
   de armonía? Cambia `size` para que cuente letras, como hace
   `motion`, y escribe una prueba.
