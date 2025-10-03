use strum::EnumIs;

#[derive(Default, EnumIs, Clone, Copy)]
pub enum Mode {
    #[default]
    Standard,
    Page,
}

#[derive(Default, Clone, Copy)]
pub struct ParserState {
    mode: Mode,
}

impl ParserState {
    pub fn mode(&self) -> Mode {
        self.mode
    }
}
