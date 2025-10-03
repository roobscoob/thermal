use facet::Facet;
use winnow::{
    Parser, Partial,
    combinator::{alt, empty},
    error::ContextError,
};

use crate::commands::reader::error::ErrorCtx;

#[derive(Facet, Clone, Copy, Debug)]
#[repr(C)]
pub enum RealtimeRequest {
    /// Equivalent to pressing the FEED button during recovery-wait.
    ResumeFeed,

    /// Recover from a recoverable error and resume printing where it left off.
    Recover,

    /// Recover by clearing buffers (and in page mode, reset to standard mode).
    Reset,
}

impl RealtimeRequest {
    pub fn parser<'i>() -> impl Parser<Partial<&'i [u8]>, Self, ContextError<ErrorCtx>> {
        alt((
            0x00.value(RealtimeRequest::ResumeFeed),
            0x01.value(RealtimeRequest::Recover),
            0x02.value(RealtimeRequest::Reset),
        ))
    }
}
