use std::iter::once;

use crate::state::effect::{Effect, IntoEffects};

#[derive(Debug, Clone)]
pub struct Feed {
    pub line_count: usize,
}

impl Feed {
    pub fn lines(count: usize) -> Feed {
        Self { line_count: count }
    }
}

impl IntoEffects for Feed {
    fn as_effects(self) -> impl Iterator<Item = Effect> {
        once(Effect::Feed(self))
    }
}
