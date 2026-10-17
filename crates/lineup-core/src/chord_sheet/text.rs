//! Chord sheets as plain text, to paste and share.
//!
//! Musicians send each other chord charts as text, in a message or a
//! file, with the bars between "|" signs:
//!
//! ```text
//! Title: Autumn Song
//! Key: C major
//! Time: 4/4
//! Tempo: 120
//! Style: Pop beat
//!
//! [Verse]
//! | C      | Am7 | Dm7 G7 | C  |
//! | F      | %   | G7     | C  |
//!
//! [Chorus ×2]
//! | F      | G   | Em7 A7 | Dm |
//! ```
//!
//! Each bar holds what would be typed in a bar of the chord sheet (see
//! [`read_bar`]): harmonies spread over the beats, "/" for a beat that
//! goes on with the harmony before, and "%" (or nothing) for a bar that
//! goes on with the bar before. A harmony keeps its marks: "C." is a
//! shot, "C.." a hold, "C..." a rest and "^C" a push. A line can hold any number of bars;
//! "||", "|:" and ":|" read as plain bar lines.
//!
//! The settings at the top are all optional. A line in square brackets
//! starts a section, with "×2" (or "x2") if it is played twice. "Key:"
//! or "Time:" after the first bar changes the key or the time signature
//! from the next bar on. Lines starting with "#" are notes for people,
//! and are skipped.

use std::fmt;
use std::ops::Range;

use super::{ChordMarker, read_bar, write_bar};
use crate::band::band_styles;
use crate::harmony::{Scale, ScaleKind};
use crate::key_map::KeyMarker;
use crate::meter::{TimeMarker, TimeSignature};
use crate::pitch::NoteName;
use crate::playlist::{MAX_REPEATS, MAX_SONG_STEPS, Section};
use crate::project::Project;
use crate::tempo::Tempo;

/// Bars on a line of text, as in the chord sheet window.
const BARS_PER_LINE: usize = 4;

/// The file name extensions of chord sheets saved as text.
pub const EXTENSIONS: [&str; 1] = ["txt"];

/// Whether `path` names a chord sheet saved as text.
pub fn is_text_file(path: &std::path::Path) -> bool {
    path.extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| {
            EXTENSIONS
                .iter()
                .any(|known| known.eq_ignore_ascii_case(extension))
        })
}

/// A chord sheet read from text, before it is put in a song.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct TextSheet {
    pub title: Option<String>,
    /// The key the song starts in.
    pub key: Option<Scale>,
    /// The time signature the song starts in.
    pub time: Option<TimeSignature>,
    /// In quarter notes per minute.
    pub tempo: Option<f32>,
    /// The name of the band style to play it in, as written, and its
    /// line.
    pub style: Option<(String, usize)>,
    /// The bars, sections and changes, in order.
    pub items: Vec<Item>,
}

/// One thing in the body of a text chord sheet. `line` is where it is
/// written, counted from 1, to say where a problem is.
#[derive(Debug, Clone, PartialEq)]
pub enum Item {
    /// A bar, as it is typed in the chord sheet: "Dm7 G7".
    Bar { text: String, line: usize },
    /// A section starts with the next bar.
    Section {
        name: String,
        repeats: usize,
        line: usize,
    },
    /// The key changes from the next bar on.
    Key(Scale),
    /// The time signature changes from the next bar on.
    Time(TimeSignature),
}

/// Why a text could not be read, and on which line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextError {
    /// Counted from 1.
    pub line: usize,
    pub message: String,
}

impl fmt::Display for TextError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "line {}: {}", self.line, self.message)
    }
}

fn error(line: usize, message: impl Into<String>) -> TextError {
    TextError {
        line,
        message: message.into(),
    }
}

impl TextSheet {
    /// How many bars it has.
    pub fn bar_count(&self) -> usize {
        self.items
            .iter()
            .filter(|item| matches!(item, Item::Bar { .. }))
            .count()
    }
}

/// Reads a chord sheet written as text. Only the layout is checked
/// here; what is in each bar is read when the sheet is put in a song,
/// which knows how long its bars are.
pub fn read(text: &str) -> Result<TextSheet, TextError> {
    let mut sheet = TextSheet::default();
    for (index, line) in text.lines().enumerate() {
        let number = index + 1;
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let started = sheet.bar_count() > 0;
        if let Some((name, value)) = setting(line) {
            let value = value.trim();
            match name.as_str() {
                "title" => sheet.title = Some(value.to_string()),
                "key" => {
                    let key = read_key(value).map_err(|message| error(number, message))?;
                    if started {
                        sheet.items.push(Item::Key(key));
                    } else {
                        sheet.key = Some(key);
                    }
                }
                "time" => {
                    let time: TimeSignature = value.parse().map_err(|message| {
                        error(
                            number,
                            format!("\"{value}\" is not a time signature: {message}."),
                        )
                    })?;
                    if started {
                        sheet.items.push(Item::Time(time));
                    } else {
                        sheet.time = Some(time);
                    }
                }
                "tempo" => {
                    let tempo = read_tempo(value).map_err(|message| error(number, message))?;
                    sheet.tempo = Some(tempo);
                }
                _ => sheet.style = Some((value.to_string(), number)),
            }
        } else if line.starts_with('[') {
            let (name, repeats) = read_heading(line).map_err(|message| error(number, message))?;
            sheet.items.push(Item::Section {
                name,
                repeats,
                line: number,
            });
        } else if line.contains('|') {
            for text in read_bars(line) {
                sheet.items.push(Item::Bar { text, line: number });
            }
        } else if !started && sheet.title.is_none() && sheet.items.is_empty() {
            // A first line that is nothing else is the title, as at the
            // top of a printed chart.
            sheet.title = Some(line.to_string());
        } else {
            return Err(error(
                number,
                format!(
                    "\"{line}\" is not a line of bars. Write each bar between | signs, as in \
                     | C | Am7 | Dm7 G7 |."
                ),
            ));
        }
    }
    Ok(sheet)
}

/// "Key: C major" as ("key", " C major"), for the settings this module
/// knows, whatever their case.
fn setting(line: &str) -> Option<(String, &str)> {
    let (name, value) = line.split_once(':')?;
    let name = name.trim().to_lowercase();
    ["title", "key", "time", "tempo", "style"]
        .contains(&name.as_str())
        .then_some((name, value))
}

/// A key of tonal music: "C major", "A minor", "F# Dorian", "Bb", "Am".
fn read_key(text: &str) -> Result<Scale, String> {
    let (first, rest) = text.split_once(char::is_whitespace).unwrap_or((text, ""));
    let rest = rest.trim().to_lowercase();
    // "e minor" as well as "E minor".
    let mut letters = first.chars();
    let first: String = letters
        .next()
        .map(|letter| letter.to_ascii_uppercase())
        .into_iter()
        .chain(letters)
        .collect();
    let first = first.as_str();
    let (root, kind) = match NoteName::parse(first) {
        Some(root) => (root, rest.as_str()),
        // "Am", "F#m": a minor key.
        None => match first.strip_suffix('m').and_then(NoteName::parse) {
            Some(root) if rest.is_empty() => (root, "minor"),
            _ => {
                return Err(format!(
                    "\"{text}\" is not a key: write it as C major or A minor."
                ));
            }
        },
    };
    let kind = match kind {
        "" | "maj" => ScaleKind::Major,
        "min" | "natural minor" => ScaleKind::NaturalMinor,
        name => ScaleKind::ALL
            .into_iter()
            .find(|kind| kind.name().eq_ignore_ascii_case(name))
            .ok_or_else(|| {
                format!(
                    "\"{text}\" is not a key the text can say. Keys are major, minor, harmonic \
                     or melodic minor, the modes (Dorian…), pentatonic or blues; a row or a set \
                     is chosen in the app."
                )
            })?,
    };
    Ok(Scale {
        root: root.pitch_class(),
        kind,
    })
}

/// "120", "120 BPM" or "96.5".
fn read_tempo(text: &str) -> Result<f32, String> {
    let number = text
        .trim_end_matches(|c: char| c.is_ascii_alphabetic() || c.is_whitespace())
        .trim();
    match number.parse::<f32>() {
        Ok(bpm) if (Tempo::MIN..=Tempo::MAX).contains(&bpm) => Ok(bpm),
        Ok(_) => Err(format!(
            "a tempo goes from {} to {} BPM.",
            Tempo::MIN,
            Tempo::MAX
        )),
        Err(_) => Err(format!("\"{text}\" is not a tempo: write it as 120.")),
    }
}

/// "[Verse]", "[Chorus ×2]", "[Chorus] x3".
fn read_heading(line: &str) -> Result<(String, usize), String> {
    let (inside, after) = line[1..]
        .split_once(']')
        .ok_or("a section's name goes between [ and ], as in [Verse].")?;
    let mut name = inside.trim().to_string();
    let mut repeats = read_times(after.trim());
    if repeats.is_none()
        && let Some((first, last)) = name.rsplit_once(char::is_whitespace)
        && let Some(times) = read_times(last)
    {
        repeats = Some(times);
        name = first.trim().to_string();
    }
    if name.is_empty() {
        return Err("a section needs a name, as in [Verse].".to_string());
    }
    let repeats = match repeats {
        Some(Some(times)) => times,
        Some(None) => {
            return Err(format!("a section is played 1 to {MAX_REPEATS} times."));
        }
        None if after.trim().is_empty() => 1,
        None => {
            return Err(format!(
                "\"{}\" after a section's name is not read: write [Chorus ×2] to play it twice.",
                after.trim()
            ));
        }
    };
    Ok((name, repeats))
}

/// "×2", "x2" or "X 2": `Some(Some(2))`; a count out of range is
/// `Some(None)`, and anything else `None`.
fn read_times(text: &str) -> Option<Option<usize>> {
    let number = text
        .strip_prefix(['×', 'x', 'X'])?
        .trim()
        .parse::<usize>()
        .ok()?;
    Some((1..=MAX_REPEATS).contains(&number).then_some(number))
}

/// The bars of a line: what is between the "|" signs. Text before the
/// first sign or after the last is a bar too, so "C | G" is two bars.
fn read_bars(line: &str) -> Vec<String> {
    let mut plain = line.replace("|:", "|").replace(":|", "|");
    while plain.contains("||") {
        plain = plain.replace("||", "|");
    }
    let pieces: Vec<&str> = plain.split('|').map(str::trim).collect();
    let first = usize::from(pieces.first().is_some_and(|piece| piece.is_empty()));
    let last =
        pieces.len() - usize::from(pieces.len() > first && pieces[pieces.len() - 1].is_empty());
    pieces[first..last]
        .iter()
        .map(|piece| piece.to_string())
        .collect()
}

/// The chords a text writes into `project`, from bar `first` on: the
/// steps it covers and the markers in them, to write as one change of
/// the chord sheet. Only the bars are taken; sections, the tempo and
/// the time signature stay as the song has them. A "Key:" in the text
/// says how its Roman numerals are read; without one, they are read in
/// the song's key.
pub fn paste(
    sheet: &TextSheet,
    project: &Project,
    first: usize,
) -> Result<(Range<usize>, Vec<ChordMarker>), TextError> {
    let meter = project.meter();
    let keys = project.key_map();
    let mut text_key = sheet.key;
    let mut bars = meter.bars().skip(first);
    let mut markers = Vec::new();
    let mut range: Option<Range<usize>> = None;
    for item in &sheet.items {
        match item {
            Item::Key(key) => text_key = Some(*key),
            Item::Bar { text, line } => {
                let bar = bars
                    .next()
                    .filter(|bar| bar.end() <= MAX_SONG_STEPS)
                    .ok_or_else(|| error(*line, "the song cannot be this long."))?;
                let key = text_key.unwrap_or_else(|| keys.key_at(bar.start));
                let written = read_bar(text, &bar, key).map_err(|message| {
                    error(*line, format!("bar {}: {message}", bar.number + 1))
                })?;
                markers.extend(written);
                let start = range.as_ref().map_or(bar.start, |range| range.start);
                range = Some(start..bar.end());
            }
            Item::Section { .. } | Item::Time(_) => {}
        }
    }
    let first_step = meter.bar(first).start;
    Ok((range.unwrap_or(first_step..first_step), markers))
}

/// A song read from a text chord sheet, and what could not be taken as
/// written.
pub struct Imported {
    pub project: Project,
    pub notes: Vec<String>,
}

/// A new song with a band that plays the chord sheet in `sheet`: its
/// key, time signature, tempo and sections, and its chords bar by bar.
///
/// The band plays in the style the text names; without one, in a style
/// in the text's time signature (the first, by genre), or the style a
/// new song starts with.
pub fn import(sheet: &TextSheet) -> Result<Imported, TextError> {
    let mut notes = Vec::new();
    let mut project = Project::with_band();
    let named = sheet.style.as_ref().and_then(|(name, line)| {
        let style = band_styles()
            .iter()
            .find(|style| style.name.eq_ignore_ascii_case(name.trim()));
        if style.is_none() {
            notes.push(format!("Line {line}: no band style is called \"{name}\"."));
        }
        style
    });
    let current = project.band().map(|band| band.style);
    let in_time = sheet
        .time
        .filter(|time| current.is_none_or(|style| style.time != *time))
        .and_then(|time| band_styles().iter().find(|style| style.time == time));
    if let Some(style) = named.or(in_time) {
        project.set_band_style(Some(style));
    }
    if let Some(time) = sheet.time
        && project.time_signature != time
    {
        project.set_time_marker(TimeMarker::new(0, time));
        if let Some(band) = project.band() {
            notes.push(format!(
                "The band plays {}, a style in {}, in {time}.",
                band.style.name, band.style.time
            ));
        }
    }
    if let Some(key) = sheet.key {
        project.set_key_marker(KeyMarker::new(0, key));
    }
    if let Some(bpm) = sheet.tempo {
        project.tempo = Tempo::new(bpm);
    }

    // Where each bar starts, with the changes of time and key on the way.
    let mut time = project.time_signature;
    let mut step = 0;
    let mut bars: Vec<(usize, &str, usize)> = Vec::new();
    let mut sections: Vec<(String, usize, usize)> = Vec::new();
    for item in &sheet.items {
        match item {
            Item::Bar { text, line } => {
                if step + time.steps_per_bar() > MAX_SONG_STEPS {
                    return Err(error(*line, "the song cannot be this long."));
                }
                bars.push((step, text, *line));
                step += time.steps_per_bar();
            }
            Item::Time(new) => {
                time = *new;
                project.set_time_marker(TimeMarker::new(step, time));
            }
            Item::Key(key) => {
                project.set_key_marker(KeyMarker::new(step, *key));
            }
            Item::Section { name, repeats, .. } => sections.push((name.clone(), *repeats, step)),
        }
    }
    let end = step;
    for (index, (name, repeats, start)) in sections.iter().enumerate() {
        let until = sections.get(index + 1).map_or(end, |next| next.2);
        if until > *start {
            let mut section = Section::new(name.clone(), *start, until - start);
            section.repeats = *repeats;
            project.add_section(section);
        }
    }

    let meter = project.meter();
    let keys = project.key_map();
    let mut markers = Vec::new();
    for (start, text, line) in bars {
        let bar = meter.bar_at(start);
        let written = read_bar(text, &bar, keys.key_at(start))
            .map_err(|message| error(line, format!("bar {}: {message}", bar.number + 1)))?;
        markers.extend(written);
    }
    project.set_chords(0, end, &markers);
    project.follow_chord_sheet();
    Ok(Imported { project, notes })
}

/// The whole chord sheet of `project` as text: the settings, then the
/// bars, four to a line, with a new line at each section and each change
/// of key or time signature.
pub fn write(project: &Project, title: Option<&str>) -> String {
    let meter = project.meter();
    let keys = project.key_map();
    let mut text = String::new();
    if let Some(title) = title {
        text.push_str(&format!("Title: {title}\n"));
    }
    text.push_str(&key_line(keys.key_at(0)));
    text.push('\n');
    text.push_str(&format!("Time: {}\n", meter.first()));
    text.push_str(&format!("Tempo: {}\n", tempo_text(project.tempo.bpm())));
    if let Some(band) = project.band() {
        text.push_str(&format!("Style: {}\n", band.style.name));
    }
    let count = sheet_bars(project);
    if count > 0 {
        text.push('\n');
        text.push_str(&layout(&lines(project, 0..count, true)));
    }
    text
}

/// The bars `bars` (by number) of `project`, four to a line, without
/// settings or sections: for copying some bars to paste them elsewhere.
pub fn write_bars(project: &Project, bars: Range<usize>) -> String {
    layout(&lines(project, bars, false))
}

/// How many bars the chord sheet has: as many as the song plays, or the
/// chords written reach, or the band plays.
pub fn sheet_bars(project: &Project) -> usize {
    let markers = project.playlist().chord_markers();
    let end = project
        .song_length()
        .max(markers.last().map_or(0, |marker| marker.start + 1))
        .max(project.band().map_or(0, |band| band.end()))
        .min(MAX_SONG_STEPS);
    match end {
        0 => 0,
        end => project.meter().bar_at(end - 1).number + 1,
    }
}

/// "Key: Bb major". A key the text cannot say, a row or a set, is
/// written as a note for people.
fn key_line(key: Scale) -> String {
    if key.kind.is_tonal() {
        format!("Key: {}", key.name())
    } else {
        format!("# Key: {} (chosen in the app)", key.name())
    }
}

/// "120", or "96.5" for a tempo between whole numbers.
fn tempo_text(bpm: f32) -> String {
    let rounded = (bpm * 10.0).round() / 10.0;
    if rounded.fract() == 0.0 {
        format!("{rounded:.0}")
    } else {
        format!("{rounded:.1}")
    }
}

/// A line of the text: words, or bars to line up with the others.
enum Line {
    Words(String),
    Bars(Vec<String>),
}

/// The lines for `bars`. With `marks`, sections and changes of key and
/// time signature are written too.
fn lines(project: &Project, bars: Range<usize>, marks: bool) -> Vec<Line> {
    let meter = project.meter();
    let keys = project.key_map();
    let markers = project.playlist().chord_markers();
    let sections = project.playlist().sections();
    let mut lines: Vec<Line> = Vec::new();
    let mut previous = None;
    for number in bars {
        let bar = meter.bar(number);
        if bar.start >= MAX_SONG_STEPS {
            break;
        }
        let key = keys.key_at(bar.start);
        let mut words = Vec::new();
        if marks && let Some((time, before)) = previous {
            if bar.time != time {
                words.push(format!("Time: {}", bar.time));
            }
            if key != before {
                words.push(key_line(key));
            }
        }
        if marks
            && let Some(section) = sections
                .iter()
                .find(|section| meter.bar_at(section.start).number == number)
        {
            if !lines.is_empty() {
                lines.push(Line::Words(String::new()));
            }
            words.push(match section.repeats {
                1 => format!("[{}]", section.name),
                times => format!("[{} ×{times}]", section.name),
            });
        }
        previous = Some((bar.time, key));
        let starts_line = !words.is_empty()
            || lines
                .last()
                .is_none_or(|line| !matches!(line, Line::Bars(bars) if bars.len() < BARS_PER_LINE));
        lines.extend(words.into_iter().map(Line::Words));
        if starts_line {
            lines.push(Line::Bars(Vec::new()));
        }
        let in_bar: Vec<ChordMarker> = markers
            .iter()
            .filter(|marker| (bar.start..bar.end()).contains(&marker.start))
            .copied()
            .collect();
        let written = write_bar(&in_bar, &bar);
        if let Some(Line::Bars(bars)) = lines.last_mut() {
            bars.push(if written.is_empty() {
                "%".to_string()
            } else {
                written
            });
        }
    }
    lines
}

/// The lines as text, the bars of every line in columns.
fn layout(lines: &[Line]) -> String {
    let mut widths = [0; BARS_PER_LINE];
    for line in lines {
        if let Line::Bars(bars) = line {
            for (width, bar) in widths.iter_mut().zip(bars) {
                *width = (*width).max(bar.chars().count());
            }
        }
    }
    let mut text = String::new();
    for line in lines {
        match line {
            Line::Words(words) => text.push_str(words),
            Line::Bars(bars) => {
                let cells: Vec<String> = bars
                    .iter()
                    .zip(widths)
                    .map(|(bar, width)| format!("{bar:<width$}"))
                    .collect();
                text.push_str(&format!("| {} |", cells.join(" | ")));
            }
        }
        text.push('\n');
    }
    text
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::library::demo::song_demo;
    use crate::library::song::SONGS;
    use crate::pitch::PitchClass;

    const CHART: &str = "Title: Autumn Song
Key: C major
Time: 4/4
Tempo: 120
Style: Pop beat

[Verse]
| C      | Am7 | Dm7 G7 | C  |
| F      | %   | G7     | C  |

[Chorus ×2]
| F      | G   | Em7 A7 | Dm |
";

    fn key(root: PitchClass, kind: ScaleKind) -> Scale {
        Scale { root, kind }
    }

    fn bars(sheet: &TextSheet) -> Vec<&str> {
        sheet
            .items
            .iter()
            .filter_map(|item| match item {
                Item::Bar { text, .. } => Some(text.as_str()),
                _ => None,
            })
            .collect()
    }

    /// The chords of `project` as (step, name).
    fn chords(project: &Project) -> Vec<(usize, String)> {
        project
            .playlist()
            .chord_markers()
            .iter()
            .map(|marker| (marker.start, marker.harmony.to_string()))
            .collect()
    }

    fn imported(text: &str) -> Project {
        let sheet = read(text).unwrap_or_else(|err| panic!("{err}"));
        import(&sheet).unwrap_or_else(|err| panic!("{err}")).project
    }

    #[test]
    fn a_chart_is_read_line_by_line() {
        let sheet = read(CHART).unwrap();
        assert_eq!(sheet.title.as_deref(), Some("Autumn Song"));
        assert_eq!(sheet.key, Some(key(PitchClass::C, ScaleKind::Major)));
        assert_eq!(sheet.time, Some(TimeSignature::COMMON));
        assert_eq!(sheet.tempo, Some(120.0));
        assert_eq!(
            sheet.style.as_ref().map(|(name, _)| name.as_str()),
            Some("Pop beat")
        );
        assert_eq!(
            bars(&sheet),
            [
                "C", "Am7", "Dm7 G7", "C", "F", "%", "G7", "C", "F", "G", "Em7 A7", "Dm"
            ]
        );
        assert_eq!(
            sheet.items[0],
            Item::Section {
                name: "Verse".to_string(),
                repeats: 1,
                line: 7
            }
        );
        assert!(
            matches!(&sheet.items[9], Item::Section { name, repeats: 2, .. } if name == "Chorus")
        );
    }

    #[test]
    fn bar_lines_are_read_however_they_are_drawn() {
        assert_eq!(read_bars("|: C | Am || F | G :|"), ["C", "Am", "F", "G"]);
        assert_eq!(read_bars("C | G"), ["C", "G"]);
        assert_eq!(read_bars("| C |   | G |"), ["C", "", "G"]);
        assert_eq!(
            read_bars("| Menor 3 T2 | {0,1,4} |"),
            ["Menor 3 T2", "{0,1,4}"]
        );
    }

    #[test]
    fn keys_tempos_and_sections_are_read() {
        let read = |text| read_key(text).unwrap();
        assert_eq!(read("Bb major"), key(PitchClass::ASharp, ScaleKind::Major));
        assert_eq!(read("Am"), key(PitchClass::A, ScaleKind::NaturalMinor));
        assert_eq!(
            read("F#m"),
            key(PitchClass::FSharp, ScaleKind::NaturalMinor)
        );
        assert_eq!(read("D"), key(PitchClass::D, ScaleKind::Major));
        assert_eq!(read("C minor"), key(PitchClass::C, ScaleKind::NaturalMinor));
        assert_eq!(
            read("e harmonic minor"),
            key(PitchClass::E, ScaleKind::HarmonicMinor)
        );
        assert_eq!(read("D dorian"), key(PitchClass::D, ScaleKind::Dorian));
        assert!(read_key("H major").is_err());
        assert!(read_key("C bebop").is_err());

        assert_eq!(read_tempo("120 BPM"), Ok(120.0));
        assert_eq!(read_tempo("96.5"), Ok(96.5));
        assert!(read_tempo("5").is_err());
        assert!(read_tempo("fast").is_err());

        assert_eq!(read_heading("[Chorus ×2]"), Ok(("Chorus".to_string(), 2)));
        assert_eq!(read_heading("[Chorus] x3"), Ok(("Chorus".to_string(), 3)));
        assert_eq!(read_heading("[Bridge 2]"), Ok(("Bridge 2".to_string(), 1)));
        assert!(read_heading("[]").is_err());
        assert!(read_heading("[Verse] twice").is_err());
        assert!(read_heading("[Verse ×100]").is_err());
    }

    #[test]
    fn a_mistake_says_its_line() {
        let err = read("Key: C major\n| C | Am |\nplay it twice").unwrap_err();
        assert_eq!(err.line, 3);
        assert_eq!(read("Time: 9/5").unwrap_err().line, 1);
        // A first line that is nothing else is the title.
        assert_eq!(
            read("Blue Moon\n| C |").unwrap().title.as_deref(),
            Some("Blue Moon")
        );
        // A bar that cannot be read says its line and its bar.
        let sheet = read("| C | Am |\n| F | Xyz |").unwrap();
        let err = import(&sheet).err().unwrap();
        assert_eq!(err.line, 2);
        assert!(err.message.starts_with("bar 4:"), "{}", err.message);
    }

    #[test]
    fn a_chart_becomes_a_song_with_a_band() {
        let project = imported(CHART);
        let band = project.band().expect("a band");
        assert_eq!(band.style.name, "Pop beat");
        assert_eq!(project.scale, key(PitchClass::C, ScaleKind::Major));
        assert_eq!(project.tempo.bpm(), 120.0);
        assert_eq!(project.time_signature, TimeSignature::COMMON);
        let sections: Vec<(&str, usize, usize, usize)> = project
            .playlist()
            .sections()
            .iter()
            .map(|section| {
                (
                    section.name.as_str(),
                    section.start,
                    section.length,
                    section.repeats,
                )
            })
            .collect();
        assert_eq!(sections, [("Verse", 0, 128, 1), ("Chorus", 128, 64, 2)]);
        let names: Vec<(usize, &str)> = vec![
            (0, "C"),
            (16, "Am7"),
            (32, "Dm7"),
            (40, "G7"),
            (48, "C"),
            (64, "F"),
            // Bar 6, "%", goes on with F.
            (96, "G7"),
            (112, "C"),
            (128, "F"),
            (144, "G"),
            (160, "Em7"),
            (168, "A7"),
            (176, "Dm"),
        ];
        let expected: Vec<(usize, String)> = names
            .into_iter()
            .map(|(step, name)| (step, name.to_string()))
            .collect();
        assert_eq!(chords(&project), expected);
        assert!(band.end() >= 192);
    }

    #[test]
    fn the_band_plays_in_the_charts_time_signature() {
        let project = imported("Time: 3/4\n| C | G7 | C |");
        assert_eq!(project.time_signature, TimeSignature::new(3, 4));
        assert_eq!(project.band().unwrap().style.time, TimeSignature::new(3, 4));
        assert_eq!(
            chords(&project),
            [
                (0, "C".to_string()),
                (12, "G7".to_string()),
                (24, "C".to_string())
            ]
        );
        // A style named in the text is the one the band plays.
        let project = imported("Style: bossa nova\n| Cmaj7 | Dm7 G7 |");
        assert_eq!(project.band().unwrap().style.name, "Bossa nova");
        // A style with no such name is said, and the band plays anyway.
        let sheet = read("Title: Mine\nStyle: Polka disco\n| C | G7 |").unwrap();
        let unknown = import(&sheet).unwrap();
        assert_eq!(
            unknown.notes,
            ["Line 2: no band style is called \"Polka disco\"."]
        );
        assert!(unknown.project.band().is_some());
    }

    #[test]
    fn keys_and_meters_change_where_the_text_says() {
        let project =
            imported("Key: C major\n| C | G7 |\nKey: G major\n| D7 | G |\nTime: 3/4\n| C | D7 |");
        let keys = project.key_map();
        assert_eq!(keys.key_at(31), key(PitchClass::C, ScaleKind::Major));
        assert_eq!(keys.key_at(32), key(PitchClass::G, ScaleKind::Major));
        let meter = project.meter();
        assert_eq!(meter.bar(4).start, 64);
        assert_eq!(meter.bar(4).time, TimeSignature::new(3, 4));
        assert_eq!(meter.bar(5).start, 76);
        assert_eq!(
            chords(&project)[4..],
            [(64, "C".to_string()), (76, "D7".to_string())]
        );
    }

    #[test]
    fn numerals_are_read_in_the_charts_key() {
        let project = imported("Key: F major\n| I | vi | ii7 V7 | I |");
        let names: Vec<String> = chords(&project).into_iter().map(|(_, name)| name).collect();
        assert_eq!(names, ["F", "Dm", "Gm7", "C7", "F"]);
    }

    #[test]
    fn pasted_bars_go_from_the_bar_chosen_on() {
        let mut project = imported("| C | C | C | C | C | C |");
        let sheet = read("[Verse]\n| Dm7 | G7 |\n| C | % |").unwrap();
        let (steps, markers) = paste(&sheet, &project, 2).unwrap();
        assert_eq!(steps, 32..96);
        assert!(project.set_chords(steps.start, steps.end, &markers));
        let names: Vec<(usize, String)> = chords(&project);
        assert_eq!(
            names,
            [
                (0, "C".to_string()),
                (16, "C".to_string()),
                (32, "Dm7".to_string()),
                (48, "G7".to_string()),
                (64, "C".to_string())
            ]
        );
        // The sections of the text are not pasted.
        assert!(project.playlist().sections().is_empty());
        let wrong = read("| C | Bm7b5b9b13 |").unwrap();
        assert_eq!(paste(&wrong, &project, 0).unwrap_err().line, 1);
    }

    #[test]
    fn the_sheet_is_written_in_columns() {
        let project = imported(CHART);
        let text = write(&project, Some("Autumn Song"));
        assert_eq!(
            text,
            CHART
                .replace(
                    "| C      | Am7 | Dm7 G7 | C  |",
                    "| C | Am7 | Dm7 G7 | C  |"
                )
                .replace(
                    "| F      | %   | G7     | C  |",
                    "| F | %   | G7     | C  |"
                )
                .replace(
                    "| F      | G   | Em7 A7 | Dm |",
                    "| F | G   | Em7 A7 | Dm |"
                )
        );
        assert_eq!(write_bars(&project, 1..3), "| Am7 | Dm7 G7 |\n");
    }

    #[test]
    fn shots_holds_rests_and_pushes_go_through_text() {
        let chart = "| C. | ^F | G7.. Am... | ^^C |\n";
        let project = imported(chart);
        let marks: Vec<String> = project
            .playlist()
            .chord_markers()
            .iter()
            .map(|marker| format!("{}@{}", marker.text(), marker.start))
            .collect();
        assert_eq!(marks, ["C.@0", "^F@16", "G7..@32", "Am...@40", "^^C@48"]);
        assert_eq!(write_bars(&project, 0..4), chart);
    }

    /// The chords of `project` with each moved to the nearest beat, as
    /// text writes them: a bar of text has nothing between its beats.
    /// Of two chords on one beat, the first is written.
    fn on_the_beat(project: &Project) -> Vec<(usize, String)> {
        let meter = project.meter();
        let mut moved: Vec<(usize, String)> = Vec::new();
        for (step, name) in chords(project) {
            let bar = meter.bar_at(step);
            let beats: Vec<usize> = bar.beat_starts().collect();
            let beat = beats[super::super::nearest_beat(&beats, step)];
            if moved.last().is_none_or(|(last, _)| *last != beat) {
                moved.push((beat, name));
            }
        }
        moved
    }

    #[test]
    fn every_song_demo_comes_back_from_its_text() {
        let mut off_the_beat = 0;
        for song in SONGS {
            let demo = song_demo(song);
            let text = write(&demo, Some(song.name));
            let project = imported(&text);
            if chords(&demo) != on_the_beat(&demo) {
                off_the_beat += 1;
            }
            assert_eq!(
                chords(&project),
                on_the_beat(&demo),
                "{}:\n{text}",
                song.name
            );
            assert_eq!(project.key_map(), demo.key_map(), "{}", song.name);
            let times = |project: &Project| {
                project
                    .meter()
                    .bars()
                    .take(sheet_bars(&demo))
                    .map(|bar| (bar.start, bar.time))
                    .collect::<Vec<_>>()
            };
            assert_eq!(times(&project), times(&demo), "{}", song.name);
        }
        // Only a few songs change chord between two beats.
        assert!(off_the_beat * 10 < SONGS.len(), "{off_the_beat} songs");
    }
}
