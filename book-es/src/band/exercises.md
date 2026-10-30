# Ejercicios: la banda

Estos ejercicios practican esta parte sobre el código real: la
biblioteca, la hoja de acordes, la banda y sus músicos, la orquesta, la
forma de la canción, las paradas, el solista, tus propios estilos, las
herramientas de práctica y la partitura guía. Cada uno dice qué **ya
existe**, para que lo amplíes en lugar de volver a construirlo. Algunos
son cosas que probar en la aplicación y escuchar; otros son piezas de
Rust con un final claro; los que se llaman «Piénsalo» son preguntas,
sin código que escribir.

Haz los de Rust en una rama propia. Ejecuta `cargo test` después de
cada cambio, y `cargo fmt --all --check` y
`cargo clippy --workspace --all-targets -- -D warnings` antes de dar
uno por terminado: la CI ejecuta los tres. Un ejercicio que añade algo
al archivo de proyecto necesita el siguiente número de formato libre
(ver [El número de
versión](../program/saving.md#el-número-de-versión)).

## La biblioteca

### 1. Un estilo propio (lineup-core, fácil)

`STYLES`, en `library/style.rs`, es una tabla de filas `style(...)` (ver
[Estilos: una tabla en un
`static`](library.md#estilos-una-tabla-en-un-static)). Añade
una: la progresión de una canción que te guste, escrita en números
romanos. Por ejemplo el Canon de Pachelbel entero,
`I | V | vi | iii | IV | I | IV | V` (el estilo **Balada de piano**
tiene solo sus seis primeros acordes), con `Comping::ArpUpDown` y
`Bass::Held`. Ejecuta `cargo test -p lineup-core` antes de abrir la
aplicación: si un número romano está mal escrito, si un compás no puede
contener sus acordes o si el nombre ya existe,
`every_style_parses_and_fits_in_a_pattern` o
`names_are_unique_and_rhythms_exist` nombran el estilo.

Después pruébalo en la aplicación sobre una canción en otra tonalidad.
¿Qué nombres de acordes cambian, y qué números romanos siguen iguales?

*Practica:* tablas `const fn`, literales de struct, leer los fallos de
los tests.

### 2. Un ritmo con cencerro y un fill (lineup-core, fácil)

Añade un patrón de batería a `RHYTHMS` en `library/rhythm.rs`, en el
bloque de su género, con una línea para `Kick`, otra para `Snare` y
otra para `Cowbell`, y un fill que exista, como `"Tom fill"`. En 4/4
cada línea tiene dieciséis caracteres, de entre `X`, `x`, `o`, `h`, `l`
y `.`; los espacios son solo para leer. Pon el cencerro en un
contratiempo o en una clave, no en cada paso, para que se oiga como un
color propio.

Está terminado cuando `every_line_is_whole_bars_of_known_characters` y
`names_are_unique_and_fills_exist` pasan, la Biblioteca muestra el
ritmo con su frase y, con la banda tocándolo, el fill llega en el
último compás de cada ocho.

*Practica:* datos como texto, tests que leen la tabla entera.

### 3. Una tercera chacarera (lineup-core, fácil)

La biblioteca tiene un ritmo `Chacarera` en 6/8 y un `Chacarera trunca`
en 3/4. Lee los dos primero: un compás de cada uno tiene doce pasos.
¿En qué pasos empiezan los pulsos de un compás de 6/8, y en cuáles los
de uno de 3/4? Ahora escribe un tercer patrón de bombo para la misma
danza, con el parche (`LowTom`) en los pulsos del 6/8 y el aro (`Rim`)
en los del 3/4, el dos contra tres del género, y un estilo que lo use.
Comprueba en el rack que los colores de los pulsos vienen en dos grupos
de seis.

*Practica:* leer los datos que ya existen antes de añadir, compases con
el mismo número de pasos.

### 4. Cerca del tempo de la canción (la aplicación, fácil)

En la Biblioteca, en una canción a 60 BPM, marca **Cerca de 60 BPM**.
¿Qué tempos conserva? Después en una canción a 180. `NEAR_SPEED` vale
1,15: calcula los márgenes antes de mirar, y di por qué un margen fijo
de ±15 BPM habría sido peor (ver [La pestaña
Biblioteca](library.md#la-pestaña-biblioteca)).

*Practica:* proporciones.

### 5. El mismo género (lineup-app, medio)

Añade una cuarta casilla, **Mismo género que el estilo de la canción**,
que conserve los preajustes del género que toca la banda. ¿Qué tiene
que saber `Narrowing`, de dónde lo sacas, y qué debería hacer la
casilla en una canción sin banda?

*Practica:* añadir un filtro sin tocar los demás.

### 6. Piénsalo: closures que se ejecutan una vez

`write_pattern_style` recibe `write: impl FnOnce(&mut Project, usize)`.
Los dos closures que se le pasan hoy solo leen lo que capturan.
¿Funcionaría `impl Fn` con ellos? ¿Qué podría hacer un closure futuro
que `Fn` rechazaría, y por qué pedir `FnOnce` es lo más amable para
quien escriba el siguiente?

## La hoja de acordes

### 7. Cuántos compases caben (lápiz, fácil)

`Columns::count` (ver [Por qué 1, 2, 4, 8, 12 o
16](chord-sheet.md#por-qué-1-2-4-8-12-o-16)) recibe el ancho
de la hoja y el ancho que necesita el compás más ancho. Calcula a mano
cuántos compases caben en una línea de una hoja de 900 píxeles cuando
el compás más ancho necesita 100, 120 y 250 píxeles. Después añade esos
tres casos al test que está junto a `count`, y ejecútalo.

*Practica:* leer de cerca una función corta.

### 8. Insertar varios compases (lineup-app, fácil)

**Insertar un compás antes** e **Insertar un compás después** llaman a
`project.insert_bars(bar, after, 1)`. Añade **Insertar compases…**, que
pregunte cuántos. ¿Qué línea de `open_bars` ya hace sitio para más de
uno, y qué impide que una canción crezca más allá de su máximo?

*Practica:* usar lo que el núcleo ya ofrece.

### 9. Un compás dentro de una repetición (lineup-core, medio)

Escribe un test en `bars.rs`: un clip repetido tres veces, y un compás
insertado en medio de su segunda vuelta. Escribe primero qué clips, y en
qué pasos, esperas después; luego ejecútalo. Los tests
`a_clip_of_a_pattern_played_elsewhere_too_is_cut_in_two` y
`a_repeated_clip_loses_the_repeats_in_the_deleted_bars` muestran cómo
preparar uno.

*Practica:* escribir lo esperado antes de que el código te lo diga.

### 10. Piénsalo: antes o después

`close_bars` lee el compás, el tempo y la tonalidad al final de los
compases eliminados *antes* de mover nada (ver [Qué se lleva un compás
eliminado](chord-sheet.md#qué-se-lleva-un-compás-eliminado)).
¿Qué leería si preguntara después, y cómo sonaría la canción?

### 11. El signo de repetición (lineup-core, medio)

Hoy `%` en un compás significa «sigue la armonía de antes», así que
después de `| C G |`, un `| % |` mantiene G, mientras que un músico que
lee una partitura vuelve a tocar C G. Haz que `%` repita todo el compás
anterior. `read_bar` solo ve su propio compás: ¿qué hay que darle? ¿Qué
deben escribir `write_bar` y `text::write` para un compás que mantiene
el acorde de antes, y para uno que repite un compás de dos acordes? Los
proyectos guardan los acordes, no el texto: ¿qué archivos podrían
cambiar de significado, y cuáles no?

*Practica:* cambiar lo que significa una notación, y quién la lee.

### 12. Un título en la canción (lineup-core y lineup-app, medio)

`text::read` lee una línea `Title:` en `TextSheet::title`, pero la
canción no la guarda: **Exportar hoja de acordes…** escribe como título
el nombre del archivo de la canción. Guarda el título en el proyecto y
escríbelo de vuelta. Es un campo nuevo del archivo: ¿qué lee un archivo
antiguo?

*Practica:* un campo nuevo desde el formato de texto hasta el archivo de
proyecto.

### 13. Pegar en la tonalidad de la canción (lineup-app, medio)

Una partitura pegada desde un mensaje conserva los nombres de sus
acordes, aunque su línea `Key:` diga otra tonalidad que la de la
canción. Añade **Pegar en la tonalidad de la canción**, que mueva los
acordes pegados de la tonalidad de la partitura a la de la canción.
`HarmonyName::moved(from, to)` ya lo hace para una armonía, con los
nombres correctos. ¿Qué necesita saber el pegado que hoy no guarda?

*Practica:* reutilizar una función de teoría en una acción de edición.

### 14. Compases por línea en el texto (lineup-core, fácil)

`text::write` pone como mucho cuatro compases por línea,
`BARS_PER_LINE`. Conviértelo en un parámetro. ¿Qué parte de la
alineación en columnas supone cuatro, y cómo tiene que cambiar
`widths`?

*Practica:* convertir una constante en un parámetro, arrays y vectores.

### 15. Piénsalo: otros formatos de partitura

- ChordPro, un formato habitual en cancioneros, escribe los acordes
  dentro de la letra: `[C]Twinkle twinkle [F]little [C]star`. No tiene
  barras de compás. ¿Qué tendría que adivinar un lector para hacer un
  `TextSheet` con él?
- Algunos músicos quieren el número romano grande y el nombre del
  acorde pequeño debajo. `HarmonyName::caption` da el número romano de
  un acorde. ¿Qué debería decir la línea grande para un conjunto, que no
  tiene número romano?

## Cómo toca la banda

### 16. Líneas de bajo a la vista (la aplicación, fácil)

Escribe `C | Am | F | G7 | C` y elige **Vals de jazz**. Mira el bajo en
el piano roll: ¿qué nota toca en el tiempo 3 de cada compás, y a qué
distancia está, por su nombre, de la fundamental del compás siguiente?
Después elige **Swing**, en 4/4: ¿qué pone ahora el walking en los
tiempos 2 y 3? Comprueba tus respuestas con `walk` en [El
bajo](band.md#el-bajo).

*Practica:* leer una línea de bajo.

### 17. Dónde cambia el acorde (la aplicación, fácil)

Escribe un compás `Dm7 G7` y un compás `C / / G7` en **Ritmo pop**. ¿En
qué tiempo empieza G7 en cada uno? Mira la parte del piano: ¿cambia su
ritmo en esos compases, o solo sus notas?

### 18. Bajo de Alberti (la aplicación, fácil)

Elige **Bajo de Alberti** sobre `C | Am | F | G7`. El patrón es «grave,
aguda, media, aguda». ¿Qué nota es la grave en el compás de Am, y por
qué no es La? (Pista: mira los cuatro compases, y [Las armonías, como
notas](band.md#las-armonías-como-notas).)

### 19. Una parte que cambiaste (la aplicación, fácil)

En una canción nueva, cambia una nota del bajo y después escribe un
acorde nuevo en el compás 2. ¿Qué partes siguen al acorde nuevo, y cuál
no? ¿Qué dice el aviso? Pulsa **Volver a tocarla**: ¿qué pasa con tu
nota?

### 20. Una banda sobre una demostración (la aplicación, fácil)

Abre el **Canon en Re** y pulsa **Balada pop** en la Biblioteca. Mira el
rack de canales: ¿qué canales quedan silenciados, y cuáles siguen
tocando? Después elige **Sin banda** en el menú de la banda. ¿Vuelve la
demostración a estar exactamente como estaba?

### 21. Piénsalo: lo que deja fuera una huella

- `part_print` deja fuera la pista en la que está un clip y el número de
  su patrón. ¿Qué vería el usuario si no lo hiciera?
- `band_styles()` devuelve `&'static [BandStyle]`. ¿Por qué `Band`
  puede guardar un `&'static BandStyle` sin parámetro de lifetime, y no
  podría guardar un `&BandStyle` prestado de un `Vec` local?

## Una banda más grande, vientos, y seguir tus acordes

### 22. Cuatro guitarristas (la aplicación, fácil)

Haz una hoja de acordes con `C Am F G` y prueba **Ritmo pop**, **Folk**,
**Ritmo de rock** y **Swing**. Pulsa **Piano**, **Bajo** y **Batería**
en la fila **Tocas tú**, para que solo quede la guitarra. ¿Cómo toca
cada guitarrista los mismos cuatro acordes?

### 23. Posiciones de acordes (lápiz y piano roll, fácil)

En **Ritmo pop**, lee en el piano roll las notas de la guitarra para G y
para F, y escribe sus posiciones como lo hacen los libros de acordes (G
es `320003`; ver [La música: cómo toca un acorde un
guitarrista](band.md#la-música-cómo-toca-un-acorde-un-guitarrista)).
¿Por qué la posición de F no tiene cuerdas al aire? ¿Qué posición
tocarías para E7, y qué notas del acorde duplica? Comprueba tus
respuestas con `open_chords_are_the_ones_in_chord_books` y
`other_keys_take_barre_chords`.

### 24. Acordes completos con distorsión (lineup-core, fácil)

En `band/extras.rs`, cambia la guitarra de Ritmo de rock de
`GuitarPart::Power(Comping::Pulse)` a
`GuitarPart::Strum(Comping::Pulse)`: posiciones completas por la
guitarra saturada. Escúchalo con una SoundFont. ¿Por qué lo evitan los
guitarristas de rock? Vuelve a dejarlo como estaba.

### 25. Una guitarra para Lo-fi (lineup-core, fácil)

**Lo-fi** no tiene músicos extra. Dale una guitarra de cuerdas de nailon
que arpegie los acordes, `GuitarPart::Strum(Comping::ArpUp)`, en
`BY_STYLE` de `extras.rs`. Después escribe mal el nombre del estilo:
¿qué test falla, y qué imprime?

*Practica:* tablas de constantes, tests que vigilan nombres.

### 26. Piénsalo: la guitarra

- Sobre un acorde mayor, el boogie shuffle se mece entre la quinta y la
  sexta; sobre un acorde menor se queda en la quinta (ver
  `the_boogie_alternates_fifth_and_sixth`). ¿Qué acorde forman La, Do,
  Mi y Fa, y por qué sería arriesgada una sexta menor sobre La menor?
  ¿En qué convertiría el acorde una sexta mayor, Fa♯?
- `try_frets` recibe `found: &mut impl FnMut(&[u8])`. Prueba con
  `found: impl FnMut(&[u8])`. ¿Qué dice el compilador sobre la llamada
  recursiva, y por qué?

### 27. Un segundo teclado (lineup-core, más difícil)

Añade un piano eléctrico (programa 4) que toque el acompañamiento del
estilo una octava más arriba en los estilos **R&B** y **Neo soul**.
¿Qué partes de `band.rs` y de `file.rs` tienen que cambiar (mira
`written` y `written_too` en [En el
archivo](band.md#en-el-archivo))? Comprueba que una canción
cuya banda tiene solo tres músicos se guarda con el mismo número de
formato que antes.

### 28. Vientos sostenidos y golpeados (la aplicación, fácil)

Marca **Sección de vientos** en **Swing** y en **Balada pop**, sobre
los mismos acordes. Compara las notas de los vientos en el piano roll:
¿qué es igual, y qué cambia? ¿Por qué la conducción suave de las voces
importa todavía más en notas largas que en notas cortas?

### 29. Notas comunes (la aplicación, fácil)

Escribe `C Am F G` en **Ritmo pop** con los vientos, y anota la
disposición de los vientos para cada acorde, desde el piano roll. ¿Qué
viento mantiene su nota de un acorde al siguiente, y durante cuántos
acordes? Una nota que comparten dos acordes es una **nota común**: ¿por
qué no se mueve el viento que la toca?

### 30. Quintas baratas (lineup-core, fácil)

`cost` cobra `PARALLEL_FIFTHS`, 24 semitonos, por cada par de vientos
que se mueve en quintas paralelas. Ponlo en 2 y ejecuta
`cargo test -p lineup-core horns`. ¿Qué test falla, entre qué dos
acordes, y qué vientos se mueven en quintas? Vuelve a dejarlo como
estaba.

### 31. Piénsalo: nombres en dos tablas

`HELD` y `BY_STYLE`, en `horns.rs`, nombran estilos por su nombre. ¿Qué
pasa si se cambia el nombre de un estilo en la biblioteca? Encuentra el
test que lo detectaría.

### 32. Un punto de división (lineup-app, medio)

Los teclados con acompañamiento suelen tener un **punto de división**:
solo las teclas por debajo de él eligen el acorde, para que la mano
derecha pueda tocar una melodía. Añade uno, por ejemplo por debajo de
Do4, como segunda opción junto a **El acompañamiento sigue los
acordes**. ¿Qué teclas llegan ahora a `chord_held`, y qué sigue pasando
con las de arriba?

### 33. Dos teclas a una segunda mayor (lineup-core, fácil)

`chord_held` lee dos teclas por el intervalo que hay entre ellas (ver
[Seguir los acordes que
mantienes](band.md#seguir-los-acordes-que-mantienes)). Haz
que dos teclas a una segunda mayor sean un acorde sus2 sobre la más
grave. ¿Qué test necesita un caso nuevo? Antes de tu cambio, mantener Fa
debajo de Sol no pedía ningún acorde; ¿qué pide ahora, y es eso lo que
querría decir un músico que las mantiene?

### 34. Piénsalo: acordes al instante

Las notas que ya suenan conservan su altura hasta que terminan, así que
un acorde mantenido todo un compás solo cambia en el compás siguiente.
Los teclados con acompañamiento tocan el acorde nuevo al instante. ¿Qué
tendría que saber el hilo de audio para hacerlo, y cuánto costaría?

## La orquesta

### 35. Palmas con el bombo (la aplicación, fácil)

Aplica la Orquesta a **La consagración de la primavera, Augurios
primaverales** y silencia la pista **Canción**. Da palmas con el bombo.
¿Puedes predecir dónde llega el golpe siguiente? Vuelve a activar la
canción y escucha los acentos de los vientos cayendo en las mismas
corcheas.

### 36. Pulsos sentidos a mano (lápiz, fácil)

`TimeSignature::felt_groups` da 13/16 como 2 + 2 + 2 + 2 + 2 + 3. ¿En
qué semicorcheas golpean los vientos en un compás de 13/16 en el que se
atienen a los pulsos sentidos? Abre **Petrushka, La habitación del
moro**, aplica la Orquesta, busca un compás de 13/16 en la regla y
compruébalo. Si ahí tocan otra cosa, ¿qué regla de [Tocar junto a la
partitura](orchestra.md#tocar-junto-a-la-partitura) lo
explica?

### 37. `in_meter` para el compás del tresillo (lineup-core, fácil)

Calcula a mano la línea que escribe `in_meter` para un compás de
3+3+2/8 a partir del `X... .... o... ....` de la Orquesta. Después
añádela a `a_rhythm_that_follows_the_meter_plays_each_bar_in_its_own`
en `rhythm.rs`, y ejecútalo.

### 38. Un estilo en 4/4 sobre compases que cambian (la aplicación, fácil)

Con la Orquesta tocando el **Juego del rapto**, pulsa **Ritmo pop** en
la Biblioteca. ¿Qué le pasa a la regla, y por qué un estilo en 4/4
tiene que hacerlo, cuando la Orquesta no lo hizo?

### 39. Solo las voces (la aplicación, fácil)

Abre **Shchedryk (Villancico de las campanas)**, aplica la Orquesta con
solo **Voces / instrumentos** marcado, y abre el **Mezclador**. ¿Qué
instrumentos tocan los canales de la pista Canción? ¿Cuáles tocan las
pistas nuevas Piano y Bajo?

### 40. Acentos que nadie escribió (lineup-core, fácil)

En `band/following.rs`, cambia `TUTTI` de `(9, 5)` a `(3, 2)` y ejecuta
`cargo test -p lineup-core accents_of_the_augurs`. ¿Aparecen acentos que
Stravinsky no escribió? Vuelve a dejarlo como estaba.

## Forma, variaciones, secciones y bloqueos

### 41. La introducción lleva a algún sitio (la aplicación, fácil)

Escribe `| Dm7 | G7 | Cmaj7 | Cmaj7 |`, un ii–V–I en Do, y dale una
introducción de 2 compases. ¿Qué acordes escribe la introducción? No
llevan a C: ¿a dónde llevan, y por qué?

### 42. El último acorde (la aplicación, fácil)

Pon la tonalidad en La menor (el menú **Tonalidad** junto a las teclas
de armonía), escribe una canción que empiece en Am y activa el final.
¿En qué acorde termina? Ahora cambia el primer acorde a F: ¿en qué
acorde termina ahora, y por qué sigue siendo correcto?

### 43. Cuánto dura la canción (la aplicación, fácil)

Exporta como WAV una canción con una introducción de 4 compases, tres
vueltas y un final. ¿Cuántos compases dura? Comprueba tu respuesta con
la duración del archivo y el tempo.

### 44. La forma se mueve (la aplicación, fácil)

- Dale a una canción una introducción de 8 compases, cambia un acorde de
  ella, y después cambia la introducción a 2 compases. ¿Qué pasa con tu
  acorde? Deshazlo.
- Escribe un acorde en el primer compás del final. ¿Dónde está ahora el
  final, y dónde el signo de fin de repetición?

### 45. Piénsalo: la forma en Rust

- `Form` es `Copy`; `Band` no. Mira sus campos: ¿por qué no podría
  serlo `Band`?
- En `played_pieces`, la vuelta solo se repite
  `if from <= start && end <= to`. ¿Qué tocaría sin esa comprobación
  una región de bucle que va del medio de la introducción al medio de la
  vuelta?

### 46. A dos, y después caminando (la aplicación, fácil)

Elige el estilo **Balada de jazz**, escribe `| Cmaj7 | Am7 | Dm7 | G7 |`
dos veces, y pon los compases 5 a 8 en la variación b. Abre los patrones
del bajo en el piano roll. ¿Cuántas notas tiene un compás en a, y en b?
¿Qué notas toca el bajo en el compás de Dm7 de cada una?

### 47. Fills y crashes (la aplicación, fácil)

- En un **Ritmo pop** de ocho compases, pon el compás 5 en la variación
  b y el 6 otra vez en a. ¿Qué compases tienen un fill de batería, y
  dónde caen los crashes? ¿Por qué podría sonar inquieto en una canción
  de verdad?
- Elige **Swing** y toca unos compases en cada variación. ¿Qué cambia y
  qué se queda? ¿Por qué el tiempo sigue en el mismo platillo?

### 48. Un estilo para algunos compases (la aplicación, fácil)

- Escribe `| C | Am | F | G |` dos veces en **Ritmo pop**, y haz que los
  compases 5 a 8 sean **Bossa nova**. ¿Dónde empieza la línea de bajo
  de la bossa nova, y en qué se diferencia de la del pop?
- Toca una sección en **Coral**, que no tiene batería. ¿Qué toca la
  batería ahí, y en el compás de antes?

### 49. Bloqueos (la aplicación, fácil)

- Abre **Oda a la alegría**, bloquea **Teclados** en el rack y aplica
  **Doo-wop**. ¿Qué pistas cambiaron, y qué conservó Teclados?
- En una canción nueva, bloquea el bombo y aplica **Bossa nova**. ¿Qué
  toca ahora el bombo?

### 50. Piénsalo: marcas y préstamos

- `tidy`, para las marcas de variación, ordena, después llama a
  `dedup_by` y después a `retain`. ¿Qué saldría mal si llamara a
  `retain` antes de ordenar?
- `played_in_runs` recibe `play: impl Fn(...)`. ¿Podría recibir un
  `FnMut`? ¿Qué cambiaría donde se la llama?
- `where_it_plays` devuelve `impl Iterator`. ¿Qué cambiaría si
  devolviera un `Vec`, y por qué `notes.extend(...)` acepta las dos
  cosas?
- `StylePattern` guarda un `&'a mut Pattern`. ¿Qué dice el borrow
  checker si llamas a otro método de `Project` mientras uno está vivo, y
  por qué eso es bueno?

## Las paradas y el solista

### 51. Stop-time (la aplicación, fácil)

Escribe un blues de doce compases en Fa con golpes en los cuatro
primeros compases:
`| F7. | F7. | F7. | F7. | Bb7 | Bb7 | F7 | F7 | C7 | Bb7 | F7 | C7 |`,
y elige **Shuffle**. Toca en los huecos con la escala de blues de Fa
(Fa La♭ Si♭ Si Do Mi♭). ¿Qué cambia en la sensación cuando la banda
vuelve en el compás 5?

### 52. Anticipaciones (la aplicación, fácil)

Escribe `| C | Am | F | G |` en **Ritmo pop** y escúchalo. Después
anticipa todos los acordes menos el primero: `| C | ^Am | ^F | ^G |`.
Da palmas con el ritmo del piano de las dos maneras. ¿Qué tiempo ya no
se golpea? Después compara el piano y el bajo en el piano roll: ¿cuánto
mantiene cada uno un acorde anticipado?

### 53. Un sostenido de dos compases (lineup-core, medio)

Escribe `C..` en el compás 1, nada en el compás 2 y `G7` en el compás
3, para que el sostenido dure dos compases, y mira el bajo: una sola
nota. [Cada músico golpea a su
manera](stops.md#cada-músico-golpea-a-su-manera) explica por
qué el bajo usa `bass_struck` y no `Bass::Held`. Escribe un test en
`band.rs` que fallaría si usara `Bass::Held`.

### 54. Un golpe de una negra (lineup-core, fácil)

Un golpe dura una corchea: `SHOT_STEPS`, en `band/stops.rs`, vale 2.
Haz que dure una negra y ejecuta `cargo test -p lineup-core`. Fallan
tests no solo en `stops.rs` (`a_shot_strikes_once_and_the_band_stops`)
sino también en las partituras y las demostraciones de la biblioteca:
¿por qué les importa a esos cuánto dura un golpe? Corrige lo que
esperan, o decide que el cambio estaba mal.

### 55. Una anticipación de una negra (lineup-core, medio)

Añade `^^^C`, una anticipación de una negra: una variante de `Push`.
Deja que el compilador te muestre cada `match` que tiene que aprenderla
(`steps`, `marks`, `describe`). ¿Qué otros sitios la necesitan? (Pista:
`read_marks` rechaza hoy tres acentos circunflejos, y `breaks_menu`, en
la aplicación, enumera las anticipaciones.)

### 56. Piénsalo: un fill antes de una parada

La batería toca un fill en el compás anterior a un cambio. ¿Debería
conservarlo un golpe o un silencio en ese compás? Busca dónde añade el
fill `drum_part` y dónde quita el groove `apply_to_drums`, decide, y
escribe un test para lo que elegiste.

### 57. Notas guía y escalas de acorde (la aplicación, fácil)

- Con el solista activado, abre su parte en el piano roll y busca las
  notas marcadas `3` y `7`. ¿Dónde pasa una 7.ª a una 3.ª medio tono
  más abajo en un cambio de acorde? Canta solo las 3.ª y las 7.ª, una
  por compás, sobre la banda.
- Escribe `| Am7 | D7 | Gmaj7 | E7 | Am7 | D7 | G6 | G6 |` en Sol mayor.
  Calcula la escala de cada acorde con las reglas de [Escalas de
  acorde](soloist.md#escalas-de-acorde) antes de mirar. Un
  acorde toca un Fa natural, aunque la tonalidad tiene Fa♯: ¿cuál, y
  qué otra nota de su escala no está en Sol mayor?
- En La menor, escribe `| Bm7b5 | E7 | Am | Am |`. ¿Qué nota de la
  escala de E7 no está en La menor natural? Búscala con el oído en el
  solo.

### 58. Saltos y rodeos (lineup-core, medio)

- El solista salta a la segunda nota del acorde una vez de cada cinco
  (`dice.chance(20)` en `target`). Ponlo en 60 y toca unos cuantos
  solos con **Otro solo**. ¿Cómo cambia la línea? ¿Qué tests siguen
  pasando, y por qué?
- Añade un **rodeo** (*enclosure*), una figura favorita del bebop: antes
  de una nota objetivo, una nota de la escala por encima y luego la nota
  medio tono por debajo, como Re Si Do hacia Do. Dale un `NoteRole`
  nuevo, deja que el compilador te muestre cada `match` que tiene que
  aprenderlo, y escribe un test al estilo de
  `roles_name_what_each_note_does`.

### 59. Un solo nuevo en cada vuelta (lineup-core, más difícil)

Con **Vueltas** en 3, el solista toca el mismo solo tres veces. Haz que
cada vuelta sea un solo nuevo. ¿Dónde se repite la vuelta (busca
`repeated_chorus`), y cómo podría cada vuelta tener su propia semilla
mientras la canción sigue sonando igual cada vez que se abre?

## Tus propios estilos y las herramientas de práctica

### 60. Lee los archivos (la aplicación y un editor de texto, fácil)

Guarda un estilo desde el editor de estilos (ver
[Probarlo](style-editor.md#probarlo)) y abre el archivo
`.lineup-style` en un editor de texto. Busca su `format`, `bars`,
`chart` y los `lanes` con cada canal y sus notas. Después aplícalo a una
canción, guarda la canción y busca `custom_rhythm` en su archivo: el
estilo entero está dentro. ¿Por qué tiene que estarlo, y qué número de
formato tiene como mínimo la canción?

### 61. Dominantes fuera de la tonalidad (lineup-core, medio)

Sobre un acorde fuera de la tonalidad, `adapt_pitch`, en
`user_style.rs`, conserva las distancias escritas sobre Do, así que el
E7 que lleva a Am toca Mi Fa♯ Sol♯ La Si Do♯ Re (ver [Acordes de fuera
de la
tonalidad](style-editor.md#acordes-de-fuera-de-la-tonalidad)).
Las escalas de acorde del solista, en `harmony/chord_scale.rs`, le
darían el Fa y el Do de La menor. Úsalas para los acordes de dominante
fuera de la tonalidad. ¿Qué notas escritas de un estilo sobre E7 en Do
mayor suenan ahora distinto? Escribe primero el test, al estilo de
`template_notes_stay_in_the_key`, y escucha: ¿vale más un patrón que
suena igual sobre cada acorde, o las notas correctas?

### 62. Cuántos compases (lápiz, fácil)

Un estilo de usuario tiene como mucho 256 pasos y como mucho dieciséis
compases. ¿Cuántos compases puede tener un estilo en 7/8, y uno en 6/4?

### 63. Lo que cuenta el metrónomo (la aplicación, fácil)

- Activa el metrónomo sobre **Noche de paz**, en 6/8. ¿Cuántos clics hay
  en un compás, y dónde cae el agudo? Ahora **Greensleeves**, en 3/4:
  las mismas seis corcheas por compás, contadas de otra manera.
- Con la cuenta de entrada activada, pausa una canción en medio de un
  compás y vuelve a pulsar Reproducir. ¿Qué oyes antes de la música, y por qué
  es mejor que contar un compás entero y empezar en medio del
  siguiente?
- Pon la velocidad al 50% y exporta un archivo WAV. ¿Cuánto dura,
  comparado con la canción a velocidad normal? ¿Por qué?

### 64. Marcar el tempo (la aplicación, fácil)

- Marca 4 veces a un ritmo regular, y después 4 veces más deprisa sin
  parar. ¿Qué tempo obtienes, y por qué no el promedio de los ocho
  intervalos?
- En una canción en 6/8, marca 80 veces por minuto. ¿Qué tempo recibe la
  canción, y por qué?
- Encárgate del bajo con **Tocas tú**, cambia un acorde y devuelve el
  bajo: ¿está el acorde nuevo en su parte?

### 65. Piénsalo: el clic y la cuenta de entrada

- `ClickVoice::strike(Click::None)` vuelve enseguida. ¿Qué oirías si
  pusiera `level` a 0?
- En `begin_play`, ¿qué sale mal si no se llama a `self.stop()` cuando
  llega la orden de reproducir mientras la canción ya suena?

### 66. El entrenador de velocidad a mano (la aplicación, fácil)

- Pon **desde 50% hasta 100% por 10% cada 2 pasadas**. Anota la
  velocidad de cada una de las ocho primeras vueltas, y después tócalo
  con la traza activada y compruébalo.
- Pon **desde 100% hasta 60%**. ¿Qué hace ahora el entrenador? Piensa en
  un uso (pista: un pasaje que ya puedes tocar rápido, pero todavía no
  parejo).
- Empieza una tanda, pulsa **Pausa** en medio y luego Reproducir.
  Después Detener y Reproducir. ¿Qué velocidad hay cada vez, y por qué es útil que los
  dos botones sean distintos?
- Con el entrenador activado y nada en bucle, toca una canción entera.
  ¿Cuándo sube la velocidad?

### 67. Piénsalo: los mensajes del entrenador

- En `came_round`, `self.laps_seen.replace(laps)` se ejecuta antes de
  comprobar `self.training`. ¿Qué saldría mal al activar el entrenador
  en medio de una canción si se ejecutara después?
- El hilo de audio tira con `throw_away` una lista vieja de tempos por
  vuelta cuando una petición nueva la sustituye. ¿Qué pasaría si
  simplemente asignara la lista nueva, y por qué importa eso justamente
  en el hilo de audio?

### 68. Más para el oído (lineup-core, medio)

- **Una melodía para la séptima mayor.** `Interval::tune` no tiene
  ninguna hacia arriba. Busca una canción cuya melodía empiece con una,
  compruébala en un piano y añádela. ¿Tiene que cambiar
  `going_up_names_a_tune`?
- **Melodías hacia abajo.** Añade un `tune_down` (el comienzo de la
  Quinta sinfonía de Beethoven es una tercera mayor hacia abajo) y úsalo
  cuando la pregunta fue hacia abajo.
- **Lo que fallas.** Lleva la cuenta, para cada intervalo, de cuántas
  veces se preguntó y cuántas acertaste, y pregunta más a menudo los
  débiles. ¿Cómo usaría `IntervalQuestion::new` esas cuentas sin perder
  la regla «misma semilla, misma pregunta»?

### 69. Piénsalo: el entrenamiento auditivo

- Los acordes de sexta quedan fuera porque C6 y Am7 tienen las mismas
  notas. ¿Podrían preguntarse con justicia si el acorde sonara siempre
  sobre su fundamental una octava más abajo? ¿Seguirían teniendo sentido
  las inversiones?
- Haz que el ejercicio de progresiones toque la propia hoja de acordes
  de la canción, con los nombres ocultos. ¿Qué necesitaría de
  `chord_sheet.rs`?

## La partitura guía y las partes

### 70. Decisiones de notación (lineup-core, medio)

- **Alteraciones de cortesía.** Cuando un compás tiene un Fa♯ y el
  siguiente un Fa natural por la armadura, muchas partituras guía
  imprimen un becuadro entre paréntesis como recordatorio. ¿Dónde
  tendría que saber `BarAccidentals` del compás anterior?
- **Semidisminuido.** Algunas partituras guía escriben Em7♭5 como Eø7.
  Añade una opción para ello. ¿Es una decisión de `layout.rs`, o del
  nombre del acorde?

### 71. Piénsalo: la página

- **Casillas de repetición.** Cuando el último compás de una sección
  repetida cambia la segunda vez, las partituras guía usan primera y
  segunda casilla: un corchete «1.» sobre un compás, «2.» sobre el otro.
  ¿Qué tendría que contarle la lista de reproducción a `score.rs`?
- **Compases de silencio.** Una melodía que calla ocho compases podría
  mostrar un solo compás con una línea gruesa y un «8» encima. ¿Dónde
  iría en `systems`, y cómo cambia los números de compás de la línea
  siguiente?

### 72. Más instrumentos (lineup-core, medio)

- **La octava exacta.** Añade una opción que escriba cada parte
  exactamente como la toca el instrumento: una novena arriba para el
  saxo tenor, una octava y una sexta para el barítono. ¿Qué función
  cambia? ¿Cuántas líneas adicionales necesita una melodía alrededor de
  Do5 para el tenor, y por qué los libros de partituras guía no lo
  hacen?
- **Instrumentos en Re y en Sol.** Una trompeta en Re suena un tono más
  aguda de lo escrito, una flauta contralto en Sol una cuarta más grave.
  Añádelos. ¿Qué devuelven `note()` y `semitones()`, y qué más de la
  ventana y de la etiqueta tiene que saber de ellos?
- **El clarinete en Mi♭** suena una tercera menor *más aguda* de lo que
  lee. ¿Es una `Transposition` nueva, o la parte del saxo alto en otra
  octava? ¿Qué tendría que saber `shift` para distinguirlas?

### 73. Partes como texto, y la tonalidad real (lineup-core, fácil)

- Dale a **Exportar hoja de acordes…** la misma elección de
  instrumentos. ¿Qué función, ella sola, hace el trabajo para cada
  acorde? (Ver [Los acordes: movidos con nombre y
  todo](lead-sheet.md#los-acordes-movidos-con-nombre-y-todo).)
- Algunas partes imprimen la tonalidad propia de la canción debajo del
  instrumento, «Tonalidad real: Si♭». ¿Dónde iría en `layout.rs`, y qué
  necesita saber `lay_out` que ahora no sabe?
