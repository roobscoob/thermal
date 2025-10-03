pub mod commands;
pub mod error;
pub mod state;

use winnow::{Parser, Partial, error::ContextError};

use crate::commands::{
    Command,
    reader::{commands::command, error::ErrorCtx, state::ParserState},
};

impl Command {
    pub fn parse<'i>(
        input: &mut Partial<&'i [u8]>,
        state: &mut ParserState,
    ) -> winnow::Result<Self, ContextError<ErrorCtx>> {
        command(&state).parse_next(input)
    }
}
