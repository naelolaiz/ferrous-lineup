//! A set of pitch classes packed into the bits of one integer.

use crate::PitchClass;

/// A set of pitch classes, stored as 12 bits of a `u16`.
///
/// Bit 0 means "C is in the set", bit 1 "C# is in the set", and so on up
/// to bit 11 for B. A C major chord (C, E, G) is the number
/// `0b0000_1001_0001`: bits 0, 4 and 7. Sets this small are cheap to copy
/// and compare, and comparing two chords is a single `==` on integers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct PitchSet(u16);

/// The twelve bits that hold pitch classes.
const ALL_BITS: u16 = 0b1111_1111_1111;

impl PitchSet {
    /// The empty set.
    pub const EMPTY: PitchSet = PitchSet(0);

    /// The set of pitch classes some semitones above C, such as
    /// `[0, 4, 7]` for C, E and G.
    pub fn from_semitones(semitones: &[u8]) -> Self {
        semitones
            .iter()
            .map(|&semitone| PitchClass::from_semitone(semitone))
            .collect()
    }

    /// Adds a pitch class (adding one that is already there does nothing).
    pub fn insert(&mut self, pitch_class: PitchClass) {
        self.0 |= 1 << pitch_class.semitone();
    }

    pub fn contains(self, pitch_class: PitchClass) -> bool {
        self.0 & (1 << pitch_class.semitone()) != 0
    }

    /// How many pitch classes are in the set.
    pub fn len(self) -> usize {
        self.0.count_ones() as usize
    }

    pub fn is_empty(self) -> bool {
        self.0 == 0
    }

    /// Whether every pitch class of `self` is also in `other`.
    pub fn is_subset_of(self, other: PitchSet) -> bool {
        self.0 & !other.0 == 0
    }

    /// The pitch classes in the set, from C upwards.
    pub fn iter(self) -> impl Iterator<Item = PitchClass> {
        PitchClass::ALL
            .into_iter()
            .filter(move |&pitch_class| self.contains(pitch_class))
    }

    /// Moves every pitch class up by some semitones, wrapping around the
    /// octave: C, E, G moved up by 2 becomes D, F#, A.
    pub fn transpose(self, semitones: u8) -> PitchSet {
        let shift = (semitones % 12) as u32;
        let bits = self.0 as u32;
        // Rotate the 12 bits left by `shift`: bits that go past B come
        // back in at C.
        let rotated = (bits << shift) | (bits >> (12 - shift));
        PitchSet(rotated as u16 & ALL_BITS)
    }

    /// The same set seen from `root`: every pitch class is moved down so
    /// that `root` becomes C. E, G and B seen from E become C, D# and G,
    /// which is how a minor triad looks when it starts on C.
    pub fn relative_to(self, root: PitchClass) -> PitchSet {
        self.transpose(12 - root.semitone())
    }
}

/// Lets you write `notes.map(...).collect::<PitchSet>()`.
impl FromIterator<PitchClass> for PitchSet {
    fn from_iter<I: IntoIterator<Item = PitchClass>>(iter: I) -> Self {
        let mut set = PitchSet::EMPTY;
        for pitch_class in iter {
            set.insert(pitch_class);
        }
        set
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use PitchClass::*;

    #[test]
    fn insert_and_contains() {
        let mut set = PitchSet::EMPTY;
        assert!(set.is_empty());
        set.insert(E);
        set.insert(E);
        assert!(set.contains(E));
        assert!(!set.contains(F));
        assert_eq!(set.len(), 1);
    }

    #[test]
    fn iterates_from_c_upwards() {
        let set: PitchSet = [G, C, E].into_iter().collect();
        assert_eq!(set.iter().collect::<Vec<_>>(), vec![C, E, G]);
        assert_eq!(set, PitchSet::from_semitones(&[0, 4, 7]));
    }

    #[test]
    fn seen_from_another_root() {
        let e_minor: PitchSet = [E, G, B].into_iter().collect();
        assert_eq!(e_minor.relative_to(E), PitchSet::from_semitones(&[0, 3, 7]));
        assert_eq!(e_minor.relative_to(C), e_minor);
        let b_flat: PitchSet = [ASharp, D, F].into_iter().collect();
        assert_eq!(
            b_flat.relative_to(ASharp),
            PitchSet::from_semitones(&[0, 4, 7])
        );
    }

    #[test]
    fn transposing_wraps_around() {
        let c_major: PitchSet = [C, E, G].into_iter().collect();
        let d_major: PitchSet = [D, FSharp, A].into_iter().collect();
        let a_major: PitchSet = [A, CSharp, E].into_iter().collect();
        assert_eq!(c_major.transpose(2), d_major);
        assert_eq!(c_major.transpose(9), a_major);
        assert_eq!(c_major.transpose(12), c_major);
    }

    #[test]
    fn subsets() {
        let c_major_scale = PitchSet::from_semitones(&[0, 2, 4, 5, 7, 9, 11]);
        let a_minor: PitchSet = [A, C, E].into_iter().collect();
        let e_major: PitchSet = [E, GSharp, B].into_iter().collect();
        assert!(a_minor.is_subset_of(c_major_scale));
        assert!(!e_major.is_subset_of(c_major_scale));
    }
}
