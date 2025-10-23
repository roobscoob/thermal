use thermal::{
    commands::{reader::Output, Command},
    state::effect::{cut::Cut, feed::Feed},
};

use crate::{device::TmT88v, emulation};

impl TmT88v {
    pub(super) fn apply_feed(
        &mut self,
        feed: Feed,
    ) -> Result<Vec<Output>, emulation::error::Error> {
        let mut output = vec![];

        for _ in 0..feed.line_count {
            output.push(Output::Command(Command::LineFeed))
        }

        Ok(output)
    }

    pub(super) fn apply_cut(&mut self, cut: Cut) -> Result<Vec<Output>, emulation::error::Error> {
        todo!()
    }

    pub(super) fn apply_feed_and_cut(
        &mut self,
        feed: Feed,
        cut: Cut,
    ) -> Result<Vec<Output>, emulation::error::Error> {
        todo!()
    }
}
