use std::slice::Iter;

use crate::{
    state::effect::Effect,
    types::{
        character_set::{AsciiVariant, Codepage},
        font::Font,
    },
};

#[derive(Default, Debug)]
pub struct Delta {
    effects: Vec<Effect>,
    pub apply_font: Option<Font>,
    pub apply_ascii_variant: Option<AsciiVariant>,
    pub apply_codepage: Option<Codepage>,
}

impl Delta {
    pub fn empty() -> Delta {
        Delta::default()
    }

    pub fn with(mut self, effect: impl Into<Effect>) -> Delta {
        self.add(effect);
        self
    }

    pub fn add(&mut self, effect: impl Into<Effect>) {
        self.effects.push(effect.into());
    }

    pub fn iter<'a>(&'a self) -> Iter<'a, Effect> {
        self.effects.iter()
    }

    /// Merge `other` into `self`, returning the combined delta.
    /// - `effects` are concatenated (self’s first, then other’s).
    /// - For `apply_*`, the last non-None seen wins.
    pub fn merged_with(mut self, other: Delta) -> Delta {
        self.effects.extend(other.effects);
        if other.apply_font.is_some() {
            self.apply_font = other.apply_font;
        }
        if other.apply_ascii_variant.is_some() {
            self.apply_ascii_variant = other.apply_ascii_variant;
        }
        if other.apply_codepage.is_some() {
            self.apply_codepage = other.apply_codepage;
        }
        self
    }
}

impl IntoIterator for Delta {
    type Item = Effect;
    type IntoIter = std::vec::IntoIter<Effect>;

    fn into_iter(self) -> Self::IntoIter {
        self.effects.into_iter()
    }
}

impl FromIterator<Delta> for Delta {
    fn from_iter<I: IntoIterator<Item = Delta>>(iter: I) -> Self {
        let mut out = Delta::default();
        for d in iter {
            out = out.merged_with(d);
        }
        out
    }
}
