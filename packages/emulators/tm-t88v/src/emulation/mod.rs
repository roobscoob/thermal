pub mod error;
pub mod feed_and_cut;
pub mod write;

use facet_pretty::FacetPretty;
use thermal::{
    commands::{reader::Output, Command},
    emulator::Emulator,
    state::{
        delta::Delta,
        effect::{self, Effect},
    },
    types::font::Font,
};

use crate::{device::TmT88v, emulation};

impl TmT88v {
    fn apply_single(&mut self, effect: Effect) -> Result<Vec<Output>, emulation::error::Error> {
        match effect {
            Effect::Write(write) => self.apply_write(write),
            Effect::Feed(feed) => self.apply_feed(feed),
            Effect::Cut(cut) => self.apply_cut(cut),
        }
    }
}

impl Emulator for TmT88v {
    type Error = emulation::error::Error;

    fn apply(&mut self, delta: Delta) -> Result<Vec<Output>, emulation::error::Error> {
        let mut collection = vec![];

        let mut iter = delta.iter().peekable();

        while let Some(effect) = iter.next() {
            if let (Effect::Feed(feed), Some(Effect::Cut(cut))) = (effect, iter.peek()) {
                iter.next();

                collection.extend_from_slice(&self.apply_feed_and_cut(feed.clone(), cut.clone())?);

                continue;
            }

            collection.extend_from_slice(&self.apply_single(effect.clone())?);
        }

        if let Some(font) = delta.apply_font {
            if font != Font::A && font != Font::B {
                return Err(emulation::error::Error::UnsupportedFont(font));
            }

            self.state = self.state.clone().with_font(font);
            collection.push(Output::Command(Command::SelectCharacterFont(font)));
        }

        if let Some(justification) = delta.apply_justification {
            self.state = self.state.clone().with_justification(justification);
            collection.push(Output::Command(Command::SelectJustification(justification)))
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

        if let Some(ref scale) = delta.apply_text_scale {
            self.state = self.state.clone().with_text_scale(*scale);
            collection.push(Output::Command(Command::SelectCharacterSize(
                scale.0, scale.1,
            )))
        }

        Ok(collection)
    }

    fn write(&mut self, commands: Vec<Command>) -> Result<Delta, emulation::error::Error> {
        todo!()
    }
}
