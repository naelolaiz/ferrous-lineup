# Un recorrido por la aplicación

Este capítulo es el manual de la aplicación en una página: qué hace cada
ventana, y las teclas y los menús que la manejan. Antes era el README
del repositorio de la aplicación. Los capítulos de cada paso cuentan
cómo se construyó cada parte y por qué; este solo dice cómo usarla.

## La reproducción, el playlist y el channel rack

Pulsa **Space** para reproducir la canción de demostración: dos
compases de una progresión de acordes Do, Lam, Fa, Sol, luego entra un pulso,
y un relleno de batería la termina. El botón de reproducción pausa
mientras suena, y Stop vuelve a donde se puso el cabezal de
reproducción. **Home** va al principio, **End** al último paso, y
**Ctrl+Home/End** también funcionan (⌘ en Mac). Estos saltos muestran el
destino incluso con **Follow playhead** desactivado. **Page Up** y
**Page Down** saltan un compás hacia atrás o hacia delante, y hacer clic
en los números de compás mueve el cabezal de reproducción. El playlist
ordena los patrones hasta formar una canción: haz clic para colocar un
clip del patrón seleccionado, arrastra los clips, haz clic derecho para
borrar, y haz doble clic en un clip para editarlo: las melodías y los
acordes se abren en el piano roll, la batería en el channel rack.
Arrastra el borde derecho de un clip para repetir su patrón. Arrastra
por la franja que hay encima de los números de compás para marcar
secciones (Intro, Verse, Chorus…), y haz clic en una sección para
renombrarla, reproducirla varias veces seguidas, hacerle loop o
copiarla. Arrastra por los números de compás para hacer loop de una
parte de la canción. Haz clic derecho en el nombre de una pista para
insertar una pista encima o debajo, subirla o bajarla, o borrarla.
**F6** muestra u oculta el channel rack, donde haces clic en los pasos y
sueltas archivos WAV para usar tus propios sonidos. Sus controles
**Play pattern**, **Pause pattern** y **Stop pattern** reproducen
directamente el patrón seleccionado. Mientras suena la canción,
**Play over the song** reproduce el patrón encima de ella, desde el
siguiente compás y a su tiempo, hasta que vuelves a hacer clic: una
manera de probar un patrón con el resto de la canción. El selector de
patrones cambia lo que se está editando (y lo que suena encima de la
canción). Después de hacer clic en el rack o en el piano roll,
**Space** reproduce o pausa ese patrón; mientras suena la canción,
primero la detiene. Haz clic en el nombre de un canal para seleccionarlo
y oírlo; haz clic otra vez, haz clic en un hueco vacío del rack o pulsa
**Escape** para deseleccionarlo. **Delete** o **Backspace** quita el
canal seleccionado del rack; Undo lo devuelve. Haz doble clic en su
nombre para abrir su piano roll. Un canal seleccionado explícitamente
sigue seleccionado aunque no tenga notas en el patrón actual. **F7**
muestra el piano roll, sobre el patrón seleccionado: cuando el canal no
tiene notas ahí, lo dice y ofrece los patrones que sí las tienen. Elige
una tonalidad para resaltar sus notas, dibuja notas o acordes enteros, y
mira los nombres de los acordes encima de la cuadrícula. Haz clic en una
nota existente para seleccionarla; **Delete** o **Backspace** quita las
notas seleccionadas. Añade o quita notas de la selección con Ctrl+clic,
o selecciona un rango con una caja de Ctrl+arrastrar, y luego muévelas
con las teclas de flecha (Alt+↑ se mantiene en la tonalidad),
inviértelas o conviértelas en arpegios desde el menú Tools, y ajusta las
velocidades en el carril de debajo de la cuadrícula. Los botones de
acordes se iluminan con los acordes que suelen venir después. **Ctrl+S**
guarda el proyecto como un archivo `.lineup` (⌘S en un Mac), **Ctrl+Z**
deshace cualquier cambio y **Ctrl+Shift+Z** lo rehace, y **File > Export
WAV…** guarda la canción como un archivo de sonido. **File > Open
recent** enumera los últimos diez proyectos. Si la aplicación no se
cerró limpiamente, ofrece la copia de recuperación que escribió al
arrancar; [Sin perder nada](step-9/recovery.md) explica ambas cosas.

## MIDI

El menú **MIDI** conecta un teclado MIDI, que toca el canal seleccionado
en el channel rack, y un puerto de salida para los canales **MIDI Out**,
que tocan un sintetizador externo. También puedes enviar allí un canal
cualquiera: su menú de instrumento, en el mixer o en el clic derecho del
rack, empieza con **Send over MIDI**, donde eliges su canal MIDI (del 1
al 16) y la salida. Conserva su instrumento, enviado como un program
change, y una batería sale como su nota de un juego de batería General
MIDI. **Play it here instead** devuelve el sonido propio de la
aplicación. ¿No tienes teclado MIDI? Activa **Typing keyboard to piano**
ahí, y las teclas de letras tocan notas. La barra de estado pone nombre
al acorde que mantienes pulsado, y **● Rec** graba lo que tocas en el
patrón mientras hace loop. **File > Export song as MIDI…** escribe un
archivo `.mid` que otros estudios pueden abrir.

## El tempo y el toque humano

La **tempo strip** (franja de tempo), arriba del playlist, dibuja el
tempo de la canción como una línea. Haz clic en ella para cambiar el
tempo a partir de ese compás, de golpe o gradualmente a lo largo de un
pulso a ocho compases, que es como se escribe un ritardando (ir más
despacio) o un accelerando (ir más deprisa); las exportaciones WAV y
MIDI también reproducen los cambios. **Loose**, a su lado, hace que las
notas caigan unos pocos milisegundos fuera de la cuadrícula, como cuando
toca una persona (desactivado por defecto). Las demos suenan como una
interpretación: los cambios de tempo y las fermatas que marcan sus
partituras, una pequeña desaceleración al final de las canciones y los
himos, y unas velocidades moldeadas por el pulso y la frase.

## Las harmony keys, la Library y las demos

Abajo del todo están las **harmony keys**: la tonalidad y el compás del
proyecto, y una tecla grande para cada acorde de la tonalidad (de I a
vii°), coloreada según lo que hace: hogar, lejos o tensión. Mantén una
pulsada, o pulsa de **1** a **7**, para tocar su acorde; se iluminan los
acordes que suelen venir después, y mientras suena la música, también se
ilumina el acorde que oyes. En una tonalidad menor, **Edit > Settings…
> Harmony** elige cómo se escriben los numerales de los grados tercero,
sexto y séptimo: bIII bVI bVII, contados desde la escala mayor como
hacen los libros de pop y jazz, o III VI VII, contados desde la escala
menor como hacen los libros clásicos. La pestaña **Library**, junto a la
lista de patrones, contiene una lista alfabética de **Presets** que
combina los 177 presets de armonía, los 112 presets de ritmo y tus
estilos guardados. **Edit > Settings… > Library > Style list** puede, en
su lugar, agruparlos por género, con un grupo aparte de **User styles**.
Las entradas equivalentes de estilo y ritmo se combinan; los patrones de
interpretación distintos siguen disponibles. Cubren pop, rock, blues,
jazz, soul, hip hop, música electrónica, latina, tango, flamenco,
reggae, folk y música clásica. Las casillas bajo la búsqueda dejan solo
los presets parecidos a la canción: en su compás (**In 4/4**), hechos
para más o menos su tempo (**Near 120 BPM**, como mucho 1,15 veces más
rápido o más lento) y **With chords** (los que traen una progresión).
Sus etiquetas muestran el compás y el tempo propios de la canción. Haz
clic en uno para revisar sus opciones de copia, y luego en **Apply**
para oírlo en tu tonalidad. **Ctrl-clic** (también ⌘-clic en un Mac)
aplica las opciones anteriores inmediatamente. Haz clic derecho y elige
**Play the whole song with this preset** para aplicarlo en toda la
canción mientras está detenida. La melodía se queda donde está.
**File > Demos** abre 343 canciones con sus melodías, por género y en
orden alfabético: desde la *Oda a la alegría*, *Für Elise* y la
*Habanera* hasta tangos, canciones latinoamericanas, temas de swing y
big band, danzas en métricas irregulares y piezas orquestales como
*Boléro* y *La consagración de la primavera*, más temas nuevos escritos
para lucir el blues, el tango, el flamenco y más. También reproduce cada
estilo como una canción corta. Escribe unas cuantas letras de un título,
compositor, género, tonalidad o compás nada más abrirse el menú para
buscarlos todos. Una demo de canción se abre con el chord sheet delante;
**File > New beat** se abre con el channel rack en su lugar.

La Library agrupa sus opciones de aplicación en acompañamiento, armonía
y tiempo: **Playing patterns**, **Voices / instruments**, **Time
signature**, **Chord progression**, **Tempo** y **Swing**. Estas
aparecen en el popup al hacer clic en un preset, preseleccionadas con
las últimas opciones confirmadas. **Apply** las recuerda entre
ejecuciones; **Cancel** descarta los cambios del popup. **Edit >
Settings… > Library** muestra los valores por defecto como un conjunto
compacto de conmutadores clicables; **Edit defaults…** abre su editor
completo. Guardar los valores por defecto cambia la siguiente aplicación
de estilo y deja la canción actual como está. Solo cambian los
componentes marcados. Las entradas marcadas con **+ chords** incluyen
una progresión; las demás dicen **keeps chords** y muestran la casilla
de progresión desmarcada y desactivada. Aplicar un preset sin acordes
conserva la elección de progresión recordada para el siguiente preset
con acordes. La progresión de acordes está desactivada al principio;
márcala para copiar también el cuadro de acordes de un preset. Una
audición vacía usa acordes de ejemplo para demostrar el preset. La lista
combinada mantiene disponibles todos los patrones de interpretación y
elecciones de instrumento originales.

La parte superior de la Library dice de dónde viene cada parte de la
canción: patrones, instrumentos, compás, acordes, tempo y swing. Abrir
una demo trae todas las partes de ella; aplicar un preset trae las
partes marcadas, incluso una que ya era igual. Cada línea nombra la
demo, el estilo, el ritmo o el estilo guardado, dice **edited, was …**
una vez que cambias esa parte tú mismo, o **yours** cuando nada la ha
traído. Un compás elegido con el selector **Time** es una edición tuya;
una parte que solo se mueve junto con otra, como el tempo que se
conserva con un compás nuevo, mantiene su origen. El popup muestra lo
mismo junto a cada casilla (**now: …**), para que veas lo que
reemplazaría. Deshacer, rehacer y guardar conservan los orígenes.

## Bloquear un canal

El candado junto al mando de volumen de cada canal en el channel rack
bloquea ese canal: los presets, el selector **Time** y la banda
conservan sus notas y su instrumento, y cambian el resto de la canción.
Una Keys bloqueada conserva los acordes de la canción, y el bajo los
sigue; una batería bloqueada conserva sus notas mientras las demás
baterías adoptan el ritmo nuevo. Cuando la banda entra en una canción
cuya Keys o Bass propia está bloqueada, le deja esa parte. Un compás
nuevo sigue encajando las notas de un canal bloqueado en los nuevos
compases, y todavía puedes editarlo a mano. La Library y el popup
enumeran lo que está bloqueado (**🔒 Kept: …**).

## Más opciones de la Library

En modo canción, copiar **Playing patterns** y **Voices / instruments**
trae la banda completa del preset, incluida su guitarra, cuerdas u
órgano extra, incluso cuando la demo todavía no tiene banda en vivo.
Deja **Chord progression** desactivada para conservar los acordes
exactos de la canción y cambiar solo su acompañamiento. Activarla
reemplaza la armonía por el cuadro del preset conservando la melodía.

El menú **Progression** del chord sheet también usa el navegador de la
Library, con los mismos nombres de preset, búsqueda y preferencia de
agrupación. Busca por nombre, género o compás; la búsqueda se mantiene
entre vistas, y **×** la limpia. **Progression** muestra los presets de
armonía que incluyen una progresión de acordes.

Selecciona **Plain accents** para tocar acentos de acordes y de bajo
sobre los grupos de pulsos de la métrica con los instrumentos actuales.
Esto funciona con todos los presets, incluidos los estilos guardados.
**Drums and plain accents** aplica la batería del preset con acentos
sencillos de acordes y de bajo. La opción de instrumento se oculta
conservando su valor recordado; **Preset patterns** restaura la copia
normal del estilo. El selector **Time** y el menú de compás del compás 1
también muestran directamente las casillas de estilo, ritmo/acento,
instrumento y swing correspondientes. Desactivar **Use matching style**
ahí usa un ritmo de batería sencillo y acentos con altura en la métrica
pedida. Los cambios de métrica repetidos usan la canción original como
referencia hasta que editas el proyecto explícitamente; volver a su
métrica original restaura su interpretación exacta. Deshacer y rehacer
conservan la referencia correspondiente. La Library muestra los patrones
de la banda y sus instrumentos como partes separadas; el chord sheet
sigue siendo la fuente de los acordes de la canción. Estas elecciones de
la banda sobreviven a guardar y a deshacer/rehacer. Un cambio de métrica
conserva la duración de cada compás ajustando el tempo.

## El editor de estilos

Usa **Edit style…** o **New style…** en el channel rack (también
**View > Style editor…**) para editar partes repetidas de batería,
acordes y bajo. [El editor de estilos](step-9/style-editor.md) cuenta
cómo funciona. El editor reutiliza el rack y el piano roll, con su
propio historial de deshacer y una vista previa de cuatro repeticiones.
**Start from…** copia un estilo existente; hacer clic derecho en un
estilo o ritmo de la Library también abre una copia editable. Las
plantillas con altura están escritas sobre do mayor y siguen los acordes
de la canción al aplicarse. Guárdalas como archivos `.lineup-style`;
**Settings > Library > Styles folder…** elige la carpeta que se carga en
**Presets** en el navegador compartido. **Start from…** usa esa misma
lista, incluidos los presets de ritmo. Los estilos aplicados se incrustan
en los archivos de proyecto, así que reabrir una canción conserva sus
patrones personalizados.

## El mixer

El botón del mixer en la barra de herramientas (tres faders),
**View > Mixer** o **F9** abren el mixer. Sus canales se enumeran en el
orden de la partitura de un director, por secciones, con toda la batería
junta. Cada uno tiene su instrumento (haz clic para elegir otro, o para
enviar el canal por MIDI), un medidor de nivel, un volumen, un paneo de
L100 a R100 (doble clic para el centro), cuánto de él va a la reverb,
mute y solo. El master tiene un medidor por cada altavoz. Funcionan
tanto con sonidos SoundFont como Lineup Osc, además de muestras. La
reverb estéreo del master tiene controles de wet/dry, decay, damping y
pre-delay. Los ajustes del mixer se guardan con el proyecto, admiten
deshacer y se usan para la exportación WAV.

## Alturas de pista, selecciones y atajos

Redimensiona una pista individual del playlist arrastrando el borde
inferior de su cabecera de nombre. Haz doble clic en ese borde para
restablecerla, o usa **Track height** en el menú de clic derecho de la
cabecera. Las alturas siguen los movimientos de las pistas, se guardan
con el proyecto y admiten Undo.

**F1**, o **Help > Keyboard shortcuts**, enumera todas las teclas a las
que responde la aplicación, ventana por ventana. La misma lista está en
el [apéndice de atajos de teclado](appendix-shortcuts.md), para
imprimir.

**Follow playhead** en la barra de herramientas y **Settings > Playback**
mantienen visible la posición de reproducción en el playlist, el chord
sheet y el piano roll. Desactivarlo te deja desplazarte libremente; la
preferencia se recuerda. Los atajos del editor siguen a la ventana
activa: **Ctrl+A** selecciona sus compases, notas o clips. El piano roll
admite **Ctrl+C/X/V** y **Ctrl+D** para notas, incluido pegar entre la
canción y el editor de estilos; su menú Tools enumera estas acciones.
Pegar o duplicar notas amplía el patrón cuando hace falta. El playlist
también admite **Ctrl+C/X/V**, **Ctrl+D**, **Delete** y **Ctrl-clic**
para añadir o quitar un clip de la selección. Ctrl-clic en un hueco
vacío del playlist fija la posición de pegado sin colocar un clip.
**Ctrl-arrastrar** sobre un hueco vacío del playlist selecciona una caja
de clips; arrastrar un clip seleccionado mueve juntos los clips
seleccionados. **Ctrl-clic** en un marcador de sección (Intro, Verse,
Chorus…) selecciona sus clips superpuestos, o elige **Select clips in
this section** en su menú. Vuelve a hacer Ctrl-clic o pulsa **Escape**
para limpiar la selección. En el chord sheet, hacer clic en el
encabezado de una sección selecciona todos los compases de esa sección;
volver a hacer clic los desmarca. **Shift-clic** o **Shift+flechas**
selecciona un rango de compases, y **Escape** lo limpia. La selección se
queda con los clips cuando se borran clips anteriores. Los popups
abiertos y los controles de configuración conservan sus propias teclas.

## El chord sheet y la banda

**F5**, o el primer botón de ventana de la barra de herramientas (una
página de nombres de acordes), abre el **chord sheet**: la armonía de la
canción compás a compás, cada sección bajo su nombre, como lo escribe un
lead sheet. **Bars per line** pone en una línea tantos compases como
muestran todos los nombres completos, a su tamaño completo (1, 2, 4, 8,
12 o 16, de modo que las líneas empiecen donde empiezan las frases), o
el número que elijas. Haz clic en un compás y escribe su armonía:
acordes (`Am7 D7`, `G7/B`, `F#m7b5`), números romanos en la tonalidad
(`ii7 V7`), conjuntos de clases de alturas (`3-11 T2`, `{0,1,4}`),
micromodos (`Menor 3`), formas de serie dodecafónica (`P0`, `RI7`) o
`N.C.`; una `/` mantiene la armonía anterior un pulso más (`C / / G7`).
Bajo cada acorde está su numeral en la tonalidad, con las cifras de una
inversión (V⁶₅), y bajo cada conjunto sus notas; mantén pulsado el ratón
sobre una armonía para oírla. **Write the chords the song plays** rellena
un chord sheet vacío a partir de lo que toca la canción, con la nota del
bajista después de la barra, y en un conjunto o una serie con los
conjuntos o micromodos oídos. Mientras suena la canción, el compás que
suena queda enmarcado en blanco. **Show**, arriba del chord sheet,
nombra la misma armonía **As written**, como **Chords**, como
**Pitch-class sets** (do mayor es 3-11 T7I) o como **Micromodes**
(Acorde menor-mayor T7I), sin cambiar una nota. Para conjuntos y
micromodos también puede leer las notas de la canción, **together**
(verticalmente) o **in a line** (en serie), en grupos de 2 a 12 notas,
de todas las pistas o de una. El chord sheet se guarda con el proyecto y
sigue los cambios de tonalidad, de métrica y las secciones copiadas. Haz
clic derecho en un compás para **Insert a bar before**, **Insert a bar
after** o **Delete this bar** (o los compases seleccionados): todo lo
que hay después se desplaza —notas, acordes, marcadores y secciones—, y
un compás nuevo queda vacío, así que la armonía anterior continúa a
través de él. Los números de compás de la regla del playlist tienen las
mismas opciones. El chord sheet es una ventana encima del playlist, o,
con **⛶ Fill the window** (también **View > Chord sheet in the main
window**), ocupa el lugar del playlist en la ventana principal, con
líneas más largas; **Back to the playlist** lo devuelve.

**File > New** abre un chord sheet vacío con una **banda**: escribe
acordes y un piano, un bajo y una batería los tocan, escritos como
patrones en sus propias pistas del playlist. **🎷 Band** elige la
formación de instrumentos, la voz de cada músico y qué músicos se oyen.
Estos controles conservan los patrones de interpretación, acordes,
métrica, tempo y swing actuales. **No band** quita el acompañamiento.
Elige ritmos y patrones de interpretación en la **Library**; un vals
puede poner la canción en 3/4 al copiar su compás. **Progression**
escribe una progresión conocida desde el compás seleccionado, en la
tonalidad de la canción, y **End the song here** (clic derecho en un
compás) dice dónde se detiene la banda. La banda vuelve a tocar cada vez
que cambian el chord sheet, la tonalidad o los compases; una parte que
editas en el piano roll se queda como la dejaste hasta que haces clic en
**Play it again**. Elegida sobre una demo, la banda toca los acordes de
la demo y silencia su propio acompañamiento hasta que eliges
**No band**. **File > New beat** es el antiguo patrón vacío con una
batería.

Para practicar con la banda, la segunda fila del chord sheet tiene un
**Count-in** (un compás de clics antes de que empiece la música), un
**Metronome**, la **Speed** a la que tocar (del 25% al 200% del tempo de
la canción, que la propia canción conserva) y **Tap**, que fija el tempo
a partir de unos cuantos clics sobre el pulso. El **Speed trainer**
arranca la banda más despacio (70%) y toca un poco más rápido cada vez
que el loop vuelve a empezar, hasta el tempo propio de la canción; Stop
lo arranca de nuevo desde la velocidad lenta. **You play** deja a un
músico fuera de la banda (el piano, el bajo, la batería, o una guitarra,
cuerdas u órgano) para que toques tú la parte; entonces tu teclado toca
el sonido de esa parte. Haz Shift+clic en un segundo compás para
seleccionar varios, y **Loop bars** los reproduce una y otra vez. Los
clics nunca llegan a una canción guardada ni a una exportación WAV.
**Follow playhead**, **Count-in** y **Metronome** son también botones
conmutadores iluminados junto al transporte principal. Los ajustes de
práctica también están en **Edit > Settings… > Playback**.

La fila **Form**, encima, da forma a toda la interpretación. **Intro**
pone 2, 4 u 8 compases delante de la canción, un turnaround que lleva a
su primer acorde (ii–V, o I–vi–ii–V), escrito en el chord sheet para que
puedas cambiarlo. **Choruses** reproduce la melodía ese número de veces,
mostrada una sola vez entre signos de repetición en el chord sheet y el
playlist. **Ending** añade dos compases sobre la tónica de la tonalidad
después del último estribillo: uno en el estilo, y luego el acorde
golpeado una vez y sostenido, con un golpe de platillo.

Haz clic derecho en un compás (o antes Shift+clic para seleccionar
varios) para elegir **Variation a** o **Variation b** del estilo desde
ahí. La variación b es el mismo estilo tocado más lleno, como cuando una
banda se entrega en un estribillo o un puente: la batería pasa del
hi-hat al ride, un bajo o un piano más ocupados donde el estilo los
tiene (el bajo de una balada de jazz camina, el piano de una balada pop
toca corcheas), un poco más fuerte. La batería toca un relleno en cada
cambio y golpea el platillo en su primer pulso, y una letra en un
recuadro lo marca en el chord sheet y en el playlist.

La opción **Style** del mismo menú toca los compases en otro estilo: una
estrofa como bossa nova y el puente como swing. Haz clic derecho en el
nombre de una sección en el chord sheet para elegirlo para toda la
sección. La banda conserva sus instrumentos y toca el comping, la línea
de bajo, la batería y el swing del nuevo estilo, con un relleno hacia el
cambio. El nombre del estilo marca dónde empieza, y **The song's style**
vuelve atrás. Solo se ofrecen estilos en el compás de esos compases.

**Swing**, junto al tempo en el chord sheet (y en el playlist), toca
pares de corcheas largo-corto, como hacen los músicos de jazz, blues y
shuffle: desde *straight* (parejas) hasta el 100%, los dos primeros
tercios y el último tercio de un tresillo. Las notas se quedan como
están escritas; solo cambia cómo se tocan, también en las exportaciones
WAV y MIDI. Cada estilo trae su propio swing (Swing, Jazz ballad, Jazz
waltz y Shuffle 100%, Rock and roll 50%), y recuerda la cantidad que
fijaste por última vez para él.

Muchos estilos traen más músicos que el piano, el bajo y la batería. Una
**guitarra** toca las posiciones que tocaría un guitarrista (el Do o Sol
abiertos de un libro de acordes, acordes con cejilla en otras
tonalidades, el bajo más grave bajo un acorde con barra) y las rasguea:
púas hacia abajo a lo largo de las cuerdas, púas hacia arriba más
ligeras en las cuatro agudas. El rock toca power chords, una banda de
swing el "four to the bar" de la guitarra, el shuffle la línea de
boogie, el reggae y el soul un chop corto en los pulsos 2 y 4. Las
**cuerdas** sostienen los acordes en baladas, tangos y disco, y un
**órgano** burbujea en el reggae o rellena un estribillo de rock;
algunos solo se suman en la variación b. Cada uno tiene su propio
instrumento General MIDI y su pista, y el menú de estilo dice quién
toca.

La banda también se detiene donde se detiene la música. Un punto después
de una armonía es un **shot** (`C.`: la banda lo golpea una vez, corto, y
se detiene hasta la siguiente armonía), dos puntos un **hold** (`C..`:
golpeado y sostenido) y tres un **rest** (`C...`: silencio). Un signo de
intercalación antes es un **push** (`^C`): la banda lo toca una corchea
antes y lo sostiene sobre el pulso, y `^^C` llega una semicorchea antes.
La batería se detiene con la banda y golpea un bombo y un platillo con
cada shot, hold y push. Haz clic derecho en un compás y elige
**Band on Do** para fijar las mismas marcas desde un menú.

**Soloist (experimental)**, al final del menú de la banda, añade un
solista que improvisa sobre el estribillo: un saxo tenor en el jazz, una
armónica en el blues, un violín en el folk, etcétera. La línea pone
notas del acorde en los pulsos y rellena el espacio entre ellos con
notas de la escala del acorde (elegidas de la tonalidad, de modo que en
do mayor Sol7 toca Sol mixolidio y Mi7, que lleva a Lam, toca la escala de la
menor armónica desde Mi: Mi frigio dominante) o con notas cromáticas a un
semitono de distancia. Respira entre frases, y **Another solo** toca uno
nuevo. Abre su parte en el piano roll para ver qué es cada nota: las
notas del acorde en verde con su grado (R 3 5 7), las notas de la escala
en azul (de paso, vecinas), las notas cromáticas en naranja; señala una
nota para que te la explique.

## Chord sheets como texto

**File > Import chord sheet…** lee un cuadro de acordes escrito como
texto, de la manera en que los cuadros circulan por correos y foros (un
archivo `.txt`, o suéltalo sobre la ventana): líneas de compás entre los
compases, el nombre de una sección entre corchetes sobre sus compases, y
unos cuantos ajustes arriba si quieres.

```text
Title: Autumn Song
Key: C major
Tempo: 120
Style: Bossa nova

[Verse ×2]
| C  | Am7 | Dm7 G7 | C |
| F  | %   | G7     | C |
```

El cuadro se convierte en una canción nueva con la banda tocándola y el
chord sheet abierto; un error se indica con su línea. **File > Export
chord sheet…** escribe el chord sheet de la canción de la misma manera,
alineado en columnas, y en el chord sheet **Ctrl+C** y **Ctrl+V** copian
y pegan los compases seleccionados como texto, hacia y desde un editor
de texto u otra canción.

## Series dodecafónicas, conjuntos de clases de alturas y micromodos

Para la música sin tonalidad de referencia, los menús de tonalidad
también ofrecen una **twelve-tone row** y un **pitch-class set**.
**Row…** en el piano roll abre la ventana dodecafónica: escribe, dibuja
o elige una serie (las series de la Suite op. 25 de Schoenberg, la
*Lyric Suite* de Berg, y la Sinfonía op. 21 y el Concierto op. 24 de
Webern, todos en dominio público ya, o unas cuantas escritas para
Ferrous Lineup), mira su matriz con las 48 formas (P, I, R y RI sobre
cada nota), y escribe cualquiera de ellas en el patrón como melodía o
como acordes, con las duraciones, dinámicas y octavas siguiendo también
la serie si quieres. **Write … into current pattern** reemplaza las
notas del instrumento seleccionado en ese patrón. **Save to new
pattern** crea y selecciona un patrón aparte, usando las mismas opciones
de escritura de la serie, longitud y compás, y deja intacto el patrón de
origen. Ambas acciones admiten Undo. **Sets…** enumera los 224 conjuntos
de clases de alturas por número de Forte, con sus formas primas y
vectores interválicos, para dibujar con ellos o usarlos como escala, y
al seleccionar notas pone nombre a su conjunto. **Hear it** preescucha
el conjunto o micromodo elegido en el instrumento seleccionado.
**Draw with it** en Sets y Micromodes se ilumina mientras está activo;
haz clic de nuevo o elige **Single note** para dejar de dibujar el
conjunto. **Draw > Single note** del piano roll hace lo mismo.
**Twelve-tone rows** de la Library y **File > Demos > Twelve-tone**
reproducen siete maneras de usar una serie.

**Micromodes…** abre los doce micromodos de Francisco Kröpfl: los doce
grupos de tres notas (los tricordios de Forte, 3-1 a 3-12) en sus tres
familias, menor, mayor y acordes. Cada uno se dibuja sobre un reloj, con
su forma prima en solfeo, sus intervalos y su hora en el Tone Clock de
Peter Schat, y se puede oír, dibujar con él o usarlo como tonalidad.
Seleccionar notas muestra qué micromodos contienen, y las ventanas
Micromodes y Sets se enlazan entre sí: un conjunto de tres notas es un
micromodo, y todo conjunto mayor está hecho de ellos. **Micromodes
(Kröpfl)** de la Library y **File > Demos > Micromodes** reproducen
música hecha a partir de un micromodo.

## Importar grabaciones y archivos MIDI

**File > Import recording…** abre un archivo MP3, WAV, FLAC u Ogg (o
suéltalo sobre la ventana) y lo escucha: el tempo, el compás y la
tonalidad se fijan a partir de la grabación, que suena en su propio
canal, y un canal Harmony toca el acorde oído en cada pulso, de modo que
el piano roll y las harmony keys lo siguen. Es un experimento temprano;
mira
[`crates/lineup-listen`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-listen/README.md)
para ver cómo funciona y con qué frecuencia acierta.

**File > Import MIDI file…** abre un archivo `.mid` (o suéltalo sobre la
ventana) como un proyecto nuevo: cada pista se convierte en un canal (la
batería del canal 10, un canal por tambor), cada nota conserva su
velocidad y su momento exacto, incluso entre dos pasos, y los cambios de
tempo, los compases y la tonalidad vienen con él. El carril Chords del
piano roll y las harmony keys ponen nombre a los acordes mientras suena.

## Sonidos, entrenamiento auditivo y el lead sheet

**Edit > Settings… > Audio > SoundFont instruments** reproduce
grabaciones de instrumentos reales en lugar del Lineup Osc: cada canal
con un instrumento General MIDI (las voces de las demos, el piano y el
bajo de la Library, los acordes de la canción de demostración) y, con
**SoundFont drums**, la batería. La primera vez, se ofrece a descargar
[GeneralUser GS](https://github.com/mrbumpy409/GeneralUser-GS), un
SoundFont General MIDI gratuito de S. Christian Collins (unos 30 MB,
guardado para la próxima vez). **Choose SoundFont…** reproduce cualquier
otro archivo `.sf2` en su lugar. La exportación WAV también lo usa. La
categoría Audio también muestra el dispositivo de salida activo, la
frecuencia de muestreo y el número de canales.

**View > Ear training** (experimental) hace preguntas para responder de
oído: qué intervalo forman dos notas, qué tipo de acorde suena (con
inversiones si quieres), y qué acordes de una tonalidad toca la banda,
nombrados en las harmony keys con los nombres de acordes ocultos. Cada
respuesta se explica con sus notas escritas, y cualquier respuesta se
puede reproducir junto a la correcta. La canción y su archivo se quedan
como están.

**View > Lead sheet** escribe la canción como la lee una banda: la
melodía en notación sobre un pentagrama, los símbolos de acordes encima,
y la tonalidad y los compases, secciones y repeticiones, dispuestos en
páginas La4 o Letter. La melodía es el canal llamado Lead (o si no, el
más agudo; se puede elegir cualquier otro), y cada nota se escribe para
la tonalidad y el acorde que suena, de modo que un Sol♯ sobre Mi7 sigue
siendo Sol♯ y un La♭ sobre Fam sigue siendo La♭. **None: chords only**
escribe barras de ritmo bajo los acordes en su lugar. **For** escribe el
lead sheet para instrumentos en Si♭, Mi♭ o Fa, de modo que un trompetista,
un saxofonista o un trompista lo lee en su propia tonalidad: la
armadura, la melodía y los acordes se mueven todos. Mientras suena la
canción, se ilumina el compás que suena. **Export PDF…** guarda el lead
sheet para imprimirlo o compartirlo, y **Print…** lo abre en tu visor de
PDF. La música se dibuja con la fuente Bravura y las palabras con Inter,
dos fuentes gratuitas que vienen con la aplicación.

## El zoom y los ajustes

¿Demasiado pequeño o demasiado grande? Las mismas teclas hacen zoom
sobre lo que tenga el puntero encima. Sobre el playlist o el piano roll,
**Ctrl** con **+** y **−** o con la rueda del ratón (⌘ en lugar de Ctrl
en un Mac) hace los compases y los pasos más anchos o más estrechos, y
**Alt** con las mismas teclas o con la rueda hace las pistas y las
teclas más altas o más bajas, alrededor de lo que esté bajo el puntero.
Sobre el channel rack hacen zoom de su único tamaño. En cualquier otro
sitio (las barras de herramientas, las harmony keys, la barra de estado,
el panel Patterns y Library) hacen más grandes o más pequeños el texto,
los botones y los menús de todas partes, como hace **Settings >
Appearance > Interface size**. **Ctrl+0** devuelve al 100% lo que esté
bajo el puntero. Los dos zooms del playlist también están en el extremo
derecho de la barra de estado. **Edit > Settings…** separa las
preferencias en las categorías **Appearance**, **Harmony**, **Library**,
**Audio** y **Playback**. Appearance contiene el tamaño de la interfaz y
todos los niveles de zoom del editor; Harmony contiene cómo se escriben
los numerales en tonalidades menores; Library contiene la agrupación de
estilos, los valores por defecto de estilos guardados y la carpeta de
estilos; Audio contiene la información de salida y los ajustes de
SoundFont; Playback contiene el seguimiento de la reproducción y los
controles de práctica. Los antiguos menús de zoom, Sounds y Practice
quedan reemplazados por estas categorías. Los controles de zoom
comparten los valores actuales con los atajos de teclado y de rueda, que
siguen funcionando como antes. El menú View también muestra u oculta
cada ventana y panel, y **Alt+F8** oculta el panel Patterns y Library
para dar más espacio al playlist. El zoom, el tamaño de la ventana de la
aplicación y dónde dejaste las ventanas se recuerdan para la próxima
vez. Una ventana se abre al tamaño de lo que muestra (el mixer, el
channel rack, los atajos) o, en el caso de una ventana en la que
trabajas (el piano roll, el chord sheet), a su propio tamaño la primera
vez después de arrancar la aplicación.

## Exportar vídeo, y programas ya compilados

**File > Export video…** guarda la canción como un archivo MP4, con las
vistas que elijas tocando a la vez: el chord sheet con el compás que
suena enmarcado, el lead sheet, el playlist, el piano roll y las harmony
keys, dispuestos en una vista previa, en HD o Full HD. Los codificadores
van integrados en la aplicación, así que no hace falta instalar nada
más.

Los programas pelados para los tres sistemas se adjuntan a cada
ejecución de CI como `ferrous-lineup-linux`, `ferrous-lineup-macos` y
`ferrous-lineup-windows`. El workflow **Release** compila los paquetes
listos para usar, con el README y la licencia: un AppImage de Linux,
`.deb` y `.tar.gz`; un Setup `.exe` de Windows y un `.zip` portable; y un
`.dmg` de macOS con `Ferrous Lineup.app`, para Apple Silicon e Intel. Se
adjuntan a cada ejecución de Release, y subir una etiqueta `v` (`v0.2.0`)
los publica en la
[página de Releases](https://github.com/naelolaiz/ferrous-lineup/releases)
del repositorio. Los programas de Windows no están firmados, y la
aplicación de macOS está firmada ad-hoc pero sin notarizar, así que
ambos avisan antes del primer arranque;
[Compilaciones de lanzamiento](step-9/releases.md) explica cada descarga
y los avisos.

## Comprobar la reproducción con una traza

Una traza enumera cada nota que toca el reproductor, una línea por nota:
cuándo empieza y cuánto dura en segundos y en pasos, su compás y su
pulso, el tempo y el compás en vigor, el canal, su programa General
MIDI, la altura y la velocidad, y cuánto ha llegado tarde después de su
paso (nudge, swing y temporización loose). Se escribe donde empiezan las
notas, en el hilo de audio, de modo que muestra lo que oyes y no lo que
dice la pantalla.

```sh
ferrous-lineup --trace song.lineup --out song.tsv   # a project file
ferrous-lineup --trace "Für Elise"                  # a demo, to the terminal
ferrous-lineup --demos                              # the demos' names
```

`--trace` reproduce la canción como hace File > Export WAV, sin ventana
ni tarjeta de sonido. Para trazar lo que tocas en la aplicación en su
lugar, marca **Write a trace while playing** en Settings > Playback:
cada reproducción, de Play a Stop, añade una tabla al archivo que se
muestra ahí. Las líneas que empiezan por `#` dicen qué contiene la tabla
y enumeran los cambios de tempo y de compás; el resto está separado por
tabuladores, para una hoja de cálculo, `awk` o `grep`. En Windows, usa
`--out`: el programa ya compilado no tiene terminal donde imprimir.
