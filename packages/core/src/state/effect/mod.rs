use crate::state::effect::{cut::Cut, feed::Feed, print::Write};

pub mod cut;
pub mod feed;
pub mod print;

/// Conceptually: Stateless commands
#[derive(Debug, Clone)]
pub enum Effect {
    Write(Write),
    Feed(Feed),
    Cut(Cut),
}

pub trait IntoEffects {
    fn as_effects(self) -> impl Iterator<Item = Effect>;
}
