use crate::{
    commands::{Command, reader::Output},
    state::delta::Delta,
};

pub trait Emulator {
    type Error;

    fn apply(&mut self, delta: Delta) -> Result<Vec<Output>, Self::Error>;
    fn write(&mut self, commands: Vec<Command>) -> Result<Delta, Self::Error>;
}
