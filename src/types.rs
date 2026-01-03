use std::collections::HashMap;
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

#[derive(Debug, PartialEq)]
pub enum Value {
    Object(HashMap<String, Value>),
    Array(Vec<Value>),
    String(String),
    Number(f64),
    Bool(bool),
    Null,
}

#[derive(Debug, PartialEq)]
pub enum Token {
    LeftSquareBracket,  // [
    LeftCurlyBracket,   // {
    RightSquareBracket, // ]
    RightCurlyBracket,  // }
    Colon,              // :
    Comma,              // ,
    String(String),
    Number(f64),
    True,
    False,
    Null,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::LeftSquareBracket => write!(f, "["),
            Token::LeftCurlyBracket => write!(f, "{{"),
            Token::RightSquareBracket => write!(f, "]"),
            Token::RightCurlyBracket => write!(f, "}}"),
            Token::Colon => write!(f, ":"),
            Token::Comma => write!(f, ","),
            Token::String(string) => write!(f, "{:?}", *string),
            Token::Number(number) => write!(f, "{:?}", *number),
            Token::True => write!(f, "true"),
            Token::False => write!(f, "false"),
            Token::Null => write!(f, "null"),
        }
    }
}
