# Ejercicios

## 1. Una ventana de historial de deshacer (lineup-app, fácil)

Añade Edit > History…, una ventana pequeña que lista cada paso que se
puede deshacer o rehacer, el más nuevo arriba, con la posición actual
marcada. Hacer clic en un paso salta hasta allí.

Necesitas dos métodos en `History`: uno que dé las etiquetas de deshacer
y otro las de rehacer, cada uno como iterador
(`self.undo.iter().map(|step| step.label)`). Saltar cinco pasos atrás son
cinco llamadas a
`history.undo`. ¿Debe ejecutarse `sync_audio` después de cada una, o una
vez al final, comparando con el proyecto de antes de la primera? ¿Qué es
más barato, y acaba el hilo de audio igual de cualquier manera? (Las
pruebas de diff del segundo capítulo responden a la última pregunta.)

*Prácticas:* iteradores sobre campos privados, `rev()`, reutilizar
`diff`.

## 2. Exportar un patrón como audio (lineup-app, fácil)

File > Export pattern as MIDI… ya escribe el patrón seleccionado como
archivo MIDI. Lo que falta es lo mismo como *audio*: "Export pattern…"
en el menú ⋯ del patrón, que guarde el patrón seleccionado, reproducido
cuatro veces, como archivo WAV.

El plan evidente es usar el modo patrón: `SetMode(Pattern)`,
`SelectPattern`, y renderizar cuatro longitudes de patrón más la cola.
Pero el modo patrón repite el patrón en loop eternamente, así que la cola
contendría el patrón empezando una quinta vez: el mismo problema que la
exportación resolvió con pasos en silencio, y el modo patrón no tiene
longitud de canción que estirar. En su lugar, haz una *copia* del
proyecto, sustituye su playlist por cuatro clips del patrón en una pista,
y exporta esa copia con el código que ya tienes. ¿Qué tiene que cambiar
en `export_song` para poder exportar un proyecto distinto de
`self.project`?

*Prácticas:* trabajar sobre una copia, hacer una función más general.

## 3. Exportar la región de loop (lineup-app, medio)

Detener una exportación ya está implementado: `export_wav_cancellable`
recibe un `AtomicBool`, la barra de estado tiene un botón Stop, y una
exportación cancelada quita su archivo temporal. Lee primero ese código;
también es el modelo para este ejercicio.

Las canciones pueden tener una región de loop, y el modo canción
reproduce solo esa parte. Añade File > Export loop region…, que escribe
solo la región en un archivo WAV. `Project::playback_arrangement()` ya
devuelve exactamente la parte que reproduce el modo canción, con las
repeticiones desenrolladas. Generaliza `song_render` para que pueda
recibir un `Arrangement` en lugar de llamar siempre a
`project.arrangement()`, y calcula el número de fotogramas a partir de la
propia longitud y los tempos de ese arrangement. ¿Qué nombre debería
tener el archivo? ¿Qué debería hacer el elemento de menú cuando no hay
región de loop, o cuando la región está vacía?

*Prácticas:* reutilizar un cálculo, hacer una función más general, pensar
en una entrada vacía.

## 4. Stems: un archivo por canal (lineup-app, medio)

Los músicos a menudo quieren "stems": la canción exportada una vez por
canal, cada archivo con solo ese canal, para mezclar en otro sitio.
Añade File > Export stems…, que pide una carpeta y escribe `Kick.wav`,
`Clap.wav` y así sucesivamente.

Cada stem es una exportación con todos los demás canales silenciados
(`SetMuted`). Renderizarlos uno tras otro es lo más sencillo;
renderizarlos en varios hilos a la vez es más rápido en un ordenador con
varios núcleos. Busca `std::thread::scope`: lanza hilos que pueden
*tomar prestado* de la función que los lanzó, porque el scope espera a
todos antes de terminar. ¿Por qué hace eso innecesaria la regla
`'static` del capítulo de la exportación? ¿Cómo debería mostrar una sola
barra de progreso varias exportaciones? ¿Y qué pasa con dos canales que
tienen el mismo nombre?

*Prácticas:* `thread::scope`, nombres de archivo a partir de texto del
usuario.

## 5. Ediciones al azar para el diff (lineup-core, medio)

Las pruebas de diff comprueban casos elegidos a mano. Escribe una que
compruebe mil casos al azar: empieza desde la demo, haz una edición al
azar (alterna un paso, añade o quita un patrón o un canal, cambia un
volumen, mueve un clip...), llama a `check_both_ways(&before, &after)`, y
repite desde el proyecto nuevo.

Para la aleatoriedad sin ningún crate, un pequeño generador "xorshift"
basta para las pruebas:

```rust
struct Random(u64);

impl Random {
    fn next(&mut self, below: usize) -> usize {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        (self.0 % below as u64) as usize
    }
}
```

Arráncalo desde un número fijo (no 0, que se queda en 0 para siempre),
para que un fallo se repita cada vez que ejecutes la prueba. Si quieres
ir más allá, el crate `proptest` hace esto como es debido, y cuando
encuentra un caso que falla lo *encoge* hasta la lista de ediciones más
pequeña que sigue fallando.

Prueba también varias ediciones al azar entre dos comprobaciones, como
podría contener un paso de deshacer. ¿Produce `diff` alguna vez una lista
larga donde bastaría una corta? ¿Es eso un problema?

*Prácticas:* pruebas basadas en propiedades, aleatoriedad reproducible.

## 6. Instantáneas más baratas (lineup-core, más difícil)

Cada paso de deshacer copia todo el proyecto, con todos los patrones,
aunque un clic en un paso cambie un solo patrón. (Un arrastre copia una
vez, cuando se abre su paso, no en cada fotograma, pero la copia sigue
siendo todo el proyecto.) Haz baratas las copias compartiendo lo que no
cambió: guarda los patrones como `Vec<Arc<Pattern>>`.

Clonar un `Vec<Arc<Pattern>>` copia solo los punteros y suma uno a cada
recuento; los propios patrones se comparten entre el proyecto actual y
cada copia del historial. Para cambiar uno, usa **`Arc::make_mut`**, que
da un `&mut Pattern` y primero hace una copia privada *solo si* el patrón
está compartido ("clone on write"). Así que `Project::pattern_mut` pasa a
ser `self.patterns.get_mut(index).map(Arc::make_mut)`.

Entonces `diff` puede saltarse rápido los patrones sin cambios: si
`Arc::ptr_eq` dice que dos patrones son la misma memoria, son iguales con
seguridad. Haz un proyecto de prueba con 999 patrones llenos de notas, y
mide el tiempo de una edición antes y después con `std::time::Instant`.
¿Sigue funcionando igual el `PartialEq` del proyecto? ¿Necesita cambiar
el formato de archivo?

*Prácticas:* `Arc` para compartir, clone on write, medir antes de
optimizar.

## 7. Piénsalo

- Las etiquetas de paso son `&'static str`, así que "rename pattern" no
  puede decir qué patrón. Una etiqueta `String` permitiría "rename
  pattern Beat", pero la mayoría de las etiquetas son texto fijo, y un
  `String` reservaría memoria para cada una. Busca
  `std::borrow::Cow<'static, str>`: ¿cómo deja que una etiqueta sea
  cualquiera de las dos, y reserva solo cuando debe?
- Deshacer mientras suena la canción. Cuando deshacer quita un canal, sus
  notas que resuenan se detienen al momento, porque el canal va a la
  basura. En el caso de los "tres Lineup Osc idénticos" del capítulo del
  diff, el hilo de audio quita el *último* canal y le da sus notas a
  otro. ¿La nota que resuena de qué canal sigue sonando, y podrías llegar
  a oír la diferencia?
- El volumen maestro, el patrón seleccionado y las ventanas abiertas no
  forman parte del proyecto, así que deshacer no los toca. ¿Cuál de ellos
  esperarías que deshacer cambiara? ¿Qué tendría que mudarse a `Project`
  para eso, y qué significaría para el archivo guardado?
- La exportación corre a 44,100 fotogramas por segundo aunque tu tarjeta
  de sonido corra a 48,000. Mira cómo se hace la batería (`drums::render`
  recibe la frecuencia de muestreo) y cómo reproduce el sampler un sonido
  grabado a otra frecuencia. ¿Podría una exportación sonar alguna vez
  distinta de lo que oyes?
- El contador de progreso usa `Ordering::Relaxed`. Supón que la ventana
  abriera el archivo en el momento en que el contador llega al total, sin
  esperar a `join`. ¿Qué podría encontrarse? (Mira qué hace `export_wav`
  después de su última llamada a `progress`.) ¿Lo arreglaría un
  `Ordering` más fuerte, o el problema está en otro sitio?
