use itertools::chain;
use thermal::{
    commands::{reader::Output, Command},
    emulator::Emulator,
    state::{
        delta::Delta,
        effect::print::{Write, WriteContents},
        IntoState,
    },
    types::character_set::{AsciiVariant, Codepage},
};
use thermal_encoding::encoding::PartialUnicodeEncoding;

use crate::{
    device::TmT88v,
    emulation::{self, error::Error},
};

trait UnicodeIntoState<T>: PartialUnicodeEncoding<T> + IntoState {}

impl<V, T: PartialUnicodeEncoding<V> + IntoState> UnicodeIntoState<V> for T {}

const SUPPORTED_ASCII_VARIANTS: [&AsciiVariant; 18] = [
    &AsciiVariant::Usa,
    &AsciiVariant::France,
    &AsciiVariant::Germany,
    &AsciiVariant::Uk,
    &AsciiVariant::Denmark1,
    &AsciiVariant::Sweden,
    &AsciiVariant::Italy,
    &AsciiVariant::Spain1,
    &AsciiVariant::Japan,
    &AsciiVariant::Norway,
    &AsciiVariant::Denmark2,
    &AsciiVariant::Spain2,
    &AsciiVariant::LatinAmerica,
    &AsciiVariant::Korea,
    &AsciiVariant::SloveniaCroatia,
    &AsciiVariant::China,
    &AsciiVariant::Vietnam,
    &AsciiVariant::Arabia,
];

const SUPPORTED_CODEPAGES: [&Codepage; 6] = [
    &Codepage::Page0_Pc437,
    &Codepage::Page1_Katakana,
    &Codepage::Page2_Pc850,
    &Codepage::Page3_Pc860,
    &Codepage::Page4_Pc863,
    &Codepage::Page5_Pc865,
];

impl TmT88v {
    pub(super) fn apply_write(&mut self, write: Write) -> Result<Vec<Output>, Error> {
        let mut commands = vec![];

        match write.contents {
            WriteContents::Utf8(string) => {
                let v = self.state.ascii_variant();
                let c = self.state.codepage();

                return Ok(thermal_encoding::encode_str(
                    &string,
                    chain!(
                        v.iter().map(|v| v as &dyn UnicodeIntoState<
                            Result<Vec<Output>, emulation::error::Error>,
                        >),
                        c.iter().map(|v| v as &dyn UnicodeIntoState<
                            Result<Vec<Output>, emulation::error::Error>,
                        >),
                        SUPPORTED_ASCII_VARIANTS.iter().copied().map(|v| v
                            as &dyn UnicodeIntoState<Result<Vec<Output>, emulation::error::Error>>),
                        SUPPORTED_CODEPAGES
                            .iter()
                            .copied()
                            .map(|v| v as &dyn UnicodeIntoState<
                                Result<Vec<Output>, emulation::error::Error>,
                            >),
                    ),
                    |a, b| {
                        let delta = self.state.delta(b.into_state());

                        self.apply(delta).map(|mut v| {
                            v.extend(a.into_iter().map(|v| Output::Raw(*v)));
                            v
                        })
                    },
                )
                .map(|rr| rr.map_err(|c| Error::Unencodable(c)).and_then(|r| r))
                .collect::<Result<Vec<Vec<Output>>, Error>>()?
                .into_iter()
                .flatten()
                .collect::<Vec<Output>>());
            }

            WriteContents::AsciiLike(data, variant, codepage) => {
                let variant_used = data.iter().any(|v| *v < 128);
                let codepage_used = data.iter().any(|v| *v >= 128);

                let mut delta = Delta::empty();

                delta.apply_ascii_variant = variant_used.then_some(variant);
                delta.apply_codepage = codepage_used.then_some(codepage);

                commands.extend_from_slice(&self.apply(delta)?);

                for byte in data {
                    commands.push(Output::Raw(byte));
                }
            }
        }

        Ok(commands)
    }
}
