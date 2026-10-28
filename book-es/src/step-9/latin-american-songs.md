# Canciones latinoamericanas, de México al Río de la Plata

Archivos: [`crates/lineup-core/src/library/song/tango.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/song/tango.rs),
[`crates/lineup-core/src/library/song/latin.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/song/latin.rs),
[`crates/lineup-core/src/library/song/folklore.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/song/folklore.rs),
[`crates/lineup-core/src/library/chart.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/chart.rs)
y [`crates/lineup-core/src/library/comping.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/comping.rs)

Las demos tenían un puñado de tangos, unas pocas canciones andinas y
algunas cubanas y brasileñas. Este capítulo añade **44 canciones
latinoamericanas**, hace que **seis antiguas toquen su partitura
completa** y añade cinco ritmos de baile que la biblioteca no tenía. Cada canción nueva se escribió a partir de una partitura,
nunca de memoria, y cada acorde se comprobó escuchando la banda
renderizada.

## Probarlo

Abre **File > Demos** y escribe «cabeza» en el cuadro de búsqueda:
**Por una cabeza**, el tango de Gardel de 1935. Luego prueba estos, uno
por cada tipo de música:

- **Carioca** (Brasil): un maxixe, el «tango brasileño» de 1913, en Mi
  menor.
- **Sobre las olas** (México): sus cuatro valses.
- **Papel de plata** (los Andes): un huayno cuyo estribillo pasa a 3/4.
- **La gata golosa** (Colombia): un pasillo, un vals con el segundo
  pulso retrasado.
- **Aquellos ojos verdes** (Cuba): un bolero.
- **Luna de Xelajú** (Guatemala): un vals en la marimba.

El género de folclore en **File > Demos** es **Latin American
folklore**, ya que llega de México a Argentina.

Las canciones nuevas:

| De dónde | Canciones |
|---|---|
| Tango | Lágrimas y sonrisas, La media luz, Adiós muchachos, Caminito, Boedo, Alma en pena, El amanecer, La la gran muñeca, Por una cabeza, El día que me quieras, Volver |
| Andes (huayno) | Papel de plata, La Quebrada de Humahuaca, Huayta huaytucha, Kicharirillaway, La las orillas del Titicaca, Chunguinada |
| México | Jesusita en Chihuahua, De colores, Sobre las olas, La golondrina, Estrellita, Jarabe tapatío, La raspa, La bamba, La cucaracha (completa), La Llorona, María Elena, Te quiero dijiste, Los machetes |
| Cuba | Quiéreme mucho, Siboney, Aquellos ojos verdes |
| Colombia | Diana triste, La gata golosa |
| Guatemala | Luna de Xelajú |
| Brasil | Apanhei-te, cavaquinho, Carioca, Atraente, Sons de carrilhões, Ó abre alas, Pezinho, Balaio, Luar do sertão |

Ahora se tocan completas: **El Choclo**, **La Cumparsita**, **La
Paloma**, **Gallo ciego**, **El cóndor pasa** y **Alma llanera**.

## La música

### Secciones, repeticiones y «D.C.»

Un tango, un choro o una marcha de 1900 no es estrofa y estribillo:
son dos o tres **secciones** de 16 compases (en 2/4), cada una tocada
dos veces, a menudo con la última en otra tonalidad. La partitura
ahorra papel con signos de repetición, casillas de primera y segunda
vez, y **D.C.** (*da capo*, «desde el comienzo»: volver al principio)
o **Re.S.** (*dal segno*, volver a un signo), hasta **Fine** («fin») o
un salto a una **coda** («cola»).

Una demo toca todo ello escrito, en el orden que dice la partitura. En
el playlist puedes ver la forma: **El Choclo** es Anacrusa, La, Si, Si,
Do, luego el D.C. lo lleva de vuelta a La y Si una vez más (las
repeticiones no se repiten una segunda vez tras un Re.Do.), y Do con su
casilla de segunda vez cierra la pieza. Antes, la demo se detenía tras
Do y nunca volvía.

### Cambios de tonalidad que puedes ver

Cada cambio de tonalidad de estas partituras es un marcador de
tonalidad en la regla (mira [Cambios de
tonalidad](../step-8/key-changes.md)):

- **El Choclo** pasa de La menor a **La mayor** para su tercera sección
  (la armadura de la partitura cambia de ninguna a tres sostenidos),
  vuelve a La menor con el Re.Do. y termina en La mayor.
- **Gallo ciego** visita el **relativo mayor**, Fa, en su segunda
  sección (misma armadura que Re menor, pero las cadencias son Do7 a
  Fa), y su tercera sección está en **Si bemol**, un cambio real de
  armadura.
- **Volver** tiene su estrofa en Sol menor y su estribillo en **Sol
  mayor**: la misma tónica, el otro modo. Escucha cómo el Si bemol se
  convierte en Si.
- **Carioca** está en Mi menor, con una segunda sección en Sol (el
  relativo mayor) y un trío en **Do**, a una tercera de distancia:
  Nazareth lo escribió como un pequeño rondó de piano.
- **Alma llanera** tiene su estrofa en Re menor y su estribillo en **Re
  mayor**, la famosa elevación del joropo.

### Cinco ritmos de baile nuevos

Los menús Rhythms y Styles de la biblioteca tienen cinco entradas
nuevas. Cuatro de ellas están en 2/4, escritas a razón de dos de sus
compases por compás de la app:

- **Polca**: la polca tal como la adoptaron los corridos y las bandas
  norteñas de México y los choro de Río: el bajo en el pulso 1, las
  guitarras en el pulso 2 y su «y».
- **Maxixe**: el baile que los compositores de piano de Río llamaban
  «tango brasileño»: el largo, corto, largo, largo de la habanera en
  la mano izquierda, y una síncopa (corto, largo, corto) en la
  melodía.
- **Marchinha**: la marcha de carnaval, cantada en la calle desde Ó
  abre alas de 1899: el bombo en ambos pulsos, el segundo más fuerte.
- **Son jalisciense** (6/8): el son de Jalisco y del mariachi,
  alternando entre dos grupos de tres corcheas y tres grupos de dos,
  como la *sesquiáltera* española.
- **Pasillo** (3/4): el vals rápido de Colombia y Ecuador, con el
  segundo pulso tocado tarde, en su «y».

### Apoyaturas

Toca **El día que me quieras** y mira el canal Lead frente al nombre
del acorde: muchas frases empiezan en una nota que **no** está en el
acorde (Fa sobre Do7, Mi bemol sobre Si bemol menor) y solo después se
mueven a una que sí está. Eso es una **apoyatura**, una nota que se
«apoya»: la tensión en el pulso fuerte y su resolución a un grado de
distancia son lo que hacen que la melodía suene a añoranza. Una
melodía no tiene que posarse en las notas del acorde; tiene que
explicar las que no lo son.

## El código

### Cada acorde se toca

`chart::accompany` convierte una hoja de acordes en las notas de las
teclas y del bajo, y reinicia el patrón de cada acorde en el acorde.
Así es como debe sonar una hoja cuyos acordes cambian en la línea de
compás. Pero toma el estribillo de **Papel de plata**, en 3/4:

```text
i bVII7 bIII | bIII bVII7 bIII | ...
```

Un acorde por pulso. El patrón de vals de las teclas toca en los
pulsos 2 y 3, así que, reiniciado en cada acorde, su primer golpe cae
un pulso después de que empiece el acorde, que ya es el siguiente
acorde: el Re7 de un pulso (`bVII7`) nunca llegaba a tocarse.

Las demos de canciones usan ahora `chart::accompany_in_bars`, que
mantiene el patrón de las teclas **en paso con los compases**, igual
que una banda mantiene su groove haga lo que haga la armonía. Cada
golpe toca el acorde que suena en ese momento, y un acorde demasiado
corto para cualquier golpe se toca una vez donde empieza:

```rust
pub fn accompany_in_bars(
    spans: &[ChordSpan],
    tonic: PitchClass,
    comping: Comping,
    bass: Bass,
    time: TimeSignature,
) -> Backing {
    let mut backing = accompany(spans, tonic, comping, bass);
    // ... build one `Played` per chord and the song's bars ...
    backing.chords = comping.play_in_bars(&bars, &harmonies);
    backing
}
```

Parte del resultado de `accompany` y reemplaza solo `chords`, así que
la línea de bajo se queda exactamente como estaba. También se probó a
mantener el bajo en paso con los compases, pero cambió el walking bass
de las demos de jazz lo suficiente como para que el buscador de
tonalidad (mira [Encontrar la
tonalidad](../step-8/finding-the-key.md)) oyera más tonalidades
equivocadas, así que el bajo sigue empezando su línea en cada acorde.

`Comping::play_in_bars` encuentra el acorde bajo cada golpe con
`partition_point`, una búsqueda binaria sobre un slice ordenado:
devuelve cuántas armonías empiezan en el golpe o antes, así que la
anterior a ese recuento es el acorde que suena.

### Una parte tocada a su manera

Una canción tenía un comping, una línea de bajo y un ritmo. Una parte
puede ahora anularlos, para una sección en otro compás:

```rust
pub struct Part {
    // ...
    pub played: Option<(Comping, Bass, &'static str)>,
}

impl Song {
    pub fn part_playing(&self, part: &Part) -> (Comping, Bass, &'static str) {
        part.played.unwrap_or((self.comping, self.bass, self.rhythm))
    }
}
```

`Option::unwrap_or` devuelve el trío de la parte cuando hay uno, el de
la canción si no. El estribillo de Papel de plata dice
`.in_time(THREE_FOUR).played(Comping::Waltz, Bass::Waltz, "Vidala")`:
los patrones en 2/4 del huayno no encajarían en un compás de tres
pulsos.

### Una prueba que se enfrentó a las formas completas

El buscador de tonalidad tiene una prueba,
`clear_modulations_are_found_on_their_bar`, que comprobaba que
encontraba cada cambio de tonalidad de unas pocas canciones
exactamente. El Choclo y Gallo ciego eran dos de ellas. Tocadas
completas, cada una vuelve ahora a su primera tonalidad tras una
**única sección de ocho compases** en otra, y el buscador oye una
sección tan corta como de paso: añade un costo, `CHANGE_COST`, por
cada cambio de tonalidad, y dos cambios cuestan más de lo que ganan
ocho compases de un ajuste ligeramente mejor. Bajar el costo los
encuentra, pero en toda la biblioteca encontraba más cambios que no
existen (89,5% de compases correctos en lugar de 91,3%). Así que esas
dos canciones pasaron a una prueba más débil que sigue cumpliéndose:
cada cambio que el buscador oye debe ser uno que la partitura hace.
Alma llanera, cuyo estribillo en Re mayor es largo, se unió a la
prueba exacta.

### Derechos de autor

Una canción se incluye si es tradicional, o se publicó en 1930 o
antes, o todos sus autores murieron antes de 1956. Un comentario sobre
cada una dice cuál. Faltan algunas canciones famosas porque no se pudo
leer ninguna partitura de ellas desde aquí (Desde el alma, Mi Buenos
Aires querido, La morocha, O samba e o tango); adjunta una partitura y
se pueden añadir.

## Ejercicios

1. Abre **Volver** y encuentra el marcador de tonalidad donde empieza
   el estribillo. ¿Qué nota de la escala de Sol menor cambia en Sol
   mayor? Encuéntrala en la melodía de los primeros compases del
   estribillo.
2. En **El Choclo**, silencia todos los canales menos Bass y toca la
   tercera sección. Encuentra el círculo de quintas Fa♯7, Si menor, Si7,
   Mi7: cada fundamental está una quinta por encima de la siguiente.
   ¿Dónde aterriza?
3. En `folklore.rs`, cambia el estribillo de Papel de plata a
   `.played(Comping::Sway, Bass::Waltz, "Vidala")`, ejecuta la app y
   toca el estribillo. Sway golpea una vez, en el pulso 2: ¿qué
   acordes de `i bVII7 bIII` oyes tocarse, y cuáles solo empiezan?
4. Toca **Los machetes** (Polca) y luego **Ó abre alas** (Marchinha),
   ambas en 2/4. ¿Qué pulso acentúa la batería en cada una?
