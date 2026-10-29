# lineup-app: la ventana

Archivos:
[`crates/lineup-app/src/main.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/main.rs)
y
[`crates/lineup-app/src/app.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app.rs)

La ventana se dibuja con [egui](https://github.com/emilk/egui), una
librería de GUI escrita en Rust, y [eframe](https://docs.rs/eframe), el
framework de egui para ejecutarla como app nativa en Linux, macOS y
Windows (e incluso en un navegador web).

## Modo inmediato

La mayoría de los toolkits de GUI son de **modo retenido**: creas los
objetos botón una vez, los guardas en un árbol y les adjuntas callbacks.
egui es de **modo inmediato**: en cada frame, llamas a una función que
describe *toda* la interfaz a partir de tus propios datos, y la librería
la dibuja y te dice en qué se ha hecho clic.

```rust
if ui.button("▶ Play").clicked() {
    self.set_playing(true);
}
```

Esa línea dibuja el botón y comprueba si se ha hecho clic en él. No hay
objetos botón que mantener sincronizados con tu estado. El estado son
simplemente los campos de tu struct, y la interfaz es siempre una imagen
de él. Esto encaja bien con Rust, porque no hay callbacks que retengan
referencias a tus datos.

## `main.rs`

```rust
mod app;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_title("Ferrous Lineup")
            .with_inner_size([960.0, 600.0])
            .with_min_inner_size([640.0, 400.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Ferrous Lineup",
        options,
        Box::new(|cc| Ok(Box::new(app::LineupApp::new(cc)))),
    )
}
```

- `main` devuelve un `Result`. Si devuelve `Err`, Rust imprime el error
  y sale con un código de fallo.
- `ViewportBuilder::default().with_title(...)` es el **patrón builder**:
  cada método toma el builder y lo devuelve modificado, para que las
  llamadas se encadenen.
- `..Default::default()` es la **sintaxis de actualización de structs**:
  «todo campo que no enumere toma su valor por defecto».
- `Box::new(|cc| ...)` pasa un closure que eframe llama una vez para
  crear nuestra app. `Box` pone un valor en el **montículo**. Aquí hace
  falta porque eframe acepta *cualquier* tipo que implemente su trait
  `App`, y para guardar un valor cuyo tipo exacto no conoce necesita un
  puntero a él: `Box<dyn App>`. La palabra clave `dyn` significa «algún
  tipo que implementa este trait, decidido en tiempo de ejecución» (un
  **trait object**).

## `LineupApp`: el estado

```rust
enum Audio {
    Running(AudioEngine),
    Unavailable(String),
}

pub struct LineupApp {
    audio: Audio,
    playing: bool,
    tempo: Tempo,
    tone_note: Note,
    volume: f32,
}
```

El enum `Audio` captura una situación real: o tenemos un motor en
marcha, o tenemos la razón por la que no. No puedes tener ambas cosas, y
no puedes no tener ninguna, así que el tipo no puede representar un
estado imposible. Compáralo con la alternativa habitual, un
`Option<AudioEngine>` más un `Option<String>`, que permite cuatro
combinaciones de las cuales dos no tienen sentido.

Usarlo:

```rust
fn send(&mut self, command: Command) {
    if let Audio::Running(engine) = &mut self.audio {
        engine.send(command);
    }
}
```

`if let` es un `match` de una sola rama: si `self.audio` es `Running`,
liga el motor que hay dentro a `engine` y ejecuta el bloque. Hacer match
sobre `&mut self.audio` toma prestado el motor de forma mutable en lugar
de moverlo fuera del struct.

## Implementar `eframe::App`

```rust
impl eframe::App for LineupApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let typing = ui.ctx().memory(|memory| memory.focused().is_some());
        if !typing && ui.ctx().input(|input| input.key_pressed(egui::Key::Space)) {
            self.set_playing(!self.playing);
        }

        egui::Panel::top("transport").show(ui, |ui| self.transport_bar(ui));
        egui::Panel::bottom("status").show(ui, |ui| self.status_bar(ui));
        egui::CentralPanel::default_margins().show(ui, |ui| self.welcome(ui));
    }
}
```

Aquí es donde nuestro tipo cumple la promesa del trait. eframe llama a
`ui` cada vez que la ventana necesita repintarse.

- Espacio alterna la reproducción, como en el estudio original, salvo que
  un campo de texto tenga el foco del teclado (para que puedas escribir
  en la casilla del tempo).
- `ui.ctx().input(|input| ...)` da a un closure acceso temporal al
  estado de entrada. egui usa mucho este estilo, para poder bloquear sus
  datos internos solo durante la duración de tu closure.
- La ventana tiene tres **paneles**: una barra superior (la transport
  bar), una barra inferior (la barra de estado) y el área central. Cada
  panel recibe un closure que dibuja su contenido. Los paneles deben
  añadirse antes del panel central, que ocupa el espacio restante.
- Los closures llaman a métodos como `self.transport_bar(ui)` para que
  `ui` no crezca. Pueden tomar prestado `self` de forma mutable porque
  cada closure se ejecuta y termina antes de que empiece el siguiente,
  algo que el borrow checker puede ver.

## La barra de transporte

```rust
let mut bpm = self.tempo.bpm();
let tempo_field = egui::DragValue::new(&mut bpm)
    .range(Tempo::MIN..=Tempo::MAX)
    .speed(0.5)
    .fixed_decimals(1)
    .suffix(" BPM");
if ui.add(tempo_field).on_hover_text("Tempo").changed() {
    self.tempo = Tempo::new(bpm);
}
```

- El campo de `Tempo` es privado, así que copiamos el valor a un `bpm`
  local, dejamos que el widget lo edite y construimos un `Tempo` nuevo
  si cambió. El tipo `Tempo` sigue siendo válido escriba lo que escriba
  el usuario.
- `&mut bpm` presta la variable al widget de forma mutable, para que el
  widget pueda escribir el nuevo valor directamente en ella.
- `Tempo::MIN..=Tempo::MAX` es un **rango inclusivo**. `a..b` excluye
  `b`, `a..=b` lo incluye.

El deslizador del tono de prueba muestra nombres de nota en lugar de
números:

```rust
let note_slider = egui::Slider::new(&mut midi, TONE_RANGE)
    .text("Test tone")
    .custom_formatter(|n, _| Note::new(n as u8).to_string());
```

El formateador es un closure que convierte el número del deslizador en
texto usando la implementación de `Display` de `lineup-core`. El `_`
ignora el segundo argumento, que no necesitamos.

Cuando cambia un valor, la app envía un `Command` al hilo de audio, por
ejemplo `Command::SetFrequency(self.tone_note.frequency())`. La interfaz
nunca espera respuesta; el hilo de audio lo aplica en su siguiente
bloque, unos milisegundos después.

## La barra de estado

```rust
match &self.audio {
    Audio::Running(engine) => { /* show device name, sample rate, channels */ }
    Audio::Unavailable(reason) => {
        ui.colored_label(ui.visuals().warn_fg_color, format!("Audio unavailable: {reason}"));
    }
}
```

`format!` funciona como `println!` pero devuelve un `String`. Dentro de
las llaves, `{reason}` imprime la variable `reason` directamente;
`{:.1}`, más adelante en el archivo, imprime un float con un decimal.

Si la computadora no tiene una tarjeta de sonido que funcione, la app se
abre igualmente y la barra de estado muestra el motivo en naranja. Un
fallo se convierte en datos que mostramos, en lugar de en un crash.
