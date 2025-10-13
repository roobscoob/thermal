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
        reader::{error::ErrorCtx, state::ParserState},
    },
    types::{
        basic_styles::BasicStyles,
        batch_print::{BatchPrintDirection, BatchPrintMode},
        bit_image_band::BitImageBand,
        character_set::{CharacterCodeTable, CharacterTable},
        font::Font,
        justification::Justification,
        print_area::PrintArea,
        print_color::PrintColor,
        print_direction::PrintDirection,
        pulse_info::PulseConnector,
        user_defined_characters::UserDefinedCharacter,
    },
};

pub fn esc_command<'i>(
    state: &ParserState,
) -> impl Parser<Partial<&'i [u8]>, Command, ErrMode<ContextError<ErrorCtx>>> {
    dispatch!(take(1usize).map(|v: &[u8]| v[0]);
        0x0C => empty.value(Command::PrintDataInPageMode),
        0x20 => u8.map(|v| Command::SetRightSideCharacterSpacing(v)),
        0x21 => BasicStyles::parser().map(|basic_styles| Command::SelectPrintMode(basic_styles)),
        0x24 => le_u16.map(|v| Command::SetAbsolutePrintPosition(v)),
        0x25 => le_u8.map(|v| Command::SelectCancelUserDefinedCharacterSet(if (v & 0b1) == 1 { true } else { false })),
        0x26 => UserDefinedCharacter::sequence_parser().map(|seq| Command::DefineUserDefinedCharacters(seq)),
        0x28 => dispatch!(take(1usize).map(|v: &[u8]| v[0]);
            0x41 => fail.context(ErrorCtx::Unimplemented),
            0x59 => (2, 0, u8, u8).map(|v| Command::SpecifyBatchPrint(
                match v.2 {
                    0 | b'0' => BatchPrintMode::Disable,
                    1 | b'1' => BatchPrintMode::Enable,

                    _ => todo!(),
                },
                match v.3 {
                    0 | b'0' => BatchPrintDirection::Forward,
                    1 | b'1' => BatchPrintDirection::Reverse,

                    _ => todo!(),
                }
            )),

            _ => fail,
        ),
        0x2A => BitImageBand::parser().map(|band| Command::SelectBitImageMode(band)),
        0x2D => dispatch!(take(1usize).map(|v: &[u8]| v[0]);
            0x00 | b'0' => empty.value(Command::TurnUnderlineModeOnOff(0)),
            0x01 | b'1' => empty.value(Command::TurnUnderlineModeOnOff(1)),
            0x02 | b'2' => empty.value(Command::TurnUnderlineModeOnOff(1)),

            _ => fail,
        ),
        0x32 => empty.value(Command::SelectDefaultLineSpacing),
        0x33 => u8.map(|v| Command::SetLineSpacing(v)),
        0x3C => empty.value(Command::ReturnHome),
        0x3D => u8.map(|v| Command::SelectPeripheralDevice(v)),
        0x3F => u8.map(|v| Command::CancelUserDefinedCharacters(v)),
        0x40 => empty.value(Command::InitializePrinter),
        0x44 => fail.context(ErrorCtx::Unimplemented),
        0x45 => le_u8.map(|v| Command::TurnEmphasizedModeOnOff(if (v & 0b1) == 1 { true } else { false })),
        0x46 => le_u8.map(|v| Command::TurnDoubleStrikeModeOnOff(if (v & 0b1) == 1 { true } else { false })),
        0x4A => le_u8.map(|v| Command::PrintAndFeedPaper(v)),
        0x4B => empty.value(Command::SelectPageMode),

        // TODO: Fix unwrap here!
        0x4D => le_u8.map(|v| Command::SelectCharacterFont(Font::from_n(v).unwrap())),

        // TODO: Fix unwrap here!
        0x52 => le_u8.map(|v| Command::SelectInternationalCharacterSet(CharacterTable::from_repr(v).unwrap())),

        0x53 => empty.value(Command::SelectStandardMode),

        // TODO: Fix unwrap here!
        0x54 => le_u8.map(|v| Command::SelectPrintDirectionInPageMode(PrintDirection::from_bits(v).unwrap())),

        0x45 => le_u8.map(|v| Command::Turn90ClockwiseRotationModeOnOff(match v {
            0 | b'0' => 0,
            1 | b'1' => 1,
            2 | b'2' => 2,

            _ => todo!()
        })),

        0x57 => PrintArea::parser().map(|v| Command::SetPrintAreaInPageMode(v)),
        0x58 => le_i16.map(|v| Command::SetRelativePrintPosition(v)),

        // TODO: Fix unwrap here!
        0x61 => le_u8.map(|v| Command::SelectJustification(Justification::from_bits(v).unwrap())),

        0x63 => dispatch!(take(1usize).map(|v: &[u8]| v[0]);
            b'3' => le_u8.map(|v| Command::SelectPaperSensorToOutputPaperEndSignals(v)),
            b'4' => le_u8.map(|v| Command::SelectPaperSensorToStopPrinting(v)),
            b'5' => le_u8.map(|v| Command::EnableDisablePanelButtons(if (v & 0b1) == 1 { true } else { false })),

            _ => fail,
        ),

        0x64 => le_u8.map(|v| Command::PrintAndFeedNLines(v)),
        0x64 => le_u8.map(|v| Command::PrintAndReverseFeedNLines(v)),
        0x69 => empty.map(|v| Command::PartialCutOne),
        0x6D => empty.map(|v| Command::PartialCutThree),

        0x70 => dispatch!(take(1usize).map(|v: &[u8]| v[0]);
            0x00 | b'0' => (u8, u8).map(|(t1, t2)| Command::GeneratePulse(PulseConnector::Pin2, t1, t2)),
            0x01 | b'1' => (u8, u8).map(|(t1, t2)| Command::GeneratePulse(PulseConnector::Pin5, t1, t2)),

            _ => fail,
        ),

        // TODO: Fix unwrap here!
        0x72 => le_u8.map(|v| Command::SelectPrintColor(PrintColor::from_bits(v).unwrap())),

        // TODO: Fix unwrap here!
        0x74 => le_u8.map(|v| Command::SelectCharacterCodeTable(CharacterCodeTable::from_repr(v).unwrap())),

        0x75 => dispatch!(take(1usize).map(|v: &[u8]| v[0]);
            0x00 | b'0' => empty.value(Command::TransmitPeripheralDeviceStatus),

            _ => fail,
        ),

        0x76 => le_u8.map(|v| Command::TurnUpsideDownPrintModeOnOff(if (v & 0b1) == 1 { true } else { false })),

        _ => fail,
    )
}
