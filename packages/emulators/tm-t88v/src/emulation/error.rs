use miette::Diagnostic;
use strum::Display;
use thermal::types::font::Font;

#[derive(thiserror::Error, Diagnostic, Debug, Display)]
pub enum Error {
    Unencodable(char),
    UnsupportedFont(Font),
}
