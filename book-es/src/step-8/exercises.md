# Ejercicios

## 1. Un estilo propio (lineup-core, fácil)

Añade un estilo a `STYLES` en `library/style.rs`: una progresión de una
canción que te guste, escrita en numerales. Por ejemplo la progresión
«Pachelbel», `I | V | vi | iii | IV | I | IV | V`, con
`Comping::ArpUpDown` y `Bass::Held`. Ejecuta `cargo test -p lineup-core`
antes de abrir la app: si un numeral está mal escrito, o un compás no
puede compartirse entre sus acordes, una prueba nombra el estilo.

Luego pruébalo en otra tonalidad desde la pestaña Library, y mira su
tooltip. ¿Qué acordes cambian de nombre, y qué numerales se quedan
igual?

*Prácticas:* tablas `const fn`, literales de struct, leer fallos de
pruebas.

## 2. Una segunda chacarera (lineup-core, fácil)

La Library ya tiene `Chacarera` y `Chacarera trunca` en `STYLES` y
`RHYTHMS`, y *Chacarera del patio* en `SONGS`. Lee primero los dos
ritmos: `Chacarera` en 6/8 pone la cabeza del bombo contra su aro, y
`Chacarera trunca` en 3/4 corta sus frases antes. Luego añade un tercer
ritmo para la misma danza con un patrón de bombo distinto, y un estilo
que lo use. Mantén el 6/8 si escribes una chacarera: un compás son doce
pasos, así que cada línea tiene doce caracteres:

```text
LowTom  "X.....  ..x.x."
Rim     "..x.x.  x....."
```

Comprueba en el rack que los colores del pulso vienen en dos grupos de
seis. ¿Por qué el estilo `Chacarera` existente encaja en un compás de
6/8 mientras que la trunca necesita 3/4?

*Prácticas:* leer los datos existentes antes de añadir, la notación de
ritmos, compases `const`.

## 3. Latch (lineup-app, fácil)

Para grabar un acorde largo, tienes que mantener pulsada la harmony key
durante todo el acorde. Añade un interruptor **Latch** junto a Triads,
7ths y 9ths: mientras está encendido, una harmony key sigue sonando
después de soltar, hasta que se pulsa otra tecla o se apaga Latch.

Mira `Down` y `Holder`. ¿Podría un acorde con Latch ser un tercer tipo
de `Holder`? ¿Qué debe pasarle cuando la barra se cierra con F8?

*Prácticas:* enums con datos, máquinas de estados.

## 4. Acordes tomados en préstamo (lineup-core y lineup-app, medio)

Las canciones en tonalidades mayores toman prestados a menudo acordes de
la tonalidad menor con la misma tónica: iv (Fa menor en Do), bVI (La♭),
bVII (Si♭) y bIII (Mi♭). Esto se llama *mixtura modal*, y está detrás de
muchos de los momentos agridulces del pop.

Añade una segunda fila de teclas, más pequeña, debajo de las harmony
keys con estos cuatro acordes, construidos a partir de numerales con
`library::numeral::parse` y `RomanChord::in_key`. Dales su propio color,
y un tooltip que diga qué acorde de la tonalidad menor es cada uno.

*Prácticas:* reutilizar los numerales de la Library, maquetación de egui.

## 5. Ampliar una canción que ya está ahí (lineup-core, medio)

*La Paloma*, *Shenandoah* y *Danny Boy* están ya en `SONGS`, y cada una
muestra algo distinto: *La Paloma* tiene estrofa, estribillo y coda;
*Shenandoah* es una canción de río que empieza en anacrusa y llega
alto; *Danny Boy* mantiene su estrofa baja y salta arriba en el
estribillo. Elige una y añade una parte que la demo aún no toca: una
introducción, una segunda estrofa o una coda. Escribe la melodía compás
a compás con `|` entre compases, dale un chart de acordes, y añádela al
`form` de la canción.

Deja que las pruebas te digan qué compases no cuadran. Luego ábrela
desde **File > Demos**. ¿Encuentra `analyze` los acordes que escribiste
mientras suena? Si no, ¿qué notas lo confunden?

*Prácticas:* la notación de melodías, leer datos existentes, las pruebas
como red de seguridad para los datos.

## 6. Un mapa de compases dentro de un patrón (lineup-core, difícil)

El paso 8 ya tiene un mapa de compases a nivel de canción: `Meter`
guarda el `TimeSignature` inicial y una lista ordenada de `TimeMarker`s,
y `bar_at`, `time_at` y `click` lo leen. Un `Pattern` sigue teniendo
solo un `time: Option<TimeSignature>`, así que una parte que cambia de
compás cada compás se corta en un patrón por tanda de compases.

Da el siguiente paso: deja que un patrón tenga un mapa de compases
propio, con cambios en las líneas de compás. `Pattern::length` cuenta
pasos, y un compás ya no es un número fijo de ellos, así que:

- ¿Qué código que pregunta `TimeSignature::steps_per_bar` o
  `Meter::bar_at` por un patrón necesitaría preguntar en su lugar al mapa
  del patrón: las herramientas de notas, `Project::pattern_clicks`, las
  líneas de compás del piano roll?
- ¿Qué deberían preguntar el playlist y el piano roll cuando un clip
  empieza a mitad de un patrón?
- ¿Cómo lo tendría en cuenta `format_for`, para que un patrón en un solo
  compás se escriba exactamente como ahora?

Empieza con `Meter` en lugar de inventar un segundo tipo. ¿Por qué
guardar los cambios en una lista ordenada convierte `bar_at` en una
búsqueda, y por qué el `Meter` existente no usa simplemente
`partition_point`?

*Prácticas:* leer una estructura de datos existente antes de ampliarla,
aritmética de compases, mantener compatibles los archivos.

## 7. Recordar el nivel de ayuda (lineup-app, medio)

El nivel de ayuda vuelve a Short cada vez que la app arranca. eframe ya
guarda la memoria propia de egui entre ejecuciones (el zoom y las
posiciones de las ventanas, ver [Adaptarse a cualquier ventana](fitting-the-window.md)),
porque su feature `persistence` está encendida. Los ajustes propios de
la app necesitan un poco más: da a `HintLevel` los `Serialize` y
`Deserialize` de serde, guárdalo en `App::save` con `eframe::set_value`,
y léelo de vuelta con `eframe::get_value` desde
`CreationContext::storage` cuando la app arranque. ¿Qué otros ajustes
recordarías de la misma forma? Si el channel rack está abierto es uno.

*Prácticas:* features de Cargo, derives de serde, el trait `eframe::App`.

## 8. Haz zoom de los popups también (lineup-app, pequeño)

Haz zoom del channel rack al 150% y haz clic derecho en el nombre de un
canal: el menú se dibuja al tamaño normal. Un popup es una capa propia,
así que nunca ve el estilo que cambió el contenido de la ventana (ver
[Encontrar tus notas](finding-your-notes.md)). Encuentra dónde deja egui
cambiar el estilo de un popup o de un combo box, y dale la escala de la
ventana. (El playlist y el piano roll ya no escalan su texto al hacer
zoom: ver [Una cacería de errores](bug-hunt.md).)

*Prácticas:* leer el código fuente de una biblioteca, las capas y los
estilos de egui.

## 9. Ayudas en todas partes (lineup-app, pequeño)

Solo la barra de herramientas y las harmony keys dan algo que decir a la
barra de ayuda (ver [Un aspecto más claro](look-and-transport.md)). El
channel rack, el playlist y el piano roll siguen usando `on_hover_text`
a secas. Cámbialos a `theme::hint`, y haz que la barra de ayuda muestre
el zoom actual o el nombre del patrón cuando no haya nada bajo el ratón.
¿Por qué `theme::hint` recibe el `Response` por valor y lo devuelve, en
lugar de recibir `&Response`?

*Prácticas:* la propiedad a través de una función, el `Response` de
egui, leer el módulo de otro antes de cambiarlo.

## 10. Escuchar mejor (lineup-core, medio)

Oír acordes acierta cerca del 91% de la Library cuando cada estilo se
toca de todas las formas (ver [Un ritmo nuevo para una canción](new-rhythm.md)).
Escribe esa comprobación como una prueba propia: recorre `STYLES` en
bucle, toca cada chart con cada `Comping` y unas cuantas líneas de
`Bass` usando `chart::accompany`, llama a `hear_chords`, y cuenta
cuántas veces vuelven los acordes tal como están escritos. Imprime las
combinaciones que más fallan. Luego intenta subir el número: cambia una
constante al principio de `listen.rs`, o escucha más de la línea de bajo
(una nota de bajo mantenida un pulso entero es probablemente una nota
del acorde). Mantén pasando `every_style_is_heard_as_its_chart`.
Ejecútalo con `cargo test --release`: en modo release la misma prueba
corre muchas veces más rápido.

*Prácticas:* medir antes de cambiar, bucles anidados sobre enums, afinar
con pruebas.

## 11. Un cambio de tempo en un patrón (lineup-core, pequeño)

`TempoMark` pone las marcas de una demo en el playlist (ver [Cambios de
tempo, y tocar como una persona](tempo-and-feel.md)). La Toccata in D
minor tiene seis fermatas en los compases que toca su demo, dejadas
fuera porque la demo ya escribe esas notas largas. Acorta las notas a lo
que escribió Bach, añade las fermatas con `.with_tempo`, y escucha las
dos versiones. Cuenta el paso de cada fermata desde el principio de la
parte. ¿Sigue pasando `every_demo_keeps_a_sensible_tempo`? ¿Qué haría
falta para que fallara?

*Prácticas:* slices `const` de enums, contar pasos en un compás.

## 12. Apoyarse en la nota más alta (lineup-core, medio)

El modelado de la melodía en `library/interpret.rs` da a una nota hasta
seis pasos de velocity por su altura en la frase. Los cantantes se
apoyan a menudo en la única nota *más alta* de una frase más que en las
notas cercanas. Dale a esa nota dos pasos más, y escribe primero la
prueba: en «Twinkle, twinkle», ¿qué nota es, y en qué frase está?

*Prácticas:* `Iterator::max_by_key`, escribir una prueba antes del
código.
