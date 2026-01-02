use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, PartialEq)]
pub enum Token {
    True,
    False,
    Null,
}

pub fn tokenize(source: &str) -> Result<Vec<Token>, String> {
    let mut chars = source.chars().peekable();
    let mut tokens = Vec::new();

    while let Some(&char) = chars.peek() {
        match char {
            _ => match tokenize_literal(&mut chars) {
                Ok(token) => tokens.push(token),
                Err(e) => return Err(e),
            },
        }
    }

    Ok(tokens)
}

fn tokenize_literal(chars: &mut Peekable<Chars>) -> Result<Token, String> {
    let mut literal = String::new();

    while let Some(&char) = chars.peek() {
        match char {
            ',' | '[' | ']' | '{' | '}' | ':' | '"' | '\r' | '\n' | '\t' | ' ' => break,
            _ => {
                chars.next();
                literal.push(char);
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
    fn tokenize_successful() {
        assert_eq!(tokenize("true"), Ok(vec![Token::True]));
        assert_eq!(tokenize("false"), Ok(vec![Token::False]));
        assert_eq!(tokenize("null"), Ok(vec![Token::Null]));
        assert_eq!(
            tokenize("Null,"),
            Err("unexpected literal: Null".to_string())
        );
    }
}
