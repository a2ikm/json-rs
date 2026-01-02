use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, PartialEq)]
pub enum Token {
    LeftSquareBracket,  // [
    LeftCurlyBracket,   // (
    RightSquareBracket, // ]
    RightCurlyBracket,  // )
    Colon,              // :
    Comma,              // ,
    String(String),
    Number(f64),
    True,
    False,
    Null,
}

pub fn tokenize(source: &str) -> Result<Vec<Token>, String> {
    let mut chars = source.chars().peekable();
    let mut tokens = Vec::new();

    while let Some(&ch) = chars.peek() {
        match ch {
            '[' => {
                tokens.push(Token::LeftSquareBracket);
                chars.next();
            }
            '(' => {
                tokens.push(Token::LeftCurlyBracket);
                chars.next();
            }
            ']' => {
                tokens.push(Token::RightSquareBracket);
                chars.next();
            }
            ')' => {
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
            '\\' => match chars.next() {
                Some('"') => string.push('\u{0022}'),
                Some('\\') => string.push('\u{005c}'),
                Some('/') => string.push('\u{002f}'),
                Some('b') => string.push('\u{0008}'),
                Some('f') => string.push('\u{000c}'),
                Some('n') => string.push('\u{000a}'),
                Some('r') => string.push('\u{000d}'),
                Some('t') => string.push('\u{0009}'),
                Some('u') => {
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

                    string.push(ch);
                }
                Some(ch) => return Err(format!("invalid escape sequence: {}", ch)),
                None => return Err("unexpected EOF".to_string()),
            },
            _ => {
                string.push(ch);
            }
        }
    }

    Ok(Token::String(string))
}

fn tokenize_literal(chars: &mut Peekable<Chars>) -> Result<Token, String> {
    let mut literal = String::new();

    while let Some(&ch) = chars.peek() {
        match ch {
            ',' | '[' | ']' | '{' | '}' | ':' | '"' | '\r' | '\n' | '\t' | ' ' => break,
            _ => {
                chars.next();
                literal.push(ch);
            }
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
    fn tokenize_left_square_bracket() {
        assert_eq!(tokenize("["), Ok(vec![Token::LeftSquareBracket]));
    }

    #[test]
    fn tokenize_left_curly_bracket() {
        assert_eq!(tokenize("("), Ok(vec![Token::LeftCurlyBracket]));
    }

    #[test]
    fn tokenize_right_square_bracket() {
        assert_eq!(tokenize("]"), Ok(vec![Token::RightSquareBracket]));
    }

    #[test]
    fn tokenize_right_curly_bracket() {
        assert_eq!(tokenize(")"), Ok(vec![Token::RightCurlyBracket]));
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
