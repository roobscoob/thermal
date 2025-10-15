use facet::Facet;

use crate::state::{IntoState, State};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Facet)]
#[repr(u8)]
pub enum Font {
    A,
    B,
    C,
    D,
    E,
    SpecialA,
    SpecialB,
}

impl Font {
    /// Parse any valid `n` byte into a semantic font (across models).
    pub fn from_n(n: u8) -> Option<Self> {
        match n {
            0 | b'0' => Some(Font::A),
            1 | b'1' => Some(Font::B),
            2 | b'2' => Some(Font::C),
            3 | b'3' => Some(Font::D),
            4 | b'4' => Some(Font::E),
            b'a' => Some(Font::SpecialA),
            b'b' => Some(Font::SpecialB),
            _ => None,
        }
    }
}

impl IntoState for Font {
    fn into_state(&self) -> crate::state::State {
        State::default().with_font(*self)
    }
}
