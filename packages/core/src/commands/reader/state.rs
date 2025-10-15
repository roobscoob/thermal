use strum::EnumIs;

#[derive(Default, EnumIs, Clone, Copy)]
pub enum Mode {
    #[default]
    Standard,
    Page,
}

pub trait ParserState {
    fn mode(&self) -> Mode;
}
