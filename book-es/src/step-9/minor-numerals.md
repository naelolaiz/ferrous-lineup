# Los números romanos en tonalidades menores: bIII o III

Archivos: [`crates/lineup-core/src/harmony/scale.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/harmony/scale.rs),
[`crates/lineup-core/src/library/numeral.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-core/src/library/numeral.rs),
[`crates/lineup-app/src/app/settings.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/settings.rs)
y [`crates/lineup-app/src/app/video.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/video.rs)

En La menor, el acorde Do mayor tiene dos nombres. Un libro de jazz o de
pop lo llama **bIII**; un libro de armonía clásica lo llama **III**. Los
dos tienen razón. Hasta ahora Ferrous Lineup solo escribía el primero.
Si estudias con un libro clásico, todo número romano de tonalidad menor
que la app te mostraba en el tercer, sexto o séptimo grado parecía
equivocado, aunque no lo estuviera.

**Edit > Settings… > Harmony > Roman numerals in minor keys** te deja
elegir ahora. Los acordes no cambian, solo cómo se escriben.

![Settings > Harmony: dos maneras de contar los grados de una tonalidad menor, con La menor como ejemplo.](../images/step-9-minor-numerals-setting.png)

## Dos maneras de contar

Un número romano dice sobre qué grado de la tonalidad está construido
un acorde: I sobre la primera nota, V sobre la quinta. Su caja indica la
cualidad del acorde: mayúscula para mayor (V), minúscula para menor
(ii), ° para disminuido (vii°), + para aumentado (III+). La pregunta es
qué significa el número romano sin alteración: ¿construido sobre **qué**
nota?

Toma La menor. Su escala menor natural es La Si Do Re Mi Fa Sol, y sus acordes
son Lam, Si°, Do, Rem, Mim, Fa y Sol.

**Contados desde la escala mayor.** Los números romanos se miden contra
la escala mayor sobre la misma tónica, La mayor: La Si Do♯ Re Mi Fa♯ Sol♯. Do está
un semitono por debajo de Do♯, así que su acorde es un III **bemol**:
bIII. De la misma manera, Fa es bVI y Sol es bVII:

```text
Am   B°    C     Dm   Em   F     G
i    ii°   bIII  iv   v    bVI   bVII
```

Los bemoles dicen «más bajo que en mayor». La ventaja es que una sola
tabla de progresiones sirve en toda tonalidad y modo: «bVI bVII i» es la
misma progresión en La menor, La dórico o La eólico, y una canción en Do
mayor que toma prestados La♭ y Si♭ de Do menor los escribe bVI y bVII
también. Así escriben los números romanos la mayoría de los libros de
pop y de jazz, y la biblioteca de la app.

**Contados desde la escala menor.** Los números romanos se miden contra
la escala propia de la tonalidad, la que da su armadura. Do es
sencillamente la tercera nota de La menor, así que su acorde es III:

```text
Am   B°    C     Dm   Em   F     G
i    ii°   III   iv   v    VI    VII
```

Así los escriben los libros de armonía clásica. III es la relativa
mayor, la tonalidad a la que más a menudo va una pieza en tonalidad
menor.

Solo tres grados se escriben distinto: el tercero, el sexto y el
séptimo. Son las notas que difieren entre La mayor y La menor. El primero,
el segundo, el cuarto y el quinto son iguales en las dos escalas, así
que sus números romanos son los mismos de ambas maneras.

### La sexta y la séptima tienen dos formas

En una tonalidad menor, las notas sexta y séptima se mueven. La menor
armónica eleva la séptima (Sol se vuelve Sol♯), de modo que el acorde sobre
el quinto grado es Mi mayor, con una sensible, en lugar de Mi menor. La
menor melódica, al subir, eleva también la sexta (Fa se vuelve Fa♯).

Así que en La menor hay dos Sol y dos Fa, y un número romano sin alteración
tiene que nombrar a uno de ellos. La regla que sigue la app es la que
usan los libros clásicos: **un número romano sin alteración nombra la
forma cuyo acorde pertenece a la tonalidad.**

- **VII** es Sol mayor (Sol Si Re), de la menor natural. **vii°** es Sol♯°
  (Sol♯ Si Re), el acorde de sensible de la menor armónica. Sol° (Sol Si♭ Re♭) no
  es un acorde de La menor, así que vii° no puede significar eso.
- **VI** es Fa mayor, de la menor natural. **vi°** es Fa♯° (Fa♯ La Do), de la
  menor melódica.

Los mismos acordes, contados desde la escala mayor, son bVII, vii°, bVI
y vi°, ya que Sol♯ y Fa♯ son las notas de La mayor. Así que el acorde de
sensible es vii° de las dos maneras, y Mi mayor es V de las dos. Aquí
están las tríadas de las tres escalas menores, escritas de las dos
maneras:

```text
                    desde la escala mayor          desde la escala menor
menor natural   i ii° bIII  iv v  bVI bVII     i ii° III  iv v  VI  VII
menor armónica  i ii° bIII+ iv V  bVI vii°     i ii° III+ iv V  VI  vii°
menor melódica  i ii  bIII+ IV V  vi° vii°     i ii  III+ IV V  vi° vii°
```

Un acorde fuera de la tonalidad recibe una alteración medida contra la
forma que nombra el número romano sin alteración. Si♭ mayor en La menor,
el napolitano, es **bII** de las dos maneras. Contado desde la escala
menor, Sol° es **bvii°** (Sol♯° rebajado) y Sol♯ mayor es **#VII** (elevado
por encima de las dos formas de Sol).

La elección no se aplica en absoluto a las tonalidades mayores ni a los
modos: Do mayor es I ii iii IV V vi vii° de ambas maneras, y Mi frigio
conserva su bII.

## Qué cambia el ajuste, y qué no

El ajuste cambia cómo se **escriben** y **leen** los números romanos: en
las harmony keys, debajo de los acordes del chord sheet, en el piano
roll, en las ayudas, en el entrenamiento auditivo, en los tooltips de
estilo y en un vídeo exportado. Los números romanos que escribes o pegas
en el chord sheet se leen de la misma manera, así que con «desde la
escala menor» elegido, escribir `VII` en La menor da Sol mayor.

No cambia ningún acorde. Lo que una canción guarda es el acorde en sí (Sol
mayor), nunca su número romano, así que una canción se lee igual elijas
lo que elijas, y alternar entre una opción y otra no cambia nada en
ella. Las progresiones de la biblioteca se leen siempre contadas desde
la escala mayor, porque así es como están escritas: el cuadro
Progression de un estilo muestra `i | bVII | bVI | V`, por ejemplo, y
toca los mismos acordes para todo el mundo.

![El Choclo, en La menor armónica, contado desde la escala menor: Sol7 y Do son VII7 y III en el chord sheet, y las harmony keys dicen III+, VI y vii°.](../images/step-9-minor-numerals-keys.png)

## En el código

### La elección es un enum

```rust
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MinorNumerals {
    /// From the major scale on the same tonic: i, ii°, bIII, iv, v, bVI,
    /// bVII. …
    #[default]
    FromMajor,
    /// From the minor scale itself: i, ii°, III, iv, v, VI, VII, as
    /// classical harmony books write them. …
    FromMinor,
}
```

`#[default]` en una variante dice a `#[derive(Default)]` qué variante es
`MinorNumerals::default()`. `Serialize` y `Deserialize` permiten que la
ventana de Settings lo guarde con `insert_persisted` de egui, igual que
se guardan las opciones de la Library.

### Qué nota nombra un número romano sin alteración

Escribir un número romano medía antes la fundamental del acorde contra
la escala mayor. Ahora la mide contra `unaltered_degree`, la nota que
nombra el número romano sin alteración:

```rust
pub(crate) fn unaltered_degree(
    self,
    letters: usize,
    quality: ChordQuality,
    numerals: MinorNumerals,
) -> u8 {
    use ScaleKind::{HarmonicMinor, MelodicMinor, NaturalMinor};
    if numerals == MinorNumerals::FromMajor || !self.is_minor_key() {
        return ScaleKind::Major.tonal_intervals()[letters];
    }
    let natural = NaturalMinor.tonal_intervals()[letters];
    if letters < 5 {
        return natural;
    }
    // Whether the chord on `semitones` is in one of these scales,
    // counted from C, since only the distance from the tonic counts.
    let of_key = |semitones: u8, kinds: &[ScaleKind]| {
        let chord = Chord::new(PitchClass::C.transpose(i32::from(semitones)), quality);
        kinds
            .iter()
            .any(|&kind| Scale::new(PitchClass::C, kind).is_diatonic(chord))
    };
    if !of_key(natural, &[NaturalMinor]) && of_key(natural + 1, &[HarmonicMinor, MelodicMinor])
    {
        natural + 1
    } else {
        natural
    }
}
```

`letters` es el grado, desde 0 para la tónica. Para los grados 0 a 4, la
nota de la menor natural es la respuesta. Para la sexta y la séptima (5
y 6), la función plantea la pregunta de la regla anterior: ¿está en la
menor natural el acorde sobre la nota natural? Si no, y el acorde un
semitono más agudo está en la menor armónica o melódica, la nota elevada
es la buscada. `of_key` construye el acorde desde Do, porque solo importa
su distancia a la tónica, no en qué tonalidad está.

Con esa nota, las alteraciones se cuentan como antes:

```rust
let unaltered =
    self.unaltered_degree(letters, chord.quality, MinorNumerals::current()) as i8;
let semitones = self.root.semitones_up_to(chord.root) as i8;
// How far the root is from the note the numeral names without an
// accidental, from -6 to 5.
let shift = (semitones - unaltered + 18).rem_euclid(12) - 6;
let accidentals = if shift < 0 { "b" } else { "#" }.repeat(shift.unsigned_abs() as usize);
```

La diferencia en semitonos de dos notas solo se conoce salvo octavas.
Sumar 6, tomar el resto módulo 12 y volver a quitar 6 da la vuelta más
corta, de −6 a 5: una fundamental un semitono por debajo da −1, una
«b», no +11. (`rem_euclid` daría un resto de 0 a 11 incluso para un
número negativo; el 12 extra de `+ 18` mantiene la suma positiva de
todos modos.)

Leer un número romano escrito (`numeral::parse_in`) usa el mismo
`unaltered_degree`, así que escribir y leer no pueden discrepar.

### Un ajuste que toda función puede ver, uno por hilo

Los números romanos se escriben en muchos lugares: las harmony keys, el
chord sheet, el piano roll, las ayudas, una exportación de vídeo. Todos
acaban en `Scale::roman_numeral`. Pasar el ajuste hacia abajo como
parámetro habría cambiado la firma de cada función del camino, docenas
de ellas, por una elección que cambia de vez en cuando. Así que el
ajuste se guarda donde `roman_numeral` puede leerlo directamente:

```rust
thread_local! {
    static MINOR_NUMERALS: Cell<MinorNumerals> = const { Cell::new(MinorNumerals::FromMajor) };
}

impl MinorNumerals {
    pub fn current() -> MinorNumerals {
        MINOR_NUMERALS.with(Cell::get)
    }

    pub fn set(self) {
        MINOR_NUMERALS.with(|numerals| numerals.set(self));
    }
}
```

El hilo de la ventana lo fija una vez por frame, desde los ajustes:

```rust
settings::minor_numerals(&ctx).set();
```

Dos piezas de Rust hacen que esto sea seguro:

- **`thread_local!`** da a cada hilo su propia copia del valor.
  `cargo test` ejecuta pruebas en muchos hilos a la vez; una prueba que
  escribe números romanos contados desde la escala menor no debe cambiar
  lo que ve una prueba que corre a su lado. Con un `static` normal, sí
  lo haría, y la otra prueba fallaría de vez en cuando, según la
  temporización.
- **`Cell`** permite cambiar el valor a través de una referencia
  compartida. La los valores thread-local solo se llega con `&`, así que
  un valor normal nunca podría fijarse. `Cell` funciona con tipos `Copy`
  moviendo valores enteros hacia dentro y hacia fuera (`get`, `set`),
  sin entregar nunca una referencia a lo que hay dentro, de modo que
  nada puede leerse mientras cambia.

Como el valor pertenece a un hilo, un hilo nuevo empieza con el valor
por defecto. Una exportación de vídeo dibuja sus vistas en un hilo
propio, así que se lleva el ajuste consigo:

```rust
let numerals = MinorNumerals::current();
let worker = thread::spawn(move || {
    // The views are made here, on the export's own thread,
    // and write numerals as the app does.
    numerals.set();
    // …
});
```

### Devolverlo, incluso tras un panic

Las pruebas necesitan escribir los números romanos de una manera durante
un rato, y luego volver:

```rust
pub fn with<T>(self, f: impl FnOnce() -> T) -> T {
    struct Restore(MinorNumerals);
    impl Drop for Restore {
        fn drop(&mut self) {
            self.0.set();
        }
    }
    let _restore = Restore(MinorNumerals::current());
    self.set();
    f()
}
```

`Restore` guarda el valor antiguo, y su `Drop` lo devuelve. Rust suelta
una variable local cuando sale de ámbito, y también mientras un **panic**
desenrolla la función. Un `assert!` que falla dentro de `f` es un panic,
así que incluso entonces vuelve el ajuste antiguo, y la siguiente prueba
en ese hilo empieza limpia. Este patrón, limpiar en un `Drop`, se llama
*guard*, y es como `MutexGuard` de Rust desbloquea un mutex.

La variable se llama `_restore`, no `_`. Con un `_` a secas, el guard se
soltaría al momento, en la misma línea, antes de `self.set()`; nada
devolvería el valor antiguo después de `f`.

## Las pruebas

`minor_keys_can_be_numbered_from_their_own_scale` escribe las tríadas y
las séptimas de la menor natural, armónica y melódica contadas desde la
escala menor (la tabla de arriba), comprueba que Do mayor y Mi frigio no
cambian, y que la manera antigua vuelve después de `with`.

`numerals_from_the_minor_scale_read_back_as_the_same_chords` comprueba
que cada número romano que escribe la app se lee de vuelta como el mismo
acorde: las dos maneras de contar, cuatro clases de tonalidad, doce
tónicas, un acorde sobre cada una de las doce notas y nueve cualidades.
Son 10.368 números romanos, escritos y leídos.

`minor_key_numerals_follow_the_harmony_setting` muestra las harmony keys
de La menor (bIII, bVI, bVII), hace clic en **Harmony** y en la segunda
opción de la ventana de Settings, y comprueba que las teclas dicen ahora
III, VI y VII, y que la elección se guardó.

## Ejercicios

1. En La menor armónica, ¿qué escribe cada manera para Do aumentado
   (Do Mi Sol♯)? ¿Y para Mi menor? Compruébalo en la app con las harmony
   keys.
2. Dórico también es un modo menor, pero la app lo cuenta desde la
   escala mayor elijas lo que elijas (`is_minor_key` es false para él).
   Re dórico es Re Mi Fa Sol La Si Do. ¿Cuáles de sus números romanos cambiarían
   si se contara desde su propia escala, y por qué podría confundir más
   que ayudar?
3. Quita el `_` de `_restore` (nómbrala `restore`) y ejecuta
   `cargo test -p lineup-core`. Luego cámbiala a un `_` a secas y
   ejecuta las pruebas otra vez. ¿Qué pasa, y por qué?
