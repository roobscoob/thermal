use crate::state::effect::print::Write;

pub mod print;

/// Conceptually: Stateless commands
#[derive(Debug, Clone)]
pub enum Effect {
    Write(Write),
}
