//! Localization and translations for Ferrous Lineup.
//!
//! Provides the [`Language`] setting and string translation lookup via [`tr`].
//! When the language is set to Spanish, English strings are looked up in
//! the translation catalog; any untranslated string cleanly falls back
//! to its English original.

use std::cell::Cell;

use eframe::egui;
use lineup_core::harmony::{Scale, ScaleKind};
use lineup_core::{NoteName, NoteNaming};
use serde::{Deserialize, Serialize};

/// The user interface language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum Language {
    #[default]
    English,
    Spanish,
}

thread_local! {
    /// The language this thread writes the interface in. The window's
    /// thread sets it from the settings each frame; each thread has its
    /// own, so tests running side by side never see each other's.
    static LANGUAGE: Cell<Language> = const { Cell::new(Language::English) };
}

impl Language {
    pub const ALL: [Language; 2] = [Language::English, Language::Spanish];

    /// The language's name, written in that language, so that anyone
    /// can find their own.
    pub fn name(self) -> &'static str {
        match self {
            Language::English => "English",
            Language::Spanish => "Español",
        }
    }

    /// The language this thread writes in.
    pub fn current() -> Language {
        LANGUAGE.with(Cell::get)
    }

    /// Writes in this language on this thread from now on.
    pub fn set(self) {
        LANGUAGE.with(|language| language.set(self));
    }

    /// Runs `f` in this language, then goes back to the one before, even
    /// if `f` panics.
    #[cfg(test)]
    pub fn with<T>(self, f: impl FnOnce() -> T) -> T {
        struct Restore(Language);
        impl Drop for Restore {
            fn drop(&mut self) {
                self.0.set();
            }
        }
        let _restore = Restore(Language::current());
        self.set();
        f()
    }
}

fn language_id() -> egui::Id {
    egui::Id::unique("app-language")
}

fn note_naming_id() -> egui::Id {
    egui::Id::unique("app-note-naming")
}

/// The language chosen in Settings > Appearance, kept with the window's
/// other preferences.
pub fn language(ctx: &egui::Context) -> Language {
    ctx.data_mut(|data| data.get_persisted(language_id()).unwrap_or_default())
}

/// Chooses the interface language, from this frame on.
pub fn set_language(ctx: &egui::Context, language: Language) {
    language.set();
    ctx.data_mut(|data| data.insert_persisted(language_id(), language));
}

/// The note naming chosen in Settings > Harmony.
pub fn note_naming(ctx: &egui::Context) -> NoteNaming {
    ctx.data_mut(|data| data.get_persisted(note_naming_id()).unwrap_or_default())
}

/// Chooses how notes and keys are named, from this frame on.
pub fn set_note_naming(ctx: &egui::Context, naming: NoteNaming) {
    naming.set();
    ctx.data_mut(|data| data.insert_persisted(note_naming_id(), naming));
}

/// Makes this thread write in the language and name notes the way the
/// settings say. The window calls it before drawing each frame.
pub fn apply_settings(ctx: &egui::Context) {
    language(ctx).set();
    note_naming(ctx).set();
}

/// The language this thread writes the interface in.
pub fn current_language() -> Language {
    Language::current()
}

/// The name of a key as the settings say: "C major", "Do major" with
/// solfège, and "Do mayor" in Spanish. A row is named by its form, "row
/// P4 (on E)", and a set by its set class, "set 4-28 on C", as
/// [`Scale::name`] names them in letters for files and text.
pub fn key_name(scale: Scale) -> String {
    let root = scale.spell(scale.root).to_string();
    match scale.kind {
        ScaleKind::Row(_) => format!(
            "{} P{} ({} {root})",
            tr("row"),
            scale.root.semitone(),
            tr("on")
        ),
        ScaleKind::Set(set) => {
            format!(
                "{} {} {} {root}",
                tr("set"),
                set.set_class().name(),
                tr("on")
            )
        }
        kind => format!("{root} {}", tr(kind.name())),
    }
}

/// `text` with each note written in braces, {A} or {G#}, named as the
/// settings say: "as here in {A} minor" reads "as here in La minor" in
/// solfège. Braces around anything else, as in the set {0,1,4}, stay.
pub fn with_notes(text: &str) -> String {
    let mut shown = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(open) = rest.find('{') {
        shown.push_str(&rest[..open]);
        let after = &rest[open + 1..];
        let note = after.find('}').and_then(|close| {
            let name = NoteName::read(&after[..close]).filter(|(_, left)| left.is_empty());
            name.map(|(name, _)| (name, close))
        });
        match note {
            Some((name, close)) => {
                shown.push_str(&name.to_string());
                rest = &after[close + 1..];
            }
            None => {
                shown.push('{');
                rest = after;
            }
        }
    }
    shown.push_str(rest);
    shown
}

/// Translates a static string into the current language.
/// If untranslated or if the language is English, returns the original text.
#[inline]
pub fn tr(text: &'static str) -> &'static str {
    if Language::current() == Language::Spanish {
        translate_es(text).unwrap_or(text)
    } else {
        text
    }
}

/// Translates a borrowed string slice into the current language.
#[inline]
pub fn tr_str(text: &str) -> &str {
    if Language::current() == Language::Spanish {
        translate_es(text).unwrap_or(text)
    } else {
        text
    }
}

/// Spanish translations dictionary for UI strings.
fn translate_es(text: &str) -> Option<&'static str> {
    Some(match text {
        // --- Top Menu Bar ---
        "File" => "Archivo",
        "Edit" => "Edición",
        "View" => "Ver",
        "MIDI" => "MIDI",
        "Help" => "Ayuda",

        // --- File Menu ---
        "New" => "Nuevo",
        "An empty chord sheet with a band to play it" => {
            "Una hoja de acordes vacía con una banda para tocarla"
        }
        "New beat" => "Nuevo ritmo",
        "An empty pattern and a drum kit" => "Un patrón vacío y una batería",
        "Open…" => "Abrir…",
        "Recent" => "Recientes",
        "Recent songs" => "Canciones recientes",
        "No recent songs" => "No hay canciones recientes",
        "Clear recent songs" => "Borrar canciones recientes",
        "Demos" => "Demostraciones",
        "Import recording…" => "Importar grabación…",
        "Open an MP3 or WAV file and add chords that follow it" => {
            "Abre un archivo MP3 o WAV y añade acordes que lo sigan"
        }
        "Import MIDI file…" => "Importar archivo MIDI…",
        "Open a .mid file: its notes, tempo, time signatures and key" => {
            "Abre un archivo .mid: sus notas, tempo, compases y tonalidad"
        }
        "Import chord sheet…" => "Importar hoja de acordes…",
        "Open a chord chart saved as text, as in | C | Am7 | Dm7 G7 |" => {
            "Abre un esquema de acordes guardado como texto, como | C | Am7 | Dm7 G7 |"
        }
        "Save" => "Guardar",
        "Save as…" => "Guardar como…",
        "Demos are read-only: this saves a copy to a new file" => {
            "Las demostraciones son de solo lectura: esto guarda una copia en un archivo nuevo"
        }
        "Export WAV…" => "Exportar WAV…",
        "Save the song as a sound file" => "Guarda la canción como un archivo de sonido",
        "Put some clips in the playlist first" => {
            "Coloca primero algunos clips en la lista de reproducción"
        }
        "Export MIDI file…" => "Exportar archivo MIDI…",
        "Save the song as a .mid file" => "Guarda la canción como un archivo .mid",
        "Export video…" => "Exportar video…",
        "Make an MP4 video of the song" => "Crea un video MP4 de la canción",
        "Export lead sheet PDF…" => "Exportar partitura guía en PDF…",
        "Save a printable PDF of the song's chords and melody" => {
            "Guarda un PDF imprimible de los acordes y la melodía de la canción"
        }
        "Write some chords on the chord sheet first" => {
            "Escribe primero algunos acordes en la hoja de acordes"
        }
        "Quit" => "Salir",

        // --- Edit Menu ---
        "Undo" => "Deshacer",
        "Redo" => "Rehacer",
        "Cut" => "Cortar",
        "Copy" => "Copiar",
        "Paste" => "Pegar",
        "Duplicate" => "Duplicar",
        "Delete" => "Eliminar",
        "Select all" => "Seleccionar todo",
        "Select nothing" => "Deseleccionar todo",
        "Settings" => "Ajustes",
        "Settings…" => "Ajustes…",
        "Preferences" => "Preferencias",

        // --- View Menu ---
        "Chord sheet" => "Hoja de acordes",
        "Chord sheet in the main window" => "Hoja de acordes en la ventana principal",
        "The chord sheet in the playlist's place, with more bars on a line" => {
            "La hoja de acordes en lugar de la lista de reproducción, con más compases por línea"
        }
        "Lead sheet" => "Partitura guía",
        "Channel rack" => "Rack de canales",
        "Style editor…" => "Editor de estilos…",
        "See and edit repeating parts, or create a style from an existing one" => {
            "Consulta y edita partes repetitivas o crea un estilo a partir de uno existente"
        }
        "Piano roll" => "Piano roll",
        "Harmony keys" => "Teclas de armonía",
        "Patterns and library" => "Patrones y biblioteca",
        "Library" => "Biblioteca",
        "Mixer" => "Mezclador",
        "Ear training" => "Entrenamiento auditivo",
        "experimental" => "experimental",
        "Reset zoom" => "Restablecer zoom",
        "Zoom in" => "Acercar",
        "Zoom out" => "Alejar",
        "Interface size" => "Tamaño de la interfaz",

        // --- MIDI Menu ---
        "Input devices" => "Dispositivos de entrada",
        "Output devices" => "Dispositivos de salida",
        "Rescan devices" => "Buscar dispositivos de nuevo",
        "No MIDI inputs found" => "No se encontraron entradas MIDI",
        "No MIDI outputs found" => "No se encontraron salidas MIDI",
        "Typing keyboard to piano" => "Teclado del equipo como piano",
        "Play notes with the letter keys: Z X C V B N M from C4, with S D G H J as the black keys, and Q to P from C5, with the numbers above them." => {
            "Toca notas con las teclas de letras: Z X C V B N M desde Do4, con S D G H J como teclas negras, y de Q a P desde Do5, con los números encima de ellas."
        }
        "Accompaniment follows chords" => "El acompañamiento sigue los acordes",
        "Input: a keyboard to play" => "Entrada: un teclado para tocar",
        "Output: an external synth to play" => "Salida: un sintetizador externo para tocar",
        "Send every channel" => "Enviar todos los canales",
        "Nothing is sent while audio is unavailable" => {
            "No se envía nada mientras el audio no esté disponible"
        }

        // --- Help Menu ---
        "Keyboard shortcuts" => "Atajos de teclado",
        "User manual" => "Manual de usuario",
        "Website" => "Sitio web",
        "Report an issue" => "Informar de un problema",
        "About Ferrous Lineup" => "Acerca de Ferrous Lineup",

        // --- About Window ---
        "Version" => "Versión",
        "Play songs with a backing band and learn harmony" => {
            "Toca canciones con una banda de acompañamiento y aprende armonía"
        }
        "Free software: you can share it and change it under the GNU General \
         Public License, version 3 or later. It comes with no warranty." => {
            "Software libre: puedes compartirlo y modificarlo según la Licencia \
             Pública General de GNU, versión 3 o posterior. Se ofrece sin ninguna \
             garantía."
        }
        "Read the license" => "Leer la licencia",
        "Source code" => "Código fuente",
        "The book" => "El libro",
        "The book (PDF)" => "El libro (PDF)",

        // --- Transport Bar & Status Bar Hints ---
        "Pattern mode: loop the selected pattern (L)" => {
            "Modo patrón: repite el patrón seleccionado (L)"
        }
        "Song mode: play the playlist (L)" => {
            "Modo canción: reproduce la lista de reproducción (L)"
        }
        "Go to the start (Home)" => "Ir al inicio (Inicio)",
        "Back one bar (Page Up)" => "Un compás hacia atrás (Re Pág)",
        "Forward one bar (Page Down)" => "Un compás hacia adelante (Av Pág)",
        "Play from the playhead (Space)" => "Reproducir desde el cursor (Espacio)",
        "Pause: stop here, and carry on from here next time" => {
            "Pausa: detenerse aquí y continuar desde aquí la próxima vez"
        }
        "Stop and go back to where you last put the playhead (Space)" => {
            "Detener y volver a la última posición del cursor (Espacio)"
        }
        "Record notes from a MIDI keyboard or the computer keyboard (R)" => {
            "Grabar notas desde un teclado MIDI o el teclado del equipo (R)"
        }
        "Master volume" => "Volumen principal",
        "Tempo in beats per minute: drag up or down to change, double-click to type" => {
            "Tempo en pulsos por minuto: arrastra para cambiar, doble clic para escribir"
        }
        "Time signature" => "Compás",
        "Metronome" => "Metrónomo",
        "Metronome (Ctrl+M)" => "Metrónomo (Ctrl+M)",
        "Count-in" => "Cuenta de entrada",
        "Count-in: one bar before recording starts" => {
            "Cuenta de entrada: un compás antes de empezar a grabar"
        }
        "Speed trainer" => "Entrenador de velocidad",
        "Speed trainer: play a loop faster each time" => {
            "Entrenador de velocidad: reproduce un bucle más rápido cada vez"
        }
        "Rename, clone or delete the selected pattern" => {
            "Renombrar, duplicar o eliminar el patrón seleccionado"
        }
        "Add a new, empty pattern" => "Añadir un nuevo patrón vacío",
        "Also removes its clips from the playlist" => {
            "También elimina sus clips de la lista de reproducción"
        }
        "Name" => "Nombre",
        "Patterns and Library (Alt+F8)" => "Patrones y Biblioteca (Alt+F8)",
        "Chord sheet (F5)" => "Hoja de acordes (F5)",
        "Channel rack (F6)" => "Rack de canales (F6)",
        "Piano roll (F7)" => "Piano roll (F7)",
        "Harmony keys (F8)" => "Teclas de armonía (F8)",
        "Mixer (F9)" => "Mezclador (F9)",
        "Chord sheet: the song's harmony bar by bar, to read and write (F5)" => {
            "Hoja de acordes: la armonía de la canción compás a compás, para leer y escribir (F5)"
        }
        "Channel rack: the steps of the selected pattern (F6)" => {
            "Rack de canales: los pasos del patrón seleccionado (F6)"
        }
        "Piano roll: melodies and chords, note by note (F7)" => {
            "Piano roll: melodías y acordes, nota por nota (F7)"
        }
        "Harmony keys: the key, the time signature and a key for each chord (F8)" => {
            "Teclas de armonía: la tonalidad, el compás y una tecla para cada acorde (F8)"
        }
        "Mixer: levels, meters, pan, solo, mute and reverb (F9)" => {
            "Mezclador: niveles, medidores, balance, solo, silencio y reverberación (F9)"
        }
        "Where the playhead is: in the pattern (PAT) or the song (SONG). Click the playlist's bar numbers to move it" => {
            "Dónde está el cursor de reproducción: en el patrón (PAT) o en la canción (SONG). Haz clic en los números de compás para moverlo"
        }
        "Tempo in beats a minute. Drag up or down, or double-click to type" => {
            "Tempo en pulsos por minuto. Arrastra hacia arriba o abajo, o doble clic para escribir"
        }
        "Space plays and stops · L switches pattern and song · F5 chord sheet · F6 channel rack · F7 piano roll · F8 harmony keys" => {
            "Espacio reproduce y detiene · L cambia patrón y canción · F5 hoja de acordes · F6 rack de canales · F7 piano roll · F8 teclas de armonía"
        }
        "Count-in: one bar of clicks before playback starts" => {
            "Cuenta de entrada: un compás de clics antes de comenzar la reproducción"
        }
        "Metronome: a click on every beat during playback" => {
            "Metrónomo: un clic en cada pulso durante la reproducción"
        }
        "Playlist" => "Lista de reproducción",
        "Export song as MIDI…" => "Exportar canción como MIDI…",
        "Save the notes of the song, to open in another studio" => {
            "Guarda las notas de la canción para abrir en otro estudio"
        }
        "Export pattern as MIDI…" => "Exportar patrón como MIDI…",
        "Save the notes of the selected pattern" => "Guarda las notas del patrón seleccionado",
        "Export chord sheet…" => "Exportar hoja de acordes…",
        "Save the chord sheet as text, to share or paste anywhere" => {
            "Guarda la hoja de acordes como texto, para compartir o pegar en cualquier lugar"
        }
        "Save the song with the chord sheet or other views playing along" => {
            "Guarda la canción con la hoja de acordes u otras vistas acompañando"
        }
        "Songs" => "Canciones",
        "Styles" => "Estilos",
        "Twelve-tone" => "Dodecafónico",
        "Tunes with their melodies, free for everyone to use" => {
            "Melodías libres para el uso de todos"
        }
        "Each harmony style as a short song" => "Cada estilo armónico como una canción corta",
        "Each row style as a short piece: the row, backwards, upside down, both" => {
            "Cada estilo de serie como una pieza corta: la serie, retrógrada, invertida y ambas"
        }
        "Each micromode style as a short piece: from the micromode, upside down, a tritone away, upside down there, and back" => {
            "Cada estilo de micromodo como una pieza corta: desde el micromodo, invertido, a un tritono, invertido allí y de regreso"
        }
        "Editor" => "Editor",
        "Horizontal" => "Horizontal",
        "Vertical" => "Vertical",
        "Linked" => "Vinculado",
        "The channel rack uses one scale for both directions." => {
            "El rack de canales usa una misma escala para ambas direcciones."
        }
        "Reset all zoom levels to 100%" => "Restablecer todos los niveles de zoom al 100%",
        "Zoom shortcuts" => "Atajos de zoom",
        "Shortcuts act on the view under the pointer." => {
            "Los atajos actúan sobre la vista bajo el cursor."
        }
        "Horizontal zoom" => "Zoom horizontal",
        "Vertical zoom" => "Zoom vertical",
        "Reset the current view" => "Restablecer la vista actual",
        "Over toolbars and panels, zoom changes the interface size. Use ⌘ in place of Ctrl on Mac." => {
            "Sobre barras de herramientas y paneles, el zoom cambia el tamaño de la interfaz. Usa ⌘ en lugar de Ctrl en Mac."
        }
        "Taller or shorter tracks (Alt+wheel)" => "Pistas más altas o más bajas (Alt+rueda)",
        "Wider or narrower bars (Ctrl+wheel)" => "Compases más anchos o más estrechos (Ctrl+rueda)",
        "Open recent" => "Abrir recientes",
        "Don't save" => "No guardar",
        "Stop" => "Detener",
        "Master" => "Principal",
        "Reset mix" => "Restablecer mezcla",
        "Twelve-tone row" => "Serie dodecafónica",
        "An export is still running" => "Una exportación aún se está ejecutando",
        "Stop it and quit" => "Detener y salir",
        "Keep exporting" => "Continuar exportando",
        "Export video" => "Exportar video",
        "The song's sound, with the views you pick playing along." => {
            "El sonido de la canción, acompañado de las vistas que elijas."
        }
        "The lead sheet fills the width of its box: make the box wider to draw it bigger" => {
            "La partitura guía llena el ancho de su cuadro: amplía el cuadro para agrandarla"
        }
        "Stack" => "Apilar",
        "Put the views one above the other again, sharing the picture" => {
            "Coloca las vistas una sobre otra nuevamente, compartiendo la imagen"
        }
        "Preview at" => "Vista previa en",
        "Drag a view to move it, drag its corner to resize it, and scroll over it to zoom." => {
            "Arrastra una vista para moverla, arrastra su esquina para redimensionarla y gira la rueda sobre ella para hacer zoom."
        }
        "Size" => "Tamaño",
        "An MP4 file: H.264 pictures, 30 a second, and MP3 sound." => {
            "Un archivo MP4: imágenes H.264, 30 por segundo, y sonido MP3."
        }
        "Export…" => "Exportar…",
        "Tick a view to show" => "Marca una vista para mostrar",
        "Show" => "Mostrar",
        "Fit the names" => "Ajustar a los nombres",
        "Fit" => "Ajustar",
        "⛶ Fill the window" => "⛶ Llenar la ventana",
        "Back to the playlist" => "Volver a la lista de reproducción",

        // --- Settings Window ---
        "Appearance" => "Apariencia",
        "Harmony" => "Armonía",
        "Audio" => "Audio",
        "Playback" => "Reproducción",
        "Language" => "Idioma",
        "Choose the interface language." => "Elige el idioma de la interfaz.",
        "Note naming" => "Nomenclatura de notas",
        "How note names are written throughout the app." => {
            "Cómo se escriben los nombres de las notas en la aplicación."
        }
        "Letters (C, D, E…)" => "Letras (C, D, E…)",
        "Solfège (Do, Re, Mi…)" => "Solfeo (Do, Re, Mi…)",
        "Roman numerals in minor keys" => "Números romanos en tonalidades menores",
        "Both ways are correct. They name the same chords and differ only in the chords on the third, sixth and seventh degrees of a minor key, as here in {A} minor." => {
            "Ambas formas son correctas. Nombran los mismos acordes y solo difieren en los acordes sobre los grados tercero, sexto y séptimo de una tonalidad menor, como aquí en {A} menor."
        }
        "bIII bVI bVII: counted from the major scale (pop and jazz)" => {
            "bIII bVI bVII: contados desde la escala mayor (pop y jazz)"
        }
        "{C} is bIII, {F} is bVI and {G} is bVII; the leading-tone chord {G#}° is vii°." => {
            "{C} es bIII, {F} es bVI y {G} es bVII; el acorde de sensible {G#}° es vii°."
        }
        "III VI VII: counted from the minor scale (classical harmony)" => {
            "III VI VII: contados desde la escala menor (armonía clásica)"
        }
        "{C} is III, {F} is VI and {G} is VII; {G#}° is still vii°, and {E} major is V." => {
            "{C} es III, {F} es VI y {G} es VII; {G#}° sigue siendo vii°, y {E} mayor es V."
        }
        "Applies to every view that shows numerals, and to numerals you type or paste on the chord sheet. A style's Progression box is always written bIII bVI bVII, as the library writes it." => {
            "Se aplica a todas las vistas que muestran números romanos, y a los que escribes o pegas en la hoja de acordes. El cuadro Progresión de un estilo siempre se escribe bIII bVI bVII, como lo escribe la biblioteca."
        }
        "Output" => "Salida",
        "Uses your system’s default audio device." => {
            "Usa el dispositivo de audio predeterminado del sistema."
        }
        "Restart audio" => "Reiniciar audio",
        "Try again" => "Reintentar",
        "Open the system’s default output device again" => {
            "Abre de nuevo el dispositivo de salida predeterminado del sistema"
        }
        "Preferences are saved automatically." => "Las preferencias se guardan automáticamente.",
        "Done" => "Listo",
        "Close" => "Cerrar",
        "Cancel" => "Cancelar",
        "Discard" => "Descartar",
        "Save changes?" => "¿Guardar cambios?",
        "There are unsaved changes. Do you want to save them before leaving?" => {
            "Hay cambios sin guardar. ¿Deseas guardarlos antes de salir?"
        }

        // --- Chord Sheet ---
        "Band" => "Banda",
        "Progression" => "Progresión",
        "Style" => "Estilo",
        "Key" => "Tonalidad",
        "major" => "mayor",
        "minor" => "menor",
        "harmonic minor" => "menor armónica",
        "melodic minor" => "menor melódica",
        "Dorian" => "dórico",
        "Phrygian" => "frigio",
        "Lydian" => "lidio",
        "Mixolydian" => "mixolidio",
        "Locrian" => "locrio",
        "major pentatonic" => "pentatónica mayor",
        "minor pentatonic" => "pentatónica menor",
        "blues" => "blues",
        "chromatic" => "cromática",
        "twelve-tone row" => "serie dodecafónica",
        "pitch-class set" => "conjunto de clases de alturas",
        "row" => "serie",
        "set" => "conjunto",
        "on" => "sobre",
        "Write the chords the song plays" => "Escribir los acordes que toca la canción",
        "Fill the sheet with the chords heard in the song, in place of what it says now: an arpeggio counts as its chord, two chords at once are written Eb7|Fb, notes that make no chord as their set; in a set or a row, the sets or micromodes heard. Undo brings the sheet back" => {
            "Llena la hoja con los acordes escuchados en la canción en lugar de lo escrito ahora. Deshacer recupera la hoja"
        }
        "Click a bar and type its harmony: chords ({A}m7 {D}7, {G}/{B}), Roman numerals (ii7 V7), sets (3-11 T2, {0,1,4}), micromodes (Menor 3), row forms (P0, RI7) or N.C." => {
            "Haz clic en un compás y escribe su armonía: acordes ({A}m7 {D}7, {G}/{B}), números romanos (ii7 V7), conjuntos (3-11 T2, {0,1,4}), micromodos (Menor 3), formas de serie (P0, RI7) o N.C."
        }
        "Each / holds the harmony before it one more beat, as in {C} / / {G}7. The band stops with a shot ({C}.), a hold ({C}..) or a rest ({C}...), and ^{C} comes in early. Ctrl+C and Ctrl+V copy and paste bars as text." => {
            "Cada / mantiene la armonía anterior un pulso más, como en {C} / / {G}7. La banda se detiene con un golpe ({C}.), sostenido ({C}..) o silencio ({C}...), y ^{C} entra antes. Ctrl+C y Ctrl+V copian y pegan compases como texto."
        }
        "Speed" => "Velocidad",
        "Tap" => "Pulsar",
        "You play" => "Tocas tú",
        "🔁 Loop" => "🔁 Bucle",
        "Loop" => "Bucle",
        "bar" => "compás",
        "bars" => "compases",
        "from" => "desde",
        "to" => "hasta",
        "by" => "por",
        "every" => "cada",
        "pass" => "pasada",
        "passes" => "pasadas",
        "Sections" => "Secciones",
        "Practice" => "Práctica",
        "Notation" => "Notación",
        "As written" => "Como está escrito",
        "Chords" => "Acordes",
        "Pitch-class sets" => "Conjuntos de clases de alturas",
        "Micromodes" => "Micromodos",
        "Fill the window" => "Llenar la ventana",
        "Leave full window" => "Salir de pantalla completa",
        "Bars per line" => "Compases por línea",
        "Auto" => "Automático",

        // --- Common Actions & Dialogs ---
        "Rename" => "Renombrar",
        "Clone" => "Duplicar",
        "Remove" => "Quitar",
        "Add track" => "Añadir pista",
        "Add channel" => "Añadir canal",
        "Pattern" => "Patrón",
        "Song" => "Canción",
        "Mute" => "Silenciar",
        "Solo" => "Solo",
        "Volume" => "Volumen",
        "Pan" => "Balance",

        // --- Voice Leading & Theory ---
        "Voice leading" => "Conducción de voces",
        "Motion" => "Movimiento",
        "Parallel" => "Paralelo",
        "Contrary" => "Contrario",
        "Oblique" => "Oblicuo",
        "Similar" => "Similar",
        "Parallel fifths" => "Quintas paralelas",
        "Parallel octaves" => "Octavas paralelas",

        // --- Keyboard shortcuts sections ---
        "Anywhere" => "En cualquier lugar",
        "Except while typing in a text field." => "Excepto al escribir en un campo de texto.",
        "Play or stop (in the rack or the piano roll: the pattern)" => {
            "Reproducir o detener (en el rack o piano roll: el patrón)"
        }
        "Go to the start" => "Ir al inicio",
        "Go to the end" => "Ir al final",
        "Back / forward one bar" => "Atrás / adelante un compás",
        "Pattern or song mode" => "Modo patrón o canción",
        "This list" => "Esta lista",

        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn english_by_default() {
        assert_eq!(Language::current(), Language::English);
        assert_eq!(tr("File"), "File");
        assert_eq!(tr("Edit"), "Edit");
    }

    #[test]
    fn spanish_translation() {
        Language::Spanish.with(|| {
            assert_eq!(Language::current(), Language::Spanish);
            assert_eq!(tr("File"), "Archivo");
            assert_eq!(tr("Edit"), "Edición");
            assert_eq!(tr("View"), "Ver");
            assert_eq!(tr("Help"), "Ayuda");
            assert_eq!(tr("Save"), "Guardar");
            assert_eq!(tr("Settings"), "Ajustes");
            assert_eq!(tr("Chord sheet"), "Hoja de acordes");
            assert_eq!(tr("Channel rack"), "Rack de canales");
            assert_eq!(tr("Piano roll"), "Piano roll");
            assert_eq!(tr("Mixer"), "Mezclador");
            assert_eq!(tr("Master"), "Principal");
            assert_eq!(tr("Untranslated String"), "Untranslated String");
        });
        assert_eq!(tr("File"), "File", "with() puts English back");
    }

    #[test]
    fn settings_are_kept_and_applied() {
        let ctx = egui::Context::default();
        assert_eq!(language(&ctx), Language::English);
        assert_eq!(note_naming(&ctx), NoteNaming::Letters);
        set_language(&ctx, Language::Spanish);
        set_note_naming(&ctx, NoteNaming::Solfege);
        Language::English.set();
        NoteNaming::Letters.set();
        apply_settings(&ctx);
        assert_eq!(Language::current(), Language::Spanish);
        assert_eq!(NoteNaming::current(), NoteNaming::Solfege);
        // This thread only: put it back for the next test it runs.
        Language::English.set();
        NoteNaming::Letters.set();
    }

    #[test]
    fn solfege_is_a_way_of_showing_names() {
        use lineup_core::PitchClass;
        use lineup_core::chord_sheet::{ChordMarker, HarmonyName};

        let marker = |text: &str| ChordMarker::new(0, HarmonyName::parse(text).unwrap());
        let b_flat = Scale::new(PitchClass::ASharp, ScaleKind::Major);
        let d_dorian = Scale::new(PitchClass::D, ScaleKind::Dorian);
        assert_eq!(key_name(b_flat), "Bb major");
        assert_eq!(marker("Am7").text(), "Am7");
        NoteNaming::Solfege.with(|| {
            assert_eq!(marker("Am7").text(), "Lam7");
            assert_eq!(marker("G/B").text(), "Sol/Si");
            // Typed either way, it is the same chord.
            assert_eq!(marker("Lam7"), marker("Am7"));
            assert_eq!(key_name(b_flat), "Sib major");
            Language::Spanish.with(|| {
                assert_eq!(key_name(b_flat), "Sib mayor");
                assert_eq!(key_name(d_dorian), "Re dórico");
            });
        });
        Language::Spanish.with(|| assert_eq!(key_name(b_flat), "Bb mayor"));
        // Notes in braces in a text show as the settings name them.
        let text = "as in {C} / / {G#}7, or a set {0,1,4}";
        assert_eq!(with_notes(text), "as in C / / G#7, or a set {0,1,4}");
        NoteNaming::Solfege.with(|| {
            assert_eq!(with_notes(text), "as in Do / / Sol#7, or a set {0,1,4}");
        });
    }
}
