# Un estilo para cada sección

Archivos: [`crates/lineup-core/src/band/sections.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/band/sections.rs),
[`crates/lineup-core/src/band.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/band.rs),
[`crates/lineup-core/src/swing.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/swing.rs)
y [`crates/lineup-app/src/chord_sheet.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/chord_sheet.rs)

Muchas canciones cambian de aire a mitad de camino. Un tema de jazz
latino toca su estrofa como bossa nova y su puente en swing; una canción
pop pasa a half time para el puente; un arreglo empieza como balada y
acaba como canción de rock. [Las variaciones a y b](variations.md) hacen
más llena una parte de una canción, pero dentro del mismo estilo. Ahora
cada parte de una canción puede tener un estilo propio.

![El chord sheet de una canción de ocho compases en Pop beat. El compás 5 tiene la palabra "Swing" en naranja después de su número: desde ahí la banda toca en swing.](../images/step-9-section-styles-sheet.png)

## Probarlo

1. Abre la hoja de acordes con **F5**, con una banda tocando (cualquier
   demo, o un estilo de la Biblioteca).
2. Haz clic derecho en el nombre de una sección en la hoja, por ejemplo
   **Estribillo**. Se seleccionan los compases de la sección, y el menú
   tiene **Variación a**, **Variación b** y **Estilo: …**, que nombra el
   estilo que suena ahí ahora.
3. Abre **Estilo**, luego un género, y elige un estilo, por ejemplo
   **Latina** › **Bossa nova**. Los compases de esa sección se tocan
   ahora como bossa nova. La hoja escribe «Bossa nova» después del
   número del primer compás, y el aviso dice qué compases cambiaron.
4. El mismo menú **Estilo** está en el menú de clic derecho de cada
   compás. Sin una selección cambia el estilo desde ese compás en
   adelante; con compases seleccionados (haz clic en el primero,
   Mayús+clic en el último) cambia solo esos compases, y los que vienen
   después conservan lo que tocaban.
5. **El estilo de la canción**, arriba del menú, vuelve al estilo propio de la
   banda.

![El menú del compás 3: Variación a, Variación b y "Style: Pop beat" abiertos, con "The song's style: Pop beat" arriba del todo, luego una entrada por género (Pop, Rock, Blues, Jazz, Soul, funk y R&Si, … Classical and film). Classical and film está abierto: Epic toms, March, Chorale, Alberti bass, Arpeggios.](../images/step-9-section-styles-menu.png)

El menú solo ofrece estilos en el compás de esos compases. Un vals en
medio de una canción en 4/4 necesitaría compases de tres pulsos, lo que
significa cambiar los compases de la canción, no solo cómo los toca la
banda. Cambiar el compás es un paso aparte, así que aquí se queda
aparte.

## Qué cambia y qué se queda

El estilo de una sección cambia **cómo** toca la banda:

- el **comping** del piano, el ritmo con el que golpea los acordes,
- la **línea de bajo**,
- la **batería**,
- los patrones de la guitarra, las cuerdas y el órgano, donde la banda
  los tiene,
- el **swing**: una sección de swing en una canción pop *straight* toca
  sus corcheas con swing, y una sección de rock en una canción de jazz
  las toca parejas.

No cambia **quién** toca ni los sonidos con los que toca. La banda
conserva sus instrumentos, así que el mismo piano y el mismo bajo pasan
de la estrofa bossa nova al puente swing, como haría una banda real.
Para cambiar los instrumentos, usa el **Formación de instrumentos** de la banda.

La batería le avisa al oyente de que se acerca un cambio, como hace con
una variación: el compás anterior al cambio toca el relleno del estilo
que termina, y el primer pulso del estilo nuevo recibe un platillo
crash. El patrón de comping de cada músico vuelve a empezar desde su
comienzo en el primer compás del estilo nuevo, así que un patrón de
bossa nova de dos compases no empieza a mitad.

## Cómo funciona

### Marcadores, igual que las variaciones

```rust
pub struct StyleMarker {
    pub start: usize,
    pub style: Option<&'static BandStyle>,
}
```

Un marcador con `style: None` vuelve al estilo propio de la banda. Los
marcadores viven en el `Band`, junto a los marcadores de variación, y
siguen las mismas reglas: `tidy` conserva solo los marcadores que
cambian algo, y todo lo que mueve la canción (una intro añadida, un
corte, una sección copiada, un compás nuevo, «Terminar la canción aquí») los
mueve a ellos también.

`&'static BandStyle` es una referencia que vive durante todo el
programa: los estilos de banda se construyen una vez (`band_styles()`
los guarda en un `OnceLock`) y nunca se liberan. Por eso un marcador
puede guardar una referencia en lugar de una copia o un nombre. El
archivo no puede guardar una referencia, así que guarda el nombre del
estilo y lo vuelve a buscar cuando lee la canción.

### Cada estilo toca la canción entera, y luego cada uno se queda con sus compases

La manera más sencilla de escribir una parte con varios estilos no fue
enseñarle a cada pieza de la banda qué son las secciones. En su lugar,
`write_band` le pide a cada estilo que suena en algún momento la canción
entera, y conserva cada nota solo donde toca ese estilo:

```rust
for style in sections::styles_played(&band.styles) {
    let playing = band.playing(style);
    let part = self.band_notes(&playing, player, &score);
    notes.extend(sections::where_it_plays(part, &band.styles, style));
}
```

`band.playing(style)` es una copia de la banda con los patrones del otro
estilo. `band_notes` es el código que ya escribía la parte de un
músico, movido a un método propio para poder llamarlo una vez por
estilo. `where_it_plays` conserva las notas que empiezan donde toca
`style`, y corta una nota que seguiría sonando dentro del estilo
siguiente:

```rust
notes
    .into_iter()
    .filter(move |(_, note)| style_in(markers, note.start) == style)
    .map(move |(channel, mut note)| {
        if let Some(change) = next_change(markers, note.start)
            && note.start + note.length > change
        {
            note.length = change - note.start;
        }
        (channel, note)
    })
```

`move` hace que cada closure posea lo que usa (`markers` y `style`,
ambos baratos de copiar), así que el iterador puede devolverse de la
función y usarse cuando esta termina. Una canción sin estilos de
sección tiene un estilo, `None`, y conserva cada nota: suena como
sonaba.

Los rellenos salen de la misma idea. La batería ya tocaba un relleno en
el compás anterior a un cambio de variación; ahora los cambios de
estilo también son cambios. Cuando la banda toca el estilo que se va a
lo largo de toda la canción, ese estilo tiene un relleno antes del
cambio, y el compás anterior al cambio le pertenece, así que su relleno
se conserva. El estilo que llega tiene un crash en el cambio, y el
cambio es su primer compás, así que su crash se conserva.

### Swing por sección

El swing se toca, no se escribe: las notas se quedan en la cuadrícula,
y la reproducción mueve la segunda corchea de cada pulso más tarde
([Swing](swing.md)). La cantidad solía ser un número para toda la
canción. Ahora un patrón de banda en una sección con estilo propio hace
swing igual que ese estilo:

```rust
pub fn pattern_swing(&self, pattern: usize) -> u8 {
    let swing = self.section_swing(pattern).unwrap_or(self.swing);
    // ...
}
```

`section_swing` mira dónde está el primer clip del patrón y devuelve el
swing de esa sección, o `None` (el swing de la canción) para tus
propios patrones y para las secciones en el estilo propio de la banda.
La banda empieza un patrón nuevo donde empieza el estilo de una
sección, y dos tramos con las mismas notas pero distinto swing nunca
comparten patrón, así que cada patrón hace swing de una sola manera.

La [traza de la reproducción](playback-trace.md) lo muestra: en una
canción Ritmo pop *straight* cuya segunda mitad es Swing, el ride y las
notas a contratiempo del piano en el compás 5 empiezan a 2,667
dieciseisavos dentro del pulso, el último tercio de un tresillo,
mientras que en los compases 1 a 4 están en la cuadrícula.

Una canción con estilos de sección necesita al menos el formato 24 ([el
formato de archivo de proyecto](../appendix-project-format.md)).

## Ejercicios

1. Escribe | C | Am | F | G | dos veces en **Ritmo pop**, y pon los
   compases 5 a 8 en **Bossa nova**. Abre los patrones del bajo en el
   piano roll. ¿Dónde empieza la línea de bajo de la bossa nova, y en
   qué se diferencia de la del pop?
2. Pon solo el compás 3 de la misma canción en **Swing**. ¿Qué compases
   tienen relleno, y dónde caen los crash?
3. Toca una sección en **Coral**, que no tiene batería. ¿Qué toca ahí
   la batería, y qué toca en el compás anterior?
4. (Rust) `where_it_plays` devuelve `impl Iterator`. ¿Qué cambiaría si
   devolviera un `Vec`? ¿Por qué `notes.extend(...)` acepta ambos?
5. (Rust) `Band::playing` clona toda la banda. ¿Qué campos de `Band`
   encarecen ese clon, y por qué aquí no importa?
