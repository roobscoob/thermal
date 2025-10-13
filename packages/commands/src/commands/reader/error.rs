use winnow::{Partial, error::ParserError};

#[derive(Debug, Clone, Copy)]
pub enum ErrorLabel {
    Depth(u8),
}

#[derive(Debug, Clone, Copy)]
pub enum ExpectedValue {
    Byte(u8),
}

#[derive(Debug, Clone, Copy)]
pub enum ErrorExpected {
    OneOf(&'static [ExpectedValue]),
    Description(&'static str),
}

#[derive(Debug, Clone, Copy)]
pub enum ErrorCtx {
    Label(ErrorLabel),
    Expected(ErrorExpected),
    Unimplemented,
}
