use std::io::Write;

use strum::EnumMessage;

use crate::{
    commands::Command,
    types::{
        batch_print::{BatchPrintDirection, BatchPrintMode},
        cut_mode::{CutMode, CuttingShape},
        pulse_info::PulseConnector,
    },
};

impl Command {
    pub fn write_to<W: Write>(&self, w: &mut W) -> Result<usize, std::io::Error> {
        match self {
            // Plain
            Self::HorizontalTab => w.write(&[b'\t']),
            Self::LineFeed => w.write(&[b'\n']),
            Self::CarriageReturn => w.write(&[b'\r']),

            Self::EndJob => w.write(&[0x0C]),
            Self::EndPage => w.write(&[0x0C]),
            Self::CancelPrintDataInPageMode => w.write(&[0x18]),

            // ESC
            Self::PrintDataInPageMode => w.write(&[0x1B, 0x0C]),
            Self::SetRightSideCharacterSpacing(s) => w.write(&[0x1B, 0x20, *s]),
            Self::SelectPrintMode(m) => w.write(&[0x1B, 0x21, m.into_bits()]),
            Self::SetAbsolutePrintPosition(pos) => {
                w.write(&[[0x1B, 0x24], pos.to_le_bytes()].concat())
            }
            Self::SelectCancelUserDefinedCharacterSet(b) => {
                w.write(&[0x1B, 0x25, if *b { 1 } else { 0 }])
            }
            Self::DefineUserDefinedCharacters(c) => todo!(),
            Self::SpecifyBatchPrint(m, d) => w.write(&[
                0x1B,
                0x28,
                0x59,
                0x02,
                0x00,
                match m {
                    BatchPrintMode::Disable => 0,
                    BatchPrintMode::Enable => 1,
                },
                match d {
                    BatchPrintDirection::Forward => 0,
                    BatchPrintDirection::Reverse => 1,
                },
            ]),
            Self::SelectBitImageMode(band) => todo!(),
            Self::TurnUnderlineModeOnOff(v) => w.write(&[0x1B, 0x2D, *v]),
            Self::SelectDefaultLineSpacing => w.write(&[0x1B, 0x32]),
            Self::SetLineSpacing(v) => w.write(&[0x1B, 0x33, *v]),
            Self::ReturnHome => w.write(&[0x1B, 0x3C]),
            Self::SelectPeripheralDevice(v) => w.write(&[0x1B, 0x3D]),
            Self::CancelUserDefinedCharacters(c) => w.write(&[0x1B, 0x3F, *c]),
            Self::InitializePrinter => w.write(&[0x1B, 0x40]),
            Self::TurnEmphasizedModeOnOff(b) => w.write(&[0x1B, 0x45, if *b { 1 } else { 0 }]),
            Self::TurnDoubleStrikeModeOnOff(b) => w.write(&[0x1B, 0x46, if *b { 1 } else { 0 }]),
            Self::PrintAndFeedPaper(v) => w.write(&[0x1B, 0x41, *v]),
            Self::SelectPageMode => w.write(&[0x1B, 0x4B]),
            Self::SelectCharacterFont(f) => w.write(&[0x1B, 0x4D, *f as u8]),
            Self::SelectInternationalCharacterSet(f) => w.write(&[0x1B, 0x52, *f as u8]),
            Self::SelectStandardMode => w.write(&[0x1B, 0x53]),
            Self::SelectPrintDirectionInPageMode(d) => w.write(&[0x1B, 0x54, *d as u8]),
            Self::Turn90ClockwiseRotationModeOnOff(v) => w.write(&[0x1B, 0x56, *v]),
            Self::SetPrintAreaInPageMode(m) => todo!(),
            Self::SetRelativePrintPosition(i) => w.write(&[[0x1B, 0x58], i.to_le_bytes()].concat()),
            Self::SelectJustification(j) => w.write(&[0x1B, 0x61, *j as u8]),
            Self::SelectPaperSensorToOutputPaperEndSignals(v) => w.write(&[0x1B, 0x63, b'3', *v]),
            Self::SelectPaperSensorToStopPrinting(v) => w.write(&[0x1B, 0x63, b'4', *v]),
            Self::EnableDisablePanelButtons(b) => {
                w.write(&[0x1B, 0x63, b'5', if *b { 1 } else { 0 }])
            }
            Self::PrintAndFeedNLines(v) => w.write(&[0x1B, 0x64, *v]),
            Self::PrintAndReverseFeedNLines(v) => w.write(&[0x1B, 0x65, *v]),
            Self::PartialCutOne => w.write(&[0x1B, 0x69]),
            Self::PartialCutThree => w.write(&[0x1B, 0x6D]),
            Self::GeneratePulse(c, t1, t2) => w.write(&[
                0x1B,
                0x70,
                match c {
                    PulseConnector::Pin2 => 0,
                    PulseConnector::Pin5 => 1,
                },
                *t1,
                *t2,
            ]),
            Self::SelectPrintColor(c) => w.write(&[0x1B, 0x72, *c as u8]),
            Self::SelectCharacterCodeTable(t) => w.write(&[0x1B, 0x74, *t as u8]),
            Self::TransmitPeripheralDeviceStatus => w.write(&[0x1B, 0x75, 0x00]),
            Self::TurnUpsideDownPrintModeOnOff(b) => w.write(&[0x1B, 0x76, if *b { 1 } else { 0 }]),

            // GS (partial)
            Self::SelectCharacterSize(a, b) => w.write(&[0x1D, 0x21, ((a - 1) << 4) | (b - 1)]),
            Self::SelectCutModeAndCutPaper(mode) => {
                match mode {
                    CutMode::Cut(shape) => match shape {
                        CuttingShape::Full => w.write(&[0x1D, 0x56, 0x00]), // or b'0'
                        CuttingShape::Partial => w.write(&[0x1D, 0x56, 0x01]), // or b'1'
                    },

                    CutMode::FeedAndCut(n, shape) => {
                        let m = match shape {
                            CuttingShape::Full => b'A',
                            CuttingShape::Partial => b'B',
                        };
                        w.write(&[0x1D, 0x56, m, *n])
                    }

                    CutMode::SetCuttingPosition(n, shape) => {
                        let m = match shape {
                            CuttingShape::Full => b'a',
                            CuttingShape::Partial => b'b',
                        };
                        w.write(&[0x1D, 0x56, m, *n])
                    }

                    CutMode::FeedAndCutAndMoveToStart(n, shape) => {
                        let m = match shape {
                            CuttingShape::Full => b'g',
                            CuttingShape::Partial => b'h',
                        };
                        w.write(&[0x1D, 0x56, m, *n])
                    }
                }
            }

            c => unimplemented!("Command {:?} not implemented", c.get_message()),
        }
    }
}
