# Paso 1: Los fundamentos

> El código que muestran los capítulos del paso 1 es el código tal como
> era al final del paso 1. Pasos posteriores cambiaron parte de él (el
> tono de prueba, por ejemplo, se reemplazó en el paso 2).

El paso 1 construye el esqueleto del que cuelga cada función posterior:

- una **ventana** con una transport bar (botón de reproducir, tempo,
  controles del tono de prueba) y una barra de estado,
- un **motor de audio en tiempo real** que abre la tarjeta de sonido y
  reproduce un tono de prueba de onda senoidal,
- una **cola de mensajes** que deja que la ventana controle el audio sin
  hacer esperar nunca al audio,
- **pruebas** para el código musical y de audio, e **integración
  continua** que compila y prueba cada cambio en Linux.

Todavía no hace música. Eso empieza en el paso 2, con el step sequencer.
Pero la decisión arquitectónica más difícil, cómo se comunican la
interfaz y el hilo de audio, se toma aquí, y todo lo demás se apoya en
ella.

## El panorama general

```text
  Hilo de la interfaz (lineup-app)          Hilo de audio (lineup-audio)
  dibuja la ventana ~60 veces por segundo   pide muestras cada pocos ms
  reacciona a clics y teclas                jamás espera ni reserva memoria

  LineupApp ──Command──▶ ring buffer (rtrb) ──▶ AudioThread ──▶ tarjeta de sonido
                                                     └── TestTone

  Ambos lados usan lineup-core (Note, Tempo): lógica musical pura, sin I/O.
```

## Archivos añadidos en este paso

```text
Cargo.toml                    el workspace: enumera los tres crates
crates/
  lineup-core/src/lib.rs        raíz del crate, reexporta Note y Tempo
  lineup-core/src/pitch.rs      Note: números MIDI, nombres, frecuencias
  lineup-core/src/tempo.rs      Tempo: pulsos por minuto, duración del pulso
  lineup-audio/src/lib.rs       raíz del crate
  lineup-audio/src/synth.rs     TestTone: un generador senoidal sin clics
  lineup-audio/src/engine.rs    AudioEngine: tarjeta de sonido, hilo de audio, comandos
  lineup-app/src/main.rs        punto de entrada del programa, abre la ventana
  lineup-app/src/app.rs         LineupApp: todo lo que se dibuja en la ventana
docs/                         este libro (hoy en book/)
.github/workflows/ci.yml      compilación y pruebas automáticas en GitHub
```

Los capítulos siguientes recorren estos archivos en orden, de fuera hacia
dentro.
