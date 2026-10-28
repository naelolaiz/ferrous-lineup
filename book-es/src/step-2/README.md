# Paso 2: El channel rack

> El código que muestran los capítulos del paso 2 es el código tal como
> era al final del paso 2. El paso 3 cambió parte de él: los patrones
> ahora guardan notas en lugar de filas `[bool; 64]`,
> `Command::SetStep` pasó a ser `Command::SetNotes`, y los instrumentos
> ganaron voces. El paso 4 cambió más: un proyecto contiene muchos
> patrones, así que `SetNotes` también nombra el patrón, y el rack se
> convirtió en una ventana flotante que muestra el patrón seleccionado.
> Este capítulo es una instantánea histórica, y la antigua rama de
> checkpoint `step-2-sequencer` ya no se publica.

El paso 2 convierte el tono de prueba en una caja de ritmos real, aunque
pequeña:

- un **channel rack** con una fila por instrumento, cada una con una luz
  de silencio, una perilla de volumen, un botón de nombre y una fila de
  **pasos**,
- un **step sequencer** que reproduce el patrón en loop al tempo del
  proyecto, con temporización exacta a la muestra y un playhead en
  movimiento,
- una **batería incorporada** (bombo, caja, palmas, hi-hat) sintetizada
  desde cero, para que no haya que distribuir archivos de sonido,
- un **sampler** que reproduce archivos WAV que arrastras al rack,
- el **Lineup Osc**, un canal de sintetizador simple (una onda de diente
  de sierra filtrada con una envolvente de pulsación) al que el paso 3 le
  dará melodías.

![El channel rack reproduciendo el ritmo de demostración](../images/step-2-rack.png)

## Cómo usarlo

| Haz esto | Para |
|----------|------|
| Haz clic en un paso | encenderlo o apagarlo |
| Haz clic derecho en un paso | apagarlo |
| Space o ▶ Play | iniciar y detener el patrón |
| Haz clic en el nombre de un canal | oír ese canal una vez |
| Haz clic derecho en el nombre de un canal | eliminar el canal |
| Arrastra la perilla arriba o abajo | cambiar el volumen del canal (el doble clic lo restablece) |
| Haz clic en la luz verde | silenciar o activar el canal |
| Suelta un archivo `.wav` sobre un canal | sustituir su sonido por el archivo |
| Suelta un archivo `.wav` debajo de las filas | añadir un canal nuevo que lo reproduzca |
| ➕ Add channel | añadir un tambor o un Lineup Osc |

## Qué cambió en la arquitectura

```text
  Hilo de la interfaz (lineup-app)              Hilo de audio (lineup-audio)
  ──────────────────────────────────────────  ───────────────────────────
  Proyecto (lineup-core)                        AudioThread
   ├─ tempo                                     ├─ Sequencer (lineup-core)
   ├─ canales   ──▶ Cola de comandos ──▶        ├─ canales: Vec<ChannelState>
   └─ patrón                                    │    └─ Instrument + pasos
                                                │
  channel rack UI    ◀── playhead (atómico) ────┤
                                                │
  collect_garbage() ◀── cola de basura ─────────┘  instrumentos por liberar
```

Tres ideas nuevas sostienen este paso:

1. **La interfaz posee el proyecto; el hilo de audio guarda su propia
   copia** de justo lo que necesita para hacer sonido. Cada cambio del
   usuario se aplica al proyecto y también se envía como `Command`. El
   hilo de audio nunca lee los datos de la interfaz, así que los dos
   nunca necesitan un cerrojo.
2. **La información también vuelve sin cerrojos.** El paso actual vuelve
   a través de un entero *atómico*, y los instrumentos que el hilo de
   audio ya no necesita vuelven por un segundo ring buffer para que su
   memoria se libere en el hilo de la interfaz.
3. **Dibujar se separa de cambiar.** El código de dibujo del rack
   devuelve una lista de `RackAction`s, y la app las aplica en un solo
   lugar.

## Archivos de este paso

```text
crates/lineup-core/src/pattern.rs     Pattern: la cuadrícula de pasos
crates/lineup-core/src/sequencer.rs   Sequencer: ¿cuándo empieza cada paso?
crates/lineup-core/src/project.rs     Project, Channel, ChannelKind, Drum
crates/lineup-audio/src/dsp.rs        Smoothed, Noise, HighPass, LowPass
crates/lineup-audio/src/drums.rs      la batería sintetizada
crates/lineup-audio/src/sample.rs     SampleData y la carga de WAV
crates/lineup-audio/src/instrument.rs Instrument: Sampler y Synth (Lineup Osc)
crates/lineup-audio/src/engine.rs     comandos nuevos, la cola de basura, el playhead
crates/lineup-app/src/channel_rack.rs el rack: filas, pasos, arrastrar y soltar
crates/lineup-app/src/widgets.rs      botón de paso, perilla y luz de silencio
crates/lineup-app/src/app.rs          aplica las acciones del rack al proyecto y al audio
```

El tono de prueba del paso 1 (`synth.rs`) ya no está. Su truco de
volumen sin clics sigue vivo como `dsp::Smoothed`, y su oscilador creció
hasta convertirse en el Lineup Osc.
