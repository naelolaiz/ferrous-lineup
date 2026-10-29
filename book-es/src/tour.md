# Un recorrido por la aplicación

Este capítulo es el manual de la aplicación en una página: qué hace cada
ventana, y las teclas y los menús que la manejan. Los capítulos de cada
paso cuentan cómo se construyó cada parte y por qué; este solo dice cómo
usarla.

## La reproducción, la lista de reproducción y el rack de canales

Pulsa **Espacio** para reproducir la canción de demostración: dos
compases de una progresión de acordes C, Am, F, G, luego entra un pulso,
y un relleno de batería la termina. El botón de reproducción pausa
mientras suena, y ⏹ vuelve a donde se puso el cabezal de
reproducción. **Inicio** va al principio, **Fin** al último paso, y
**Ctrl+Inicio/Fin** también funcionan (⌘ en Mac). Estos saltos muestran el
destino incluso con **Seguir el cursor de reproducción** desactivado. **RePág** y
**AvPág** saltan un compás hacia atrás o hacia delante, y hacer clic
en los números de compás mueve el cabezal de reproducción. La lista de reproducción
ordena los patrones hasta formar una canción: haz clic para colocar un
clip del patrón seleccionado, arrastra los clips, haz clic derecho para
borrar, y haz doble clic en un clip para editarlo: las melodías y los
acordes se abren en el piano roll, la batería en el rack de canales.
Arrastra el borde derecho de un clip para repetir su patrón. Arrastra
por la franja que hay encima de los números de compás para marcar
secciones (Introducción, Estrofa, Estribillo…), y haz clic en una sección para
renombrarla, reproducirla varias veces seguidas, repetirla en bucle o
copiarla. Arrastra por los números de compás para repetir en bucle una
parte de la canción. Haz clic derecho en el nombre de una pista para
insertar una pista encima o debajo, subirla o bajarla, o borrarla.
**F6** muestra u oculta el rack de canales, donde haces clic en los pasos y
sueltas archivos WAV para usar tus propios sonidos. Sus controles
**Tocar patrón**, **Pausar patrón** y **Detener patrón** reproducen
directamente el patrón seleccionado. Mientras suena la canción,
**Tocar sobre la canción** reproduce el patrón encima de ella, desde el
siguiente compás y a su tiempo, hasta que vuelves a hacer clic: una
manera de probar un patrón con el resto de la canción. El selector de
patrones cambia lo que se está editando (y lo que suena encima de la
canción). Después de hacer clic en el rack o en el piano roll,
**Espacio** reproduce o pausa ese patrón; mientras suena la canción,
primero la detiene. Haz clic en el nombre de un canal para seleccionarlo
y oírlo; haz clic otra vez, haz clic en un hueco vacío del rack o pulsa
**Esc** para deseleccionarlo. **Supr** o **Retroceso** quita el
canal seleccionado del rack; Deshacer lo devuelve. Haz doble clic en su
nombre para abrir su piano roll. Un canal seleccionado explícitamente
sigue seleccionado aunque no tenga notas en el patrón actual. **F7**
muestra el piano roll, sobre el patrón seleccionado: cuando el canal no
tiene notas ahí, lo dice y ofrece los patrones que sí las tienen. Elige
una tonalidad para resaltar sus notas, dibuja notas o acordes enteros, y
mira los nombres de los acordes encima de la cuadrícula. Haz clic en una
nota existente para seleccionarla; **Supr** o **Retroceso** quita las
notas seleccionadas. Añade o quita notas de la selección con Ctrl+clic,
o selecciona un rango con una caja de Ctrl+arrastrar, y luego muévelas
con las teclas de flecha (Alt+↑ se mantiene en la tonalidad),
inviértelas o conviértelas en arpegios desde el menú Herramientas, y ajusta las
velocidades en el carril de debajo de la cuadrícula. Los botones de
acordes se iluminan con los acordes que suelen venir después. **Ctrl+S**
guarda el proyecto como un archivo `.lineup` (⌘S en un Mac), **Ctrl+Z**
deshace cualquier cambio y **Ctrl+Mayús+Z** lo rehace, y **Archivo > Exportar WAV…** guarda la canción como un archivo de sonido. **Archivo > Abrir recientes** enumera los últimos diez proyectos. Si la aplicación no se
cerró limpiamente, ofrece la copia de recuperación que escribió al
arrancar; [Sin perder nada](step-9/recovery.md) explica ambas cosas.

## MIDI

El menú **MIDI** conecta un teclado MIDI, que toca el canal seleccionado
en el rack de canales, y un puerto de salida para los canales **Salida MIDI**,
que tocan un sintetizador externo. También puedes enviar allí un canal
cualquiera: su menú de instrumento, en el mezclador o en el clic derecho del
rack, empieza con **Enviar por MIDI**, donde eliges su canal MIDI (del 1
al 16) y la salida. Conserva su instrumento, enviado como un program
change, y una batería sale como su nota de un juego de batería General
MIDI. **Tocarlo aquí en su lugar** devuelve el sonido propio de la
aplicación. ¿No tienes teclado MIDI? Activa **Teclado del equipo como piano**
ahí, y las teclas de letras tocan notas. La barra de estado pone nombre
al acorde que mantienes pulsado, y **⏺** graba lo que tocas en el
patrón mientras se repite en bucle. **Archivo > Exportar canción como MIDI…** escribe un
archivo `.mid` que otros estudios pueden abrir.

## El tempo y el toque humano

La **franja de tempo**, arriba de la lista de reproducción, dibuja el
tempo de la canción como una línea. Haz clic en ella para cambiar el
tempo a partir de ese compás, de golpe o gradualmente a lo largo de un
pulso a ocho compases, que es como se escribe un ritardando (ir más
despacio) o un accelerando (ir más deprisa); las exportaciones WAV y
MIDI también reproducen los cambios. **Soltura**, a su lado, hace que las
notas caigan unos pocos milisegundos fuera de la cuadrícula, como cuando
toca una persona (desactivado por defecto). Las demos suenan como una
interpretación: los cambios de tempo y las fermatas que marcan sus
partituras, una pequeña desaceleración al final de las canciones y los
himos, y unas velocidades moldeadas por el pulso y la frase.

## Las teclas de armonía, la Biblioteca y las demos

Abajo del todo están las **teclas de armonía**: la tonalidad y el compás del
proyecto, y una tecla grande para cada acorde de la tonalidad (de I a
vii°), coloreada según lo que hace: hogar, lejos o tensión. Mantén una
pulsada, o pulsa de **1** a **7**, para tocar su acorde; se iluminan los
acordes que suelen venir después, y mientras suena la música, también se
ilumina el acorde que oyes. En una tonalidad menor, **Edición > Ajustes… > Armonía** elige cómo se escriben los numerales de los grados tercero,
sexto y séptimo: bIII bVI bVII, contados desde la escala mayor como
hacen los libros de pop y jazz, o III VI VII, contados desde la escala
menor como hacen los libros clásicos. **Armonía > Nomenclatura de notas** nombra
las notas, las tonalidades y los acordes con letras (C, Am7) o en solfeo
(Do, Lam7), y **Apariencia > Idioma** muestra la app en inglés o en
español (ver [La app en español, y las notas en letras o en
solfeo](step-9/languages.md)). La pestaña **Biblioteca**, junto a la
lista de patrones, contiene una lista alfabética de **Preajustes** que
combina los 177 preajustes de armonía, los 112 preajustes de ritmo y tus
estilos guardados. **Edición > Ajustes… > Biblioteca > Lista de estilos** puede, en
su lugar, agruparlos por género, con un grupo aparte de **Estilos de usuario**.
Las entradas equivalentes de estilo y ritmo se combinan; los patrones de
interpretación distintos siguen disponibles. Cubren pop, rock, blues,
jazz, soul, hip hop, música electrónica, latina, tango, flamenco,
reggae, folk y música clásica. Las casillas bajo la búsqueda dejan solo
los preajustes parecidos a la canción: en su compás (**En 4/4**), hechos
para más o menos su tempo (**Cerca de 120 BPM**, como mucho 1,15 veces más
rápido o más lento) y **Con acordes** (los que traen una progresión).
Sus etiquetas muestran el compás y el tempo propios de la canción. Haz
clic en uno para revisar sus opciones de copia, y luego en **Aplicar**
para oírlo en tu tonalidad. **Ctrl-clic** (también ⌘-clic en un Mac)
aplica las opciones anteriores inmediatamente. Haz clic derecho y elige
**Tocar toda la canción con este preajuste** para aplicarlo en toda la
canción mientras está detenida. La melodía se queda donde está.
**Archivo > Demostraciones** abre 343 canciones con sus melodías, por género y en
orden alfabético: desde la *Oda a la alegría*, *Para Elisa* y la
*Habanera* hasta tangos, canciones latinoamericanas, temas de swing y
big band, danzas en métricas irregulares y piezas orquestales como
*Boléro* y *La consagración de la primavera*, más temas nuevos escritos
para lucir el blues, el tango, el flamenco y más. También reproduce cada
estilo como una canción corta. Escribe unas cuantas letras de un título,
compositor, género, tonalidad o compás nada más abrirse el menú para
buscarlos todos. Una demo de canción se abre con la hoja de acordes delante;
**Archivo > Nuevo ritmo** se abre con el rack de canales en su lugar.

La Biblioteca agrupa sus opciones de aplicación en acompañamiento, armonía
y tiempo: **Patrones de ejecución**, **Voces / instrumentos**, **Compás**, **Progresión de acordes**, **Tempo** y **Swing**. Estas
aparecen en el popup al hacer clic en un preajuste, preseleccionadas con
las últimas opciones confirmadas. **Aplicar** las recuerda entre
ejecuciones; **Cancelar** descarta los cambios del popup. **Edición > Ajustes… > Biblioteca** muestra los valores por defecto como un conjunto
compacto de conmutadores clicables; **Editar valores por defecto…** abre su editor
completo. Guardar los valores por defecto cambia la siguiente aplicación
de estilo y deja la canción actual como está. Solo cambian los
componentes marcados. Las entradas marcadas con **+ acordes** incluyen
una progresión; las demás dicen **conserva los acordes** y muestran la casilla
de progresión desmarcada y desactivada. Aplicar un preajuste sin acordes
conserva la elección de progresión recordada para el siguiente preajuste
con acordes. La progresión de acordes está desactivada al principio;
márcala para copiar también el cuadro de acordes de un preajuste. Una
audición vacía usa acordes de ejemplo para demostrar el preajuste. La lista
combinada mantiene disponibles todos los patrones de interpretación y
elecciones de instrumento originales.

La parte superior de la Biblioteca dice de dónde viene cada parte de la
canción: patrones, instrumentos, compás, acordes, tempo y swing. Abrir
una demo trae todas las partes de ella; aplicar un preajuste trae las
partes marcadas, incluso una que ya era igual. Cada línea nombra la
demo, el estilo, el ritmo o el estilo guardado, dice **editado, era …**
una vez que cambias esa parte tú mismo, o **tuyo** cuando nada la ha
traído. Un compás elegido con el selector **Compás** es una edición tuya;
una parte que solo se mueve junto con otra, como el tempo que se
conserva con un compás nuevo, mantiene su origen. El popup muestra lo
mismo junto a cada casilla (**ahora: …**), para que veas lo que
reemplazaría. Deshacer, rehacer y guardar conservan los orígenes.

## Bloquear un canal

El candado junto al mando de volumen de cada canal en el rack de canales
bloquea ese canal: los preajustes, el selector **Compás** y la banda
conservan sus notas y su instrumento, y cambian el resto de la canción.
Un canal Teclados bloqueado conserva los acordes de la canción, y el bajo los
sigue; una batería bloqueada conserva sus notas mientras las demás
baterías adoptan el ritmo nuevo. Cuando la banda entra en una canción
cuyo canal Teclados o Bajo propio está bloqueado, le deja esa parte. Un compás
nuevo sigue encajando las notas de un canal bloqueado en los nuevos
compases, y todavía puedes editarlo a mano. La Biblioteca y el popup
enumeran lo que está bloqueado (**🔒 Se conservan: …**).

## Más opciones de la Biblioteca

En modo canción, copiar **Patrones de ejecución** y **Voces / instrumentos**
trae la banda completa del preajuste, incluida su guitarra, cuerdas u
órgano extra, incluso cuando la demo todavía no tiene banda en vivo.
Deja **Progresión de acordes** desactivada para conservar los acordes
exactos de la canción y cambiar solo su acompañamiento. Activarla
reemplaza la armonía por el cuadro del preajuste conservando la melodía.

El menú **Progresión** de la hoja de acordes también usa el navegador de la
Biblioteca, con los mismos nombres de preajuste, búsqueda y preferencia de
agrupación. Busca por nombre, género o compás; la búsqueda se mantiene
entre vistas, y **×** la limpia. **Progresión** muestra los preajustes de
armonía que incluyen una progresión de acordes.

Selecciona **Acentos simples** para tocar acentos de acordes y de bajo
sobre los grupos de pulsos de la métrica con los instrumentos actuales.
Esto funciona con todos los preajustes, incluidos los estilos guardados.
**Batería y acentos simples** aplica la batería del preajuste con acentos
sencillos de acordes y de bajo. La opción de instrumento se oculta
conservando su valor recordado; **Patrones del preajuste** restaura la copia
normal del estilo. El selector **Compás** y el menú de compás del compás 1
también muestran directamente las casillas de estilo, ritmo/acento,
instrumento y swing correspondientes. Desactivar **Usar el estilo correspondiente**
ahí usa un ritmo de batería sencillo y acentos con altura en la métrica
pedida. Los cambios de métrica repetidos usan la canción original como
referencia hasta que editas el proyecto explícitamente; volver a su
métrica original restaura su interpretación exacta. Deshacer y rehacer
conservan la referencia correspondiente. La Biblioteca muestra los patrones
de la banda y sus instrumentos como partes separadas; la hoja de acordes
sigue siendo la fuente de los acordes de la canción. Estas elecciones de
la banda sobreviven a guardar y a deshacer/rehacer. Un cambio de métrica
conserva la duración de cada compás ajustando el tempo.

## El editor de estilos

Usa **Editar estilo…** o **Nuevo estilo…** en el rack de canales (también
**Ver > Editor de estilos…**) para editar partes repetidas de batería,
acordes y bajo. [El editor de estilos](step-9/style-editor.md) cuenta
cómo funciona. El editor reutiliza el rack y el piano roll, con su
propio historial de deshacer y una vista previa de cuatro repeticiones.
**Empezar desde…** copia un estilo existente; hacer clic derecho en un
estilo o ritmo de la Biblioteca también abre una copia editable. Las
plantillas con altura están escritas sobre do mayor y siguen los acordes
de la canción al aplicarse. Guárdalas como archivos `.lineup-style`;
**Ajustes > Biblioteca > Carpeta de estilos…** elige la carpeta que se carga en
**Preajustes** en el navegador compartido. **Empezar desde…** usa esa misma
lista, incluidos los preajustes de ritmo. Los estilos aplicados se incrustan
en los archivos de proyecto, así que reabrir una canción conserva sus
patrones personalizados.

## El mezclador

El botón del mezclador en la barra de herramientas (tres faders),
**Ver > Mezclador** o **F9** abren el mezclador. Sus canales se enumeran en el
orden de la partitura de un director, por secciones, con toda la batería
junta. Cada uno tiene su instrumento (haz clic para elegir otro, o para
enviar el canal por MIDI), un medidor de nivel, un volumen, un paneo de
L100 a R100 (doble clic para el centro), cuánto de él va a la reverb,
mute y solo. El master tiene un medidor por cada altavoz. Funcionan
tanto con sonidos SoundFont como Lineup Osc, además de muestras. La
reverb estéreo del master tiene controles de húmedo / seco, decaimiento, amortiguación y predelay. Los ajustes del mezclador se guardan con el proyecto, admiten
deshacer y se usan para la exportación WAV.

## Alturas de pista, selecciones y atajos

Redimensiona una pista individual de la lista de reproducción arrastrando el borde
inferior de su cabecera de nombre. Haz doble clic en ese borde para
restablecerla, o usa **Alto de la pista** en el menú de clic derecho de la
cabecera. Las alturas siguen los movimientos de las pistas, se guardan
con el proyecto y se pueden deshacer.

**F1**, o **Ayuda > Atajos de teclado**, enumera todas las teclas a las
que responde la aplicación, ventana por ventana. La misma lista está en
el [apéndice de atajos de teclado](appendix-shortcuts.md), para
imprimir.

**Ayuda > Acerca de Ferrous Lineup** muestra la versión que estás usando, la
licencia, y enlaces a este libro, a su PDF, al código fuente y a la
lista de problemas conocidos.

**Seguir el cursor de reproducción** en la barra de herramientas y **Ajustes > Reproducción**
mantienen visible la posición de reproducción en la lista de reproducción, la hoja de acordes y el piano roll. Desactivarlo te deja desplazarte libremente; la
preferencia se recuerda. Los atajos del editor siguen a la ventana
activa: **Ctrl+A** selecciona sus compases, notas o clips. El piano roll
admite **Ctrl+C/X/V** y **Ctrl+D** para notas, incluido pegar entre la
canción y el editor de estilos; su menú Herramientas enumera estas acciones.
Pegar o duplicar notas amplía el patrón cuando hace falta. La lista de reproducción
también admite **Ctrl+C/X/V**, **Ctrl+D**, **Supr** y **Ctrl-clic**
para añadir o quitar un clip de la selección. Ctrl-clic en un hueco
vacío de la lista de reproducción fija la posición de pegado sin colocar un clip.
**Ctrl-arrastrar** sobre un hueco vacío de la lista de reproducción selecciona una caja
de clips; arrastrar un clip seleccionado mueve juntos los clips
seleccionados. **Ctrl-clic** en un marcador de sección (Introducción, Estrofa, Estribillo…) selecciona sus clips superpuestos, o elige **Seleccionar los clips de esta sección** en su menú. Vuelve a hacer Ctrl-clic o pulsa **Esc**
para limpiar la selección. En la hoja de acordes, hacer clic en el
encabezado de una sección selecciona todos los compases de esa sección;
volver a hacer clic los desmarca. **Mayús-clic** o **Mayús+flechas**
selecciona un rango de compases, y **Esc** lo limpia. La selección se
queda con los clips cuando se borran clips anteriores. Los popups
abiertos y los controles de configuración conservan sus propias teclas.

## La hoja de acordes y la banda

**F5**, o el primer botón de ventana de la barra de herramientas (una
página de nombres de acordes), abre la **hoja de acordes**: la armonía de la
canción compás a compás, cada sección bajo su nombre, como lo escribe una partitura guía. **Compases por línea** pone en una línea tantos compases como
muestran todos los nombres completos, a su tamaño completo (1, 2, 4, 8,
12 o 16, de modo que las líneas empiecen donde empiezan las frases), o
el número que elijas. Haz clic en un compás y escribe su armonía:
acordes (`Am7 D7`, `G7/B`, `F#m7b5`), números romanos en la tonalidad
(`ii7 V7`), conjuntos de clases de alturas (`3-11 T2`, `{0,1,4}`),
micromodos (`Menor 3`), formas de serie dodecafónica (`P0`, `RI7`) o
`N.C.`; una `/` mantiene la armonía anterior un pulso más (`C / / G7`).
Bajo cada acorde está su numeral en la tonalidad, con las cifras de una
inversión (V⁶₅), y bajo cada conjunto sus notas; mantén pulsado el ratón
sobre una armonía para oírla. **Escribir los acordes que toca la canción** rellena
una hoja de acordes vacía a partir de lo que toca la canción, con la nota del
bajista después de la barra, y en un conjunto o una serie con los
conjuntos o micromodos oídos. Mientras suena la canción, el compás que
suena queda enmarcado en blanco. **Mostrar**, arriba de la hoja de acordes,
nombra la misma armonía **Como está escrito**, como **Acordes**, como
**Conjuntos de clases de alturas** (do mayor es 3-11 T7I) o como **Micromodos**
(Acorde menor-mayor T7I), sin cambiar una nota. Para conjuntos y
micromodos también puede leer las notas de la canción, **juntas**
(verticalmente) o **in a line** (en serie), en grupos de 2 a 12 notas,
de todas las pistas o de una. La hoja de acordes se guarda con el proyecto y
sigue los cambios de tonalidad, de métrica y las secciones copiadas. Haz
clic derecho en un compás para **Insertar un compás antes**, **Insertar un compás después** o **Eliminar este compás** (o los compases seleccionados): todo lo
que hay después se desplaza —notas, acordes, marcadores y secciones—, y
un compás nuevo queda vacío, así que la armonía anterior continúa a
través de él. Los números de compás de la regla de la lista de reproducción tienen las
mismas opciones. La hoja de acordes es una ventana encima de la lista de reproducción, o,
con **⛶ Llenar la ventana** (también **Ver > Hoja de acordes en la ventana principal**), ocupa el lugar de la lista de reproducción en la ventana principal, con
líneas más largas; **Volver a la lista de reproducción** lo devuelve.

**Archivo > Nuevo** abre una hoja de acordes vacía con una **banda**: escribe
acordes y un piano, un bajo y una batería los tocan, escritos como
patrones en sus propias pistas de la lista de reproducción. **🎷 Banda** elige la
formación de instrumentos, la voz de cada músico y qué músicos se oyen.
Estos controles conservan los patrones de interpretación, acordes,
métrica, tempo y swing actuales. **Sin banda** quita el acompañamiento.
Elige ritmos y patrones de interpretación en la **Biblioteca**; un vals
puede poner la canción en 3/4 al copiar su compás. **Progresión**
escribe una progresión conocida desde el compás seleccionado, en la
tonalidad de la canción, y **Terminar la canción aquí** (clic derecho en un
compás) dice dónde se detiene la banda. La banda vuelve a tocar cada vez
que cambian la hoja de acordes, la tonalidad o los compases; una parte que
editas en el piano roll se queda como la dejaste hasta que haces clic en
**Volver a tocarla**. Elegida sobre una demo, la banda toca los acordes de
la demo y silencia su propio acompañamiento hasta que eliges
**Sin banda**. **Archivo > Nuevo ritmo** es el antiguo patrón vacío con una
batería.

Para practicar con la banda, la segunda fila de la hoja de acordes tiene un
**Cuenta de entrada** (un compás de clics antes de que empiece la música), un
**Metrónomo**, la **Velocidad** a la que tocar (del 25% al 200% del tempo de
la canción, que la propia canción conserva) y **Pulsar**, que fija el tempo
a partir de unos cuantos clics sobre el pulso. El **Entrenador de velocidad**
arranca la banda más despacio (70%) y toca un poco más rápido cada vez
que el bucle vuelve a empezar, hasta el tempo propio de la canción; ⏹ lo arranca de nuevo desde la velocidad lenta. **Tocas tú** deja a un
músico fuera de la banda (el piano, el bajo, la batería, o una guitarra,
cuerdas u órgano) para que toques tú la parte; entonces tu teclado toca
el sonido de esa parte. Haz Mayús+clic en un segundo compás para
seleccionar varios, y **Bucle** los reproduce una y otra vez. Los
clics nunca llegan a una canción guardada ni a una exportación WAV.
**Seguir el cursor de reproducción**, **Cuenta de entrada** y **Metrónomo** son también botones
conmutadores iluminados junto al transporte principal. Los ajustes de
práctica también están en **Edición > Ajustes… > Reproducción**.

La fila **Forma**, encima, da forma a toda la interpretación. **Introducción**
pone 2, 4 u 8 compases delante de la canción, un turnaround que lleva a
su primer acorde (ii–V, o I–vi–ii–V), escrito en la hoja de acordes para que
puedas cambiarlo. **Vueltas** reproduce la melodía ese número de veces,
mostrada una sola vez entre signos de repetición en la hoja de acordes y la lista de reproducción. **Final** añade dos compases sobre la tónica de la tonalidad
después del último estribillo: uno en el estilo, y luego el acorde
golpeado una vez y sostenido, con un golpe de platillo.

Haz clic derecho en un compás (o antes Mayús+clic para seleccionar
varios) para elegir **Variación a** o **Variación b** del estilo desde
ahí. La variación b es el mismo estilo tocado más lleno, como cuando una
banda se entrega en un estribillo o un puente: la batería pasa del
hi-hat al ride, un bajo o un piano más ocupados donde el estilo los
tiene (el bajo de una balada de jazz camina, el piano de una balada pop
toca corcheas), un poco más fuerte. La batería toca un relleno en cada
cambio y golpea el platillo en su primer pulso, y una letra en un
recuadro lo marca en la hoja de acordes y en la lista de reproducción.

La opción **Estilo** del mismo menú toca los compases en otro estilo: una
estrofa como bossa nova y el puente como swing. Haz clic derecho en el
nombre de una sección en la hoja de acordes para elegirlo para toda la
sección. La banda conserva sus instrumentos y toca el comping, la línea
de bajo, la batería y el swing del nuevo estilo, con un relleno hacia el
cambio. El nombre del estilo marca dónde empieza, y **El estilo de la canción**
vuelve atrás. Solo se ofrecen estilos en el compás de esos compases.

**Swing**, junto al tempo en la hoja de acordes (y en la lista de reproducción), toca
pares de corcheas largo-corto, como hacen los músicos de jazz, blues y
shuffle: desde *straight* (parejas) hasta el 100%, los dos primeros
tercios y el último tercio de un tresillo. Las notas se quedan como
están escritas; solo cambia cómo se tocan, también en las exportaciones
WAV y MIDI. Cada estilo trae su propio swing (Swing, Balada de jazz, Vals de jazz y Shuffle 100%, Rock and roll 50%), y recuerda la cantidad que
fijaste por última vez para él.

Muchos estilos traen más músicos que el piano, el bajo y la batería. Una
**guitarra** toca las posiciones que tocaría un guitarrista (el C o G
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
de una armonía es un **golpe** (`C.`: la banda lo golpea una vez, corto, y
se detiene hasta la siguiente armonía), dos puntos un **sostenido** (`C..`:
golpeado y sostenido) y tres un **silencio** (`C...`: silencio). Un signo de
intercalación antes es una **anticipación** (`^C`): la banda lo toca una corchea
antes y lo sostiene sobre el pulso, y `^^C` llega una semicorchea antes.
La batería se detiene con la banda y golpea un bombo y un platillo con cada golpe, sostenido y anticipación. Haz clic derecho en un compás y elige
**Banda en C** para fijar las mismas marcas desde un menú.

**Solista (experimental)**, al final del menú de la banda, añade un
solista que improvisa sobre el estribillo: un saxo tenor en el jazz, una
armónica en el blues, un violín en el folk, etcétera. La línea pone
notas del acorde en los pulsos y rellena el espacio entre ellos con
notas de la escala del acorde (elegidas de la tonalidad, de modo que en
do mayor G7 toca Sol mixolidio y E7, que lleva a Am, toca la escala de la
menor armónica desde Mi: Mi frigio dominante) o con notas cromáticas a un
semitono de distancia. Respira entre frases, y **Otro solo** toca uno
nuevo. Abre su parte en el piano roll para ver qué es cada nota: las
notas del acorde en verde con su grado (R 3 5 7), las notas de la escala
en azul (de paso, vecinas), las notas cromáticas en naranja; señala una
nota para que te la explique.

## Hojas de acordes como texto

**Archivo > Importar hoja de acordes…** lee un cuadro de acordes escrito como
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

El cuadro se convierte en una canción nueva con la banda tocándola y la hoja de acordes abierta; un error se indica con su línea. **Archivo > Exportar hoja de acordes…** escribe la hoja de acordes de la canción de la misma manera,
alineado en columnas, y en la hoja de acordes **Ctrl+C** y **Ctrl+V** copian
y pegan los compases seleccionados como texto, hacia y desde un editor
de texto u otra canción.

## Series dodecafónicas, conjuntos de clases de alturas y micromodos

Para la música sin tonalidad de referencia, los menús de tonalidad
también ofrecen una **serie dodecafónica** y un **conjunto de clases de alturas**.
**Serie…** en el piano roll abre la ventana dodecafónica: escribe, dibuja
o elige una serie (las series de la Suite op. 25 de Schoenberg, la
*Lyric Suite* de Berg, y la Sinfonía op. 21 y el Concierto op. 24 de
Webern, todos en dominio público ya, o unas cuantas escritas para
Ferrous Lineup), mira su matriz con las 48 formas (P, I, R y RI sobre
cada nota), y escribe cualquiera de ellas en el patrón como melodía o
como acordes, con las duraciones, dinámicas y octavas siguiendo también
la serie si quieres. **Escribir … en el patrón actual** reemplaza las
notas del instrumento seleccionado en ese patrón. **Guardar en un patrón nuevo** crea y selecciona un patrón aparte, usando las mismas opciones
de escritura de la serie, longitud y compás, y deja intacto el patrón de
origen. Ambas acciones se pueden deshacer. **Conjuntos…** enumera los 224 conjuntos
de clases de alturas por número de Forte, con sus formas primas y
vectores interválicos, para dibujar con ellos o usarlos como escala, y
al seleccionar notas pone nombre a su conjunto. **Escucharlo** preescucha
el conjunto o micromodo elegido en el instrumento seleccionado.
**Dibujar con él** en Conjuntos y Micromodos se ilumina mientras está activo;
haz clic de nuevo o elige **Nota sola** para dejar de dibujar el
conjunto. **Dibujar > Nota sola** del piano roll hace lo mismo.
**Series dodecafónicas** de la Biblioteca y **Archivo > Demostraciones > Dodecafónico**
reproducen siete maneras de usar una serie.

**Micromodos…** abre los doce micromodos de Francisco Kröpfl: los doce
grupos de tres notas (los tricordios de Forte, 3-1 a 3-12) en sus tres
familias, menor, mayor y acordes. Cada uno se dibuja sobre un reloj, con
su forma prima en solfeo, sus intervalos y su hora en el Tone Clock de
Peter Schat, y se puede oír, dibujar con él o usarlo como tonalidad.
Seleccionar notas muestra qué micromodos contienen, y las ventanas Micromodos y Conjuntos se enlazan entre sí: un conjunto de tres notas es un
micromodo, y todo conjunto mayor está hecho de ellos. **Micromodos (Kröpfl)** de la Biblioteca y **Archivo > Demostraciones > Micromodos** reproducen
música hecha a partir de un micromodo.

## Importar grabaciones y archivos MIDI

**Archivo > Importar grabación…** abre un archivo MP3, WAV, FLAC u Ogg (o
suéltalo sobre la ventana) y lo escucha: el tempo, el compás y la
tonalidad se fijan a partir de la grabación, que suena en su propio
canal, y un canal Armonía toca el acorde oído en cada pulso, de modo que
el piano roll y las teclas de armonía lo siguen. Es un experimento temprano;
mira
[`crates/lineup-listen`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-listen/README.md)
para ver cómo funciona y con qué frecuencia acierta.

**Archivo > Importar archivo MIDI…** abre un archivo `.mid` (o suéltalo sobre la
ventana) como un proyecto nuevo: cada pista se convierte en un canal (la
batería del canal 10, un canal por tambor), cada nota conserva su
velocidad y su momento exacto, incluso entre dos pasos, y los cambios de
tempo, los compases y la tonalidad vienen con él. El carril Acordes del
piano roll y las teclas de armonía ponen nombre a los acordes mientras suena.

## Sonidos, entrenamiento auditivo y la partitura guía

**Edición > Ajustes… > Audio > Instrumentos del SoundFont** reproduce
grabaciones de instrumentos reales en lugar del Lineup Osc: cada canal
con un instrumento General MIDI (las voces de las demos, el piano y el
bajo de la Biblioteca, los acordes de la canción de demostración) y, con
**Batería del SoundFont**, la batería. La primera vez, se ofrece a descargar
[GeneralUser GS](https://github.com/mrbumpy409/GeneralUser-GS), un
SoundFont General MIDI gratuito de S. Christian Collins (unos 30 MB,
guardado para la próxima vez). **Elegir SoundFont…** reproduce cualquier
otro archivo `.sf2` en su lugar. La exportación WAV también lo usa. La
categoría Audio también muestra el dispositivo de salida activo, la
frecuencia de muestreo y el número de canales.

**Ver > Entrenamiento auditivo** (experimental) hace preguntas para responder de
oído: qué intervalo forman dos notas, qué tipo de acorde suena (con
inversiones si quieres), y qué acordes de una tonalidad toca la banda,
nombrados en las teclas de armonía con los nombres de acordes ocultos. Cada
respuesta se explica con sus notas escritas, y cualquier respuesta se
puede reproducir junto a la correcta. La canción y su archivo se quedan
como están.

**Ver > Partitura guía** escribe la canción como la lee una banda: la
melodía en notación sobre un pentagrama, los símbolos de acordes encima,
y la tonalidad y los compases, secciones y repeticiones, dispuestos en
páginas A4 o Carta. La melodía es el canal llamado Melodía (o si no, el
más agudo; se puede elegir cualquier otro), y cada nota se escribe para
la tonalidad y el acorde que suena, de modo que un Sol♯ sobre E7 sigue
siendo Sol♯ y un La♭ sobre Fm sigue siendo La♭. **Ninguna: solo acordes**
escribe barras de ritmo bajo los acordes en su lugar. **Para** escribe la partitura guía para instrumentos en Si♭, Mi♭ o Fa, de modo que un trompetista,
un saxofonista o un trompista lo lee en su propia tonalidad: la
armadura, la melodía y los acordes se mueven todos. Mientras suena la
canción, se ilumina el compás que suena. **Exportar PDF…** guarda la partitura guía para imprimirlo o compartirlo, y **Imprimir…** lo abre en tu visor de
PDF. La música se dibuja con la fuente Bravura y las palabras con Inter,
dos fuentes gratuitas que vienen con la aplicación.

## El zoom y los ajustes

¿Demasiado pequeño o demasiado grande? Las mismas teclas hacen zoom
sobre lo que tenga el puntero encima. Sobre la lista de reproducción o el piano roll,
**Ctrl** con **+** y **−** o con la rueda del ratón (⌘ en lugar de Ctrl
en un Mac) hace los compases y los pasos más anchos o más estrechos, y
**Alt** con las mismas teclas o con la rueda hace las pistas y las
teclas más altas o más bajas, alrededor de lo que esté bajo el puntero.
Sobre el rack de canales hacen zoom de su único tamaño. En cualquier otro
sitio (las barras de herramientas, las teclas de armonía, la barra de estado,
el panel Patrones y Biblioteca) hacen más grandes o más pequeños el texto,
los botones y los menús de todas partes, como hace **Ajustes > Apariencia > Tamaño de la interfaz**. **Ctrl+0** devuelve al 100% lo que esté
bajo el puntero. Los dos zooms de la lista de reproducción también están en el extremo
derecho de la barra de estado. **Edición > Ajustes…** separa las
preferencias en las categorías **Apariencia**, **Armonía**, **Biblioteca**,
**Audio** y **Reproducción**. Apariencia contiene el idioma, el tamaño de la
interfaz y todos los niveles de zoom del editor; Armonía contiene cómo
se nombran las notas y cómo se escriben los numerales en tonalidades
menores; Biblioteca contiene la agrupación de
estilos, los valores por defecto de estilos guardados y la carpeta de
estilos; Audio contiene la información de salida y los ajustes de
SoundFont; Reproducción contiene el seguimiento de la reproducción y los
controles de práctica. Los controles de zoom comparten los valores
actuales con los atajos de teclado y de rueda. El menú Ver también muestra u oculta
cada ventana y panel, y **Alt+F8** oculta el panel Patrones y Biblioteca
para dar más espacio a la lista de reproducción. El zoom, el tamaño de la ventana de la
aplicación y dónde dejaste las ventanas se recuerdan para la próxima
vez. Una ventana se abre al tamaño de lo que muestra (el mezclador, el rack de canales, los atajos) o, en el caso de una ventana en la que
trabajas (el piano roll, la hoja de acordes), a su propio tamaño la primera
vez después de arrancar la aplicación.

## Exportar vídeo, y programas ya compilados

**Archivo > Exportar video…** guarda la canción como un archivo MP4, con las
vistas que elijas tocando a la vez: la hoja de acordes con el compás que
suena enmarcado, la partitura guía, la lista de reproducción, el piano roll y las teclas de armonía, dispuestos en una vista previa, en HD o Full HD. Los codificadores
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

`--trace` reproduce la canción como hace Archivo > Exportar WAV, sin ventana
ni tarjeta de sonido. Para trazar lo que tocas en la aplicación en su
lugar, marca **Escribir un registro mientras suena** en Ajustes > Reproducción:
cada reproducción, de ▶ a ⏹, añade una tabla al archivo que se
muestra ahí. Las líneas que empiezan por `#` dicen qué contiene la tabla
y enumeran los cambios de tempo y de compás; el resto está separado por
tabuladores, para una hoja de cálculo, `awk` o `grep`. En Windows, usa
`--out`: el programa ya compilado no tiene terminal donde imprimir.
