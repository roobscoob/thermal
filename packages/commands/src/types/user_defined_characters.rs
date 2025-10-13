use facet::Facet;
use winnow::{
    Parser, Partial,
    binary::u8,
    error::{AddContext, ContextError, ErrMode, FromExternalError, ParserError},
    token::take,
};

use crate::commands::reader::error::{ErrorCtx, ErrorExpected};

#[derive(Clone, Facet)]
pub struct UserDefinedCharacter {
    character: u8,
    character_width: u8,
    canvas_height: u16, // total pixel rows (y * 8)
    /// Row-major bitmap, length == (canvas_height as usize) * (character_width as usize).
    /// Each entry is 0 (off) or 1 (on).
    /// TODO: Replace with some bit-vector
    canvas: Vec<u8>,
}

impl<'i> UserDefinedCharacter {
    pub fn sequence_parser()
    -> impl Parser<Partial<&'i [u8]>, Vec<UserDefinedCharacter>, ErrMode<ContextError<ErrorCtx>>> {
        move |input: &mut Partial<&'i [u8]>| {
            let y = u8.parse_next(input)?;
            let c1 = u8.parse_next(input)?;
            let c2 = u8.parse_next(input)?;
            let x = u8.parse_next(input)?;

            // TODO: Error handling
            // Fail when:
            //   y == 0 || x == 0
            //   c2 < c1
            //   c1 < 32
            //   c2 > 126

            let k = (c2 - c1) + 1;
            let bpc = (y as u16) * (x as u16); // bytes per character (column-major, y bytes per column)
            let len = ((bpc as u32) * (k as u32)) as usize;

            let data = take(len).parse_next(input)?;
            let x_usize = x as usize;
            let y_usize = y as usize;
            let bpc_usize = bpc as usize;
            let canvas_height: u16 = (y as u16) * 8;
            let canvas_h_usize = canvas_height as usize;

            let mut out: Vec<UserDefinedCharacter> = Vec::with_capacity(k as usize);

            for (idx, ch) in (c1..=c2).enumerate() {
                let base = idx * bpc_usize;
                let slice = &data[base..base + bpc_usize];

                // Row-major canvas: rows = canvas_h_usize, cols = x_usize
                let mut canvas = vec![0u8; canvas_h_usize * x_usize];

                // Input is column-major: for each column (0..x), there are `y` bytes top→down.
                for col in 0..x_usize {
                    for by in 0..y_usize {
                        let b = slice[col * y_usize + by];
                        // MSB..LSB map to top..bottom rows within this 8-dot block
                        for bit in 0..8 {
                            let row = by * 8 + bit;
                            if row >= canvas_h_usize {
                                break;
                            }
                            let on = (b >> (7 - bit)) & 1 == 1;
                            if on {
                                let idx_rm = row * x_usize + col; // row-major index
                                canvas[idx_rm] = 1;
                            }
                        }
                    }
                }

                out.push(UserDefinedCharacter {
                    character: ch,
                    character_width: x,
                    canvas_height,
                    canvas,
                });
            }

            Ok(out)
        }
    }
}
