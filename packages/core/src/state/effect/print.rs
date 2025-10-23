use std::iter::once;

use crate::{
    state::effect::{Effect, IntoEffects},
    types::{
        character_set::{AsciiVariant, Codepage},
        font::Font,
        justification::Justification,
    },
};

#[derive(Debug, Clone)]
pub enum WriteContents {
    Utf8(String),
    AsciiLike(Vec<u8>, AsciiVariant, Codepage),
}

impl WriteContents {
    pub fn from_str(v: &str) -> WriteContents {
        Self::Utf8(v.to_string())
    }
}

#[derive(Debug, Clone)]
pub struct Write {
    pub contents: WriteContents,
    pub font: Font,
    pub justification: Justification,
    pub scale: (u8, u8),
}

impl Write {
    pub fn from_str(v: &str) -> Write {
        Write {
            contents: WriteContents::from_str(v),
            font: Font::A,
            justification: Justification::LeftJustified,
            scale: (1, 1),
        }
    }

    pub fn with_font(mut self, font: Font) -> Write {
        self.font = font;
        self
    }

    pub fn with_justification(mut self, justification: Justification) -> Write {
        self.justification = justification;
        self
    }

    pub fn with_scale(mut self, x_scale: u8, y_scale: u8) -> Write {
        self.scale = (x_scale, y_scale);
        self
    }
}

impl IntoEffects for Write {
    fn as_effects(self) -> impl Iterator<Item = Effect> {
        once(Effect::Write(self))
    }
}
