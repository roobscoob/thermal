use facet::Facet;
use winnow::{
    Parser, Partial,
    binary::le_u16,
    error::{ContextError, ErrMode},
};

use crate::commands::reader::error::ErrorCtx;

#[derive(Clone, Copy, Facet)]
pub struct PrintArea {
    pub x: u16,
    pub y: u16,
    pub dx: u16,
    pub dy: u16,
}

impl<'a> PrintArea {
    pub fn parser() -> impl Parser<Partial<&'a [u8]>, Self, ErrMode<ContextError<ErrorCtx>>> {
        (le_u16, le_u16, le_u16, le_u16).map(|(x, y, dx, dy)| Self { x, y, dx, dy })
    }
}
