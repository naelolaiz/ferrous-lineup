//! What each chord of the chord sheet does in its key: the chords that
//! come from outside the key, and the chords where the key changes.
//!
//! A Roman numeral says where a chord stands in the key: in C major, Dm
//! is ii and G7 is V7. Most chords of a song are the key's own, but the
//! interesting ones often are not, and harmony has names for why they
//! are there:
//!
//! - A **borrowed chord** comes from the *parallel* key, the one on the
//!   same tonic in the other mode: Ab in C major is bVI, borrowed from C
//!   minor. Borrowing (also called *modal mixture*) colours the key
//!   without leaving it.
//! - A **secondary dominant** is the dominant of one of the key's other
//!   chords: D7 in C major is the V7 of G, written "V7/V". It makes G
//!   sound like a tonic for a moment (a *tonicization*). A **secondary
//!   leading-tone chord** does the same from a semitone below: F#°7 is
//!   "vii°7/V".
//! - The **Neapolitan** chord is a major chord on the lowered second
//!   degree, Db in C major, most often in first inversion ("N⁶"), on its
//!   way to the dominant.
//! - Where the key changes (a key marker), a **pivot chord** is a chord
//!   of both keys that the music passes through: in a move from C major
//!   to G major, Am is vi in C and ii in G. The ear hears it in the old
//!   key, and the chord after it, one the old key does not have, shows
//!   that it belonged to the new one.
//!
//! [`Functions::read`] reads the whole sheet at once, since whether a
//! secondary dominant goes where it leads, and which chord is the pivot,
//! depend on the chords around it.

use crate::harmony::{
    Chord, ChordFamily, ChordQuality, ChordSymbol, Scale, ScaleKind, with_figures,
};
use crate::key_map::KeyMap;
use crate::pitch::{NoteName, PitchClass};

use super::{ChordMarker, HarmonyName};

/// What a chord is to its key, beyond its numeral.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    /// One of the key's own chords, or a chord outside it that none of
    /// the others names.
    Plain,
    /// Taken from the parallel key, `from`.
    Borrowed { from: Scale },
    /// The dominant of `of`, one of the key's chords.
    SecondaryDominant { of: Chord },
    /// The chord on the leading tone of `of`, one of the key's chords.
    SecondaryLeadingTone { of: Chord },
    /// A major chord on the lowered second degree.
    Neapolitan,
}

/// A chord of the chord sheet, read in its key.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Function {
    /// The step it starts on.
    pub start: usize,
    pub symbol: ChordSymbol,
    /// The key it is read in: the key on its step.
    pub key: Scale,
    pub role: Role,
    /// The next chord, if one follows straight on: the next chord
    /// written that is not the same chord again.
    pub next: Option<ChordSymbol>,
    /// Whether it is the last chord of the sheet.
    pub last: bool,
    /// Whether the new key of the key change after it has already come:
    /// it stands between the first chord of that key alone and the key
    /// marker, and is read in the new key.
    pub early: bool,
    /// The key change it is the pivot chord of, if it is one.
    pub pivot: Option<Modulation>,
}

/// A change of key on the chord sheet, and how the music gets there.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Modulation {
    /// The step of the key marker.
    pub start: usize,
    pub from: Scale,
    pub to: Scale,
    /// The pivot chord: a chord of both keys, just before the first chord
    /// of the new key alone. Its step and symbol.
    pub pivot: Option<(usize, ChordSymbol)>,
    /// The first chord near the change that belongs to the new key and
    /// not to the old one. Its step and symbol.
    pub arrival: Option<(usize, ChordSymbol)>,
}

/// Every chord of a chord sheet read in its key, and every change of
/// key.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Functions {
    /// The chords written, in order. Other harmonies (sets, rows, N.C.)
    /// have none.
    pub chords: Vec<Function>,
    pub modulations: Vec<Modulation>,
}

impl Functions {
    /// Reads `markers`, the chord sheet in order, in the keys of `keys`.
    pub fn read(markers: &[ChordMarker], keys: &KeyMap) -> Functions {
        // The chords, with the harmony after each that is not the same
        // chord: a chord written again in the next bar has not moved on.
        let mut chords: Vec<Function> = Vec::new();
        let last_chord = markers
            .iter()
            .rposition(|marker| matches!(marker.harmony, HarmonyName::Chord(_)));
        for (index, marker) in markers.iter().enumerate() {
            let HarmonyName::Chord(symbol) = marker.harmony else {
                continue;
            };
            let next = markers[index + 1..]
                .iter()
                .find(|later| match later.harmony {
                    HarmonyName::Chord(later) => later.chord() != symbol.chord(),
                    _ => true,
                })
                .and_then(|later| match later.harmony {
                    HarmonyName::Chord(later) => Some(later),
                    _ => None,
                });
            let key = keys.key_at(marker.start);
            let last = last_chord == Some(index);
            chords.push(Function {
                start: marker.start,
                symbol,
                key,
                role: role(key, symbol, next),
                next,
                last,
                early: false,
                pivot: None,
            });
        }
        let modulations: Vec<Modulation> = changes(keys)
            .into_iter()
            .map(|(start, from, to, (since, until))| {
                modulation(&chords, start, from, to, since, until)
            })
            .collect();
        for change in &modulations {
            if let Some((step, _)) = change.pivot
                && let Some(chord) = chords.iter_mut().find(|chord| chord.start == step)
            {
                chord.pivot = Some(*change);
            }
            // Where the new key came before its marker, the chords from
            // its first chord on are the new key's.
            if let Some((arrival, _)) = change.arrival {
                for chord in &mut chords {
                    if (arrival..change.start).contains(&chord.start) {
                        chord.key = change.to;
                        chord.role = role(change.to, chord.symbol, chord.next);
                        chord.early = true;
                    }
                }
            }
        }
        Functions {
            chords,
            modulations,
        }
    }

    /// The chord written on `step`, if there is one.
    pub fn at(&self, step: usize) -> Option<&Function> {
        let index = self.chords.partition_point(|chord| chord.start < step);
        self.chords.get(index).filter(|chord| chord.start == step)
    }

    /// The key change on `step`, if there is one.
    pub fn modulation_at(&self, step: usize) -> Option<&Modulation> {
        self.modulations.iter().find(|change| change.start == step)
    }
}

/// The key changes of `keys` that change the key, with the stretch each
/// one's chords are looked for in: from the change before it to the one
/// after it.
fn changes(keys: &KeyMap) -> Vec<(usize, Scale, Scale, (usize, usize))> {
    let keys = keys.tidy();
    let markers = keys.markers();
    let mut from = keys.first();
    let mut found = Vec::new();
    for (index, marker) in markers.iter().enumerate() {
        let since = index
            .checked_sub(1)
            .map_or(0, |before| markers[before].start);
        let until = markers
            .get(index + 1)
            .map_or(usize::MAX, |after| after.start);
        found.push((marker.start, from, marker.scale, (since, until)));
        from = marker.scale;
    }
    found
}

/// Whether `key` is a key with chords of its own: seven notes, read by
/// Roman numerals.
fn has_chords(key: Scale) -> bool {
    key.is_heptatonic() && key.kind.is_tonal()
}

/// What `symbol` is in `key`, going on to `next`.
fn role(key: Scale, symbol: ChordSymbol, next: Option<ChordSymbol>) -> Role {
    use ScaleKind::{HarmonicMinor, MelodicMinor, NaturalMinor};
    let chord = symbol.chord();
    // Borrowed and secondary chords are what major and minor keys call
    // them. In a mode a chord from outside it has its numeral only: E
    // major in E Phrygian is the tonic of flamenco, not the dominant of
    // A minor.
    let major_or_minor = key.kind == ScaleKind::Major || key.is_minor_key();
    if !has_chords(key)
        || !major_or_minor
        || key.is_chord_of_key(chord)
        || in_minor(key, chord, &[NaturalMinor, HarmonicMinor])
    {
        return Role::Plain;
    }
    let goes_to = |of: Chord| next.is_some_and(|next| arrives(next, of));
    let dominant = key.secondary_dominant_target(chord);
    let leading = key.secondary_leading_tone_target(chord);
    let secondary = match (dominant, leading) {
        (Some(of), _) => Some(Role::SecondaryDominant { of }),
        // A leading-tone chord is named after where it goes: F#ø7 going
        // on to B7 in C major is the ii of E minor, not the vii of G.
        (_, Some(of)) if goes_to(of) => Some(Role::SecondaryLeadingTone { of }),
        _ => None,
    };
    // A chord that is both, such as A major in A minor (the major tonic,
    // or the V of Dm), is a secondary dominant where it goes to the
    // chord it leads to, and a borrowed chord anywhere else.
    if let Some(of) = dominant.or(leading)
        && goes_to(of)
    {
        return secondary.unwrap_or(Role::Plain);
    }
    // So is a chord of melodic minor, such as D major in A minor: the V
    // of G going to G, and the key's own IV anywhere else.
    if in_minor(key, chord, &[MelodicMinor]) {
        return Role::Plain;
    }
    if let Some(from) = borrowed_from(key, symbol) {
        return Role::Borrowed { from };
    }
    if is_neapolitan(key, symbol) {
        return Role::Neapolitan;
    }
    secondary.unwrap_or(Role::Plain)
}

/// Whether `chord` is a chord of one of the minor scales `kinds` on the
/// tonic of `key`, a minor key. Music in A minor plays C and G from the
/// natural minor as readily as E7 and G#dim7 from the harmonic minor,
/// whichever of the two the key is set to, and Bm and D from the melodic
/// minor.
fn in_minor(key: Scale, chord: Chord, kinds: &[ScaleKind]) -> bool {
    minor_scales(key)
        .iter()
        .any(|scale| kinds.contains(&scale.kind) && scale.is_chord_of_key(chord))
}

/// Whether `pitch_class` is a note of `key`: in a minor key, a note of
/// any of its three minor scales, so in A minor only C#, D# and A# are
/// not.
fn in_key(key: Scale, pitch_class: PitchClass) -> bool {
    key.contains(pitch_class)
        || minor_scales(key)
            .iter()
            .any(|scale| scale.contains(pitch_class))
}

/// The key `symbol` is borrowed from, if it is borrowed: a chord of the
/// parallel key, every note of it written as that key or `key` writes
/// it, on the letters up from their tonic. Gaug in C major has the notes
/// of Eb augmented, the III+ of C minor, but its D# rises to E: it is no
/// Eb from C minor.
fn borrowed_from(key: Scale, symbol: ChordSymbol) -> Option<Scale> {
    let from = key.borrowed_from(symbol.chord())?;
    // A major key borrows from harmonic minor too: Bdim7 in C major.
    let mut scales = vec![key, from, Scale::new(from.root, ScaleKind::HarmonicMinor)];
    scales.extend(minor_scales(key));
    let tonic = key.tonic_name().letter;
    let written = |name: &NoteName| {
        scales.iter().any(|scale| {
            scale.degree_of(name.pitch_class()).is_some_and(|degree| {
                NoteName::with_letter(name.pitch_class(), tonic.up(degree as i32)) == *name
            })
        })
    };
    symbol.notes().iter().all(written).then_some(from)
}

/// The three minor scales on the tonic of `key`, if it is a minor key.
fn minor_scales(key: Scale) -> Vec<Scale> {
    use ScaleKind::{HarmonicMinor, MelodicMinor, NaturalMinor};
    if !key.is_minor_key() {
        return Vec::new();
    }
    [NaturalMinor, HarmonicMinor, MelodicMinor]
        .into_iter()
        .map(|kind| Scale::new(key.root, kind))
        .collect()
}

/// Whether `next` comes to `of`, the chord a secondary chord leads to:
/// on its root, or on a six-four chord over its root, which holds it
/// back for a moment (C/G before G: the cadential six-four).
fn arrives(next: ChordSymbol, of: Chord) -> bool {
    next.root.pitch_class() == of.root || is_six_four_over(next, of.root)
}

/// Whether `symbol` is a major or minor triad with its fifth in the
/// bass, over `bass`: C/G over G.
fn is_six_four_over(symbol: ChordSymbol, bass: PitchClass) -> bool {
    symbol.bass.is_some()
        && symbol.bass_pitch_class() == bass
        && symbol.root.pitch_class().transpose(7) == bass
        && matches!(symbol.quality, ChordQuality::Major | ChordQuality::Minor)
}

/// Whether `symbol` is the Neapolitan chord of `key`: a major triad on
/// the lowered second degree, written on the second letter (Db in C
/// major, not C#).
fn is_neapolitan(key: Scale, symbol: ChordSymbol) -> bool {
    let major_or_minor = matches!(
        key.kind,
        ScaleKind::Major
            | ScaleKind::NaturalMinor
            | ScaleKind::HarmonicMinor
            | ScaleKind::MelodicMinor
    );
    major_or_minor
        && symbol.quality == ChordQuality::Major
        && key.root.semitones_up_to(symbol.root.pitch_class()) == 1
        && key.tonic_name().letter.steps_up_to(symbol.root.letter) == 1
}

/// Whether `chord` belongs to `key`: one of its chords, or in a minor
/// key one of the chords of its three minor scales.
fn belongs(key: Scale, chord: Chord) -> bool {
    use ScaleKind::{HarmonicMinor, MelodicMinor, NaturalMinor};
    key.is_chord_of_key(chord) || in_minor(key, chord, &[NaturalMinor, HarmonicMinor, MelodicMinor])
}

/// Whether `chord` can be heard in `key` before the music leaves it: one
/// of its chords, or one it borrows from its parallel key.
fn heard_in(key: Scale, chord: Chord) -> bool {
    belongs(key, chord) || key.borrowed_from(chord).is_some()
}

/// The key change on step `start` from `from` to `to`, its chords looked
/// for from step `since` up to step `until`.
fn modulation(
    chords: &[Function],
    start: usize,
    from: Scale,
    to: Scale,
    since: usize,
    until: usize,
) -> Modulation {
    let mut change = Modulation {
        start,
        from,
        to,
        pivot: None,
        arrival: None,
    };
    if !has_chords(from) || !has_chords(to) || from.root == to.root {
        return change;
    }
    let range: Vec<&Function> = chords
        .iter()
        .filter(|chord| (since..until).contains(&chord.start))
        .collect();
    let new_only = |chord: &Function| {
        let chord = chord.symbol.chord();
        belongs(to, chord) && !heard_in(from, chord)
    };
    let common = |chord: &Function| {
        let chord = chord.symbol.chord();
        belongs(to, chord) && heard_in(from, chord)
    };
    // The new key may have come before its marker: the chords just
    // before it that belong to the new key alone.
    let first_after = range.partition_point(|chord| chord.start < start);
    let mut arrival = None;
    let mut index = first_after;
    while index > 0 && new_only(range[index - 1]) {
        index -= 1;
        arrival = Some(index);
    }
    // Or after it, past the chords of both keys.
    if arrival.is_none() {
        arrival = range[first_after..]
            .iter()
            .position(|chord| !common(chord))
            .map(|offset| first_after + offset)
            .filter(|&index| new_only(range[index]));
    }
    let Some(arrival) = arrival else {
        return change;
    };
    change.arrival = Some((range[arrival].start, range[arrival].symbol));
    change.pivot = arrival
        .checked_sub(1)
        .map(|before| range[before])
        .filter(|before| common(before))
        .map(|before| (before.start, before.symbol));
    change
}

impl Function {
    /// The Roman numeral the chord sheet writes under it: the numeral of
    /// a secondary chord over the chord it leads to ("V⁶₅/V"), "N⁶" for
    /// the Neapolitan, and for a pivot chord its numerals in both keys
    /// ("vi = ii"). `None` in a key without Roman numerals.
    pub fn numeral(&self) -> Option<String> {
        if let Some(change) = self.pivot {
            let old = figured(change.from, self.symbol)?;
            let new = figured(change.to, self.symbol)?;
            return Some(format!("{old} = {new}"));
        }
        self.own_numeral()
    }

    /// The numeral of the chord in its own key, as [`Function::numeral`]
    /// writes it for a chord that is no pivot.
    fn own_numeral(&self) -> Option<String> {
        let chord = self.symbol.chord();
        let figures = chord.inversion_figures(self.symbol.bass_pitch_class());
        match self.role {
            Role::SecondaryDominant { of } => Some(with_figures(
                &format!(
                    "V{}/{}",
                    chord.quality.numeral_suffix(),
                    self.target_numeral(of)?
                ),
                figures,
            )),
            Role::SecondaryLeadingTone { of } => Some(with_figures(
                &format!(
                    "vii{}/{}",
                    chord.quality.numeral_suffix(),
                    self.target_numeral(of)?
                ),
                figures,
            )),
            Role::Neapolitan => Some(with_figures("N", figures)),
            Role::Plain | Role::Borrowed { .. } => figured(self.key, self.symbol),
        }
    }

    /// The word the chord sheet writes after the numeral, for a chord
    /// from outside the key: "borrowed", "pivot".
    pub fn tag(&self) -> Option<String> {
        if self.pivot.is_some() {
            return Some("pivot".to_string());
        }
        match self.role {
            Role::Borrowed { .. } => Some("borrowed".to_string()),
            _ if self.early => Some(format!("in {}", self.key.name())),
            _ => None,
        }
    }

    /// Whether it is anything but a plain chord of the key on its step:
    /// borrowed, secondary, Neapolitan, a pivot, or read in a key that
    /// has come before its marker.
    pub fn is_marked(&self) -> bool {
        self.role != Role::Plain || self.pivot.is_some() || self.early
    }

    /// The numeral of the chord a secondary chord leads to, in its key:
    /// "V" for G in C major. When the music goes on to that root in the
    /// other mode, the numeral is that chord's: B7 going to Em in A minor
    /// is V7/v, where on its own it would be V7/V.
    fn target_numeral(&self, of: Chord) -> Option<String> {
        let of = match self
            .next
            .map(|next| (next.root.pitch_class(), next.quality.family()))
        {
            Some((root, ChordFamily::Major)) if root == of.root => {
                Chord::new(of.root, ChordQuality::Major)
            }
            Some((root, ChordFamily::Minor)) if root == of.root => {
                Chord::new(of.root, ChordQuality::Minor)
            }
            _ => of,
        };
        self.key.roman_numeral(of)
    }

    /// What the chord sheet says about a chord it marks: its name, its
    /// numeral in its key, what it does there, and its notes.
    pub fn describe(&self) -> String {
        let quality = self.symbol.quality.name();
        let mut text = format!("{}: {} {quality}", self.symbol, self.symbol.root);
        if let Some(bass) = self.symbol.bass {
            text.push_str(&format!(" with {bass} in the bass"));
        }
        match self.own_numeral() {
            Some(numeral) => text.push_str(&format!(", {numeral} in {}.", self.key.name())),
            None => text.push('.'),
        }
        if let Some(about) = self.explain() {
            text.push(' ');
            text.push_str(&about);
        }
        let notes: Vec<String> = self
            .symbol
            .notes()
            .iter()
            .map(NoteName::to_string)
            .collect();
        text.push_str(&format!(" Its notes: {}.", notes.join(" ")));
        text
    }

    /// Sentences that say what the chord does, for its tooltip on the
    /// chord sheet. `None` for a plain chord.
    pub fn explain(&self) -> Option<String> {
        let key = self.key;
        let key_name = key.name();
        let mut text = match self.role {
            Role::Plain => String::new(),
            Role::Borrowed { from } => {
                let outside = self.outside_notes();
                let (mode, colour, mood) = match from.kind {
                    ScaleKind::Major => ("major", "brighten", "the minor key"),
                    _ => ("minor", "darken", "the major key"),
                };
                let (comes, they, colour) = match outside.len() {
                    1 => ("comes", "it", format!("{colour}s")),
                    _ => ("come", "they", colour.to_string()),
                };
                let mut text = format!(
                    "A borrowed chord: {key_name} takes it from {}, the {mode} key on the same \
                     tonic (this is called modal mixture). Its {} {comes} from {}, not \
                     {key_name}: {they} {colour} {mood} for a moment without leaving it.",
                    from.name(),
                    listed(&outside),
                    from.name(),
                );
                let tonic = self.symbol.root.pitch_class() == key.root;
                if tonic && from.kind == ScaleKind::Major && self.last {
                    text.push_str(
                        " Ending a piece in minor on the major tonic chord like this is called a \
                         Picardy third.",
                    );
                }
                text
            }
            Role::SecondaryDominant { of } => {
                let name = key.chord_name(of);
                let numeral = self.target_numeral(of).unwrap_or_default();
                let chord = self.symbol.chord();
                let third = chord
                    .spell(self.symbol.root)
                    .get(1)
                    .map_or_else(String::new, NoteName::to_string);
                let mut text = format!(
                    "A secondary dominant: {} is the dominant of {name}, the {numeral} of \
                     {key_name}, as if {name} were a tonic. Its third, {third}, is the leading \
                     tone of {}, a semitone below it, and pulls up to it, so {name} sounds for a \
                     moment like a key of its own (a tonicization).",
                    self.symbol,
                    key.spell_root(of),
                );
                let outside = self.outside_notes();
                if !outside.is_empty() && !outside.iter().any(|name| name.to_string() == third) {
                    text.push_str(&format!(" {} not in {key_name}.", notes_are(&outside)));
                }
                text.push_str(&self.resolution(of, "as a dominant goes to its tonic"));
                text
            }
            Role::SecondaryLeadingTone { of } => {
                let name = key.chord_name(of);
                let numeral = self.target_numeral(of).unwrap_or_default();
                let mut text = format!(
                    "A secondary leading-tone chord: {} is built on {}, the leading tone of \
                     {name} (the {numeral} of {key_name}), a semitone below its root, and pulls \
                     up into {name} as vii° pulls into I, so {name} sounds for a moment like a \
                     key of its own (a tonicization).",
                    self.symbol, self.symbol.root,
                );
                text.push_str(&self.resolution(of, "its root rising a semitone into it"));
                text
            }
            Role::Neapolitan => {
                let chord = self.symbol.chord();
                let notes = chord.spell(self.symbol.root);
                let second = key.spell(key.root.transpose(2));
                let third = notes.get(1).map_or_else(String::new, NoteName::to_string);
                let dominant = key.spell(key.root.transpose(7));
                format!(
                    "The Neapolitan chord: a major chord on the lowered second degree, {} where \
                     {key_name} has {second}. It is most often played with its third, {third}, in \
                     the bass (N⁶, the Neapolitan sixth), and leads to the dominant, {dominant}, \
                     often by way of the tonic six-four chord.",
                    self.symbol.root,
                )
            }
        };
        if let Some(change) = self.pivot {
            if !text.is_empty() {
                text.push(' ');
            }
            text.push_str(&pivot_text(&change));
        }
        if self.early {
            if !text.is_empty() {
                text.push(' ');
            }
            text.push_str(&format!(
                "{key_name} has already come here, before the key change is marked: after the \
                 pivot chord the music is read in {key_name}."
            ));
        }
        (!text.is_empty()).then_some(text)
    }

    /// Where a secondary chord goes, as a sentence: whether it goes on to
    /// the chord `of` it leads to, `how`.
    fn resolution(&self, of: Chord, how: &str) -> String {
        let name = self.key.chord_name(of);
        match self.next {
            Some(next) if next.root.pitch_class() == of.root => {
                format!(" Here it goes on to {next}, {how}.")
            }
            Some(next) if arrives(next, of) => format!(
                " Here it goes on to {next}, a six-four chord over {}: the bass has arrived, \
                 and {name} comes when the two notes above it fall a step (as in the cadential \
                 six-four).",
                self.key.spell_root(of),
            ),
            Some(next) => format!(" Here it goes to {next} instead of {name}."),
            None => String::new(),
        }
    }

    /// The notes of the chord that are not in its key.
    fn outside_notes(&self) -> Vec<NoteName> {
        self.symbol
            .notes()
            .into_iter()
            .filter(|name| !in_key(self.key, name.pitch_class()))
            .collect()
    }
}

impl Modulation {
    /// What happens at the change, for the tooltip of the key change on
    /// the chord sheet.
    pub fn describe(&self) -> String {
        let (from, to) = (self.from.name(), self.to.name());
        let head = format!("A modulation from {from} to {to}.");
        if !has_chords(self.from) || !has_chords(self.to) {
            return head;
        }
        if self.from.root == self.to.root {
            return format!(
                "{head} The tonic stays and the mode changes: a change of mode, with no chord \
                 needed to lead there."
            );
        }
        match (self.pivot, self.arrival) {
            (Some(_), _) => format!("{head} {}", pivot_text(self)),
            (None, Some((_, arrival))) => format!(
                "{head} {arrival} is the first chord of {to} alone, and the chord before it is \
                 not one of both keys: a direct modulation, with no pivot chord."
            ),
            (None, None) => format!(
                "{head} No chord near the change belongs to {to} alone, so the chord sheet \
                 names no pivot chord."
            ),
        }
    }
}

/// The sentence that explains the pivot chord of `change`.
fn pivot_text(change: &Modulation) -> String {
    let (Some((_, pivot)), Some((_, arrival))) = (change.pivot, change.arrival) else {
        return String::new();
    };
    let (from, to) = (change.from.name(), change.to.name());
    let old = figured(change.from, pivot).unwrap_or_default();
    let new = figured(change.to, pivot).unwrap_or_default();
    format!(
        "The pivot chord of the modulation to {to}: {pivot} is {old} in {from} and {new} in {to}, \
         a chord of both keys. The ear hears it in {from}; {arrival}, which {from} does not \
         have, comes next and shows that it was the {new} of {to}."
    )
}

/// The Roman numeral of `symbol` in `key`, with the figures of an
/// inversion.
fn figured(key: Scale, symbol: ChordSymbol) -> Option<String> {
    key.spelled_figured_numeral(symbol.root, symbol.chord(), symbol.bass_pitch_class())
}

/// "Its Ab is", "Its Ab and Eb are".
fn notes_are(notes: &[NoteName]) -> String {
    match notes.len() {
        1 => format!("Its {} is", listed(notes)),
        _ => format!("Its {} are", listed(notes)),
    }
}

/// Note names in words: "Ab", "Ab and Eb", "F#, A and C".
fn listed(notes: &[NoteName]) -> String {
    let names: Vec<String> = notes.iter().map(NoteName::to_string).collect();
    match names.as_slice() {
        [] => String::new(),
        [one] => one.clone(),
        [rest @ .., last] => format!("{} and {last}", rest.join(", ")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::key_map::KeyMarker;
    use crate::pitch::PitchClass;

    const BAR: usize = 16;

    /// The chords of `text`, one a bar ("C Am D7 G"), read in the keys of
    /// `keys`: the first, and then one from each bar given.
    fn read(text: &str, first: Scale, changes: &[(usize, Scale)]) -> Functions {
        let markers: Vec<ChordMarker> = text
            .split_whitespace()
            .enumerate()
            .map(|(bar, name)| ChordMarker::new(bar * BAR, HarmonyName::parse(name).unwrap()))
            .collect();
        let keys = KeyMap::new(
            first,
            changes
                .iter()
                .map(|&(bar, scale)| KeyMarker::new(bar * BAR, scale)),
        );
        Functions::read(&markers, &keys)
    }

    fn major(root: PitchClass) -> Scale {
        Scale::new(root, ScaleKind::Major)
    }

    fn minor(root: PitchClass) -> Scale {
        Scale::new(root, ScaleKind::NaturalMinor)
    }

    /// The numeral written under each chord.
    fn numerals(functions: &Functions) -> Vec<String> {
        functions
            .chords
            .iter()
            .map(|chord| chord.numeral().unwrap_or_default())
            .collect()
    }

    #[test]
    fn a_chord_from_the_parallel_minor_is_borrowed() {
        let sheet = read("C Am Ab Fm G7 C", major(PitchClass::C), &[]);
        assert_eq!(numerals(&sheet), ["I", "vi", "bVI", "iv", "V7", "I"]);
        let ab = &sheet.chords[2];
        assert_eq!(
            ab.role,
            Role::Borrowed {
                from: minor(PitchClass::C)
            }
        );
        assert_eq!(ab.tag().as_deref(), Some("borrowed"));
        let about = ab.explain().unwrap();
        assert!(about.contains("C minor"), "{about}");
        assert!(about.contains("Its Ab and Eb come from C minor"), "{about}");
        // Fm has one note of C minor's own.
        let fm = sheet.chords[3].explain().unwrap();
        assert!(fm.contains("Its Ab comes from C minor"), "{fm}");
        assert!(fm.contains("it darkens the major key"), "{fm}");
        assert_eq!(sheet.chords[0].explain(), None);
        assert_eq!(
            sheet.chords[2].describe(),
            "Ab: Ab major, bVI in C major. A borrowed chord: C major takes it from C minor, the \
             minor key on the same tonic (this is called modal mixture). Its Ab and Eb come from \
             C minor, not C major: they darken the major key for a moment without leaving it. Its \
             notes: Ab C Eb."
        );
    }

    #[test]
    fn secondary_dominants_are_numbered_over_the_chord_they_lead_to() {
        let sheet = read("C E7 Am D7/F# G7 C A7 Dm", major(PitchClass::C), &[]);
        assert_eq!(
            numerals(&sheet),
            ["I", "V7/vi", "vi", "V⁶₅/V", "V7", "I", "V7/ii", "ii"]
        );
        let d7 = &sheet.chords[3];
        assert_eq!(
            d7.role,
            Role::SecondaryDominant {
                of: Chord::new(PitchClass::G, ChordQuality::Major)
            }
        );
        let about = d7.explain().unwrap();
        assert!(
            about.contains("the dominant of G, the V of C major"),
            "{about}"
        );
        assert!(
            about.contains("Its third, F#, is the leading tone of G"),
            "{about}"
        );
        assert!(about.contains("Here it goes on to G7"), "{about}");
    }

    #[test]
    fn a_secondary_dominant_that_goes_elsewhere_says_so() {
        let sheet = read("C E7 F G7 C", major(PitchClass::C), &[]);
        assert_eq!(sheet.chords[1].numeral().as_deref(), Some("V7/vi"));
        let about = sheet.chords[1].explain().unwrap();
        assert!(about.contains("Here it goes to F instead of Am"), "{about}");
    }

    #[test]
    fn a_dominant_seventh_on_the_tonic_points_to_the_subdominant() {
        let sheet = read("C C7 F G7 C", major(PitchClass::C), &[]);
        assert_eq!(numerals(&sheet), ["I", "V7/IV", "IV", "V7", "I"]);
        let about = sheet.chords[1].explain().unwrap();
        assert!(
            about.contains("Its third, E, is the leading tone of F"),
            "{about}"
        );
        assert!(about.contains("Its Bb is not in C major"), "{about}");
    }

    #[test]
    fn secondary_leading_tone_chords_rise_into_their_chord() {
        let sheet = read(
            "C F#dim7 G G#dim Am C#m7b5 Dm G7 C",
            major(PitchClass::C),
            &[],
        );
        assert_eq!(
            numerals(&sheet),
            [
                "I",
                "vii°7/V",
                "V",
                "vii°/vi",
                "vi",
                "viiø7/ii",
                "ii",
                "V7",
                "I"
            ]
        );
        let about = sheet.chords[1].explain().unwrap();
        assert!(
            about.contains("built on F#, the leading tone of G"),
            "{about}"
        );
        assert!(about.contains("Here it goes on to G"), "{about}");
    }

    #[test]
    fn a_leading_tone_chord_that_goes_elsewhere_keeps_its_own_numeral() {
        // Autumn Leaves in C: F#ø7 B7 Em is ii, V and i of E minor.
        let sheet = read("Am7 D7 Gmaj7 Cmaj7 F#m7b5 B7 Em", major(PitchClass::C), &[]);
        assert_eq!(
            numerals(&sheet),
            ["vi7", "V7/V", "Vmaj7", "Imaj7", "#ivø7", "V7/iii", "iii"]
        );
        assert_eq!(sheet.chords[4].role, Role::Plain);
    }

    #[test]
    fn a_six_four_chord_over_its_root_is_where_a_secondary_chord_goes() {
        // V/V and vii°7/V going on to the cadential six-four, I⁶₄, and
        // then to V.
        for chord in ["D7", "F#dim7"] {
            let text = format!("C {chord} C/G G7 C");
            let sheet = read(&text, major(PitchClass::C), &[]);
            let secondary = &sheet.chords[1];
            assert!(secondary.role != Role::Plain, "{chord}");
            let about = secondary.explain().unwrap();
            assert!(
                about.contains("Here it goes on to C/G, a six-four chord over G"),
                "{about}"
            );
        }
        // C/E is no six-four chord: D7 goes to it instead of G.
        let sheet = read("C D7 C/E G7 C", major(PitchClass::C), &[]);
        let about = sheet.chords[1].explain().unwrap();
        assert!(
            about.contains("Here it goes to C/E instead of G"),
            "{about}"
        );
    }

    #[test]
    fn the_neapolitan_is_a_major_chord_on_the_lowered_second() {
        let sheet = read("Cm Db/F G7 Cm", minor(PitchClass::C), &[]);
        assert_eq!(sheet.chords[1].role, Role::Neapolitan);
        assert_eq!(sheet.chords[1].numeral().as_deref(), Some("N⁶"));
        let about = sheet.chords[1].explain().unwrap();
        assert!(about.contains("Db where C minor has D"), "{about}");
        assert!(about.contains("its third, F, in the bass"), "{about}");
        // C# major in C major is no Neapolitan: it is written on C.
        let sharp = read("C C# C", major(PitchClass::C), &[]);
        assert_eq!(sharp.chords[1].role, Role::Plain);
    }

    #[test]
    fn in_minor_the_dominant_of_the_dominant_is_v_of_v() {
        let a = minor(PitchClass::A);
        let sheet = read("Am B7 E7 Am", a, &[]);
        assert_eq!(numerals(&sheet), ["i", "V7/V", "V7", "i"]);
        // Going to a minor v, it is the V of v.
        let sheet = read("Am B7 Em Am", a, &[]);
        assert_eq!(sheet.chords[1].numeral().as_deref(), Some("V7/v"));
    }

    #[test]
    fn a_major_tonic_in_minor_leads_to_iv_or_ends_the_piece() {
        let a = minor(PitchClass::A);
        let sheet = read("Am A7 Dm E7 A", a, &[]);
        assert_eq!(sheet.chords[1].numeral().as_deref(), Some("V7/iv"));
        let last = &sheet.chords[4];
        assert_eq!(
            last.role,
            Role::Borrowed {
                from: major(PitchClass::A)
            }
        );
        let about = last.explain().unwrap();
        assert!(about.contains("Picardy third"), "{about}");
        assert!(about.contains("Its C# comes from A major"), "{about}");
        assert!(about.contains("it brightens the minor key"), "{about}");
    }

    #[test]
    fn the_pivot_chord_is_the_last_chord_of_both_keys() {
        let g = major(PitchClass::G);
        // The key may be marked as changing on the Am, on the D7, or on
        // the G, where the new key has arrived.
        for marker in [1, 2, 3] {
            let sheet = read(
                "C Am D7 G C/E Am/C D7 G",
                major(PitchClass::C),
                &[(marker, g)],
            );
            assert_eq!(sheet.modulations.len(), 1);
            let change = sheet.modulations[0];
            assert_eq!(change.pivot.map(|(step, _)| step), Some(BAR), "{marker}");
            assert_eq!(
                change.arrival.map(|(step, _)| step),
                Some(2 * BAR),
                "{marker}"
            );
            let am = &sheet.chords[1];
            assert_eq!(am.numeral().as_deref(), Some("vi = ii"));
            let described = am.describe();
            assert!(described.starts_with("Am: A minor, "), "{described}");
            assert!(described.contains(". The pivot chord"), "{described}");
            assert_eq!(am.tag().as_deref(), Some("pivot"));
            // From the D7 on the music is in G major, wherever the key
            // change is marked.
            let d7 = &sheet.chords[2];
            assert_eq!(d7.numeral().as_deref(), Some("V7"));
            assert_eq!(d7.key, g);
            assert_eq!(d7.early, marker == 3);
            assert_eq!(d7.is_marked(), marker == 3);
            let about = am.explain().unwrap();
            assert!(
                about.contains("Am is vi in C major and ii in G major"),
                "{about}"
            );
            assert!(about.contains("D7, which C major does not have"), "{about}");
            assert!(change.describe().contains("from C major to G major"));
        }
    }

    #[test]
    fn a_borrowed_chord_can_be_a_pivot() {
        // Fm is iv borrowed from C minor, and vi of Ab major.
        let ab = major(PitchClass::GSharp);
        let sheet = read("C F Fm Eb7 Ab", major(PitchClass::C), &[(4, ab)]);
        let change = sheet.modulations[0];
        assert_eq!(change.pivot.map(|(step, _)| step), Some(2 * BAR));
        assert_eq!(sheet.chords[2].numeral().as_deref(), Some("iv = vi"));
    }

    #[test]
    fn a_change_without_a_common_chord_is_direct() {
        let db = major(PitchClass::CSharp);
        let sheet = read("C F G C Db Gb Ab Db", major(PitchClass::C), &[(4, db)]);
        let change = sheet.modulations[0];
        assert_eq!(change.pivot, None);
        assert_eq!(change.arrival.map(|(step, _)| step), Some(4 * BAR));
        assert!(
            change.describe().contains("a direct modulation"),
            "{}",
            change.describe()
        );
        assert!(sheet.chords.iter().all(|chord| chord.pivot.is_none()));
    }

    #[test]
    fn a_change_of_mode_keeps_the_tonic() {
        let sheet = read(
            "C F G C Cm Fm G7 Cm",
            major(PitchClass::C),
            &[(4, minor(PitchClass::C))],
        );
        let change = sheet.modulations[0];
        assert_eq!(change.pivot, None);
        assert!(change.describe().contains("change of mode"));
        // In C minor, Cm and Fm are the key's own.
        assert_eq!(sheet.chords[5].role, Role::Plain);
    }

    #[test]
    fn a_minor_key_has_the_chords_of_its_three_scales() {
        let harmonic = Scale::new(PitchClass::A, ScaleKind::HarmonicMinor);
        // C and G are A minor's own chords, set as harmonic minor or not:
        // not secondary dominants of F and C.
        for key in [harmonic, minor(PitchClass::A)] {
            let sheet = read("Am C G7 C F Dm E7 Am", key, &[]);
            assert!(
                sheet.chords.iter().all(|chord| chord.role == Role::Plain),
                "{:?}",
                numerals(&sheet)
            );
        }
        // D major, from melodic minor, is the IV on its way to E, and the
        // V of G when it goes there.
        let sheet = read("Am D E7 Am D G C", harmonic, &[]);
        assert_eq!(sheet.chords[1].role, Role::Plain);
        assert!(matches!(
            sheet.chords[4].role,
            Role::SecondaryDominant { of } if of.root == PitchClass::G
        ));
        // B7, the V7/V, has one note from outside A minor: D#.
        let sheet = read("Am B7 E7 Am", harmonic, &[]);
        let about = sheet.chords[1].explain().unwrap();
        assert!(!about.contains("not in"), "{about}");
        assert!(about.contains("Its third, D#"), "{about}");
    }

    #[test]
    fn a_mode_names_no_borrowed_or_secondary_chords() {
        // E major in E Phrygian is the tonic of flamenco, going to F.
        let phrygian = Scale::new(PitchClass::E, ScaleKind::Phrygian);
        let sheet = read("E F G F E", phrygian, &[]);
        assert!(sheet.chords.iter().all(|chord| !chord.is_marked()));
    }

    #[test]
    fn a_borrowed_chord_is_written_as_the_other_key_writes_it() {
        let c = major(PitchClass::C);
        // G augmented has the notes of Eb augmented, from C minor, but
        // its D# is no Eb.
        let sheet = read("C Gaug C Bdim7 C", c, &[]);
        assert_eq!(sheet.chords[1].role, Role::Plain);
        assert_eq!(
            sheet.chords[3].role,
            Role::Borrowed {
                from: minor(PitchClass::C)
            }
        );
    }
}
