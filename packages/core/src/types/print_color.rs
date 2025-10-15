use facet::Facet;
use strum::{Display, EnumIter, FromRepr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Facet)]
#[repr(u8)]
pub enum PrintColor {
    Black,
    Red,
}

impl PrintColor {
    pub fn from_bits(bits: u8) -> Option<PrintColor> {
        Some(match bits {
            0 | b'0' => PrintColor::Black,
            1 | b'1' => PrintColor::Red,

            _ => return None,
        })
    }
}
