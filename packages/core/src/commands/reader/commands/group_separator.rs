use winnow::{
    Parser, Partial,
    binary::{le_i16, le_u8, le_u16, u8},
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
        cut_mode::{CutMode, CuttingShape},
        pulse_info::RealtimePulseInfo,
        realtime_request::RealtimeRequest,
        requested_status::RequestedStatus,
    },
};

pub fn gs_command<'i>(
    state: &impl ParserState,
) -> impl Parser<Partial<&'i [u8]>, Command, ErrMode<ContextError<ErrorCtx>>> {
    dispatch!(take(1usize).map(|v: &[u8]| v[0]);
        0x21 => u8.map(|n| {
            let height = (n & 0b0000_0111) + 1;
            let width  = ((n >> 4) & 0b0000_0111) + 1;
            Command::SelectCharacterSize(width, height)
        }),

        0x56 => dispatch!(take(1usize).map(|v: &[u8]| v[0]);
            0x00 | b'0' => empty.value(Command::SelectCutModeAndCutPaper(CutMode::Cut(CuttingShape::Full))),
            0x01 | b'1' => empty.value(Command::SelectCutModeAndCutPaper(CutMode::Cut(CuttingShape::Partial))),

            b'A' => u8.map(|v| Command::SelectCutModeAndCutPaper(CutMode::FeedAndCut(v, CuttingShape::Full))),
            b'B' => u8.map(|v| Command::SelectCutModeAndCutPaper(CutMode::FeedAndCut(v, CuttingShape::Partial))),

            b'a' => u8.map(|v| Command::SelectCutModeAndCutPaper(CutMode::SetCuttingPosition(v, CuttingShape::Full))),
            b'b' => u8.map(|v| Command::SelectCutModeAndCutPaper(CutMode::SetCuttingPosition(v, CuttingShape::Partial))),

            b'g' => u8.map(|v| Command::SelectCutModeAndCutPaper(CutMode::FeedAndCutAndMoveToStart(v, CuttingShape::Full))),
            b'h' => u8.map(|v| Command::SelectCutModeAndCutPaper(CutMode::FeedAndCutAndMoveToStart(v, CuttingShape::Partial))),

            _ => fail,
        ),

        _ => fail,
    )
}
