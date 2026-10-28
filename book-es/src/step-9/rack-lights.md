# Luces de paso en el channel rack

Archivos: [`crates/lineup-app/src/widgets.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/widgets.rs)
y [`crates/lineup-app/src/channel_rack.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/channel_rack.rs)

Un step sequencer de hardware (una caja de ritmos) tiene una fila de
luces pequeñas, una sobre cada paso, y la luz del paso que se toca
corre con la música. El channel rack ya sabía qué paso estaba sonando,
pero solo dibujaba un contorno naranja fino alrededor de los botones de
ese paso, que era fácil de perder.

Ahora el rack tiene esa fila de luces sobre sus pasos. La luz sobre el
paso que suena está encendida en naranja con un brillo suave, y toda la
columna que hay debajo se tiñe de naranja también, en cada fila: los
pasos de batería y las vistas previas pequeñas de melodías y acordes
por igual.

![El channel rack de la canción inicial mientras suena: sobre los pasos, una fila de luces grises pequeñas, con la octava encendida en naranja; debajo, el octavo paso de Kick, Clap, Hat, Snare y Lineup Osc teñido de naranja.](../images/step-9-rack-lights-song.png)

## Probarlo

1. Abre la canción inicial y muestra el rack con **F6**. Muestra el
   patrón **Beat**.
2. Pulsa **L** para el modo patrón (**PAT** se enciende) y **Space**. La
   luz corre por los dieciséis pasos, una y otra vez.
3. Pulsa **Space** para parar, **L** para el modo canción (**SONG**) y
   **Space** otra vez. Los compases 1 y 2 no tienen clip de Beat, así
   que no hay ninguna luz encendida; desde el compás 3 la luz corre con
   los clips de Beat en el playlist.
4. Elige **Chords** en la lista de patrones del rack. Su fila de Lineup
   Osc es una vista previa de los acordes, y la columna que suena
   brilla sobre ella también.

![El rack tocando el patrón Beat por sí solo: la tercera luz está encendida y la tercera columna brilla.](../images/step-9-rack-lights-pattern.png)

Cuando el rack muestra un patrón y la canción está tocando otro, el
rack se queda en el patrón que elegiste, y sus luces siguen apagadas
hasta que ese patrón suena. Sin encender, la luz del primer paso de
cada compás es un poco más brillante que las demás, y la luz del primer
paso de cada pulso un poco más brillante que los pasos de en medio, así
que puedes contar la fila igual que cuentas los pasos que hay debajo.
En 3/4 o 6/8 los pulsos caen donde los pone ese compás.

## Cómo funciona

### Qué paso está sonando

Nada cambió en la reproducción. La app ya tenía
`LineupApp::pattern_playhead`: en modo patrón es el paso que informa el
motor de audio; en modo canción pregunta al proyecto en qué paso de
*este* patrón está la canción, con `Project::pattern_step_in_song`, que
busca un clip del patrón en una pista no silenciada que cubra el paso
actual de la canción. Si ninguno lo hace, devuelve `None`, y por eso
las luces se quedan apagadas mientras el patrón no suena.

### Pintar las luces después de las filas

Las luces deben quedar exactamente sobre los pasos, pero los pasos
empiezan después de la luz de silencio, la perilla, el bloqueo y el
nombre del canal, y sus anchos cambian con el zoom del rack. En lugar
de sumar esos anchos (y equivocarse la próxima vez que se añada un
control a una fila), el rack deja que las filas se lo digan:

```rust
let (lights, _) = ui.allocate_exact_size(
    egui::vec2(1.0, widgets::step_lights_height(ui)),
    egui::Sense::hover(),
);
let mut steps_left = None;
for (index, channel) in project.channels().iter().enumerate() {
    let row = ui.horizontal(|ui| {
        // ... mute light, knob, lock, name ...
        steps_left.get_or_insert(ui.cursor().left());
        // ... the steps ...
    });
}
if let Some(left) = steps_left {
    widgets::step_lights(ui, lights, left, &marks, playhead);
}
```

`allocate_exact_size` reserva una franja vacía arriba, así que las
filas van debajo. Mientras se coloca la primera fila,
`ui.cursor().left()` es donde está a punto de ir su primer paso;
`get_or_insert` conserva el primer valor e ignora las filas que vienen
después. Una vez terminadas las filas, `step_lights` pinta en la franja
reservada. egui dibuja todo al final del fotograma, así que pintar en
espacio reservado antes está bien.

### Las luces

`step_lights` recibe un slice de `StepMark`, un enum pequeño que dice
si cada paso empieza un compás, empieza un pulso, o ninguna de las dos
cosas:

```rust
pub enum StepMark {
    Bar,
    Beat,
    Plain,
}
```

El rack lo calcula desde el compás del patrón: un paso empieza un
compás cuando `step % steps_per_bar == 0`, y un pulso cuando
`time.beat_at(step).start` es el lugar de ese paso en el compás. Cada
luz se coloca un ancho de paso (el botón más el hueco que le sigue)
después de la anterior, la misma aritmética que usa `mini_roll`, así
que luces, botones y vistas previas se alinean a cualquier zoom.

### La columna que brilla

`step_button` ya recibía `is_playhead`. Ahora mezcla además un poco de
naranja en el color del botón con `Color32::lerp_to_gamma`, un poco
menos en un paso que está encendido, para que un paso blanco siga
siendo blanco. `mini_roll` rellena la columna que suena con un naranja
transparente (`gamma_multiply(0.25)` deja el color a un cuarto de
fuerza) bajo su contorno.

## Ejercicios

1. Haz verde la luz encendida en lugar de naranja. ¿Dónde está definido
   el color, y qué más lo usa?
2. Haz que las luces del primer paso de cada compás tengan un color
   distinto de las luces de pulso, en lugar de solo más brillante.
3. Mientras la canción toca un patrón distinto del que muestra el rack,
   las luces se quedan apagadas. Añade un aviso tenue en el rack, como
   «Not playing now», para ese caso. ¿Qué valores de
   `channel_rack::show` te dicen que la canción está sonando?
