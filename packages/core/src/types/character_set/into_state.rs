use crate::{
    state::{IntoState, State},
    types::character_set::{AsciiVariant, Codepage},
};

impl IntoState for AsciiVariant {
    fn into_state(&self) -> crate::state::State {
        State::default().with_ascii_variant(*self)
    }
}

impl IntoState for Codepage {
    fn into_state(&self) -> State {
        State::default().with_codepage(*self)
    }
}
