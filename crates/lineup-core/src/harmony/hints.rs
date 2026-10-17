//! How much the harmony hints say.
//!
//! The hints explain what a chord does and which chords often follow it.
//! While you are learning, the whole sentence helps: "IV to V: a step up
//! to the dominant, ready to come home." Once you know it, a few words
//! are enough to remind you ("step up to the dominant"), and later just
//! the chords themselves, with no reason at all.

/// How much the harmony hints explain.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HintLevel {
    /// Whole sentences, for learning.
    Detailed,
    /// The same information in a few words.
    #[default]
    Short,
    /// Only the chords: you know or can work out why.
    Minimal,
}

impl HintLevel {
    pub const ALL: [HintLevel; 3] = [HintLevel::Detailed, HintLevel::Short, HintLevel::Minimal];

    pub fn label(self) -> &'static str {
        match self {
            HintLevel::Detailed => "Detailed",
            HintLevel::Short => "Short",
            HintLevel::Minimal => "Minimal",
        }
    }

    /// What the level shows, for its tooltip.
    pub fn description(self) -> &'static str {
        match self {
            HintLevel::Detailed => "Whole sentences explaining each chord and move",
            HintLevel::Short => "The same explanations in a few words",
            HintLevel::Minimal => "Only the chord names, no explanations",
        }
    }
}
