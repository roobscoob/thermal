use facet::Facet;
use winnow::{
    Parser, Partial,
    error::{AddContext, ContextError, ErrMode, FromExternalError},
    token::{any, take},
};

use crate::commands::reader::error::{ErrorCtx, ErrorExpected};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Facet)]
#[repr(C)]
pub enum BitImageMode {
    /// 8-dot single-density
    Single8,
    /// 8-dot double-density
    Double8,
    /// 24-dot single-density
    Single24,
    /// 24-dot double-density
    Double24,
    /// Unknown vendor or reserved mode
    Other(u8),
}

impl BitImageMode {
    pub fn from_byte(b: u8) -> Self {
        match b {
            0 => Self::Single8,
            1 => Self::Double8,
            32 => Self::Single24,
            33 => Self::Double24,
            x => Self::Other(x),
        }
    }

    pub fn bytes_per_col(self) -> usize {
        match self {
            Self::Single8 | Self::Double8 => 1,
            Self::Single24 | Self::Double24 => 3,
            Self::Other(x) if (x & 0x20) == 0 => 1,
            Self::Other(_) => 3,
        }
    }

    pub fn height_rows(self) -> u16 {
        match self {
            Self::Single8 | Self::Double8 => 8,
            Self::Single24 | Self::Double24 => 24,
            Self::Other(x) if (x & 0x20) == 0 => 8,
            Self::Other(_) => 24,
        }
    }
}

#[derive(Clone, Facet, Debug)]
pub struct BitImageBand {
    mode: BitImageMode,
    width_cols: u16,
    height_rows: u16,
    /// Row-major canvas: height_rows * width_cols entries of 0/1
    canvas: Vec<u8>,
}

impl<'i> BitImageBand {
    pub fn parser() -> impl Parser<Partial<&'i [u8]>, BitImageBand, ErrMode<ContextError<ErrorCtx>>>
    {
        move |input: &mut Partial<&'i [u8]>| {
            // Mode and width bytes
            let m_byte = any.parse_next(input)?;
            let nL = any.parse_next(input)?;
            let nH = any.parse_next(input)?;

            let mode = BitImageMode::from_byte(m_byte);
            let width_cols: u16 = (nL as u16) | ((nH as u16) << 8);

            // TODO:
            // fail if:
            //   width_cols == 0

            let bpc = mode.bytes_per_col();
            let k = (width_cols as usize) * bpc;

            // Payload bytes
            let data = take(k).parse_next(input)?;

            let height_rows = mode.height_rows();
            let w = width_cols as usize;
            let h = height_rows as usize;
            let mut canvas = vec![0u8; w * h];

            match bpc {
                1 => {
                    for x in 0..w {
                        let b = data[x];
                        for bit in 0..8 {
                            let on = (b >> (7 - bit)) & 1 == 1;
                            if on {
                                canvas[bit * w + x] = 1;
                            }
                        }
                    }
                }
                3 => {
                    for x in 0..w {
                        let i = x * 3;
                        let (b0, b1, b2) = (data[i], data[i + 1], data[i + 2]);
                        for bit in 0..8 {
                            if ((b0 >> (7 - bit)) & 1) == 1 {
                                canvas[(0 + bit) * w + x] = 1;
                            }
                            if ((b1 >> (7 - bit)) & 1) == 1 {
                                canvas[(8 + bit) * w + x] = 1;
                            }
                            if ((b2 >> (7 - bit)) & 1) == 1 {
                                canvas[(16 + bit) * w + x] = 1;
                            }
                        }
                    }
                }
                _ => unreachable!(),
            }

            Ok(BitImageBand {
                mode,
                width_cols,
                height_rows,
                canvas,
            })
        }
    }
}
