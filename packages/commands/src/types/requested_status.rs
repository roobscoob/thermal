use facet::Facet;
use winnow::{
    Parser, Partial,
    binary::u8,
    combinator::{dispatch, empty},
    error::{ContextError, ErrMode},
};

use crate::commands::reader::error::ErrorCtx;

#[derive(Facet, Clone, Copy, Debug)]
#[repr(C)]
pub enum RequestedStatus {
    Printer,
    OfflineCause,
    ErrorCause,
    RollPaperSensor,
    Ink { channel: u8 },
    Peeler,
    Interface,
    DisplayModule,
}

impl RequestedStatus {
    pub fn parser<'i>() -> impl Parser<Partial<&'i [u8]>, Self, ErrMode<ContextError<ErrorCtx>>> {
        dispatch! {u8; // read the first tag byte
            0x01 => empty.value(RequestedStatus::Printer),
            0x02 => empty.value(RequestedStatus::OfflineCause),
            0x03 => empty.value(RequestedStatus::ErrorCause),
            0x04 => empty.value(RequestedStatus::RollPaperSensor),

            // needs one more byte (channel)
            0x07 => u8.map(|channel| RequestedStatus::Ink { channel }),

            // needs one more fixed byte (0x03)
            0x08 => 0x03u8.value(RequestedStatus::Peeler),

            // needs one more selector byte
            0x12 => dispatch! {u8;
                0x01 => empty.value(RequestedStatus::Interface),
                0x03 => empty.value(RequestedStatus::DisplayModule),
                _    => winnow::combinator::fail, // unknown subcode
            },

            _ => winnow::combinator::fail, // unknown top-level tag
        }
    }
}
