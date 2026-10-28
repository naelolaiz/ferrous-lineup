# Un ritmo en otro compás: cambiar la métrica de la canción

Archivos: [`crates/lineup-core/src/remeter.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/remeter.rs),
[`crates/lineup-core/src/library/apply.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/apply.rs)
y [`crates/lineup-app/src/app/library.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/library.rs)

[Un ritmo nuevo para una canción](new-rhythm.md) enseñó a la canción
entera a tocar en otro ritmo: batería nueva, y los acordes y el bajo
tocados como manda el ritmo. Conservaba el compás de la canción a
propósito, porque la melodía no debía moverse. Eso funciona mientras
el ritmo esté en el mismo compás que la canción. Sale mal cuando no lo
está.

Elige «Jazz waltz» (3/4) mientras suena «Oda a la alegría» (4/4). Un
compás de vals dura doce pasos, uno de 4/4 dieciséis. Colocado en
compases de 4/4, el vals toca su «un dos tres» y luego un pulso de
silencio, en cada compás: la batería cambia, pero el compás sigue
diciendo 4/4 y nunca suena a vals. Tú habías pedido que la canción se
volviera un vals.

Ahora lo hace. Un ritmo (o un estilo) en otro compás primero pone la
canción entera en ese compás, compás a compás, y luego la toca con el
ritmo nuevo.

![Oda a la alegría en 4/4: dieciséis pasos por compás.](../images/step-8-remeter-before.png)

![La misma canción después de elegir Jazz waltz: cada compás es ahora de 3/4, y la melodía conserva su lugar en cada compás.](../images/step-8-remeter-after.png)

## Comprimir un compás

La regla es simple: **el compás n de la canción vieja se convierte en
el compás n de la nueva, y cada nota conserva su lugar en su compás**.
Una nota a mitad de un compás de 4/4 cae a mitad del compás de 3/4.
Una nota a un cuarto (pulso 2) cae a un cuarto.

```text
4/4: | F# . . . F# . . . G . . . A . . . |   16 pasos
3/4: | F# . . F# . . G . . A . . |           12 pasos
```

Así que la canción conserva sus compases, sus frases y sus acordes
compás a compás, y el ritmo nuevo encaja exactamente en cada compás.

## Cada compás dura lo mismo que antes

El tempo cuenta negras. Un compás de 3/4 tiene tres, uno de 4/4
cuatro, así que al mismo tempo un compás de 3/4 acaba en tres cuartos
del tiempo y la canción entera se precipitaría. Y con «Use their
tempo» marcado solía ser peor: la canción tomaba el tempo propio del
vals, así que El Choclo pasaba de 112 a 180 y corría el doble de
rápido.

Así que el tempo baja en la misma proporción que el compás: 4/4 a 112
se convierte en 3/4 a 84. Un compás de 3/4 a 84 dura exactamente lo
mismo que un compás de 4/4 a 112 (2.14 segundos), y la canción tarda
exactamente lo mismo que antes. Los cambios de tempo dentro de la
canción (un ritardando al final) se escalan igual. «Use their tempo»
no se aplica cuando el compás cambia: el tempo propio del ritmo
precipitaría o arrastraría la canción.

Eso tiene una consecuencia bonita: **la melodía suena exactamente como
antes**. Cada nota dura lo mismo en segundos y empieza en el mismo
momento. Lo que cambia es cómo se *escribe* y se *cuenta*, y lo que
hacen a su alrededor la batería y el acompañamiento:

- **De 4/4 a 3/4.** Cuatro negras a 112 se convierten en cuatro
  corcheas con puntillo a 84: el mismo sonido, escrito de otra manera.
  Contra los tres pulsos del vals, las cuatro notas de la melodía son
  «cuatro contra tres», una polirritmia. Un compositor que arreglara
  la «Oda a la alegría» como vals preferiría reescribirla, haciendo
  más larga una nota de cada grupo (una blanca y una negra, digamos).
  Esto es un cambio mecánico, no un arreglo.
- **De 4/4 a 6/8.** Este encaja de manera más natural. El 6/8 se
  siente en dos pulsos grandes (dos negras con puntillo), y el 4/4
  también se parte en dos mitades. Los pulsos 1 y 3 caen en los dos
  pulsos del 6/8. Los pulsos 2 y 4 caen a mitad de cada pulso de 6/8:
  dos notas parejas en un pulso de tres corcheas, lo que los músicos
  llaman un *dosillo*.
- **De 3/4 a 4/4.** Tres pulsos estirados sobre cuatro: tres notas
  parejas en un compás de 4/4 son un *tresillo de blancas*. El tempo
  sube un tercio.
- **De 3/4 a 6/8 (o al revés).** Ambos compases son doce pasos, así
  que ninguna nota se mueve y el tempo se queda. Solo cambia el
  compás, y con él dónde se sienten los pulsos: tres pulsos de negra,
  o dos de negra con puntillo. El ritmo nuevo te hace oír la
  diferencia.

Una canción que ya cambia de compás a mitad de camino, como un himno
en 4/4 con una última línea en 3/4, sale en un único compás: cada
compás, fuera lo que fuera, se convierte en un compás del nuevo, y los
marcadores de tiempo se van.

## El cuadro Time lo hace también

Al principio solo la biblioteca cambiaba el compás de la canción.
Elegir 3/4 en el Time de las harmony keys (o en el menú sobre el
compás 1 del playlist) hacía lo de siempre: cambiaba el compás y nada
más. Cada nota se quedaba en su paso, y solo las líneas de compás se
dibujaban en otros sitios.

El primer arreglo hizo que el Time comprimiera cada compás, como hace
un ritmo. Seguía sonando exactamente igual, y vale la pena ver por
qué. La compresión mete cuatro negras en tres pulsos, y el tempo baja
para que el compás dure lo mismo que antes (ver arriba). Las dos cosas
se anulan: cada nota, batería incluida, sigue empezando en el mismo
momento, en segundos. La música solo quedaba *escrita* de otra manera.

Lo que oyes como compás no son las líneas de compás: es dónde caen los
pulsos fuertes y débiles, y en una canción eso es sobre todo la
batería y la manera en que se golpean los acordes. Un vals es un vals
por su «oom-pah-pah». Así que un compás elegido a mano ahora hace lo
que hace un clic en un ritmo de la biblioteca: comprime los compases
*y* la canción toca un ritmo en el compás nuevo, con sus acordes y su
bajo tocados como manda ese ritmo. ¿Qué ritmo? `Rhythm::for_time`
elige, en este orden:

1. Un ritmo del mismo tipo que el que toca la canción, en el compás
   nuevo: El Choclo (una milonga) en 3/4 toca un tango vals, una zamba
   en 3/4 se queda en el folclore sudamericano.
2. El de siempre para ese compás: un vals para 3/4, un six-eight para
   6/8, un slow blues para 12/8, un pop beat para 4/4.
3. Para los compases en los que la biblioteca no tiene ritmo (5/4,
   7/8, 2+2+3/8, cualquier otro), un pulso básico hecho del propio
   compás.

Volver al compás en el que estaba la canción trae de vuelta la canción
como era, con su propia batería incluida: el original sigue siendo la
referencia.

Puedes comprobar que suena distinto sin fiarte de tus oídos. Exporta
El Choclo antes y después, y encuentra el hueco más regular entre los
ataques del sonido: antes, los acentos 3+3+2 de la milonga se repiten
cada 0.805 segundos. Con el primer arreglo, todavía 0.805 segundos.
Ahora, 0.715 segundos: una negra a 84, el pulso de un compás de 3/4.

Una canción sin notas en absoluto no tiene nada que tocar distinto,
así que solo recibe los compases nuevos y conserva su tempo. Un cambio
a mitad de camino, desde el menú sobre cualquier compás que no sea el
primero, sigue cambiando solo los compases de ahí en adelante: así se
escribe un himno con una última línea en 3/4.

### Un pulso básico para cualquier compás

El pulso básico sigue los pulsos del compás. El bombo toca el primer
pulso. Con dos pulsos la caja toca el segundo; con tres, la caja toca
dos pulsos ligeros, como en un vals; con cuatro o más, el bombo y la
caja se turnan. El hi-hat toca cada corchea, un poco más fuerte en los
pulsos. El 5/4 sale como:

```text
Bombo   "X.......x.......x..."
Caja    "....X.......X......."
Hi-hat  "x.o.x.o.x.o.x.o.x.o."
```

Los compases de corcheas que nadie ha agrupado todavía se tocan como
los sienten los bateristas, en doses con un tres al final: 5/8 como
2+3 y 7/8 como 2+2+3, que son tres pulsos, uno fuerte y dos ligeros.

### Rincón de Rust: texto hecho en tiempo de ejecución donde se espera `'static`

Los ritmos de la biblioteca están escritos en el código fuente, así
que un `Rhythm` guarda `&'static str`: texto que vive tanto como el
programa. Un pulso básico se hace mientras la app corre, a partir de
un `String`. `String::leak` convierte un `String` en un `&'static str`
prometiendo no liberarlo nunca, y `Box::leak` y `Vec::leak` hacen lo
mismo con un valor y con una lista. Hacer leak está bien aquí porque
cada compás hace su pulso una sola vez: los pulsos se guardan en un
`static Mutex<Vec<&'static Rhythm>>` y se encuentran ahí la próxima
vez.

`Mutex::new` se puede llamar en un `static` porque es un `const fn`.
Si un hilo hiciera panic mientras sujeta el lock, `lock` devuelve un
error que todavía lleva los datos; `unwrap_or_else(PoisonError::into_inner)`
toma los datos igualmente, ya que una lista de pulsos no puede quedarse
a medio escribir.

## Exacto, entre los pasos

Comprimir dieciséis pasos en doce pone muchas notas entre dos pasos:
las corcheas, a dos pasos de distancia en 4/4, quedan a 1.5 pasos en
3/4. Redondearlas a la cuadrícula las haría desiguales (larga, corta,
larga, corta) y la polirritmia sonaría a error.

Así que la compresión se hace en *ticks*, las 24 partes de un paso que
trajo la importación MIDI ([abrir archivos MIDI](midi-import.md)). 1.5
pasos son 36 ticks, así que las corcheas quedan perfectamente parejas:
una de cada dos empieza 12 ticks después de un paso. Todo el cálculo
es una línea de aritmética de enteros:

```rust
// Where tick `tick` of an old bar goes in the new one, to the nearest
// tick. `length` is the old bar's length in ticks, `new_bar` the new one.
bar.number * new_bar + ((tick - start) * new_bar + length / 2) / length
```

Sumar `length / 2` antes de dividir redondea al tick más cercano en
lugar de siempre hacia abajo, el truco de siempre para redondear con
números enteros. Multiplicar antes de dividir conserva la precisión:
`(tick - start) / length * new_bar` daría 0 para cada tick dentro del
compás.

Esa línea vive en `Remap::tick`. Un `Remap` guarda el
[`Meter`](time-markers.md) viejo de la canción (dónde empieza cada
compás viejo y cuánto dura) y la longitud de un compás nuevo. Todo lo
colocado en la canción le pregunta adónde va: notas, clips, secciones,
el loop, marcadores de tempo y de tonalidad. El glide de un marcador
de tempo se mapea por sus dos extremos, así que un ritardando a lo
largo de dos compases de 4/4 dura dos compases de 3/4.

## Cuando un patrón ya no cabe

Un patrón contiene como mucho 256 pasos. Dieciséis compases de 4/4 son
justo eso, pero dieciséis compases de 12/8 son 384 pasos. En lugar de
soltar notas, el patrón se corta en trozos que caben (diez compases de
12/8, luego seis), llamados «Theme», «Theme (2)» y así, y cada clip
suyo se convierte en clips de los trozos, uno tras otro. La canción
suena igual; solo el playlist muestra un clip más.

## El original sigue siendo la referencia

Cada compresión redondea un poco. Comprime 4/4 a 7/8, luego eso a 5/4,
y luego de vuelta a 4/4, y algunas notas que iban juntas en un tick
pueden acabar a un tick de distancia. Peor: hojear ritmos haría que
cada resultado dependiera del orden en que los probaste.

Así que la app recuerda la canción como era antes de que la biblioteca
la cambiara por primera vez, y **cada ritmo o estilo elegido para la
canción entera vuelve a empezar desde ese original**. 4/4, luego un
vals, luego una zamba en 6/8, luego un pop beat: el pop beat recibe de
vuelta la canción original en 4/4, nota por nota, no una canción en
6/8 estirada de nuevo.

¿Cómo sabe la app que la canción sigue siendo «de la biblioteca»? De
la misma manera en que la biblioteca sabe que un patrón que escribió
sigue sin cambios ([La biblioteca](library.md)): guarda pares de
(canción que hizo la biblioteca, canción desde la que empezó) y
compara el proyecto con ellos. Si cambias una nota, el proyecto no
coincide con ninguno, así que se convierte en el nuevo original: tu
edición se conserva. Undo y redo traen de vuelta versiones que hizo la
biblioteca, que sí coinciden, así que la referencia sobrevive a
Ctrl+Z.

```rust
fn song_original(&self) -> Project {
    self.song_originals
        .iter()
        .rev()
        .find(|(version, _)| *version == self.project)
        .map_or_else(|| self.project.clone(), |(_, original)| original.clone())
}
```

`map_or_else` toma dos closures: una para cuando no hay nada (usar el
propio proyecto) y otra para cuando sí (usar su original). Escribir
`map_or(self.project.clone(), ...)` también funcionaría, pero clonaría
el proyecto cada vez, incluso cuando no hace falta; la closure solo
corre cuando tiene que hacerlo.

Los fills y los estilos dodecafónico y de micromodos se tocan encima
de lo que suena ahora, como antes, y el resultado se recuerda con el
mismo original, de modo que el siguiente ritmo sigue empezando desde
él.

## Mantener tu sitio

Mientras la canción suena, tú estás en algún sitio: compás 13, pulso
2. Después de la compresión, el paso 196 ya no está en el compás 13
(en 3/4 está en el compás 17). Así que la app mueve el cabezal de
reproducción con la canción: construye un `Remap` a partir del compás
de la canción *tal como se reprodujo* (repeticiones incluidas) y envía
el hilo de audio al sitio nuevo. La canción continúa desde el compás
13, pulso 2, en 3/4. Lo mismo pasa con dónde empieza Play y adónde
vuelve Stop, y después de Ctrl+Z y Ctrl+Y, que también cambian el
compás de un lado a otro. Ese caso no usa `Remap` (la canción a la que
deshaces puede tener marcadores de tiempo propios): encuentra en qué
compás está el cabezal de reproducción, y a qué altura de él, y va al
mismo compás y a la misma fracción de él.

## Rincón de Rust

- **`pub(crate)`.** `remeter` reescribe los clips y marcadores del
  playlist directamente, así que los campos `patterns` y `playlist` de
  `Project` pasaron a ser `pub(crate)`: visibles para todos los
  módulos de `lineup-core`, todavía ocultos para la app. Las reglas
  del proyecto (cada clip apunta a un patrón que existe) las mantiene
  el código de dentro del crate.
- **`dedup_by`.** Dos marcadores de tempo a un paso de distancia en
  2/4 pueden quedar comprimidos en el mismo paso. `Vec::dedup_by` quita
  los vecinos que la closure llama iguales; la closure recibe primero
  el elemento posterior y segundo el anterior, y puede cambiar el
  anterior, así que «gana el marcador posterior» es una copia dentro
  de la closure antes de responder `true`.
- **Calcular primero, escribir después.** `remeter` calcula cada
  patrón nuevo a partir del proyecto intacto antes de cambiar nada, y
  luego escribe. Leer y escribir el mismo `Project` a la vez
  necesitaría el permiso del borrow checker para ambas cosas al mismo
  tiempo, que con razón rechaza.

## Pruébalo

1. Abre «Oda a la alegría» desde File > Demos y reprodúcela. En la
   Library, busca «jazz waltz» y haz clic en el ritmo. El Time dice
   3/4, el tempo pasa de 124 a 93, la canción continúa desde el mismo
   compás, y la melodía suena igual que antes: solo cambian la batería
   y los acordes.
2. Haz clic en «Zamba» (6/8), luego en «Pop beat» (4/4). El último
   devuelve exactamente la canción en 4/4. Ctrl+Z retrocede por cada
   uno de ellos.
3. Cambia una nota en el piano roll, luego elige un ritmo en 3/4,
   luego uno en 4/4. Tu nota sigue ahí: tu edición convirtió esa
   versión en el nuevo original.
4. Pausa a mitad de la canción, elige un vals, y luego pulsa Ctrl+Z y
   Ctrl+Y. El cabezal de reproducción se queda en el mismo compás cada
   vez, y el tempo va y viene entre los dos.
5. Abre El Choclo y elige 3/4 en el Time. La barra de estado dice que
   la canción ahora toca Tango vals en 3/4, y oyes tres pulsos por
   compás. Elige 5/4: un pulso básico en cinco. Elige 4/4: El Choclo
   como era.
