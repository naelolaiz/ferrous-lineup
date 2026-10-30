# Ejercicios: las canciones como datos

Estos ejercicios practican esta parte con las canciones de verdad: las
constantes de `library/song/`, la demostración en que se convierten, y
la música de la que se leyeron. Cada uno dice qué **ya existe**, para
que lo extiendas en lugar de construirlo otra vez. La mayoría son cosas
para abrir en la aplicación y escuchar, o para resolver con lápiz;
algunos son trozos de Rust con un final claro; los que se llaman
«Piénsalo» son preguntas, sin código que escribir.

Haz los de Rust en una rama propia. Ejecuta `cargo test` después de
cada cambio, y `cargo fmt --all --check` y
`cargo clippy --workspace --all-targets -- -D warnings` antes de dar
uno por terminado: el CI ejecuta los tres.

## Las canciones como constantes

### 1. Una canción nueva en el archivo de su género (lineup-core, media)

Agrega una melodía de dominio público a las demostraciones. `Song`,
`Part`, `SONGS` y los archivos de género ya existen; agregas una
entrada, no una función (ver [Una canción es una
constante](demos.md#una-canción-es-una-constante)).

1. Lee dos canciones del archivo de género que elijas, como `folk.rs`
   o `blues.rs`, y copia la forma de una, cada campo de `name` a
   `about`.
2. Escribe la melodía con `|` entre compases y un cifrado debajo, y
   dale a la parte un nombre como «Verse» o «Tune».
3. Hazla `pub(super) const`, y agrégala a `SONGS` en `song.rs`, al final
   del bloque de su género.
4. Deja que los tests encuentren los errores: un compás que no suma, un
   acorde que no se puede leer, un estilo en otro compás, una nota que
   no está en la tonalidad.

```sh
cargo test -p lineup-core library
cargo run --release -- --demos
```

Está terminado cuando los tests pasan y la canción está en **Archivo >
Demostraciones > Canciones**, en su género, se abre en la hoja de
acordes y suena.

*Practica:* literales de structs, datos `const`, leer los fallos de los
tests.

### 2. Extender una canción que ya está (lineup-core, media)

**Shenandoah** toca su canción dos veces después de una anacrusa, y
**Danny Boy** una estrofa y un estribillo. Elige una y agrégale una
parte que la demostración todavía no toca: una introducción solo para
la banda, una última estrofa o una coda. Escríbela compás por compás,
dale un cifrado y agrégala a la `form` de la canción. Después abre la
demostración y mira la hoja de acordes: ¿muestra los acordes que
escribiste, en los compases que querías?

*Practica:* la notación de las melodías, `form` como una lista de
nombres.

### 3. Un último estribillo un semitono más arriba (lineup-core, fácil)

Muchas canciones pop suben su último estribillo un semitono. Agrega una
copia del estribillo de una canción con `.in_key(...)` un semitono por
encima de la tonalidad de la canción, y ponla al final de la `form`.
¿Qué números romanos del cifrado hay que cambiar? (Ninguno: se leen en
la tonalidad de la parte.) Escucha el final: el último compás de la
demostración está en el acorde de tónica de la tonalidad nueva, como
comprueba `a_song_that_ends_after_a_modulation_ends_in_the_new_key`.

*Practica:* métodos constructores sobre un valor `const`.

### 4. Un canon a la quinta (lineup-core, media)

`with_voices` toma pares `(voice, bar)`, así que cada voz de una ronda
canta la melodía a la misma altura. Muchos cánones tienen la segunda
voz una quinta más arriba o más abajo. Convierte cada par en un trío,
`(voice, bar, semitones)`, y mueve las notas de cada voz sus semitonos
donde la demostración construye las voces. Cada ronda ya escrita recibe
un `0`. Después prueba Frère Jacques con su segunda voz `7` semitonos
más arriba: ¿qué notas chocan, y por qué una ronda necesita sus copias
a la misma altura para encajar en el mismo acorde?

*Practica:* cambiar un tipo y seguir los errores del compilador.

### 5. Las primeras notas de tu canción (lineup-core, fácil)

`the_first_notes_are_right` comprueba que la Oda a la alegría empieza
fa♯ fa♯ sol la y Estrellita, ¿dónde estás? do do sol sol. Agrega una
línea para una canción que conozcas bien, con sus cuatro primeras notas
como números MIDI y sus nombres en el mensaje. Equivoca una a propósito
primero, y lee lo que dice el fallo.

*Practica:* `assert_eq!` con un mensaje, los números de nota MIDI.

### 6. Apoyar la nota más aguda (lineup-core, media)

`perform`, en `library/interpret.rs`, le da a una nota de una melodía
hasta seis puntos de velocidad según lo aguda que sea en su frase. Quien
canta suele apoyarse en la *única* nota más aguda de una frase más que
en las notas que la rodean. Dale a esa nota dos puntos más, y escribe
el test primero: en Estrellita, ¿dónde estás?, ¿cuál es la nota más
aguda de la primera frase? Mantén pasando el test
`a_tune_gets_its_accents_but_stays_close_to_how_it_was_written`.

*Practica:* `Iterator::max_by_key`, escribir el test antes que el
código.

### 7. Buscar por compás y tonalidad (la aplicación, fácil)

Abre **Archivo > Demostraciones** y escribe `7/8` en el cuadro de
búsqueda, luego `5/4`, luego `2+2+3/8`. Después prueba `re menor` y `D
menor`. ¿Qué coincidencias van primero, y cuáles más tenues? ¿Por qué
`fold` conserva `/` y `+` cuando quita todos los demás signos (ver
[Buscar](demos.md#buscar))?

### 8. Piénsalo: métodos que toman `self`

`in_key` y `with_tempo` son `const fn` que toman `self` por valor y
devuelven una `Part` nueva. ¿Por qué eso permite encadenarlas dentro de
una constante, `Part::new(...).in_key(...).with_tempo(...)`? ¿Por qué
no funcionaría ahí un método que tome `&mut self`?

## Leer la música

### 9. ¿Qué tecla negra? (lápiz, fácil)

En do mayor, nombra la tecla negra entre fa y sol, con las reglas de
[El nombre de las
notas](from-midi-and-scores.md#el-nombre-de-las-notas):
sobre un acorde D7; en la línea sol, ?, fa; y sobre un A♭7 que va a G.
Después nombra la tecla negra entre la y si sobre Gm, y sobre F♯7.

### 10. ¿Una nota nueva, o un modo nuevo? (lápiz y la aplicación, fácil)

Escribe las escalas de mi menor natural, mi menor armónica y mi dórico.
¿Qué nota cambia en cada una? Después toca los compases 21 a 23 de
**Balderrama**. La melodía canta do♯: ¿sobre qué acorde? ¿Está la
música en mi dórico ahí, o el do♯ es una nota de camino a otra? ¿Y por
qué la canción está escrita en mi menor natural y no en mi menor
armónica (ver [La melodía y la
tonalidad](from-midi-and-scores.md#la-melodía-y-la-tonalidad))?

### 11. Dos maneras de contar seis (la aplicación, fácil)

Da seis palmadas parejas, después agrúpalas 3+3 (UNO dos tres CUATRO
cinco seis) y 2+2+2 (UNO dos TRES cuatro CINCO seis). Toca **Acuarela
del río**, una polca en 6/8, y acompaña con palmas al bajo, luego a la
batería. ¿Qué agrupación toca cada uno?

### 12. El tempo de un pianista (la traza de reproducción, fácil)

Traza **Uno**:

```sh
ferrous-lineup --trace "Uno" --out uno.tsv
```

y lee sus líneas `# tempo:` de los primeros ocho compases. ¿Dónde
frena el pianista, y en qué lugar de la frase? ¿Cuántas veces cambia el
tempo en toda la canción?

### 13. Fundamentales a una quinta (la aplicación, fácil)

En **Porque un niño nos ha nacido**, toca los compases 41 y 42 con solo
el canal Teclados sin silenciar. Escribe los acordes desde el segundo
pulso del compás 41 y sus fundamentales. ¿A qué distancia está cada
fundamental de la siguiente? La nota que muestra el menú
Demostraciones para la canción nombra la misma cadena.

### 14. Otro acorde de 4/2 (la aplicación, fácil)

El compás 28 del coro de Handel tiene `A7/G`, un acorde de séptima con
la séptima en el bajo, cifrado 4/2 (ver [Bajo
cifrado](from-midi-and-scores.md#bajo-cifrado-acordes-escritos-como-números)).
El compás 45 tiene otro. ¿Qué acorde es, qué acorde lo sigue, y la nota
siguiente del bajo está un grado más abajo?

### 15. Las dominantes de la trucha (la aplicación, fácil)

**Die Forelle (La trucha)** está en re♭ mayor, y su tercera estrofa es
la que cambia. Busca los cuatro compases de E♭7 en la hoja de acordes.
¿Qué acorde viene después, y de qué es dominante? ¿Y de qué es
dominante E♭7?

### 16. Por qué do menor necesita si natural (la aplicación, fácil)

En **Las ovejas pueden pastar tranquilas**, busca en la hoja de acordes
los compases de la sección central en do menor. ¿Qué acorde trae el si
natural, y por qué lo necesita una tonalidad menor? (Compara la escala
de la tonalidad, la menor armónica, con do menor natural.)

### 17. Quién toca el Boléro (la aplicación, fácil)

Toca **Boléro** y anota, para cada exposición de la melodía, el
instrumento que la toca: te lo dicen los nombres de los patrones en la
pista Canción. ¿Cuántas exposiciones hay antes de que la música deje do
mayor, y a qué tonalidad va? Después lee las partes del Boléro en
`classical.rs`: en «Horn, celesta and piccolos» (Corno, celesta y
flautines), `played_by` les da a los flautines 19 y 28 semitonos. ¿Qué
intervalos son esos por encima de la melodía del corno, y qué notas de
la serie armónica copian?

### 18. El acorde de Petrushka (lápiz, fácil)

Toca do, mi, sol y fa♯, la♯, do♯ juntos: dos tríadas mayores a un
tritono, el acorde de Petrushka. Ahora toca la escala octatónica do,
re♭, mi♭, mi, fa♯, sol, la, si♭, que alterna semitonos y tonos. ¿Qué
notas de la escala no están en el acorde? ¿Por qué el acorde no puede
estar en ninguna tonalidad mayor o menor?

### 19. Un modo sobre un bordón (la aplicación, fácil)

Abre **Petrushka, La feria de carnaval al atardecer**, activa **🔍
Buscar la tonalidad** y toca la Danza de los cocheros, escrita en mi
mixolidio. ¿Qué tonalidad muestra el detector? Mi mixolidio tiene las
notas de la mayor: ¿por qué un detector que compara cuánto suena cada
nota oye la tonalidad con las mismas notas? Después imprime el informe
de tonalidades:

```sh
cargo test --release -p lineup-core --test key_finding report -- --ignored --nocapture
```

y busca en él el último cuadro de Petrushka. ¿Qué tonalidades da la
partitura, y cuáles oye el detector en su lugar?

### 20. Un acorde que la tonalidad no tiene (la aplicación, fácil)

**Tea for Two** está en la♭ mayor. Busca la marca de tonalidad donde
empieza su segunda parte. ¿Qué acorde de la♭ es do mayor? (Cuenta desde
la♭: do es la tercera nota, así que do mayor es III, un acorde que la
tonalidad de la♭ no contiene.)

### 21. De menor a mayor (la aplicación, fácil)

La estrofa de **Volver** está en sol menor y su estribillo en sol
mayor. Busca la marca de tonalidad donde empieza el estribillo. ¿Qué
dos notas de sol menor armónica cambian en sol mayor? Búscalas en los
primeros compases del estribillo.

### 22. Dos tempos y cuatro acordes (lápiz y la traza de reproducción, fácil)

Traza el **Tema de The Big Bang Theory** y busca la primera nota del
compás 5 y del compás 6. ¿Cuánto dura un compás de 4/4 a 159 pulsos por
minuto? (4 × 60 / 159 = 1,509 segundos.) ¿Y a 130? Después:

- Nombra los cuatro acordes de la estrofa, IVadd9 V I vi7, en sol y
  luego en la.
- En el compás 10 la melodía alterna sol y fa♯ sobre G, y luego sobre
  Em7. Sobre G, fa♯ es la séptima mayor del acorde; ¿qué es sobre mi
  menor?

### 23. Un tango contra su partitura guía (investigación, media)

Elige uno de los tangos de `tango_played.rs`, como **Patotero
sentimental**, y busca una partitura guía. Transporta sus acordes a la
tonalidad del pianista, y compara una parte compás por compás con el
cifrado de la demostración. ¿Qué acordes nombraron las notas solas por
un color (una sexta, una novena) en lugar del acorde sobre el que se
construye la canción?

### 24. Piénsalo: un script dentro de la aplicación

Los scripts que leen archivos MIDI y grabados no son parte de la
aplicación. ¿Qué haría falta para que **Archivo > Importar archivo
MIDI…** escribiera una constante de demostración en lugar de un
proyecto? Haz la lista de lo que hoy decide una persona y tendría que
decidir el importador: la tonalidad, dónde empiezan las partes, qué
pista es la melodía, qué acordes permite una partitura guía. ¿Cuáles de
esas cosas ya sabe adivinar la aplicación (ver [Oír la
armonía](../theory/hearing.md))?
