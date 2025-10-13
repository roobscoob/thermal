pub mod data_link_escape;
pub mod escape;
pub mod field_separator;
pub mod group_separator;

use winnow::{
    Parser, Partial,
    combinator::{dispatch, empty, fail},
    error::ContextError,
    token::take,
};

use crate::commands::{
    Command,
    reader::{
        Output,
        commands::{
            data_link_escape::dle_command, escape::esc_command, field_separator::fs_command,
            group_separator::gs_command,
        },
        error::ErrorCtx,
        state::{Mode, ParserState},
    },
};

pub fn command<'i>(
    state: &ParserState,
) -> impl Parser<Partial<&'i [u8]>, Output, winnow::error::ErrMode<ContextError<ErrorCtx>>> {
    dispatch!(take(1usize).map(|v: &[u8]| v[0]);
        b'\t' => empty.value(Output::Command(Command::HorizontalTab)),
        b'\n' => empty.value(Output::Command(Command::LineFeed)),
        b'\r' => empty.value(Output::Command(Command::CarriageReturn)),

        0x1B => esc_command(state).map(|v| Output::Command(v)),
        0x18 => empty.value(Output::Command(Command::CancelPrintDataInPageMode)),
        0x10 => dle_command(state).map(|v| Output::Command(v)),
        0x0C => empty.value(Output::Command(match state.mode() {
            Mode::Standard => Command::EndJob,
            Mode::Page => Command::EndPage,
        })),

        0x1C => fs_command(state).map(|v| Output::Command(v)),
        0x1D => gs_command(state).map(|v| Output::Command(v)),

        v => empty.value(Output::Raw(v)),
    )
}
