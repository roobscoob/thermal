pub mod error;
pub mod write;

use thermal::{
    commands::{reader::Output, Command},
    emulator::Emulator,
    state::{delta::Delta, effect::Effect},
};

use crate::{device::TmT88v, emulation};

impl TmT88v {
    fn apply_single(&mut self, effect: Effect) -> Result<Vec<Output>, emulation::error::Error> {
        match effect {
            Effect::Write(write) => self.apply_write(write),
        }
    }
}

impl Emulator for TmT88v {
    type Error = emulation::error::Error;

    fn apply(&mut self, delta: Delta) -> Result<Vec<Output>, emulation::error::Error> {
        let mut collection = vec![];

        for effect in delta.iter() {
            collection.extend_from_slice(&self.apply_single(effect.clone())?);
        }

        if let Some(ref font) = delta.apply_font {
            self.state = self.state.clone().with_font(*font);
            collection.push(Output::Command(Command::SelectCharacterFont(*font)));
        }

        if let Some(ref ascii_variant) = delta.apply_ascii_variant {
            self.state = self.state.clone().with_ascii_variant(*ascii_variant);
            collection.push(Output::Command(Command::SelectInternationalCharacterSet(
                *ascii_variant,
            )))
        }

        if let Some(ref codepage) = delta.apply_codepage {
            self.state = self.state.clone().with_codepage(*codepage);
            collection.push(Output::Command(Command::SelectCharacterCodeTable(
                *codepage,
            )))
        }

        Ok(collection)
    }

    fn write(&mut self, commands: Vec<Command>) -> Result<Delta, emulation::error::Error> {
        todo!()
    }
}
