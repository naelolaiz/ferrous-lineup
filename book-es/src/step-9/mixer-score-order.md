# El mixer en orden de partitura: la batería junta

Archivos: [`crates/lineup-core/src/score_order.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/score_order.rs)
y [`crates/lineup-app/src/app/mixer.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/mixer.rs)

El mixer solía listar los canales en el orden en que se añadían al
channel rack. Una canción nueva empieza con Kick, Clap, Hat, Snare y un
Lineup Osc; un estilo que necesita un rim shot, un bajo, un cowbell y
una conga los añade al final, así que el mixer leía Kick, Clap, Hat,
Snare, Lineup Osc, **Rim**, Bass, **Cowbell**, **Conga**: la batería
esparcida alrededor del piano y el bajo.

Ahora el mixer lista los canales como la partitura de un director lista
los instrumentos, en secciones con un encabezado cada una, y toda la
batería se sienta junta bajo **Drums**. Cada canal conserva su propio
volumen, mute y solo, exactamente como antes. Bajo la batería, la
columna Instrument dice ahora **Built-in drum** o **Sample** (un
archivo de sonido) en lugar de «Sample / drum».

![El mixer de la canción inicial tras Bossa nova y Salsa montuno: bajo «Drums», Cowbell, Clap, Conga, Hat, Snare, Rim y Kick; bajo «Harp and guitars», el Bass (Electric Bass, finger); bajo «Keyboards», el Lineup Osc tocando un Acoustic Grand Piano. Cada fila tiene su propio deslizador de volumen, su casilla de mute y su S de solo.](../images/step-9-mixer-score-order.png)

## Probarlo

1. Empieza desde **File > New**, o la canción que se abre con la app.
2. En la **Library**, haz clic en **Bossa nova** y luego en **Salsa
   montuno**. El channel rack termina ahora con Rim, Bass, Cowbell y
   Conga.
3. Pulsa **F9** para el mixer. La batería está bajo **Drums**, el bajo
   bajo **Harp and guitars**, el piano bajo **Keyboards**.
4. Haz clic en **S** en la fila de Rim: en el channel rack (**F6**),
   solo la luz de Rim se queda encendida. El mixer muestra las filas en
   otro orden, pero cada fila sigue perteneciendo a su propio canal.

Solo el mixer cambia su orden. El channel rack, el piano roll y el
archivo guardado conservan los canales en el orden en que se añadieron,
y nada suena distinto.

## Orden de partitura

Una partitura orquestal tiene un pentagrama por instrumento, y toda
partitura los pone en el mismo orden, para que un director encuentre
los oboes o los chelos sin buscar. El orden va por familias, y dentro
de una familia, sobre todo de agudo a grave:

| Sección | Instrumentos, de arriba abajo |
|---|---|
| Woodwinds | flautín, flautas, oboes, corno inglés, clarinetes, saxofones, fagotes |
| Brass | trompas, trompetas, trombones, tuba |
| Percussion | timbales; glockenspiel, xilófono, vibráfono, marimba, campanas tubulares; el resto |
| Drums | percusión de mano, y luego la batería |
| Harp and guitars | arpa, guitarras, banjo, koto, bajo eléctrico |
| Keyboards | celesta, piano, clavecín, órgano, acordeón y bandoneón |
| Synths | leads, pads y efectos de sintetizador; un Lineup Osc sin instrumento |
| Voices | una voz solista, y luego el coro |
| Strings | secciones de cuerda, violines, violas, chelos, contrabajos |

Las trompas van encima de las trompetas, aunque suenan más graves,
porque las trompas se funden con las maderas y durante mucho tiempo se
escribieron junto a ellas. Las cuerdas van al final, de modo que los
contrabajos, la parte más grave de la orquesta, cierran la página. Las
voces van justo encima de las cuerdas, como aparece un coro en un
réquiem o en un oratorio.

El mixer no mira el nombre de un canal para colocarlo. Mira el
instrumento: el programa General MIDI que toca el canal (mira [Sonidos
reales de instrumento](soundfont.md)). Un canal llamado «Lead» que toca
una flauta va con las maderas; uno que toca un violín va con las
cuerdas. Los números de programa ya vienen en su mayoría por familias
(los metales son del 56 al 63), pero no siempre como los quiere una
partitura: General MIDI pone los timbales con las cuerdas y la celesta
con la percusión de láminas, y la partitura pone los timbales primero
en la percusión y la celesta con los teclados.

### La batería

Una batería se escribe en un solo pentagrama, con cada tambor en su
propia línea o espacio: los platillos en y sobre la línea superior (el
crash el más alto, luego el hi-hat, luego el ride), el tom agudo en el
espacio superior, la caja en el medio, el tom de piso debajo y el kick
en el espacio inferior. La sección **Drums** sigue ese pentagrama de
arriba abajo: Crash, Hat, Open hat, Ride, High tom, Snare, Rim (un
cross-stick, escrito donde está la caja), Low tom, Kick. La percusión
de mano (Cowbell, Shaker, Clap, Conga) suele tener pentagramas propios,
encima de la batería, así que va primero. Un canal que toca un archivo
de sonido, y un canal MIDI Out que envía al canal MIDI 10 (la batería
de General MIDI), van también con la batería.

## El código

### Dónde va un canal

`score_order.rs`, en `lineup-core`, no tiene código de UI, así que se
puede probar con un simple `cargo test`. Un `Section` es un enum en
orden de partitura:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Section {
    Woodwinds,
    Brass,
    Percussion,
    Drums,
    HarpAndGuitars,
    Keyboards,
    Synths,
    Voices,
    Strings,
}
```

Derivar `PartialOrd` y `Ord` en un enum ordena sus variantes tal como
están escritas: `Section::Woodwinds < Section::Brass`. El orden de la
partitura es el orden del código fuente.

Cada sección lista sus programas, primero el de arriba de la partitura,
y `place` encuentra la sección de un canal y su posición dentro de
ella:

```rust
pub fn place(channel: &Channel) -> (Section, usize) {
    match &channel.kind {
        ChannelKind::Sampler(SampleSource::Builtin(drum)) => {
            let rank = DRUMS.iter().position(|d| d == drum).unwrap_or(DRUMS.len());
            (Section::Drums, rank)
        }
        ChannelKind::Sampler(SampleSource::File(_)) => (Section::Drums, DRUMS.len() + 1),
        ChannelKind::MidiOut {
            channel: DRUM_CHANNEL,
        } => (Section::Drums, DRUMS.len()),
        ChannelKind::Synth | ChannelKind::MidiOut { .. } => match channel.program {
            Some(program) => program_place(program),
            None => (Section::Synths, usize::MAX),
        },
    }
}
```

`ChannelKind::MidiOut { channel: DRUM_CHANNEL }` encaja solo con los
canales MIDI Out cuyo campo es igual a la constante `DRUM_CHANNEL` (9,
mostrado como 10). Un `const` puede aparecer en un patrón como un
número literal. La siguiente rama, `MidiOut { .. }`, atrapa cualquier
otro canal MIDI; `match` prueba las ramas desde arriba, así que el
canal de batería nunca llega tan lejos.

`program_place` busca el programa en la lista de cada sección:

```rust
fn program_place(program: u8) -> (Section, usize) {
    Section::ALL
        .into_iter()
        .find_map(|section| {
            let rank = section.programs().iter().position(|&p| p == program)?;
            Some((section, rank))
        })
        .unwrap_or((Section::Synths, usize::MAX))
}
```

`find_map` ejecuta el closure en cada sección y se detiene en la
primera que devuelve `Some`. Dentro, el `?` tras `position` devuelve
`None` desde el closure cuando el programa no está en esa sección, lo
que le dice a `find_map` que pruebe la siguiente. Una prueba comprueba
que cada uno de los 128 programas está en exactamente una lista, de
modo que ninguno se olvida y ninguno está en dos sitios.

### Ordenar y agrupar

```rust
pub fn score_order(channels: &[Channel]) -> Vec<(Section, Vec<usize>)> {
    let mut order: Vec<usize> = (0..channels.len()).collect();
    order.sort_by_key(|&index| place(&channels[index]));
    let mut sections: Vec<(Section, Vec<usize>)> = Vec::new();
    for index in order {
        let section = place(&channels[index]).0;
        match sections.last_mut() {
            Some((last, indices)) if *last == section => indices.push(index),
            _ => sections.push((section, vec![index])),
        }
    }
    sections
}
```

Ordena los **números** de los canales, no los canales: el mixer
necesita saber qué canal es cada fila, para cambiar el volumen de ese
canal y hacer solo al correcto. `sort_by_key` ordena por una clave
calculada para cada elemento; aquí la clave es el par `(Section,
usize)`, y las tuplas se comparan campo a campo, de modo que la sección
decide primero y el lugar dentro de ella desempata. `sort_by_key` es
una ordenación *estable*: dos canales con la misma clave, como «Lead»,
«Voice 2» y «Voice 3» cantando los tres Voice Oohs, conservan el orden
del rack.

El loop recorre luego la lista ordenada y empieza un grupo nuevo cada
vez que cambia la sección. `last_mut` da una referencia mutable al
último grupo, si lo hay; el `if` tras el patrón (una *guardia de
match*) solo toma la primera rama cuando ese grupo es la misma sección.

### La ventana del mixer

`mixer.rs` dibuja una fila de encabezado para cada sección y una fila
para cada uno de sus canales, usando el propio número del canal para
todo lo demás:

```rust
for (section, indices) in score_order(&levels) {
    ui.label(egui::RichText::new(section.name()).color(theme::ACCENT));
    ui.end_row();
    for index in indices {
        let channel = &mut levels[index];
        // name, instrument, volume, mute ...
        if ui.selectable_label(mix.solo == Some(index), "S").clicked() {
            mix.solo = if mix.solo == Some(index) { None } else { Some(index) };
        }
        ui.end_row();
    }
}
```

`levels` es la copia de los canales del mixer. Las filas la cambian en
el sitio a través de `levels[index]`; cuando la ventana termina, la app
compara la copia con el proyecto y hace un paso de deshacer por lo que
haya cambiado, como antes. La prueba
`the_mixer_keeps_the_drums_together_and_each_one_still_solos_alone`
dibuja el mixer, comprueba que un Ride añadido después de la banda se
dibuja entre el Hat y el Snare, hace clic en la **S** de su fila y
comprueba que el solo fue al canal del Ride.

## Ejercicios

1. Abre *In the Mood*, y luego *Sing, Sing, Sing*, con el mixer
   abierto. ¿En qué sección está el canal «Horns» de cada canción? Mira
   la columna Instrument para ver por qué.
2. Abre *Danny Boy*, haz clic derecho en su Lead en el channel rack y
   elige **Instrument: Violin > Pipe > 74 Flute**. ¿Adónde se mueve en
   el mixer?
3. En `score_order.rs`, los saxofones están entre los clarinetes y los
   fagotes, como los tienen la mayoría de las partituras orquestales.
   En una partitura de big band los saxofones van primero, encima de
   las trompetas. ¿Qué cambiarías para conseguir el orden de big band,
   y qué prueba te diría que rompiste el orquestal?
