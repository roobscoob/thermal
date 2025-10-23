use std::slice::Iter;

use crate::{
    state::effect::{Effect, IntoEffects},
    types::{
        character_set::{AsciiVariant, Codepage},
        font::Font,
        justification::Justification,
    },
};

#[derive(Default, Debug)]
pub struct Delta {
    effects: Vec<Effect>,
    pub apply_font: Option<Font>,
    pub apply_ascii_variant: Option<AsciiVariant>,
    pub apply_codepage: Option<Codepage>,
    pub apply_justification: Option<Justification>,
    pub apply_text_scale: Option<(u8, u8)>,
}

impl Delta {
    pub fn empty() -> Delta {
        Delta::default()
    }

    pub fn with(mut self, effect: impl IntoEffects) -> Delta {
        self.add(effect);
        self
    }

    pub fn add(&mut self, effect: impl IntoEffects) {
        for item in effect.as_effects() {
            self.effects.push(item);
        }
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
        if other.apply_justification.is_some() {
            self.apply_justification = other.apply_justification
        }
        if other.apply_text_scale.is_some() {
            self.apply_text_scale = other.apply_text_scale
        }
        self
    }
}

impl std::ops::Add for Delta {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.merged_with(rhs)
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
