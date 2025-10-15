pub mod delta;
pub mod effect;

use crate::{
    state::delta::Delta,
    types::{
        character_set::{AsciiVariant, Codepage},
        font::Font,
    },
};

#[derive(Clone, Default)]
pub struct State {
    pub(crate) font: Option<Font>,
    pub(crate) ascii_variant: Option<AsciiVariant>,
    pub(crate) codepage: Option<Codepage>,
}

pub trait IntoState {
    fn into_state(&self) -> State;
}

impl State {
    pub fn with_codepage(mut self, codepage: Codepage) -> Self {
        self.codepage = Some(codepage);
        self
    }

    pub fn with_ascii_variant(mut self, ascii_variant: AsciiVariant) -> Self {
        self.ascii_variant = Some(ascii_variant);
        self
    }

    pub fn with_font(mut self, font: Font) -> Self {
        self.font = Some(font);
        self
    }

    pub fn codepage(&self) -> Option<Codepage> {
        self.codepage
    }

    pub fn ascii_variant(&self) -> Option<AsciiVariant> {
        self.ascii_variant
    }

    pub fn font(&self) -> Option<Font> {
        self.font
    }

    pub fn delta(&self, into: State) -> Delta {
        let mut delta = Delta::empty();

        if let Some(font) = into.font
            && Some(font) != self.font
        {
            delta.apply_font = Some(font);
        }

        if let Some(ascii_variant) = into.ascii_variant
            && Some(ascii_variant) != self.ascii_variant
        {
            delta.apply_ascii_variant = Some(ascii_variant);
        }

        if let Some(codepage) = into.codepage
            && Some(codepage) != self.codepage
        {
            delta.apply_codepage = Some(codepage);
        }

        delta
    }
}
