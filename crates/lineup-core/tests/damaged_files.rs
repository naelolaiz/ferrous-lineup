//! Files come from anywhere: cut short by a full disk, edited by hand,
//! or not what their name says. Reading one must end in an error or a
//! project, never in a crash, and a project that was read must save and
//! read back.
//!
//! These tests damage good files in many small ways and read each
//! result, as a fuzzer does. The damage comes from a fixed seed, so a
//! failure is the same on every run, and the message shows the file.

use std::panic::{self, AssertUnwindSafe};

use lineup_core::Project;
use lineup_core::chord_sheet::text;
use lineup_core::file;
use lineup_core::library::demo::song_demo;
use lineup_core::library::song::SONGS;
use lineup_core::midi_file::{import, read};

/// How many damaged copies each test reads.
const ROUNDS: usize = 600;

/// A small, fast source of numbers that are the same on every run
/// (xorshift).
struct Damage(u64);

impl Damage {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }

    /// A number from 0 to `n - 1`.
    fn below(&mut self, n: usize) -> usize {
        (self.next() % n.max(1) as u64) as usize
    }

    /// A place in `text` between two characters.
    fn place(&mut self, text: &str, from: usize, most: usize) -> usize {
        let mut at = (from + self.below(most + 1)).min(text.len());
        while !text.is_char_boundary(at) {
            at -= 1;
        }
        at
    }

    /// `text` with a few numbers made extreme, and pieces cut out or
    /// repeated.
    fn text(&mut self, text: &str) -> String {
        const NUMBERS: [&str; 12] = [
            "0",
            "-1",
            "255",
            "256",
            "65535",
            "4294967295",
            "18446744073709551615",
            "1e308",
            "NaN",
            "-0",
            "999999",
            "1000",
        ];
        let mut text = text.to_string();
        for _ in 0..=self.below(3) {
            match self.below(4) {
                0 | 1 => {
                    let digits: Vec<usize> = text
                        .char_indices()
                        .filter(|(_, c)| c.is_ascii_digit())
                        .map(|(at, _)| at)
                        .collect();
                    if digits.is_empty() {
                        continue;
                    }
                    let start = digits[self.below(digits.len())];
                    let end = text[start..]
                        .find(|c: char| !c.is_ascii_digit() && c != '.')
                        .map_or(text.len(), |end| start + end);
                    text.replace_range(start..end, NUMBERS[self.below(NUMBERS.len())]);
                }
                2 => {
                    let start = self.place(&text, 0, text.len());
                    let end = self.place(&text, start, 40);
                    text.replace_range(start..end, "");
                }
                _ => {
                    let start = self.place(&text, 0, text.len());
                    let end = self.place(&text, start, 200);
                    let piece = text[start..end].to_string();
                    text.insert_str(end, &piece);
                }
            }
        }
        text
    }

    /// `bytes` with a few bytes changed, cut short or repeated.
    fn bytes(&mut self, bytes: &[u8]) -> Vec<u8> {
        let mut bytes = bytes.to_vec();
        for _ in 0..=self.below(7) {
            if bytes.is_empty() {
                break;
            }
            let at = self.below(bytes.len());
            match self.below(4) {
                0 => bytes[at] = self.next() as u8,
                1 => bytes[at] = [0x00, 0x7F, 0x80, 0xFF][self.below(4)],
                2 => bytes.truncate(at),
                _ => {
                    let length = self.below(16).min(bytes.len() - at);
                    let piece = bytes[at..at + length].to_vec();
                    bytes.splice(at..at, piece);
                }
            }
        }
        bytes
    }
}

/// Runs `read` on `input`, and fails with `input` in the message if it
/// panics.
fn must_not_panic<T: std::fmt::Debug>(input: T, read: impl FnOnce(&T)) {
    if panic::catch_unwind(AssertUnwindSafe(|| read(&input))).is_err() {
        let shown = format!("{input:?}");
        let shown: String = shown.chars().take(4000).collect();
        panic!("reading this damaged file panicked:\n{shown}");
    }
}

/// What the app does with a project once it is read: plays it, and can
/// save it and open it again.
fn use_project(project: &Project) {
    std::hint::black_box(project.arrangement());
    std::hint::black_box(project.song_length());
    let text = file::to_text(project).expect("a project that was read saves");
    file::from_text(&text).expect("a project that was saved reads back");
}

#[test]
fn damaged_project_files_never_crash() {
    let mut seeds = vec![Project::demo(), Project::template()];
    seeds.extend(SONGS.iter().step_by(60).map(song_demo));
    let seeds: Vec<String> = seeds
        .iter()
        .map(|project| file::to_text(project).unwrap())
        .collect();
    let mut damage = Damage(0x9E37_79B9_7F4A_7C15);
    for round in 0..ROUNDS {
        let text = damage.text(&seeds[round % seeds.len()]);
        must_not_panic(text, |text| {
            if let Ok(project) = file::from_text(text) {
                use_project(&project);
            }
        });
    }
}

/// A small MIDI file with what importing looks at: tempo, meter and key,
/// a program change, notes by running status, the sustain pedal, a pitch
/// bend, and drums.
fn midi_file() -> Vec<u8> {
    let conductor: &[u8] = &[
        0x00, 0xFF, 0x51, 0x03, 0x07, 0xA1, 0x20, // 120 BPM
        0x00, 0xFF, 0x58, 0x04, 0x03, 0x02, 0x18, 0x08, // 3/4
        0x00, 0xFF, 0x59, 0x02, 0xFE, 0x01, // B♭ minor
        0x00, 0xFF, 0x2F, 0x00,
    ];
    let band: &[u8] = &[
        0x00, 0xC0, 0x00, // piano
        0x00, 0x90, 58, 90, // B♭3
        0x00, 61, 80, // D♭4, by running status
        0x00, 0xB0, 64, 127, // pedal down
        0x60, 0x90, 58, 0, //
        0x00, 61, 0, //
        0x00, 0xE0, 0x00, 0x50, // a bend
        0x00, 0x99, 36, 100, // kick
        0x30, 0x89, 36, 0, //
        0x00, 0xB0, 64, 0, // pedal up
        0x00, 0xFF, 0x2F, 0x00,
    ];
    let mut bytes = b"MThd".to_vec();
    bytes.extend(6u32.to_be_bytes());
    bytes.extend(1u16.to_be_bytes());
    bytes.extend(2u16.to_be_bytes());
    bytes.extend(96u16.to_be_bytes());
    for track in [conductor, band] {
        bytes.extend(b"MTrk");
        bytes.extend((track.len() as u32).to_be_bytes());
        bytes.extend(track);
    }
    bytes
}

#[test]
fn damaged_midi_files_never_crash() {
    let good = midi_file();
    let imported = import::import(&read::read(&good).unwrap(), "Test").unwrap();
    use_project(&imported.project);
    let mut damage = Damage(0xD1B5_4A32_D192_ED03);
    for _ in 0..ROUNDS {
        must_not_panic(damage.bytes(&good), |bytes| {
            if let Ok(midi) = read::read(bytes)
                && let Ok(imported) = import::import(&midi, "Test")
            {
                use_project(&imported.project);
            }
        });
    }
}

#[test]
fn damaged_chord_sheets_never_crash() {
    let good = "Title: Autumn Song
Key: C major
Time: 4/4
Tempo: 120
Style: Pop beat

[Verse]
| C      | Am7 | Dm7 G7 | C  |
| F      | %   | G7     | C  |

[Chorus ×2]
| F      | G   | Em7 A7 | Dm |
| Eb7|Fb | C/E | G7sus4 | C6/9 |
";
    let mut damage = Damage(0x94D0_49BB_1331_11EB);
    for _ in 0..ROUNDS {
        must_not_panic(damage.text(good), |sheet| {
            if let Ok(sheet) = text::read(sheet)
                && let Ok(imported) = text::import(&sheet)
            {
                use_project(&imported.project);
            }
        });
    }
}
