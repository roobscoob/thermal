use facet::Facet;
use strum::{Display, EnumIter, FromRepr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Facet)]
#[repr(u8)]
pub enum Justification {
    LeftJustified,
    Centered,
    RightJustified,
}

impl Justification {
    pub fn from_bits(bits: u8) -> Option<Justification> {
        Some(match bits {
            0 | b'0' => Justification::LeftJustified,
            1 | b'1' => Justification::Centered,
            2 | b'2' => Justification::RightJustified,

            _ => return None,
        })
    }
}
