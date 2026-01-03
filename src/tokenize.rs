use std::fmt;
use std::iter::Peekable;
use std::str::Chars;

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

pub fn tokenize(source: &str) -> Result<Vec<Token>, String> {
    let mut chars = source.chars().peekable();
    let mut tokens = Vec::new();

    while let Some(&ch) = chars.peek() {
        match ch {
            ' ' | '\t' | '\n' | '\r' => {
                chars.next();
            }
            '[' => {
                tokens.push(Token::LeftSquareBracket);
                chars.next();
            }
            '{' => {
                tokens.push(Token::LeftCurlyBracket);
                chars.next();
            }
            ']' => {
                tokens.push(Token::RightSquareBracket);
                chars.next();
            }
            '}' => {
                tokens.push(Token::RightCurlyBracket);
                chars.next();
            }
            ':' => {
                tokens.push(Token::Colon);
                chars.next();
            }
            ',' => {
                tokens.push(Token::Comma);
                chars.next();
            }
            '-' | '0'..='9' => match tokenize_number(&mut chars) {
                Ok(token) => tokens.push(token),
                Err(e) => return Err(e),
            },
            '"' => match tokenize_string(&mut chars) {
                Ok(token) => tokens.push(token),
                Err(e) => return Err(e),
            },
            _ => match tokenize_literal(&mut chars) {
                Ok(token) => tokens.push(token),
                Err(e) => return Err(e),
            },
        }
    }

    Ok(tokens)
}

fn tokenize_number(chars: &mut Peekable<Chars>) -> Result<Token, String> {
    let mut number = String::new();

    while let Some(&ch) = chars.peek() {
        match ch {
            '-' | '+' | '0'..='9' | 'e' | 'E' | '.' => {
                number.push(ch);
                chars.next();
            }
            _ => break,
        }
    }

    match number.parse::<f64>() {
        Ok(value) => Ok(Token::Number(value)),
        Err(_) => Err(format!("invalid number representation: {}", number)),
    }
}

fn tokenize_string(chars: &mut Peekable<Chars>) -> Result<Token, String> {
    chars.next(); // opening quote

    let mut string = String::new();

    while let Some(ch) = chars.next() {
        match ch {
            '"' => break,
            '\\' => {
                let ch = read_escaped_char(chars)?;
                string.push(ch);
            }
            _ => {
                string.push(ch);
            }
        }
    }

    Ok(Token::String(string))
}

fn read_escaped_char(chars: &mut Peekable<Chars>) -> Result<char, String> {
    if let Some(ch) = chars.next() {
        match ch {
            '"' => Ok('\u{0022}'),
            '\\' => Ok('\u{005c}'),
            '/' => Ok('\u{002f}'),
            'b' => Ok('\u{0008}'),
            'f' => Ok('\u{000c}'),
            'n' => Ok('\u{000a}'),
            'r' => Ok('\u{000d}'),
            't' => Ok('\u{0009}'),
            'u' => {
                let ch = read_hex_digits_char(chars)?;
                Ok(ch)
            }
            _ => Err(format!("invalid escape sequence: {}", ch)),
        }
    } else {
        Err("unexpected EOF".to_string())
    }
}

fn read_hex_digits_char(chars: &mut Peekable<Chars>) -> Result<char, String> {
    let mut hex_digits = String::new();

    while let Some(&ch) = chars.peek() {
        if ch == '"' {
            // To break the outer while loop with `"`, we can't consume it.
            break;
        } else {
            hex_digits.push(ch);
            chars.next();
            if hex_digits.len() == 4 {
                break;
            }
        }
    }

    if hex_digits.len() != 4 {
        return Err(format!("invalid code point: {}", hex_digits));
    }

    let Ok(code_point) = u32::from_str_radix(&hex_digits, 16) else {
        return Err(format!("invalid code point: {}", hex_digits));
    };

    let Some(ch) = char::from_u32(code_point) else {
        return Err(format!("invalid code point: {}", hex_digits));
    };

    Ok(ch)
}

fn tokenize_literal(chars: &mut Peekable<Chars>) -> Result<Token, String> {
    let mut literal = String::new();

    while let Some(&ch) = chars.peek() {
        if ch.is_ascii_alphanumeric() {
            chars.next();
            literal.push(ch);
        } else {
            break;
        }
    }

    match literal.as_str() {
        "true" => Ok(Token::True),
        "false" => Ok(Token::False),
        "null" => Ok(Token::Null),
        _ => Err(format!("unexpected literal: {}", literal)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenize_whitespace() {
        assert_eq!(tokenize(" \t\r\n"), Ok(vec![]));
    }

    #[test]
    fn tokenize_left_square_bracket() {
        assert_eq!(tokenize("["), Ok(vec![Token::LeftSquareBracket]));
    }

    #[test]
    fn tokenize_left_curly_bracket() {
        assert_eq!(tokenize("{"), Ok(vec![Token::LeftCurlyBracket]));
    }

    #[test]
    fn tokenize_right_square_bracket() {
        assert_eq!(tokenize("]"), Ok(vec![Token::RightSquareBracket]));
    }

    #[test]
    fn tokenize_right_curly_bracket() {
        assert_eq!(tokenize("}"), Ok(vec![Token::RightCurlyBracket]));
    }

    #[test]
    fn tokenize_colon() {
        assert_eq!(tokenize(":"), Ok(vec![Token::Colon]));
    }

    #[test]
    fn tokenize_comma_bracket() {
        assert_eq!(tokenize(","), Ok(vec![Token::Comma]));
    }

    #[test]
    fn tokenize_number() {
        assert_eq!(tokenize("123.45e10"), Ok(vec![Token::Number(123.45e10)]),);
        assert_eq!(tokenize("-123.45e10"), Ok(vec![Token::Number(-123.45e10)]),);
    }

    #[test]
    fn tokenize_string() {
        assert_eq!(
            tokenize("\"foo\""),
            Ok(vec![Token::String("foo".to_string())])
        );
        assert_eq!(
            tokenize("\"\\\"\""),
            Ok(vec![Token::String("\u{0022}".to_string())])
        );
        assert_eq!(
            tokenize("\"\\\\\""),
            Ok(vec![Token::String("\u{005c}".to_string())])
        );
        assert_eq!(
            tokenize("\"\\/"),
            Ok(vec![Token::String("\u{002f}".to_string())])
        );
        assert_eq!(
            tokenize("\"\\b"),
            Ok(vec![Token::String("\u{0008}".to_string())])
        );
        assert_eq!(
            tokenize("\"\\f\""),
            Ok(vec![Token::String("\u{000c}".to_string())])
        );
        assert_eq!(
            tokenize("\"\\n\""),
            Ok(vec![Token::String("\u{000a}".to_string())])
        );
        assert_eq!(
            tokenize("\"\\r\""),
            Ok(vec![Token::String("\u{000d}".to_string())])
        );
        assert_eq!(
            tokenize("\"\\t\""),
            Ok(vec![Token::String("\u{0009}".to_string())])
        );
        assert_eq! {
            tokenize("\"\\u0041\""),
            Ok(vec![Token::String("\u{0041}".to_string())])
        }
    }

    #[test]
    fn tokenize_true() {
        assert_eq!(tokenize("true"), Ok(vec![Token::True]));
    }

    #[test]
    fn tokenize_false() {
        assert_eq!(tokenize("false"), Ok(vec![Token::False]));
    }

    #[test]
    fn tokenize_null() {
        assert_eq!(tokenize("null"), Ok(vec![Token::Null]));
    }
}
