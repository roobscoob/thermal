use miette::Diagnostic;
use strum::Display;

#[derive(thiserror::Error, Diagnostic, Debug, Display)]
pub enum Error {
    Unencodable(char),
}
