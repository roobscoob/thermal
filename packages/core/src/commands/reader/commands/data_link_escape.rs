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

pub fn dle_command<'i>(
    state: &impl ParserState,
) -> impl Parser<Partial<&'i [u8]>, Command, ErrMode<ContextError<ErrorCtx>>> {
    dispatch!(take(1usize).map(|v: &[u8]| v[0]);
        0x04 => RequestedStatus::parser().map(|status| Command::RequestStatus(status)),
        0x05 => RealtimeRequest::parser().map(|status| Command::RealtimeRequest(status)),
        0x14 => dispatch!(take(1usize).map(|v: &[u8]| v[0]);
            0x01 => RealtimePulseInfo::parser().map(|pulse_info| Command::RealtimeGeneratePulse(pulse_info)),
            0x02 => (1, 8).map(|_| Command::ExecutePowerOffSequence),
            0x03 => fail.context(ErrorCtx::Unimplemented), // SoundBuzzerInRealTime
            0x07 => fail.context(ErrorCtx::Unimplemented), // TransmitSpecifiedStatusInRealTime
            0x08 => (1, 3, 20, 1, 6, 2, 8).map(|_| Command::ClearBuffer),

            _ => fail,
        ),

        _ => fail,
    )
}
