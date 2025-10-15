use crate::{
    state::effect::Effect,
    types::{
        character_set::{AsciiVariant, Codepage},
        font::Font,
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
}

impl Write {
    pub fn from_str(v: &str) -> Write {
        Write {
            contents: WriteContents::from_str(v),
            font: Font::A,
        }
    }

    pub fn with_font(mut self, font: Font) -> Write {
        self.font = font;
        self
    }
}

impl Into<Effect> for Write {
    fn into(self) -> Effect {
        Effect::Write(self)
    }
}
