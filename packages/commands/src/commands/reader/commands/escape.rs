use winnow::{
    Parser, Partial,
    binary::{le_u16, u8},
    combinator::{dispatch, empty, fail},
    error::ContextError,
    token::take,
};

use crate::{
    commands::{
        Command,
        reader::{error::ErrorCtx, state::ParserState},
    },
    types::basic_styles::BasicStyles,
};

pub fn esc_command<'i>(
    state: &ParserState,
) -> impl Parser<Partial<&'i [u8]>, Command, ContextError<ErrorCtx>> {
    dispatch!(take(1usize).map(|v: &[u8]| v[0]);
        0x0C => empty.value(Command::PrintDataInPageMode),
        0x20 => u8.map(|v| Command::SetRightSideCharacterSpacing(v)),
        0x21 => BasicStyles::parser().map(|basic_styles| Command::SelectPrintMode(basic_styles)),
        0x24 => le_u16.map(|v| Command::SetAbsolutePrintPosition(v)),
        0x25 => fail.context(ErrorCtx::Unimplemented),

        _ => fail,
    )
}
