# Exportar vídeo: la canción con las vistas tocando a la vez

Archivos: [`crates/lineup-video/`](https://github.com/naelolaiz/ferrous-lineup/tree/main/crates/lineup-video),
[`crates/lineup-app/src/video.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/video.rs),
[`crates/lineup-app/src/app/video.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/video.rs)

Un archivo WAV lleva la canción, pero no la armonía que ves mientras
suena. **Archivo > Exportar video…** guarda un archivo MP4 con el sonido y
las vistas que eliges siguiendo la música: el compás que suena
enmarcado en la hoja de acordes, el acorde iluminado en las teclas de armonía, el cursor de reproducción moviéndose por la lista de reproducción y el piano roll, y cada cambio de
tonalidad mostrado cuando llega.

![Un frame de El Choclo al minuto: el compás 29 enmarcado, E7 iluminado como V7](../images/step-9-video-chord-sheet.png)

## Usarlo

1. Abre una canción (o una demo) y elige **Archivo > Exportar video…**.
2. Marca las vistas que mostrar:
   - **Hoja de acordes**: la armonía compás a compás.
   - **Partitura guía**: la melodía en un pentagrama.
   - **Lista de reproducción**: las pistas y sus clips.
   - **Piano roll**: las notas del canal que tiene seleccionado, o la
     melodía.
   - **Teclas de armonía**: los acordes de la tonalidad y el piano pequeño.

   Las vistas marcadas comparten la imagen, una encima de otra. Una
   barra de título arriba nombra la canción, el compás y el pulso, la
   tonalidad, el compás y el tempo.
3. Organízalas en la vista previa, si quieres (mira abajo).
4. Elige un tamaño: **1280×720** (HD) o **1920×1080** (Full HD). Full HD
   dibuja el mismo layout una vez y media más grande, así que el texto
   es más nítido pero no cabe nada más.
5. Haz clic en **Exportar…** y elige dónde guardar el archivo `.mp4`.

La exportación corre en segundo plano, con su progreso en la barra de
estado y un botón **Detener** al lado. Cerrar la ventana mientras una
exportación WAV o de video corre pregunta primero, y la ventana se
cierra solo cuando la exportación se ha detenido. Una exportación
detenida borra su archivo temporal, y una fallida nunca reemplaza un
archivo más viejo: el archivo nuevo se renombra a su lugar solo cuando
está completo. El diálogo recuerda tus elecciones para la próxima vez.

![Lead sheet, playlist y piano roll juntos](../images/step-9-video-three-views.png)

## Organizar las vistas

El diálogo muestra una vista previa: la imagen que tendrá el video en el
compás que eliges con **Vista previa en**. Cada vista en ella tiene un marco
con su nombre y su zoom:

- **Arrastra una vista** para mover su caja.
- **Arrastra su esquina** (el cuadrado pequeño) para cambiar el tamaño
  de la caja.
- **Haz scroll sobre ella** para acercar o alejar, de 50% a 250%. El
  slider junto a la marca de cada vista hace lo mismo.
- **Apilar** vuelve a poner las vistas una encima de otra, compartiendo
  la imagen. Marcar o desmarcar una vista también las apila.

Las cajas pueden superponerse: la vista más abajo en la lista se dibuja
encima. La partitura guía no tiene zoom: siempre llena el ancho de su caja,
así que una caja más ancha lo dibuja más grande. El diálogo recuerda el
layout para el próximo video.

![El diálogo Export video: el chord sheet movido a la derecha y con zoom al 250%](../images/step-9-video-layout.png)

Aquí la hoja de acordes, con zoom al 140%, se sienta junto a la partitura guía,
con las teclas de armonía al 120% debajo:

![El Choclo a 1920×1080: chord sheet y lead sheet lado a lado](../images/step-9-video-side-by-side.png)

## Nada que instalar

La manera habitual de hacer un video desde un programa es entregar
imágenes a FFmpeg. Pero FFmpeg es un programa aparte que Windows y macOS
no tienen, así que cada usuario tendría que instalarlo. En su lugar, la
aplicación compila dos codificadores muy conocidos desde sus fuentes en
C junto con ella misma, a través de dos crates:

| Qué | Formato | Codificador | Crate |
|------|--------|---------|-------|
| Imágenes | H.264, 30 por segundo | OpenH264 de Cisco (licencia BSD) | `openh264` |
| Sonido | MP3, 256 kbit/s estéreo | LAME (LGPL) | `mp3lame-encoder` |
| Ambos juntos | MP4 | escrito a mano, unas 500 líneas | (nuestro) |

Un crate con C dentro tiene un `build.rs` que ejecuta un compilador de C
antes de que Rust compile el resto, usando el crate [`cc`](https://docs.rs/cc).
La matriz de CI ya compila en Linux, Windows y los dos tipos de Mac, así
que prueba que esto funciona en todas partes.

H.264 y MP3 no son los formatos más nuevos. Se eligieron porque cada
reproductor, teléfono, navegador y sitio de videos los lee.

## El crate nuevo: `lineup-video`

El crate no sabe nada de canciones. Toma imágenes y sonido y escribe un
archivo:

```rust
let mut writer = VideoWriter::create(&path, settings)?;
for each picture {
    writer.add_picture(&rgba)?;      // or writer.repeat_picture()
    writer.add_sound(&samples)?;     // what plays while it is shown
}
writer.finish()?;
```

Tiene cuatro módulos:

- **`canvas`** pinta las shapes de egui en una imagen en memoria (mira
  abajo).
- **`h264`** convierte cada imagen en H.264. Las imágenes llegan como
  RGB y el codificador quiere YUV: brillo para cada píxel, y color para
  cada cuadrado de cuatro, porque el ojo ve el detalle en el brillo
  mucho mejor que en el color. Después de la primera imagen, el
  codificador solo guarda lo que cambió. Una pantalla donde solo se
  mueve un marco no cuesta casi nada. Cada dos segundos hay una *imagen
  clave* que se sostiene sola, para que un reproductor pueda saltar.
- **`mp3`** convierte el sonido en *frames* de MP3 de 1152 muestras cada
  uno, y los separa leyendo la cabecera de 4 bytes de cada frame.
- **`mp4`** escribe el archivo.

`VideoWriter::create` escribe en `name.mp4.tmp` y lo renombra al final,
así que una exportación fallida nunca reemplaza un video más viejo con
la mitad de uno nuevo.

## Dentro de un archivo MP4

Un archivo MP4 es un árbol de **cajas**. Cada caja es una longitud de 4
bytes, un nombre de 4 letras, y su contenido, que pueden ser más cajas:

```text
ftyp                   «esto es un archivo MP4»
mdat                   todas las imágenes y el sonido comprimidos, una cosa tras otra
moov                   el índice:
  mvhd                 cuánto dura todo el conjunto
  trak                 las imágenes:
    tkhd, mdia …       tamaño, reloj, y las tablas de abajo
      avc1 + avcC      «H.264», y los conjuntos de parámetros del codificador
      stts             cuánto se muestra cada imagen
      stss             qué imágenes son imágenes clave
      stsz, stco       cuán grande es cada una, y dónde está en mdat
  trak                 el sonido:
    edts/elst          dónde empieza de verdad el sonido (mira abajo)
    mp4a + esds        «MP3»
    stts, stsz, stco   las mismas tablas para los frames de MP3
```

El índice tiene que venir después de los datos. Hasta el final, nadie
sabe cuán grande es cada imagen ni dónde aterrizó. Así que `Mp4Writer`
escribe `mdat` sobre la marcha, guarda una lista de tamaños y offsets, y
escribe `moov` en `finish`. Luego vuelve atrás y rellena la longitud de
`mdat`, como 64 bits, ya que un video largo en Full HD puede pasar de
4 GB.

`stts` guarda *rachas*: «300 imágenes de 1/30 s cada una» ocupa una
entrada, no 300. Eso es lo que hace barato saltarse imágenes sin cambios
(mira abajo). `repeat_picture` solo hace que la última imagen dure
1/30 s más.

### Hacer que el sonido empiece a tiempo

Un codificador MP3 pone 576 muestras de silencio antes del sonido, y un
decodificador añade 529 más. Sin un arreglo, la música llegaría 25 ms
tarde. Eso es casi una imagen, y basta para ver un acorde iluminarse
antes de oírlo. La **lista de edición** (`elst`) de la pista de sonido
le dice al reproductor que se salte esas 1105 muestras:

```rust
write_full_box(out, b"elst", 0, 0, |out| {
    out.extend_from_slice(&1_u32.to_be_bytes());               // one edit
    out.extend_from_slice(&(duration_ms as u32).to_be_bytes()); // as long as the song
    out.extend_from_slice(&format.priming.to_be_bytes());       // start 1105 samples in
    …
```

Para la prueba, se exportó un sonido de clic a los 1.0, 2.5 y 4.0 s,
con un destello blanco en las imágenes en los mismos momentos. Después
de decodificar el archivo con FFmpeg, los clics estaban exactamente a
los 1.0, 2.5 y 4.0 s, y los destellos en las imágenes 30, 75 y 120.

## Dibujar la aplicación sin ventana

Las vistas son código egui normal. La hoja de acordes en el video *es* la hoja de acordes en la aplicación, a través de pequeñas funciones `video` que
llaman al mismo código de dibujo sin los menús. egui no dibuja píxeles
él mismo. Cada frame, convierte la interfaz en una lista de
**triángulos** con colores y coordenadas de textura (la textura guarda
las letras de la fuente). En la aplicación, `egui_glow` envía esos
triángulos a la tarjeta gráfica.

La exportación no puede contar con una tarjeta gráfica, y corre en un
hilo propio. Así que hace un segundo `egui::Context` sin ventana, le da
una pantalla inventada de 1280×720, y pinta los triángulos en la CPU en
`Canvas`:

```rust
let mut output = ctx.run_ui(input, |ui| self.ui(ui, at)); // lay out the views
let primitives = ctx.tessellate(output.shapes, ppp);      // into triangles
self.canvas.paint(primitives, &mut output.textures_delta, ppp, background);
```

Para pintar un triángulo, visitas cada píxel cerca de él y preguntas
«¿está este píxel dentro?». La prueba es la **función de borde**: para
un borde de La a Si, este producto cruzado es positivo en un lado y
negativo en el otro.

```rust
fn edge(from: (i64, i64), to: (i64, i64), p: (i64, i64)) -> i64 {
    (to.0 - from.0) * (p.1 - from.1) - (to.1 - from.1) * (p.0 - from.0)
}
```

Un píxel está dentro cuando los tres bordes coinciden. Los mismos tres
números, divididos por su total, dicen cuán cerca está el píxel de cada
esquina. Eso mezcla los colores y las coordenadas de textura de las
esquinas, que es como las letras obtienen sus formas. Unos pocos
detalles hacen que coincida con la pantalla:

- **Enteros, 1/256 de píxel.** Dos triángulos que comparten un borde no
  deben pintar ambos los píxeles que hay en él, o una forma
  semitransparente obtiene una costura más oscura. Con números enteros,
  la «regla de arriba a la izquierda» resuelve los empates exactamente.
- **Colores premultiplicados.** egui guarda los colores ya multiplicados
  por su opacidad, y mezcla con `new + old × (1 − alpha)`, como hace su
  shader de la tarjeta gráfica.
- **Paso a paso, no multiplicando.** Moverse un píxel a la derecha
  cambia cada función de borde por una constante, así que el bucle
  interno suma en lugar de multiplicar. Para los triángulos simples de
  un solo color (la mayor parte de una interfaz), un camino rápido se
  salta la textura del todo.
- **Hilos.** La imagen se corta en bandas de filas, y cada banda se
  pinta en su propio núcleo con `std::thread::scope`, que deja a los
  hilos tomar prestada la imagen porque todos terminan antes de que
  devuelva.

### Una caja para cada vista

Cada vista tiene un **pane**: un `egui::Context` propio y un `Canvas` del
tamaño de su caja. El zoom funciona como el zoom propio de egui: el pane
le dice a su contexto que hay más píxeles por punto (1.4 veces más al
140%), así que egui distribuye la vista en menos puntos y dibuja cada
letra al tamaño mayor. Si la vista se dibujara pequeña y la imagen se
estirara después, las letras saldrían borrosas.

```rust
let ppp = self.pixels_per_point;            // the picture's, times the zoom
let size = vec2(width as f32, height as f32) / ppp;
let mut input = egui::RawInput {
    screen_rect: Some(Rect::from_min_size(Pos2::ZERO, size)),
    ..
};
input.viewports.entry(egui::ViewportId::ROOT).or_default()
    .native_pixels_per_point = Some(ppp);
```

Cada pane pinta su propio canvas. `Canvas::copy_from` copia entonces los
panes en la imagen, en sus cajas, debajo de la línea de título. Mover
una caja solo copia otra vez. Una caja cuyo tamaño o zoom cambió recibe
un canvas nuevo y se dibuja dos veces, porque egui mide algunas cosas en
su primer frame.

Dos detalles mantienen las vistas en el pulso:

- **Sin deslizamiento.** En la ventana, una vista se desliza suavemente
  hasta el compás que suena, tardando medio segundo. En un video eso es
  medio segundo tarde, así que los panes apagan la animación de scroll
  de egui y saltan allí.
- **Ningún frame tarde.** Una vista que hace scroll solo se mueve en el
  siguiente frame, y egui pide uno enseguida (un retardo de repintado de
  cero). El pane dibuja otra vez dentro de la misma imagen, así que el
  compás se muestra en su primer pulso.

### Imágenes que no cambiaron

Cuando una canción suena, la mayoría de los frames se parecen al
anterior: la hoja de acordes solo se mueve cuando cambia el compás.
`Canvas::paint` guarda los triángulos del último frame. Si los nuevos
son iguales (un `Vec` de meshes comparado con `==`), no pinta, el
codificador no corre, y la última imagen simplemente dura más. Eso, y el
rasterizador más rápido, bajaron El Choclo (101 s) de 88 s de
exportación a 14 s.

## Mantener la imagen con el sonido

La canción la toca el mismo `Renderer` sin conexión que una exportación
WAV, 1/30 s a la vez: a 44 100 muestras por segundo, exactamente 1470
muestras por imagen. Las imágenes deben mostrar *dónde está la música*,
y con cambios de tempo, ritardandos y fermatas eso no es una suma
simple. Así que el renderer lo informa. Después de cada bloque de
sonido, el secuenciador guarda su posición en pasos, el mismo número que
lee el cursor de reproducción de la aplicación, y `Renderer::position` la devuelve:

```rust
for picture in 0..pictures {
    let position = if picture == 0 { Some(0.0) } else { renderer.position() };
    match scene.draw(time, position) {
        Some(rgba) => writer.add_picture(rgba)?,
        None => writer.repeat_picture(),     // nothing changed
    }
    renderer.render(&mut sound);             // the next 1/30 s
    writer.add_sound(&sound)?;
}
```

La posición cuenta los pasos *como se tocaron*. Con una repetición o un
estribillo de la forma de la canción, ese no es el lugar en la lista de reproducción.
`Scene::draw` lo devuelve con el arreglo de la canción (`pass_at`), como
hace la aplicación con su cursor de reproducción.

Para comprobarlo, se exportó El Choclo y se midió el volumen cada 10 ms.
A 112 BPM en 4/4, el compás 15 empieza a los 30.0 s, el compás 29 a los
60.0 s y el compás 43 a los 90.0 s. Las notas empiezan exactamente en
esos tiempos, y las imágenes allí muestran «Compás 15, pulso 1», «Compás 29,
pulso 1» y «Compás 43, pulso 1».

## Qué probar

- Exporta una demo solo con la **hoja de acordes** y las **teclas de armonía**. Es
  un video para tocar encima y practicar los cambios.
- Exporta con la **partitura guía** para ver la melodía mientras se canta o
  se toca.
- Abre el archivo en cualquier reproductor y salta de un lado a otro.
  Una imagen clave cada dos segundos significa que empieza a dibujar
  enseguida.
