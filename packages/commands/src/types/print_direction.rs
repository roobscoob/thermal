use facet::Facet;
use strum::{Display, EnumIter, FromRepr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Facet)]
#[repr(u8)]
pub enum PrintDirection {
    LeftToRight,
    BottomToTop,
    RightToLeft,
    TopToBottom,
}

impl PrintDirection {
    pub fn from_bits(bits: u8) -> Option<PrintDirection> {
        Some(match bits {
            0 | b'0' => PrintDirection::LeftToRight,
            1 | b'1' => PrintDirection::BottomToTop,
            2 | b'2' => PrintDirection::RightToLeft,
            3 | b'3' => PrintDirection::TopToBottom,

            _ => return None,
        })
    }
}
