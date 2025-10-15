pub mod commands;
pub mod error;
pub mod state;

use facet::Facet;
use winnow::{
    Parser, Partial,
    error::{ContextError, ErrMode},
};

use crate::commands::{
    Command,
    reader::{commands::command, error::ErrorCtx, state::ParserState},
};

#[derive(Clone, Facet)]
#[repr(C)]
pub enum Output {
    Command(Command),
    Raw(u8),
}

impl Command {
    pub fn parse<'i>(
        input: &mut Partial<&'i [u8]>,
        state: &impl ParserState,
    ) -> winnow::Result<Output, ErrMode<ContextError<ErrorCtx>>> {
        command(state).parse_next(input)
    }
}
