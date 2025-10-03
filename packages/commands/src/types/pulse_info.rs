use facet::Facet;
use winnow::{
    Parser, Partial,
    combinator::{dispatch, empty, fail},
    error::ContextError,
    token::take,
};

use crate::commands::reader::error::ErrorCtx;

#[derive(Facet, Clone, Copy)]
#[repr(u8)]
pub enum PulseConnector {
    Pin2 = 0,
    Pin5 = 1,
}

#[derive(Facet, Clone, Copy)]
pub struct RealtimePulseInfo(PulseConnector, u8);

impl RealtimePulseInfo {
    pub fn parser<'i>() -> impl Parser<Partial<&'i [u8]>, Self, ContextError<ErrorCtx>> {
        dispatch!(take(2usize).map(|v: &[u8]| (v[0], v[1]));
            (0, v @ 1..8) => empty.value(RealtimePulseInfo(PulseConnector::Pin2, v)),
            (1, v @ 1..8) => empty.value(RealtimePulseInfo(PulseConnector::Pin5, v)),
            _ => fail,
        )
    }
}
