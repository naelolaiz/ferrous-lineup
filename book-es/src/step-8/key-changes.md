# Cambios de tonalidad a mitad de camino, y la Marcha turca completa

Archivos: [`crates/lineup-core/src/key_map.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/key_map.rs),
[`crates/lineup-core/src/project.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/project.rs),
[`crates/lineup-core/src/library/demo.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/demo.rs),
[`crates/lineup-core/src/library/song/electronic.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/song/electronic.rs)
y [`crates/lineup-app/src/playlist/sections.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/playlist/sections.rs)

La pregunta con la que empezó todo esto: *«No veo la modulación de la
armadura en la demo de la Marcha turca. ¿No hay una modulación en el
original?»* Sí la hay. El Rondo alla Turca de Mozart, último movimiento
de su Sonata para piano n.º 11 (K. 331), cambia su armadura tres veces:

| Compases | Armadura | Tonalidad |
|---|---|---|
| 1–24 | ninguna | La menor (los compases 9–16 pasan por Do mayor) |
| 25–32 | tres sostenidos | La mayor: la marcha "turca" fuerte |
| 33–40 | tres sostenidos | Fa♯ menor, terminando en Do♯ menor |
| 41–48 | tres sostenidos | escalas de La mayor |
| 49–56 | tres sostenidos | Fa♯ menor |
| 57–64 | tres sostenidos | La mayor: la marcha otra vez |
| 65–88 | ninguna | La menor: el tema otra vez |
| 89–128 | tres sostenidos | La mayor: el finale y la coda |

La demo ya tocaba su marcha en La mayor, pero no podías verlo: un
proyecto tenía una sola tonalidad, `Project::scale`, y la tonalidad
mostrada no se movía. Una parte escrita con `.in_key(A, Major)` solo
cambiaba cómo se leían sus numerales de acorde. Ahora una canción puede
cambiar de tonalidad, y la app lo muestra.

## Por qué la misma armadura puede significar dos tonalidades

Tres sostenidos (Fa♯, Do♯, Sol♯) es la armadura de La mayor, y también de
Fa♯ menor, su *relativa menor*: la tonalidad menor que empieza en la
sexta nota de la escala mayor y usa las mismas notas. Así que los
compases 25–64 mantienen una armadura mientras la música se mueve entre
La mayor y Fa♯ menor. Oyes la diferencia por dónde vuelve a casa la
música. En los compases 33–40 el bajo no deja de volver a Fa♯, y la
melodía se apoya en Mi♯ (la séptima elevada de Fa♯ menor armónica, la
nota justo debajo de Fa♯). La demo marca la tonalidad en la que está la
música, no solo la armadura, así que la regla dice *F# harmonic minor*
desde el compás 33.

Los compases en Do mayor del tema (9–16) son distintos: la música
visita Do mayor unos pocos compases sin armadura nueva y sin quedarse
allí. Eso se llama una *tonicización* más que una modulación, así que
la demo mantiene allí La menor y escribe los acordes como ♭III y ♭VII
de La menor.

## Un marcador de tonalidad

Un cambio de tonalidad es un marcador más en el playlist, junto a los
marcadores de compás y de tempo:

```rust
pub struct KeyMarker {
    pub start: usize, // the step it starts on
    pub scale: Scale, // the key from there on
}
```

`KeyMap` responde "¿en qué tonalidad está el paso 300?" igual que
`Meter` responde "¿en qué compás está?": la tonalidad propia del
proyecto hasta el primer marcador, luego la tonalidad de cada marcador
por turno. `Project::key_at(step)` se lo pregunta, y
`Project::pattern_key(pattern)` da la tonalidad donde empieza el primer
clip de un patrón, como hace `pattern_time` para el compás.

Un marcador de tonalidad nunca mueve una nota. Esa es la diferencia con
los menús Key (véase [Cambiar de tonalidad](changing-key.md)), que
transponen la canción entera. Cambiar la tonalidad de la canción ahora
también mueve cada marcador de tonalidad el mismo intervalo, y cada
patrón se mueve de la tonalidad en la que se toca a donde fue esa
tonalidad. Mueve la Marcha turca de La menor arriba a Do menor y la
marcha se vuelve Do mayor y el episodio La menor, como leería una
partitura transpuesta.

Una canción exportada como archivo MIDI recibe un evento de armadura al
principio y en cada cambio, de modo que un programa de notación que lo
abra vea aparecer los tres sostenidos en el compás 25.

Los archivos de proyecto guardan los marcadores en una lista
`key_markers`. Una canción con alguno se guarda como formato 8; una sin
ninguno se queda en el formato viejo, así que las versiones viejas de
Ferrous Lineup pueden seguir abriéndola.

## Verlo en la app

- La regla escribe la tonalidad tras el número de compás en el compás 1
  y dondequiera que cambie, en azul.
- Haz clic derecho en los números de compás: bajo los compases ahora
  está **Key from bar N**, con una fundamental y una escala que elegir,
  y **Remove this key change**. El compás 1 es la tonalidad propia de
  la canción, que cambian los menús Key.
- En modo canción, las harmony keys siguen la tonalidad en el playhead
  (o donde vaya a empezar Play). Cuando no es la tonalidad en la que
  empieza la canción, *Now in A major* aparece bajo los menús Key, y
  las siete teclas de acorde son los acordes de esa tonalidad.
- El piano roll resalta las notas de un patrón en la tonalidad en la
  que se toca, y dice *This pattern: A major* junto a los menús Key.

![La Marcha turca en el compás 27, en la marcha: la regla dice A major en el compás 26 y F# harmonic minor en el compás 34, y las harmony keys son las de La mayor.](../images/step-8-key-change-march.png)

![El clic derecho en los números de compás: el compás, luego la tonalidad desde ese compás en adelante.](../images/step-8-key-menu.png)

## La pieza entera

La demo vieja tocaba solo los primeros 32 compases, y se saltaba los
compases en Do mayor. Ahora sigue la partitura, transcrita de la
edición Humdrum del K. 331 (una partitura gratuita y legible por
máquina): cada sección y cada repetición, escritas en orden, los
primeros y segundos finales incluidos, hasta los últimos acordes de La
mayor de la coda. Cada compás de la demo contiene dos compases de la
partitura, que está en 2/4, así que el compás 25 de la partitura (la
marcha) aparece por primera vez en el compás 26 de la demo. La línea
superior de la mano derecha es la melodía, y la mano izquierda se lee
como acordes. Unos pocos acordes necesitan una explicación:

- **Compás 20, Fa La Re♯** es un acorde de *sexta aumentada*: Fa y Re♯ están
  a una sexta aumentada y tiran hacia fuera a Mi. Suena como F7, que es
  como lo escribe la hoja de acordes (♭VI7).
- **Compás 27, Re♯ Fa♯ La** es un acorde disminuido sobre el cuarto grado
  elevado (♯iv°), que lleva a Mi: el bajo de la marcha sube Re, Re♯, Mi.
- **Compases 38–40** terminan el episodio en Fa♯ menor en Do♯ menor, su
  quinto grado vuelto menor (v), mediante G♯7 (II7).

Las notas de adorno (las notas rápidas antes de un pulso) se omiten, y
el trino del compás 23 es una nota normal.
