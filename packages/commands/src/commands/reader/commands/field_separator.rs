use winnow::{
    Parser, Partial,
    combinator::{dispatch, empty, fail},
    error::{ContextError, ErrMode},
    token::take,
};

use crate::{
    commands::{
        Command,
        reader::{
            error::ErrorCtx,
            state::{Mode, ParserState},
        },
    },
    types::{
        pulse_info::RealtimePulseInfo, realtime_request::RealtimeRequest,
        requested_status::RequestedStatus,
    },
};

pub fn fs_command<'i>(
    state: &ParserState,
) -> impl Parser<Partial<&'i [u8]>, Command, ErrMode<ContextError<ErrorCtx>>> {
    dispatch!(take(1usize).map(|v: &[u8]| v[0]);
        0x2E => empty.value(Command::CancelKanjiCharacterMode),

        _ => fail,
    )
}
