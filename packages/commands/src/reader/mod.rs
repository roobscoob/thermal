use crate::reader::state::{parser::ParserState, printer::PrinterState};

pub mod state;

#[derive(Debug, Clone, Copy, Default)]
pub struct CommandReader {
    parser: ParserState,
    printer: PrinterState,
}

impl CommandReader {
    pub fn read<'a, 'command_data>(&'a mut self, data: &'a mut &'command_data [u8]) {}
}
