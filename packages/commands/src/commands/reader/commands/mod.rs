pub mod data_link_escape;
pub mod escape;

use winnow::{
    Parser, Partial,
    combinator::{dispatch, empty, fail},
    token::take,
};

use crate::commands::{
    Command,
    reader::{
        commands::{data_link_escape::dle_command, escape::esc_command},
        error::ErrorCtx,
        state::{Mode, ParserState},
    },
};

pub fn command<'i>(
    state: &ParserState,
) -> impl Parser<Partial<&'i [u8]>, Command, winnow::error::ContextError<ErrorCtx>> {
    dispatch!(take(1usize).map(|v: &[u8]| v[0]);
        b'\t' => empty.value(Command::HorizontalTab),
        b'\n' => empty.value(Command::LineFeed),
        b'\r' => empty.value(Command::CarriageReturn),

        0x1B => esc_command(state),
        0x18 => empty.value(Command::CancelPrintDataInPageMode),
        0x10 => dle_command(state),
        0x0C => empty.value(match state.mode() {
            Mode::Standard => Command::EndJob,
            Mode::Page => Command::EndPage,
        }),

        _ => fail,
    )
}
