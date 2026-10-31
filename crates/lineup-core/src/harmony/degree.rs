//! Where a form sits in a key made of one set: its **degree**.
//!
//! In C major the chord on D is ii: "ii" is what D minor is called when
//! the key is moved to C, so a progression written in numerals plays in
//! any key. Music built on one pitch-class set, such as one of Kröpfl's
//! micromodes, needs the same kind of name. Every form of the set has
//! the same intervals, so only two things tell the forms apart: how far
//! each is moved (`T`, for transposed) and whether it is upside down
//! (`I`, for inverted).
//!
//! So a form's degree is the name it has when the key's own form is
//! moved to C: in a key of Menor 3 on D (D D# F#), E F G# is `T2`, a
//! whole tone above home, and the key's own form is `T0`. The names are
//! the ones the chord sheet already writes after a set ("Menor 3 T2",
//! "3-11 T7I"), counted from the key instead of from C, and like a
//! numeral the degree stays the same when the whole piece moves to
//! another key.
//!
//! Two forms can also share notes. For three notes, keeping two and
//! moving one is the smoothest link, keeping one note is a pivot, and
//! keeping none is a contrast: [`kept`] counts them. For the major and
//! minor triads, the three links that keep two notes have names of their
//! own, the P, L and R of neo-Riemannian theory ([`Triadic`]).

use std::fmt;

use super::PitchSet;
use crate::i18n::tr;
use crate::pitch::PitchClass;

/// The degree of a form: the name it has when the key's own form is
/// moved to C (see the module documentation).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Degree {
    /// How many semitones it is moved, from 0 to 11.
    pub up: u8,
    /// Whether it is upside down from the set's prime form.
    pub inverted: bool,
}

impl Degree {
    /// The key's own form, when that form is the prime form moved.
    pub const HOME: Degree = Degree {
        up: 0,
        inverted: false,
    };

    /// The degree of `form` in a key whose own form is `key`, or `None`
    /// if the two are not the same set moved (not of one set class).
    pub fn of(key: PitchSet, form: PitchSet) -> Option<Degree> {
        let prime = key.prime_form();
        if key.is_empty() || form.prime_form() != prime {
            return None;
        }
        let (up, _) = prime.moved_to(key)?;
        let (up, inverted) = prime.moved_to(form.transpose((12 - up) % 12))?;
        Some(Degree { up, inverted })
    }

    /// The form at this degree in a key whose own form is `key`: the
    /// prime form moved as the degree says, then moved as far as the key
    /// is.
    pub fn form(self, key: PitchSet) -> Option<PitchSet> {
        let prime = key.prime_form();
        let (up, _) = prime.moved_to(key)?;
        let shape = if self.inverted { prime.invert() } else { prime };
        Some(shape.transpose((self.up + up) % 12))
    }

    /// Reads a degree as the chord sheet writes it: "T2", "T7I", "t0".
    pub fn parse(text: &str) -> Option<Degree> {
        let number = text.trim().strip_prefix(['T', 't'])?;
        let (number, inverted) = match number.strip_suffix(['I', 'i']) {
            Some(number) => (number, true),
            None => (number, false),
        };
        if number.is_empty() || !number.chars().all(|c| c.is_ascii_digit()) {
            return None;
        }
        let up = number.parse::<u8>().ok().filter(|&up| up < 12)?;
        Some(Degree { up, inverted })
    }

    /// Whether it is the key's own form. The key's form may itself be
    /// upside down from the prime form (the major triad, F A C, is the
    /// minor triad turned over), so this asks the key.
    pub fn is_home(self, key: PitchSet) -> bool {
        self.form(key) == Some(key)
    }
}

impl fmt::Display for Degree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "T{}", self.up)?;
        if self.inverted {
            write!(f, "I")?;
        }
        Ok(())
    }
}

/// How many notes two groups share.
pub fn kept(a: PitchSet, b: PitchSet) -> usize {
    PitchSet::from_bits(a.bits() & b.bits()).len()
}

/// The notes of `to` that are not in `from`, and the notes of `from` that
/// `to` leaves out: what arrives and what goes when one group follows
/// the other.
pub fn changed(from: PitchSet, to: PitchSet) -> (Vec<PitchClass>, Vec<PitchClass>) {
    let arriving = PitchSet::from_bits(to.bits() & !from.bits())
        .iter()
        .collect();
    let leaving = PitchSet::from_bits(from.bits() & !to.bits())
        .iter()
        .collect();
    (arriving, leaving)
}

/// Every form of the set `key` is a form of (the set itself, moved and
/// turned upside down), each once: the key's own form first, then the
/// others from the nearest to the farthest. A form is nearer the more
/// notes it shares with the key's; among forms as near, by degree, the
/// form the right way up before the one upside down.
pub fn forms_by_nearness(key: PitchSet) -> Vec<(PitchSet, Degree)> {
    let prime = key.prime_form();
    let inverted = prime.invert();
    let mut forms: Vec<(PitchSet, Degree)> = Vec::new();
    for up in 0..12 {
        for shape in [prime, inverted] {
            let form = shape.transpose(up);
            if forms.iter().any(|(seen, _)| *seen == form) {
                continue;
            }
            if let Some(degree) = Degree::of(key, form) {
                forms.push((form, degree));
            }
        }
    }
    forms.sort_by_key(|&(form, degree)| {
        let home = form == key;
        (
            !home,
            std::cmp::Reverse(kept(key, form)),
            degree.inverted,
            degree.up,
        )
    });
    forms
}

/// The forms that keep all but one note of `form`: the smoothest ways
/// on, each moving a single note. For three notes, the forms that keep
/// two.
pub fn smooth_links(form: PitchSet) -> Vec<PitchSet> {
    let size = form.len();
    if size < 2 {
        return Vec::new();
    }
    forms_by_nearness(form)
        .into_iter()
        .map(|(other, _)| other)
        .filter(|&other| other != form && kept(form, other) == size - 1)
        .collect()
}

/// The three moves between major and minor triads that keep two notes,
/// as neo-Riemannian theory names them.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Triadic {
    /// Parallel: C major and C minor keep the root and the fifth.
    Parallel,
    /// Leading-tone exchange: C major and E minor keep E and G.
    LeadingTone,
    /// Relative: C major and A minor keep C and E.
    Relative,
}

impl Triadic {
    pub const ALL: [Triadic; 3] = [Triadic::Parallel, Triadic::LeadingTone, Triadic::Relative];

    /// The letter theorists write for it: P, L or R.
    pub fn letter(self) -> &'static str {
        match self {
            Triadic::Parallel => "P",
            Triadic::LeadingTone => "L",
            Triadic::Relative => "R",
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Triadic::Parallel => "parallel",
            Triadic::LeadingTone => "leading-tone exchange",
            Triadic::Relative => "relative",
        }
    }

    /// The move from triad `from` to triad `to`, if they are a major and
    /// a minor triad that keep two notes.
    pub fn between(from: PitchSet, to: PitchSet) -> Option<Triadic> {
        let triad = PitchSet::from_semitones(&[0, 3, 7]);
        if from.prime_form() != triad || to.prime_form() != triad || kept(from, to) != 2 {
            return None;
        }
        let major_root = |set: PitchSet| {
            set.iter()
                .find(|&root| set.relative_to(root) == PitchSet::from_semitones(&[0, 4, 7]))
        };
        let minor_root = |set: PitchSet| {
            set.iter()
                .find(|&root| set.relative_to(root) == PitchSet::from_semitones(&[0, 3, 7]))
        };
        // Name the move from the major triad, whichever comes first.
        let (major, minor) = match (major_root(from), minor_root(to)) {
            (Some(major), Some(minor)) => (major, minor),
            _ => (major_root(to)?, minor_root(from)?),
        };
        match major.semitones_up_to(minor) {
            0 => Some(Triadic::Parallel),
            4 => Some(Triadic::LeadingTone),
            9 => Some(Triadic::Relative),
            _ => None,
        }
    }
}

/// How one group goes on to the next, in words: "keeps C# and E, C goes
/// to D", "keeps G, moves C and E", "keeps no note". Notes are named by
/// `name`.
pub fn describe_link(from: PitchSet, to: PitchSet, name: impl Fn(PitchClass) -> String) -> String {
    let shared: Vec<String> = PitchSet::from_bits(from.bits() & to.bits())
        .iter()
        .map(&name)
        .collect();
    let (arriving, leaving) = changed(from, to);
    if shared.is_empty() {
        return tr("keeps no note").to_string();
    }
    let shared = crate::i18n::list(&shared);
    match (leaving.as_slice(), arriving.as_slice()) {
        ([], []) => crate::i18n::trf("keeps every note, {}", &[&shared]),
        ([gone], [new]) => crate::i18n::trf(
            "keeps {}; {} goes to {}",
            &[&shared, &name(*gone), &name(*new)],
        ),
        _ => {
            let gone: Vec<String> = leaving.iter().map(|&p| name(p)).collect();
            let new: Vec<String> = arriving.iter().map(|&p| name(p)).collect();
            crate::i18n::trf(
                "keeps {}; {} go to {}",
                &[&shared, &crate::i18n::list(&gone), &crate::i18n::list(&new)],
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::harmony::Micromode;

    fn set(semitones: &[u8]) -> PitchSet {
        PitchSet::from_semitones(semitones)
    }

    #[test]
    fn a_degree_counts_from_the_keys_own_form() {
        // Menor 3 on D: D D# F#. E F G# is a whole tone up.
        let key = set(&[2, 3, 6]);
        assert_eq!(Degree::of(key, key), Some(Degree::HOME));
        let up = Degree::of(key, set(&[4, 5, 8])).unwrap();
        assert_eq!(up.to_string(), "T2");
        assert_eq!(up.form(key), Some(set(&[4, 5, 8])));
        // The same as the chord sheet's name for it with the key on C.
        let on_c = set(&[0, 1, 4]);
        for form in Micromode::named("Menor 3").unwrap().members() {
            let degree = Degree::of(key, form).unwrap();
            let moved_to_c = form.transpose(10);
            assert_eq!(Degree::of(on_c, moved_to_c), Some(degree));
            assert_eq!(degree.form(key), Some(form));
        }
        // Not the same set: no degree.
        assert_eq!(Degree::of(key, set(&[0, 4, 7])), None);
    }

    #[test]
    fn a_degree_stays_when_the_piece_moves() {
        for micromode in Micromode::all() {
            let key = micromode.prime.transpose(3);
            for form in micromode.members() {
                let degree = Degree::of(key, form).unwrap();
                for up in 1..12 {
                    let moved = Degree::of(key.transpose(up), form.transpose(up));
                    assert_eq!(moved, Some(degree), "{} {form:?}", micromode.name);
                }
            }
        }
    }

    #[test]
    fn a_key_upside_down_is_home_too() {
        // C minor is the prime form of the triads; F A C is it upside
        // down, so in a key of F major home is T0I.
        let f_major = set(&[5, 9, 0]);
        let home = Degree::of(f_major, f_major).unwrap();
        assert!(home.inverted);
        assert!(home.is_home(f_major));
        assert!(!Degree::HOME.is_home(f_major));
    }

    #[test]
    fn degrees_are_read_as_written() {
        assert_eq!(
            Degree::parse("T7I"),
            Some(Degree {
                up: 7,
                inverted: true
            })
        );
        assert_eq!(Degree::parse("t0"), Some(Degree::HOME));
        assert_eq!(Degree::parse("T12"), None);
        assert_eq!(Degree::parse("T"), None);
        assert_eq!(Degree::parse("TI"), None);
        assert_eq!(Degree::parse("T-1"), None);
    }

    #[test]
    fn forms_go_from_the_nearest_to_the_farthest() {
        // Menor 3: 24 forms; 3 keep two notes, 9 keep one, 11 none.
        let key = set(&[0, 1, 4]);
        let forms = forms_by_nearness(key);
        assert_eq!(forms.len(), 24);
        assert_eq!(forms[0], (key, Degree::HOME));
        let count = |n| {
            forms
                .iter()
                .skip(1)
                .filter(|(f, _)| kept(key, *f) == n)
                .count()
        };
        assert_eq!((count(2), count(1), count(0)), (3, 9, 11));
        let kept_in_order: Vec<usize> = forms.iter().map(|(f, _)| kept(key, *f)).collect();
        assert!(kept_in_order.windows(2).all(|w| w[0] >= w[1]));
        // The augmented triad has only four forms, none sharing two notes.
        let augmented = set(&[0, 4, 8]);
        assert_eq!(forms_by_nearness(augmented).len(), 4);
        assert!(smooth_links(augmented).is_empty());
        // The smooth links of Menor 3, as the book lists them.
        let links = smooth_links(key);
        for form in [set(&[9, 0, 1]), set(&[0, 3, 4]), set(&[1, 4, 5])] {
            assert!(links.contains(&form), "{form:?}");
        }
        assert_eq!(links.len(), 3);
    }

    #[test]
    fn the_triads_moves_have_their_names() {
        let c = set(&[0, 4, 7]);
        assert_eq!(
            Triadic::between(c, set(&[0, 3, 7])),
            Some(Triadic::Parallel)
        );
        assert_eq!(
            Triadic::between(c, set(&[4, 7, 11])),
            Some(Triadic::LeadingTone)
        );
        assert_eq!(
            Triadic::between(c, set(&[9, 0, 4])),
            Some(Triadic::Relative)
        );
        // The same move, from the minor triad back.
        assert_eq!(
            Triadic::between(set(&[9, 0, 4]), c),
            Some(Triadic::Relative)
        );
        // Two major triads share no two notes; Menor 3 is not a triad.
        assert_eq!(Triadic::between(c, set(&[7, 11, 2])), None);
        assert_eq!(Triadic::between(set(&[0, 1, 4]), set(&[1, 4, 5])), None);
        // The three smooth links of C major are exactly P, L and R.
        let moves: Vec<Triadic> = smooth_links(c)
            .into_iter()
            .filter_map(|form| Triadic::between(c, form))
            .collect();
        assert_eq!(moves.len(), 3);
    }

    #[test]
    fn a_link_says_what_stays_and_what_moves() {
        let name = |p: PitchClass| p.to_string();
        let text = describe_link(set(&[0, 1, 4]), set(&[1, 4, 5]), name);
        assert!(text.contains("goes to"), "{text}");
        assert_eq!(
            describe_link(set(&[0, 1, 4]), set(&[6, 7, 10]), |p| p.to_string()),
            "keeps no note"
        );
    }
}
