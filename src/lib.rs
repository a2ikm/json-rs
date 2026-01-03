use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Error {
    UnexpectedToken(String),
    UnexpectedEOF,
    InvalidLiteral(String),
    InvalidEscapeSequence(String),
    InvalidCodePoint(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::UnexpectedToken(string) => write!(f, "unexpected token: {}", string),
            Error::UnexpectedEOF => write!(f, "unexpected EOF"),
            Error::InvalidLiteral(string) => write!(f, "invalid literal: {}", string),
            Error::InvalidEscapeSequence(string) => {
                write!(f, "invalid escape sequence: \\{}", string)
            }
            Error::InvalidCodePoint(string) => write!(f, "invalid code point: {}", string),
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;

mod parse;
mod tokenize;

pub use parse::parse;
